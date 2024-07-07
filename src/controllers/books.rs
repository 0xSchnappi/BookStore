use rocket::{
    http::Status,
    response::status,
    serde::{json::Json, Deserialize, Serialize},
    time::format_description::modifier::Year,
    State,
};
use sea_orm::{prelude::DateTimeUtc, *};
use std::time::SystemTime;

use super::{ErrorResponse, Response, SuccessResponse};
use crate::auth::AuthenticatedUser;
use sea_orm::*;

use crate::entities::{book, prelude::*};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ResBook {
    pub id: i32,
    pub author_id: i32,
    pub title: String,
    pub year: String,
    pub cover: String,
}

impl From<&book::Model> for ResBook {
    fn from(value: &book::Model) -> Self {
        Self {
            id: value.id,
            author_id: value.author_id,
            title: value.title.to_owned(),
            year: value.year.to_owned(),
            cover: value.cover.to_owned(),
        }
    }
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ResBookList {
    pub total: usize,
    pub books: Vec<ResBook>,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ReqBook {
    author_id: i32,
    title: String,
    year: String,
    cover: String,
}

#[get("/")]
pub async fn index(
    db: &State<DatabaseConnection>,
    _user: AuthenticatedUser,
) -> Response<Json<ResBookList>> {
    let db = db as &DatabaseConnection;

    let books = Book::find()
        .order_by_desc(book::Column::UpdatedAt)
        .all(db)
        .await?
        .iter()
        .map(ResBook::from)
        .collect::<Vec<_>>();

    Ok(SuccessResponse((
        Status::Ok,
        Json(ResBookList {
            total: books.len(),
            books,
        }),
    )))
}

#[post("/", data = "<req_book>")]
pub async fn create(
    db: &State<DatabaseConnection>,
    user: AuthenticatedUser,
    req_book: Json<ReqBook>,
) -> Response<Json<ResBook>> {
    let db = db as &DatabaseConnection;

    let book = book::ActiveModel {
        user_id: Set(user.id),
        author_id: Set(req_book.author_id),
        title: Set(req_book.title.to_owned()),
        year: Set(req_book.year.to_owned()),
        cover: Set(req_book.cover.to_owned()),
        ..Default::default()
    };

    let book = book.insert(db).await?;

    Ok(SuccessResponse((Status::Ok, Json(ResBook::from(&book)))))
}

#[get("/<id>")]
pub async fn show(
    db: &State<DatabaseConnection>,
    _user: AuthenticatedUser,
    id: i32,
) -> Response<Json<ResBook>> {
    let db = db as &DatabaseConnection;

    let book = Book::find_by_id(id).one(db).await?;

    let book = match book {
        Some(book) => book,
        None => {
            return Err(super::ErrorResponse((
                Status::NotFound,
                "Cannot find a book with the specified ID.".to_string(),
            )));
        }
    };

    Ok(SuccessResponse((Status::Ok, Json(ResBook::from(&book)))))
}

#[put("/<id>", data = "<req_book>")]
pub async fn update(
    db: &State<DatabaseConnection>,
    _user: AuthenticatedUser,
    id: i32,
    req_book: Json<ReqBook>,
) -> Response<Json<ResBook>> {
    let db = db as &DatabaseConnection;

    let mut book: book::ActiveModel = match Book::find_by_id(id).one(db).await? {
        Some(b) => b.into(),
        None => {
            return Err(ErrorResponse((
                Status::NotFound,
                "No book with the specified ID.".to_string(),
            )));
        }
    };

    book.title = Set(req_book.title.to_owned());
    book.year = Set(req_book.year.to_owned());
    book.cover = Set(req_book.cover.to_owned());

    book.updated_at = Set(Some((DateTimeUtc::from(SystemTime::now()))));

    let book = book.update(db).await?;

    Ok(SuccessResponse((Status::Ok, Json(ResBook::from(&book)))))
}

#[delete("/<id>")]
pub async fn delete(
    db: &State<DatabaseConnection>,
    _user: AuthenticatedUser,
    id: i32,
) -> Response<String> {
    let db = db as &DatabaseConnection;

    let book = match Book::find_by_id(id).one(db).await? {
        Some(b) => b,
        None => {
            return Err(ErrorResponse((
                Status::NotFound,
                "No book with the specified ID.".to_string(),
            )));
        }
    };

    book.delete(db).await?;

    Ok(SuccessResponse((Status::Ok, "book deleted.".to_string())))
}
