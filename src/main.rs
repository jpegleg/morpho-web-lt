use std::{fs::File, io::BufReader};
use actix_files::Files;
use rustls::{Certificate, PrivateKey, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys};
use actix_web::{middleware, App, HttpServer, get, Responder, HttpRequest};
use actix_files::NamedFile;
use actix_web_lab::{header::StrictTransportSecurity, middleware::RedirectHttps};
use std::env;

#[get("/")]
async fn index(req: HttpRequest) -> impl Responder {
    NamedFile::open_async("./static/index.html").await
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("morpho running on 3443 in minimal no logging mode");
    HttpServer::new(|| {
        App::new()
            .wrap(RedirectHttps::default())
            .wrap(RedirectHttps::with_hsts(StrictTransportSecurity::recommended()))
            .wrap(middleware::DefaultHeaders::new().add(("x-content-type-options", "nosniff")))
            .wrap(middleware::DefaultHeaders::new().add(("x-frame-options", "SAMEORIGIN")))
            .wrap(middleware::DefaultHeaders::new().add(("x-xss-protection", "1; mode=block")))
            .service(index)
            .service(Files::new("/", "static"))

    })
    .bind_rustls("0.0.0.0:3443", config)?
    .run()
    .await
}

fn load_rustls_config() -> rustls::ServerConfig {
    let config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth();
    let cert_file = &mut BufReader::new(File::open("cert.pem").unwrap());
    let key_file = &mut BufReader::new(File::open("privkey.pem").unwrap());
    let cert_chain = certs(cert_file)
        .unwrap()
        .into_iter()
        .map(Certificate)
        .collect();
    let mut keys: Vec<PrivateKey> = pkcs8_private_keys(key_file)
        .unwrap()
        .into_iter()
        .map(PrivateKey)
        .collect();
    if keys.is_empty() {
        eprintln!("morpho FATAL - Open of privkey.pem paired with cert.pem failed, server must shutdown. Use PKCS8 PEM compatible with rustls.");
        std::process::exit(1);
    }
    config.with_single_cert(cert_chain, keys.remove(0)).unwrap()
}
