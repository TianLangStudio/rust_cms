-- MySQL dump 10.13  Distrib 5.7.31, for Linux (x86_64)
--
-- Host: 127.0.0.1    Database: rust_cms
-- ------------------------------------------------------
-- Server version	5.7.18

/*!40101 SET @OLD_CHARACTER_SET_CLIENT=@@CHARACTER_SET_CLIENT */;
/*!40101 SET @OLD_CHARACTER_SET_RESULTS=@@CHARACTER_SET_RESULTS */;
/*!40101 SET @OLD_COLLATION_CONNECTION=@@COLLATION_CONNECTION */;
/*!40101 SET NAMES utf8 */;
/*!40103 SET @OLD_TIME_ZONE=@@TIME_ZONE */;
/*!40103 SET TIME_ZONE='+00:00' */;
/*!40014 SET @OLD_UNIQUE_CHECKS=@@UNIQUE_CHECKS, UNIQUE_CHECKS=0 */;
/*!40014 SET @OLD_FOREIGN_KEY_CHECKS=@@FOREIGN_KEY_CHECKS, FOREIGN_KEY_CHECKS=0 */;
/*!40101 SET @OLD_SQL_MODE=@@SQL_MODE, SQL_MODE='NO_AUTO_VALUE_ON_ZERO' */;
/*!40111 SET @OLD_SQL_NOTES=@@SQL_NOTES, SQL_NOTES=0 */;

-- create database
CREATE DATABASE IF NOT EXISTS rust_cms DEFAULT CHARSET utf8 COLLATE utf8_general_ci;

-- create user
create user rust_cms identified by '123456';
grant all privileges on rust_cms.* to 'rust_cms'@'%' identified by '123456' with grant option;
flush privileges;

-- Table structure for table `__diesel_schema_migrations`
--
use rust_cms;

DROP TABLE IF EXISTS `__diesel_schema_migrations`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `__diesel_schema_migrations` (
  `version` varchar(50) COLLATE utf8mb4_unicode_ci NOT NULL,
  `run_on` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (`version`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `__diesel_schema_migrations`
--

LOCK TABLES `__diesel_schema_migrations` WRITE;
/*!40000 ALTER TABLE `__diesel_schema_migrations` DISABLE KEYS */;
INSERT INTO `__diesel_schema_migrations` VALUES ('20200616014400','2020-06-16 09:56:22');
/*!40000 ALTER TABLE `__diesel_schema_migrations` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `tb_article`
--

DROP TABLE IF EXISTS `tb_article`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `tb_article` (
  `id` varchar(40) NOT NULL COMMENT 'ID',
  `title` varchar(200) DEFAULT '' COMMENT '文章标题',
  `subtitle` varchar(200) DEFAULT '' COMMENT '文章副标题',
  `intro` varchar(300) DEFAULT '' COMMENT '文章简介',
  `rcmd_weight` int(3) DEFAULT '-1' COMMENT '推荐权重',
  `url` varchar(300) DEFAULT NULL COMMENT '访问链接',
  `status` int(2) DEFAULT '0' COMMENT '文章状态',
  `creater` varchar(50) NOT NULL COMMENT '创建人',
  `create_at` datetime NOT NULL COMMENT '创建时间',
  `update_at` datetime NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='文章';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `tb_article`
--

LOCK TABLES `tb_article` WRITE;
/*!40000 ALTER TABLE `tb_article` DISABLE KEYS */;
INSERT INTO `tb_article` VALUES ('208ef34c-95a5-496e-8a0c-2ced6ac75109','hello name','','hello intro',-1,'',0,'zhangsan','2020-08-05 12:53:00','2020-08-05 14:33:08'),('251572cc-1d8f-4fd8-bb9f-50d098d85c7d','hello name','','hello intro',-1,'',0,'zhangsan','2020-08-05 12:50:19','2020-08-05 14:33:08'),('2e5eac14-9942-4033-9bfd-a15079a8ca0c','hello name','','hello intro',-1,'',0,'zhangsan','2020-08-05 12:49:08','2020-08-05 14:33:08'),('87049086-77e7-4aff-a653-31d2f9b4e358','hello name','','hello intro',-1,'',8,'zhangsan','2020-08-05 12:55:00','2020-08-05 14:33:08'),('8a39bec8-6c25-4748-b59b-a74a900c0c66','hello name','','hello intro',-1,'',0,'zhangsan','2020-08-05 12:43:43','2020-08-05 14:33:08'),('8ee45e13-d785-4041-99f3-bcca8711b06f','hello name','','hello intro',-1,'',0,'zhangsan','2020-08-05 12:29:56','2020-08-05 14:33:08'),('a7b46e55-f518-4884-94ac-07e0c727c817','hello name','','hello intro',-1,'',0,'zhangsan','2020-08-05 12:31:48','2020-08-05 14:33:08'),('baad0a71-6e01-4d53-9838-b51a2ae42ef2','hello name','','hello intro',-1,'',0,'zhangsan','2020-08-05 12:51:43','2020-08-05 14:33:08'),('c41b4a67-8f5b-4e1e-ab58-287bff1b4df9','hello name','','hello intro',-1,'',0,'zhangsan','2020-08-05 11:47:36','2020-08-05 14:33:08'),('d31a176d-9ca4-43dd-b570-8d0d209be2b3','hello name','','hello intro',-1,'',0,'zhangsan','2020-08-05 12:42:54','2020-08-05 14:33:08'),('e2bbfa86-7b42-4499-820f-093f7a999ec5','hello name','','hello intro',-1,'',8,'zhangsan','2020-08-05 12:43:25','2020-08-05 14:33:08');
/*!40000 ALTER TABLE `tb_article` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `tb_article_content`
--

DROP TABLE IF EXISTS `tb_article_content`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `tb_article_content` (
  `id` varchar(40) NOT NULL COMMENT 'ID',
  `status` int(2) DEFAULT '0' COMMENT '内容状态',
  `article_id` varchar(40) NOT NULL COMMENT 'article ID',
  `content` longtext COMMENT '文件内容',
  `create_at` datetime DEFAULT NULL COMMENT '创建时间',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='文章内容';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `tb_article_content`
--

LOCK TABLES `tb_article_content` WRITE;
/*!40000 ALTER TABLE `tb_article_content` DISABLE KEYS */;
INSERT INTO `tb_article_content` VALUES ('208ef34c-95a5-496e-8a0c-2ced6ac75109',0,'208ef34c-95a5-496e-8a0c-2ced6ac75109','<p>hello content</p>\n',NULL),('251572cc-1d8f-4fd8-bb9f-50d098d85c7d',0,'251572cc-1d8f-4fd8-bb9f-50d098d85c7d','<p>hello content</p>\n',NULL),('2e5eac14-9942-4033-9bfd-a15079a8ca0c',0,'2e5eac14-9942-4033-9bfd-a15079a8ca0c','<p>hellocontent</p>\n',NULL),('5cb1e9a8-6479-46cd-b372-fe23f9778d7c',8,'e2bbfa86-7b42-4499-820f-093f7a999ec5','<p>hello world hello<br>\n<a href=\"http://www.baidu.com\">hello</a><br>\nhelloworld<br>\nhelloworld<br>\nhelloworld</p>\n','2020-08-05 14:33:08'),('819e20d8-2dd0-43a2-9f1a-0758e07c6c9a',0,'','<p>hello world hahaha</p>\n',NULL),('87049086-77e7-4aff-a653-31d2f9b4e358',0,'87049086-77e7-4aff-a653-31d2f9b4e358','<p>hello content</p>\n',NULL),('8a39bec8-6c25-4748-b59b-a74a900c0c66',0,'8a39bec8-6c25-4748-b59b-a74a900c0c66','<p>hellocontent</p>\n',NULL),('8c0daf35-bde4-4397-b860-34fa40bc0041',0,'','<p>hello world hahaha</p>\n',NULL),('8ee45e13-d785-4041-99f3-bcca8711b06f',0,'8ee45e13-d785-4041-99f3-bcca8711b06f','<p>hellocontent</p>\n',NULL),('a7b46e55-f518-4884-94ac-07e0c727c817',0,'a7b46e55-f518-4884-94ac-07e0c727c817','<p>hellocontent</p>\n',NULL),('b9cfb7fb-146b-41ad-8122-3aa6ec1d7b05',0,'','<p>hello world</p>\n',NULL),('baad0a71-6e01-4d53-9838-b51a2ae42ef2',0,'baad0a71-6e01-4d53-9838-b51a2ae42ef2','<p>hello content</p>\n',NULL),('c41b4a67-8f5b-4e1e-ab58-287bff1b4df9',0,'c41b4a67-8f5b-4e1e-ab58-287bff1b4df9','<p>hello world hahaha</p>\n',NULL),('c87e4a1a-e3b0-415b-8c9f-2405bd5bf9da',0,'e2bbfa86-7b42-4499-820f-093f7a999ec5','<p>hello content</p>\n',NULL),('d31a176d-9ca4-43dd-b570-8d0d209be2b3',0,'d31a176d-9ca4-43dd-b570-8d0d209be2b3','<p>hellocontent</p>\n',NULL),('d60f3dee-b7a5-4735-bb98-87bd6c6336fd',0,'e2bbfa86-7b42-4499-820f-093f7a999ec5','<p>hello world hello<br>\n<a href=\"http://www.baidu.com\">hello</a></p>\n',NULL),('dd483c16-3aac-432a-b5aa-3d59aa2b0a08',8,'87049086-77e7-4aff-a653-31d2f9b4e358','<p>hello content</p>\n',NULL);
/*!40000 ALTER TABLE `tb_article_content` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `tb_file`
--

DROP TABLE IF EXISTS `tb_file`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `tb_file` (
  `id` varchar(40) NOT NULL COMMENT 'ID',
  `name` varchar(200) DEFAULT '' COMMENT '文件名称',
  `ext` varchar(200) DEFAULT '' COMMENT '文件扩展名',
  `is_private` int(1) DEFAULT '0' COMMENT '是否私有文件 0：非私有、否　1:私有、是',
  `creater` varchar(50) NOT NULL COMMENT '创建人',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='文件';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `tb_file`
--

LOCK TABLES `tb_file` WRITE;
/*!40000 ALTER TABLE `tb_file` DISABLE KEYS */;
INSERT INTO `tb_file` VALUES ('7005a289-6dd6-46b0-9707-674c841586a0','9tnwXrF.jpg','',0,'zhangsan');
/*!40000 ALTER TABLE `tb_file` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `tb_login_info`
--

DROP TABLE IF EXISTS `tb_login_info`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `tb_login_info` (
  `id` bigint(18) NOT NULL AUTO_INCREMENT,
  `username` varchar(32) NOT NULL,
  `password` varchar(200) NOT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `uk_username` (`username`)
) ENGINE=InnoDB AUTO_INCREMENT=6 DEFAULT CHARSET=utf8mb4;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `tb_login_info`
--

LOCK TABLES `tb_login_info` WRITE;
/*!40000 ALTER TABLE `tb_login_info` DISABLE KEYS */;
INSERT INTO `tb_login_info` VALUES (1,'tianlang','DR4Fy9TvcvV/p+loQsdGP0GR4S1b/oQZUo2c2Sq/6ks='),(5,'zhangsan','X04fbwsU2lfA5qOzcNOEKMpp9JFFP9mJomaIBbCj7+k=');
/*!40000 ALTER TABLE `tb_login_info` ENABLE KEYS */;
UNLOCK TABLES;
/*!40103 SET TIME_ZONE=@OLD_TIME_ZONE */;

/*!40101 SET SQL_MODE=@OLD_SQL_MODE */;
/*!40014 SET FOREIGN_KEY_CHECKS=@OLD_FOREIGN_KEY_CHECKS */;
/*!40014 SET UNIQUE_CHECKS=@OLD_UNIQUE_CHECKS */;
/*!40101 SET CHARACTER_SET_CLIENT=@OLD_CHARACTER_SET_CLIENT */;
/*!40101 SET CHARACTER_SET_RESULTS=@OLD_CHARACTER_SET_RESULTS */;
/*!40101 SET COLLATION_CONNECTION=@OLD_COLLATION_CONNECTION */;
/*!40111 SET SQL_NOTES=@OLD_SQL_NOTES */;

-- Dump completed on 2020-08-06 17:10:56
