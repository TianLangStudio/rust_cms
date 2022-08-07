![Rust CMS](./doc/imgs/screenshot.png)  

⚠这是个学习用项目，不要直接用于生产  
It is a project for learning. Don't use it in production!!!. 

# 功能
- [x]  登录登出
- [x]  文件上传　　
- [x]  文章列表   
- [x]  最新更新文章列表获取
- [x]  推荐文章列表获取
- [x]  文章发布　 
- [x]  文章详情　
- [x]  actix-web 样例
- [x]  diesel 样例 
- [x]  session 
- [x]  基于session的登录验证  
- [x]  Tera模板/自定义函数等
- [x]  支持Markdown 
- [ ]  我的文章页面
- [ ]  支持审核发布状态　
- [ ]  支持graphql
---  

# 运行 
如果已经安装了[Rust](https://www.rust-lang.org/tools/install)和[Docker](https://docs.docker.com/engine/install/) 可以直接执行脚本运行
> ./bin/start.sh  

脚本中会使用Docker初始化数据库并运行rust_cms,启动成功后就可以使用浏览器访问http://127.0.0.1:8088了  

你也可以自己初始化数据库,无论如何 **请先安装[Rust](https://www.rust-lang.org/tools/install)**  
> git clone git@github.com:TianLangStudio/rust_cms.git     
> cd rust_cms 
- 创建Mysql数据库　导入doc/rust_cms.sql  
- 修改conf/application_dev.yaml文件中的数据库链接配置　
> cargo run
---  

# 使用Rust编写的内容管理系统(CMS)
在项目[rust_login](https://github.com/TianLangStudio/rust_login)的基础上增加CMS功能 
**开发中** 进度可关注[博客](https://blog.csdn.net/tianlangstudio/article/details/106169242)  

---

# build error  
1. throw an error when building third part lib
   Maybe it is because of the version conflict,  you can remove the file named Cargo.lock then build again

2. cannot find -lmysqlclient
   > error: linking with `cc` failed: exit status: 1   
   > ....   
   > /usr/bin/ld: cannot find -lmysqlclient  

   install mysqlclient then build again ex. install mysqlclient in Ubuntu  
   >  sudo apt-get install libmysqlclient-dev
