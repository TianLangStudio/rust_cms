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
     
CREATE TABLE `tb_article` (
  `id` int(11) NOT NULL AUTO_INCREMENT COMMENT 'id',
  `folder_id` int(11) DEFAULT '1' COMMENT '目录id',
  `title` varchar(200) DEFAULT '' COMMENT '文章名称',
  `content` longtext COMMENT '文件内容',
  `count_view` int(11) DEFAULT '0' COMMENT '浏览数',
  `count_comment` int(11) DEFAULT '0' COMMENT '评论数',
  `type` int(11) DEFAULT '1' COMMENT '类型：1 正常 2 预览展示概述 3 程序调用处理',
  `status` varchar(20) DEFAULT '1' COMMENT '状态//radio/2,隐藏,1,显示',
  `is_comment` int(11) DEFAULT '1' COMMENT '是否评论：2 否 1 是',
  `is_recommend` int(11) DEFAULT '2' COMMENT '是否推荐：2 否 1 是',
  `sort` int(11) DEFAULT '1' COMMENT '排序',
  `jump_url` varchar(256) DEFAULT NULL COMMENT '跳转地址',
  `image_url` varchar(256) DEFAULT NULL COMMENT '图片路径',
  `image_net_url` varchar(256) DEFAULT NULL COMMENT '网络图片路径',
  `file_url` varchar(256) DEFAULT NULL,
  `file_name` varchar(256) DEFAULT NULL,
  `approve_status` int(11) DEFAULT NULL COMMENT '审核状态',
  `publish_time` varchar(64) DEFAULT NULL COMMENT '发布时间',
  `publish_user` varchar(64) DEFAULT '1' COMMENT '发布者',
  `start_time` varchar(64) DEFAULT NULL COMMENT '开始时间',
  `end_time` varchar(64) DEFAULT NULL COMMENT '结束时间',
  `update_time` varchar(64) DEFAULT NULL COMMENT '更新时间',
  `create_time` varchar(64) DEFAULT NULL COMMENT '创建时间',
  `create_id` int(11) DEFAULT '0' COMMENT '创建者',
  `is_vote` int(2) DEFAULT '0',
  `support_init_percent` int(3) DEFAULT '50',
  `support_text` varchar(50) DEFAULT '支持',
  `oppose_text` varchar(50) DEFAULT '反對',
  `neutral_text` varchar(50) DEFAULT '不关我事',
  `image_title` varchar(100) DEFAULT '',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=4271 DEFAULT CHARSET=utf8 COMMENT='文章' 