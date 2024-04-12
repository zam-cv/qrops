use crate::database::Database;
use actix_web::{error, get, web, HttpResponse, Responder, Result};

const CONTEXT_PATH: &str = "/api/admin/admins";

#[utoipa::path(
  context_path = CONTEXT_PATH,
  responses(
    (status = 200, description = "The users were found", body = Vec<Admin>)
  )
)]
#[get("")]
pub async fn get_admins(database: web::Data<Database>) -> Result<impl Responder> {
    let admins = database
        .get_admins()
        .await
        .map_err(|_| error::ErrorBadRequest("Failed"))?;

    Ok(HttpResponse::Ok().json(admins))
}