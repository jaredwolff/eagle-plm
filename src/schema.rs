use diesel::table;

table! {
    builds (id) {
        id -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        estimated_completion -> Timestamp,
        quantity -> Integer,
        cost -> Nullable<Float>,
        complete -> Integer,
        notes -> Nullable<Text>,
        part_ver -> Integer,
        part_id -> Integer,
    }
}

table! {
    inventories (id) {
        id -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        quantity -> Integer,
        consumed -> Integer,
        unit_price -> Nullable<Float>,
        notes -> Nullable<Text>,
        part_ver -> Integer,
        part_id -> Integer,
    }
}

table! {
    parts (id) {
        id -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        pn -> Text,
        mpn -> Text,
        digikeypn -> Nullable<Text>,
        descr -> Text,
        ver -> Integer,
        val -> Nullable<Text>,
        mqty -> Integer,
    }
}

table! {
    parts_parts (id) {
        id -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        quantity -> Integer,
        bom_ver -> Integer,
        refdes -> Text,
        nostuff -> Integer,
        bom_part_id -> Integer,
        part_id -> Integer,
    }
}

joinable!(builds -> parts (part_id));
joinable!(inventories -> parts (part_id));

allow_tables_to_appear_in_same_query!(builds, inventories, parts, parts_parts,);
