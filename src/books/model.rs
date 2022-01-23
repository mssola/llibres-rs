use crate::db;
use crate::schema::books;
use crate::visitor;
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use dropshot::HttpError;

// TODO: how to sort by surname

/// Book represents a book in our database.
///
/// Note that for `kind` we could be using a fancy enum, but that would require
/// doing funny things around `diesel` or adding some fishy boilerplate.
/// Instead, since we will only have one client, we allow the client to make
/// sense of this integer.
#[derive(Serialize, Deserialize, Queryable, Insertable, QueryableByName, PartialEq, JsonSchema)]
#[table_name = "books"]
pub struct Book {
    pub id: Uuid,
    pub title: String,
    pub supertitle: Option<String>,
    pub rate: i16,
    pub status: i16,
    pub location: Option<String>,
    pub author: String,
    pub publisher: String,
    pub language: String,
    pub notes: Option<String>,
    pub kind: Option<i16>,
    pub created_at: NaiveDateTime,
    pub bought_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

/// BookRequest represents the parameters we expect from an HTTP request on the
/// Book resource.
#[derive(Serialize, Deserialize, AsChangeset, Debug, JsonSchema)]
#[table_name = "books"]
pub struct BookRequest {
    pub title: String,
    pub supertitle: Option<String>,
    pub rate: i16,
    pub status: i16,
    pub location: Option<String>,
    pub author: String,
    pub publisher: String,
    pub language: String,
    pub notes: Option<String>,
    pub kind: Option<i16>,
    #[serde(default = "visitor::default_optional_datetime")]
    #[serde(deserialize_with = "visitor::from_optional_datetime")]
    pub bought_at: Option<NaiveDateTime>,
    #[serde(skip)]
    pub updated_at: Option<NaiveDateTime>,
}

impl Book {
    /// Returns a Result of all the given books on the database or an error.
    pub fn all() -> Result<Vec<Self>, HttpError> {
        let conn = db::connection()?;
        let b: Result<Vec<Self>, diesel::result::Error> =
            diesel::sql_query("SELECT * FROM books ORDER BY created_at").get_results(&conn);

        Ok(b.unwrap())
    }

    /// Inserts a new book into the database or returns an error.
    pub fn create(book: BookRequest) -> Result<Uuid, HttpError> {
        let conn = db::connection()?;

        let book = Book::from(book);
        match diesel::insert_into(books::table)
            .values(&book)
            .get_result::<Book>(&conn)
        {
            Ok(v) => Ok(v.id),
            // TODO: more fine-grained
            Err(_e) => Err(HttpError::for_bad_request(
                None,
                String::from("wabalubadubub"),
            )),
        }
    }

    /// Updates the desired book or returns an error.
    pub fn update(uid: Uuid, mut book: BookRequest) -> Result<Self, HttpError> {
        let conn = db::connection()?;

        // Force this optional field to the current time.
        book.updated_at = Some(Utc::now().naive_utc());

        match diesel::update(books::table)
            .filter(books::id.eq(uid))
            .set(book)
            .get_result(&conn)
        {
            Ok(v) => Ok(v),
            // TODO: more fine-grained
            Err(_e) => Err(HttpError::for_bad_request(
                None,
                String::from("wabalubadubub"),
            )),
        }
    }

    /// Deletes the book from the given id.
    pub fn delete(uid: Uuid) -> Result<usize, HttpError> {
        let conn = db::connection()?;

        match diesel::delete(books::table.filter(books::id.eq(uid))).execute(&conn) {
            Ok(v) => Ok(v),
            // TODO: more fine-grained
            Err(_e) => Err(HttpError::for_bad_request(
                None,
                String::from("wabalubadubub"),
            )),
        }
    }
}

/// Converting the data of an HTTP request targetting the books resource into
/// the Book model.
impl From<BookRequest> for Book {
    fn from(book: BookRequest) -> Self {
        Book {
            id: Uuid::new_v4(),
            title: book.title.trim().to_string(),
            supertitle: book.supertitle,
            rate: book.rate,
            status: book.status,
            location: book.location,
            author: book.author,
            publisher: book.publisher,
            notes: book.notes,
            language: book.language,
            kind: book.kind,
            bought_at: book.bought_at,
            created_at: Utc::now().naive_utc(),
            updated_at: Some(Utc::now().naive_utc()),
        }
    }
}
