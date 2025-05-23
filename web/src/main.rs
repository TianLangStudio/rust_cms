use actix_files as fs;
use actix_http::body::BoxBody;
use actix_session::{SessionExt, SessionMiddleware, storage::CookieSessionStore};
use actix_web::cookie::Key;
use actix_web::dev::ServiceResponse;
use actix_web::http::header::ContentType;
use actix_web::middleware::{ErrorHandlerResponse, ErrorHandlers};
use actix_web::web::redirect;
use actix_web::{App, HttpResponse, HttpServer, web};
use log::*;
use rustls::{
    ServerConfig,
    pki_types::{CertificateDer, PrivateKeyDer, pem::PemObject},
};
use tera::Tera;

use common::config_util::get_app_config;
use common::{config_util, db_util, log_util};

mod articlectrl;
mod filectrl;
mod funs;
mod indexctrl;
mod middleware;
mod userctrl;
mod web_util;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    log_util::init();
    info!("app starting");
    let secret_key = Key::generate();
    let server = HttpServer::new(move || {
        let mut tera = match Tera::new("template/**/*.*ml") {
            Ok(t) => t,
            Err(e) => {
                error!("Tera Parsing error: {}", e);
                ::std::process::exit(1);
            }
        };

        tera.register_function(
            "list_new_articles",
            funs::article::make_list_new_articles(db_util::get_pool().clone()),
        );
        tera.register_function(
            "list_recommend_articles",
            funs::article::make_list_recommend_articles(db_util::get_pool().clone()),
        );
        //  tera.full_reload();

        App::new()
            .app_data(web::Data::new(tera))
            .app_data(web::Data::new(db_util::get_pool().clone())) //绑定数据库链接池
            .wrap(middleware::AuthService) //添加根据Session验证登录状态的中间件
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(),
                secret_key.clone(),
            ))
            .service(filectrl::upload) //文件上传api
            .service(filectrl::view_file) //使用ID查看文件
            .service(userctrl::login) //用户登录接口
            .service(userctrl::logout) //退出登录
            .service(userctrl::register) //用户注册接口
            .service(userctrl::admin_test) //用于测试AuthService中间件是否有效的接口
            .service(articlectrl::admin_save_article) //保存文章接口
            .service(articlectrl::admin_publish_article) //文章发布
            // .service(articlectrl::admin_edit_article)//编辑文章接口
            .service(articlectrl::admin_edit_view) //文章编辑页面
            .service(articlectrl::view_article_by_id) //文章详情页面
            .service(articlectrl::view_article_by_id_and_status) //文章详情页面
            .service(articlectrl::view_articles) //文章列表页面
            .service(fs::Files::new("/static", "static").index_file("index.html")) //静态文件
            .service(indexctrl::favicon) //favicon
            .service(indexctrl::index) //首页
            .service(indexctrl::sitemap) //sitemap.xml
            .service(indexctrl::health)
            .service(redirect("/ads.txt", "/static/ads.txt"))
            .service(web::scope("").wrap(error_handlers()))
    });

    if get_app_config()
        .get_bool("tl.app.https.enable")
        .unwrap_or_default()
    {
        let config = load_rustls_config();
        let https_host = get_app_config()
            .get_string("tl.app.https.host")
            .unwrap_or("127.0.0.1".to_string());
        let https_port = get_app_config()
            .get_int("tl.app.https.host")
            .unwrap_or(8443);
        let address = format!("{}:{}", https_host, https_port);
        server.bind_rustls_0_23(&address, config)?.run().await
    } else {
        let port = config_util::get_app_config()
            .get_string("tl.app.http.port")
            .expect("port is required");
        let host = config_util::get_app_config()
            .get_string("tl.app.http.host")
            .expect("host is required");
        let host_port = host + ":" + &port;
        server.bind(&host_port)?.run().await
    }
}

fn load_rustls_config() -> rustls::ServerConfig {
    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .expect("Failed to install rustls::crypto::aws_lc_rs");

    // load TLS key/cert files
    let cert = "cert.pem";
    let cert_chain = CertificateDer::pem_file_iter(cert)
        .unwrap_or_else(|err| panic!("Failed to load certificate {} error: {:?}", cert, err))
        .flatten()
        .collect();
    let key = "key.pem";
    let key_der = PrivateKeyDer::from_pem_file(key)
        .unwrap_or_else(|err| panic!("Failed to load private key {} error: {:?}", key, err));
    ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(cert_chain, key_der)
        .unwrap()
}

// Custom error handlers, to return HTML responses when an error occurs.
fn error_handlers() -> ErrorHandlers<BoxBody> {
    ErrorHandlers::new().handler(http::StatusCode::NOT_FOUND, not_found)
}

// Error handler for a 404 Page not found error.
fn not_found<B>(res: ServiceResponse<B>) -> actix_web::Result<ErrorHandlerResponse<BoxBody>> {
    let response = get_error_response(&res, "Page not found");
    Ok(ErrorHandlerResponse::Response(ServiceResponse::new(
        res.into_parts().0,
        response.map_into_left_body(),
    )))
}

// Generic error handler.
fn get_error_response<B>(res: &ServiceResponse<B>, error: &str) -> HttpResponse {
    let request = res.request();
    log::error!("{} {}", error, request.path());
    let session = request.get_session();
    // Provide a fallback to a simple plain text response in case an error occurs during the
    // rendering of the error page.
    let fallback = |err: &str| {
        HttpResponse::build(res.status())
            .content_type(ContentType::plaintext())
            .body(err.to_string())
    };

    let tera = request.app_data::<web::Data<Tera>>().map(|t| t.get_ref());
    match tera {
        Some(tera) => {
            let tmpl_name = web_util::get_tmpl_from_session(&session);
            let tmpl_name = tmpl_name + "/error.html";
            let username = web_util::get_username_from_session(&session).unwrap_or_default();
            let mut context = tera::Context::new();
            context.insert("username", &username);
            context.insert("error", error);
            context.insert("status_code", res.status().as_str());
            let body = tera.render(&tmpl_name, &context);

            match body {
                Ok(body) => HttpResponse::build(res.status())
                    .content_type(ContentType::html())
                    .body(body),
                Err(_) => fallback(error),
            }
        }
        None => fallback(error),
    }
}
