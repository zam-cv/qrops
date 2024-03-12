use crate::{
    config::{self, CONFIG},
    database::Database,
    models,
    utils,
};
use actix_web::{error, post, web, HttpRequest, HttpResponse, Responder, Result};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2, PasswordHash, PasswordVerifier,
};
use validator::Validate;
use woothee::parser::Parser;

#[post("/signin")]
pub async fn signin(
    database: web::Data<Database>,
    profile: web::Json<models::Admin>,
) -> impl Responder {
    if let Ok(Some(user)) = database.get_user_by_email(profile.email.clone()).await {
        if let Ok(password) = PasswordHash::new(&user.password) {
            if Argon2::default()
                .verify_password(profile.password.as_bytes(), &password)
                .is_ok()
            {
                if let Some(id) = user.id {
                    if let Ok(token) = utils::create_token(&CONFIG.user_secret_key, id) {
                        let cookie = utils::get_cookie_with_token(&token);
                        return HttpResponse::Ok().cookie(cookie).finish();
                    }
                }
            }
        }
    }

    HttpResponse::Unauthorized().body("Username or password is incorrect")
}

#[post("/register")]
pub async fn register(
    req: HttpRequest,
    database: web::Data<Database>,
    mut user: web::Json<models::User>,
) -> Result<impl Responder> {
    if let Err(_) = user.validate() {
        return Ok(HttpResponse::Unauthorized().body("Invalid"));
    }

    if let Ok(hash) = utils::get_hash!(user.password) {
        if let Ok(None) = database.get_user_by_email(user.email.clone()).await {
            let parser = Parser::new();

            let ip = req.peer_addr().map(|addr| addr.ip().to_string());
            log::debug!("IP: {:?}", ip);
            let os = if let Some(user_agent) = req.headers().get("user-agent") {
                if let Ok(user_agent) = user_agent.to_str() {
                    parser.parse(user_agent).map(|ua| ua.os.to_string())
                } else {
                    None
                }
            } else {
                None
            };

            let player_id = database
                .create_player(models::Player {
                    id: None,
                    current_cycle: 0,
                    current_score: config::INITIAL_SCORE,
                    current_balance: config::INITIAL_BALANCE,
                    max_plots: config::INITIAL_MAX_PLOTS,
                })
                .await
                .map_err(|_| error::ErrorBadRequest("Failed"))?;

            user.id = None;
            user.ip = ip;
            user.password = hash.to_string();
            user.os = os;
            user.player_id = player_id;

            let user_id = database
                .create_user(user.into_inner())
                .await
                .map_err(|_| error::ErrorBadRequest("Failed"))?;

            if let Ok(token) = utils::create_token(&CONFIG.user_secret_key, user_id) {
                let cookie = utils::get_cookie_with_token(&token);
                return Ok(HttpResponse::Ok().cookie(cookie).finish());
            }
        }

        return Ok(HttpResponse::Unauthorized().body("Email already exists"));
    }

    Err(actix_web::error::ErrorBadRequest("Failed"))
}
