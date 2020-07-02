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
        PRIMARY KEY (`id`)
) ENGINE=InnoDB  DEFAULT CHARSET=utf8mb4 COMMENT='文章' ;

CREATE TABLE `tb_article_content` (
    `id` bigint(18) NOT NULL auto_increment COMMENT 'ID',
    `article_id` varchar(40) NOT NULL  COMMENT 'article ID',
    `content` longtext COMMENT '文件内容',
       PRIMARY KEY (`id`)
) ENGINE=InnoDB  DEFAULT CHARSET=utf8mb4 COMMENT='文章内容' ;

CREATE TABLE `tb_file`   (
  `id` varchar(40) NOT NULL COMMENT 'ID',
  `name` varchar(200) DEFAULT '' COMMENT '文件名称',
  `ext` varchar(200) DEFAULT '' COMMENT  '文件扩展名',
  `is_private` int(1) DEFAULT 0  COMMENT  '是否私有文件 0：非私有、否　1:私有、是',
  `creater` varchar(50) NOT NULL COMMENT '创建人',
   PRIMARY KEY (`id`)
)ENGINE=InnoDB  DEFAULT CHARSET=utf8mb4 COMMENT='文件' ;