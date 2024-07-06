use rocket::http::Status;
use sea_orm::DbErr;

pub mod auth;
pub mod authors;
pub mod books;

#[derive(Responder)]
pub struct SuccessResponse<T>(pub(Status, T));

#[derive(Responder)]
pub struct ErrorResponse(pub(Status, String));

pub type Response<T> = Result<SuccessResponse<T>, ErrorResponse>;

impl From<DbErr> for ErrorResponse {
    // 实现数据库错误，可以用?，用于数据库错误转换
    fn from(err: DbErr) -> Self {
        ErrorResponse((Status::InternalServerError, err.to_string()))
    }
}