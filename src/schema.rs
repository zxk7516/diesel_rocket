/// infer_schema!("dotenv:DATABASE_URL");
table! {
    posts (id) {
        id -> Integer,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}
