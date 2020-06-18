use std::fs::File;
use std::io::BufReader;
use actix_web::{ App, HttpServer};
use actix_session::{CookieSession};
use rustls::{NoClientAuth, ServerConfig};
use rustls::internal::pemfile::{certs, rsa_private_keys};

use common::{log_util, config_util, db_util};
mod userctrl;


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    log_util::init();

    //let app_config = config_util::APP_CONFIG;
    let is_prod = config_util::is_prod();
    let server = HttpServer::new(move || App::new()
            .data(db_util::POOL.clone())
                .wrap(
                    CookieSession::signed(&[0; 32]) // <- create cookie based session middleware
                        .secure(is_prod),
                ).service(userctrl::login)
                 .service(userctrl::register)
    );

    if is_prod  {

        let mut config = ServerConfig::new(NoClientAuth::new());
        let cert_file = &mut BufReader::new(File::open("./conf/cert.pem").unwrap());
        let key_file = &mut BufReader::new(File::open("./conf/key.pem").unwrap());
        let cert_chain = certs(cert_file).unwrap();
        let mut keys = rsa_private_keys(key_file).unwrap();
        config.set_single_cert(cert_chain, keys.remove(0)).unwrap();
        server.bind_rustls("127.0.0.1:8443", config)?
            .run()
            .await
    }else {
        server.bind("127.0.0.1:8088")?
            .run()
            .await
    }
}

