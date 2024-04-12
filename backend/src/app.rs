use crate::{
    bank, config::CONFIG, database::Database, docs::ApiDoc, middlewares, models, routes, socket,
    socket::server::Server, utils,
};
use actix_cors::Cors;
use actix_files as fs;
use actix_web::{middleware::Logger, web, App, HttpServer};
use actix_web_lab::middleware::from_fn;
use ip2location::DB;
use openssl::ssl::{SslAcceptor, SslAcceptorBuilder, SslFiletype, SslMethod};
use std::sync::{atomic::AtomicUsize, Arc, Mutex};
use tokio::sync::broadcast;
use utoipa::OpenApi;

const IPV6BIN: &str = "assets/IP2LOCATION-LITE-DB5.IPV6.BIN";

fn get_ssl_acceptor() -> anyhow::Result<SslAcceptorBuilder> {
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls())?;
    builder.set_private_key_file("cert/key.pem", SslFiletype::PEM)?;
    builder.set_certificate_chain_file("cert/cert.pem")?;
    Ok(builder)
}

async fn create_default_admin(database: &Database) {
    if let Ok(None) = database
        .get_admin_by_email(CONFIG.admin_default_email.clone())
        .await
    {
        if let Ok(password) = utils::get_hash_in_string(&CONFIG.admin_default_password) {
            let admin = models::Admin {
                id: None,
                email: CONFIG.admin_default_email.clone(),
                password
            };

            if let Ok(_) = database.create_admin(admin).await {
                log::info!("Default admin created");
            }
        }
    }
}

pub async fn app() -> std::io::Result<()> {
    // Create a channel for the viewer
    let (viewer_tx, _) = broadcast::channel::<()>(10);
    let viewer_tx_clone = viewer_tx.clone();

    // Create the bank
    let bank = bank::Bank::new();
    log::info!("Bank created");

    // Load the location database
    let location_db = if let Ok(db) = DB::from_file(IPV6BIN) {
        Some(Arc::new(Mutex::new(db)))
    } else {
        log::warn!("Failed to load location database, using empty database");
        None
    };
    log::info!("Location database connected");

    // Create the database
    let database = Database::new();
    log::info!("Database connected");

    // Create the default admin
    create_default_admin(&database).await;

    // Create the socket server
    let (mut socket_server, server_tx) = Server::new(bank, database.clone());
    tokio::spawn(async move { socket_server.run().await });
    log::info!("Socket server started");

    // Create a counter for the number of visitors
    let visitor_count = Arc::new(AtomicUsize::new(0));

    // Generate the OpenAPI documentation
    let openapi = ApiDoc::openapi();
    let doc_json = openapi.to_json().unwrap();
    log::info!("OpenAPI documentation generated");

    // Create the server
    let server = HttpServer::new(move || {
        // Create the CORS middleware
        let cors = Cors::permissive().supports_credentials();

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(location_db.clone()))
            .app_data(web::Data::new(database.clone()))
            .app_data(web::Data::new(server_tx.clone()))
            .app_data(web::Data::new(viewer_tx_clone.clone()))
            .app_data(web::Data::new(visitor_count.clone()))
            .app_data(web::Data::new(doc_json.clone()))
            .route(
                "/ws/",
                web::get()
                    .to(socket::server_index)
                    // Wrap the websocket route with the user_auth middleware
                    .wrap(from_fn(middlewares::user_auth)),
            )
            .route(
                "/viewer/",
                web::get()
                    .to(socket::viewer_index)
                    .wrap(from_fn(middlewares::admin_auth)),
            )
            .service(
                web::scope("/api")
                    .service(
                        web::scope("/auth")
                            .service(routes::auth::signin)
                            .service(routes::auth::register)
                            .service(routes::auth::signout)
                            .service(
                                web::scope("")
                                    .wrap(from_fn(middlewares::user_auth))
                                    .service(routes::auth::auth),
                            ),
                    )
                    .service(
                        web::scope("/admin")
                            .service(
                                web::scope("/auth")
                                    .service(routes::admin::auth::signin)
                                    .service(routes::admin::auth::signout)
                                    .service(
                                        web::scope("")
                                            .wrap(from_fn(middlewares::admin_auth))
                                            .service(routes::admin::auth::auth),
                                    ),
                            )
                            .service(
                                web::scope("")
                                    .wrap(from_fn(middlewares::admin_auth))
                                    .service(routes::admin::docs::api)
                                    .service(
                                        web::scope("/user")
                                            .service(routes::admin::user::get_user_statistics)
                                            .service(routes::admin::user::get_user),
                                    )
                                    .service(
                                        web::scope("/users")
                                            .service(routes::admin::users::get_users),
                                    )
                                    .service(
                                        web::scope("/player")
                                            .service(routes::admin::player::get_player),
                                    )
                                    .service(
                                        web::scope("/players")
                                            .service(routes::admin::players::get_players_count),
                                    )
                                    .service(
                                        web::scope("/data")
                                            .service(routes::admin::data::create_crop_type),
                                    ),
                            ),
                    ),
            )
            .service(
                fs::Files::new("/", "../page/")
                    .show_files_listing()
                    .index_file("index.html"),
            )
            .wrap(Logger::default())
    });

    // SSL configuration
    let server = if let Ok(builder) = get_ssl_acceptor() {
        log::info!("SSL configuration loaded");
        log::info!("Server running at https://{}", &CONFIG.address);
        server.bind_openssl(&CONFIG.address, builder)?
    } else {
        log::warn!("Failed to load SSL configuration, using insecure connection");
        log::info!("Server running at http://{}", &CONFIG.address);
        server.bind(&CONFIG.address)?
    };

    server.run().await
}
