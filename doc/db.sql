--用户登录信息表
CREATE  TABLE `tb_login_info` (
                            `id` bigint(18) NOT NULL auto_increment,
                            `username` varchar(32)  NOT NULL,
                            `password` varchar(200) NOT NULL,
                            PRIMARY KEY (`id`),
                            UNIQUE KEY  `uk_username`(`username`)
) ENGINE=InnoDB  DEFAULT CHARSET=utf8mb4;