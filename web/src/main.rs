use actix_files as fs;
use actix_session::{SessionMiddleware, storage::CookieSessionStore};
use actix_web::cookie::Key;
use actix_web::{web, App, HttpServer};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use log::*;

use tera::Tera;

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
    //let app_config = config_util::APP_CONFIG;
    let is_prod = config_util::is_prod();
    let secret_key = Key::generate();
    let server = HttpServer::new(move || {
        let mut tera = match Tera::new("template/**/*.html") {
            Ok(t) => t,
            Err(e) => {
                error!("Tera Parsing error: {}", e);
                ::std::process::exit(1);
            }
        };

        tera.register_function(
            "list_new_articles",
            funs::article::make_list_new_articles(db_util::POOL.clone()),
        );
        tera.register_function(
            "list_recommend_articles",
            funs::article::make_list_recommend_articles(db_util::POOL.clone()),
        );
        //  tera.full_reload();

        App::new()
            .app_data(web::Data::new(tera))
            .app_data(web::Data::new(db_util::POOL.clone())) //绑定数据库链接池
            .wrap(middleware::AuthService) //添加根据Session验证登录状态的中间件
            .wrap(SessionMiddleware::new(CookieSessionStore::default(), secret_key.clone()))
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
            .service(articlectrl::view_articles) //文章列表页面
            .service(fs::Files::new("/static", "static").show_files_listing()) //静态文件
            .service(indexctrl::favicon) //favicon
            .service(indexctrl::index) //首页
    });

    if is_prod {
        let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
        builder
            .set_private_key_file("./conf/key.pem", SslFiletype::PEM)
            .unwrap();
        builder.set_certificate_chain_file("./conf/cert.pem").unwrap();
        server.bind_openssl("127.0.0.1:8443", builder)?.run().await
    } else {
        let port = config_util::APP_CONFIG
            .get_string("tl.app.http.port")
            .expect("port is required");
        let host = config_util::APP_CONFIG
            .get_string("tl.app.http.host")
            .expect("host is required");
        let host_port = host + ":" + &port;
        server.bind(&host_port)?.run().await
    }
}
