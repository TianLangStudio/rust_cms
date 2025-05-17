![Rust CMS](./doc/imgs/rust_cms_screenshot_en.png)  

[Demo](https://www.tianlang.tech) run on [Cloudflare](https://www.cloudflare.com/)

⚠Don't use it in production!!!. It is a project for learning. 

English|[中文](./README_CN.md)

# Features
- [x]  Login & Logout
- [x]  File Upload　　
- [x]  Articles list
- [x]  Got new Articles
- [x]  Got Recommend Articles
- [x]  Publish Article Page　 
- [x]  Article Details Page
- [x]  actix-web demo
- [x]  diesel demo
- [x]  Support Session 
- [x]  Login auth base on Session  
- [x]  Tera Demo includes custom define Functions 
- [x]  Support Markdown in write Article 
- [ ]  My article page
- [x]  Support under review　  
    enable this feature by config tl.app.approval.enable=true and set who can approve through this configuration item tl.app.approval.users=usernameA,usernameB... 

- [ ]  Support graphql
---  

# How to run
```bash 
git clone git@github.com:TianLangStudio/rust_cms.git     
cd rust_cms
```   
If you have installed [Rust](https://www.rust-lang.org/tools/install) and [Docker](https://docs.docker.com/engine/install/), just execute the script below.  
```bash 
./bin/start.sh
```  
This command will run script to initial a MySQL database using docker and then run rust_cms using `Cargo run`

You could use a exists MySql database, installing **[Rust](https://www.rust-lang.org/tools/install)** is required

- import doc/rust_cms.sql to your Mysql, this will create a database named rust_cms.  
- edit conf/application_dev.yaml to change the database link information. 
- now you can execute command `cargo run` to run rust_cms
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

Contact [FusionZhu](https://www.upwork.com/fl/huanqingzhu?mp_source=share) for more help 
