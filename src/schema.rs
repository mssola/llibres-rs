table! {
    books (id) {
        id -> Uuid,
        title -> Varchar,
        supertitle -> Nullable<Varchar>,
        rate -> Int2,
        status -> Int2,
        location -> Nullable<Varchar>,
        author -> Varchar,
        publisher -> Varchar,
        language -> Varchar,
        notes -> Nullable<Varchar>,
        kind -> Nullable<Int2>,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        bought_at -> Nullable<Timestamp>,
    }
}
