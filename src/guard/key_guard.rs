use std::env;

use actix_web::guard::{Guard, GuardContext};

pub struct KeyGuardAuthorization;

impl Guard for KeyGuardAuthorization {
    fn check(&self, req: &GuardContext) -> bool {
        let key = req.head().headers().get("X-API-KEY");
        if let Some(key) = key {
            // Bearer key
            let key = key.to_str().unwrap();
            let api_key = env::var("API_KEY").expect("Missing API_KEY Env");
            if key == api_key {
                return true;
            }
        }
        false
    }
}