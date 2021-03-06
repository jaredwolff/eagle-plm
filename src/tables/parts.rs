extern crate diesel;

use prettytable::{row, Table};
use serde::Deserialize;

use crate::{models::*, *};
use diesel::prelude::*;

use std::fs::File;
use std::io::BufReader;

#[derive(Debug, Deserialize)]
struct Record {
    pn: String,
    mpn: String,
    desc: String,
}

pub fn create(app: &mut crate::Application) {
    // Get the input from stdin
    let pn = app.prompt.ask_text_entry("Part Number: ");
    let mpn = app.prompt.ask_text_entry("Manufacturer Part Number: ");
    let desc = app.prompt.ask_text_entry("Description: ");
    let ver = app.prompt.ask_text_entry("Version: ");
    let ver: i32 = ver.trim().parse().expect("Invalid version number!");

    // Create the part
    let part = NewUpdatePart {
        pn: &pn,
        mpn: &mpn,
        descr: &desc,
        ver: &ver,
        mqty: &1,
    };

    let found = find_part_by_pn(&app.conn, &pn);

    // If already found ask if it should be updated
    if let Ok(found) = found {
        let question = format!("{} already exists! Would you like to update it?", pn);
        let update = app.prompt.ask_yes_no_question(&question);

        // Update if they said yes.
        if update {
            update_part(&app.conn, &found.id, &part).expect("Unable to update part!");

            // Check for success
            println!("{} updated!", pn);
        }
    } else {
        create_part(&app.conn, &part).expect("Unable to create part!");
    }
}

pub fn rename(app: &mut crate::Application) {
    // Get the input from stdin
    let pn = app.prompt.ask_text_entry("Part Number: ");
    let newpn = app.prompt.ask_text_entry("New Part Number: ");

    rename_part(&app.conn, &pn, &newpn).expect("Unable to change pn");
}

pub fn create_by_csv(app: &mut crate::Application, filename: &str) {
    // Open the file
    let file = File::open(filename).unwrap();
    let file = BufReader::new(file);

    let mut records: Vec<Record> = Vec::new();

    let mut rdr = csv::Reader::from_reader(file);

    // TODO handle empty or malformed content a bit... better.
    // TODO: handle invalid data that's not UTF8
    for result in rdr.deserialize() {
        // Notice that we need to provide a type hint for automatic
        // deserialization.
        let record: Record = result.expect("Unable to deserialize.");
        println!("Processing: {:?}", record);
        records.push(record);
    }

    // Iterate through all the records.
    for record in records {
        // Create a new part from the CSV file
        let part = models::NewUpdatePart {
            pn: &record.pn,
            mpn: &record.mpn,
            descr: &record.desc,
            ver: &1,
            mqty: &1,
        };

        let found = find_part_by_pn(&app.conn, &part.pn);

        // If already found ask if it should be updated
        if let Ok(found) = found {
            // Compare the two make sure they're different
            if found.mpn != part.mpn || found.descr != part.descr || found.ver != *part.ver {
                let question = format!("{} already exists! Would you like to update it?", part.pn);

                // Create the table
                let mut table = Table::new();
                table.add_row(row![
                    "Current:",
                    found.pn,
                    found.mpn,
                    found.descr,
                    found.ver
                ]);
                table.add_row(row!["Change to:", part.pn, part.mpn, part.descr, part.ver]);
                table.printstd();

                let update = app.prompt.ask_yes_no_question(&question);

                // Update if they said yes.
                if update {
                    update_part(&app.conn, &found.id, &part).expect("Unable to update part!");

                    // Check for success
                    println!("{} updated!", part.pn);
                }
            }
        } else {
            println!("Creating: {:?}", part);
            create_part(&app.conn, &part).expect("Unable to create part!");
        }
    }
}

pub fn delete(app: &mut crate::Application) {
    let part = app.prompt.ask_text_entry("Part Number: ");

    // First find the parts.
    let part = find_part_by_pn(&app.conn, &part).expect("Unable to find part!");

    // Then ask the user to confirm they want to delete
    let question = format!("Would you like to delete {}?", part.pn);
    let delete = app.prompt.ask_yes_no_question(&question);

    // THEN, delete if they said yes.
    if delete {
        // Delete the part
        let res = delete_part(&app.conn, &part.id);

        // Depending on the result show the feedback
        if res.is_err() {
            panic!("Error deleting part {}.", part.pn);
        } else {
            println!("Deleted {}", part.pn);
        }
    }
}

pub fn show(app: &mut crate::Application) {
    use crate::schema::*;

    // Create the table
    let mut table = Table::new();

    let results = parts::dsl::parts
        .load::<models::Part>(&app.conn)
        .expect("Error loading parts");

    println!("Displaying {} parts", results.len());
    table.add_row(row!["PN", "MPN", "Desc", "Mqty", "Ver"]);
    for part in results {
        table.add_row(row![part.pn, part.mpn, part.descr, part.mqty, part.ver]);
    }
    table.printstd();
}

// pub fn search() {
//   // TODO: use a partial/pattern to search
//   // TODO: default is to search by PN but options to search by mpn, desc, etc.
// }
