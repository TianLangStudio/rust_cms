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
) ENGINE=InnoDB  DEFAULT CHARSET=utf8mb4 COMMENT='文章' ;
     