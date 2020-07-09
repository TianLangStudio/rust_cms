use std::fs::File;
use std::io::BufReader;
use actix_web::{ App, HttpServer};
use actix_files as fs;
use actix_session::{CookieSession};
use rustls::{NoClientAuth, ServerConfig};
use rustls::internal::pemfile::{certs, rsa_private_keys};
use tera::{Tera};
use log::*;

use common::{log_util, config_util, db_util};
mod web_util;
mod userctrl;
mod articlectrl;
mod indexctrl;
mod filectrl;
mod middleware;
mod funs;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    log_util::init();
    info!("app starting");
  
    //let app_config = config_util::APP_CONFIG;
    let is_prod = config_util::is_prod();
    let server = HttpServer::new(move || {
        let mut tera =
        match Tera::new("template/**/*.html")  {
            Ok(t) => t,
            Err(e) => {
                error!("Tera Parsing error: {}", e);
                ::std::process::exit(1);
            }
         };

         tera.register_function("list_new_articles", funs::article::make_list_new_articles(db_util::POOL.clone()));
       //  tera.full_reload();

        App::new()
            .data(tera)
            .data(db_util::POOL.clone()) //绑定数据库链接池
            .wrap(middleware::AuthService{}) //添加根据Session验证登录状态的中间件
            .wrap(
                    CookieSession::signed(&[0; 32]) // <- 添加使用cookie实现的session中间件
                        .secure(is_prod),
            )
            .service(filectrl::upload)//文件上传api
            .service(filectrl::view_file)//使用ID查看文件
            .service(userctrl::login) //用户登录接口
            .service(userctrl::logout)//退出登录
            .service(userctrl::register)//用户注册接口
            .service(userctrl::admin_test)//用于测试AuthService中间件是否有效的接口
            .service(articlectrl::admin_add_article)//新增文章接口
            .service(articlectrl::admin_edit_article)//编辑文章接口
            .service(articlectrl::admin_edit_view)//文章编辑页面
            .service(articlectrl::view_article_by_id)//文章详情页面
            .service(fs::Files::new("/static", "static").show_files_listing())//静态文件
            .service(indexctrl::favicon)//favicon
            .service(indexctrl::index)//首页
    });

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
        let port = config_util::APP_CONFIG.get_str("tl.app.http.port").expect("port is required");
        let host = config_util::APP_CONFIG.get_str("tl.app.http.host").expect("host is required");
        let host_port = host + ":" + &port;
        server.bind(&host_port)?
            .run()
            .await
    }
}

