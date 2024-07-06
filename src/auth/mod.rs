use jsonwebtoken::{decode, DecodingKey, Validation};
use rocket::{
    http::Status,
    request::{self, FromRequest, Outcome, Request},
    serde::{Deserialize, Serialize},
};

use crate::AppConfig;

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct Claims {
    sub: i32,
    role: String, // 角色
    exp: u64,     // 过期时间
}


pub struct AuthenticatedUser {
    pub id: i32,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = String;

    async fn from_request(req: &'r Request<'_>)-> request::Outcome<Self, Self::Error> {
        if let Some(token) = req.headers().get_one("Token") {
            let config = req.rocket().state::<AppConfig>().unwrap();

            let data = decode::<Claims>(
                token, 
                &DecodingKey::from_secret(config.jwt_sercert.as_bytes()), 
                &Validation::new(jsonwebtoken::Algorithm::HS256),
            );

            let claims = match data {
                Ok(p) => p.claims,
                Err(_) => {
                    return Outcome::Error((Status::Unauthorized, "Invalid token".to_string()));
                }
            };

            Outcome::Success(AuthenticatedUser {id: claims.sub})
        }else {
            Outcome::Error((Status::Unauthorized, "Token absent".to_string()))
        }
    }
}