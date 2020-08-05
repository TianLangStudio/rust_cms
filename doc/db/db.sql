--用户登录信息表
CREATE  TABLE `tb_login_info` (
                            `id` bigint(18) NOT NULL auto_increment,
                            `username` varchar(32)  NOT NULL,
                            `password` varchar(200) NOT NULL,
                            PRIMARY KEY (`id`),
                            UNIQUE KEY  `uk_username`(`username`)
) ENGINE=InnoDB  DEFAULT CHARSET=utf8mb4;

CREATE TABLE `tb_article` (
     `id` varchar(40) NOT NULL  COMMENT 'ID',
      `title` varchar(200) DEFAULT '' COMMENT '文章标题',
      `subtitle` varchar(200) DEFAULT '' COMMENT '文章副标题',
      `intro` varchar(300) DEFAULT '' COMMENT '文章简介',
      `creater` varchar(50) NOT NULL COMMENT '创建人',
        PRIMARY KEY (`id`)
) ENGINE=InnoDB  DEFAULT CHARSET=utf8mb4 COMMENT='文章' ;

alter table tb_article add column rcmd_weight int(3)  default -1 COMMENT  '推荐权重' after intro  ;
alter table tb_article add column url varchar(300)   COMMENT  '访问链接' after rcmd_weight  ;
alter table tb_article add column status int(2)  default 0  COMMENT  '文章状态' after url; -- 0 新建  8  发布 文章状态
alter table tb_article add column create_at datetime NOT NULL COMMENT '创建时间' after creater;
alter table tb_article add column update_at datetime NOT NULL COMMENT '更新时间' after create_at;

--alter table tb_article modify column create_at datetime NOT NULL  COMMENT '创建时间' after creater;
CREATE TABLE `tb_article_content` (
     `id` varchar(40) NOT NULL  COMMENT 'ID',
     `status`  int(2)  default 0  COMMENT  '内容状态',
    `article_id` varchar(40) NOT NULL  COMMENT 'article ID',
    `content` longtext COMMENT '文件内容',
       PRIMARY KEY (`id`)
) ENGINE=InnoDB  DEFAULT CHARSET=utf8mb4 COMMENT='文章内容' ;
alter table tb_article_content add column status int(2)  default 0  COMMENT  '内容状态' after article_id; -- 0 新建  8  发布 内容状态


CREATE TABLE `tb_file`   (
  `id` varchar(40) NOT NULL COMMENT 'ID',
  `name` varchar(200) DEFAULT '' COMMENT '文件名称',
  `ext` varchar(200) DEFAULT '' COMMENT  '文件扩展名',
  `is_private` int(1) DEFAULT 0  COMMENT  '是否私有文件 0：非私有、否　1:私有、是',
  `creater` varchar(50) NOT NULL COMMENT '创建人',
   PRIMARY KEY (`id`)
)ENGINE=InnoDB  DEFAULT CHARSET=utf8mb4 COMMENT='文件' ;