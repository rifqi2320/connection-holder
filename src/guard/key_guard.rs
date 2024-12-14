use std::env;
use actix_web::guard::{Guard, GuardContext};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Claims {
    topic: String,
}

pub struct KeyGuardAuthorization;

impl Guard for KeyGuardAuthorization {
    fn check(&self, req: &GuardContext) -> bool {
        let auth = req.head().headers().get("Authorization");
        let path = req.head().uri.path();

        if let Some(auth) = auth {
            if let Ok(auth_str) = auth.to_str() {
                let token = auth_str.split("Bearer ").nth(1).unwrap_or("");
                let key = env::var("JWT_SECRET").unwrap_or_default();

                if let Ok(claims) = decode::<Claims>(
                    token,
                    &DecodingKey::from_secret(key.as_ref()),
                    &Validation::new(jsonwebtoken::Algorithm::HS256),
                ) {
                    let topic = path.split('/').nth(2).unwrap_or("");
                    return topic == claims.claims.topic;
                }
            }
        }
        false
    }
}
