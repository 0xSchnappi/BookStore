use controllers::SuccessResponse;
use fairings::cors::options;
use migrator::Migrator;
use rocket::http::Status;
use sea_orm_migration::MigratorTrait;

#[macro_use]
extern crate rocket;

mod auth;
mod controllers;
mod db;
mod entities;
mod fairings;
mod migrator;

pub struct AppConfig {
    db_host: String,
    db_port: String,
    db_username: String,
    db_password: String,
    db_database: String,
    jwt_sercert: String,
}

impl AppConfig {
    fn new() -> Self {
        Self {
            db_host: std::env::var("BOOKSTORE_DB_HOST").unwrap_or("localhost".to_string()),
            db_port: std::env::var("BOOKSTORE_DB_PORT").unwrap_or("3306".to_string()),
            db_username: std::env::var("BOOKSTORE_DB_USERNAME").unwrap_or("bookstore".to_string()),
            db_password: std::env::var("BOOKSTORE_DB_PASSWORD")
                .unwrap_or("ZhangYing.730298".to_string()),
            db_database: std::env::var("BOOKSTORE_DB_DATABASE").unwrap_or("bookstore".to_string()),
            jwt_sercert: std::env::var("BOOKSTORE_JWT_SECRET")
                .expect("Please set the BOOKSTORE_JWT_SECRET env variable."),
        }
    }
}

#[get("/")]
fn index() -> controllers::Response<String> {
    Ok(SuccessResponse((Status::Ok, "Hello World".to_string())))
}

#[launch]
async fn rocket() -> _ {
    dotenvy::dotenv().ok();
    // env::set_var("ROCKET_ADDRESS", "0.0.0.0");
    // env::set_var("ROCKET_PORT", "80");

    let config = AppConfig::new();

    let db = match db::connect(&config).await {
        Ok(db) => db,
        Err(err) => panic!("[-]数据库连接失败{}", err),
    };

    match Migrator::up(&db, None).await {
        Err(err) => panic!("[-] 数据库迁移失败{}", err),
        Ok(_) => (),
    };

    rocket::build()
        .attach(fairings::cors::CORS)
        .manage(db)
        .manage(config)
        .mount("/", routes![options])
        .mount("/", routes![index])
        .mount(
            "/auth",
            routes![
                controllers::auth::sigin_in,
                controllers::auth::sigin_up,
                controllers::auth::me,
            ],
        )
        .mount(
            "/authors",
            routes![
                controllers::authors::index,
                controllers::authors::create,
                controllers::authors::show,
                controllers::authors::update,
                controllers::authors::delete,
                controllers::authors::get_books,
            ],
        )
        .mount(
            "/books",
            routes![
                controllers::books::index,
                controllers::books::create,
                controllers::books::show,
                controllers::books::update,
                controllers::books::delete,
            ],
        )
}
