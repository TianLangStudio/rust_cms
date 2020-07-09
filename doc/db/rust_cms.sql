-- MySQL dump 10.13  Distrib 5.7.30, for Linux (x86_64)
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

--
-- Table structure for table `__diesel_schema_migrations`
--

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
  `creater` varchar(50) NOT NULL COMMENT '创建人',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='文章';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `tb_article`
--

LOCK TABLES `tb_article` WRITE;
/*!40000 ALTER TABLE `tb_article` DISABLE KEYS */;
INSERT INTO `tb_article` VALUES ('7bc3ba05-0e1d-4265-91af-f6fe5f65e61b','helloworld','','hello world editor','zhangsan');
/*!40000 ALTER TABLE `tb_article` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `tb_article_content`
--

DROP TABLE IF EXISTS `tb_article_content`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `tb_article_content` (
  `id` bigint(18) NOT NULL AUTO_INCREMENT COMMENT 'ID',
  `article_id` varchar(40) NOT NULL COMMENT 'article ID',
  `content` longtext COMMENT '文件内容',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=4 DEFAULT CHARSET=utf8mb4 COMMENT='文章内容';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `tb_article_content`
--

LOCK TABLES `tb_article_content` WRITE;
/*!40000 ALTER TABLE `tb_article_content` DISABLE KEYS */;
INSERT INTO `tb_article_content` VALUES (2,'7bc3ba05-0e1d-4265-91af-f6fe5f65e61b','<b>hello world&nbsp;</b>'),(3,'7bc3ba05-0e1d-4265-91af-f6fe5f65e61b','\n                 <b>hello world editor</b>');
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
) ENGINE=InnoDB AUTO_INCREMENT=4 DEFAULT CHARSET=utf8mb4;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `tb_login_info`
--

LOCK TABLES `tb_login_info` WRITE;
/*!40000 ALTER TABLE `tb_login_info` DISABLE KEYS */;
INSERT INTO `tb_login_info` VALUES (1,'tianlang','77245687E9A2C7F020AFACBE53919CD017AEAAAE448B5F1B2C71F6C0A210CD050F5FB8EF615BC9AC4CA582EE34198B73F3888DBC8C258FB41C5CB44E4DC8B22D'),(3,'zhangsan','wYGYGePjkfBHQXNcUobmKPYnEGJKQsjRyT1t5mkxLr6Mk94MJSRuKWJ6kZjxO4XYhEzNPe2/0YCptgGA714bcw==');
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

-- Dump completed on 2020-07-09 22:33:00
