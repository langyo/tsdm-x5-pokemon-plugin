/*M!999999\- enable the sandbox mode */ 

/*!40101 SET @OLD_CHARACTER_SET_CLIENT=@@CHARACTER_SET_CLIENT */;
/*!40101 SET @OLD_CHARACTER_SET_RESULTS=@@CHARACTER_SET_RESULTS */;
/*!40101 SET @OLD_COLLATION_CONNECTION=@@COLLATION_CONNECTION */;
/*!40101 SET NAMES utf8mb4 */;
/*!40103 SET @OLD_TIME_ZONE=@@TIME_ZONE */;
/*!40103 SET TIME_ZONE='+00:00' */;
/*!40014 SET @OLD_UNIQUE_CHECKS=@@UNIQUE_CHECKS, UNIQUE_CHECKS=0 */;
/*!40014 SET @OLD_FOREIGN_KEY_CHECKS=@@FOREIGN_KEY_CHECKS, FOREIGN_KEY_CHECKS=0 */;
/*!40101 SET @OLD_SQL_MODE=@@SQL_MODE, SQL_MODE='NO_AUTO_VALUE_ON_ZERO' */;
/*!40111 SET @OLD_SQL_NOTES=@@SQL_NOTES, SQL_NOTES=0 */;

CREATE DATABASE /*!32312 IF NOT EXISTS*/ `discuz` /*!40100 DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci */;

USE `discuz`;
DROP TABLE IF EXISTS `pre_common_admincp_cmenu`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_common_admincp_cmenu` (
  `id` smallint(6) unsigned NOT NULL AUTO_INCREMENT,
  `title` varchar(255) NOT NULL,
  `url` varchar(255) NOT NULL,
  `sort` tinyint(1) NOT NULL DEFAULT 0,
  `displayorder` tinyint(3) NOT NULL,
  `clicks` smallint(6) unsigned NOT NULL DEFAULT 1,
  `uid` mediumint(8) unsigned NOT NULL,
  `dateline` int(10) unsigned NOT NULL,
  PRIMARY KEY (`id`),
  KEY `uid` (`uid`),
  KEY `displayorder` (`displayorder`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_common_admincp_cmenu` WRITE;
/*!40000 ALTER TABLE `pre_common_admincp_cmenu` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_common_admincp_cmenu` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_common_admincp_group`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_common_admincp_group` (
  `cpgroupid` smallint(6) unsigned NOT NULL AUTO_INCREMENT,
  `cpgroupname` varchar(255) NOT NULL,
  PRIMARY KEY (`cpgroupid`)
) ENGINE=InnoDB AUTO_INCREMENT=6 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_common_admincp_group` WRITE;
/*!40000 ALTER TABLE `pre_common_admincp_group` DISABLE KEYS */;
INSERT INTO `pre_common_admincp_group` VALUES
(1,'闂ㄦ埛绠＄悊鍛?),
(2,'璁哄潧绠＄悊鍛?),
(3,'鍦堝瓙绠＄悊鍛?),
(4,'绌洪棿绠＄悊鍛?),
(5,'鐢ㄦ埛绠＄悊鍛?);
/*!40000 ALTER TABLE `pre_common_admincp_group` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_common_admincp_member`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_common_admincp_member` (
  `uid` int(10) unsigned NOT NULL,
  `cpgroupid` int(10) unsigned NOT NULL,
  `customperm` text NOT NULL,
  PRIMARY KEY (`uid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_common_admincp_member` WRITE;
/*!40000 ALTER TABLE `pre_common_admincp_member` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_common_admincp_member` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_common_admincp_menu_platform`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_common_admincp_menu_platform` (
  `platform` varchar(255) NOT NULL DEFAULT 'system',
  `menu` text NOT NULL,
  `displayorder` smallint(6) NOT NULL DEFAULT 0,
  PRIMARY KEY (`platform`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_common_admincp_menu_platform` WRITE;
/*!40000 ALTER TABLE `pre_common_admincp_menu_platform` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_common_admincp_menu_platform` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_common_admincp_perm`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_common_admincp_perm` (
  `cpgroupid` smallint(6) unsigned NOT NULL,
  `perm` varchar(100) NOT NULL,
  UNIQUE KEY `cpgroupperm` (`cpgroupid`,`perm`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_common_admincp_perm` WRITE;
/*!40000 ALTER TABLE `pre_common_admincp_perm` DISABLE KEYS */;
INSERT INTO `pre_common_admincp_perm` VALUES
(1,'_allowpost'),
(1,'albumcategory'),
(1,'article'),
(1,'block'),
(1,'blockstyle'),
(1,'blogcategory'),
(1,'diytemplate'),
(1,'portalcategory'),
(1,'topic'),
(2,'_allowpost'),
(2,'attach'),
(2,'forums'),
(2,'forums_merge'),
(2,'misc_attachtype'),
(2,'misc_censor'),
(2,'moderate_replies'),
(2,'moderate_threads'),
(2,'prune'),
(2,'recyclebin'),
(2,'report'),
(2,'threads'),
(2,'threads_forumstick'),
(2,'threads_postposition'),
(2,'threadtypes'),
(3,'_allowpost'),
(3,'attach_group'),
(3,'group_deletegroup'),
(3,'group_editgroup'),
(3,'group_level'),
(3,'group_manage'),
(3,'group_setting'),
(3,'group_type'),
(3,'group_userperm'),
(3,'prune_group'),
(3,'threads_group'),
(4,'_allowpost'),
(4,'album'),
(4,'blog'),
(4,'click'),
(4,'comment'),
(4,'doing'),
(4,'feed'),
(4,'pic'),
(4,'setting_home'),
(4,'share'),
(5,'_allowpost'),
(5,'admingroup'),
(5,'members_access'),
(5,'members_add'),
(5,'members_ban'),
(5,'members_clean'),
(5,'members_credit'),
(5,'members_edit'),
(5,'members_group'),
(5,'members_ipban'),
(5,'members_medal'),
(5,'members_newsletter'),
(5,'members_profile'),
(5,'members_repeat'),
(5,'members_reward'),
(5,'members_search'),
(5,'members_verify'),
(5,'moderate_members'),
(5,'specialuser_defaultuser'),
(5,'specialuser_follow'),
(5,'usergroups'),
(5,'verify_verify');
/*!40000 ALTER TABLE `pre_common_admincp_perm` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_common_admincp_session`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_common_admincp_session` (
  `uid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `adminid` smallint(6) unsigned NOT NULL DEFAULT 0,
  `panel` tinyint(1) NOT NULL DEFAULT 0,
  `ip` varchar(45) NOT NULL DEFAULT '',
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  `errorcount` tinyint(1) NOT NULL DEFAULT 0,
  `storage` mediumtext NOT NULL,
  PRIMARY KEY (`uid`,`panel`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_common_admincp_session` WRITE;
/*!40000 ALTER TABLE `pre_common_admincp_session` DISABLE KEYS */;
INSERT INTO `pre_common_admincp_session` VALUES
(1,1,1,'192.168.123.79',1784453715,-1,'');
/*!40000 ALTER TABLE `pre_common_admincp_session` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_common_admingroup`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_common_admingroup` (
  `admingid` smallint(6) unsigned NOT NULL DEFAULT 0,
  `alloweditpost` tinyint(1) NOT NULL DEFAULT 0,
  `alloweditpoll` tinyint(1) NOT NULL DEFAULT 0,
  `allowstickthread` tinyint(1) NOT NULL DEFAULT 0,
  `allowmodpost` tinyint(1) NOT NULL DEFAULT 0,
  `allowdelpost` tinyint(1) NOT NULL DEFAULT 0,
  `allowmassprune` tinyint(1) NOT NULL DEFAULT 0,
  `allowrefund` tinyint(1) NOT NULL DEFAULT 0,
  `allowcensorword` tinyint(1) NOT NULL DEFAULT 0,
  `allowviewip` tinyint(1) NOT NULL DEFAULT 0,
  `allowbanip` tinyint(1) NOT NULL DEFAULT 0,
  `allowedituser` tinyint(1) NOT NULL DEFAULT 0,
  `allowmoduser` tinyint(1) NOT NULL DEFAULT 0,
  `allowbanuser` tinyint(1) NOT NULL DEFAULT 0,
  `allowbanvisituser` tinyint(1) NOT NULL DEFAULT 0,
  `allowpostannounce` tinyint(1) NOT NULL DEFAULT 0,
  `allowviewlog` tinyint(1) NOT NULL DEFAULT 0,
  `allowbanpost` tinyint(1) NOT NULL DEFAULT 0,
  `supe_allowpushthread` tinyint(1) NOT NULL DEFAULT 0,
  `allowhighlightthread` tinyint(1) NOT NULL DEFAULT 0,
  `allowlivethread` tinyint(1) NOT NULL DEFAULT 0,
  `allowdigestthread` tinyint(1) NOT NULL DEFAULT 0,
  `allowrecommendthread` tinyint(1) NOT NULL DEFAULT 0,
  `allowbumpthread` tinyint(1) NOT NULL DEFAULT 0,
  `allowclosethread` tinyint(1) NOT NULL DEFAULT 0,
  `allowmovethread` tinyint(1) NOT NULL DEFAULT 0,
  `allowedittypethread` tinyint(1) NOT NULL DEFAULT 0,
  `allowstampthread` tinyint(1) NOT NULL DEFAULT 0,
  `allowstamplist` tinyint(1) NOT NULL DEFAULT 0,
  `allowcopythread` tinyint(1) NOT NULL DEFAULT 0,
  `allowmergethread` tinyint(1) NOT NULL DEFAULT 0,
  `allowsplitthread` tinyint(1) NOT NULL DEFAULT 0,
  `allowrepairthread` tinyint(1) NOT NULL DEFAULT 0,
  `allowwarnpost` tinyint(1) NOT NULL DEFAULT 0,
  `allowviewreport` tinyint(1) NOT NULL DEFAULT 0,
  `alloweditforum` tinyint(1) NOT NULL DEFAULT 0,
  `allowremovereward` tinyint(1) NOT NULL DEFAULT 0,
  `allowedittrade` tinyint(1) NOT NULL DEFAULT 0,
  `alloweditactivity` tinyint(1) NOT NULL DEFAULT 0,
  `allowstickreply` tinyint(1) NOT NULL DEFAULT 0,
  `allowmanagearticle` tinyint(1) NOT NULL DEFAULT 0,
  `allowaddtopic` tinyint(1) NOT NULL DEFAULT 0,
  `allowmanagetopic` tinyint(1) NOT NULL DEFAULT 0,
  `allowdiy` tinyint(1) NOT NULL DEFAULT 0,
  `allowclearrecycle` tinyint(1) NOT NULL DEFAULT 0,
  `allowmanagetag` tinyint(1) NOT NULL DEFAULT 0,
  `alloweditusertag` tinyint(1) NOT NULL DEFAULT 0,
  `managefeed` tinyint(1) NOT NULL DEFAULT 0,
  `managedoing` tinyint(1) NOT NULL DEFAULT 0,
  `manageshare` tinyint(1) NOT NULL DEFAULT 0,
  `manageblog` tinyint(1) NOT NULL DEFAULT 0,
  `managealbum` tinyint(1) NOT NULL DEFAULT 0,
  `managecomment` tinyint(1) NOT NULL DEFAULT 0,
  `managemagiclog` tinyint(1) NOT NULL DEFAULT 0,
  `managereport` tinyint(1) NOT NULL DEFAULT 0,
  `managehotuser` tinyint(1) NOT NULL DEFAULT 0,
  `managedefaultuser` tinyint(1) NOT NULL DEFAULT 0,
  `managemagic` tinyint(1) NOT NULL DEFAULT 0,
  `manageclick` tinyint(1) NOT NULL DEFAULT 0,
  `allowmanagecollection` tinyint(1) NOT NULL DEFAULT 0,
  `allowmakehtml` tinyint(1) NOT NULL DEFAULT 0,
  PRIMARY KEY (`admingid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_common_admingroup` WRITE;
/*!40000 ALTER TABLE `pre_common_admingroup` DISABLE KEYS */;
INSERT INTO `pre_common_admingroup` VALUES
(1,1,1,3,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,3,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,1,1,1,1,1,1,1,1,1,1,1,1,1,1),
(2,1,0,2,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,1,1,3,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0),
(3,1,0,1,1,1,0,0,0,1,0,0,1,1,0,0,1,1,0,1,1,3,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0),
(16,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,1,0,1,1,1,1,0,0,1,0,0,0,0,0,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0),
(17,1,0,2,1,0,0,1,0,1,0,0,0,0,0,1,1,1,0,1,0,3,1,1,1,1,1,1,0,1,1,1,1,1,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0),
(18,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0),
(19,0,0,0,1,0,0,0,0,1,1,0,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
/*!40000 ALTER TABLE `pre_common_admingroup` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_common_adminnote`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_common_adminnote` (
  `id` mediumint(8) unsigned NOT NULL AUTO_INCREMENT,
  `admin` varchar(50) NOT NULL DEFAULT '',
  `access` tinyint(3) NOT NULL DEFAULT 0,
  `adminid` tinyint(3) NOT NULL DEFAULT 0,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  `expiration` int(10) unsigned NOT NULL DEFAULT 0,
  `message` text NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=2 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_common_adminnote` WRITE;
/*!40000 ALTER TABLE `pre_common_adminnote` DISABLE KEYS */;
INSERT INTO `pre_common_adminnote` VALUES
(1,'Discuz',0,0,1784453715,1787045715,'鎰熻阿鎮ㄥ畨瑁呬娇鐢?Discuz! X锛岃繖閲屾槸 Discuz! 鐨勭鐞嗕腑蹇冿紝鎮ㄥ彲浠ュ湪杩欓噷璋冩暣缃戠珯鐨勫悇椤硅缃€?);
/*!40000 ALTER TABLE `pre_common_adminnote` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_common_advertisement`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_common_advertisement` (
  `advid` mediumint(8) unsigned NOT NULL AUTO_INCREMENT,
  `available` tinyint(1) NOT NULL DEFAULT 0,
  `type` varchar(50) NOT NULL DEFAULT '0',
  `displayorder` tinyint(3) NOT NULL DEFAULT 0,
  `title` varchar(255) NOT NULL DEFAULT '',
  `targets` text NOT NULL,
  `parameters` text NOT NULL,
  `code` text NOT NULL,
  `starttime` int(10) unsigned NOT NULL DEFAULT 0,
  `endtime` int(10) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`advid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_common_advertisement` WRITE;
/*!40000 ALTER TABLE `pre_common_advertisement` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_common_advertisement` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_common_advertisement_custom`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_common_advertisement_custom` (
  `id` smallint(5) unsigned NOT NULL AUTO_INCREMENT,
  `name` varchar(255) NOT NULL,
  PRIMARY KEY (`id`),
  KEY `name` (`name`(100))
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_common_advertisement_custom` WRITE;
/*!40000 ALTER TABLE `pre_common_advertisement_custom` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_common_advertisement_custom` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_common_banned`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_common_banned` (
  `id` smallint(6) unsigned NOT NULL AUTO_INCREMENT,
  `ip` varchar(49) NOT NULL DEFAULT '',
  `lowerip` varbinary(16) NOT NULL DEFAULT '\0',
  `upperip` varbinary(16) NOT NULL DEFAULT '\0',
  `admin` varchar(50) NOT NULL DEFAULT '',
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  `expiration` int(10) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`id`),
  KEY `iprange` (`lowerip`,`upperip`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_common_banned` WRITE;
/*!40000 ALTER TABLE `pre_common_banned` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_common_banned` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_common_block`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_common_block` (
  `bid` mediumint(8) unsigned NOT NULL AUTO_INCREMENT,
  `blockclass` varchar(255) NOT NULL DEFAULT '0',
  `blocktype` tinyint(1) NOT NULL DEFAULT 0,
  `name` varchar(255) NOT NULL DEFAULT '',
  `title` text NOT NULL,
  `classname` varchar(255) NOT NULL DEFAULT '',
  `summary` text NOT NULL,
  `uid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `username` varchar(255) NOT NULL DEFAULT '',
  `styleid` smallint(6) unsigned NOT NULL DEFAULT 0,
  `blockstyle` text NOT NULL,
  `picwidth` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `picheight` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `target` varchar(255) NOT NULL DEFAULT '',
  `dateformat` varchar(255) NOT NULL DEFAULT '',
  `dateuformat` tinyint(1) NOT NULL DEFAULT 0,
  `script` varchar(255) NOT NULL DEFAULT '',
  `param` text NOT NULL,
  `shownum` smallint(6) unsigned NOT NULL DEFAULT 0,
  `cachetime` int(10) NOT NULL DEFAULT 0,
  `cachetimerange` char(5) NOT NULL DEFAULT '',
  `punctualupdate` tinyint(1) NOT NULL DEFAULT 0,
  `hidedisplay` tinyint(1) NOT NULL DEFAULT 0,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  `notinherited` tinyint(1) NOT NULL DEFAULT 0,
  `isblank` tinyint(1) NOT NULL DEFAULT 0,
  PRIMARY KEY (`bid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_common_block` WRITE;
/*!40000 ALTER TABLE `pre_common_block` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_common_block` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_common_block_favorite`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_common_block_favorite` (
  `favid` mediumint(8) unsigned NOT NULL AUTO_INCREMENT,
  `uid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `bid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`favid`),
  KEY `uid` (`uid`,`dateline`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_common_block_favorite` WRITE;
/*!40000 ALTER TABLE `pre_common_block_favorite` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_common_block_favorite` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_common_block_item`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_common_block_item` (
  `itemid` int(10) unsigned NOT NULL AUTO_INCREMENT,
  `bid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `id` int(10) unsigned NOT NULL DEFAULT 0,
  `idtype` varchar(255) NOT NULL DEFAULT '',
  `itemtype` tinyint(1) NOT NULL DEFAULT 0,
  `title` varchar(255) NOT NULL DEFAULT '',
  `url` varchar(255) NOT NULL DEFAULT '',
  `pic` varchar(255) NOT NULL DEFAULT '',
  `picflag` tinyint(1) NOT NULL DEFAULT 0,
  `makethumb` tinyint(1) NOT NULL DEFAULT 0,
  `thumbpath` varchar(255) NOT NULL DEFAULT '',
  `summary` text NOT NULL,
  `showstyle` text NOT NULL,
  `related` text NOT NULL,
  `fields` text NOT NULL,
  `displayorder` smallint(6) NOT NULL DEFAULT 0,
  `startdate` int(10) unsigned NOT NULL DEFAULT 0,
  `enddate` int(10) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`itemid`),
  KEY `bid` (`bid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_common_block_item` WRITE;
/*!40000 ALTER TABLE `pre_common_block_item` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_common_block_item` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_common_block_item_data`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_common_block_item_data` (
  `dataid` int(10) unsigned NOT NULL AUTO_INCREMENT,
  `bid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `id` int(10) unsigned NOT NULL DEFAULT 0,
  `idtype` varchar(255) NOT NULL DEFAULT '',
  `itemtype` tinyint(1) NOT NULL DEFAULT 0,
  `title` varchar(255) NOT NULL DEFAULT '',
  `url` varchar(255) NOT NULL DEFAULT '',
  `pic` varchar(255) NOT NULL DEFAULT '',
  `picflag` tinyint(1) NOT NULL DEFAULT 0,
  `makethumb` tinyint(1) NOT NULL DEFAULT 0,
  `summary` text NOT NULL,
  `showstyle` text NOT NULL,
  `related` text NOT NULL,
  `fields` text NOT NULL,
  `displayorder` smallint(6) NOT NULL DEFAULT 0,
  `startdate` int(10) unsigned NOT NULL DEFAULT 0,
  `enddate` int(10) unsigned NOT NULL DEFAULT 0,
  `uid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `username` varchar(255) NOT NULL DEFAULT '',
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  `isverified` tinyint(1) NOT NULL DEFAULT 0,
  `verifiedtime` int(10) unsigned NOT NULL DEFAULT 0,
  `stickgrade` tinyint(2) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`dataid`),
  KEY `bid` (`bid`,`stickgrade`,`displayorder`,`verifiedtime`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_common_block_item_data` WRITE;
/*!40000 ALTER TABLE `pre_common_block_item_data` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_common_block_item_data` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_common_block_permission`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_common_block_permission` (
  `bid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `uid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `allowmanage` tinyint(1) NOT NULL DEFAULT 0,
  `allowrecommend` tinyint(1) NOT NULL DEFAULT 0,
  `needverify` tinyint(1) NOT NULL DEFAULT 0,
  `inheritedtplname` varchar(255) NOT NULL DEFAULT '',
  PRIMARY KEY (`bid`,`uid`),
  KEY `uid` (`uid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_common_block_permission` WRITE;
/*!40000 ALTER TABLE `pre_common_block_permission` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_common_block_permission` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_common_block_pic`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_common_block_pic` (
  `picid` int(10) unsigned NOT NULL AUTO_INCREMENT,
  `bid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `itemid` int(10) unsigned NOT NULL DEFAULT 0,
  `pic` varchar(255) NOT NULL DEFAULT '',
  `picflag` tinyint(1) NOT NULL DEFAULT 0,
  `type` tinyint(1) NOT NULL DEFAULT 0,
  PRIMARY KEY (`picid`),
  KEY `bid` (`bid`,`itemid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_common_block_pic` WRITE;
/*!40000 ALTER TABLE `pre_common_block_pic` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_common_block_pic` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_common_block_style`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_common_block_style` (
  `styleid` smallint(6) unsigned NOT NULL AUTO_INCREMENT,
  `blockclass` varchar(255) NOT NULL DEFAULT '',
  `name` varchar(255) NOT NULL DEFAULT '',
  `template` text NOT NULL,
  `hash` varchar(255) NOT NULL DEFAULT '',
  `getpic` tinyint(1) NOT NULL DEFAULT 0,
  `getsummary` tinyint(1) NOT NULL DEFAULT 0,
  `makethumb` tinyint(1) NOT NULL DEFAULT 0,
  `settarget` tinyint(1) NOT NULL DEFAULT 0,
  `fields` text NOT NULL,
  `moreurl` tinyint(1) NOT NULL DEFAULT 0,
  PRIMARY KEY (`styleid`),
  KEY `hash` (`hash`(10)),
  KEY `blockclass` (`blockclass`(50))
) ENGINE=InnoDB AUTO_INCREMENT=111 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_common_block_style` WRITE;
/*!40000 ALTER TABLE `pre_common_block_style` DISABLE KEYS */;
INSERT INTO `pre_common_block_style` VALUES
(1,'html_html','[鍐呯疆]绌烘ā鏉?,'a:9:{s:3:\"raw\";s:0:\"\";s:6:\"footer\";s:0:\"\";s:6:\"header\";s:0:\"\";s:9:\"indexplus\";a:0:{}s:5:\"index\";a:0:{}s:9:\"orderplus\";a:0:{}s:5:\"order\";a:0:{}s:8:\"loopplus\";a:0:{}s:4:\"loop\";a:0:{}}','ee3e718a',0,0,0,0,'a:0:{}',0),
(2,'forum_forum','[鍐呯疆]鐗堝潡鍚嶇О鍒楄〃','a:9:{s:3:\"raw\";s:130:\"<div class=\"module cl xl xl1\">\r\n<ul>\r\n[loop]\r\n<li><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></li>\r\n[/loop]\r\n</ul>\r\n</div>\";s:6:\"footer\";s:0:\"\";s:6:\"header\";s:0:\"\";s:9:\"indexplus\";a:0:{}s:5:\"index\";a:0:{}s:9:\"orderplus\";a:0:{}s:5:\"order\";a:0:{}s:8:\"loopplus\";a:0:{}s:4:\"loop\";s:60:\"<li><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></li>\";}','c6c48ef5',0,0,0,1,'a:2:{i:0;s:3:\"url\";i:1;s:5:\"title\";}',0),
(3,'forum_forum','[鍐呯疆]鐗堝潡鍚嶇О锛嬫€诲笘鏁?,'a:9:{s:3:\"raw\";s:146:\"<div class=\"module cl xl xl1\">\r\n<ul>\r\n[loop]\r\n<li><em>{posts}</em><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></li>\r\n[/loop]\r\n</ul>\r\n</div>\";s:6:\"footer\";s:0:\"\";s:6:\"header\";s:0:\"\";s:9:\"indexplus\";a:0:{}s:5:\"index\";a:0:{}s:9:\"orderplus\";a:0:{}s:5:\"order\";a:0:{}s:8:\"loopplus\";a:0:{}s:4:\"loop\";s:76:\"<li><em>{posts}</em><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></li>\";}','91c25611',0,0,0,1,'a:3:{i:0;s:5:\"posts\";i:1;s:3:\"url\";i:2;s:5:\"title\";}',0),
(4,'forum_forum','[鍐呯疆]鐗堝潡鍚嶇О+鎬诲笘鏁帮紙鏈夊簭锛?,'a:9:{s:3:\"raw\";s:146:\"<div class=\"module cl xl xl1\">\r\n<ol>\r\n[loop]\r\n<li><em>{posts}</em><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></li>\r\n[/loop]\r\n</ol>\r\n</div>\";s:6:\"footer\";s:0:\"\";s:6:\"header\";s:0:\"\";s:9:\"indexplus\";a:0:{}s:5:\"index\";a:0:{}s:9:\"orderplus\";a:0:{}s:5:\"order\";a:0:{}s:8:\"loopplus\";a:0:{}s:4:\"loop\";s:76:\"<li><em>{posts}</em><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></li>\";}','951323a8',0,0,0,1,'a:3:{i:0;s:5:\"posts\";i:1;s:3:\"url\";i:2;s:5:\"title\";}',0),
(5,'forum_forum','[鍐呯疆]鐗堝潡鍚嶇О+浠婃棩鍙戣创鏁?,'a:9:{s:3:\"raw\";s:151:\"<div class=\"module cl xl xl1\">\r\n<ul>\r\n[loop]\r\n<li><em>{todayposts}</em><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></li>\r\n[/loop]\r\n</ul>\r\n</div>\";s:6:\"footer\";s:0:\"\";s:6:\"header\";s:0:\"\";s:9:\"indexplus\";a:0:{}s:5:\"index\";a:0:{}s:9:\"orderplus\";a:0:{}s:5:\"order\";a:0:{}s:8:\"loopplus\";a:0:{}s:4:\"loop\";s:81:\"<li><em>{todayposts}</em><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></li>\";}','e08c8a30',0,0,0,1,'a:3:{i:0;s:10:\"todayposts\";i:1;s:3:\"url\";i:2;s:5:\"title\";}',0),
(6,'forum_forum','[鍐呯疆]鐗堝潡鍚嶇О+浠婃棩鍙戣创鏁帮紙鏈夊簭锛?,'a:9:{s:3:\"raw\";s:151:\"<div class=\"module cl xl xl1\">\r\n<ol>\r\n[loop]\r\n<li><em>{todayposts}</em><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></li>\r\n[/loop]\r\n</ol>\r\n</div>\";s:6:\"footer\";s:0:\"\";s:6:\"header\";s:0:\"\";s:9:\"indexplus\";a:0:{}s:5:\"index\";a:0:{}s:9:\"orderplus\";a:0:{}s:5:\"order\";a:0:{}s:8:\"loopplus\";a:0:{}s:4:\"loop\";s:81:\"<li><em>{todayposts}</em><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></li>\";}','12516b2d',0,0,0,1,'a:3:{i:0;s:10:\"todayposts\";i:1;s:3:\"url\";i:2;s:5:\"title\";}',0),
(7,'forum_forum','[鍐呯疆]鐗堝潡鍚嶇О锛堜袱鍒楋級','a:9:{s:3:\"raw\";s:130:\"<div class=\"module cl xl xl2\">\r\n<ul>\r\n[loop]\r\n<li><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></li>\r\n[/loop]\r\n</ul>\r\n</div>\";s:6:\"footer\";s:0:\"\";s:6:\"header\";s:0:\"\";s:9:\"indexplus\";a:0:{}s:5:\"index\";a:0:{}s:9:\"orderplus\";a:0:{}s:5:\"order\";a:0:{}s:8:\"loopplus\";a:0:{}s:4:\"loop\";s:60:\"<li><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></li>\";}','0e51a193',0,0,0,1,'a:2:{i:0;s:3:\"url\";i:1;s:5:\"title\";}',0),
(8,'forum_forum','[鍐呯疆]鐗堝潡鍚嶇О锛嬩粙缁?,'a:9:{s:3:\"raw\";s:160:\"<div class=\"module cl xld\">\r\n[loop]\r\n<dl class=\"cl\">\r\n	<dt><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></dt>\r\n	<dd>{summary}</dd>\r\n</dl>\r\n[/loop]\r\n</div>\";s:6:\"footer\";s:0:\"\";s:6:\"header\";s:0:\"\";s:9:\"indexplus\";a:0:{}s:5:\"index\";a:0:{}s:9:\"orderplus\";a:0:{}s:5:\"order\";a:0:{}s:8:\"loopplus\";a:0:{}s:4:\"loop\";s:106:\"<dl class=\"cl\">\r\n	<dt><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></dt>\r\n	<dd>{summary}</dd>\r\n</dl>\";}','2bf344ae',0,1,0,1,'a:3:{i:0;s:3:\"url\";i:1;s:5:\"title\";i:2;s:7:\"summary\";}',0),
(9,'forum_thread','[鍐呯疆]甯栧瓙鏍囬','a:9:{s:3:\"raw\";s:130:\"<div class=\"module cl xl xl1\">\r\n<ul>\r\n[loop]\r\n<li><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></li>\r\n[/loop]\r\n</ul>\r\n</div>\";s:6:\"footer\";s:0:\"\";s:6:\"header\";s:0:\"\";s:9:\"indexplus\";a:0:{}s:5:\"index\";a:0:{}s:9:\"orderplus\";a:0:{}s:5:\"order\";a:0:{}s:8:\"loopplus\";a:0:{}s:4:\"loop\";s:60:\"<li><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></li>\";}','079cd140',0,0,0,1,'a:2:{i:0;s:3:\"url\";i:1;s:5:\"title\";}',0),
(10,'forum_thread','[鍐呯疆]甯栧瓙鏍囬+鍥炲鏁?,'a:9:{s:3:\"raw\";s:148:\"<div class=\"module cl xl xl1\">\r\n<ul>\r\n[loop]\r\n<li><em>{replies}</em><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></li>\r\n[/loop]\r\n</ul>\r\n</div>\";s:6:\"footer\";s:0:\"\";s:6:\"header\";s:0:\"\";s:9:\"indexplus\";a:0:{}s:5:\"index\";a:0:{}s:9:\"orderplus\";a:0:{}s:5:\"order\";a:0:{}s:8:\"loopplus\";a:0:{}s:4:\"loop\";s:78:\"<li><em>{replies}</em><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></li>\";}','0cc45858',0,0,0,1,'a:3:{i:0;s:7:\"replies\";i:1;s:3:\"url\";i:2;s:5:\"title\";}',0),
(11,'forum_thread','[鍐呯疆]甯栧瓙鏍囬+鏌ョ湅鏁?,'a:9:{s:3:\"raw\";s:146:\"<div class=\"module cl xl xl1\">\r\n<ul>\r\n[loop]\r\n<li><em>{views}</em><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></li>\r\n[/loop]\r\n</ul>\r\n</div>\";s:6:\"footer\";s:0:\"\";s:6:\"header\";s:0:\"\";s:9:\"indexplus\";a:0:{}s:5:\"index\";a:0:{}s:9:\"orderplus\";a:0:{}s:5:\"order\";a:0:{}s:8:\"loopplus\";a:0:{}s:4:\"loop\";s:76:\"<li><em>{views}</em><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></li>\";}','c5361e32',0,0,0,1,'a:3:{i:0;s:5:\"views\";i:1;s:3:\"url\";i:2;s:5:\"title\";}',0),
(12,'forum_thread','[鍐呯疆]甯栧瓙鏍囬+鐑害','a:9:{s:3:\"raw\";s:146:\"<div class=\"module cl xl xl1\">\r\n<ul>\r\n[loop]\r\n<li><em>{heats}</em><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></li>\r\n[/loop]\r\n</ul>\r\n</div>\";s:6:\"footer\";s:0:\"\";s:6:\"header\";s:0:\"\";s:9:\"indexplus\";a:0:{}s:5:\"index\";a:0:{}s:9:\"orderplus\";a:0:{}s:5:\"order\";a:0:{}s:8:\"loopplus\";a:0:{}s:4:\"loop\";s:76:\"<li><em>{heats}</em><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></li>\";}','dfac2b4f',0,0,0,1,'a:3:{i:0;s:5:\"heats\";i:1;s:3:\"url\";i:2;s:5:\"title\";}',0),
(13,'forum_thread','[鍐呯疆]甯栧瓙鏍囬+鍙戝笘鏃堕棿','a:9:{s:3:\"raw\";s:149:\"<div class=\"module cl xl xl1\">\r\n<ul>\r\n[loop]\r\n<li><em>{dateline}</em><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></li>\r\n[/loop]\r\n</ul>\r\n</div>\";s:6:\"footer\";s:0:\"\";s:6:\"header\";s:0:\"\";s:9:\"indexplus\";a:0:{}s:5:\"index\";a:0:{}s:9:\"orderplus\";a:0:{}s:5:\"order\";a:0:{}s:8:\"loopplus\";a:0:{}s:4:\"loop\";s:79:\"<li><em>{dateline}</em><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></li>\";}','37a3603a',0,0,0,1,'a:3:{i:0;s:8:\"dateline\";i:1;s:3:\"url\";i:2;s:5:\"title\";}',0),
(14,'forum_thread','[鍐呯疆]甯栧瓙鏍囬+鏈€鍚庡洖澶嶆椂闂?,'a:9:{s:3:\"raw\";s:149:\"<div class=\"module cl xl xl1\">\r\n<ul>\r\n[loop]\r\n<li><em>{lastpost}</em><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></li>\r\n[/loop]\r\n</ul>\r\n</div>\";s:6:\"footer\";s:0:\"\";s:6:\"header\";s:0:\"\";s:9:\"indexplus\";a:0:{}s:5:\"index\";a:0:{}s:9:\"orderplus\";a:0:{}s:5:\"order\";a:0:{}s:8:\"loopplus\";a:0:{}s:4:\"loop\";s:79:\"<li><em>{lastpost}</em><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></li>\";}','1ae9c85b',0,0,0,1,'a:3:{i:0;s:8:\"lastpost\";i:1;s:3:\"url\";i:2;s:5:\"title\";}',0),
(15,'forum_thread','[鍐呯疆]甯栧瓙鏍囬+浣滆€?,'a:9:{s:3:\"raw\";s:203:\"<div class=\"module cl xl xl1\">\r\n<ul>\r\n[loop]\r\n<li><em><a href=\"home.php?mod=space&uid={authorid}\"{target}>{author}</a></em><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></li>\r\n[/loop]\r\n</ul>\r\n</div>\";s:6:\"footer\";s:0:\"\";s:6:\"header\";s:0:\"\";s:9:\"indexplus\";a:0:{}s:5:\"index\";a:0:{}s:9:\"orderplus\";a:0:{}s:5:\"order\";a:0:{}s:8:\"loopplus\";a:0:{}s:4:\"loop\";s:133:\"<li><em><a href=\"home.php?mod=space&uid={authorid}\"{target}>{author}</a></em><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></li>\";}','30def87f',0,0,0,1,'a:4:{i:0;s:8:\"authorid\";i:1;s:6:\"author\";i:2;s:3:\"url\";i:3;s:5:\"title\";}',0),
(16,'forum_thread','[鍐呯疆]甯栧瓙鏍囬+浣滆€?鎽樿','a:9:{s:3:\"raw\";s:251:\"<div class=\"module cl xld\">\r\n[loop]\r\n<dl class=\"cl\">\r\n	<dt><em class=\"y xg1 xw0\"><a href=\"home.php?mod=space&uid={authorid}\"{target}>{author}</a></em><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></dt>\r\n	<dd>{summary}</dd>\r\n</dl>\r\n[/loop]\r\n</div>\";s:6:\"footer\";s:0:\"\";s:6:\"header\";s:0:\"\";s:9:\"indexplus\";a:0:{}s:5:\"index\";a:0:{}s:9:\"orderplus\";a:0:{}s:5:\"order\";a:0:{}s:8:\"loopplus\";a:0:{}s:4:\"loop\";s:197:\"<dl class=\"cl\">\r\n	<dt><em class=\"y xg1 xw0\"><a href=\"home.php?mod=space&uid={authorid}\"{target}>{author}</a></em><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></dt>\r\n	<dd>{summary}</dd>\r\n</dl>\";}','8ebc8d5f',0,1,0,1,'a:5:{i:0;s:8:\"authorid\";i:1;s:6:\"author\";i:2;s:3:\"url\";i:3;s:5:\"title\";i:4;s:7:\"summary\";}',0),
(17,'forum_thread','[鍐呯疆]甯栧瓙鏍囬+鎽樿','a:9:{s:3:\"raw\";s:160:\"<div class=\"module cl xld\">\r\n[loop]\r\n<dl class=\"cl\">\r\n	<dt><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></dt>\r\n	<dd>{summary}</dd>\r\n</dl>\r\n[/loop]\r\n</div>\";s:6:\"footer\";s:0:\"\";s:6:\"header\";s:0:\"\";s:9:\"indexplus\";a:0:{}s:5:\"index\";a:0:{}s:9:\"orderplus\";a:0:{}s:5:\"order\";a:0:{}s:8:\"loopplus\";a:0:{}s:4:\"loop\";s:106:\"<dl class=\"cl\">\r\n	<dt><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></dt>\r\n	<dd>{summary}</dd>\r\n</dl>\";}','1107d2bd',0,1,0,1,'a:3:{i:0;s:3:\"url\";i:1;s:5:\"title\";i:2;s:7:\"summary\";}',0),
(18,'forum_thread','[鍐呯疆]鐒︾偣妯″紡','a:9:{s:3:\"raw\";s:164:\"<div class=\"module cl xld fcs\">\r\n[loop]\r\n<dl class=\"cl\">\r\n	<dt><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></dt>\r\n	<dd>{summary}</dd>\r\n</dl>\r\n[/loop]\r\n</div>\";s:6:\"footer\";s:0:\"\";s:6:\"header\";s:0:\"\";s:9:\"indexplus\";a:0:{}s:5:\"index\";a:0:{}s:9:\"orderplus\";a:0:{}s:5:\"order\";a:0:{}s:8:\"loopplus\";a:0:{}s:4:\"loop\";s:106:\"<dl class=\"cl\">\r\n	<dt><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></dt>\r\n	<dd>{summary}</dd>\r\n</dl>\";}','b6337920',0,1,0,1,'a:3:{i:0;s:3:\"url\";i:1;s:5:\"title\";i:2;s:7:\"summary\";}',0),
(19,'forum_thread','[鍐呯疆]甯栧瓙鏍囬锛堢涓€鏉″甫鎽樿锛?,'a:9:{s:3:\"raw\";s:297:\"<div class=\"module cl xl\">\r\n<ul>\r\n[loop]\r\n<li><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></li>\r\n[/loop]\r\n[order=1]\r\n<li>\r\n	<dl class=\"cl xld\">\r\n		<dt><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></dt>\r\n		<dd>{summary}</dd>\r\n	</dl> \r\n	<hr class=\"da\" />\r\n</li>\r\n[/order]\r\n</ul>\r\n</div>\";s:6:\"footer\";s:0:\"\";s:6:\"header\";s:0:\"\";s:9:\"indexplus\";a:0:{}s:5:\"index\";a:0:{}s:9:\"orderplus\";a:0:{}s:5:\"order\";a:1:{i:1;s:148:\"<li>\r\n	<dl class=\"cl xld\">\r\n		<dt><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></dt>\r\n		<dd>{summary}</dd>\r\n	</dl> \r\n	<hr class=\"da\" />\r\n</li>\";}s:8:\"loopplus\";a:0:{}s:4:\"loop\";s:60:\"<li><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></li>\";}','2e06f8b5',0,1,0,1,'a:3:{i:0;s:3:\"url\";i:1;s:5:\"title\";i:2;s:7:\"summary\";}',0),
(24,'group_thread','[鍐呯疆]甯栧瓙鏍囬','a:9:{s:3:\"raw\";s:130:\"<div class=\"module cl xl xl1\">\r\n<ul>\r\n[loop]\r\n<li><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></li>\r\n[/loop]\r\n</ul>\r\n</div>\";s:6:\"footer\";s:0:\"\";s:6:\"header\";s:0:\"\";s:9:\"indexplus\";a:0:{}s:5:\"index\";a:0:{}s:9:\"orderplus\";a:0:{}s:5:\"order\";a:0:{}s:8:\"loopplus\";a:0:{}s:4:\"loop\";s:60:\"<li><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></li>\";}','176fcc68',0,0,0,1,'a:2:{i:0;s:3:\"url\";i:1;s:5:\"title\";}',0),
(25,'group_thread','[鍐呯疆]甯栧瓙鏍囬+鍥炲鏁?,'a:9:{s:3:\"raw\";s:148:\"<div class=\"module cl xl xl1\">\r\n<ul>\r\n[loop]\r\n<li><em>{replies}</em><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></li>\r\n[/loop]\r\n</ul>\r\n</div>\";s:6:\"footer\";s:0:\"\";s:6:\"header\";s:0:\"\";s:9:\"indexplus\";a:0:{}s:5:\"index\";a:0:{}s:9:\"orderplus\";a:0:{}s:5:\"order\";a:0:{}s:8:\"loopplus\";a:0:{}s:4:\"loop\";s:78:\"<li><em>{replies}</em><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></li>\";}','8baa57ad',0,0,0,1,'a:3:{i:0;s:7:\"replies\";i:1;s:3:\"url\";i:2;s:5:\"title\";}',0),
(26,'group_thread','[鍐呯疆]甯栧瓙鏍囬+鏌ョ湅鏁?,'a:9:{s:3:\"raw\";s:146:\"<div class=\"module cl xl xl1\">\r\n<ul>\r\n[loop]\r\n<li><em>{views}</em><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></li>\r\n[/loop]\r\n</ul>\r\n</div>\";s:6:\"footer\";s:0:\"\";s:6:\"header\";s:0:\"\";s:9:\"indexplus\";a:0:{}s:5:\"index\";a:0:{}s:9:\"orderplus\";a:0:{}s:5:\"order\";a:0:{}s:8:\"loopplus\";a:0:{}s:4:\"loop\";s:76:\"<li><em>{views}</em><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></li>\";}','8f012db4',0,0,0,1,'a:3:{i:0;s:5:\"views\";i:1;s:3:\"url\";i:2;s:5:\"title\";}',0),
(27,'group_thread','[鍐呯疆]甯栧瓙鏍囬+鐑害','a:9:{s:3:\"raw\";s:146:\"<div class=\"module cl xl xl1\">\r\n<ul>\r\n[loop]\r\n<li><em>{heats}</em><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></li>\r\n[/loop]\r\n</ul>\r\n</div>\";s:6:\"footer\";s:0:\"\";s:6:\"header\";s:0:\"\";s:9:\"indexplus\";a:0:{}s:5:\"index\";a:0:{}s:9:\"orderplus\";a:0:{}s:5:\"order\";a:0:{}s:8:\"loopplus\";a:0:{}s:4:\"loop\";s:76:\"<li><em>{heats}</em><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></li>\";}','7f002523',0,0,0,1,'a:3:{i:0;s:5:\"heats\";i:1;s:3:\"url\";i:2;s:5:\"title\";}',0),
(28,'group_thread','[鍐呯疆]甯栧瓙鏍囬+鍙戝笘鏃堕棿','a:9:{s:3:\"raw\";s:149:\"<div class=\"module cl xl xl1\">\r\n<ul>\r\n[loop]\r\n<li><em>{dateline}</em><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></li>\r\n[/loop]\r\n</ul>\r\n</div>\";s:6:\"footer\";s:0:\"\";s:6:\"header\";s:0:\"\";s:9:\"indexplus\";a:0:{}s:5:\"index\";a:0:{}s:9:\"orderplus\";a:0:{}s:5:\"order\";a:0:{}s:8:\"loopplus\";a:0:{}s:4:\"loop\";s:79:\"<li><em>{dateline}</em><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></li>\";}','23ba8554',0,0,0,1,'a:3:{i:0;s:8:\"dateline\";i:1;s:3:\"url\";i:2;s:5:\"title\";}',0),
(29,'group_thread','[鍐呯疆]甯栧瓙鏍囬+鏈€鍚庡洖澶嶆椂闂?,'a:9:{s:3:\"raw\";s:149:\"<div class=\"module cl xl xl1\">\r\n<ul>\r\n[loop]\r\n<li><em>{lastpost}</em><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></li>\r\n[/loop]\r\n</ul>\r\n</div>\";s:6:\"footer\";s:0:\"\";s:6:\"header\";s:0:\"\";s:9:\"indexplus\";a:0:{}s:5:\"index\";a:0:{}s:9:\"orderplus\";a:0:{}s:5:\"order\";a:0:{}s:8:\"loopplus\";a:0:{}s:4:\"loop\";s:79:\"<li><em>{lastpost}</em><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></li>\";}','a6fbd13d',0,0,0,1,'a:3:{i:0;s:8:\"lastpost\";i:1;s:3:\"url\";i:2;s:5:\"title\";}',0),
(30,'group_thread','[鍐呯疆]甯栧瓙鏍囬+浣滆€?,'a:9:{s:3:\"raw\";s:203:\"<div class=\"module cl xl xl1\">\r\n<ul>\r\n[loop]\r\n<li><em><a href=\"home.php?mod=space&uid={authorid}\"{target}>{author}</a></em><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></li>\r\n[/loop]\r\n</ul>\r\n</div>\";s:6:\"footer\";s:0:\"\";s:6:\"header\";s:0:\"\";s:9:\"indexplus\";a:0:{}s:5:\"index\";a:0:{}s:9:\"orderplus\";a:0:{}s:5:\"order\";a:0:{}s:8:\"loopplus\";a:0:{}s:4:\"loop\";s:133:\"<li><em><a href=\"home.php?mod=space&uid={authorid}\"{target}>{author}</a></em><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></li>\";}','49245e40',0,0,0,1,'a:4:{i:0;s:8:\"authorid\";i:1;s:6:\"author\";i:2;s:3:\"url\";i:3;s:5:\"title\";}',0),
(31,'group_thread','[鍐呯疆]甯栧瓙鏍囬+浣滆€?鎽樿','a:9:{s:3:\"raw\";s:243:\"<div class=\"module cl xld\">\r\n[loop]\r\n<dl class=\"cl\">\r\n	<dt><em class=\"y\"><a href=\"home.php?mod=space&uid={authorid}\"{target}>{author}</a></em><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></dt>\r\n	<dd>{summary}</dd>\r\n</dl>\r\n[/loop]\r\n</div>\";s:6:\"footer\";s:0:\"\";s:6:\"header\";s:0:\"\";s:9:\"indexplus\";a:0:{}s:5:\"index\";a:0:{}s:9:\"orderplus\";a:0:{}s:5:\"order\";a:0:{}s:8:\"loopplus\";a:0:{}s:4:\"loop\";s:189:\"<dl class=\"cl\">\r\n	<dt><em class=\"y\"><a href=\"home.php?mod=space&uid={authorid}\"{target}>{author}</a></em><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></dt>\r\n	<dd>{summary}</dd>\r\n</dl>\";}','d9c23f31',0,1,0,1,'a:5:{i:0;s:8:\"authorid\";i:1;s:6:\"author\";i:2;s:3:\"url\";i:3;s:5:\"title\";i:4;s:7:\"summary\";}',0),
(32,'group_thread','[鍐呯疆]甯栧瓙鏍囬+鎽樿','a:9:{s:3:\"raw\";s:160:\"<div class=\"module cl xld\">\r\n[loop]\r\n<dl class=\"cl\">\r\n	<dt><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></dt>\r\n	<dd>{summary}</dd>\r\n</dl>\r\n[/loop]\r\n</div>\";s:6:\"footer\";s:0:\"\";s:6:\"header\";s:0:\"\";s:9:\"indexplus\";a:0:{}s:5:\"index\";a:0:{}s:9:\"orderplus\";a:0:{}s:5:\"order\";a:0:{}s:8:\"loopplus\";a:0:{}s:4:\"loop\";s:106:\"<dl class=\"cl\">\r\n	<dt><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></dt>\r\n	<dd>{summary}</dd>\r\n</dl>\";}','9e90211d',0,1,0,1,'a:3:{i:0;s:3:\"url\";i:1;s:5:\"title\";i:2;s:7:\"summary\";}',0),
(33,'group_thread','[鍐呯疆]鐒︾偣妯″紡','a:9:{s:3:\"raw\";s:164:\"<div class=\"module cl xld fcs\">\r\n[loop]\r\n<dl class=\"cl\">\r\n	<dt><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></dt>\r\n	<dd>{summary}</dd>\r\n</dl>\r\n[/loop]\r\n</div>\";s:6:\"footer\";s:0:\"\";s:6:\"header\";s:0:\"\";s:9:\"indexplus\";a:0:{}s:5:\"index\";a:0:{}s:9:\"orderplus\";a:0:{}s:5:\"order\";a:0:{}s:8:\"loopplus\";a:0:{}s:4:\"loop\";s:106:\"<dl class=\"cl\">\r\n	<dt><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></dt>\r\n	<dd>{summary}</dd>\r\n</dl>\";}','9670c626',0,1,0,1,'a:3:{i:0;s:3:\"url\";i:1;s:5:\"title\";i:2;s:7:\"summary\";}',0),
(34,'group_thread','[鍐呯疆]甯栧瓙鏍囬锛堢涓€鏉″甫鎽樿锛?,'a:9:{s:3:\"raw\";s:297:\"<div class=\"module cl xl\">\r\n<ul>\r\n[loop]\r\n<li><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></li>\r\n[/loop]\r\n[order=1]\r\n<li>\r\n	<dl class=\"cl xld\">\r\n		<dt><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></dt>\r\n		<dd>{summary}</dd>\r\n	</dl> \r\n	<hr class=\"da\" />\r\n</li>\r\n[/order]\r\n</ul>\r\n</div>\";s:6:\"footer\";s:0:\"\";s:6:\"header\";s:0:\"\";s:9:\"indexplus\";a:0:{}s:5:\"index\";a:0:{}s:9:\"orderplus\";a:0:{}s:5:\"order\";a:1:{i:1;s:148:\"<li>\r\n	<dl class=\"cl xld\">\r\n		<dt><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></dt>\r\n		<dd>{summary}</dd>\r\n	</dl> \r\n	<hr class=\"da\" />\r\n</li>\";}s:8:\"loopplus\";a:0:{}s:4:\"loop\";s:60:\"<li><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></li>\";}','9355f559',0,1,0,1,'a:3:{i:0;s:3:\"url\";i:1;s:5:\"title\";i:2;s:7:\"summary\";}',0),
(39,'group_group','[鍐呯疆]鍦堝瓙鍚嶇О','a:9:{s:3:\"raw\";s:130:\"<div class=\"module cl xl xl1\">\r\n<ul>\r\n[loop]\r\n<li><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></li>\r\n[/loop]\r\n</ul>\r\n</div>\";s:6:\"footer\";s:0:\"\";s:6:\"header\";s:0:\"\";s:9:\"indexplus\";a:0:{}s:5:\"index\";a:0:{}s:9:\"orderplus\";a:0:{}s:5:\"order\";a:0:{}s:8:\"loopplus\";a:0:{}s:4:\"loop\";s:60:\"<li><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></li>\";}','9872d550',0,0,0,1,'a:2:{i:0;s:3:\"url\";i:1;s:5:\"title\";}',0),
(40,'group_group','[鍐呯疆]鍦堝瓙鍚嶇О+鎴愬憳鏁?,'a:9:{s:3:\"raw\";s:150:\"<div class=\"module cl xl xl1\">\r\n<ul>\r\n[loop]\r\n<li><em>{membernum}</em><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></li>\r\n[/loop]\r\n</ul>\r\n</div>\";s:6:\"footer\";s:0:\"\";s:6:\"header\";s:0:\"\";s:9:\"indexplus\";a:0:{}s:5:\"index\";a:0:{}s:9:\"orderplus\";a:0:{}s:5:\"order\";a:0:{}s:8:\"loopplus\";a:0:{}s:4:\"loop\";s:80:\"<li><em>{membernum}</em><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></li>\";}','20a09ec8',0,0,0,1,'a:3:{i:0;s:9:\"membernum\";i:1;s:3:\"url\";i:2;s:5:\"title\";}',0),
(41,'group_group','[鍐呯疆]鍦堝瓙鍚嶇О+鎴愬憳鏁帮紙鏈夊簭锛?,'a:9:{s:3:\"raw\";s:150:\"<div class=\"module cl xl xl1\">\r\n<ol>\r\n[loop]\r\n<li><em>{membernum}</em><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></li>\r\n[/loop]\r\n</ol>\r\n</div>\";s:6:\"footer\";s:0:\"\";s:6:\"header\";s:0:\"\";s:9:\"indexplus\";a:0:{}s:5:\"index\";a:0:{}s:9:\"orderplus\";a:0:{}s:5:\"order\";a:0:{}s:8:\"loopplus\";a:0:{}s:4:\"loop\";s:80:\"<li><em>{membernum}</em><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></li>\";}','af166b44',0,0,0,1,'a:3:{i:0;s:9:\"membernum\";i:1;s:3:\"url\";i:2;s:5:\"title\";}',0),
(42,'group_group','[鍐呯疆]鍦堝瓙鍚嶇О+鎬诲笘鏁?,'a:9:{s:3:\"raw\";s:146:\"<div class=\"module cl xl xl1\">\r\n<ul>\r\n[loop]\r\n<li><em>{posts}</em><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></li>\r\n[/loop]\r\n</ul>\r\n</div>\";s:6:\"footer\";s:0:\"\";s:6:\"header\";s:0:\"\";s:9:\"indexplus\";a:0:{}s:5:\"index\";a:0:{}s:9:\"orderplus\";a:0:{}s:5:\"order\";a:0:{}s:8:\"loopplus\";a:0:{}s:4:\"loop\";s:76:\"<li><em>{posts}</em><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></li>\";}','43ed1e7c',0,0,0,1,'a:3:{i:0;s:5:\"posts\";i:1;s:3:\"url\";i:2;s:5:\"title\";}',0),
(43,'group_group','[鍐呯疆]鍦堝瓙鍚嶇О+浠婃棩鍙戣创鏁?,'a:9:{s:3:\"raw\";s:151:\"<div class=\"module cl xl xl1\">\r\n<ul>\r\n[loop]\r\n<li><em>{todayposts}</em><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></li>\r\n[/loop]\r\n</ul>\r\n</div>\";s:6:\"footer\";s:0:\"\";s:6:\"header\";s:0:\"\";s:9:\"indexplus\";a:0:{}s:5:\"index\";a:0:{}s:9:\"orderplus\";a:0:{}s:5:\"order\";a:0:{}s:8:\"loopplus\";a:0:{}s:4:\"loop\";s:81:\"<li><em>{todayposts}</em><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></li>\";}','3c59217b',0,0,0,1,'a:3:{i:0;s:10:\"todayposts\";i:1;s:3:\"url\";i:2;s:5:\"title\";}',0),
(44,'group_group','[鍐呯疆]鍦堝瓙鍥炬爣+鍚嶇О+浠嬬粛','a:9:{s:3:\"raw\";s:253:\"<div class=\"module cl xld\">\r\n[loop]\r\n<dl class=\"cl\">\r\n	<dd class=\"m\"><a href=\"{url}\"{target}><img src=\"{icon}\" width=\"48\" height=\"48\" /></a></dd>\r\n	<dt><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></dt>\r\n	<dd>{summary}</dd>\r\n</dl>\r\n[/loop]\r\n</div>\";s:6:\"footer\";s:0:\"\";s:6:\"header\";s:0:\"\";s:9:\"indexplus\";a:0:{}s:5:\"index\";a:0:{}s:9:\"orderplus\";a:0:{}s:5:\"order\";a:0:{}s:8:\"loopplus\";a:0:{}s:4:\"loop\";s:199:\"<dl class=\"cl\">\r\n	<dd class=\"m\"><a href=\"{url}\"{target}><img src=\"{icon}\" width=\"48\" height=\"48\" /></a></dd>\r\n	<dt><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></dt>\r\n	<dd>{summary}</dd>\r\n</dl>\";}','6f470107',0,1,0,1,'a:4:{i:0;s:3:\"url\";i:1;s:4:\"icon\";i:2;s:5:\"title\";i:3;s:7:\"summary\";}',0),
(45,'group_group','[鍐呯疆]鍦堝瓙鍥炬爣鍒楄〃','a:9:{s:3:\"raw\";s:208:\"<div class=\"module cl ml mls\">\r\n<ul>\r\n[loop]\r\n<li><a href=\"{url}\"{target}><img src=\"{icon}\" width=\"48\" height=\"48\" /></a><p><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></p></li>\r\n[/loop]\r\n</ul>\r\n</div>\";s:6:\"footer\";s:0:\"\";s:6:\"header\";s:0:\"\";s:9:\"indexplus\";a:0:{}s:5:\"index\";a:0:{}s:9:\"orderplus\";a:0:{}s:5:\"order\";a:0:{}s:8:\"loopplus\";a:0:{}s:4:\"loop\";s:138:\"<li><a href=\"{url}\"{target}><img src=\"{icon}\" width=\"48\" height=\"48\" /></a><p><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></p></li>\";}','f3646b2a',0,0,0,1,'a:3:{i:0;s:3:\"url\";i:1;s:4:\"icon\";i:2;s:5:\"title\";}',0),
(46,'group_group','[鍐呯疆]鍦堝瓙鍚嶇О锛堜袱鍒楋級','a:9:{s:3:\"raw\";s:130:\"<div class=\"module cl xl xl2\">\r\n<ul>\r\n[loop]\r\n<li><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></li>\r\n[/loop]\r\n</ul>\r\n</div>\";s:6:\"footer\";s:0:\"\";s:6:\"header\";s:0:\"\";s:9:\"indexplus\";a:0:{}s:5:\"index\";a:0:{}s:9:\"orderplus\";a:0:{}s:5:\"order\";a:0:{}s:8:\"loopplus\";a:0:{}s:4:\"loop\";s:60:\"<li><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></li>\";}','5279d89d',0,0,0,1,'a:2:{i:0;s:3:\"url\";i:1;s:5:\"title\";}',0),
(47,'portal_article','[鍐呯疆]鏂囩珷鏍囬','a:9:{s:3:\"raw\";s:130:\"<div class=\"module cl xl xl1\">\r\n<ul>\r\n[loop]\r\n<li><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></li>\r\n[/loop]\r\n</ul>\r\n</div>\";s:6:\"footer\";s:0:\"\";s:6:\"header\";s:0:\"\";s:9:\"indexplus\";a:0:{}s:5:\"index\";a:0:{}s:9:\"orderplus\";a:0:{}s:5:\"order\";a:0:{}s:8:\"loopplus\";a:0:{}s:4:\"loop\";s:60:\"<li><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></li>\";}','527a563d',0,0,0,1,'a:2:{i:0;s:3:\"url\";i:1;s:5:\"title\";}',0),
(48,'portal_article','[鍐呯疆]鏂囩珷鏍囬+鏃堕棿','a:9:{s:3:\"raw\";s:149:\"<div class=\"module cl xl xl1\">\r\n<ul>\r\n[loop]\r\n<li><em>{dateline}</em><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></li>\r\n[/loop]\r\n</ul>\r\n</div>\";s:6:\"footer\";s:0:\"\";s:6:\"header\";s:0:\"\";s:9:\"indexplus\";a:0:{}s:5:\"index\";a:0:{}s:9:\"orderplus\";a:0:{}s:5:\"order\";a:0:{}s:8:\"loopplus\";a:0:{}s:4:\"loop\";s:79:\"<li><em>{dateline}</em><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></li>\";}','6e4be436',0,0,0,1,'a:3:{i:0;s:8:\"dateline\";i:1;s:3:\"url\";i:2;s:5:\"title\";}',0),
(49,'portal_article','[鍐呯疆]鏂囩珷鏍囬+鏃堕棿锛堝甫鏍忕洰锛?,'a:9:{s:3:\"raw\";s:206:\"<div class=\"module cl xl xl1\">\r\n<ul>\r\n[loop]\r\n<li><em>{dateline}</em><label>[<a href=\"{caturl}\"{target}>{catname}</a>]</label><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></li>\r\n[/loop]\r\n</ul>\r\n</div>\";s:6:\"footer\";s:0:\"\";s:6:\"header\";s:0:\"\";s:9:\"indexplus\";a:0:{}s:5:\"index\";a:0:{}s:9:\"orderplus\";a:0:{}s:5:\"order\";a:0:{}s:8:\"loopplus\";a:0:{}s:4:\"loop\";s:136:\"<li><em>{dateline}</em><label>[<a href=\"{caturl}\"{target}>{catname}</a>]</label><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></li>\";}','c3b98a2f',0,0,0,1,'a:5:{i:0;s:8:\"dateline\";i:1;s:6:\"caturl\";i:2;s:7:\"catname\";i:3;s:3:\"url\";i:4;s:5:\"title\";}',0),
(50,'portal_article','[鍐呯疆]鏂囩珷鏍囬+鎽樿+缂╃暐鍥?,'a:9:{s:3:\"raw\";s:279:\"<div class=\"module cl xld\">\r\n[loop]\r\n<dl class=\"cl\">\r\n	<dd class=\"m\"><a href=\"{url}\"{target}><img src=\"{pic}\" width=\"{picwidth}\" height=\"{picheight}\" alt=\"{title}\" /></dd>\r\n	<dt><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></dt>\r\n	<dd>{summary}</dd>\r\n</dl>\r\n[/loop]\r\n</div>\";s:6:\"footer\";s:0:\"\";s:6:\"header\";s:0:\"\";s:9:\"indexplus\";a:0:{}s:5:\"index\";a:0:{}s:9:\"orderplus\";a:0:{}s:5:\"order\";a:0:{}s:8:\"loopplus\";a:0:{}s:4:\"loop\";s:225:\"<dl class=\"cl\">\r\n	<dd class=\"m\"><a href=\"{url}\"{target}><img src=\"{pic}\" width=\"{picwidth}\" height=\"{picheight}\" alt=\"{title}\" /></dd>\r\n	<dt><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></dt>\r\n	<dd>{summary}</dd>\r\n</dl>\";}','a5b550ee',1,1,1,1,'a:4:{i:0;s:3:\"url\";i:1;s:3:\"pic\";i:2;s:5:\"title\";i:3;s:7:\"summary\";}',0),
(51,'portal_article','[鍐呯疆]鏂囩珷鏍囬+鎽樿','a:9:{s:3:\"raw\";s:160:\"<div class=\"module cl xld\">\r\n[loop]\r\n<dl class=\"cl\">\r\n	<dt><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></dt>\r\n	<dd>{summary}</dd>\r\n</dl>\r\n[/loop]\r\n</div>\";s:6:\"footer\";s:0:\"\";s:6:\"header\";s:0:\"\";s:9:\"indexplus\";a:0:{}s:5:\"index\";a:0:{}s:9:\"orderplus\";a:0:{}s:5:\"order\";a:0:{}s:8:\"loopplus\";a:0:{}s:4:\"loop\";s:106:\"<dl class=\"cl\">\r\n	<dt><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></dt>\r\n	<dd>{summary}</dd>\r\n</dl>\";}','e57dbe5a',0,1,0,1,'a:3:{i:0;s:3:\"url\";i:1;s:5:\"title\";i:2;s:7:\"summary\";}',0),
(52,'portal_article','[鍐呯疆]鐒︾偣妯″紡','a:9:{s:3:\"raw\";s:164:\"<div class=\"module cl xld fcs\">\r\n[loop]\r\n<dl class=\"cl\">\r\n	<dt><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></dt>\r\n	<dd>{summary}</dd>\r\n</dl>\r\n[/loop]\r\n</div>\";s:6:\"footer\";s:0:\"\";s:6:\"header\";s:0:\"\";s:9:\"indexplus\";a:0:{}s:5:\"index\";a:0:{}s:9:\"orderplus\";a:0:{}s:5:\"order\";a:0:{}s:8:\"loopplus\";a:0:{}s:4:\"loop\";s:106:\"<dl class=\"cl\">\r\n	<dt><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></dt>\r\n	<dd>{summary}</dd>\r\n</dl>\";}','3b234c9c',0,1,0,1,'a:3:{i:0;s:3:\"url\";i:1;s:5:\"title\";i:2;s:7:\"summary\";}',0),
(53,'portal_article','[鍐呯疆]鏂囩珷鍥剧墖骞荤伅','a:9:{s:3:\"raw\";s:333:\"<div class=\"module cl slidebox\">\r\n<ul class=\"slideshow\">\r\n[loop]\r\n<li style=\"width: {picwidth}px; height: {picheight}px;\"><a href=\"{url}\"{target}><img src=\"{pic}\" width=\"{picwidth}\" height=\"{picheight}\" /></a><span class=\"title\">{title}</span></li>\r\n[/loop]\r\n</ul>\r\n</div>\r\n<script type=\"text/javascript\">\r\nrunslideshow();\r\n</script>\";s:6:\"footer\";s:0:\"\";s:6:\"header\";s:0:\"\";s:9:\"indexplus\";a:0:{}s:5:\"index\";a:0:{}s:9:\"orderplus\";a:0:{}s:5:\"order\";a:0:{}s:8:\"loopplus\";a:0:{}s:4:\"loop\";s:182:\"<li style=\"width: {picwidth}px; height: {picheight}px;\"><a href=\"{url}\"{target}><img src=\"{pic}\" width=\"{picwidth}\" height=\"{picheight}\" /></a><span class=\"title\">{title}</span></li>\";}','8ff81e35',1,0,1,1,'a:3:{i:0;s:3:\"url\";i:1;s:3:\"pic\";i:2;s:5:\"title\";}',0),
(54,'portal_article','[鍐呯疆]鏂囩珷鍥炬枃骞荤伅','a:9:{s:3:\"raw\";s:336:\"<div class=\"module cl xld slideshow\">\r\n[loop]\r\n<dl class=\"cl\">\r\n	<dd class=\"m\"><a href=\"{url}\"{target}><img src=\"{pic}\" width=\"{picwidth}\" height=\"{picheight}\" /></dd>\r\n	<dt><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></dt>\r\n	<dd>{summary}</dd>\r\n</dl>\r\n[/loop]\r\n</div>\r\n<script type=\"text/javascript\">\r\nrunslideshow();\r\n</script>\";s:6:\"footer\";s:0:\"\";s:6:\"header\";s:0:\"\";s:9:\"indexplus\";a:0:{}s:5:\"index\";a:0:{}s:9:\"orderplus\";a:0:{}s:5:\"order\";a:0:{}s:8:\"loopplus\";a:0:{}s:4:\"loop\";s:211:\"<dl class=\"cl\">\r\n	<dd class=\"m\"><a href=\"{url}\"{target}><img src=\"{pic}\" width=\"{picwidth}\" height=\"{picheight}\" /></dd>\r\n	<dt><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></dt>\r\n	<dd>{summary}</dd>\r\n</dl>\";}','d88aded4',1,1,1,1,'a:4:{i:0;s:3:\"url\";i:1;s:3:\"pic\";i:2;s:5:\"title\";i:3;s:7:\"summary\";}',0),
(55,'portal_category','[鍐呯疆]鏍忕洰鍚嶇О','a:9:{s:3:\"raw\";s:130:\"<div class=\"module cl xl xl1\">\r\n<ul>\r\n[loop]\r\n<li><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></li>\r\n[/loop]\r\n</ul>\r\n</div>\";s:6:\"footer\";s:0:\"\";s:6:\"header\";s:0:\"\";s:9:\"indexplus\";a:0:{}s:5:\"index\";a:0:{}s:9:\"orderplus\";a:0:{}s:5:\"order\";a:0:{}s:8:\"loopplus\";a:0:{}s:4:\"loop\";s:60:\"<li><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></li>\";}','6846b818',0,0,0,1,'a:2:{i:0;s:3:\"url\";i:1;s:5:\"title\";}',0),
(56,'portal_category','[鍐呯疆]鏍忕洰鍚嶇О锛堜袱鍒楋級','a:9:{s:3:\"raw\";s:130:\"<div class=\"module cl xl xl2\">\r\n<ul>\r\n[loop]\r\n<li><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></li>\r\n[/loop]\r\n</ul>\r\n</div>\";s:6:\"footer\";s:0:\"\";s:6:\"header\";s:0:\"\";s:9:\"indexplus\";a:0:{}s:5:\"index\";a:0:{}s:9:\"orderplus\";a:0:{}s:5:\"order\";a:0:{}s:8:\"loopplus\";a:0:{}s:4:\"loop\";s:60:\"<li><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></li>\";}','fa5b40c1',0,0,0,1,'a:2:{i:0;s:3:\"url\";i:1;s:5:\"title\";}',0),
(57,'portal_topic','[鍐呯疆]涓撻鍚嶇О','a:9:{s:3:\"raw\";s:130:\"<div class=\"module cl xl xl1\">\r\n<ul>\r\n[loop]\r\n<li><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></li>\r\n[/loop]\r\n</ul>\r\n</div>\";s:6:\"footer\";s:0:\"\";s:6:\"header\";s:0:\"\";s:9:\"indexplus\";a:0:{}s:5:\"index\";a:0:{}s:9:\"orderplus\";a:0:{}s:5:\"order\";a:0:{}s:8:\"loopplus\";a:0:{}s:4:\"loop\";s:60:\"<li><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></li>\";}','268501b8',0,0,0,1,'a:2:{i:0;s:3:\"url\";i:1;s:5:\"title\";}',0),
(58,'portal_topic','[鍐呯疆]涓撻鍚嶇О锛堜袱鍒楋級','a:9:{s:3:\"raw\";s:130:\"<div class=\"module cl xl xl2\">\r\n<ul>\r\n[loop]\r\n<li><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></li>\r\n[/loop]\r\n</ul>\r\n</div>\";s:6:\"footer\";s:0:\"\";s:6:\"header\";s:0:\"\";s:9:\"indexplus\";a:0:{}s:5:\"index\";a:0:{}s:9:\"orderplus\";a:0:{}s:5:\"order\";a:0:{}s:8:\"loopplus\";a:0:{}s:4:\"loop\";s:60:\"<li><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></li>\";}','b21a9795',0,0,0,1,'a:2:{i:0;s:3:\"url\";i:1;s:5:\"title\";}',0),
(59,'portal_topic','[鍐呯疆]涓撻鍚嶇О+浠嬬粛+缂╃暐鍥?,'a:9:{s:3:\"raw\";s:279:\"<div class=\"module cl xld\">\r\n[loop]\r\n<dl class=\"cl\">\r\n	<dd class=\"m\"><a href=\"{url}\"{target}><img src=\"{pic}\" width=\"{picwidth}\" height=\"{picheight}\" alt=\"{title}\" /></dd>\r\n	<dt><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></dt>\r\n	<dd>{summary}</dd>\r\n</dl>\r\n[/loop]\r\n</div>\";s:6:\"footer\";s:0:\"\";s:6:\"header\";s:0:\"\";s:9:\"indexplus\";a:0:{}s:5:\"index\";a:0:{}s:9:\"orderplus\";a:0:{}s:5:\"order\";a:0:{}s:8:\"loopplus\";a:0:{}s:4:\"loop\";s:225:\"<dl class=\"cl\">\r\n	<dd class=\"m\"><a href=\"{url}\"{target}><img src=\"{pic}\" width=\"{picwidth}\" height=\"{picheight}\" alt=\"{title}\" /></dd>\r\n	<dt><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></dt>\r\n	<dd>{summary}</dd>\r\n</dl>\";}','e07e6128',1,1,1,1,'a:4:{i:0;s:3:\"url\";i:1;s:3:\"pic\";i:2;s:5:\"title\";i:3;s:7:\"summary\";}',0),
(60,'portal_topic','[鍐呯疆]涓撻鍚嶇О+浠嬬粛','a:9:{s:3:\"raw\";s:160:\"<div class=\"module cl xld\">\r\n[loop]\r\n<dl class=\"cl\">\r\n	<dt><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></dt>\r\n	<dd>{summary}</dd>\r\n</dl>\r\n[/loop]\r\n</div>\";s:6:\"footer\";s:0:\"\";s:6:\"header\";s:0:\"\";s:9:\"indexplus\";a:0:{}s:5:\"index\";a:0:{}s:9:\"orderplus\";a:0:{}s:5:\"order\";a:0:{}s:8:\"loopplus\";a:0:{}s:4:\"loop\";s:106:\"<dl class=\"cl\">\r\n	<dt><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></dt>\r\n	<dd>{summary}</dd>\r\n</dl>\";}','573d0170',0,1,0,1,'a:3:{i:0;s:3:\"url\";i:1;s:5:\"title\";i:2;s:7:\"summary\";}',0),
(61,'portal_topic','[鍐呯疆]鐒︾偣妯″紡','a:9:{s:3:\"raw\";s:164:\"<div class=\"module cl xld fcs\">\r\n[loop]\r\n<dl class=\"cl\">\r\n	<dt><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></dt>\r\n	<dd>{summary}</dd>\r\n</dl>\r\n[/loop]\r\n</div>\";s:6:\"footer\";s:0:\"\";s:6:\"header\";s:0:\"\";s:9:\"indexplus\";a:0:{}s:5:\"index\";a:0:{}s:9:\"orderplus\";a:0:{}s:5:\"order\";a:0:{}s:8:\"loopplus\";a:0:{}s:4:\"loop\";s:106:\"<dl class=\"cl\">\r\n	<dt><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></dt>\r\n	<dd>{summary}</dd>\r\n</dl>\";}','7cc2ab53',0,1,0,1,'a:3:{i:0;s:3:\"url\";i:1;s:5:\"title\";i:2;s:7:\"summary\";}',0),
(62,'space_doing','[鍐呯疆]浣滆€?鍐呭','a:9:{s:3:\"raw\";s:202:\"<div class=\"module cl xl xl1\">\r\n<ul>\r\n[loop]\r\n<li><a href=\"home.php?mod=space&uid={uid}\" title=\"{username}\" c=\"1\"{target}>{username}</a>: <a href=\"{url}\"{target}>{title}</a></li>\r\n[/loop]\r\n</ul>\r\n</div>\";s:6:\"footer\";s:0:\"\";s:6:\"header\";s:0:\"\";s:9:\"indexplus\";a:0:{}s:5:\"index\";a:0:{}s:9:\"orderplus\";a:0:{}s:5:\"order\";a:0:{}s:8:\"loopplus\";a:0:{}s:4:\"loop\";s:132:\"<li><a href=\"home.php?mod=space&uid={uid}\" title=\"{username}\" c=\"1\"{target}>{username}</a>: <a href=\"{url}\"{target}>{title}</a></li>\";}','d0ca1426',0,0,0,1,'a:4:{i:0;s:3:\"uid\";i:1;s:8:\"username\";i:2;s:3:\"url\";i:3;s:5:\"title\";}',0),
(63,'space_doing','[鍐呯疆]澶村儚+浣滆€?鍐呭','a:9:{s:3:\"raw\";s:392:\"<div class=\"module cl xld\">\r\n[loop]\r\n<dl class=\"cl\">\r\n	<dd class=\"m\"><a href=\"home.php?mod=space&uid={uid}\" c=\"1\"{target}><img src=\"{avatar}\" width=\"48\" height=\"48\" alt=\"{username}\" /></a></dd>\r\n	<dt><a href=\"home.php?mod=space&uid={uid}\" title=\"{username}\"{target}>{username}</a> <em class=\"xg1 xw0\">{dateline}</em></dt>\r\n	<dd><a href=\"{url}\"{target}>{title}</a></dd>\r\n</dl>\r\n[/loop]\r\n</div>\";s:6:\"footer\";s:0:\"\";s:6:\"header\";s:0:\"\";s:9:\"indexplus\";a:0:{}s:5:\"index\";a:0:{}s:9:\"orderplus\";a:0:{}s:5:\"order\";a:0:{}s:8:\"loopplus\";a:0:{}s:4:\"loop\";s:338:\"<dl class=\"cl\">\r\n	<dd class=\"m\"><a href=\"home.php?mod=space&uid={uid}\" c=\"1\"{target}><img src=\"{avatar}\" width=\"48\" height=\"48\" alt=\"{username}\" /></a></dd>\r\n	<dt><a href=\"home.php?mod=space&uid={uid}\" title=\"{username}\"{target}>{username}</a> <em class=\"xg1 xw0\">{dateline}</em></dt>\r\n	<dd><a href=\"{url}\"{target}>{title}</a></dd>\r\n</dl>\";}','13f43cab',0,0,0,1,'a:6:{i:0;s:3:\"uid\";i:1;s:6:\"avatar\";i:2;s:8:\"username\";i:3;s:8:\"dateline\";i:4;s:3:\"url\";i:5;s:5:\"title\";}',0),
(64,'space_doing','[鍐呯疆]浣滆€?鍐呭锛堝琛岋級+鏃堕棿','a:9:{s:3:\"raw\";s:236:\"<div class=\"module cl xl\">\r\n<ul>\r\n[loop]\r\n<li><a href=\"home.php?mod=space&uid={uid}\" title=\"{username}\" c=\"1\"{target}>{username}</a>: <a href=\"{url}\"{target}>{title}</a> <span class=\"xg1\">({dateline})</span></li>\r\n[/loop]\r\n</ul>\r\n</div>\";s:6:\"footer\";s:0:\"\";s:6:\"header\";s:0:\"\";s:9:\"indexplus\";a:0:{}s:5:\"index\";a:0:{}s:9:\"orderplus\";a:0:{}s:5:\"order\";a:0:{}s:8:\"loopplus\";a:0:{}s:4:\"loop\";s:170:\"<li><a href=\"home.php?mod=space&uid={uid}\" title=\"{username}\" c=\"1\"{target}>{username}</a>: <a href=\"{url}\"{target}>{title}</a> <span class=\"xg1\">({dateline})</span></li>\";}','927ed021',0,0,0,1,'a:5:{i:0;s:3:\"uid\";i:1;s:8:\"username\";i:2;s:3:\"url\";i:3;s:5:\"title\";i:4;s:8:\"dateline\";}',0),
(65,'space_blog','[鍐呯疆]鏃ュ織鏍囬','a:9:{s:3:\"raw\";s:130:\"<div class=\"module cl xl xl1\">\r\n<ul>\r\n[loop]\r\n<li><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></li>\r\n[/loop]\r\n</ul>\r\n</div>\";s:6:\"footer\";s:0:\"\";s:6:\"header\";s:0:\"\";s:9:\"indexplus\";a:0:{}s:5:\"index\";a:0:{}s:9:\"orderplus\";a:0:{}s:5:\"order\";a:0:{}s:8:\"loopplus\";a:0:{}s:4:\"loop\";s:60:\"<li><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></li>\";}','9349072a',0,0,0,1,'a:2:{i:0;s:3:\"url\";i:1;s:5:\"title\";}',0),
(66,'space_blog','[鍐呯疆]鏃ュ織鏍囬+浣滆€?,'a:9:{s:3:\"raw\";s:200:\"<div class=\"module cl xl xl1\">\r\n<ul>\r\n[loop]\r\n<li><em><a href=\"home.php?mod=space&uid={uid}\"{target}>{username}</a></em><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></li>\r\n[/loop]\r\n</ul>\r\n</div>\";s:6:\"footer\";s:0:\"\";s:6:\"header\";s:0:\"\";s:9:\"indexplus\";a:0:{}s:5:\"index\";a:0:{}s:9:\"orderplus\";a:0:{}s:5:\"order\";a:0:{}s:8:\"loopplus\";a:0:{}s:4:\"loop\";s:130:\"<li><em><a href=\"home.php?mod=space&uid={uid}\"{target}>{username}</a></em><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></li>\";}','d2a5c82a',0,0,0,1,'a:4:{i:0;s:3:\"uid\";i:1;s:8:\"username\";i:2;s:3:\"url\";i:3;s:5:\"title\";}',0),
(67,'space_blog','[鍐呯疆]鏃ュ織鏍囬+鍙戝竷鏃堕棿','a:9:{s:3:\"raw\";s:149:\"<div class=\"module cl xl xl1\">\r\n<ul>\r\n[loop]\r\n<li><em>{dateline}</em><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></li>\r\n[/loop]\r\n</ul>\r\n</div>\";s:6:\"footer\";s:0:\"\";s:6:\"header\";s:0:\"\";s:9:\"indexplus\";a:0:{}s:5:\"index\";a:0:{}s:9:\"orderplus\";a:0:{}s:5:\"order\";a:0:{}s:8:\"loopplus\";a:0:{}s:4:\"loop\";s:79:\"<li><em>{dateline}</em><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></li>\";}','c68ceade',0,0,0,1,'a:3:{i:0;s:8:\"dateline\";i:1;s:3:\"url\";i:2;s:5:\"title\";}',0),
(68,'space_blog','[鍐呯疆]鏃ュ織鏍囬+璇勮鏁?,'a:9:{s:3:\"raw\";s:149:\"<div class=\"module cl xl xl1\">\r\n<ul>\r\n[loop]\r\n<li><em>{replynum}</em><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></li>\r\n[/loop]\r\n</ul>\r\n</div>\";s:6:\"footer\";s:0:\"\";s:6:\"header\";s:0:\"\";s:9:\"indexplus\";a:0:{}s:5:\"index\";a:0:{}s:9:\"orderplus\";a:0:{}s:5:\"order\";a:0:{}s:8:\"loopplus\";a:0:{}s:4:\"loop\";s:79:\"<li><em>{replynum}</em><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></li>\";}','0345faa7',0,0,0,1,'a:3:{i:0;s:8:\"replynum\";i:1;s:3:\"url\";i:2;s:5:\"title\";}',0),
(69,'space_blog','[鍐呯疆]鏃ュ織鏍囬+浣滆€?绠€浠?,'a:9:{s:3:\"raw\";s:248:\"<div class=\"module cl xld\">\r\n[loop]\r\n<dl class=\"cl\">\r\n	<dt><em class=\"y xg1 xw0\"><a href=\"home.php?mod=space&uid={uid}\"{target}>{username}</a></em><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></dt>\r\n	<dd>{summary}</dd>\r\n</dl>\r\n[/loop]\r\n</div>\";s:6:\"footer\";s:0:\"\";s:6:\"header\";s:0:\"\";s:9:\"indexplus\";a:0:{}s:5:\"index\";a:0:{}s:9:\"orderplus\";a:0:{}s:5:\"order\";a:0:{}s:8:\"loopplus\";a:0:{}s:4:\"loop\";s:194:\"<dl class=\"cl\">\r\n	<dt><em class=\"y xg1 xw0\"><a href=\"home.php?mod=space&uid={uid}\"{target}>{username}</a></em><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></dt>\r\n	<dd>{summary}</dd>\r\n</dl>\";}','cd5e700c',0,1,0,1,'a:5:{i:0;s:3:\"uid\";i:1;s:8:\"username\";i:2;s:3:\"url\";i:3;s:5:\"title\";i:4;s:7:\"summary\";}',0),
(70,'space_blog','[鍐呯疆]鏃ュ織缂╃暐鍥?鏍囬+绠€浠?,'a:9:{s:3:\"raw\";s:361:\"<div class=\"module cl xld\">\r\n[loop]\r\n<dl class=\"cl\">\r\n	<dd class=\"m\"><a href=\"{url}\"{target}><img src=\"{pic}\" width=\"{picwidth}\" height=\"{picheight}\" alt=\"{title}\" /></a></dd>\r\n	<dt><em class=\"y xg1 xw0\"><a href=\"home.php?uid={uid}\"{target}>{username}</a></em><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></dt>\r\n	<dd>{summary}</dd>\r\n</dl>\r\n[/loop]\r\n</div>\";s:6:\"footer\";s:0:\"\";s:6:\"header\";s:0:\"\";s:9:\"indexplus\";a:0:{}s:5:\"index\";a:0:{}s:9:\"orderplus\";a:0:{}s:5:\"order\";a:0:{}s:8:\"loopplus\";a:0:{}s:4:\"loop\";s:307:\"<dl class=\"cl\">\r\n	<dd class=\"m\"><a href=\"{url}\"{target}><img src=\"{pic}\" width=\"{picwidth}\" height=\"{picheight}\" alt=\"{title}\" /></a></dd>\r\n	<dt><em class=\"y xg1 xw0\"><a href=\"home.php?uid={uid}\"{target}>{username}</a></em><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></dt>\r\n	<dd>{summary}</dd>\r\n</dl>\";}','323bc8e0',1,1,1,1,'a:6:{i:0;s:3:\"url\";i:1;s:3:\"pic\";i:2;s:5:\"title\";i:3;s:3:\"uid\";i:4;s:8:\"username\";i:5;s:7:\"summary\";}',0),
(71,'space_blog','[鍐呯疆]鏃ュ織鍥剧墖骞荤伅','a:9:{s:3:\"raw\";s:333:\"<div class=\"module cl slidebox\">\r\n<ul class=\"slideshow\">\r\n[loop]\r\n<li style=\"width: {picwidth}px; height: {picheight}px;\"><a href=\"{url}\"{target}><img src=\"{pic}\" width=\"{picwidth}\" height=\"{picheight}\" /></a><span class=\"title\">{title}</span></li>\r\n[/loop]\r\n</ul>\r\n</div>\r\n<script type=\"text/javascript\">\r\nrunslideshow();\r\n</script>\";s:6:\"footer\";s:0:\"\";s:6:\"header\";s:0:\"\";s:9:\"indexplus\";a:0:{}s:5:\"index\";a:0:{}s:9:\"orderplus\";a:0:{}s:5:\"order\";a:0:{}s:8:\"loopplus\";a:0:{}s:4:\"loop\";s:182:\"<li style=\"width: {picwidth}px; height: {picheight}px;\"><a href=\"{url}\"{target}><img src=\"{pic}\" width=\"{picwidth}\" height=\"{picheight}\" /></a><span class=\"title\">{title}</span></li>\";}','c23cc347',1,0,1,1,'a:3:{i:0;s:3:\"url\";i:1;s:3:\"pic\";i:2;s:5:\"title\";}',0),
(72,'space_blog','[鍐呯疆]鐒︾偣妯″紡','a:9:{s:3:\"raw\";s:164:\"<div class=\"module cl xld fcs\">\r\n[loop]\r\n<dl class=\"cl\">\r\n	<dt><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></dt>\r\n	<dd>{summary}</dd>\r\n</dl>\r\n[/loop]\r\n</div>\";s:6:\"footer\";s:0:\"\";s:6:\"header\";s:0:\"\";s:9:\"indexplus\";a:0:{}s:5:\"index\";a:0:{}s:9:\"orderplus\";a:0:{}s:5:\"order\";a:0:{}s:8:\"loopplus\";a:0:{}s:4:\"loop\";s:106:\"<dl class=\"cl\">\r\n	<dt><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></dt>\r\n	<dd>{summary}</dd>\r\n</dl>\";}','3bb0bf67',0,1,0,1,'a:3:{i:0;s:3:\"url\";i:1;s:5:\"title\";i:2;s:7:\"summary\";}',0),
(73,'space_album','[鍐呯疆]鐩稿唽鍒楄〃','a:9:{s:3:\"raw\";s:253:\"<div class=\"module cl ml\">\r\n<ul>\r\n[loop]\r\n<li>\r\n	<a href=\"{url}\"{target}><img src=\"{pic}\" width=\"{picwidth}\" height=\"{picheight}\" alt=\"{title}\" /></a>\r\n	<p><a href=\"{url}\" title=\"{title}\"{target}>{title}</a> ({picnum})</p>\r\n</li>\r\n[/loop]\r\n</ul>\r\n</div>\";s:6:\"footer\";s:0:\"\";s:6:\"header\";s:0:\"\";s:9:\"indexplus\";a:0:{}s:5:\"index\";a:0:{}s:9:\"orderplus\";a:0:{}s:5:\"order\";a:0:{}s:8:\"loopplus\";a:0:{}s:4:\"loop\";s:187:\"<li>\r\n	<a href=\"{url}\"{target}><img src=\"{pic}\" width=\"{picwidth}\" height=\"{picheight}\" alt=\"{title}\" /></a>\r\n	<p><a href=\"{url}\" title=\"{title}\"{target}>{title}</a> ({picnum})</p>\r\n</li>\";}','73e0a54f',1,0,1,1,'a:4:{i:0;s:3:\"url\";i:1;s:3:\"pic\";i:2;s:5:\"title\";i:3;s:6:\"picnum\";}',0),
(74,'space_album','[鍐呯疆]鐩稿唽鍒楄〃+鍚嶇О+鐢ㄦ埛','a:9:{s:3:\"raw\";s:320:\"<div class=\"module cl ml\">\r\n<ul>\r\n[loop]\r\n<li>\r\n	<a href=\"{url}\"{target}><img src=\"{pic}\" width=\"{picwidth}\" height=\"{picheight}\" alt=\"{title}\" /></a>\r\n	<p><a href=\"{url}\" title=\"{title}\"{target}>{title}</a> ({picnum})</p>\r\n	<span><a href=\"home.php?uid={uid}\"{target}>{username}</a></span>\r\n</li>\r\n[/loop]\r\n</ul>\r\n</div>\";s:6:\"footer\";s:0:\"\";s:6:\"header\";s:0:\"\";s:9:\"indexplus\";a:0:{}s:5:\"index\";a:0:{}s:9:\"orderplus\";a:0:{}s:5:\"order\";a:0:{}s:8:\"loopplus\";a:0:{}s:4:\"loop\";s:254:\"<li>\r\n	<a href=\"{url}\"{target}><img src=\"{pic}\" width=\"{picwidth}\" height=\"{picheight}\" alt=\"{title}\" /></a>\r\n	<p><a href=\"{url}\" title=\"{title}\"{target}>{title}</a> ({picnum})</p>\r\n	<span><a href=\"home.php?uid={uid}\"{target}>{username}</a></span>\r\n</li>\";}','cc34db30',1,0,1,1,'a:6:{i:0;s:3:\"url\";i:1;s:3:\"pic\";i:2;s:5:\"title\";i:3;s:6:\"picnum\";i:4;s:3:\"uid\";i:5;s:8:\"username\";}',0),
(75,'space_pic','[鍐呯疆]鍥剧墖鍒楄〃','a:9:{s:3:\"raw\";s:271:\"<div class=\"module cl ml\">\r\n<ul>\r\n[loop]\r\n<li style=\"width: {picwidth}px;\">\r\n	<a href=\"{url}\"{target}><img src=\"{pic}\" width=\"{picwidth}\" height=\"{picheight}\" alt=\"{title}\" /></a>\r\n	<p><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></p>\r\n</li>\r\n[/loop]\r\n</ul>\r\n</div>\";s:6:\"footer\";s:0:\"\";s:6:\"header\";s:0:\"\";s:9:\"indexplus\";a:0:{}s:5:\"index\";a:0:{}s:9:\"orderplus\";a:0:{}s:5:\"order\";a:0:{}s:8:\"loopplus\";a:0:{}s:4:\"loop\";s:205:\"<li style=\"width: {picwidth}px;\">\r\n	<a href=\"{url}\"{target}><img src=\"{pic}\" width=\"{picwidth}\" height=\"{picheight}\" alt=\"{title}\" /></a>\r\n	<p><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></p>\r\n</li>\";}','9e9201a8',1,0,1,1,'a:3:{i:0;s:3:\"url\";i:1;s:3:\"pic\";i:2;s:5:\"title\";}',0),
(76,'space_pic','[鍐呯疆]鍥剧墖骞荤伅','a:9:{s:3:\"raw\";s:333:\"<div class=\"module cl slidebox\">\r\n<ul class=\"slideshow\">\r\n[loop]\r\n<li style=\"width: {picwidth}px; height: {picheight}px;\"><a href=\"{url}\"{target}><img src=\"{pic}\" width=\"{picwidth}\" height=\"{picheight}\" /></a><span class=\"title\">{title}</span></li>\r\n[/loop]\r\n</ul>\r\n</div>\r\n<script type=\"text/javascript\">\r\nrunslideshow();\r\n</script>\";s:6:\"footer\";s:0:\"\";s:6:\"header\";s:0:\"\";s:9:\"indexplus\";a:0:{}s:5:\"index\";a:0:{}s:9:\"orderplus\";a:0:{}s:5:\"order\";a:0:{}s:8:\"loopplus\";a:0:{}s:4:\"loop\";s:182:\"<li style=\"width: {picwidth}px; height: {picheight}px;\"><a href=\"{url}\"{target}><img src=\"{pic}\" width=\"{picwidth}\" height=\"{picheight}\" /></a><span class=\"title\">{title}</span></li>\";}','c5d88e6d',1,0,1,1,'a:3:{i:0;s:3:\"url\";i:1;s:3:\"pic\";i:2;s:5:\"title\";}',0),
(77,'member_member','[鍐呯疆]浼氬憳澶村儚鍒楄〃','a:9:{s:3:\"raw\";s:238:\"<div class=\"module cl ml mls\">\r\n<ul>\r\n[loop]\r\n<li>\r\n	<a href=\"{url}\" c=\"1\"{target}><img src=\"{avatar}\" width=\"48\" height=\"48\" alt=\"{title}\" /></a>\r\n	<p><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></p>\r\n</li>\r\n[/loop]\r\n</ul>\r\n</div>\";s:6:\"footer\";s:0:\"\";s:6:\"header\";s:0:\"\";s:9:\"indexplus\";a:0:{}s:5:\"index\";a:0:{}s:9:\"orderplus\";a:0:{}s:5:\"order\";a:0:{}s:8:\"loopplus\";a:0:{}s:4:\"loop\";s:168:\"<li>\r\n	<a href=\"{url}\" c=\"1\"{target}><img src=\"{avatar}\" width=\"48\" height=\"48\" alt=\"{title}\" /></a>\r\n	<p><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></p>\r\n</li>\";}','2ef16e64',0,0,0,1,'a:3:{i:0;s:3:\"url\";i:1;s:6:\"avatar\";i:2;s:5:\"title\";}',0),
(78,'member_member','[鍐呯疆]鐢ㄦ埛鍚嶅垪琛?,'a:9:{s:3:\"raw\";s:136:\"<div class=\"module cl xl xl1\">\r\n<ul>\r\n[loop]\r\n<li><a href=\"{url}\" title=\"{title}\" c=\"1\"{target}>{title}</a></li>\r\n[/loop]\r\n</ul>\r\n</div>\";s:6:\"footer\";s:0:\"\";s:6:\"header\";s:0:\"\";s:9:\"indexplus\";a:0:{}s:5:\"index\";a:0:{}s:9:\"orderplus\";a:0:{}s:5:\"order\";a:0:{}s:8:\"loopplus\";a:0:{}s:4:\"loop\";s:66:\"<li><a href=\"{url}\" title=\"{title}\" c=\"1\"{target}>{title}</a></li>\";}','ed36c3b0',0,0,0,1,'a:2:{i:0;s:3:\"url\";i:1;s:5:\"title\";}',0),
(79,'member_member','[鍐呯疆]澶村儚+鐢ㄦ埛鍚?鍙戣创鏁帮紙鏈夊簭锛?,'a:9:{s:3:\"raw\";s:223:\"<div class=\"module cl xl xl1\">\r\n<ol>\r\n[loop]\r\n<li><em>{posts}</em><img class=\"vm\" src=\"{avatar}\" width=\"16\" height=\"16\" alt=\"{title}\" /> <a href=\"{url}\" title=\"{title}\" c=\"1\"{target}>{title}</a></li>\r\n[/loop]\r\n</ol>\r\n</div>\";s:6:\"footer\";s:0:\"\";s:6:\"header\";s:0:\"\";s:9:\"indexplus\";a:0:{}s:5:\"index\";a:0:{}s:9:\"orderplus\";a:0:{}s:5:\"order\";a:0:{}s:8:\"loopplus\";a:0:{}s:4:\"loop\";s:153:\"<li><em>{posts}</em><img class=\"vm\" src=\"{avatar}\" width=\"16\" height=\"16\" alt=\"{title}\" /> <a href=\"{url}\" title=\"{title}\" c=\"1\"{target}>{title}</a></li>\";}','b185afb9',0,0,0,1,'a:4:{i:0;s:5:\"posts\";i:1;s:6:\"avatar\";i:2;s:5:\"title\";i:3;s:3:\"url\";}',0),
(80,'member_member','[鍐呯疆]澶村儚+鐢ㄦ埛鍚?绉垎鏁帮紙鏈夊簭锛?,'a:9:{s:3:\"raw\";s:225:\"<div class=\"module cl xl xl1\">\r\n<ol>\r\n[loop]\r\n<li><em>{credits}</em><img class=\"vm\" src=\"{avatar}\" width=\"16\" height=\"16\" alt=\"{title}\" /> <a href=\"{url}\" title=\"{title}\" c=\"1\"{target}>{title}</a></li>\r\n[/loop]\r\n</ol>\r\n</div>\";s:6:\"footer\";s:0:\"\";s:6:\"header\";s:0:\"\";s:9:\"indexplus\";a:0:{}s:5:\"index\";a:0:{}s:9:\"orderplus\";a:0:{}s:5:\"order\";a:0:{}s:8:\"loopplus\";a:0:{}s:4:\"loop\";s:155:\"<li><em>{credits}</em><img class=\"vm\" src=\"{avatar}\" width=\"16\" height=\"16\" alt=\"{title}\" /> <a href=\"{url}\" title=\"{title}\" c=\"1\"{target}>{title}</a></li>\";}','8431f4e1',0,0,0,1,'a:4:{i:0;s:7:\"credits\";i:1;s:6:\"avatar\";i:2;s:5:\"title\";i:3;s:3:\"url\";}',0),
(81,'forum_trade','[鍐呯疆]鍟嗗搧鍒楄〃','a:9:{s:3:\"raw\";s:423:\"<div class=\"module cl ml\">\r\n<ul>\r\n[loop]\r\n<li style=\"padding: 0 12px 10px; width: {picwidth}px;\">\r\n<a href=\"{url}\"{target}><img src=\"{pic}\" width=\"{picwidth}\" height=\"{picheight}\" alt=\"{title}\" style=\"padding: 1px; border: 1px solid #CCC; background: #FFF;\" /></a>\r\n<p class=\"xs2\"><a href=\"{url}\"{target} class=\"xi1\">{price}</a></p>\r\n<p><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></p>\r\n</li>\r\n[/loop]\r\n</ul>\r\n</div>\";s:6:\"footer\";s:0:\"\";s:6:\"header\";s:0:\"\";s:9:\"indexplus\";a:0:{}s:5:\"index\";a:0:{}s:9:\"orderplus\";a:0:{}s:5:\"order\";a:0:{}s:8:\"loopplus\";a:0:{}s:4:\"loop\";s:357:\"<li style=\"padding: 0 12px 10px; width: {picwidth}px;\">\r\n<a href=\"{url}\"{target}><img src=\"{pic}\" width=\"{picwidth}\" height=\"{picheight}\" alt=\"{title}\" style=\"padding: 1px; border: 1px solid #CCC; background: #FFF;\" /></a>\r\n<p class=\"xs2\"><a href=\"{url}\"{target} class=\"xi1\">{price}</a></p>\r\n<p><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></p>\r\n</li>\";}','4fd3ffc9',1,0,1,1,'a:4:{i:0;s:3:\"url\";i:1;s:3:\"pic\";i:2;s:5:\"title\";i:3;s:5:\"price\";}',0),
(82,'forum_activity','[鍐呯疆]娲诲姩鍒楄〃','a:9:{s:3:\"raw\";s:331:\"<div class=\"module cl xld\">\r\n[loop]\r\n<dl class=\"cl\">\r\n	<dd class=\"m\"><a href=\"{url}\"{target}><img src=\"{pic}\" width=\"{picwidth}\" height=\"{picheight}\" alt=\"{title}\" /></a></dd>\r\n	<dt><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></dt>\r\n	<dd>{time} {place}</dd>\r\n	<dd> 宸叉湁 {applynumber} 浜烘姤鍚?/dd>\r\n</dl>\r\n[/loop]\r\n</div>\";s:6:\"footer\";s:0:\"\";s:6:\"header\";s:0:\"\";s:9:\"indexplus\";a:0:{}s:5:\"index\";a:0:{}s:9:\"orderplus\";a:0:{}s:5:\"order\";a:0:{}s:8:\"loopplus\";a:0:{}s:4:\"loop\";s:277:\"<dl class=\"cl\">\r\n	<dd class=\"m\"><a href=\"{url}\"{target}><img src=\"{pic}\" width=\"{picwidth}\" height=\"{picheight}\" alt=\"{title}\" /></a></dd>\r\n	<dt><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></dt>\r\n	<dd>{time} {place}</dd>\r\n	<dd> 宸叉湁 {applynumber} 浜烘姤鍚?/dd>\r\n</dl>\";}','3d04a558',1,0,1,1,'a:6:{i:0;s:3:\"url\";i:1;s:3:\"pic\";i:2;s:5:\"title\";i:3;s:4:\"time\";i:4;s:5:\"place\";i:5;s:11:\"applynumber\";}',0),
(83,'group_trade','[鍐呯疆]鍟嗗搧鍒楄〃','a:9:{s:3:\"raw\";s:288:\"<div class=\"module cl ml\">\r\n<ul>\r\n[loop]\r\n<li style=\"width: {picwidth}px;\">\r\n	<a href=\"{url}\"{target}><img src=\"{pic}\" width=\"{picwidth}\" height=\"{picheight}\" alt=\"{title}\" /></a>\r\n	<p><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></p>\r\n	<p>{price}</p>\r\n</li>\r\n[/loop]\r\n</ul>\r\n</div>\";s:6:\"footer\";s:0:\"\";s:6:\"header\";s:0:\"\";s:9:\"indexplus\";a:0:{}s:5:\"index\";a:0:{}s:9:\"orderplus\";a:0:{}s:5:\"order\";a:0:{}s:8:\"loopplus\";a:0:{}s:4:\"loop\";s:222:\"<li style=\"width: {picwidth}px;\">\r\n	<a href=\"{url}\"{target}><img src=\"{pic}\" width=\"{picwidth}\" height=\"{picheight}\" alt=\"{title}\" /></a>\r\n	<p><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></p>\r\n	<p>{price}</p>\r\n</li>\";}','edd331a7',1,0,1,1,'a:4:{i:0;s:3:\"url\";i:1;s:3:\"pic\";i:2;s:5:\"title\";i:3;s:5:\"price\";}',0),
(84,'group_activity','[鍐呯疆]娲诲姩鍒楄〃','a:9:{s:3:\"raw\";s:331:\"<div class=\"module cl xld\">\r\n[loop]\r\n<dl class=\"cl\">\r\n	<dd class=\"m\"><a href=\"{url}\"{target}><img src=\"{pic}\" width=\"{picwidth}\" height=\"{picheight}\" alt=\"{title}\" /></a></dd>\r\n	<dt><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></dt>\r\n	<dd>{time} {place}</dd>\r\n	<dd> 宸叉湁 {applynumber} 浜烘姤鍚?/dd>\r\n</dl>\r\n[/loop]\r\n</div>\";s:6:\"footer\";s:0:\"\";s:6:\"header\";s:0:\"\";s:9:\"indexplus\";a:0:{}s:5:\"index\";a:0:{}s:9:\"orderplus\";a:0:{}s:5:\"order\";a:0:{}s:8:\"loopplus\";a:0:{}s:4:\"loop\";s:277:\"<dl class=\"cl\">\r\n	<dd class=\"m\"><a href=\"{url}\"{target}><img src=\"{pic}\" width=\"{picwidth}\" height=\"{picheight}\" alt=\"{title}\" /></a></dd>\r\n	<dt><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></dt>\r\n	<dd>{time} {place}</dd>\r\n	<dd> 宸叉湁 {applynumber} 浜烘姤鍚?/dd>\r\n</dl>\";}','502cc3f6',1,0,1,1,'a:6:{i:0;s:3:\"url\";i:1;s:3:\"pic\";i:2;s:5:\"title\";i:3;s:4:\"time\";i:4;s:5:\"place\";i:5;s:11:\"applynumber\";}',0),
(85,'forum_thread','[鍐呯疆]甯栧瓙浣滆€咃紜鏍囬+鎽樿锛堝甫澶村儚锛?,'a:9:{s:3:\"raw\";s:468:\"<div class=\"module cl xld xlda\">\r\n[loop]\r\n<dl class=\"cl\">\r\n<dd class=\"m\"><a href=\"home.php?mod=space&uid={authorid}\" c=\"1\"{target}><img src=\"{avatar}\" width=\"48\" height=\"48\" alt=\"{author}\" /></a></dd>\r\n<dt style=\"padding-bottom: 0;\"><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></dt>\r\n<dd style=\"margin-bottom: 0;\">{summary}</dd>\r\n<dd style=\"margin-bottom: 0;\">浣滆€? <a href=\"home.php?mod=space&uid={authorid}\"{target}>{author}</a></dd>\r\n</dl>\r\n[/loop]\r\n</div>\";s:6:\"footer\";s:0:\"\";s:6:\"header\";s:0:\"\";s:9:\"indexplus\";a:0:{}s:5:\"index\";a:0:{}s:9:\"orderplus\";a:0:{}s:5:\"order\";a:0:{}s:8:\"loopplus\";a:0:{}s:4:\"loop\";s:409:\"<dl class=\"cl\">\r\n<dd class=\"m\"><a href=\"home.php?mod=space&uid={authorid}\" c=\"1\"{target}><img src=\"{avatar}\" width=\"48\" height=\"48\" alt=\"{author}\" /></a></dd>\r\n<dt style=\"padding-bottom: 0;\"><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></dt>\r\n<dd style=\"margin-bottom: 0;\">{summary}</dd>\r\n<dd style=\"margin-bottom: 0;\">浣滆€? <a href=\"home.php?mod=space&uid={authorid}\"{target}>{author}</a></dd>\r\n</dl>\";}','87d533ea',0,1,0,1,'a:6:{i:0;s:8:\"authorid\";i:1;s:6:\"avatar\";i:2;s:6:\"author\";i:3;s:3:\"url\";i:4;s:5:\"title\";i:5;s:7:\"summary\";}',0),
(86,'portal_article','[鍐呯疆]棰戦亾鏍忕洰+鏍囬','a:9:{s:3:\"raw\";s:205:\"<div class=\"module cl xl xl1\">\r\n<ul>\r\n[loop]\r\n<li><label>[<a href=\"{caturl}\" title=\"{catname}\"{target}>{catname}</a>]</label><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></li>\r\n[/loop]\r\n</ul>\r\n</div>\";s:6:\"footer\";s:0:\"\";s:6:\"header\";s:0:\"\";s:9:\"indexplus\";a:0:{}s:5:\"index\";a:0:{}s:9:\"orderplus\";a:0:{}s:5:\"order\";a:0:{}s:8:\"loopplus\";a:0:{}s:4:\"loop\";s:135:\"<li><label>[<a href=\"{caturl}\" title=\"{catname}\"{target}>{catname}</a>]</label><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></li>\";}','7720f457',0,0,0,1,'a:4:{i:0;s:6:\"caturl\";i:1;s:7:\"catname\";i:2;s:3:\"url\";i:3;s:5:\"title\";}',0),
(87,'forum_thread','[鍐呯疆]鎮祻涓婚涓撶敤鏍峰紡','a:9:{s:3:\"raw\";s:139:\"<div class=\"module cl xl xl1\">\r\n<ul>\r\n[loop]\r\n<li><a href=\"{url}\" title=\"{title}\"{target}>{title}</a>{summary}</li>\r\n[/loop]\r\n</ul>\r\n</div>\";s:6:\"footer\";s:0:\"\";s:6:\"header\";s:0:\"\";s:9:\"indexplus\";a:0:{}s:5:\"index\";a:0:{}s:9:\"orderplus\";a:0:{}s:5:\"order\";a:0:{}s:8:\"loopplus\";a:0:{}s:4:\"loop\";s:69:\"<li><a href=\"{url}\" title=\"{title}\"{target}>{title}</a>{summary}</li>\";}','56bffda0',0,1,0,1,'a:3:{i:0;s:3:\"url\";i:1;s:5:\"title\";i:2;s:7:\"summary\";}',0),
(88,'forum_thread','[鍐呯疆]棣栭〉鐑-甯栧瓙','a:9:{s:3:\"raw\";s:278:\"<div class=\"module cl xld\">\r\n[loop]\r\n<dl>\r\n	<dd style=\"margin-bottom: 0; font-size: 12px; color: #369\">{author} &#8250;</dd>\r\n	<dt style=\"padding: 0;\"><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></dt>\r\n	<dd style=\"margin-bottom: 0;\">{summary}</dd>\r\n</dl>\r\n[/loop]\r\n</div>\";s:6:\"footer\";s:0:\"\";s:6:\"header\";s:0:\"\";s:9:\"indexplus\";a:0:{}s:5:\"index\";a:0:{}s:9:\"orderplus\";a:0:{}s:5:\"order\";a:0:{}s:8:\"loopplus\";a:0:{}s:4:\"loop\";s:224:\"<dl>\r\n	<dd style=\"margin-bottom: 0; font-size: 12px; color: #369\">{author} &#8250;</dd>\r\n	<dt style=\"padding: 0;\"><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></dt>\r\n	<dd style=\"margin-bottom: 0;\">{summary}</dd>\r\n</dl>\";}','08596517',0,1,0,1,'a:4:{i:0;s:6:\"author\";i:1;s:3:\"url\";i:2;s:5:\"title\";i:3;s:7:\"summary\";}',0),
(89,'group_thread','[鍐呯疆]棣栭〉鐑-鍦堝瓙甯栧瓙','a:9:{s:3:\"raw\";s:278:\"<div class=\"module cl xld\">\r\n[loop]\r\n<dl>\r\n	<dd style=\"margin-bottom: 0; font-size: 12px; color: #369\">{author} &#8250;</dd>\r\n	<dt style=\"padding: 0;\"><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></dt>\r\n	<dd style=\"margin-bottom: 0;\">{summary}</dd>\r\n</dl>\r\n[/loop]\r\n</div>\";s:6:\"footer\";s:0:\"\";s:6:\"header\";s:0:\"\";s:9:\"indexplus\";a:0:{}s:5:\"index\";a:0:{}s:9:\"orderplus\";a:0:{}s:5:\"order\";a:0:{}s:8:\"loopplus\";a:0:{}s:4:\"loop\";s:224:\"<dl>\r\n	<dd style=\"margin-bottom: 0; font-size: 12px; color: #369\">{author} &#8250;</dd>\r\n	<dt style=\"padding: 0;\"><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></dt>\r\n	<dd style=\"margin-bottom: 0;\">{summary}</dd>\r\n</dl>\";}','a75db897',0,1,0,1,'a:4:{i:0;s:6:\"author\";i:1;s:3:\"url\";i:2;s:5:\"title\";i:3;s:7:\"summary\";}',0),
(90,'space_blog','[鍐呯疆]棣栭〉鐑-鏃ュ織','a:9:{s:3:\"raw\";s:280:\"<div class=\"module cl xld\">\r\n[loop]\r\n<dl>\r\n	<dd style=\"margin-bottom: 0; font-size: 12px; color: #369\">{username} &#8250;</dd>\r\n	<dt style=\"padding: 0;\"><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></dt>\r\n	<dd style=\"margin-bottom: 0;\">{summary}</dd>\r\n</dl>\r\n[/loop]\r\n</div>\";s:6:\"footer\";s:0:\"\";s:6:\"header\";s:0:\"\";s:9:\"indexplus\";a:0:{}s:5:\"index\";a:0:{}s:9:\"orderplus\";a:0:{}s:5:\"order\";a:0:{}s:8:\"loopplus\";a:0:{}s:4:\"loop\";s:226:\"<dl>\r\n	<dd style=\"margin-bottom: 0; font-size: 12px; color: #369\">{username} &#8250;</dd>\r\n	<dt style=\"padding: 0;\"><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></dt>\r\n	<dd style=\"margin-bottom: 0;\">{summary}</dd>\r\n</dl>\";}','9e68bc9b',0,1,0,1,'a:4:{i:0;s:8:\"username\";i:1;s:3:\"url\";i:2;s:5:\"title\";i:3;s:7:\"summary\";}',0),
(91,'forum_thread','[鍐呯疆]鎶曠エ涓婚涓撶敤鏍峰紡','a:9:{s:3:\"raw\";s:166:\"<div class=\"module cl xld b_poll\">\r\n[loop]\r\n<dl>\r\n<dt class=\"xs2\"><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></dt>\r\n<dd>{summary}</dd>\r\n</dl>\r\n[/loop]\r\n</div>\";s:6:\"footer\";s:0:\"\";s:6:\"header\";s:0:\"\";s:9:\"indexplus\";a:0:{}s:5:\"index\";a:0:{}s:9:\"orderplus\";a:0:{}s:5:\"order\";a:0:{}s:8:\"loopplus\";a:0:{}s:4:\"loop\";s:105:\"<dl>\r\n<dt class=\"xs2\"><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></dt>\r\n<dd>{summary}</dd>\r\n</dl>\";}','fa07a66f',0,1,0,1,'a:3:{i:0;s:3:\"url\";i:1;s:5:\"title\";i:2;s:7:\"summary\";}',0),
(92,'forum_thread','[鍐呯疆]杈╄涓婚涓撶敤鏍峰紡','a:9:{s:3:\"raw\";s:168:\"<div class=\"module cl xld b_debate\">\r\n[loop]\r\n<dl>\r\n<dt class=\"xs2\"><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></dt>\r\n<dd>{summary}</dd>\r\n</dl>\r\n[/loop]\r\n</div>\";s:6:\"footer\";s:0:\"\";s:6:\"header\";s:0:\"\";s:9:\"indexplus\";a:0:{}s:5:\"index\";a:0:{}s:9:\"orderplus\";a:0:{}s:5:\"order\";a:0:{}s:8:\"loopplus\";a:0:{}s:4:\"loop\";s:105:\"<dl>\r\n<dt class=\"xs2\"><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></dt>\r\n<dd>{summary}</dd>\r\n</dl>\";}','6a480986',0,1,0,1,'a:3:{i:0;s:3:\"url\";i:1;s:5:\"title\";i:2;s:7:\"summary\";}',0),
(93,'group_activity','[鍐呯疆]鍦堝瓙娲诲姩:澶у浘锛嬫憳瑕?,'a:9:{s:3:\"raw\";s:368:\"<div class=\"module cl xld\">\r\n[loop]\r\n<dl>\r\n<dd class=\"m\"><a href=\"{url}\"{target}><img src=\"{pic}\" width=\"120\" height=\"140\" alt=\"{title}\" /></a></dd>\r\n<dt class=\"xs2\"><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></dt>\r\n<dd>\r\n<p class=\"pbn\">{summary}</p>\r\n<p>{place} {class}</p>\r\n<p>鏃堕棿: {time}</p>\r\n<p>{applynumber} 浜哄叧娉?/p>\r\n</dd>\r\n</dl>\r\n[/loop]\r\n</div>\";s:6:\"footer\";s:0:\"\";s:6:\"header\";s:0:\"\";s:9:\"indexplus\";a:0:{}s:5:\"index\";a:0:{}s:9:\"orderplus\";a:0:{}s:5:\"order\";a:0:{}s:8:\"loopplus\";a:0:{}s:4:\"loop\";s:314:\"<dl>\r\n<dd class=\"m\"><a href=\"{url}\"{target}><img src=\"{pic}\" width=\"120\" height=\"140\" alt=\"{title}\" /></a></dd>\r\n<dt class=\"xs2\"><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></dt>\r\n<dd>\r\n<p class=\"pbn\">{summary}</p>\r\n<p>{place} {class}</p>\r\n<p>鏃堕棿: {time}</p>\r\n<p>{applynumber} 浜哄叧娉?/p>\r\n</dd>\r\n</dl>\";}','11d4011e',1,1,0,1,'a:8:{i:0;s:3:\"url\";i:1;s:3:\"pic\";i:2;s:5:\"title\";i:3;s:7:\"summary\";i:4;s:5:\"place\";i:5;s:5:\"class\";i:6;s:4:\"time\";i:7;s:11:\"applynumber\";}',0),
(94,'group_activity','[鍐呯疆]鍦堝瓙娲诲姩:灏忓浘锛嬫爣棰?,'a:9:{s:3:\"raw\";s:382:\"<div class=\"module cl xld\">\r\n[loop]\r\n<dl class=\"cl\">\r\n<dd class=\"m\"><a href=\"{url}\"{target}><img src=\"{pic}\" width=\"48\" height=\"48鈥?alt=\"{title}\" /></a></dd>\r\n<dt style=\"padding-bottom: 0;\"><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></dt>\r\n<dd style=\"margin: 0;\"> {time} {place}</dd>\r\n<dd class=\"xg1\" style=\"margin: 0;\">{applynumber} 浜哄叧娉?/dd>\r\n</dl>\r\n[/loop]\r\n</div>\";s:6:\"footer\";s:0:\"\";s:6:\"header\";s:0:\"\";s:9:\"indexplus\";a:0:{}s:5:\"index\";a:0:{}s:9:\"orderplus\";a:0:{}s:5:\"order\";a:0:{}s:8:\"loopplus\";a:0:{}s:4:\"loop\";s:328:\"<dl class=\"cl\">\r\n<dd class=\"m\"><a href=\"{url}\"{target}><img src=\"{pic}\" width=\"48\" height=\"48鈥?alt=\"{title}\" /></a></dd>\r\n<dt style=\"padding-bottom: 0;\"><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></dt>\r\n<dd style=\"margin: 0;\"> {time} {place}</dd>\r\n<dd class=\"xg1\" style=\"margin: 0;\">{applynumber} 浜哄叧娉?/dd>\r\n</dl>\";}','51658dfa',1,0,0,1,'a:6:{i:0;s:3:\"url\";i:1;s:3:\"pic\";i:2;s:5:\"title\";i:3;s:4:\"time\";i:4;s:5:\"place\";i:5;s:11:\"applynumber\";}',0),
(95,'space_album','[鍐呯疆]鐩稿唽鍒楄〃锛堢珫绾垮垎闅旓級','a:9:{s:3:\"raw\";s:594:\"<div class=\"module cl ml\">\r\n<ul>\r\n[loop]\r\n<li style=\"width: {picwidth}px;\">\r\n	<a href=\"{url}\"><img src=\"{pic}\" width=\"{picwidth}\" height=\"{picheight}\" alt=\"{title}\" /></a>\r\n	<p><a href=\"{url}\" title=\"{title}\" title=\"{title}\"{target}>{title}</a> ({picnum})</p>\r\n</li>\r\n[/loop]\r\n[order=odd]\r\n<li style=\"margin-right: 18px; padding-right: 24px; border-right: 1px solid #CCC; width: {picwidth}px;\">\r\n	<a href=\"{url}\"><img src=\"{pic}\" width=\"{picwidth}\" height=\"{picheight}\" alt=\"{title}\" /></a>\r\n	<p><a href=\"{url}\" title=\"{title}\"{target}>{title}</a> ({picnum})</p>\r\n</li>\r\n[/order]\r\n</ul>\r\n</div>\";s:6:\"footer\";s:0:\"\";s:6:\"header\";s:0:\"\";s:9:\"indexplus\";a:0:{}s:5:\"index\";a:0:{}s:9:\"orderplus\";a:0:{}s:5:\"order\";a:1:{s:3:\"odd\";s:279:\"<li style=\"margin-right: 18px; padding-right: 24px; border-right: 1px solid #CCC; width: {picwidth}px;\">\r\n	<a href=\"{url}\"><img src=\"{pic}\" width=\"{picwidth}\" height=\"{picheight}\" alt=\"{title}\" /></a>\r\n	<p><a href=\"{url}\" title=\"{title}\"{target}>{title}</a> ({picnum})</p>\r\n</li>\";}s:8:\"loopplus\";a:0:{}s:4:\"loop\";s:224:\"<li style=\"width: {picwidth}px;\">\r\n	<a href=\"{url}\"><img src=\"{pic}\" width=\"{picwidth}\" height=\"{picheight}\" alt=\"{title}\" /></a>\r\n	<p><a href=\"{url}\" title=\"{title}\" title=\"{title}\"{target}>{title}</a> ({picnum})</p>\r\n</li>\";}','771549b7',1,0,1,1,'a:4:{i:0;s:3:\"url\";i:1;s:3:\"pic\";i:2;s:5:\"title\";i:3;s:6:\"picnum\";}',0),
(96,'space_pic','[鍐呯疆]鍥剧墖鍒楄〃锛堢珫绾垮垎闅旓級','a:9:{s:3:\"raw\";s:556:\"<div class=\"module cl ml\">\r\n<ul>\r\n[loop]\r\n<li style=\"width: {picwidth}px;\">\r\n	<a href=\"{url}\"><img src=\"{pic}\" width=\"{picwidth}\" height=\"{picheight}\" alt=\"{title}\" /></a>\r\n	<p><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></p>\r\n</li>\r\n[/loop]\r\n[order=odd]\r\n<li style=\"margin-right: 18px; padding-right: 24px; border-right: 1px solid #EEE; width: {picwidth}px;\">\r\n	<a href=\"{url}\"><img src=\"{pic}\" width=\"{picwidth}\" height=\"{picheight}\" alt=\"{title}\" /></a>\r\n	<p><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></p>\r\n</li>\r\n[/order]\r\n</ul>\r\n</div>\";s:6:\"footer\";s:0:\"\";s:6:\"header\";s:0:\"\";s:9:\"indexplus\";a:0:{}s:5:\"index\";a:0:{}s:9:\"orderplus\";a:0:{}s:5:\"order\";a:1:{s:3:\"odd\";s:268:\"<li style=\"margin-right: 18px; padding-right: 24px; border-right: 1px solid #EEE; width: {picwidth}px;\">\r\n	<a href=\"{url}\"><img src=\"{pic}\" width=\"{picwidth}\" height=\"{picheight}\" alt=\"{title}\" /></a>\r\n	<p><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></p>\r\n</li>\";}s:8:\"loopplus\";a:0:{}s:4:\"loop\";s:197:\"<li style=\"width: {picwidth}px;\">\r\n	<a href=\"{url}\"><img src=\"{pic}\" width=\"{picwidth}\" height=\"{picheight}\" alt=\"{title}\" /></a>\r\n	<p><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></p>\r\n</li>\";}','ab23af19',1,0,1,1,'a:3:{i:0;s:3:\"url\";i:1;s:3:\"pic\";i:2;s:5:\"title\";}',0),
(97,'portal_article','[鍐呯疆]纰庣墖寮忔枃绔犳爣棰樺垪琛?,'a:9:{s:3:\"raw\";s:261:\"<div class=\"module cl xl xl1\">\r\n<ul>\r\n[loop]\r\n<li><a href=\"{url}\" title=\"{title}\"{target}>{title}</a>\r\n[/loop]\r\n[order=even]\r\n<a href=\"{url}\" title=\"{title}\"{target} class=\"lit\" style=\"margin-left: 5px; font-size: 12px\">{title}</a></li>\r\n[/order]\r\n</ul>\r\n</div>\";s:6:\"footer\";s:0:\"\";s:6:\"header\";s:0:\"\";s:9:\"indexplus\";a:0:{}s:5:\"index\";a:0:{}s:9:\"orderplus\";a:0:{}s:5:\"order\";a:1:{s:4:\"even\";s:110:\"<a href=\"{url}\" title=\"{title}\"{target} class=\"lit\" style=\"margin-left: 5px; font-size: 12px\">{title}</a></li>\";}s:8:\"loopplus\";a:0:{}s:4:\"loop\";s:55:\"<li><a href=\"{url}\" title=\"{title}\"{target}>{title}</a>\";}','bc85eab4',0,0,0,1,'a:2:{i:0;s:3:\"url\";i:1;s:5:\"title\";}',0),
(98,'portal_article','[鍐呯疆]鏂囩珷灏侀潰鍒楄〃锛堢珫绾垮垎闅旓級','a:9:{s:3:\"raw\";s:556:\"<div class=\"module cl ml\">\r\n<ul>\r\n[loop]\r\n<li style=\"width: {picwidth}px;\">\r\n	<a href=\"{url}\"><img src=\"{pic}\" width=\"{picwidth}\" height=\"{picheight}\" alt=\"{title}\" /></a>\r\n	<p><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></p>\r\n</li>\r\n[/loop]\r\n[order=odd]\r\n<li style=\"margin-right: 18px; padding-right: 24px; border-right: 1px solid #EEE; width: {picwidth}px;\">\r\n	<a href=\"{url}\"><img src=\"{pic}\" width=\"{picwidth}\" height=\"{picheight}\" alt=\"{title}\" /></a>\r\n	<p><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></p>\r\n</li>\r\n[/order]\r\n</ul>\r\n</div>\";s:6:\"footer\";s:0:\"\";s:6:\"header\";s:0:\"\";s:9:\"indexplus\";a:0:{}s:5:\"index\";a:0:{}s:9:\"orderplus\";a:0:{}s:5:\"order\";a:1:{s:3:\"odd\";s:268:\"<li style=\"margin-right: 18px; padding-right: 24px; border-right: 1px solid #EEE; width: {picwidth}px;\">\r\n	<a href=\"{url}\"><img src=\"{pic}\" width=\"{picwidth}\" height=\"{picheight}\" alt=\"{title}\" /></a>\r\n	<p><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></p>\r\n</li>\";}s:8:\"loopplus\";a:0:{}s:4:\"loop\";s:197:\"<li style=\"width: {picwidth}px;\">\r\n	<a href=\"{url}\"><img src=\"{pic}\" width=\"{picwidth}\" height=\"{picheight}\" alt=\"{title}\" /></a>\r\n	<p><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></p>\r\n</li>\";}','6b653acb',1,0,1,1,'a:3:{i:0;s:3:\"url\";i:1;s:3:\"pic\";i:2;s:5:\"title\";}',0),
(99,'html_announcement','[鍐呯疆]绔欑偣鍏憡','a:9:{s:3:\"raw\";s:197:\"<div class=\"module cl\">\r\n<ul>\r\n[loop]\r\n<li><img alt=\"鍏憡\" src=\"static/image/common/ann_icon.gif\"><a href=\"{url}\" title=\"{title}\"{target}>{title}锛坽starttime}锛?/a></li>\r\n[/loop]\r\n</ul>\r\n</div>\";s:6:\"footer\";s:0:\"\";s:6:\"header\";s:0:\"\";s:9:\"indexplus\";a:0:{}s:5:\"index\";a:0:{}s:9:\"orderplus\";a:0:{}s:5:\"order\";a:0:{}s:8:\"loopplus\";a:0:{}s:4:\"loop\";s:134:\"<li><img alt=\"鍏憡\" src=\"static/image/common/ann_icon.gif\"><a href=\"{url}\" title=\"{title}\"{target}>{title}锛坽starttime}锛?/a></li>\";}','1f88cc82',0,0,0,1,'a:3:{i:0;s:3:\"url\";i:1;s:5:\"title\";i:2;s:9:\"starttime\";}',0),
(100,'forum_thread','[鍐呯疆]甯栧瓙鍥炬枃灞曠ず','a:9:{s:3:\"raw\";s:374:\"<div class=\"module cl xld\">\r\n[loop]\r\n<dl class=\"cl\">\r\n	<dd class=\"m\"><a href=\"{url}\"{target}><img src=\"{pic}\" width=\"{picwidth}\" height=\"{picheight}\" alt=\"{title}\" /></a></dd>\r\n	<dt><em class=\"y xg1 xw0\"><a href=\"home.php?mod=space&uid={authorid}\"{target}>{author}</a></em><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></dt>\r\n	<dd>{summary}</dd>\r\n</dl>\r\n[/loop]\r\n</div>\";s:6:\"footer\";s:0:\"\";s:6:\"header\";s:0:\"\";s:9:\"indexplus\";a:0:{}s:5:\"index\";a:0:{}s:9:\"orderplus\";a:0:{}s:5:\"order\";a:0:{}s:8:\"loopplus\";a:0:{}s:4:\"loop\";s:320:\"<dl class=\"cl\">\r\n	<dd class=\"m\"><a href=\"{url}\"{target}><img src=\"{pic}\" width=\"{picwidth}\" height=\"{picheight}\" alt=\"{title}\" /></a></dd>\r\n	<dt><em class=\"y xg1 xw0\"><a href=\"home.php?mod=space&uid={authorid}\"{target}>{author}</a></em><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></dt>\r\n	<dd>{summary}</dd>\r\n</dl>\";}','881ee4a3',1,1,1,1,'a:6:{i:0;s:3:\"url\";i:1;s:3:\"pic\";i:2;s:5:\"title\";i:3;s:8:\"authorid\";i:4;s:6:\"author\";i:5;s:7:\"summary\";}',0),
(101,'group_thread','[鍐呯疆]甯栧瓙鍥炬枃鍒楄〃','a:9:{s:3:\"raw\";s:374:\"<div class=\"module cl xld\">\r\n[loop]\r\n<dl class=\"cl\">\r\n	<dd class=\"m\"><a href=\"{url}\"{target}><img src=\"{pic}\" width=\"{picwidth}\" height=\"{picheight}\" alt=\"{title}\" /></a></dd>\r\n	<dt><em class=\"y xg1 xw0\"><a href=\"home.php?mod=space&uid={authorid}\"{target}>{author}</a></em><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></dt>\r\n	<dd>{summary}</dd>\r\n</dl>\r\n[/loop]\r\n</div>\";s:6:\"footer\";s:0:\"\";s:6:\"header\";s:0:\"\";s:9:\"indexplus\";a:0:{}s:5:\"index\";a:0:{}s:9:\"orderplus\";a:0:{}s:5:\"order\";a:0:{}s:8:\"loopplus\";a:0:{}s:4:\"loop\";s:320:\"<dl class=\"cl\">\r\n	<dd class=\"m\"><a href=\"{url}\"{target}><img src=\"{pic}\" width=\"{picwidth}\" height=\"{picheight}\" alt=\"{title}\" /></a></dd>\r\n	<dt><em class=\"y xg1 xw0\"><a href=\"home.php?mod=space&uid={authorid}\"{target}>{author}</a></em><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></dt>\r\n	<dd>{summary}</dd>\r\n</dl>\";}','b67132d6',1,1,1,1,'a:6:{i:0;s:3:\"url\";i:1;s:3:\"pic\";i:2;s:5:\"title\";i:3;s:8:\"authorid\";i:4;s:6:\"author\";i:5;s:7:\"summary\";}',0),
(102,'group_thread','[鍐呯疆][鍦堝瓙鍚峕+鍦堝瓙甯栧瓙鏍囬','a:9:{s:3:\"raw\";s:177:\"<div class=\"module cl xl xl1\">\r\n<ul>\r\n[loop]\r\n<li>[<a href=\"{groupurl}\"{target}>{groupname}</a>] <a href=\"{url}\" title=\"{title}\"{target}>{title}</a></li>\r\n[/loop]\r\n</ul>\r\n</div>\";s:6:\"footer\";s:0:\"\";s:6:\"header\";s:0:\"\";s:9:\"indexplus\";a:0:{}s:5:\"index\";a:0:{}s:9:\"orderplus\";a:0:{}s:5:\"order\";a:0:{}s:8:\"loopplus\";a:0:{}s:4:\"loop\";s:107:\"<li>[<a href=\"{groupurl}\"{target}>{groupname}</a>] <a href=\"{url}\" title=\"{title}\"{target}>{title}</a></li>\";}','a2f9089e',0,0,0,1,'a:4:{i:0;s:8:\"groupurl\";i:1;s:9:\"groupname\";i:2;s:3:\"url\";i:3;s:5:\"title\";}',0),
(103,'other_otherfriendlink','[鍐呯疆]鍙嬫儏閾炬帴鍥炬枃','a:9:{s:3:\"raw\";s:298:\"<div class=\"bn lk\">\r\n<ul class=\"m cl\">\r\n[loop]\r\n<li class=\"cl\">\r\n<div class=\"forumlogo\"><a href=\"{url}\" {target}><img border=\"0\" alt=\"{title}\" src=\"{pic}\"></a></div>\r\n<div class=\"forumcontent\"><h5><a target=\"_blank\" href=\"{url}\">{title}</a></h5><p>{summary}</p></div>\r\n</li>\r\n[/loop]\r\n</ul>\r\n</div>\";s:6:\"footer\";s:0:\"\";s:6:\"header\";s:0:\"\";s:9:\"indexplus\";a:0:{}s:5:\"index\";a:0:{}s:9:\"orderplus\";a:0:{}s:5:\"order\";a:0:{}s:8:\"loopplus\";a:0:{}s:4:\"loop\";s:226:\"<li class=\"cl\">\r\n<div class=\"forumlogo\"><a href=\"{url}\" {target}><img border=\"0\" alt=\"{title}\" src=\"{pic}\"></a></div>\r\n<div class=\"forumcontent\"><h5><a target=\"_blank\" href=\"{url}\">{title}</a></h5><p>{summary}</p></div>\r\n</li>\";}','b921ea24',0,1,1,1,'a:4:{i:0;s:3:\"url\";i:1;s:5:\"title\";i:2;s:3:\"pic\";i:3;s:7:\"summary\";}',0),
(104,'other_otherfriendlink','[鍐呯疆]鍙嬫儏閾炬帴浠呭浘鐗?,'a:9:{s:3:\"raw\";s:147:\"<div class=\"bn lk\">\r\n<div class=\"cl mbm\">\r\n[loop]\r\n<a href=\"{url}\" {target}><img border=\"0\" alt=\"{title}\" src=\"{pic}\"></a>\r\n[/loop]\r\n</div>\r\n</div>\";s:6:\"footer\";s:0:\"\";s:6:\"header\";s:0:\"\";s:9:\"indexplus\";a:0:{}s:5:\"index\";a:0:{}s:9:\"orderplus\";a:0:{}s:5:\"order\";a:0:{}s:8:\"loopplus\";a:0:{}s:4:\"loop\";s:71:\"<a href=\"{url}\" {target}><img border=\"0\" alt=\"{title}\" src=\"{pic}\"></a>\";}','c8d00338',0,1,0,1,'a:3:{i:0;s:3:\"url\";i:1;s:5:\"title\";i:2;s:3:\"pic\";}',0),
(105,'other_otherfriendlink','[鍐呯疆]鍙嬫儏閾炬帴浠呮枃瀛?,'a:9:{s:3:\"raw\";s:118:\"<div class=\"x cl\">\r\n<ul class=\"cl mbm\">\r\n[loop]\r\n<li><a href=\"{url}\" {target}>{title}</a></li>\r\n[/loop]\r\n</ul>\r\n</div>\";s:6:\"footer\";s:0:\"\";s:6:\"header\";s:0:\"\";s:9:\"indexplus\";a:0:{}s:5:\"index\";a:0:{}s:9:\"orderplus\";a:0:{}s:5:\"order\";a:0:{}s:8:\"loopplus\";a:0:{}s:4:\"loop\";s:45:\"<li><a href=\"{url}\" {target}>{title}</a></li>\";}','b615e0d0',0,0,0,1,'a:2:{i:0;s:3:\"url\";i:1;s:5:\"title\";}',0),
(106,'other_otherstat','[鍐呯疆]鍏ㄩ儴缁熻淇℃伅','a:9:{s:3:\"raw\";s:664:\"[loop]<div class=\"tns\">\r\n<ul>\r\n<li>{posts_title}:<em>{posts}</em></li>\r\n<li>{groups_title}:<em>{groups}</em></li>\r\n<li>{members_title}:<em>{members}</em></li>\r\n<li>{groupnewposts_title}:<em>{groupnewposts}</em></li>\r\n<li>{bbsnewposts_title}:<em>{bbsnewposts}</em></li>\r\n<li>{bbslastposts_title}:<em>{bbslastposts}</em></li>\r\n<li>{onlinemembers_title}:<em>{onlinemembers}</em></li>\r\n<li>{maxmembers_title}:<em>{maxmembers}</em></li>\r\n<li>{doings_title}:<em>{doings}</em></li>\r\n<li>{blogs_title}:<em>{blogs}</em></li>\r\n<li>{albums_title}:<em>{albums}</em></li>\r\n<li>{pics_title}:<em>{pics}</em></li>\r\n<li>{shares_title}:<em>{shares}</em></li>\r\n</ul>\r\n</div>\r\n[/loop]\";s:6:\"footer\";s:0:\"\";s:6:\"header\";s:0:\"\";s:9:\"indexplus\";a:0:{}s:5:\"index\";a:0:{}s:9:\"orderplus\";a:0:{}s:5:\"order\";a:0:{}s:8:\"loopplus\";a:0:{}s:4:\"loop\";s:649:\"<div class=\"tns\">\r\n<ul>\r\n<li>{posts_title}:<em>{posts}</em></li>\r\n<li>{groups_title}:<em>{groups}</em></li>\r\n<li>{members_title}:<em>{members}</em></li>\r\n<li>{groupnewposts_title}:<em>{groupnewposts}</em></li>\r\n<li>{bbsnewposts_title}:<em>{bbsnewposts}</em></li>\r\n<li>{bbslastposts_title}:<em>{bbslastposts}</em></li>\r\n<li>{onlinemembers_title}:<em>{onlinemembers}</em></li>\r\n<li>{maxmembers_title}:<em>{maxmembers}</em></li>\r\n<li>{doings_title}:<em>{doings}</em></li>\r\n<li>{blogs_title}:<em>{blogs}</em></li>\r\n<li>{albums_title}:<em>{albums}</em></li>\r\n<li>{pics_title}:<em>{pics}</em></li>\r\n<li>{shares_title}:<em>{shares}</em></li>\r\n</ul>\r\n</div>\";}','027d3e60',0,0,0,0,'a:26:{i:0;s:11:\"posts_title\";i:1;s:5:\"posts\";i:2;s:12:\"groups_title\";i:3;s:6:\"groups\";i:4;s:13:\"members_title\";i:5;s:7:\"members\";i:6;s:19:\"groupnewposts_title\";i:7;s:13:\"groupnewposts\";i:8;s:17:\"bbsnewposts_title\";i:9;s:11:\"bbsnewposts\";i:10;s:18:\"bbslastposts_title\";i:11;s:12:\"bbslastposts\";i:12;s:19:\"onlinemembers_title\";i:13;s:13:\"onlinemembers\";i:14;s:16:\"maxmembers_title\";i:15;s:10:\"maxmembers\";i:16;s:12:\"doings_title\";i:17;s:6:\"doings\";i:18;s:11:\"blogs_title\";i:19;s:5:\"blogs\";i:20;s:12:\"albums_title\";i:21;s:6:\"albums\";i:22;s:10:\"pics_title\";i:23;s:4:\"pics\";i:24;s:12:\"shares_title\";i:25;s:6:\"shares\";}',0),
(107,'forum_thread','[鍐呯疆]涓€绠€浠?涓ゅ垪鏍囬','a:9:{s:3:\"raw\";s:284:\"<div class=\"bm bw0\">\r\n[index=1]\r\n<dl class=\"cl xld\">\r\n<dt><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></dt>\r\n<dd>{summary}</dd>\r\n</dl>\r\n<hr class=\"da\" />\r\n[/index]\r\n<ul class=\"xl xl2 cl\">\r\n[loop]<li><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></li>[/loop]\r\n</ul>\r\n</div>\";s:6:\"footer\";s:0:\"\";s:6:\"header\";s:0:\"\";s:9:\"indexplus\";a:0:{}s:5:\"index\";a:1:{i:1;s:127:\"<dl class=\"cl xld\">\r\n<dt><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></dt>\r\n<dd>{summary}</dd>\r\n</dl>\r\n<hr class=\"da\" />\";}s:9:\"orderplus\";a:0:{}s:5:\"order\";a:0:{}s:8:\"loopplus\";a:0:{}s:4:\"loop\";s:60:\"<li><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></li>\";}','9e2ea31f',0,1,0,1,'a:3:{i:0;s:3:\"url\";i:1;s:5:\"title\";i:2;s:7:\"summary\";}',0),
(108,'forum_thread','[鍐呯疆]甯栧瓙鍥剧墖骞荤伅鐗?,'a:9:{s:3:\"raw\";s:333:\"<div class=\"module cl slidebox\">\r\n<ul class=\"slideshow\">\r\n[loop]\r\n<li style=\"width: {picwidth}px; height: {picheight}px;\"><a href=\"{url}\"{target}><img src=\"{pic}\" width=\"{picwidth}\" height=\"{picheight}\" /></a><span class=\"title\">{title}</span></li>\r\n[/loop]\r\n</ul>\r\n</div>\r\n<script type=\"text/javascript\">\r\nrunslideshow();\r\n</script>\";s:6:\"footer\";s:0:\"\";s:6:\"header\";s:0:\"\";s:9:\"indexplus\";a:0:{}s:5:\"index\";a:0:{}s:9:\"orderplus\";a:0:{}s:5:\"order\";a:0:{}s:8:\"loopplus\";a:0:{}s:4:\"loop\";s:182:\"<li style=\"width: {picwidth}px; height: {picheight}px;\"><a href=\"{url}\"{target}><img src=\"{pic}\" width=\"{picwidth}\" height=\"{picheight}\" /></a><span class=\"title\">{title}</span></li>\";}','cba1f109',1,0,1,1,'a:3:{i:0;s:3:\"url\";i:1;s:3:\"pic\";i:2;s:5:\"title\";}',0),
(109,'forum_thread','[鍐呯疆]甯栧瓙鍥剧墖鍒楄〃','a:9:{s:3:\"raw\";s:271:\"<div class=\"module cl ml\">\r\n<ul>\r\n[loop]\r\n<li style=\"width: {picwidth}px;\">\r\n	<a href=\"{url}\"{target}><img src=\"{pic}\" width=\"{picwidth}\" height=\"{picheight}\" alt=\"{title}\" /></a>\r\n	<p><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></p>\r\n</li>\r\n[/loop]\r\n</ul>\r\n</div>\";s:6:\"footer\";s:0:\"\";s:6:\"header\";s:0:\"\";s:9:\"indexplus\";a:0:{}s:5:\"index\";a:0:{}s:9:\"orderplus\";a:0:{}s:5:\"order\";a:0:{}s:8:\"loopplus\";a:0:{}s:4:\"loop\";s:205:\"<li style=\"width: {picwidth}px;\">\r\n	<a href=\"{url}\"{target}><img src=\"{pic}\" width=\"{picwidth}\" height=\"{picheight}\" alt=\"{title}\" /></a>\r\n	<p><a href=\"{url}\" title=\"{title}\"{target}>{title}</a></p>\r\n</li>\";}','0ab2e307',1,0,1,1,'a:3:{i:0;s:3:\"url\";i:1;s:3:\"pic\";i:2;s:5:\"title\";}',0),
(110,'html_misctag','[鍐呯疆]鏍囩妯＄増','a:9:{s:3:\"raw\";s:361:\"<!-- 鐑棬鏍囩妯″潡 -->\r\n<div class=\"tag-cloud-module\">\r\n	<div class=\"tag-cloud-container\">\r\n		[loop]\r\n		<a href=\"{url}\"\r\n		   title=\"{title} ({related_count}绡囧唴瀹?\"\r\n		   class=\"tag-cloud-item tag-size-{size_level} tag-color-{color_level}\"\r\n		   data-count=\"{related_count}\"\r\n		   data-hot=\"{hot_score}\">\r\n			{title}\r\n		</a>\r\n		[/loop]\r\n	</div>\r\n</div>\";s:6:\"footer\";s:0:\"\";s:6:\"header\";s:0:\"\";s:9:\"indexplus\";a:0:{}s:5:\"index\";a:0:{}s:9:\"orderplus\";a:0:{}s:5:\"order\";a:0:{}s:8:\"loopplus\";a:0:{}s:4:\"loop\";s:224:\"<a href=\"{url}\"\r\n		   title=\"{title} ({related_count}绡囧唴瀹?\"\r\n		   class=\"tag-cloud-item tag-size-{size_level} tag-color-{color_level}\"\r\n		   data-count=\"{related_count}\"\r\n		   data-hot=\"{hot_score}\">\r\n			{title}\r\n		</a>\";}','391cb72a',0,0,0,0,'a:6:{i:0;s:3:\"url\";i:1;s:5:\"title\";i:2;s:13:\"related_count\";i:3;s:10:\"size_level\";i:4;s:11:\"color_level\";i:5;s:9:\"hot_score\";}',0);
/*!40000 ALTER TABLE `pre_common_block_style` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_common_block_xml`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_common_block_xml` (
  `id` smallint(5) unsigned NOT NULL AUTO_INCREMENT,
  `name` varchar(255) NOT NULL,
  `version` varchar(255) NOT NULL,
  `url` varchar(255) NOT NULL,
  `clientid` varchar(255) NOT NULL,
  `key` varchar(255) NOT NULL,
  `signtype` varchar(255) NOT NULL,
  `data` text NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_common_block_xml` WRITE;
/*!40000 ALTER TABLE `pre_common_block_xml` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_common_block_xml` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_common_cache`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_common_cache` (
  `cachekey` varchar(190) NOT NULL DEFAULT '',
  `cachevalue` mediumblob NOT NULL,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`cachekey`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_common_cache` WRITE;
/*!40000 ALTER TABLE `pre_common_cache` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_common_cache` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_common_card`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_common_card` (
  `id` varchar(190) NOT NULL DEFAULT '',
  `typeid` smallint(6) unsigned NOT NULL DEFAULT 1,
  `maketype` tinyint(1) NOT NULL DEFAULT 0,
  `makeruid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `price` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `extcreditskey` tinyint(1) NOT NULL DEFAULT 0,
  `extcreditsval` int(10) NOT NULL DEFAULT 0,
  `status` tinyint(3) unsigned NOT NULL DEFAULT 1,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  `cleardateline` int(10) unsigned NOT NULL DEFAULT 0,
  `useddateline` int(10) unsigned NOT NULL DEFAULT 0,
  `uid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`id`),
  KEY `dateline` (`dateline`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_common_card` WRITE;
/*!40000 ALTER TABLE `pre_common_card` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_common_card` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_common_card_log`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_common_card_log` (
  `id` smallint(6) NOT NULL AUTO_INCREMENT,
  `uid` mediumint(8) NOT NULL DEFAULT 0,
  `username` varchar(50) NOT NULL DEFAULT '',
  `cardrule` varchar(255) NOT NULL DEFAULT '',
  `info` text NOT NULL,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  `description` mediumtext NOT NULL,
  `operation` tinyint(1) NOT NULL DEFAULT 0,
  PRIMARY KEY (`id`),
  KEY `dateline` (`dateline`),
  KEY `operation_dateline` (`operation`,`dateline`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_common_card_log` WRITE;
/*!40000 ALTER TABLE `pre_common_card_log` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_common_card_log` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_common_card_type`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_common_card_type` (
  `id` smallint(6) NOT NULL AUTO_INCREMENT,
  `typename` char(20) NOT NULL DEFAULT '',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_common_card_type` WRITE;
/*!40000 ALTER TABLE `pre_common_card_type` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_common_card_type` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_common_credit_log`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_common_credit_log` (
  `logid` int(10) unsigned NOT NULL AUTO_INCREMENT,
  `uid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `operation` char(3) NOT NULL DEFAULT '',
  `relatedid` int(10) unsigned NOT NULL,
  `dateline` int(10) unsigned NOT NULL,
  `extcredits1` int(10) NOT NULL,
  `extcredits2` int(10) NOT NULL,
  `extcredits3` int(10) NOT NULL,
  `extcredits4` int(10) NOT NULL,
  `extcredits5` int(10) NOT NULL,
  `extcredits6` int(10) NOT NULL,
  `extcredits7` int(10) NOT NULL,
  `extcredits8` int(10) NOT NULL,
  PRIMARY KEY (`logid`),
  KEY `uid` (`uid`),
  KEY `operation` (`operation`),
  KEY `relatedid` (`relatedid`),
  KEY `dateline` (`dateline`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_common_credit_log` WRITE;
/*!40000 ALTER TABLE `pre_common_credit_log` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_common_credit_log` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_common_credit_log_field`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_common_credit_log_field` (
  `logid` int(10) unsigned NOT NULL,
  `uid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `title` varchar(100) NOT NULL,
  `text` text NOT NULL,
  `dateline` int(10) unsigned NOT NULL,
  `ac_extcredits1` int(10) NOT NULL,
  `ac_extcredits2` int(10) NOT NULL,
  `ac_extcredits3` int(10) NOT NULL,
  `ac_extcredits4` int(10) NOT NULL,
  `ac_extcredits5` int(10) NOT NULL,
  `ac_extcredits6` int(10) NOT NULL,
  `ac_extcredits7` int(10) NOT NULL,
  `ac_extcredits8` int(10) NOT NULL,
  KEY `logid` (`logid`),
  KEY `uid` (`uid`),
  KEY `dateline` (`dateline`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_common_credit_log_field` WRITE;
/*!40000 ALTER TABLE `pre_common_credit_log_field` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_common_credit_log_field` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_common_credit_rule`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_common_credit_rule` (
  `rid` mediumint(8) unsigned NOT NULL AUTO_INCREMENT,
  `rulename` varchar(20) NOT NULL DEFAULT '',
  `action` varchar(20) NOT NULL DEFAULT '',
  `cycletype` tinyint(1) NOT NULL DEFAULT 0,
  `cycletime` int(10) NOT NULL DEFAULT 0,
  `rewardnum` tinyint(2) NOT NULL DEFAULT 1,
  `norepeat` tinyint(1) NOT NULL DEFAULT 0,
  `extcredits1` int(10) NOT NULL DEFAULT 0,
  `extcredits2` int(10) NOT NULL DEFAULT 0,
  `extcredits3` int(10) NOT NULL DEFAULT 0,
  `extcredits4` int(10) NOT NULL DEFAULT 0,
  `extcredits5` int(10) NOT NULL DEFAULT 0,
  `extcredits6` int(10) NOT NULL DEFAULT 0,
  `extcredits7` int(10) NOT NULL DEFAULT 0,
  `extcredits8` int(10) NOT NULL DEFAULT 0,
  `fids` text NOT NULL,
  PRIMARY KEY (`rid`),
  UNIQUE KEY `action` (`action`)
) ENGINE=InnoDB AUTO_INCREMENT=32 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_common_credit_rule` WRITE;
/*!40000 ALTER TABLE `pre_common_credit_rule` DISABLE KEYS */;
INSERT INTO `pre_common_credit_rule` VALUES
(1,'鍙戣〃涓婚','post',4,0,0,0,0,2,0,0,0,0,0,0,''),
(2,'鍙戣〃鍥炲','reply',4,0,0,0,0,1,0,0,0,0,0,0,''),
(3,'鍔犵簿鍗?,'digest',4,0,0,0,0,5,0,0,0,0,0,0,''),
(4,'涓婁紶闄勪欢','postattach',4,0,0,0,0,0,0,0,0,0,0,0,''),
(5,'涓嬭浇闄勪欢','getattach',4,0,0,0,0,0,0,0,0,0,0,0,''),
(6,'鍙戠煭娑堟伅','sendpm',4,0,0,0,0,0,0,0,0,0,0,0,''),
(7,'鎼滅储','search',4,0,0,0,0,0,0,0,0,0,0,0,''),
(8,'璁块棶鎺ㄥ箍','promotion_visit',1,0,1,0,0,0,0,0,0,0,0,0,''),
(9,'娉ㄥ唽鎺ㄥ箍','promotion_register',1,0,1,0,0,0,0,0,0,0,0,0,''),
(10,'鎴愬姛浜ゆ槗','tradefinished',4,0,0,0,0,0,0,0,0,0,0,0,''),
(11,'閭璁よ瘉','realemail',0,0,1,0,0,10,0,0,0,0,0,0,''),
(12,'璁剧疆澶村儚','setavatar',0,0,1,0,0,5,0,0,0,0,0,0,''),
(14,'鐑偣淇℃伅','hotinfo',4,0,0,0,0,0,0,0,0,0,0,0,''),
(15,'姣忓ぉ鐧诲綍','daylogin',1,0,1,0,0,2,0,0,0,0,0,0,''),
(16,'璁块棶鍒汉绌洪棿','visit',1,0,10,2,0,0,0,0,0,0,0,0,''),
(17,'鎵撴嫑鍛?,'poke',1,0,10,2,0,0,0,0,0,0,0,0,''),
(18,'鐣欒█','guestbook',1,0,20,2,0,1,0,0,0,0,0,0,''),
(19,'琚暀瑷€','getguestbook',1,0,5,2,0,1,0,0,0,0,0,0,''),
(20,'鍙戣〃璁板綍','doing',1,0,5,0,0,1,0,0,0,0,0,0,''),
(21,'鍙戣〃鏃ュ織','publishblog',1,0,3,0,0,2,0,0,0,0,0,0,''),
(22,'鍙備笌鎶曠エ','joinpoll',1,0,10,1,0,1,0,0,0,0,0,0,''),
(23,'鍙戣捣鍒嗕韩','createshare',1,0,3,0,0,1,0,0,0,0,0,0,''),
(24,'璇勮','comment',1,0,40,1,0,1,0,0,0,0,0,0,''),
(25,'琚瘎璁?,'getcomment',1,0,20,1,0,2,0,0,0,0,0,0,''),
(28,'淇℃伅琛ㄦ€?,'click',1,0,10,1,0,0,0,0,0,0,0,0,''),
(29,'淇敼鍩熷悕','modifydomain',0,0,1,0,0,0,0,0,0,0,0,0,''),
(30,'鏂囩珷璇勮','portalcomment',1,0,40,1,0,1,0,0,0,0,0,0,''),
(31,'娣樹笓杈戣璁㈤槄','followedcollection',1,0,3,0,0,1,0,0,0,0,0,0,'');
/*!40000 ALTER TABLE `pre_common_credit_rule` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_common_credit_rule_log`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_common_credit_rule_log` (
  `clid` mediumint(8) unsigned NOT NULL AUTO_INCREMENT,
  `uid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `rid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `fid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `total` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `cyclenum` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `extcredits1` int(10) NOT NULL DEFAULT 0,
  `extcredits2` int(10) NOT NULL DEFAULT 0,
  `extcredits3` int(10) NOT NULL DEFAULT 0,
  `extcredits4` int(10) NOT NULL DEFAULT 0,
  `extcredits5` int(10) NOT NULL DEFAULT 0,
  `extcredits6` int(10) NOT NULL DEFAULT 0,
  `extcredits7` int(10) NOT NULL DEFAULT 0,
  `extcredits8` int(10) NOT NULL DEFAULT 0,
  `starttime` int(10) unsigned NOT NULL DEFAULT 0,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`clid`),
  KEY `uid` (`uid`,`rid`,`fid`),
  KEY `dateline` (`dateline`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_common_credit_rule_log` WRITE;
/*!40000 ALTER TABLE `pre_common_credit_rule_log` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_common_credit_rule_log` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_common_credit_rule_log_field`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_common_credit_rule_log_field` (
  `clid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `uid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `info` text NOT NULL,
  `user` text NOT NULL,
  `app` text NOT NULL,
  PRIMARY KEY (`uid`,`clid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_common_credit_rule_log_field` WRITE;
/*!40000 ALTER TABLE `pre_common_credit_rule_log_field` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_common_credit_rule_log_field` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_common_devicetoken`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_common_devicetoken` (
  `uid` mediumint(8) unsigned NOT NULL,
  `token` char(50) NOT NULL,
  PRIMARY KEY (`uid`),
  KEY `token` (`token`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_common_devicetoken` WRITE;
/*!40000 ALTER TABLE `pre_common_devicetoken` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_common_devicetoken` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_common_district`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_common_district` (
  `id` mediumint(8) unsigned NOT NULL AUTO_INCREMENT,
  `name` varchar(255) NOT NULL DEFAULT '',
  `level` tinyint(4) unsigned NOT NULL DEFAULT 0,
  `usetype` tinyint(3) unsigned NOT NULL DEFAULT 0,
  `upid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `displayorder` smallint(6) NOT NULL DEFAULT 0,
  PRIMARY KEY (`id`),
  KEY `upid` (`upid`,`displayorder`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_common_district` WRITE;
/*!40000 ALTER TABLE `pre_common_district` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_common_district` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_common_diy_data`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_common_diy_data` (
  `targettplname` varchar(100) NOT NULL DEFAULT '',
  `tpldirectory` varchar(80) NOT NULL DEFAULT '',
  `primaltplname` varchar(255) NOT NULL DEFAULT '',
  `diycontent` mediumtext NOT NULL,
  `name` varchar(255) NOT NULL DEFAULT '',
  `uid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `username` varchar(50) NOT NULL DEFAULT '',
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`targettplname`,`tpldirectory`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_common_diy_data` WRITE;
/*!40000 ALTER TABLE `pre_common_diy_data` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_common_diy_data` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_common_domain`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_common_domain` (
  `domain` char(30) NOT NULL DEFAULT '',
  `domainroot` char(60) NOT NULL DEFAULT '',
  `id` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `idtype` char(15) NOT NULL DEFAULT '',
  PRIMARY KEY (`id`,`idtype`),
  KEY `domain` (`domain`,`domainroot`),
  KEY `idtype` (`idtype`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_common_domain` WRITE;
/*!40000 ALTER TABLE `pre_common_domain` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_common_domain` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_common_editorblock`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_common_editorblock` (
  `blockid` int(10) unsigned NOT NULL AUTO_INCREMENT,
  `available` tinyint(1) NOT NULL DEFAULT 0,
  `columns` tinyint(1) NOT NULL DEFAULT 0,
  `type` int(10) NOT NULL DEFAULT 0,
  `sort` int(10) NOT NULL DEFAULT 0,
  `name` varchar(255) NOT NULL DEFAULT '',
  `version` varchar(255) NOT NULL DEFAULT '',
  `identifier` varchar(255) NOT NULL DEFAULT '',
  `description` varchar(255) NOT NULL DEFAULT '',
  `class` varchar(255) NOT NULL DEFAULT '0',
  `parser` mediumtext NOT NULL,
  `style` mediumtext NOT NULL,
  `config` text NOT NULL,
  `filename` varchar(255) NOT NULL DEFAULT '',
  `i18n` text NOT NULL,
  `parameters` text NOT NULL,
  `plugin` varchar(255) NOT NULL DEFAULT '',
  `filemtime` int(10) unsigned NOT NULL DEFAULT 0,
  `copyright` varchar(255) NOT NULL DEFAULT '',
  PRIMARY KEY (`blockid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_common_editorblock` WRITE;
/*!40000 ALTER TABLE `pre_common_editorblock` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_common_editorblock` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_common_emaillog`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_common_emaillog` (
  `logid` int(10) unsigned NOT NULL AUTO_INCREMENT,
  `uid` mediumint(8) unsigned NOT NULL,
  `emailtype` int(10) NOT NULL DEFAULT 0,
  `svctype` int(10) NOT NULL DEFAULT 0,
  `status` int(10) NOT NULL DEFAULT 0,
  `verify` int(10) NOT NULL DEFAULT 0,
  `email` varchar(255) NOT NULL DEFAULT '',
  `ip` varchar(45) NOT NULL DEFAULT '',
  `port` smallint(6) unsigned NOT NULL DEFAULT 0,
  `content` text NOT NULL,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`logid`),
  KEY `dateline` (`email`,`dateline`),
  KEY `uid` (`uid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_common_emaillog` WRITE;
/*!40000 ALTER TABLE `pre_common_emaillog` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_common_emaillog` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_common_failedip`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_common_failedip` (
  `ip` varchar(45) NOT NULL DEFAULT '',
  `lastupdate` int(10) unsigned NOT NULL DEFAULT 0,
  `count` tinyint(3) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`ip`,`lastupdate`),
  KEY `lastupdate` (`lastupdate`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_common_failedip` WRITE;
/*!40000 ALTER TABLE `pre_common_failedip` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_common_failedip` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_common_failedlogin`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_common_failedlogin` (
  `ip` varchar(45) NOT NULL DEFAULT '',
  `username` char(50) NOT NULL DEFAULT '',
  `count` tinyint(3) unsigned NOT NULL DEFAULT 0,
  `lastupdate` int(10) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`ip`,`username`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_common_failedlogin` WRITE;
/*!40000 ALTER TABLE `pre_common_failedlogin` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_common_failedlogin` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_common_friendlink`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_common_friendlink` (
  `id` smallint(6) unsigned NOT NULL AUTO_INCREMENT,
  `displayorder` tinyint(3) NOT NULL DEFAULT 0,
  `name` varchar(100) NOT NULL DEFAULT '',
  `url` varchar(255) NOT NULL DEFAULT '',
  `description` mediumtext NOT NULL,
  `logo` varchar(255) NOT NULL DEFAULT '',
  `type` tinyint(3) NOT NULL DEFAULT 0,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=3 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_common_friendlink` WRITE;
/*!40000 ALTER TABLE `pre_common_friendlink` DISABLE KEYS */;
INSERT INTO `pre_common_friendlink` VALUES
(1,0,'Discuz! 瀹樻柟璁哄潧','https://www.discuz.vip/','鎻愪緵鏈€鏂?Discuz! 浜у搧鏂伴椈銆佽蒋浠朵笅杞戒笌鎶€鏈氦娴?,'static/image/common/logo_88_31.gif',2),
(2,4,'Discuz! 搴旂敤涓績','https://addon.dismall.com/','','',2);
/*!40000 ALTER TABLE `pre_common_friendlink` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_common_grouppm`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_common_grouppm` (
  `id` smallint(6) unsigned NOT NULL AUTO_INCREMENT,
  `authorid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `author` varchar(50) NOT NULL DEFAULT '',
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  `message` text NOT NULL,
  `numbers` mediumint(8) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_common_grouppm` WRITE;
/*!40000 ALTER TABLE `pre_common_grouppm` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_common_grouppm` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_common_invite`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_common_invite` (
  `id` mediumint(8) unsigned NOT NULL AUTO_INCREMENT,
  `uid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `code` char(20) NOT NULL DEFAULT '',
  `fuid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `fusername` char(50) NOT NULL DEFAULT '',
  `type` tinyint(1) NOT NULL DEFAULT 0,
  `email` varchar(255) NOT NULL DEFAULT '',
  `inviteip` varchar(45) NOT NULL DEFAULT '',
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  `endtime` int(10) unsigned NOT NULL DEFAULT 0,
  `regdateline` int(10) unsigned NOT NULL DEFAULT 0,
  `status` tinyint(1) NOT NULL DEFAULT 1,
  `orderid` char(32) NOT NULL DEFAULT '',
  PRIMARY KEY (`id`),
  KEY `uid` (`uid`,`dateline`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_common_invite` WRITE;
/*!40000 ALTER TABLE `pre_common_invite` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_common_invite` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_common_log`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_common_log` (
  `id` bigint(20) unsigned NOT NULL AUTO_INCREMENT,
  `uid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `loginname` char(50) NOT NULL DEFAULT '',
  `username` char(50) NOT NULL DEFAULT '',
  `type` varchar(255) NOT NULL DEFAULT '',
  `data` longtext CHARACTER SET utf8mb4 COLLATE utf8mb4_bin NOT NULL CHECK (json_valid(`data`)),
  `operationuid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `source` varchar(255) NOT NULL DEFAULT '',
  `device` longtext CHARACTER SET utf8mb4 COLLATE utf8mb4_bin NOT NULL CHECK (json_valid(`device`)),
  `record` varchar(255) NOT NULL DEFAULT '',
  `dateline` bigint(20) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`id`),
  KEY `uid` (`uid`),
  KEY `dateline` (`dateline`),
  KEY `type` (`type`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_common_log` WRITE;
/*!40000 ALTER TABLE `pre_common_log` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_common_log` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_common_magic`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_common_magic` (
  `magicid` smallint(6) unsigned NOT NULL AUTO_INCREMENT,
  `available` tinyint(1) NOT NULL DEFAULT 0,
  `name` varchar(50) NOT NULL,
  `identifier` varchar(40) NOT NULL,
  `description` varchar(255) NOT NULL,
  `displayorder` tinyint(3) NOT NULL DEFAULT 0,
  `credit` tinyint(1) NOT NULL DEFAULT 0,
  `price` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `num` smallint(6) unsigned NOT NULL DEFAULT 0,
  `salevolume` smallint(6) unsigned NOT NULL DEFAULT 0,
  `supplytype` tinyint(1) NOT NULL DEFAULT 0,
  `supplynum` smallint(6) unsigned NOT NULL DEFAULT 0,
  `useperoid` tinyint(1) NOT NULL DEFAULT 0,
  `usenum` smallint(6) unsigned NOT NULL DEFAULT 0,
  `weight` tinyint(3) unsigned NOT NULL DEFAULT 1,
  `magicperm` text NOT NULL,
  `useevent` tinyint(1) NOT NULL DEFAULT 0,
  PRIMARY KEY (`magicid`),
  UNIQUE KEY `identifier` (`identifier`),
  KEY `displayorder` (`available`,`displayorder`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_common_magic` WRITE;
/*!40000 ALTER TABLE `pre_common_magic` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_common_magic` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_common_magiclog`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_common_magiclog` (
  `uid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `magicid` smallint(6) unsigned NOT NULL DEFAULT 0,
  `action` tinyint(1) NOT NULL DEFAULT 0,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  `amount` smallint(6) unsigned NOT NULL DEFAULT 0,
  `credit` tinyint(3) unsigned NOT NULL DEFAULT 0,
  `price` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `targetid` int(10) unsigned NOT NULL DEFAULT 0,
  `idtype` char(6) DEFAULT NULL,
  `targetuid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  KEY `uid` (`uid`,`dateline`),
  KEY `action` (`action`),
  KEY `targetuid` (`targetuid`,`dateline`),
  KEY `magicid` (`magicid`,`dateline`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_common_magiclog` WRITE;
/*!40000 ALTER TABLE `pre_common_magiclog` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_common_magiclog` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_common_mailcron`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_common_mailcron` (
  `cid` mediumint(8) unsigned NOT NULL AUTO_INCREMENT,
  `touid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `email` varchar(255) NOT NULL DEFAULT '',
  `sendtime` int(10) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`cid`),
  KEY `sendtime` (`sendtime`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_common_mailcron` WRITE;
/*!40000 ALTER TABLE `pre_common_mailcron` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_common_mailcron` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_common_mailqueue`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_common_mailqueue` (
  `qid` mediumint(8) unsigned NOT NULL AUTO_INCREMENT,
  `cid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `subject` text NOT NULL,
  `message` text NOT NULL,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`qid`),
  KEY `mcid` (`cid`,`dateline`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_common_mailqueue` WRITE;
/*!40000 ALTER TABLE `pre_common_mailqueue` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_common_mailqueue` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_common_member`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_common_member` (
  `uid` mediumint(8) unsigned NOT NULL AUTO_INCREMENT,
  `email` varchar(255) NOT NULL DEFAULT '',
  `loginname` char(50) NOT NULL DEFAULT '',
  `username` char(50) NOT NULL DEFAULT '',
  `password` char(32) NOT NULL DEFAULT '',
  `secmobicc` varchar(3) NOT NULL DEFAULT '',
  `secmobile` varchar(12) NOT NULL DEFAULT '',
  `status` tinyint(1) NOT NULL DEFAULT 0,
  `emailstatus` tinyint(1) NOT NULL DEFAULT 0,
  `avatarstatus` tinyint(1) NOT NULL DEFAULT 0,
  `secmobilestatus` tinyint(1) NOT NULL DEFAULT 0,
  `adminid` tinyint(1) NOT NULL DEFAULT 0,
  `groupid` smallint(6) unsigned NOT NULL DEFAULT 0,
  `groupexpiry` int(10) unsigned NOT NULL DEFAULT 0,
  `extgroupids` char(20) NOT NULL DEFAULT '',
  `regdate` int(10) unsigned NOT NULL DEFAULT 0,
  `credits` int(10) NOT NULL DEFAULT 0,
  `notifysound` tinyint(1) NOT NULL DEFAULT 0,
  `timeoffset` char(4) NOT NULL DEFAULT '',
  `newpm` smallint(6) unsigned NOT NULL DEFAULT 0,
  `newprompt` smallint(6) unsigned NOT NULL DEFAULT 0,
  `accessmasks` tinyint(1) NOT NULL DEFAULT 0,
  `allowadmincp` tinyint(1) NOT NULL DEFAULT 0,
  `onlyacceptfriendpm` tinyint(1) NOT NULL DEFAULT 0,
  `conisbind` tinyint(1) NOT NULL DEFAULT 0,
  `freeze` tinyint(1) NOT NULL DEFAULT 0,
  PRIMARY KEY (`uid`),
  UNIQUE KEY `loginname` (`loginname`),
  UNIQUE KEY `username` (`username`),
  KEY `email` (`email`(40)),
  KEY `groupid` (`groupid`),
  KEY `conisbind` (`conisbind`),
  KEY `regdate` (`regdate`),
  KEY `secmobile` (`secmobile`,`secmobicc`)
) ENGINE=InnoDB AUTO_INCREMENT=2 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_common_member` WRITE;
/*!40000 ALTER TABLE `pre_common_member` DISABLE KEYS */;
INSERT INTO `pre_common_member` VALUES
(1,'admin@admin.com','admin','admin','0c909a141f1f2c0a1cb602b0b2d7d050','','',0,0,0,0,1,1,0,'',1784453715,0,0,'9999',0,0,0,1,0,0,0);
/*!40000 ALTER TABLE `pre_common_member` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_common_member_account`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_common_member_account` (
  `id` int(11) unsigned NOT NULL AUTO_INCREMENT,
  `uid` mediumint(8) unsigned NOT NULL,
  `atype` tinyint(1) NOT NULL,
  `account` varchar(255) NOT NULL,
  `create_time` timestamp NOT NULL DEFAULT current_timestamp() ON UPDATE current_timestamp(),
  `bindname` varchar(255) NOT NULL DEFAULT '',
  PRIMARY KEY (`id`),
  UNIQUE KEY `uid` (`uid`,`atype`),
  UNIQUE KEY `atype` (`atype`,`account`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_common_member_account` WRITE;
/*!40000 ALTER TABLE `pre_common_member_account` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_common_member_account` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_common_member_action_log`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_common_member_action_log` (
  `id` int(10) unsigned NOT NULL AUTO_INCREMENT,
  `uid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `action` tinyint(5) NOT NULL DEFAULT 0,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`id`),
  KEY `dateline` (`dateline`,`action`,`uid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_common_member_action_log` WRITE;
/*!40000 ALTER TABLE `pre_common_member_action_log` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_common_member_action_log` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_common_member_count`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_common_member_count` (
  `uid` mediumint(8) unsigned NOT NULL,
  `extcredits1` int(10) NOT NULL DEFAULT 0,
  `extcredits2` int(10) NOT NULL DEFAULT 0,
  `extcredits3` int(10) NOT NULL DEFAULT 0,
  `extcredits4` int(10) NOT NULL DEFAULT 0,
  `extcredits5` int(10) NOT NULL DEFAULT 0,
  `extcredits6` int(10) NOT NULL DEFAULT 0,
  `extcredits7` int(10) NOT NULL DEFAULT 0,
  `extcredits8` int(10) NOT NULL DEFAULT 0,
  `friends` smallint(6) unsigned NOT NULL DEFAULT 0,
  `posts` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `threads` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `digestposts` smallint(6) unsigned NOT NULL DEFAULT 0,
  `doings` smallint(6) unsigned NOT NULL DEFAULT 0,
  `blogs` smallint(6) unsigned NOT NULL DEFAULT 0,
  `albums` smallint(6) unsigned NOT NULL DEFAULT 0,
  `sharings` smallint(6) unsigned NOT NULL DEFAULT 0,
  `attachsize` int(10) unsigned NOT NULL DEFAULT 0,
  `views` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `oltime` smallint(6) unsigned NOT NULL DEFAULT 0,
  `todayattachs` smallint(6) unsigned NOT NULL DEFAULT 0,
  `todayattachsize` int(10) unsigned NOT NULL DEFAULT 0,
  `feeds` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `follower` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `following` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `newfollower` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `blacklist` mediumint(8) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`uid`),
  KEY `posts` (`posts`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_common_member_count` WRITE;
/*!40000 ALTER TABLE `pre_common_member_count` DISABLE KEYS */;
INSERT INTO `pre_common_member_count` VALUES
(1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
/*!40000 ALTER TABLE `pre_common_member_count` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_common_member_crime`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_common_member_crime` (
  `cid` mediumint(8) unsigned NOT NULL AUTO_INCREMENT,
  `uid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `operatorid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `operator` varchar(50) NOT NULL,
  `action` tinyint(5) NOT NULL,
  `reason` text NOT NULL,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`cid`),
  KEY `uid` (`uid`,`action`,`dateline`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_common_member_crime` WRITE;
/*!40000 ALTER TABLE `pre_common_member_crime` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_common_member_crime` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_common_member_field_forum`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_common_member_field_forum` (
  `uid` mediumint(8) unsigned NOT NULL,
  `publishfeed` tinyint(3) NOT NULL DEFAULT 0,
  `customshow` tinyint(3) unsigned NOT NULL DEFAULT 26,
  `customstatus` varchar(30) NOT NULL DEFAULT '',
  `medals` text NOT NULL,
  `sightml` text NOT NULL,
  `groupterms` text NOT NULL,
  `authstr` varchar(255) NOT NULL DEFAULT '',
  `groups` mediumtext NOT NULL,
  `attentiongroup` varchar(255) NOT NULL DEFAULT '',
  PRIMARY KEY (`uid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_common_member_field_forum` WRITE;
/*!40000 ALTER TABLE `pre_common_member_field_forum` DISABLE KEYS */;
INSERT INTO `pre_common_member_field_forum` VALUES
(1,0,26,'','','','','','','');
/*!40000 ALTER TABLE `pre_common_member_field_forum` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_common_member_field_home`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_common_member_field_home` (
  `uid` mediumint(8) unsigned NOT NULL,
  `spacename` varchar(255) NOT NULL DEFAULT '',
  `spacedescription` varchar(255) NOT NULL DEFAULT '',
  `domain` char(15) NOT NULL DEFAULT '',
  `addsize` int(10) unsigned NOT NULL DEFAULT 0,
  `addfriend` smallint(6) unsigned NOT NULL DEFAULT 0,
  `allowasfriend` tinyint(1) NOT NULL DEFAULT 1,
  `allowasfollow` tinyint(1) NOT NULL DEFAULT 1,
  `menunum` smallint(6) unsigned NOT NULL DEFAULT 0,
  `theme` varchar(20) NOT NULL DEFAULT '',
  `spacecss` text NOT NULL,
  `blockposition` text NOT NULL,
  `recentnote` text NOT NULL,
  `spacenote` text NOT NULL,
  `privacy` text NOT NULL,
  `feedfriend` mediumtext NOT NULL,
  `acceptemail` text NOT NULL,
  `magicgift` text NOT NULL,
  `stickblogs` text NOT NULL,
  PRIMARY KEY (`uid`),
  KEY `domain` (`domain`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_common_member_field_home` WRITE;
/*!40000 ALTER TABLE `pre_common_member_field_home` DISABLE KEYS */;
INSERT INTO `pre_common_member_field_home` VALUES
(1,'','','',0,0,1,1,0,'','','','','','','','','','');
/*!40000 ALTER TABLE `pre_common_member_field_home` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_common_member_forum_buylog`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_common_member_forum_buylog` (
  `uid` mediumint(8) unsigned NOT NULL,
  `fid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `credits` int(10) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`uid`,`fid`),
  KEY `fid` (`fid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_common_member_forum_buylog` WRITE;
/*!40000 ALTER TABLE `pre_common_member_forum_buylog` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_common_member_forum_buylog` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_common_member_grouppm`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_common_member_grouppm` (
  `uid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `gpmid` smallint(6) unsigned NOT NULL AUTO_INCREMENT,
  `status` tinyint(1) NOT NULL DEFAULT 0,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`uid`,`gpmid`),
  KEY `gpmid` (`gpmid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_common_member_grouppm` WRITE;
/*!40000 ALTER TABLE `pre_common_member_grouppm` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_common_member_grouppm` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_common_member_magic`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_common_member_magic` (
  `uid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `magicid` smallint(6) unsigned NOT NULL DEFAULT 0,
  `num` smallint(6) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`uid`,`magicid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_common_member_magic` WRITE;
/*!40000 ALTER TABLE `pre_common_member_magic` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_common_member_magic` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_common_member_medal`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_common_member_medal` (
  `uid` mediumint(8) unsigned NOT NULL,
  `medalid` smallint(6) unsigned NOT NULL,
  PRIMARY KEY (`uid`,`medalid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_common_member_medal` WRITE;
/*!40000 ALTER TABLE `pre_common_member_medal` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_common_member_medal` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_common_member_newprompt`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_common_member_newprompt` (
  `uid` mediumint(8) unsigned NOT NULL,
  `data` varchar(255) NOT NULL DEFAULT '',
  PRIMARY KEY (`uid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_common_member_newprompt` WRITE;
/*!40000 ALTER TABLE `pre_common_member_newprompt` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_common_member_newprompt` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_common_member_profile`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_common_member_profile` (
  `uid` mediumint(8) unsigned NOT NULL,
  `realname` varchar(255) NOT NULL DEFAULT '',
  `gender` tinyint(1) NOT NULL DEFAULT 0,
  `birthyear` smallint(6) unsigned NOT NULL DEFAULT 0,
  `birthmonth` tinyint(3) unsigned NOT NULL DEFAULT 0,
  `birthday` tinyint(3) unsigned NOT NULL DEFAULT 0,
  `constellation` varchar(255) NOT NULL DEFAULT '',
  `zodiac` varchar(255) NOT NULL DEFAULT '',
  `telephone` varchar(255) NOT NULL DEFAULT '',
  `mobile` varchar(255) NOT NULL DEFAULT '',
  `idcardtype` varchar(255) NOT NULL DEFAULT '',
  `idcard` varchar(255) NOT NULL DEFAULT '',
  `address` varchar(255) NOT NULL DEFAULT '',
  `zipcode` varchar(255) NOT NULL DEFAULT '',
  `nationality` varchar(255) NOT NULL DEFAULT '',
  `birthcountry` varchar(255) NOT NULL DEFAULT '',
  `birthprovince` varchar(255) NOT NULL DEFAULT '',
  `birthcity` varchar(255) NOT NULL DEFAULT '',
  `birthdist` varchar(20) NOT NULL DEFAULT '',
  `birthcommunity` varchar(255) NOT NULL DEFAULT '',
  `residecountry` varchar(255) NOT NULL DEFAULT '',
  `resideprovince` varchar(255) NOT NULL DEFAULT '',
  `residecity` varchar(255) NOT NULL DEFAULT '',
  `residedist` varchar(20) NOT NULL DEFAULT '',
  `residecommunity` varchar(255) NOT NULL DEFAULT '',
  `residesuite` varchar(255) NOT NULL DEFAULT '',
  `graduateschool` varchar(255) NOT NULL DEFAULT '',
  `company` varchar(255) NOT NULL DEFAULT '',
  `education` varchar(255) NOT NULL DEFAULT '',
  `occupation` varchar(255) NOT NULL DEFAULT '',
  `position` varchar(255) NOT NULL DEFAULT '',
  `revenue` varchar(255) NOT NULL DEFAULT '',
  `affectivestatus` varchar(255) NOT NULL DEFAULT '',
  `lookingfor` varchar(255) NOT NULL DEFAULT '',
  `bloodtype` varchar(255) NOT NULL DEFAULT '',
  `height` varchar(255) NOT NULL DEFAULT '',
  `weight` varchar(255) NOT NULL DEFAULT '',
  `alipay` varchar(255) NOT NULL DEFAULT '',
  `icq` varchar(255) NOT NULL DEFAULT '',
  `qq` varchar(255) NOT NULL DEFAULT '',
  `yahoo` varchar(255) NOT NULL DEFAULT '',
  `msn` varchar(255) NOT NULL DEFAULT '',
  `taobao` varchar(255) NOT NULL DEFAULT '',
  `site` varchar(255) NOT NULL DEFAULT '',
  `bio` text NOT NULL,
  `interest` text NOT NULL,
  `field1` text NOT NULL,
  `field2` text NOT NULL,
  `field3` text NOT NULL,
  `field4` text NOT NULL,
  `field5` text NOT NULL,
  `field6` text NOT NULL,
  `field7` text NOT NULL,
  `field8` text NOT NULL,
  `fields` longtext CHARACTER SET utf8mb4 COLLATE utf8mb4_bin NOT NULL CHECK (json_valid(`fields`)),
  PRIMARY KEY (`uid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_common_member_profile` WRITE;
/*!40000 ALTER TABLE `pre_common_member_profile` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_common_member_profile` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_common_member_profile_history`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_common_member_profile_history` (
  `hid` int(10) unsigned NOT NULL AUTO_INCREMENT,
  `uid` mediumint(8) unsigned NOT NULL,
  `realname` varchar(255) NOT NULL DEFAULT '',
  `gender` tinyint(1) NOT NULL DEFAULT 0,
  `birthyear` smallint(6) unsigned NOT NULL DEFAULT 0,
  `birthmonth` tinyint(3) unsigned NOT NULL DEFAULT 0,
  `birthday` tinyint(3) unsigned NOT NULL DEFAULT 0,
  `constellation` varchar(255) NOT NULL DEFAULT '',
  `zodiac` varchar(255) NOT NULL DEFAULT '',
  `telephone` varchar(255) NOT NULL DEFAULT '',
  `mobile` varchar(255) NOT NULL DEFAULT '',
  `idcardtype` varchar(255) NOT NULL DEFAULT '',
  `idcard` varchar(255) NOT NULL DEFAULT '',
  `address` varchar(255) NOT NULL DEFAULT '',
  `zipcode` varchar(255) NOT NULL DEFAULT '',
  `nationality` varchar(255) NOT NULL DEFAULT '',
  `birthcountry` varchar(255) NOT NULL DEFAULT '',
  `birthprovince` varchar(255) NOT NULL DEFAULT '',
  `birthcity` varchar(255) NOT NULL DEFAULT '',
  `birthdist` varchar(20) NOT NULL DEFAULT '',
  `birthcommunity` varchar(255) NOT NULL DEFAULT '',
  `residecountry` varchar(255) NOT NULL DEFAULT '',
  `resideprovince` varchar(255) NOT NULL DEFAULT '',
  `residecity` varchar(255) NOT NULL DEFAULT '',
  `residedist` varchar(20) NOT NULL DEFAULT '',
  `residecommunity` varchar(255) NOT NULL DEFAULT '',
  `residesuite` varchar(255) NOT NULL DEFAULT '',
  `graduateschool` varchar(255) NOT NULL DEFAULT '',
  `company` varchar(255) NOT NULL DEFAULT '',
  `education` varchar(255) NOT NULL DEFAULT '',
  `occupation` varchar(255) NOT NULL DEFAULT '',
  `position` varchar(255) NOT NULL DEFAULT '',
  `revenue` varchar(255) NOT NULL DEFAULT '',
  `affectivestatus` varchar(255) NOT NULL DEFAULT '',
  `lookingfor` varchar(255) NOT NULL DEFAULT '',
  `bloodtype` varchar(255) NOT NULL DEFAULT '',
  `height` varchar(255) NOT NULL DEFAULT '',
  `weight` varchar(255) NOT NULL DEFAULT '',
  `alipay` varchar(255) NOT NULL DEFAULT '',
  `icq` varchar(255) NOT NULL DEFAULT '',
  `qq` varchar(255) NOT NULL DEFAULT '',
  `yahoo` varchar(255) NOT NULL DEFAULT '',
  `msn` varchar(255) NOT NULL DEFAULT '',
  `taobao` varchar(255) NOT NULL DEFAULT '',
  `site` varchar(255) NOT NULL DEFAULT '',
  `bio` text NOT NULL,
  `interest` text NOT NULL,
  `field1` text NOT NULL,
  `field2` text NOT NULL,
  `field3` text NOT NULL,
  `field4` text NOT NULL,
  `field5` text NOT NULL,
  `field6` text NOT NULL,
  `field7` text NOT NULL,
  `field8` text NOT NULL,
  `fields` longtext CHARACTER SET utf8mb4 COLLATE utf8mb4_bin NOT NULL CHECK (json_valid(`fields`)),
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`hid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_common_member_profile_history` WRITE;
/*!40000 ALTER TABLE `pre_common_member_profile_history` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_common_member_profile_history` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_common_member_profile_setting`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_common_member_profile_setting` (
  `fieldid` varchar(190) NOT NULL DEFAULT '',
  `available` tinyint(1) NOT NULL DEFAULT 0,
  `invisible` tinyint(1) NOT NULL DEFAULT 0,
  `needverify` tinyint(1) NOT NULL DEFAULT 0,
  `title` varchar(255) NOT NULL DEFAULT '',
  `description` varchar(255) NOT NULL DEFAULT '',
  `displayorder` smallint(6) unsigned NOT NULL DEFAULT 0,
  `required` tinyint(1) NOT NULL DEFAULT 0,
  `unchangeable` tinyint(1) NOT NULL DEFAULT 0,
  `showincard` tinyint(1) NOT NULL DEFAULT 0,
  `showinthread` tinyint(1) NOT NULL DEFAULT 0,
  `showinregister` tinyint(1) NOT NULL DEFAULT 0,
  `allowsearch` tinyint(1) NOT NULL DEFAULT 0,
  `formtype` varchar(255) NOT NULL,
  `size` smallint(6) unsigned NOT NULL DEFAULT 0,
  `choices` text NOT NULL,
  `validate` text NOT NULL,
  `encrypt` tinyint(1) NOT NULL DEFAULT 0,
  PRIMARY KEY (`fieldid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_common_member_profile_setting` WRITE;
/*!40000 ALTER TABLE `pre_common_member_profile_setting` DISABLE KEYS */;
INSERT INTO `pre_common_member_profile_setting` VALUES
('address',1,1,0,'閭瘎鍦板潃','',0,0,0,0,0,0,0,'text',0,'','',0),
('affectivestatus',1,1,0,'鎯呮劅鐘舵€?,'',0,0,0,0,0,0,0,'text',0,'','',0),
('alipay',1,1,0,'鏀粯瀹?,'',0,0,0,0,0,0,0,'text',0,'','',0),
('bio',1,1,0,'鑷垜浠嬬粛','',0,0,0,0,0,0,0,'textarea',0,'','',0),
('birthcity',1,0,0,'鍑虹敓鍦?,'',0,0,0,0,0,0,0,'select',0,'','',0),
('birthcommunity',1,0,0,'鍑虹敓灏忓尯','',0,0,0,0,0,0,0,'select',0,'','',0),
('birthcountry',1,0,0,'鍑虹敓鍥藉','',0,0,0,0,0,0,0,'select',0,'','',0),
('birthday',1,0,0,'鐢熸棩','',0,0,0,0,0,0,0,'select',0,'','',0),
('birthdist',1,0,0,'鍑虹敓鍘?,'鍑虹敓琛屾斂鍖?鍘?,0,0,0,0,0,0,0,'select',0,'','',0),
('birthmonth',1,0,0,'鍑虹敓鏈堜唤','',0,0,0,0,0,0,0,'select',0,'','',0),
('birthprovince',1,0,0,'鍑虹敓鐪佷唤','',0,0,0,0,0,0,0,'select',0,'','',0),
('birthyear',1,0,0,'鍑虹敓骞翠唤','',0,0,0,0,0,0,1,'select',0,'','',0),
('bloodtype',1,1,0,'琛€鍨?,'',0,0,0,0,0,0,0,'select',0,'A\r\nB\r\nAB\r\nO\r\n鍏跺畠','',0),
('company',1,0,0,'鍏徃','',0,0,0,0,0,0,0,'text',0,'','',0),
('constellation',1,1,0,'鏄熷骇','鏄熷骇(鏍规嵁鐢熸棩鑷姩璁＄畻)',0,0,0,0,0,0,0,'text',0,'','',0),
('education',1,0,0,'瀛﹀巻','',0,0,0,0,0,0,0,'select',0,'鍗氬＋\r\n纭曞＋\r\n鏈\r\n涓撶\r\n涓\r\n灏忓\r\n鍏跺畠','',0),
('field1',0,1,0,'鑷畾涔夊瓧娈?','',0,0,0,0,0,0,0,'text',0,'','',0),
('field2',0,1,0,'鑷畾涔夊瓧娈?','',0,0,0,0,0,0,0,'text',0,'','',0),
('field3',0,1,0,'鑷畾涔夊瓧娈?','',0,0,0,0,0,0,0,'text',0,'','',0),
('field4',0,1,0,'鑷畾涔夊瓧娈?','',0,0,0,0,0,0,0,'text',0,'','',0),
('field5',0,1,0,'鑷畾涔夊瓧娈?','',0,0,0,0,0,0,0,'text',0,'','',0),
('field6',0,1,0,'鑷畾涔夊瓧娈?','',0,0,0,0,0,0,0,'text',0,'','',0),
('field7',0,1,0,'鑷畾涔夊瓧娈?','',0,0,0,0,0,0,0,'text',0,'','',0),
('field8',0,1,0,'鑷畾涔夊瓧娈?','',0,0,0,0,0,0,0,'text',0,'','',0),
('fields',1,1,0,'鏇村鑷畾涔夎祫鏂?,'',0,0,0,0,0,0,0,'json',0,'','',0),
('gender',1,0,0,'鎬у埆','',0,0,0,0,0,0,1,'select',0,'','',0),
('graduateschool',1,0,0,'姣曚笟瀛︽牎','',0,0,0,0,0,0,0,'text',0,'','',0),
('height',0,1,0,'韬珮','鍗曚綅 cm',0,0,0,0,0,0,0,'text',0,'','',0),
('icq',0,1,0,'ICQ','',0,0,0,0,0,0,0,'text',0,'','',0),
('idcard',1,1,0,'璇佷欢鍙?,'',0,0,0,0,0,0,0,'text',0,'','',0),
('idcardtype',1,1,0,'璇佷欢绫诲瀷','韬唤璇?鎶ょ収 椹鹃┒璇佺瓑',0,0,0,0,0,0,0,'select',0,'韬唤璇乗r\n鎶ょ収\r\n椹鹃┒璇?,'',0),
('interest',1,0,0,'鍏磋叮鐖卞ソ','',0,0,0,0,0,0,0,'textarea',0,'','',0),
('lookingfor',1,0,0,'浜ゅ弸鐩殑','甯屾湜鍦ㄧ綉绔欐壘鍒颁粈涔堟牱鐨勬湅鍙?,0,0,0,0,0,0,0,'text',0,'','',0),
('mobile',1,1,0,'鎵嬫満','',0,0,0,0,0,0,0,'text',0,'','',0),
('msn',1,1,0,'MSN','',0,0,0,0,0,0,0,'text',0,'','',0),
('nationality',0,0,0,'鍥界睄','',0,0,0,0,0,0,0,'text',0,'','',0),
('occupation',1,0,0,'鑱屼笟','',0,0,0,0,0,0,0,'text',0,'','',0),
('position',1,0,0,'鑱屼綅','',0,0,0,0,0,0,0,'text',0,'','',0),
('qq',1,1,0,'QQ','',0,0,0,0,0,0,0,'text',0,'','',0),
('realname',1,0,0,'鐪熷疄濮撳悕','',0,0,0,0,0,0,1,'text',0,'','',0),
('residecity',1,0,0,'灞呬綇鍦?,'',0,0,0,0,0,0,0,'select',0,'','',0),
('residecommunity',1,0,0,'灞呬綇灏忓尯','',0,0,0,0,0,0,0,'select',0,'','',0),
('residecountry',1,0,0,'灞呬綇鍥藉','',0,0,0,0,0,0,0,'select',0,'','',0),
('residedist',1,0,0,'灞呬綇鍘?,'灞呬綇琛屾斂鍖?鍘?,0,0,0,0,0,0,0,'select',0,'','',0),
('resideprovince',1,0,0,'灞呬綇鐪佷唤','',0,0,0,0,0,0,0,'select',0,'','',0),
('residesuite',0,0,0,'鎴块棿','灏忓尯銆佸啓瀛楁ゼ闂ㄧ墝鍙?,0,0,0,0,0,0,0,'text',0,'','',0),
('revenue',1,1,0,'骞存敹鍏?,'鍗曚綅 鍏?,0,0,0,0,0,0,0,'text',0,'','',0),
('site',1,0,0,'涓汉涓婚〉','',0,0,0,0,0,0,0,'text',0,'','',0),
('taobao',1,1,0,'闃块噷鏃烘椇','',0,0,0,0,0,0,0,'text',0,'','',0),
('telephone',1,1,0,'鍥哄畾鐢佃瘽','',0,0,0,0,0,0,0,'text',0,'','',0),
('weight',0,1,0,'浣撻噸','鍗曚綅 kg',0,0,0,0,0,0,0,'text',0,'','',0),
('yahoo',0,1,0,'YAHOO璐﹀彿','',0,0,0,0,0,0,0,'text',0,'','',0),
('zipcode',1,1,0,'閭紪','',0,0,0,0,0,0,0,'text',0,'','',0),
('zodiac',1,1,0,'鐢熻倴','鐢熻倴(鏍规嵁鐢熸棩鑷姩璁＄畻)',0,0,0,0,0,0,0,'text',0,'','',0);
/*!40000 ALTER TABLE `pre_common_member_profile_setting` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_common_member_security`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_common_member_security` (
  `securityid` mediumint(8) unsigned NOT NULL AUTO_INCREMENT,
  `uid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `username` varchar(255) NOT NULL DEFAULT '',
  `fieldid` varchar(255) NOT NULL DEFAULT '',
  `oldvalue` text NOT NULL,
  `newvalue` text NOT NULL,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`securityid`),
  KEY `uid` (`uid`,`fieldid`(40)),
  KEY `dateline` (`dateline`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_common_member_security` WRITE;
/*!40000 ALTER TABLE `pre_common_member_security` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_common_member_security` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_common_member_secwhite`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_common_member_secwhite` (
  `uid` int(10) NOT NULL,
  `dateline` int(10) NOT NULL,
  PRIMARY KEY (`uid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_common_member_secwhite` WRITE;
/*!40000 ALTER TABLE `pre_common_member_secwhite` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_common_member_secwhite` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_common_member_stat_field`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_common_member_stat_field` (
  `optionid` mediumint(8) unsigned NOT NULL AUTO_INCREMENT,
  `fieldid` varchar(255) NOT NULL DEFAULT '',
  `fieldvalue` varchar(255) NOT NULL DEFAULT '',
  `hash` varchar(255) NOT NULL DEFAULT '',
  `users` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `updatetime` int(10) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`optionid`),
  KEY `fieldid` (`fieldid`(40))
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_common_member_stat_field` WRITE;
/*!40000 ALTER TABLE `pre_common_member_stat_field` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_common_member_stat_field` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_common_member_status`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_common_member_status` (
  `uid` mediumint(8) unsigned NOT NULL,
  `regip` varchar(45) NOT NULL DEFAULT '',
  `lastip` varchar(45) NOT NULL DEFAULT '',
  `regport` smallint(6) unsigned NOT NULL DEFAULT 0,
  `port` smallint(6) unsigned NOT NULL DEFAULT 0,
  `lastvisit` int(10) unsigned NOT NULL DEFAULT 0,
  `lastactivity` int(10) unsigned NOT NULL DEFAULT 0,
  `lastpost` int(10) unsigned NOT NULL DEFAULT 0,
  `lastsendmail` int(10) unsigned NOT NULL DEFAULT 0,
  `invisible` tinyint(1) NOT NULL DEFAULT 0,
  `buyercredit` smallint(6) NOT NULL DEFAULT 0,
  `sellercredit` smallint(6) NOT NULL DEFAULT 0,
  `favtimes` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `sharetimes` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `profileprogress` tinyint(2) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`uid`),
  KEY `lastactivity` (`lastactivity`,`invisible`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_common_member_status` WRITE;
/*!40000 ALTER TABLE `pre_common_member_status` DISABLE KEYS */;
INSERT INTO `pre_common_member_status` VALUES
(1,'','',0,0,0,0,0,0,0,0,0,0,0,0);
/*!40000 ALTER TABLE `pre_common_member_status` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_common_member_username_history`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_common_member_username_history` (
  `username` char(50) NOT NULL DEFAULT '',
  `uid` mediumint(8) unsigned NOT NULL,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`username`),
  KEY `uid` (`uid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_common_member_username_history` WRITE;
/*!40000 ALTER TABLE `pre_common_member_username_history` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_common_member_username_history` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_common_member_validate`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_common_member_validate` (
  `uid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `submitdate` int(10) unsigned NOT NULL DEFAULT 0,
  `moddate` int(10) unsigned NOT NULL DEFAULT 0,
  `admin` varchar(50) NOT NULL DEFAULT '',
  `submittimes` tinyint(3) unsigned NOT NULL DEFAULT 0,
  `status` tinyint(1) NOT NULL DEFAULT 0,
  `message` text NOT NULL,
  `remark` text NOT NULL,
  PRIMARY KEY (`uid`),
  KEY `status` (`status`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_common_member_validate` WRITE;
/*!40000 ALTER TABLE `pre_common_member_validate` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_common_member_validate` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_common_member_verify`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_common_member_verify` (
  `uid` mediumint(8) unsigned NOT NULL,
  `verify1` tinyint(1) NOT NULL DEFAULT 0,
  `verify2` tinyint(1) NOT NULL DEFAULT 0,
  `verify3` tinyint(1) NOT NULL DEFAULT 0,
  `verify4` tinyint(1) NOT NULL DEFAULT 0,
  `verify5` tinyint(1) NOT NULL DEFAULT 0,
  `verify6` tinyint(1) NOT NULL DEFAULT 0,
  PRIMARY KEY (`uid`),
  KEY `verify1` (`verify1`),
  KEY `verify2` (`verify2`),
  KEY `verify3` (`verify3`),
  KEY `verify4` (`verify4`),
  KEY `verify5` (`verify5`),
  KEY `verify6` (`verify6`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_common_member_verify` WRITE;
/*!40000 ALTER TABLE `pre_common_member_verify` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_common_member_verify` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_common_member_verify_info`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_common_member_verify_info` (
  `vid` mediumint(8) unsigned NOT NULL AUTO_INCREMENT,
  `uid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `username` varchar(100) NOT NULL DEFAULT '',
  `verifytype` tinyint(1) NOT NULL DEFAULT 0,
  `flag` tinyint(1) NOT NULL DEFAULT 0,
  `field` text NOT NULL,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`vid`),
  KEY `verifytype` (`verifytype`,`flag`),
  KEY `uid` (`uid`,`verifytype`,`dateline`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_common_member_verify_info` WRITE;
/*!40000 ALTER TABLE `pre_common_member_verify_info` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_common_member_verify_info` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_common_mytask`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_common_mytask` (
  `uid` mediumint(8) unsigned NOT NULL,
  `username` char(50) NOT NULL DEFAULT '',
  `taskid` smallint(6) unsigned NOT NULL,
  `status` tinyint(1) NOT NULL DEFAULT 0,
  `csc` char(255) NOT NULL DEFAULT '',
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`uid`,`taskid`),
  KEY `parter` (`taskid`,`dateline`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_common_mytask` WRITE;
/*!40000 ALTER TABLE `pre_common_mytask` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_common_mytask` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_common_nav`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_common_nav` (
  `id` smallint(6) unsigned NOT NULL AUTO_INCREMENT,
  `parentid` smallint(6) unsigned NOT NULL DEFAULT 0,
  `name` varchar(255) NOT NULL,
  `title` varchar(255) NOT NULL,
  `url` varchar(255) NOT NULL,
  `identifier` varchar(255) NOT NULL,
  `target` tinyint(1) NOT NULL DEFAULT 0,
  `type` tinyint(1) NOT NULL DEFAULT 0,
  `available` tinyint(1) NOT NULL DEFAULT 0,
  `displayorder` tinyint(3) NOT NULL,
  `highlight` tinyint(1) NOT NULL DEFAULT 0,
  `level` tinyint(1) NOT NULL DEFAULT 0,
  `subtype` tinyint(1) NOT NULL DEFAULT 0,
  `subcols` tinyint(1) NOT NULL DEFAULT 0,
  `icon` varchar(255) NOT NULL,
  `subname` varchar(255) NOT NULL,
  `suburl` varchar(255) NOT NULL,
  `navtype` tinyint(1) NOT NULL DEFAULT 0,
  `logo` varchar(255) NOT NULL,
  PRIMARY KEY (`id`),
  KEY `navtype` (`navtype`)
) ENGINE=InnoDB AUTO_INCREMENT=53 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_common_nav` WRITE;
/*!40000 ALTER TABLE `pre_common_nav` DISABLE KEYS */;
INSERT INTO `pre_common_nav` VALUES
(1,0,'闂ㄦ埛','Portal','portal.php','1',0,0,-1,1,0,0,0,0,'','','',0,''),
(2,0,'璁哄潧','BBS','forum.php','2',0,0,1,2,0,0,0,0,'','','',0,''),
(3,0,'鍦堝瓙','Group','group.php','3',0,0,-1,7,0,0,0,0,'','','',0,''),
(4,0,'鍔ㄦ€?,'Space','home.php','4',0,0,-1,8,0,0,0,0,'','','',0,''),
(5,0,'鎻掍欢','Plugin','#','6',0,0,1,9,0,0,0,0,'','','',0,''),
(6,0,'甯姪','Help','misc.php?mod=faq','7',0,0,0,10,0,0,0,0,'','','',0,''),
(7,0,'鎺掕姒?,'Ranklist','misc.php?mod=ranklist','8',0,0,-1,16,0,0,0,0,'','','',0,''),
(8,0,'骞挎挱','Follow','home.php?mod=follow','9',0,0,-1,5,0,0,0,0,'','','',0,''),
(9,0,'瀵艰','Guide','forum.php?mod=guide','10',0,0,-1,3,0,0,0,0,'','','',0,''),
(10,0,'娣樺笘','Collection','forum.php?mod=collection','11',0,0,-1,11,0,0,0,0,'','','',0,''),
(11,0,'鏃ュ織','Blog','home.php?mod=space&do=blog','12',0,0,-1,12,0,0,0,0,'','','',0,''),
(12,0,'鐩稿唽','Album','home.php?mod=space&do=album','13',0,0,-1,13,0,0,0,0,'','','',0,''),
(13,0,'鍒嗕韩','Share','home.php?mod=space&do=share','14',0,0,-1,14,0,0,0,0,'','','',0,''),
(14,0,'璁板綍','Doing','home.php?mod=space&do=doing','15',0,0,-1,15,0,0,0,0,'','','',0,''),
(15,0,'棣栭〉','Index','forum.php?mod=forumdisplay&fid=0','16',0,0,0,16,0,0,0,0,'','','',0,''),
(16,0,'绔欑偣缁熻','','misc.php?mod=stat','stat',0,0,1,1,0,0,0,0,'','','',1,''),
(17,0,'涓炬姤','','#','report',0,0,1,2,0,0,0,0,'','','',1,''),
(18,0,'Archiver','','archiver/','archiver',0,0,1,3,0,0,0,0,'','','',1,''),
(19,0,'鎵嬫満鐗?,'','forum.php?showmobile=yes','mobile',0,0,1,3,0,0,0,0,'','','',1,''),
(20,0,'灏忛粦灞?,'','misc.php?mod=darkroom','darkroom',0,0,1,3,0,0,0,0,'','','',1,''),
(21,0,'濂藉弸','','home.php?mod=space&do=friend','friend',0,0,-1,1,0,0,0,0,'{STATICURL}image/app/friend.svg','','',3,''),
(22,0,'甯栧瓙','','home.php?mod=space&do=thread&view=me','thread',0,0,1,2,0,0,0,0,'{STATICURL}image/app/forum.svg','','',3,''),
(23,0,'鏀惰棌','','home.php?mod=space&do=favorite&view=me','favorite',0,0,-1,3,0,0,0,0,'{STATICURL}image/app/favorite.svg','','',3,''),
(24,0,'閬撳叿','','home.php?mod=magic','magic',0,0,-1,4,0,0,0,0,'{STATICURL}image/app/magic.svg','','',3,''),
(25,0,'鍕嬬珷','','home.php?mod=medal','medal',0,0,-1,5,0,0,0,0,'{STATICURL}image/app/medal.svg','','',3,''),
(26,0,'浠诲姟','','home.php?mod=task','task',0,0,-1,6,0,0,0,0,'{STATICURL}image/app/task.svg','','',3,''),
(27,0,'娣樺笘','','forum.php?mod=collection&op=my','collection',0,0,-1,7,0,0,0,0,'{STATICURL}image/app/collection.svg','','',3,''),
(28,0,'鍔ㄦ€?,'','home.php','feed',0,0,-1,8,0,0,0,0,'{STATICURL}image/app/feed.svg','','',3,''),
(29,0,'鏃ュ織','','home.php?mod=space&do=blog','blog',0,0,-1,9,0,0,0,0,'{STATICURL}image/app/blog.svg','','',3,''),
(30,0,'鐩稿唽','','home.php?mod=space&do=album','album',0,0,-1,10,0,0,0,0,'{STATICURL}image/app/album.svg','','',3,''),
(31,0,'鍒嗕韩','','home.php?mod=space&do=share','share',0,0,-1,11,0,0,0,0,'{STATICURL}image/app/share.svg','','',3,''),
(32,0,'璁板綍','','home.php?mod=space&do=doing','doing',0,0,-1,12,0,0,0,0,'{STATICURL}image/app/doing.svg','','',3,''),
(33,0,'鐣欒█鏉?,'','home.php?mod=space&do=wall','wall',0,0,-1,13,0,0,0,0,'{STATICURL}image/app/wall.svg','','',3,''),
(34,0,'骞挎挱','','home.php?mod=follow','follow',0,0,-1,14,0,0,0,0,'{STATICURL}image/app/follow.svg','','',3,''),
(35,0,'鍦堝瓙','','group.php','group',0,0,-1,15,0,0,0,0,'{STATICURL}image/app/group.svg','','',3,''),
(36,0,'闂ㄦ埛','','portal.php','portal',0,0,-1,16,0,0,0,0,'{STATICURL}image/app/portal.svg','','',3,''),
(37,0,'瀵艰','','forum.php?mod=guide','guide',0,0,-1,17,0,0,0,0,'{STATICURL}image/app/guide.svg','','',3,''),
(38,0,'鎺掕姒?,'','misc.php?mod=ranklist','ranklist',0,0,-1,18,0,0,0,0,'{STATICURL}image/app/ranklist.svg','','',3,''),
(41,0,'鎼滅储','','search.php?mod=forum','search',0,0,1,1,0,0,0,0,'','','',5,''),
(42,0,'鍏憡','','forum.php?mod=announcement','announcement',0,0,1,1,0,0,0,0,'','','',5,''),
(43,0,'瀵艰','','forum.php?mod=guide&view=newthread','guide',0,0,0,1,0,0,0,0,'','','',5,''),
(44,0,'璧勮','','portal.php?mod=list&catid=1','portal',0,0,0,1,0,0,0,0,'','','',5,''),
(45,0,'鏃ュ織','','home.php?mod=space&do=blog','blog',0,0,0,1,0,0,0,0,'','','',5,''),
(46,0,'鍒嗕韩','','home.php?mod=space&do=share','share',0,0,0,1,0,0,0,0,'','','',5,''),
(47,0,'鎺掕姒?,'','misc.php?mod=ranklist','ranklist',0,0,0,1,0,0,0,0,'','','',5,''),
(48,0,'棣栭〉','','index.php','',0,1,1,1,0,0,0,0,'/static/image/mobile/touch/home.svg','','',6,''),
(49,0,'璁哄潧','','forum.php','',0,1,1,2,0,0,0,0,'/static/image/mobile/touch/forum.svg','','',6,''),
(50,0,'鍙戝竷','','forum.php?mod=misc&action=nav','post',0,1,1,3,0,0,0,0,'/static/image/mobile/touch/plus_btn.svg','','',6,''),
(51,0,'鍙戠幇','','forum.php?mod=find','',0,1,1,4,0,0,0,0,'/static/image/mobile/touch/explore.svg','','',6,''),
(52,0,'鎴戠殑','','home.php?mod=space','',0,1,1,5,0,0,0,0,'/static/image/mobile/touch/space.svg','','',6,'');
/*!40000 ALTER TABLE `pre_common_nav` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_common_onlinetime`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_common_onlinetime` (
  `uid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `thismonth` smallint(6) unsigned NOT NULL DEFAULT 0,
  `total` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `lastupdate` int(10) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`uid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_common_onlinetime` WRITE;
/*!40000 ALTER TABLE `pre_common_onlinetime` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_common_onlinetime` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_common_optimizer`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_common_optimizer` (
  `k` char(100) NOT NULL DEFAULT '',
  `v` char(255) NOT NULL DEFAULT '',
  PRIMARY KEY (`k`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_common_optimizer` WRITE;
/*!40000 ALTER TABLE `pre_common_optimizer` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_common_optimizer` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_common_patch`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_common_patch` (
  `serial` varchar(10) NOT NULL DEFAULT '',
  `rule` text NOT NULL,
  `note` text NOT NULL,
  `status` tinyint(1) NOT NULL DEFAULT 0,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`serial`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_common_patch` WRITE;
/*!40000 ALTER TABLE `pre_common_patch` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_common_patch` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_common_payment_order`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_common_payment_order` (
  `id` int(10) unsigned NOT NULL AUTO_INCREMENT,
  `out_biz_no` varchar(64) NOT NULL,
  `type` varchar(190) NOT NULL,
  `type_name` varchar(255) DEFAULT NULL,
  `uid` int(10) unsigned NOT NULL DEFAULT 0,
  `amount` int(10) unsigned NOT NULL,
  `amount_fee` int(10) unsigned NOT NULL,
  `subject` varchar(255) NOT NULL,
  `description` varchar(255) DEFAULT NULL,
  `expire_time` int(10) unsigned NOT NULL,
  `status` tinyint(1) NOT NULL,
  `return_url` varchar(255) DEFAULT NULL,
  `data` text DEFAULT NULL,
  `clientip` varchar(255) NOT NULL DEFAULT '',
  `remoteport` smallint(6) unsigned NOT NULL DEFAULT 0,
  `dateline` int(10) unsigned NOT NULL,
  `trade_no` varchar(255) DEFAULT NULL,
  `channel` varchar(255) DEFAULT NULL,
  `payment_time` int(10) unsigned DEFAULT NULL,
  `callback_status` tinyint(1) DEFAULT 0,
  PRIMARY KEY (`id`),
  UNIQUE KEY `out_biz_no` (`out_biz_no`),
  KEY `uid` (`uid`),
  KEY `type` (`type`),
  KEY `status` (`status`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_common_payment_order` WRITE;
/*!40000 ALTER TABLE `pre_common_payment_order` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_common_payment_order` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_common_payment_refund`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_common_payment_refund` (
  `id` int(10) unsigned NOT NULL AUTO_INCREMENT,
  `order_id` int(10) unsigned NOT NULL,
  `out_biz_no` varchar(64) NOT NULL,
  `amount` int(10) unsigned NOT NULL,
  `description` varchar(255) NOT NULL,
  `status` tinyint(1) NOT NULL,
  `error` varchar(255) DEFAULT NULL,
  `refund_time` int(10) DEFAULT NULL,
  `clientip` varchar(255) NOT NULL DEFAULT '',
  `remoteport` smallint(6) unsigned NOT NULL DEFAULT 0,
  `dateline` int(10) NOT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `out_biz_no` (`out_biz_no`),
  KEY `order_id` (`order_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_common_payment_refund` WRITE;
/*!40000 ALTER TABLE `pre_common_payment_refund` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_common_payment_refund` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_common_payment_transfer`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_common_payment_transfer` (
  `id` int(10) unsigned NOT NULL AUTO_INCREMENT,
  `uid` int(10) unsigned NOT NULL,
  `out_biz_no` varchar(64) NOT NULL,
  `amount` int(10) unsigned NOT NULL,
  `subject` varchar(255) NOT NULL,
  `description` varchar(255) DEFAULT NULL,
  `realname` varchar(255) NOT NULL,
  `account` varchar(255) NOT NULL,
  `channel` varchar(255) DEFAULT NULL,
  `status` tinyint(3) unsigned NOT NULL,
  `error` varchar(255) DEFAULT NULL,
  `trade_no` varchar(255) DEFAULT NULL,
  `trade_time` int(10) unsigned DEFAULT NULL,
  `clientip` varchar(255) NOT NULL DEFAULT '',
  `remoteport` smallint(6) unsigned NOT NULL DEFAULT 0,
  `dateline` int(10) unsigned NOT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `out_biz_no` (`out_biz_no`),
  KEY `uid` (`uid`),
  KEY `status` (`status`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_common_payment_transfer` WRITE;
/*!40000 ALTER TABLE `pre_common_payment_transfer` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_common_payment_transfer` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_common_plugin`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_common_plugin` (
  `pluginid` smallint(6) unsigned NOT NULL AUTO_INCREMENT,
  `available` tinyint(1) NOT NULL DEFAULT 0,
  `adminid` tinyint(3) unsigned NOT NULL DEFAULT 0,
  `name` varchar(40) NOT NULL DEFAULT '',
  `identifier` varchar(40) NOT NULL DEFAULT '',
  `description` varchar(255) NOT NULL DEFAULT '',
  `datatables` varchar(255) NOT NULL DEFAULT '',
  `directory` varchar(100) NOT NULL DEFAULT '',
  `copyright` varchar(100) NOT NULL DEFAULT '',
  `modules` text NOT NULL,
  `version` varchar(20) NOT NULL DEFAULT '',
  PRIMARY KEY (`pluginid`),
  UNIQUE KEY `identifier` (`identifier`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_common_plugin` WRITE;
/*!40000 ALTER TABLE `pre_common_plugin` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_common_plugin` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_common_pluginvar`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_common_pluginvar` (
  `pluginvarid` mediumint(8) unsigned NOT NULL AUTO_INCREMENT,
  `pluginid` smallint(6) unsigned NOT NULL DEFAULT 0,
  `displayorder` int(10) NOT NULL DEFAULT 0,
  `title` varchar(100) NOT NULL DEFAULT '',
  `description` varchar(255) NOT NULL DEFAULT '',
  `variable` varchar(40) NOT NULL DEFAULT '',
  `type` varchar(255) NOT NULL DEFAULT 'text',
  `value` text NOT NULL,
  `extra` text NOT NULL,
  PRIMARY KEY (`pluginvarid`),
  KEY `pluginid` (`pluginid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_common_pluginvar` WRITE;
/*!40000 ALTER TABLE `pre_common_pluginvar` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_common_pluginvar` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_common_regip`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_common_regip` (
  `ip` varchar(45) NOT NULL DEFAULT '',
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  `count` smallint(6) NOT NULL DEFAULT 0,
  KEY `ip` (`ip`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_common_regip` WRITE;
/*!40000 ALTER TABLE `pre_common_regip` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_common_regip` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_common_relatedlink`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_common_relatedlink` (
  `id` smallint(6) unsigned NOT NULL AUTO_INCREMENT,
  `name` varchar(100) NOT NULL DEFAULT '',
  `url` varchar(255) NOT NULL DEFAULT '',
  `extent` tinyint(3) NOT NULL DEFAULT 0,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_common_relatedlink` WRITE;
/*!40000 ALTER TABLE `pre_common_relatedlink` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_common_relatedlink` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_common_report`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_common_report` (
  `id` mediumint(8) unsigned NOT NULL AUTO_INCREMENT,
  `urlkey` char(32) NOT NULL DEFAULT '',
  `url` varchar(255) NOT NULL DEFAULT '',
  `message` text NOT NULL,
  `uid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `username` varchar(50) NOT NULL DEFAULT '',
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  `num` smallint(6) unsigned NOT NULL DEFAULT 1,
  `opuid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `opname` varchar(50) NOT NULL DEFAULT '',
  `optime` int(10) unsigned NOT NULL DEFAULT 0,
  `opresult` varchar(255) NOT NULL DEFAULT '',
  `fid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`id`),
  KEY `urlkey` (`urlkey`),
  KEY `fid` (`fid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_common_report` WRITE;
/*!40000 ALTER TABLE `pre_common_report` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_common_report` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_common_searchindex`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_common_searchindex` (
  `searchid` int(10) unsigned NOT NULL AUTO_INCREMENT,
  `srchmod` tinyint(3) unsigned NOT NULL,
  `keywords` varchar(255) NOT NULL DEFAULT '',
  `searchstring` text NOT NULL,
  `useip` varchar(45) NOT NULL DEFAULT '',
  `uid` mediumint(10) unsigned NOT NULL DEFAULT 0,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  `expiration` int(10) unsigned NOT NULL DEFAULT 0,
  `threadsortid` smallint(6) unsigned NOT NULL DEFAULT 0,
  `num` smallint(6) unsigned NOT NULL DEFAULT 0,
  `ids` text NOT NULL,
  PRIMARY KEY (`searchid`),
  KEY `srchmod` (`srchmod`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_common_searchindex` WRITE;
/*!40000 ALTER TABLE `pre_common_searchindex` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_common_searchindex` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_common_seccheck`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_common_seccheck` (
  `ssid` int(10) NOT NULL AUTO_INCREMENT,
  `dateline` int(10) NOT NULL,
  `code` char(6) NOT NULL,
  `succeed` tinyint(1) NOT NULL,
  `verified` tinyint(1) NOT NULL,
  PRIMARY KEY (`ssid`),
  KEY `dateline` (`dateline`),
  KEY `succeed` (`succeed`),
  KEY `verified` (`verified`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_common_seccheck` WRITE;
/*!40000 ALTER TABLE `pre_common_seccheck` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_common_seccheck` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_common_secquestion`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_common_secquestion` (
  `id` smallint(6) unsigned NOT NULL AUTO_INCREMENT,
  `type` tinyint(3) unsigned NOT NULL,
  `question` text NOT NULL,
  `answer` varchar(255) NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_common_secquestion` WRITE;
/*!40000 ALTER TABLE `pre_common_secquestion` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_common_secquestion` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_common_setting`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_common_setting` (
  `skey` varchar(190) NOT NULL DEFAULT '',
  `svalue` text NOT NULL,
  PRIMARY KEY (`skey`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_common_setting` WRITE;
/*!40000 ALTER TABLE `pre_common_setting` DISABLE KEYS */;
INSERT INTO `pre_common_setting` VALUES
('accessemail',''),
('accountguard','a:3:{s:12:\"loginpwcheck\";s:1:\"0\";s:14:\"loginoutofdate\";s:1:\"0\";s:17:\"loginoutofdatenum\";s:0:\"\";}'),
('activitycredit','1'),
('activityextnum','0'),
('activityfield','a:3:{s:8:\"realname\";s:12:\"鐪熷疄濮撳悕\";s:6:\"mobile\";s:6:\"鎵嬫満\";s:2:\"qq\";s:5:\"QQ鍙穃";}'),
('activityforumid','0'),
('activitypp','8'),
('activitytype','鏈嬪弸鑱氫細\r\n鍑哄閮婃父\r\n鑷┚鍑鸿\r\n鍏泭娲诲姩\r\n绾夸笂娲诲姩'),
('adminemail','admin@admin.com'),
('adminipaccess',''),
('adminnotifytypes','verifythread,verifypost,verifyuser,verifyblog,verifydoing,verifypic,verifyshare,verifycommontes,verifyrecycle,verifyrecyclepost,verifyarticle,verifyacommont,verifymedal,verify_1,verify_2,verify_3,verify_4,verify_5,verify_6,verify_7'),
('advexpiration','a:3:{s:5:\"allow\";b:0;s:3:\"day\";s:0:\"\";s:5:\"users\";s:0:\"\";}'),
('advtype','a:0:{}'),
('albumcategoryrequired','0'),
('albumcategorystat','0'),
('albumstatus','0'),
('allowattachurl','0'),
('allowdomain','0'),
('alloweditpost','0'),
('allowfastreply','0'),
('allowgroupdomain','0'),
('allowmoderatingthread','1'),
('allowpostcomment','a:2:{i:0;s:1:\"1\";i:1;s:1:\"2\";}'),
('allowquickviewprofile','1'),
('allowreplybg','0'),
('allowspacedomain','0'),
('allowswitcheditor','1'),
('allowthreadplugin',''),
('allowviewuserthread','a:2:{s:5:\"allow\";s:1:\"1\";s:4:\"fids\";a:1:{i:0;s:0:\"\";}}'),
('allowwidthauto','0'),
('anonymoustext','鍖垮悕'),
('antitheft','a:2:{s:5:\"allow\";i:0;s:3:\"max\";i:200;}'),
('archiver','1'),
('archiverredirect','0'),
('article_tags','a:8:{i:1;s:6:\"鍘熷垱\";i:2;s:6:\"鐑偣\";i:3;s:6:\"缁勫浘\";i:4;s:6:\"鐖嗘枡\";i:5;s:6:\"澶存潯\";i:6;s:6:\"骞荤伅\";i:7;s:6:\"婊氬姩\";i:8;s:6:\"鎺ㄨ崘\";}'),
('at_anyone','0'),
('attachbanperiods',''),
('attachdir','./data/attachment'),
('attachexpire',''),
('attachimgpost','1'),
('attachrefcheck','0'),
('attachsave','3'),
('attachurl','data/attachment'),
('authkey',''),
('authoronleft','1'),
('autoidselect','0'),
('avatarmethod','0'),
('backupdir','83efcd'),
('bannedmessages','1'),
('bbclosed',''),
('bbname','Discuz!'),
('bbrules','0'),
('bbrulesforce','0'),
('bbrulestxt',''),
('bdaystatus','0'),
('binddomains','a:0:{}'),
('blockmaxaggregationitem','20000'),
('blogcategoryrequired','0'),
('blogcategorystat','0'),
('blogrecyclebin','0'),
('blogstatus','0'),
('boardlicensed','0'),
('cacheindexlife','0'),
('cachethreaddir','data/threadcache'),
('cachethreadlife','0'),
('card','a:1:{s:4:\"open\";s:1:\"0\";}'),
('censoremail',''),
('censoruser',''),
('change_email','0'),
('change_secmobile','0'),
('chatpmrefreshtime','8'),
('close_leftinfo','0'),
('close_leftinfo_userctrl','0'),
('closedallowactivation','0'),
('closedreason',''),
('closeforumorderby','0'),
('collectionnum','10'),
('collectionrecommend','a:3:{s:5:\"ctids\";N;s:13:\"autorecommend\";i:0;s:14:\"adminrecommend\";i:0;}'),
('collectionrecommendnum','0'),
('collectionstatus','0'),
('collectionteamworkernum','3'),
('commentfirstpost','1'),
('commentitem','					'),
('commentnumber','5'),
('commentpostself','0'),
('connect','a:19:{s:5:\"allow\";s:1:\"1\";s:4:\"feed\";a:2:{s:5:\"allow\";s:1:\"1\";s:5:\"group\";s:1:\"0\";}s:1:\"t\";a:2:{s:5:\"allow\";s:1:\"1\";s:5:\"group\";s:1:\"0\";}s:10:\"like_allow\";s:1:\"1\";s:7:\"like_qq\";s:0:\"\";s:10:\"turl_allow\";s:1:\"1\";s:7:\"turl_qq\";s:0:\"\";s:8:\"like_url\";s:0:\"\";s:17:\"register_birthday\";s:1:\"0\";s:15:\"register_gender\";s:1:\"0\";s:17:\"register_uinlimit\";s:0:\"\";s:21:\"register_rewardcredit\";s:1:\"1\";s:18:\"register_addcredit\";s:0:\"\";s:16:\"register_groupid\";s:1:\"0\";s:18:\"register_regverify\";s:1:\"1\";s:15:\"register_invite\";s:1:\"0\";s:10:\"newbiespan\";s:0:\"\";s:9:\"turl_code\";s:0:\"\";s:13:\"mblog_app_key\";s:3:\"abc\";}'),
('creditnotice','1'),
('creditsformula','posts+digestposts*5+extcredits1*2+extcredits2+extcredits3'),
('creditsformulaexp','<u>{credits_CREDITS}</u>=<u>{credits_POSTS}</u>+<u>{credits_DIGESTPOSTS}</u>*5+<u>濞佹湜</u>*2+<u>閲戦挶</u>+<u>璐＄尞</u>'),
('creditsnotify',''),
('creditspolicy','a:12:{s:4:\"post\";a:0:{}s:5:\"reply\";a:0:{}s:6:\"digest\";a:1:{i:1;i:10;}s:10:\"postattach\";a:0:{}s:9:\"getattach\";a:0:{}s:6:\"sendpm\";a:0:{}s:6:\"search\";a:0:{}s:15:\"promotion_visit\";a:0:{}s:18:\"promotion_register\";a:0:{}s:13:\"tradefinished\";a:0:{}s:8:\"votepoll\";a:0:{}s:10:\"lowerlimit\";a:0:{}}'),
('creditspolicymobile','0'),
('creditstax','0.2'),
('creditstrans','2,0,0,0,0,0,0'),
('csspathv','data/cache/'),
('customauthorinfo','a:1:{i:0;a:10:{s:5:\"posts\";a:2:{s:5:\"order\";s:0:\"\";s:4:\"menu\";s:1:\"1\";}s:7:\"threads\";a:1:{s:5:\"order\";s:0:\"\";}s:6:\"digest\";a:2:{s:5:\"order\";s:0:\"\";s:4:\"menu\";s:1:\"1\";}s:7:\"credits\";a:2:{s:5:\"order\";s:0:\"\";s:4:\"menu\";s:1:\"1\";}s:8:\"readperm\";a:2:{s:5:\"order\";s:0:\"\";s:4:\"menu\";s:1:\"1\";}s:7:\"regtime\";a:2:{s:5:\"order\";s:0:\"\";s:4:\"menu\";s:1:\"1\";}s:8:\"lastdate\";a:2:{s:5:\"order\";s:0:\"\";s:4:\"menu\";s:1:\"1\";}s:11:\"extcredits1\";a:1:{s:5:\"order\";s:0:\"\";}s:11:\"extcredits2\";a:1:{s:5:\"order\";s:0:\"\";}s:11:\"extcredits3\";a:1:{s:5:\"order\";s:0:\"\";}}}'),
('custombackup',''),
('darkroom','1'),
('dateconvert','1'),
('dateformat','Y-n-j'),
('debateforumid','0'),
('debug','1'),
('defaulteditormode','1'),
('defaultindex','forum.php'),
('delayviewcount','0'),
('deletereason',''),
('disableipnotice','0'),
('disallowfloat','a:1:{i:3;s:9:\"newthread\";}'),
('disfixedavatar','0'),
('disfixednv_forumdisplay','0'),
('disfixednv_forumindex','0'),
('disfixednv_viewthread','0'),
('doingstatus','0'),
('domain','a:5:{s:12:\"defaultindex\";s:9:\"forum.php\";s:10:\"holddomain\";s:18:\"www|*blog*|*space*\";s:4:\"list\";a:0:{}s:3:\"app\";a:5:{s:6:\"portal\";s:0:\"\";s:5:\"forum\";s:0:\"\";s:5:\"group\";s:0:\"\";s:4:\"home\";s:0:\"\";s:7:\"default\";s:0:\"\";}s:4:\"root\";a:5:{s:4:\"home\";s:0:\"\";s:5:\"group\";s:0:\"\";s:5:\"forum\";s:0:\"\";s:5:\"topic\";s:0:\"\";s:7:\"channel\";s:0:\"\";}}'),
('domainroot',''),
('domainwhitelist',''),
('doublee','1'),
('dupkarmarate','0'),
('dynavt','1'),
('ec_account',''),
('ec_contract',''),
('ec_credit','a:2:{s:18:\"maxcreditspermonth\";i:6;s:4:\"rank\";a:15:{i:1;i:4;i:2;i:11;i:3;i:41;i:4;i:91;i:5;i:151;i:6;i:251;i:7;i:501;i:8;i:1001;i:9;i:2001;i:10;i:5001;i:11;i:10001;i:12;i:20001;i:13;i:50001;i:14;i:100001;i:15;i:200001;}}'),
('ec_maxcredits','1000'),
('ec_maxcreditspermonth','0'),
('ec_mincredits','0'),
('ec_ratio','0'),
('editedby','1'),
('editorfids','a:1:{i:0;s:0:\"\";}'),
('editorgroupid','a:1:{i:0;s:0:\"\";}'),
('editormodetype','0'),
('editoroptions','6'),
('editperdel','0'),
('edittimelimit',''),
('exchangemincredits','100'),
('extcredits','a:8:{i:1;a:8:{s:3:\"img\";s:0:\"\";s:5:\"title\";s:6:\"濞佹湜\";s:4:\"unit\";s:0:\"\";s:5:\"ratio\";i:0;s:9:\"available\";s:1:\"1\";s:12:\"showinthread\";N;s:15:\"allowexchangein\";N;s:16:\"allowexchangeout\";N;}i:2;a:8:{s:3:\"img\";s:0:\"\";s:5:\"title\";s:6:\"閲戦挶\";s:4:\"unit\";s:0:\"\";s:5:\"ratio\";i:0;s:9:\"available\";s:1:\"1\";s:12:\"showinthread\";N;s:15:\"allowexchangein\";N;s:16:\"allowexchangeout\";N;}i:3;a:8:{s:3:\"img\";s:0:\"\";s:5:\"title\";s:6:\"璐＄尞\";s:4:\"unit\";s:0:\"\";s:5:\"ratio\";i:0;s:9:\"available\";s:1:\"1\";s:12:\"showinthread\";N;s:15:\"allowexchangein\";N;s:16:\"allowexchangeout\";N;}i:4;a:8:{s:3:\"img\";s:0:\"\";s:5:\"title\";s:0:\"\";s:4:\"unit\";s:0:\"\";s:5:\"ratio\";i:0;s:9:\"available\";N;s:12:\"showinthread\";N;s:15:\"allowexchangein\";N;s:16:\"allowexchangeout\";N;}i:5;a:8:{s:3:\"img\";s:0:\"\";s:5:\"title\";s:0:\"\";s:4:\"unit\";s:0:\"\";s:5:\"ratio\";i:0;s:9:\"available\";N;s:12:\"showinthread\";N;s:15:\"allowexchangein\";N;s:16:\"allowexchangeout\";N;}i:6;a:8:{s:3:\"img\";s:0:\"\";s:5:\"title\";s:0:\"\";s:4:\"unit\";s:0:\"\";s:5:\"ratio\";i:0;s:9:\"available\";N;s:12:\"showinthread\";N;s:15:\"allowexchangein\";N;s:16:\"allowexchangeout\";N;}i:7;a:8:{s:3:\"img\";s:0:\"\";s:5:\"title\";s:0:\"\";s:4:\"unit\";s:0:\"\";s:5:\"ratio\";i:0;s:9:\"available\";N;s:12:\"showinthread\";N;s:15:\"allowexchangein\";N;s:16:\"allowexchangeout\";N;}i:8;a:8:{s:3:\"img\";s:0:\"\";s:5:\"title\";s:0:\"\";s:4:\"unit\";s:0:\"\";s:5:\"ratio\";i:0;s:9:\"available\";N;s:12:\"showinthread\";N;s:15:\"allowexchangein\";N;s:16:\"allowexchangeout\";N;}}'),
('fastpost','1'),
('fastsmiley','a:1:{i:0;a:16:{i:0;s:1:\"1\";i:1;s:1:\"2\";i:2;s:1:\"3\";i:3;s:1:\"4\";i:8;s:1:\"5\";i:9;s:1:\"6\";i:10;s:1:\"7\";i:11;s:1:\"8\";i:12;s:1:\"9\";i:13;s:2:\"10\";i:14;s:2:\"11\";i:15;s:2:\"12\";i:16;s:2:\"13\";i:17;s:2:\"14\";i:18;s:2:\"15\";i:19;s:2:\"16\";}}'),
('fastsmilies','1'),
('favoritestatus','0'),
('feedday','7'),
('feedhotday','2'),
('feedhotmin','3'),
('feedhotnum','3'),
('feedmaxnum','100'),
('feedstatus','0'),
('feedtargetblank','1'),
('filterednovote','1'),
('floodctrl','15'),
('focus','a:3:{s:5:\"title\";s:12:\"绔欓暱鎺ㄨ崘\";s:4:\"data\";a:0:{}s:6:\"cookie\";s:1:\"1\";}'),
('followaddnotice','0'),
('followretainday','7'),
('followstatus','0'),
('forumallowside','0'),
('forumdisplaythreadpreview','1'),
('forumdomains','a:0:{}'),
('forumjump','0'),
('forumlinkstatus','1'),
('forumpicstyle','a:3:{s:10:\"thumbwidth\";i:0;s:11:\"thumbheight\";i:0;s:8:\"thumbnum\";i:0;}'),
('forumseparator','1'),
('forumstatus','1'),
('forumstickthreads','a:0:{}'),
('frameon','0'),
('framewidth','180'),
('friendgroupnum','8'),
('friendstatus','0'),
('ftp','a:10:{s:2:\"on\";s:1:\"0\";s:3:\"ssl\";s:1:\"0\";s:4:\"host\";s:0:\"\";s:4:\"port\";s:2:\"21\";s:8:\"username\";s:0:\"\";s:8:\"password\";s:0:\"\";s:9:\"attachdir\";s:1:\".\";s:9:\"attachurl\";s:0:\"\";s:7:\"hideurl\";s:1:\"0\";s:7:\"timeout\";s:1:\"0\";}'),
('globalstick','1'),
('grid','a:8:{s:8:\"showgrid\";s:1:\"0\";s:8:\"gridtype\";s:1:\"0\";s:8:\"textleng\";s:2:\"30\";s:4:\"fids\";a:1:{i:0;i:0;}s:9:\"highlight\";s:1:\"1\";s:11:\"targetblank\";s:1:\"1\";s:8:\"showtips\";s:1:\"1\";s:9:\"cachelife\";s:3:\"600\";}'),
('group_admingroupids','a:1:{i:1;s:1:\"1\";}'),
('group_allowfeed','1'),
('group_description',''),
('group_imgsizelimit','512'),
('group_keywords',''),
('group_recommend','a:0:{}'),
('group_userperm','a:22:{s:16:\"allowstickthread\";s:1:\"1\";s:15:\"allowbumpthread\";s:1:\"1\";s:20:\"allowhighlightthread\";s:1:\"1\";s:16:\"allowstampthread\";s:1:\"1\";s:16:\"allowclosethread\";s:1:\"1\";s:16:\"allowmergethread\";s:1:\"1\";s:16:\"allowsplitthread\";s:1:\"1\";s:17:\"allowrepairthread\";s:1:\"1\";s:11:\"allowrefund\";s:1:\"1\";s:13:\"alloweditpoll\";s:1:\"1\";s:17:\"allowremovereward\";s:1:\"1\";s:17:\"alloweditactivity\";s:1:\"1\";s:14:\"allowedittrade\";s:1:\"1\";s:17:\"allowdigestthread\";s:1:\"3\";s:13:\"alloweditpost\";s:1:\"1\";s:13:\"allowwarnpost\";s:1:\"1\";s:12:\"allowbanpost\";s:1:\"1\";s:12:\"allowdelpost\";s:1:\"1\";s:13:\"allowupbanner\";s:1:\"1\";s:15:\"disablepostctrl\";s:1:\"0\";s:11:\"allowviewip\";s:1:\"1\";s:15:\"allowlivethread\";s:1:\"1\";}'),
('groupmod','0'),
('groupstatus','0'),
('guesttipsinthread','a:2:{s:4:\"flag\";i:0;s:4:\"text\";s:0:\"\";}'),
('guestviewthumb','a:3:{s:4:\"flag\";i:0;s:5:\"width\";i:100;s:6:\"height\";i:100;}'),
('guide','a:2:{s:5:\"hotdt\";i:604800;s:8:\"digestdt\";i:604800;}'),
('guidestatus','0'),
('heatthread','a:5:{s:4:\"type\";s:1:\"2\";s:5:\"reply\";i:5;s:9:\"recommend\";i:3;s:6:\"period\";s:2:\"15\";s:10:\"iconlevels\";s:10:\"50,100,200\";}'),
('hideattachdown','0'),
('hideattachtips','0'),
('hidefilteredpost','0'),
('hideprivate','1'),
('historyposts','0	7'),
('holddomain','www|*blog*|*space*|*bbs*'),
('homepagestyle','0'),
('homestatus','0'),
('homestyle','0'),
('hottopic','10'),
('icp',''),
('imageimpath',''),
('imagelib','0'),
('imagelistthumb','0'),
('imagemaxwidth','600'),
('indexhot','a:7:{s:6:\"status\";s:1:\"0\";s:5:\"limit\";s:2:\"10\";s:4:\"days\";s:1:\"7\";s:10:\"expiration\";s:3:\"900\";s:10:\"messagecut\";s:3:\"200\";s:5:\"width\";i:100;s:6:\"height\";i:70;}'),
('indextype','classics'),
('infosidestatus','0'),
('initcredits','0,0,0,0,0,0,0,0,0'),
('inviteconfig',''),
('ipaccess',''),
('ipregctrl',''),
('ipregctrltime','72'),
('ipverifywhite',''),
('jscachelife','1800'),
('jsdateformat',''),
('jspath','data/cache/'),
('jsrefdomains',''),
('jsstatus','0'),
('jswizard',''),
('karmaratelimit','0'),
('lazyload','0'),
('leftsideopen','0'),
('leftsidewidth','0'),
('log','a:14:{s:13:\"clearlogstime\";s:1:\"0\";s:14:\"clearlogstypes\";a:2:{i:0;s:2:\"cp\";i:1;s:5:\"error\";}s:7:\"illegal\";s:1:\"1\";s:3:\"ban\";s:1:\"1\";s:4:\"mods\";s:1:\"1\";s:3:\"sms\";s:1:\"1\";s:5:\"login\";s:1:\"1\";s:2:\"cp\";s:1:\"1\";s:5:\"modcp\";s:1:\"1\";s:5:\"error\";s:1:\"1\";s:8:\"sendmail\";s:1:\"1\";s:4:\"SMTP\";s:1:\"1\";s:4:\"rate\";s:1:\"1\";s:3:\"pmt\";s:1:\"1\";}'),
('losslessdel','365'),
('magicdiscount','85'),
('magicmarket','1'),
('magicstatus','0'),
('mail','a:17:{s:8:\"mailsend\";s:1:\"1\";s:6:\"server\";s:13:\"smtp.21cn.com\";s:4:\"port\";s:2:\"25\";s:4:\"auth\";s:1:\"1\";s:4:\"from\";s:26:\"Discuz <username@21cn.com>\";s:13:\"auth_username\";s:17:\"username@21cn.com\";s:13:\"auth_password\";s:8:\"password\";s:13:\"maildelimiter\";s:1:\"0\";s:12:\"mailusername\";s:1:\"1\";s:15:\"sendmail_silent\";s:1:\"1\";s:15:\"emailcodestatus\";s:1:\"0\";s:22:\"emailcodedefaultlength\";s:1:\"6\";s:16:\"emailverifylimit\";s:1:\"5\";s:14:\"emailtimelimit\";s:5:\"86400\";s:13:\"emailnumlimit\";s:1:\"5\";s:13:\"emailinterval\";s:3:\"300\";s:13:\"emailglblimit\";s:4:\"1000\";}'),
('maxavatarpixel','120'),
('maxavatarsize','20000'),
('maxbdays','0'),
('maxchargespan','0'),
('maxfavorites','100'),
('maxincperthread','0'),
('maxmagicprice','50'),
('maxmodworksmonths','3'),
('maxonlinelist','0'),
('maxonlines','5000'),
('maxpage','100'),
('maxpolloptions','20'),
('maxpostsize','10000'),
('maxsigrows','100'),
('maxsmilies','10'),
('maxsubjectsize','80'),
('medalstatus','0'),
('membermaxpages','100'),
('memberperpage','25'),
('memliststatus','1'),
('memory','a:16:{s:13:\"common_member\";i:0;s:19:\"common_member_count\";i:0;s:20:\"common_member_status\";i:0;s:21:\"common_member_profile\";i:0;s:24:\"common_member_field_home\";i:0;s:25:\"common_member_field_forum\";i:0;s:20:\"common_member_verify\";i:0;s:12:\"forum_thread\";i:172800;s:25:\"forum_thread_forumdisplay\";i:300;s:23:\"forum_collectionrelated\";i:0;s:15:\"forum_postcache\";i:300;s:16:\"forum_collection\";i:300;s:11:\"home_follow\";i:86400;s:10:\"forumindex\";i:30;s:8:\"diyblock\";i:300;s:14:\"diyblockoutput\";i:30;}'),
('menunavs',''),
('minpostsize','10'),
('minpostsize_mobile','0'),
('minsubjectsize','1'),
('mobile','a:13:{s:11:\"allowmobile\";i:1;s:9:\"allowmnew\";i:0;s:13:\"mobileforward\";i:1;s:14:\"mobileregister\";i:1;s:13:\"mobileseccode\";i:0;s:16:\"mobilesimpletype\";i:0;s:15:\"mobilecachetime\";i:0;s:14:\"mobilecomefrom\";s:0:\"\";s:13:\"mobilepreview\";i:0;s:6:\"legacy\";i:1;s:3:\"wml\";i:0;s:6:\"portal\";a:1:{s:6:\"catnav\";i:0;}s:5:\"forum\";a:6:{s:5:\"index\";i:0;s:8:\"statshow\";i:0;s:13:\"displayorder3\";i:1;s:12:\"topicperpage\";i:20;s:11:\"postperpage\";i:10;s:9:\"forumview\";i:0;}}'),
('modasban','1'),
('moddetail','0'),
('moddisplay','flat'),
('modratelimit','0'),
('modreasons','骞垮憡/SPAM\r\n鎭舵剰鐏屾按\r\n杩濊鍐呭\r\n鏂囦笉瀵归\r\n閲嶅鍙戝笘\r\n\r\n鎴戝緢璧炲悓\r\n绮惧搧鏂囩珷\r\n鍘熷垱鍐呭'),
('modreasons_public','0'),
('moduser_public','0'),
('modworkstatus','1'),
('mps',''),
('msgforward','a:3:{s:11:\"refreshtime\";i:2;s:5:\"quick\";i:1;s:8:\"messages\";a:14:{i:0;s:19:\"thread_poll_succeed\";i:1;s:19:\"thread_rate_succeed\";i:2;s:23:\"usergroups_join_succeed\";i:3;s:23:\"usergroups_exit_succeed\";i:4;s:25:\"usergroups_update_succeed\";i:5;s:20:\"buddy_update_succeed\";i:6;s:17:\"post_edit_succeed\";i:7;s:18:\"post_reply_succeed\";i:8;s:24:\"post_edit_delete_succeed\";i:9;s:22:\"post_newthread_succeed\";i:10;s:13:\"admin_succeed\";i:11;s:17:\"pm_delete_succeed\";i:12;s:15:\"search_redirect\";i:13;s:10:\"do_success\";}}'),
('msn',''),
('my_closecheckupdate',''),
('my_ip',''),
('my_search_data',''),
('my_siteid',''),
('my_sitekey',''),
('navdms','a:0:{}'),
('navmn','a:1:{s:9:\"forum.php\";s:8:\"mn_forum\";}'),
('navmns','a:2:{s:8:\"misc.php\";a:1:{i:0;a:2:{i:0;a:1:{s:3:\"mod\";s:3:\"faq\";}i:1;s:8:\"mn_N0a2c\";}}s:9:\"forum.php\";a:1:{i:0;a:2:{i:0;a:2:{s:3:\"mod\";s:12:\"forumdisplay\";s:3:\"fid\";s:1:\"0\";}i:1;s:11:\"mn_forum_16\";}}}'),
('navs','a:4:{i:2;a:7:{s:7:\"navname\";s:6:\"璁哄潧\";s:8:\"filename\";s:9:\"forum.php\";s:9:\"available\";s:1:\"1\";s:4:\"data\";a:19:{s:2:\"id\";s:1:\"2\";s:8:\"parentid\";s:1:\"0\";s:4:\"name\";s:6:\"璁哄潧\";s:5:\"title\";s:3:\"BBS\";s:3:\"url\";s:9:\"forum.php\";s:10:\"identifier\";s:1:\"2\";s:6:\"target\";s:1:\"0\";s:4:\"type\";s:1:\"0\";s:9:\"available\";s:1:\"1\";s:12:\"displayorder\";s:1:\"2\";s:9:\"highlight\";s:1:\"0\";s:5:\"level\";s:1:\"0\";s:7:\"subtype\";s:1:\"0\";s:7:\"subcols\";s:1:\"0\";s:4:\"icon\";s:0:\"\";s:7:\"subname\";s:0:\"\";s:6:\"suburl\";s:0:\"\";s:7:\"navtype\";s:1:\"0\";s:4:\"logo\";s:0:\"\";}s:5:\"navid\";s:8:\"mn_forum\";s:5:\"level\";s:1:\"0\";s:3:\"nav\";s:91:\"id=\"mn_forum\" ><a href=\"forum.php\" hidefocus=\"true\" title=\"BBS\"  >璁哄潧<span>BBS</span></a\";}i:6;a:3:{s:7:\"navname\";s:6:\"鎻掍欢\";s:8:\"filename\";s:1:\"#\";s:9:\"available\";i:0;}i:7;a:7:{s:7:\"navname\";s:6:\"甯姪\";s:8:\"filename\";s:16:\"misc.php?mod=faq\";s:9:\"available\";s:1:\"0\";s:4:\"data\";a:19:{s:2:\"id\";s:1:\"6\";s:8:\"parentid\";s:1:\"0\";s:4:\"name\";s:6:\"甯姪\";s:5:\"title\";s:4:\"Help\";s:3:\"url\";s:16:\"misc.php?mod=faq\";s:10:\"identifier\";s:1:\"7\";s:6:\"target\";s:1:\"0\";s:4:\"type\";s:1:\"0\";s:9:\"available\";s:1:\"0\";s:12:\"displayorder\";s:2:\"10\";s:9:\"highlight\";s:1:\"0\";s:5:\"level\";s:1:\"0\";s:7:\"subtype\";s:1:\"0\";s:7:\"subcols\";s:1:\"0\";s:4:\"icon\";s:0:\"\";s:7:\"subname\";s:0:\"\";s:6:\"suburl\";s:0:\"\";s:7:\"navtype\";s:1:\"0\";s:4:\"logo\";s:0:\"\";}s:5:\"navid\";s:8:\"mn_N0a2c\";s:5:\"level\";s:1:\"0\";s:3:\"nav\";s:100:\"id=\"mn_N0a2c\" ><a href=\"misc.php?mod=faq\" hidefocus=\"true\" title=\"Help\"  >甯姪<span>Help</span></a\";}i:16;a:7:{s:7:\"navname\";s:6:\"棣栭〉\";s:8:\"filename\";s:32:\"forum.php?mod=forumdisplay&fid=0\";s:9:\"available\";s:1:\"0\";s:4:\"data\";a:19:{s:2:\"id\";s:2:\"15\";s:8:\"parentid\";s:1:\"0\";s:4:\"name\";s:6:\"棣栭〉\";s:5:\"title\";s:5:\"Index\";s:3:\"url\";s:32:\"forum.php?mod=forumdisplay&fid=0\";s:10:\"identifier\";s:2:\"16\";s:6:\"target\";s:1:\"0\";s:4:\"type\";s:1:\"0\";s:9:\"available\";s:1:\"0\";s:12:\"displayorder\";s:2:\"16\";s:9:\"highlight\";s:1:\"0\";s:5:\"level\";s:1:\"0\";s:7:\"subtype\";s:1:\"0\";s:7:\"subcols\";s:1:\"0\";s:4:\"icon\";s:0:\"\";s:7:\"subname\";s:0:\"\";s:6:\"suburl\";s:0:\"\";s:7:\"navtype\";s:1:\"0\";s:4:\"logo\";s:0:\"\";}s:5:\"navid\";s:11:\"mn_forum_16\";s:5:\"level\";s:1:\"0\";s:3:\"nav\";s:121:\"id=\"mn_forum_16\" ><a href=\"forum.php?mod=forumdisplay&fid=0\" hidefocus=\"true\" title=\"Index\"  >棣栭〉<span>Index</span></a\";}}'),
('navsubhover','0'),
('need_avatar','0'),
('need_email','0'),
('need_secmobile','0'),
('networkpage','0'),
('newbie','20'),
('newbiespan','2'),
('newbietasks',''),
('newbietaskupdate',''),
('newsletter',''),
('newspaceavatar','0'),
('nocacheheaders','0'),
('nofilteredpost','0'),
('notifyusers','a:1:{i:1;a:2:{s:8:\"username\";s:5:\"admin\";s:5:\"types\";s:20:\"11111111111111111111\";}}'),
('nsprofiles','1'),
('numbercard','a:1:{s:3:\"row\";a:3:{i:1;s:7:\"threads\";i:2;s:5:\"posts\";i:3;s:7:\"credits\";}}'),
('oltimespan','10'),
('onlineguestsmultiple','10'),
('onlinehold','15'),
('onlinerecord','7	1269749404'),
('onlyacceptfriendpm','0'),
('optimizeviews','0'),
('outlandverify','0'),
('pmreportuser','1'),
('pollforumid','0'),
('portalarticleimgthumbclosed','0'),
('portalstatus','0'),
('postappend','0'),
('postbanperiods',''),
('postignorearea',''),
('postignoreip',''),
('postmodperiods',''),
('postno','#'),
('postnocustom','a:4:{i:0;s:6:\"妤间富\";i:1;s:6:\"娌欏彂\";i:2;s:6:\"鏉垮嚦\";i:3;s:6:\"鍦版澘\";}'),
('postperpage','10'),
('preventrefresh','1'),
('privacy','a:2:{s:4:\"view\";a:8:{s:5:\"index\";i:0;s:6:\"friend\";i:0;s:4:\"wall\";i:0;s:4:\"home\";i:0;s:5:\"doing\";i:0;s:4:\"blog\";i:0;s:5:\"album\";i:0;s:5:\"share\";i:0;}s:4:\"feed\";a:5:{s:5:\"doing\";i:1;s:4:\"blog\";i:1;s:6:\"upload\";i:1;s:4:\"poll\";i:1;s:9:\"newthread\";i:1;}}'),
('profilegroup','a:5:{s:4:\"base\";a:4:{s:9:\"available\";i:1;s:12:\"displayorder\";i:0;s:5:\"title\";s:12:\"鍩烘湰璧勬枡\";s:5:\"field\";a:17:{s:6:\"gender\";s:6:\"gender\";s:8:\"birthday\";s:8:\"birthday\";s:8:\"realname\";s:8:\"realname\";s:9:\"birthcity\";s:9:\"birthcity\";s:9:\"bloodtype\";s:9:\"bloodtype\";s:10:\"lookingfor\";s:10:\"lookingfor\";s:10:\"residecity\";s:10:\"residecity\";s:10:\"residedist\";s:10:\"residedist\";s:15:\"affectivestatus\";s:15:\"affectivestatus\";s:6:\"field1\";s:6:\"field1\";s:6:\"field2\";s:6:\"field2\";s:6:\"field3\";s:6:\"field3\";s:6:\"field4\";s:6:\"field4\";s:6:\"field5\";s:6:\"field5\";s:6:\"field6\";s:6:\"field6\";s:6:\"field7\";s:6:\"field7\";s:6:\"field8\";s:6:\"field8\";}}s:7:\"contact\";a:4:{s:5:\"title\";s:12:\"鑱旂郴鏂瑰紡\";s:9:\"available\";s:1:\"1\";s:12:\"displayorder\";s:1:\"1\";s:5:\"field\";a:7:{s:2:\"qq\";s:2:\"qq\";s:3:\"msn\";s:3:\"msn\";s:6:\"mobile\";s:6:\"mobile\";s:6:\"taobao\";s:6:\"taobao\";s:9:\"telephone\";s:9:\"telephone\";s:3:\"icq\";s:3:\"icq\";s:5:\"yahoo\";s:5:\"yahoo\";}}s:3:\"edu\";a:4:{s:9:\"available\";i:1;s:12:\"displayorder\";i:2;s:5:\"title\";s:12:\"鏁欒偛鎯呭喌\";s:5:\"field\";a:2:{s:9:\"education\";s:9:\"education\";s:14:\"graduateschool\";s:14:\"graduateschool\";}}s:4:\"work\";a:4:{s:9:\"available\";i:1;s:12:\"displayorder\";i:3;s:5:\"title\";s:12:\"宸ヤ綔鎯呭喌\";s:5:\"field\";a:4:{s:7:\"company\";s:7:\"company\";s:7:\"revenue\";s:7:\"revenue\";s:8:\"position\";s:8:\"position\";s:10:\"occupation\";s:10:\"occupation\";}}s:4:\"info\";a:4:{s:5:\"title\";s:12:\"涓汉淇℃伅\";s:9:\"available\";s:1:\"1\";s:12:\"displayorder\";s:1:\"4\";s:5:\"field\";a:10:{s:3:\"bio\";s:3:\"bio\";s:4:\"site\";s:4:\"site\";s:6:\"idcard\";s:6:\"idcard\";s:7:\"address\";s:7:\"address\";s:7:\"zipcode\";s:7:\"zipcode\";s:8:\"interest\";s:8:\"interest\";s:10:\"idcardtype\";s:10:\"idcardtype\";s:7:\"sightml\";s:7:\"sightml\";s:12:\"customstatus\";s:12:\"customstatus\";s:10:\"timeoffset\";s:10:\"timeoffset\";}}}'),
('profilegroupnew',''),
('profilehistory','0'),
('pvfrequence','60'),
('pwlength','6'),
('qihoo','a:9:{s:6:\"status\";i:0;s:9:\"searchbox\";i:6;s:7:\"summary\";i:1;s:6:\"jammer\";i:1;s:9:\"maxtopics\";i:10;s:8:\"keywords\";s:0:\"\";s:10:\"adminemail\";s:0:\"\";s:8:\"validity\";i:1;s:14:\"relatedthreads\";a:6:{s:6:\"bbsnum\";i:0;s:6:\"webnum\";i:0;s:4:\"type\";a:3:{s:4:\"blog\";s:4:\"blog\";s:4:\"news\";s:4:\"news\";s:3:\"bbs\";s:3:\"bbs\";}s:6:\"banurl\";s:0:\"\";s:8:\"position\";i:1;s:8:\"validity\";i:1;}}'),
('ranklist','a:12:{s:6:\"status\";s:1:\"1\";s:10:\"membershow\";s:1:\"1\";s:10:\"cache_time\";s:1:\"1\";s:12:\"index_select\";s:8:\"thisweek\";s:6:\"member\";a:3:{s:9:\"available\";s:1:\"1\";s:10:\"cache_time\";s:1:\"5\";s:8:\"show_num\";s:2:\"20\";}s:6:\"thread\";a:3:{s:9:\"available\";s:1:\"1\";s:10:\"cache_time\";s:1:\"5\";s:8:\"show_num\";s:2:\"20\";}s:4:\"blog\";a:3:{s:9:\"available\";s:1:\"1\";s:10:\"cache_time\";s:1:\"5\";s:8:\"show_num\";s:2:\"20\";}s:4:\"poll\";a:3:{s:9:\"available\";s:1:\"1\";s:10:\"cache_time\";s:1:\"5\";s:8:\"show_num\";s:2:\"20\";}s:8:\"activity\";a:3:{s:9:\"available\";s:1:\"1\";s:10:\"cache_time\";s:1:\"5\";s:8:\"show_num\";s:2:\"20\";}s:7:\"picture\";a:3:{s:9:\"available\";s:1:\"1\";s:10:\"cache_time\";s:1:\"5\";s:8:\"show_num\";s:2:\"20\";}s:5:\"forum\";a:3:{s:9:\"available\";s:1:\"1\";s:10:\"cache_time\";s:1:\"5\";s:8:\"show_num\";s:2:\"20\";}s:5:\"group\";a:3:{s:9:\"available\";s:1:\"1\";s:10:\"cache_time\";s:1:\"5\";s:8:\"show_num\";s:2:\"20\";}}'),
('rankliststatus','0'),
('ratelogon','1'),
('ratelogrecord','20'),
('realname','0'),
('recommendthread','a:7:{s:6:\"status\";s:1:\"0\";s:7:\"addtext\";s:6:\"鏀寔\";s:12:\"subtracttext\";s:6:\"鍙嶅\";s:11:\"defaultshow\";s:1:\"1\";s:8:\"daycount\";s:1:\"0\";s:9:\"ownthread\";s:1:\"0\";s:10:\"iconlevels\";s:10:\"50,100,200\";}'),
('regclosemessage',''),
('regconnect','1'),
('regctrl','0'),
('regemail','1'),
('regfloodctrl','0'),
('reginput','a:4:{s:8:\"username\";s:8:\"username\";s:8:\"password\";s:8:\"password\";s:9:\"password2\";s:9:\"password2\";s:5:\"email\";s:5:\"email\";}'),
('reglinkname','绔嬪嵆娉ㄥ唽'),
('regname','register'),
('regstatus','1'),
('regverify','0'),
('relatedlinkstatus','0'),
('relatedtag',''),
('relatenum','10'),
('relatetime','60'),
('repliesrank','0'),
('report_receive','a:2:{s:9:\"adminuser\";a:1:{i:0;s:1:\"1\";}s:12:\"supmoderator\";N;}'),
('report_reward','a:2:{s:3:\"min\";i:-3;s:3:\"max\";i:3;}'),
('rewardexpiration','30'),
('rewardforumid','0'),
('rewritecompatible',''),
('rewriteguest','0'),
('rewritemobile','0'),
('rewriterule',''),
('rewritestatus','0'),
('robotarchiver','0'),
('rssstatus','1'),
('rssttl','60'),
('runwizard','1'),
('search','a:6:{s:6:\"portal\";a:4:{s:6:\"status\";i:1;s:10:\"searchctrl\";i:10;s:6:\"maxspm\";i:10;s:16:\"maxsearchresults\";i:500;}s:5:\"forum\";a:4:{s:6:\"status\";i:1;s:10:\"searchctrl\";i:10;s:6:\"maxspm\";i:10;s:16:\"maxsearchresults\";i:500;}s:4:\"blog\";a:4:{s:6:\"status\";i:1;s:10:\"searchctrl\";i:10;s:6:\"maxspm\";i:10;s:16:\"maxsearchresults\";i:500;}s:5:\"album\";a:4:{s:6:\"status\";i:1;s:10:\"searchctrl\";i:10;s:6:\"maxspm\";i:10;s:16:\"maxsearchresults\";i:500;}s:5:\"group\";a:4:{s:6:\"status\";i:1;s:10:\"searchctrl\";i:10;s:6:\"maxspm\";i:10;s:16:\"maxsearchresults\";i:500;}s:10:\"collection\";a:4:{s:6:\"status\";i:1;s:10:\"searchctrl\";i:10;s:6:\"maxspm\";i:10;s:16:\"maxsearchresults\";i:500;}}'),
('searchbanperiods',''),
('seccodedata','a:14:{s:4:\"type\";s:1:\"0\";s:5:\"width\";s:3:\"100\";s:6:\"height\";s:2:\"30\";s:13:\"shuffer_order\";s:1:\"0\";s:7:\"scatter\";s:1:\"0\";s:10:\"background\";s:1:\"0\";s:10:\"adulterate\";s:1:\"1\";s:3:\"ttf\";s:1:\"0\";s:5:\"angle\";s:1:\"0\";s:7:\"warping\";s:1:\"0\";s:5:\"color\";s:1:\"1\";s:4:\"size\";s:1:\"1\";s:6:\"shadow\";s:1:\"0\";s:8:\"animator\";s:1:\"0\";}'),
('seccodestatus','16'),
('seclevel','1'),
('secmobilelogin','0'),
('secqaa','a:5:{s:8:\"statuses\";a:2:{i:0;s:8:\"register\";i:1;s:5:\"login\";}s:8:\"minposts\";i:0;s:4:\"perm\";s:0:\"\";s:9:\"allowcode\";i:1;s:7:\"allowqa\";i:0;}'),
('security_email','1'),
('security_logoff','0'),
('security_mobile','0'),
('security_password','1'),
('security_question','1'),
('security_rename','1'),
('security_verify','a:0:{}'),
('sendmailday','0'),
('sendregisterurl','0'),
('seodescription',''),
('seohead',''),
('seohead_mobile',''),
('seokeywords',''),
('seotitle','a:4:{s:6:\"portal\";s:6:\"闂ㄦ埛\";s:5:\"forum\";s:6:\"璁哄潧\";s:5:\"group\";s:6:\"鍦堝瓙\";s:4:\"home\";s:6:\"瀹跺洯\";}'),
('sharestatus','0'),
('showallfriendnum','8'),
('showavatars','1'),
('showemail',''),
('showexif','0'),
('showfjump','1'),
('showfollowcollection','8'),
('showimages','1'),
('shownewuser','0'),
('showsettings','7'),
('showsignatures','1'),
('showsignin','1'),
('showusercard','1'),
('sigimgclick','0'),
('sigviewcond','0'),
('simplemode','0'),
('site_qq',''),
('sitemessage','a:5:{s:4:\"time\";s:1:\"3\";s:8:\"register\";s:0:\"\";s:5:\"login\";s:0:\"\";s:9:\"newthread\";s:0:\"\";s:5:\"reply\";s:0:\"\";}'),
('sitename','Discuz! X'),
('siteuniqueid','DX0HTR9P773bH0b0'),
('siteurl','https://www.discuz.vip/'),
('sitevipkey','1'),
('smcols','8'),
('smrows','5'),
('smsdefaultcc','86'),
('smsdefaultlength','4'),
('smsglblimit','1000'),
('smsinterval','300'),
('smsmillimit','20'),
('smsnumlimit','5'),
('smsstatus','0'),
('smstimelimit','86400'),
('smthumb','20'),
('sourceheight',''),
('sourcewidth',''),
('spacedata','a:11:{s:9:\"cachelife\";s:3:\"900\";s:14:\"limitmythreads\";s:1:\"5\";s:14:\"limitmyreplies\";s:1:\"5\";s:14:\"limitmyrewards\";s:1:\"5\";s:13:\"limitmytrades\";s:1:\"5\";s:13:\"limitmyvideos\";s:1:\"0\";s:12:\"limitmyblogs\";s:1:\"8\";s:14:\"limitmyfriends\";s:1:\"0\";s:16:\"limitmyfavforums\";s:1:\"5\";s:17:\"limitmyfavthreads\";s:1:\"0\";s:10:\"textlength\";s:3:\"300\";}'),
('spacestatus','1'),
('srchcensor','1'),
('srchhotkeywords','娲诲姩\r\n浜ゅ弸\r\ndiscuz'),
('stamplistlevel','3'),
('starthreshold','2'),
('statcode',''),
('statscachelife','180'),
('statstatus',''),
('strongpw','0'),
('styleid','2'),
('styleid1','1'),
('styleid2','1'),
('styleid3','1'),
('stylejump','1'),
('subforumsindex','0'),
('submitlock','0'),
('subnavs','a:0:{}'),
('switchwidthauto','1'),
('tagstatus','1'),
('targetblank','0'),
('taskstatus','0'),
('tasktypes','a:3:{s:9:\"promotion\";a:2:{s:4:\"name\";s:18:\"缃戠珯鎺ㄥ箍浠诲姟\";s:7:\"version\";s:3:\"1.0\";}s:4:\"gift\";a:2:{s:4:\"name\";s:15:\"绾㈠寘绫讳换鍔";s:7:\"version\";s:3:\"1.0\";}s:6:\"avatar\";a:2:{s:4:\"name\";s:15:\"澶村儚绫讳换鍔";s:7:\"version\";s:3:\"1.0\";}}'),
('threadblacklist','1'),
('threadfilternum','10'),
('threadguestlite','0'),
('threadhotreplies','0'),
('threadmaxpages','1000'),
('threadsticky','鍏ㄥ眬缃《,鍒嗙被缃《,鏈増缃《'),
('thumbheight','300'),
('thumbquality','100'),
('thumbsource','0'),
('thumbstatus',''),
('thumbwidth','400'),
('timeformat','H:i'),
('timeoffset','8'),
('topcachetime','60'),
('topicperpage','20'),
('tradeforumid','0'),
('transfermincredits','1000'),
('uc','a:1:{s:7:\"addfeed\";i:1;}'),
('ucactivation','1'),
('uidlogin','0'),
('updatestat','1'),
('userdateformat','Y-n-j\r\nY/n/j\r\nj-n-Y\r\nj/n/Y'),
('userreasons','寰堢粰鍔?\r\n绁為┈閮芥槸娴簯\r\n璧炰竴涓?\r\n灞卞\r\n娣″畾'),
('userstatusby','1'),
('verify','a:7:{i:6;a:6:{s:5:\"title\";s:12:\"瀹炲悕璁よ瘉\";s:9:\"available\";s:1:\"0\";s:8:\"showicon\";s:1:\"0\";s:12:\"viewrealname\";s:1:\"0\";s:5:\"field\";a:1:{s:8:\"realname\";s:8:\"realname\";}s:4:\"icon\";b:0;}s:7:\"enabled\";b:0;i:1;a:1:{s:4:\"icon\";s:0:\"\";}i:2;a:1:{s:4:\"icon\";s:0:\"\";}i:3;a:1:{s:4:\"icon\";s:0:\"\";}i:4;a:1:{s:4:\"icon\";s:0:\"\";}i:5;a:1:{s:4:\"icon\";s:0:\"\";}}'),
('video_allowalbum','0'),
('video_allowblog','0'),
('video_allowcomment','0'),
('video_allowdoing','1'),
('video_allowfriend','1'),
('video_allowpoke','1'),
('video_allowshare','0'),
('video_allowviewspace','1'),
('video_allowwall','1'),
('viewthreadtags','100'),
('visitbanperiods',''),
('visitedforums','10'),
('visitedthreads','0'),
('vtonlinestatus','1'),
('wallstatus','0'),
('wapcharset','0'),
('wapdateformat','n/j'),
('wapmps','500'),
('wapppp','5'),
('wapregister','0'),
('wapstatus','0'),
('waptpp','10'),
('warningexpiration','30'),
('warninglimit','3'),
('watermarkminheight','a:3:{s:6:\"portal\";s:1:\"0\";s:5:\"forum\";s:1:\"0\";s:5:\"album\";s:1:\"0\";}'),
('watermarkminwidth','a:3:{s:6:\"portal\";s:1:\"0\";s:5:\"forum\";s:1:\"0\";s:5:\"album\";s:1:\"0\";}'),
('watermarkquality','a:3:{s:6:\"portal\";s:2:\"90\";s:5:\"forum\";i:90;s:5:\"album\";i:90;}'),
('watermarkstatus','a:3:{s:6:\"portal\";s:1:\"0\";s:5:\"forum\";s:1:\"0\";s:5:\"album\";s:1:\"0\";}'),
('watermarktext','a:12:{s:4:\"text\";a:3:{s:6:\"portal\";s:0:\"\";s:5:\"forum\";s:0:\"\";s:5:\"album\";s:0:\"\";}s:8:\"fontpath\";a:3:{s:6:\"portal\";s:21:\"FetteSteinschrift.ttf\";s:5:\"forum\";s:21:\"FetteSteinschrift.ttf\";s:5:\"album\";s:21:\"FetteSteinschrift.ttf\";}s:4:\"size\";a:3:{s:6:\"portal\";s:0:\"\";s:5:\"forum\";s:0:\"\";s:5:\"album\";s:0:\"\";}s:5:\"angle\";a:3:{s:6:\"portal\";s:0:\"\";s:5:\"forum\";s:0:\"\";s:5:\"album\";s:0:\"\";}s:5:\"color\";a:3:{s:6:\"portal\";s:0:\"\";s:5:\"forum\";s:0:\"\";s:5:\"album\";s:0:\"\";}s:7:\"shadowx\";a:3:{s:6:\"portal\";s:0:\"\";s:5:\"forum\";s:0:\"\";s:5:\"album\";s:0:\"\";}s:7:\"shadowy\";a:3:{s:6:\"portal\";s:0:\"\";s:5:\"forum\";s:0:\"\";s:5:\"album\";s:0:\"\";}s:11:\"shadowcolor\";a:3:{s:6:\"portal\";s:0:\"\";s:5:\"forum\";s:0:\"\";s:5:\"album\";s:0:\"\";}s:10:\"translatex\";a:3:{s:6:\"portal\";s:0:\"\";s:5:\"forum\";s:0:\"\";s:5:\"album\";s:0:\"\";}s:10:\"translatey\";a:3:{s:6:\"portal\";s:0:\"\";s:5:\"forum\";s:0:\"\";s:5:\"album\";s:0:\"\";}s:5:\"skewx\";a:3:{s:6:\"portal\";s:0:\"\";s:5:\"forum\";s:0:\"\";s:5:\"album\";s:0:\"\";}s:5:\"skewy\";a:3:{s:6:\"portal\";s:0:\"\";s:5:\"forum\";s:0:\"\";s:5:\"album\";s:0:\"\";}}'),
('watermarktrans','a:3:{s:6:\"portal\";s:2:\"50\";s:5:\"forum\";i:50;s:5:\"album\";i:50;}'),
('watermarktype','a:3:{s:6:\"portal\";s:3:\"png\";s:5:\"forum\";s:3:\"png\";s:5:\"album\";s:3:\"png\";}'),
('welcomemsg','1'),
('welcomemsgtitle','{username}锛屾偍濂斤紝鎰熻阿鎮ㄧ殑娉ㄥ唽锛岃闃呰浠ヤ笅鍐呭銆?),
('welcomemsgtxt','灏婃暚鐨剓username}锛屾偍宸茬粡娉ㄥ唽鎴愪负{sitename}鐨勪細鍛橈紝璇锋偍鍦ㄥ彂琛ㄨ█璁烘椂锛岄伒瀹堝綋鍦版硶寰嬫硶瑙勩€俓r\n濡傛灉鎮ㄦ湁浠€涔堢枒闂彲浠ヨ仈绯荤鐞嗗憳锛孍mail: {adminemail}銆俓r\n\r\n\r\n{bbname}\r\n{time}'),
('whosonline_contract','0'),
('whosonlinestatus','3'),
('zoomstatus','1	600');
/*!40000 ALTER TABLE `pre_common_setting` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_common_smiley`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_common_smiley` (
  `id` smallint(6) unsigned NOT NULL AUTO_INCREMENT,
  `typeid` smallint(6) unsigned NOT NULL,
  `displayorder` tinyint(1) NOT NULL DEFAULT 0,
  `type` enum('smiley','stamp','stamplist') NOT NULL DEFAULT 'smiley',
  `code` varchar(30) NOT NULL DEFAULT '',
  `url` varchar(30) NOT NULL DEFAULT '',
  PRIMARY KEY (`id`),
  KEY `type` (`type`,`displayorder`)
) ENGINE=InnoDB AUTO_INCREMENT=179 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_common_smiley` WRITE;
/*!40000 ALTER TABLE `pre_common_smiley` DISABLE KEYS */;
INSERT INTO `pre_common_smiley` VALUES
(1,1,1,'smiley',':)','smile.gif'),
(2,1,2,'smiley',':(','sad.gif'),
(3,1,3,'smiley',':D','biggrin.gif'),
(4,1,4,'smiley',':\'(','cry.gif'),
(5,1,5,'smiley',':@','huffy.gif'),
(6,1,6,'smiley',':o','shocked.gif'),
(7,1,7,'smiley',':P','tongue.gif'),
(8,1,8,'smiley',':$','shy.gif'),
(9,1,9,'smiley',';P','titter.gif'),
(10,1,10,'smiley',':L','sweat.gif'),
(11,1,11,'smiley',':Q','mad.gif'),
(12,1,12,'smiley',':lol','lol.gif'),
(13,1,13,'smiley',':loveliness:','loveliness.gif'),
(14,1,14,'smiley',':funk:','funk.gif'),
(15,1,15,'smiley',':curse:','curse.gif'),
(16,1,16,'smiley',':dizzy:','dizzy.gif'),
(17,1,17,'smiley',':shutup:','shutup.gif'),
(18,1,18,'smiley',':sleepy:','sleepy.gif'),
(19,1,19,'smiley',':hug:','hug.gif'),
(20,1,20,'smiley',':victory:','victory.gif'),
(21,1,21,'smiley',':time:','time.gif'),
(22,1,22,'smiley',':kiss:','kiss.gif'),
(23,1,23,'smiley',':handshake','handshake.gif'),
(24,1,24,'smiley',':call:','call.gif'),
(25,2,1,'smiley','{:2_25:}','01.gif'),
(26,2,2,'smiley','{:2_26:}','02.gif'),
(27,2,3,'smiley','{:2_27:}','03.gif'),
(28,2,4,'smiley','{:2_28:}','04.gif'),
(29,2,5,'smiley','{:2_29:}','05.gif'),
(30,2,6,'smiley','{:2_30:}','06.gif'),
(31,2,7,'smiley','{:2_31:}','07.gif'),
(32,2,8,'smiley','{:2_32:}','08.gif'),
(33,2,9,'smiley','{:2_33:}','09.gif'),
(34,2,10,'smiley','{:2_34:}','10.gif'),
(35,2,11,'smiley','{:2_35:}','11.gif'),
(36,2,12,'smiley','{:2_36:}','12.gif'),
(37,2,13,'smiley','{:2_37:}','13.gif'),
(38,2,14,'smiley','{:2_38:}','14.gif'),
(39,2,15,'smiley','{:2_39:}','15.gif'),
(40,2,16,'smiley','{:2_40:}','16.gif'),
(41,3,1,'smiley','{:3_41:}','01.gif'),
(42,3,2,'smiley','{:3_42:}','02.gif'),
(43,3,3,'smiley','{:3_43:}','03.gif'),
(44,3,4,'smiley','{:3_44:}','04.gif'),
(45,3,5,'smiley','{:3_45:}','05.gif'),
(46,3,6,'smiley','{:3_46:}','06.gif'),
(47,3,7,'smiley','{:3_47:}','07.gif'),
(48,3,8,'smiley','{:3_48:}','08.gif'),
(49,3,9,'smiley','{:3_49:}','09.gif'),
(50,3,10,'smiley','{:3_50:}','10.gif'),
(51,3,11,'smiley','{:3_51:}','11.gif'),
(52,3,12,'smiley','{:3_52:}','12.gif'),
(53,3,13,'smiley','{:3_53:}','13.gif'),
(54,3,14,'smiley','{:3_54:}','14.gif'),
(55,3,15,'smiley','{:3_55:}','15.gif'),
(56,3,16,'smiley','{:3_56:}','16.gif'),
(57,3,17,'smiley','{:3_57:}','17.gif'),
(58,3,18,'smiley','{:3_58:}','18.gif'),
(59,3,19,'smiley','{:3_59:}','19.gif'),
(60,3,20,'smiley','{:3_60:}','20.gif'),
(61,3,21,'smiley','{:3_61:}','21.gif'),
(62,3,22,'smiley','{:3_62:}','22.gif'),
(63,3,23,'smiley','{:3_63:}','23.gif'),
(64,3,24,'smiley','{:3_64:}','24.gif'),
(65,0,0,'stamp','绮惧崕','001.gif'),
(66,0,1,'stamp','鐑笘','002.gif'),
(67,0,2,'stamp','缇庡浘','003.gif'),
(68,0,3,'stamp','浼樼','004.gif'),
(69,0,4,'stamp','缃《','005.gif'),
(70,0,5,'stamp','鎺ㄨ崘','006.gif'),
(71,0,6,'stamp','鍘熷垱','007.gif'),
(72,0,7,'stamp','鐗堜富鎺ㄨ崘','008.gif'),
(73,0,8,'stamp','鐖嗘枡','009.gif'),
(74,0,9,'stamplist','绮惧崕','001.small.gif'),
(75,0,10,'stamplist','鐑笘','002.small.gif'),
(76,0,11,'stamplist','缇庡浘','003.small.gif'),
(77,0,12,'stamplist','浼樼','004.small.gif'),
(78,0,13,'stamplist','缃《','005.small.gif'),
(79,0,14,'stamplist','鎺ㄨ崘','006.small.gif'),
(80,0,15,'stamplist','鍘熷垱','007.small.gif'),
(81,0,16,'stamplist','鐗堜富鎺ㄨ崘','008.small.gif'),
(82,0,17,'stamplist','鐖嗘枡','009.small.gif'),
(83,4,19,'stamp','缂栬緫閲囩敤','010.gif'),
(84,0,18,'stamplist','缂栬緫閲囩敤','010.small.gif'),
(85,0,20,'stamplist','鏂颁汉甯?,'011.small.gif'),
(86,4,1,'smiley',':kelian:','kelian.gif'),
(87,4,2,'smiley',':haqian:','haqian.gif'),
(88,4,3,'smiley',':woshou:','woshou.gif'),
(89,4,4,'smiley',':aixin:','aixin.gif'),
(90,4,5,'smiley',':zuohengheng:','zuohengheng.gif'),
(91,4,6,'smiley',':weixiao:','weixiao.gif'),
(92,4,7,'smiley',':jingkong:','jingkong.gif'),
(93,4,8,'smiley',':tiaopi:','tiaopi.gif'),
(94,4,9,'smiley',':touxiao:','touxiao.gif'),
(95,4,10,'smiley',':youling:','youling.gif'),
(96,4,11,'smiley',':caidao:','caidao.gif'),
(97,4,12,'smiley',':cahan:','cahan.gif'),
(98,4,13,'smiley',':hecai:','hecai.gif'),
(99,4,14,'smiley',':keai:','keai.gif'),
(100,4,15,'smiley',':ciya:','ciya.gif'),
(101,4,16,'smiley',':saorao:','saorao.gif'),
(102,4,17,'smiley',':jingxi:','jingxi.gif'),
(103,4,18,'smiley',':ku:','ku.gif'),
(104,4,19,'smiley',':piezui:','piezui.gif'),
(105,4,20,'smiley',':se:','se.gif'),
(106,4,21,'smiley',':xia:','xia.gif'),
(107,4,22,'smiley',':yinxian:','yinxian.gif'),
(108,4,23,'smiley',':zhouma:','zhouma.gif'),
(110,4,24,'smiley',':kulou:','kulou.gif'),
(111,4,25,'smiley',':xu:','xu.gif'),
(112,4,26,'smiley',':jingya:','jingya.gif'),
(113,4,27,'smiley',':doge:','doge.gif'),
(114,4,28,'smiley',':bizui:','bizui.gif'),
(115,4,29,'smiley',':yangtuo:','yangtuo.gif'),
(116,4,30,'smiley',':shouqiang:','shouqiang.gif'),
(117,4,31,'smiley',':baoquan:','baoquan.gif'),
(118,4,32,'smiley',':yun:','yun.gif'),
(119,4,33,'smiley',':lanqiu:','lanqiu.gif'),
(120,4,34,'smiley',':zhemo:','zhemo.gif'),
(121,4,35,'smiley',':guzhang:','guzhang.gif'),
(122,4,36,'smiley',':shengli:','shengli.gif'),
(123,4,37,'smiley',':zaijian:','zaijian.gif'),
(124,4,38,'smiley',':dabing:','dabing.gif'),
(125,4,39,'smiley',':deyi:','deyi.gif'),
(126,4,40,'smiley',':hanxiao:','hanxiao.gif'),
(127,4,41,'smiley',':kun:','kun.gif'),
(128,4,42,'smiley',':hexie:','hexie.gif'),
(129,4,43,'smiley',':daku:','daku.gif'),
(130,4,44,'smiley',':wozuimei:','wozuimei.gif'),
(131,4,45,'smiley',':xiaoku:','xiaoku.gif'),
(132,4,46,'smiley',':xigua:','xigua.gif'),
(133,4,47,'smiley',':huaixiao:','huaixiao.gif'),
(134,4,48,'smiley',':liulei:','liulei.gif'),
(135,4,49,'smiley',':lenghan:','lenghan.gif'),
(136,4,50,'smiley',':qiudale:','qiudale.gif'),
(137,4,51,'smiley',':zhayanjian:','zhayanjian.gif'),
(138,4,52,'smiley',':qiaoda:','qiaoda.gif'),
(139,4,53,'smiley',':baojin:','baojin.gif'),
(140,4,54,'smiley',':OK:','OK.gif'),
(141,4,55,'smiley',':xiaojiujie:','xiaojiujie.gif'),
(142,4,56,'smiley',':gouyin:','gouyin.gif'),
(143,4,57,'smiley',':youhengheng:','youhengheng.gif'),
(144,4,58,'smiley',':tuosai:','tuosai.gif'),
(145,4,59,'smiley',':nanguo:','nanguo.gif'),
(146,4,60,'smiley',':quantou:','quantou.gif'),
(147,4,61,'smiley',':haixiu:','haixiu.gif'),
(148,4,62,'smiley',':koubi:','koubi.gif'),
(149,4,63,'smiley',':qiang:','qiang.gif'),
(150,4,64,'smiley',':pijiu:','pijiu.gif'),
(151,4,65,'smiley',':bishi:','bishi.gif'),
(152,4,66,'smiley',':yiwen:','yiwen.gif'),
(153,4,67,'smiley',':liuhan:','liuhan.gif'),
(154,4,68,'smiley',':wunai:','wunai.gif'),
(155,4,69,'smiley',':aini:','aini.gif'),
(156,4,70,'smiley',':bangbangtang:','bangbangtang.gif'),
(157,4,71,'smiley',':penxue:','penxue.gif'),
(158,4,72,'smiley',':haobang:','haobang.gif'),
(159,4,73,'smiley',':qinqin:','qinqin.gif'),
(160,4,74,'smiley',':xiaoyanger:','xiaoyanger.gif'),
(161,4,75,'smiley',':fendou:','fendou.gif'),
(162,4,76,'smiley',':ganga:','ganga.gif'),
(163,4,77,'smiley',':shuai:','shuai.gif'),
(164,4,78,'smiley',':juhua:','juhua.gif'),
(165,4,79,'smiley',':baiyan:','baiyan.gif'),
(166,4,80,'smiley',':fanu:','fanu.gif'),
(167,4,81,'smiley',':jie:','jie.gif'),
(168,4,82,'smiley',':chi:','chi.gif'),
(169,4,83,'smiley',':kuaikule:','kuaikule.gif'),
(170,4,84,'smiley',':zhuakuang:','zhuakuang.gif'),
(171,4,85,'smiley',':shui:','shui.gif'),
(172,4,86,'smiley',':dan:','dan.gif'),
(173,4,87,'smiley',':aoman:','aoman.gif'),
(174,4,88,'smiley',':fadai:','fadai.gif'),
(175,4,89,'smiley',':leiben:','leiben.gif'),
(176,4,90,'smiley',':tu:','tu.gif'),
(177,4,91,'smiley',':weiqu:','weiqu.gif'),
(178,4,92,'smiley',':xieyanxiao:','xieyanxiao.gif');
/*!40000 ALTER TABLE `pre_common_smiley` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_common_smsgw`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_common_smsgw` (
  `smsgwid` int(10) unsigned NOT NULL AUTO_INCREMENT,
  `available` tinyint(1) NOT NULL DEFAULT 0,
  `type` int(10) NOT NULL DEFAULT 0,
  `order` int(10) NOT NULL DEFAULT 0,
  `name` varchar(255) NOT NULL DEFAULT '',
  `class` varchar(255) NOT NULL DEFAULT '0',
  `sendrule` text NOT NULL,
  `parameters` text NOT NULL,
  PRIMARY KEY (`smsgwid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_common_smsgw` WRITE;
/*!40000 ALTER TABLE `pre_common_smsgw` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_common_smsgw` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_common_smslog`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_common_smslog` (
  `smslogid` int(10) unsigned NOT NULL AUTO_INCREMENT,
  `uid` mediumint(8) unsigned NOT NULL,
  `smstype` int(10) NOT NULL DEFAULT 0,
  `svctype` int(10) NOT NULL DEFAULT 0,
  `smsgw` int(10) NOT NULL DEFAULT 0,
  `status` int(10) NOT NULL DEFAULT 0,
  `verify` int(10) NOT NULL DEFAULT 0,
  `secmobicc` varchar(3) NOT NULL DEFAULT '',
  `secmobile` varchar(12) NOT NULL DEFAULT '',
  `ip` varchar(45) NOT NULL DEFAULT '',
  `port` smallint(6) unsigned NOT NULL DEFAULT 0,
  `content` text NOT NULL,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`smslogid`),
  KEY `dateline` (`secmobicc`,`secmobile`,`dateline`),
  KEY `uid` (`uid`),
  KEY `status` (`status`,`dateline`,`uid`),
  KEY `dateline2` (`dateline`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_common_smslog` WRITE;
/*!40000 ALTER TABLE `pre_common_smslog` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_common_smslog` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_common_smslog_archive`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_common_smslog_archive` (
  `smslogid` int(10) unsigned NOT NULL AUTO_INCREMENT,
  `uid` mediumint(8) unsigned NOT NULL,
  `smstype` int(10) NOT NULL DEFAULT 0,
  `svctype` int(10) NOT NULL DEFAULT 0,
  `smsgw` int(10) NOT NULL DEFAULT 0,
  `status` int(10) NOT NULL DEFAULT 0,
  `verify` int(10) NOT NULL DEFAULT 0,
  `secmobicc` varchar(3) NOT NULL DEFAULT '',
  `secmobile` varchar(12) NOT NULL DEFAULT '',
  `ip` varchar(45) NOT NULL DEFAULT '',
  `port` smallint(6) unsigned NOT NULL DEFAULT 0,
  `content` text NOT NULL,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`smslogid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_common_smslog_archive` WRITE;
/*!40000 ALTER TABLE `pre_common_smslog_archive` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_common_smslog_archive` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_common_sphinxcounter`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_common_sphinxcounter` (
  `indexid` tinyint(1) NOT NULL,
  `maxid` int(10) NOT NULL,
  PRIMARY KEY (`indexid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_common_sphinxcounter` WRITE;
/*!40000 ALTER TABLE `pre_common_sphinxcounter` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_common_sphinxcounter` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_common_stat`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_common_stat` (
  `daytime` int(10) unsigned NOT NULL DEFAULT 0,
  `login` int(10) unsigned NOT NULL DEFAULT 0,
  `mobilelogin` int(10) unsigned NOT NULL DEFAULT 0,
  `connectlogin` int(10) unsigned NOT NULL DEFAULT 0,
  `register` int(10) unsigned NOT NULL DEFAULT 0,
  `invite` int(10) unsigned NOT NULL DEFAULT 0,
  `doing` int(10) unsigned NOT NULL DEFAULT 0,
  `blog` int(10) unsigned NOT NULL DEFAULT 0,
  `pic` int(10) unsigned NOT NULL DEFAULT 0,
  `poll` int(10) unsigned NOT NULL DEFAULT 0,
  `activity` int(10) unsigned NOT NULL DEFAULT 0,
  `share` int(10) unsigned NOT NULL DEFAULT 0,
  `thread` int(10) unsigned NOT NULL DEFAULT 0,
  `docomment` int(10) unsigned NOT NULL DEFAULT 0,
  `blogcomment` int(10) unsigned NOT NULL DEFAULT 0,
  `piccomment` int(10) unsigned NOT NULL DEFAULT 0,
  `sharecomment` int(10) unsigned NOT NULL DEFAULT 0,
  `reward` int(10) unsigned NOT NULL DEFAULT 0,
  `debate` int(10) unsigned NOT NULL DEFAULT 0,
  `trade` int(10) unsigned NOT NULL DEFAULT 0,
  `group` int(10) unsigned NOT NULL DEFAULT 0,
  `groupjoin` int(10) unsigned NOT NULL DEFAULT 0,
  `groupthread` int(10) unsigned NOT NULL DEFAULT 0,
  `grouppost` int(10) unsigned NOT NULL DEFAULT 0,
  `post` int(10) unsigned NOT NULL DEFAULT 0,
  `wall` int(10) unsigned NOT NULL DEFAULT 0,
  `poke` int(10) unsigned NOT NULL DEFAULT 0,
  `click` int(10) unsigned NOT NULL DEFAULT 0,
  `sendpm` int(10) unsigned NOT NULL DEFAULT 0,
  `friend` int(10) unsigned NOT NULL DEFAULT 0,
  `addfriend` int(10) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`daytime`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_common_stat` WRITE;
/*!40000 ALTER TABLE `pre_common_stat` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_common_stat` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_common_statuser`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_common_statuser` (
  `uid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `daytime` int(10) unsigned NOT NULL DEFAULT 0,
  `type` char(20) NOT NULL DEFAULT '',
  KEY `uid` (`uid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_common_statuser` WRITE;
/*!40000 ALTER TABLE `pre_common_statuser` DISABLE KEYS */;
INSERT INTO `pre_common_statuser` VALUES
(1,0,'login');
/*!40000 ALTER TABLE `pre_common_statuser` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_common_style`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_common_style` (
  `styleid` smallint(6) unsigned NOT NULL AUTO_INCREMENT,
  `name` varchar(20) NOT NULL DEFAULT '',
  `available` tinyint(1) NOT NULL DEFAULT 1,
  `templateid` smallint(6) unsigned NOT NULL DEFAULT 0,
  `extstyle` varchar(255) NOT NULL DEFAULT '',
  `version` varchar(20) NOT NULL DEFAULT '',
  PRIMARY KEY (`styleid`)
) ENGINE=InnoDB AUTO_INCREMENT=3 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_common_style` WRITE;
/*!40000 ALTER TABLE `pre_common_style` DISABLE KEYS */;
INSERT INTO `pre_common_style` VALUES
(1,'榛樿椋庢牸',1,1,'t1	t2	t3	t4	t5|','1.0.0'),
(2,'X5妯＄増',1,2,'','');
/*!40000 ALTER TABLE `pre_common_style` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_common_stylevar`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_common_stylevar` (
  `stylevarid` mediumint(8) unsigned NOT NULL AUTO_INCREMENT,
  `styleid` smallint(6) unsigned NOT NULL DEFAULT 0,
  `variable` text NOT NULL,
  `substitute` text NOT NULL,
  PRIMARY KEY (`stylevarid`),
  KEY `styleid` (`styleid`)
) ENGINE=InnoDB AUTO_INCREMENT=101 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_common_stylevar` WRITE;
/*!40000 ALTER TABLE `pre_common_stylevar` DISABLE KEYS */;
INSERT INTO `pre_common_stylevar` VALUES
(1,1,'menuhoverbgcolor','#004FA0'),
(2,1,'menucurbgcolor','#005AB4'),
(3,1,'lightlink','#FFF'),
(4,1,'floatbgcolor','#FFF'),
(5,1,'dropmenubgcolor','#FEFEFE'),
(6,1,'floatmaskbgcolor','#000'),
(7,1,'dropmenuborder','#DDD'),
(8,1,'specialbg','#E5EDF2'),
(9,1,'specialborder','#C2D5E3'),
(10,1,'commonbg','#F2F2F2'),
(11,1,'commonborder','#CDCDCD'),
(12,1,'inputbg','#FFF'),
(13,1,'stypeid','1'),
(14,1,'inputborderdarkcolor','#848484'),
(15,1,'headerbgcolor',''),
(16,1,'headerborder','0'),
(17,1,'sidebgcolor','#E8F0F7'),
(18,1,'msgfontsize','14px'),
(19,1,'bgcolor','#FFF'),
(20,1,'noticetext','#F26C4F'),
(21,1,'highlightlink','#369'),
(22,1,'link','#333'),
(23,1,'lighttext','#999'),
(24,1,'midtext','#666'),
(25,1,'tabletext','#444'),
(26,1,'smfontsize','0.83em'),
(27,1,'threadtitlefont','Tahoma,Helvetica,\'Microsoft Yahei\',sans-serif'),
(28,1,'threadtitlefontsize','14px'),
(29,1,'smfont','Tahoma,Helvetica,sans-serif'),
(30,1,'titlebgcolor','#E5EDF2'),
(31,1,'fontsize','12px/1.5'),
(32,1,'font','Tahoma,Helvetica,\'Microsoft Yahei\',sans-serif'),
(33,1,'styleimgdir',''),
(34,1,'imgdir',''),
(35,1,'boardimg','logo.svg'),
(36,1,'searchimg','logo_sc.svg'),
(37,1,'touchimg','logo_m.svg'),
(38,1,'available',''),
(39,1,'headertext','#444'),
(40,1,'footertext','#666'),
(41,1,'menubgcolor','#2B7ACD'),
(42,1,'menutext','#FFF'),
(43,1,'menuhovertext','#FFF'),
(44,1,'wrapbg','#FFF'),
(45,1,'wrapbordercolor','#CCC'),
(46,1,'contentwidth','630px'),
(47,1,'contentseparate','#C2D5E3'),
(48,1,'inputborder','#E0E0E0'),
(49,2,'menuhoverbgcolor','#0051cc'),
(50,2,'menucurbgcolor','#0051cc'),
(51,2,'lightlink','#ccc'),
(52,2,'floatbgcolor','#FFF'),
(53,2,'dropmenubgcolor','#FEFEFE'),
(54,2,'floatmaskbgcolor','#000'),
(55,2,'dropmenuborder','#eeeeee'),
(56,2,'specialbg','#d6e4ff'),
(57,2,'specialborder','#d6e4ff'),
(58,2,'commonbg','#F9f9f9'),
(59,2,'commonborder','#eeeeee'),
(60,2,'inputbg','#FFF'),
(61,2,'stypeid','1'),
(62,2,'inputborderdarkcolor','#e3e3e3'),
(63,2,'headerbgcolor',''),
(64,2,'headerborder','0'),
(65,2,'sidebgcolor','#E8F0F7'),
(66,2,'msgfontsize','14px'),
(67,2,'bgcolor','#f7f9fa'),
(68,2,'noticetext','#F26C4F'),
(69,2,'highlightlink','#0066ff'),
(70,2,'link','#333'),
(71,2,'lighttext','#999'),
(72,2,'midtext','#666'),
(73,2,'tabletext','#333'),
(74,2,'smfontsize','0.83em'),
(75,2,'threadtitlefont','Tahoma,Helvetica,\'Microsoft Yahei\',sans-serif'),
(76,2,'threadtitlefontsize','16px'),
(77,2,'smfont','Tahoma,Helvetica,sans-serif'),
(78,2,'titlebgcolor','#E5EDF2'),
(79,2,'fontsize','12px/1.5'),
(80,2,'font','Tahoma,Helvetica,\'Microsoft Yahei\',sans-serif'),
(81,2,'styleimgdir','template/discuzx5/static/'),
(82,2,'imgdir',''),
(83,2,'boardimg','logo.png'),
(84,2,'searchimg','logo_sc.svg'),
(85,2,'touchimg','logo_m.svg'),
(86,2,'available',''),
(87,2,'headertext','#444'),
(88,2,'footertext','#666'),
(89,2,'menubgcolor','#0066ff'),
(90,2,'menutext','#0066ff'),
(91,2,'menuhovertext','#FFF'),
(92,2,'wrapbg','#FFF'),
(93,2,'wrapbordercolor','#CCC'),
(94,2,'contentwidth','630px'),
(95,2,'contentseparate','#d6e4ff'),
(96,2,'inputborder','#e3e3e3'),
(97,2,'btnbg','#d6e4ff'),
(98,2,'btntxt','#0066ff'),
(99,2,'btnbga','#0066ff'),
(100,2,'btntxta','#ffffff');
/*!40000 ALTER TABLE `pre_common_stylevar` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_common_stylevar_extra`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_common_stylevar_extra` (
  `stylevarid` mediumint(8) unsigned NOT NULL AUTO_INCREMENT,
  `styleid` smallint(6) unsigned NOT NULL DEFAULT 0,
  `displayorder` int(10) NOT NULL DEFAULT 0,
  `title` varchar(100) NOT NULL DEFAULT '',
  `description` varchar(255) NOT NULL DEFAULT '',
  `variable` varchar(40) NOT NULL DEFAULT '',
  `type` varchar(255) NOT NULL DEFAULT 'text',
  `value` text NOT NULL,
  `extra` text NOT NULL,
  PRIMARY KEY (`stylevarid`),
  KEY `styleid` (`styleid`)
) ENGINE=InnoDB AUTO_INCREMENT=53 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_common_stylevar_extra` WRITE;
/*!40000 ALTER TABLE `pre_common_stylevar_extra` DISABLE KEYS */;
INSERT INTO `pre_common_stylevar_extra` VALUES
(1,1,0,'鎵嬫満鐗堥厤鑹?,'','touch_style','stylePage','',''),
(2,1,2,'妯℃澘涓昏壊璋?,'妯℃澘涓昏壊璋冮鑹? 榛樿鍊?#2B7ACD','touch_style_color','color','#2B7ACD',''),
(3,1,3,'甯歌楂樹寒鑹?,'甯歌楂樹寒鏂囧瓧鎴栬儗鏅鑹? 榛樿鍊?#FF5656','touch_style_bg2','color','#FF5656',''),
(4,1,4,'甯歌杈规鑹?,'甯歌杈规棰滆壊, 榛樿鍊?#EDEDED','touch_style_border','color','#EDEDED',''),
(5,1,8,'甯歌鑳屾櫙棰滆壊','','touch_style_bgs','styleTitle','',''),
(6,1,11,'椤甸潰鑳屾櫙','鍏ㄥ眬椤甸潰鑳屾櫙鑹? 榛樿鍊?#EEEEEE','touch_style_bodybg','color','#EEEEEE',''),
(7,1,12,'妯″潡鑳屾櫙','甯歌妯″潡鑳屾櫙鑹? 榛樿鍊?#FFFFFF','touch_style_bg0','color','#FFFFFF',''),
(8,1,13,'娣辫壊鑳屾櫙','甯歌娣辫壊鑳屾櫙鑹? 榛樿鍊?#333333','touch_style_bg1','color','#333333',''),
(9,1,14,'杈呭姪鑳屾櫙','杈呭姪鑳屾櫙鑹? 涓昏鐢ㄤ簬鎺掕姒滄垨缃《鍥炬爣绛夎儗鏅娇鐢? 榛樿鍊? #FF9C00','touch_style_bg3','color','#FF9900',''),
(10,1,15,'杈呭姪鑳屾櫙','杈呭姪鑳屾櫙鑹? 涓昏鐢ㄤ簬鎺掕姒滄垨缃《鍥炬爣绛夎儗鏅娇鐢? 榛樿鍊?#B3CC0D','touch_style_bg4','color','#B3CC0D',''),
(11,1,16,'杈呭姪鑳屾櫙','杈呭姪鑳屾櫙鑹? 涓昏鐢ㄤ簬涓€浜涘浘鏍囨垨鎸夐挳鐨勮儗鏅娇鐢? 榛樿鍊?#F3F3F3','touch_style_bg5','color','#F3F3F3',''),
(12,1,17,'杈呭姪鑳屾櫙','杈呭姪鑳屾櫙鑹? 涓昏鐢ㄤ簬涓€浜涘浘鏍囨垨鎸夐挳鐨勮儗鏅娇鐢? 榛樿鍊?#CCCCCC','touch_style_bg6','color','#CCCCCC',''),
(13,1,18,'杈呭姪鑳屾櫙','杈呭姪鑳屾櫙鑹? 涓昏鐢ㄤ簬涓€浜涘浘鏍囨垨鎸夐挳鐨勮儗鏅娇鐢? 榛樿鍊?#A0C8EA','touch_style_bgn','color','#A0C8EA',''),
(14,1,20,'甯歌鏂囧瓧棰滆壊','','touch_style_text','styleTitle','',''),
(15,1,21,'鐧借壊鏂囧瓧鑹?,'鐧借壊鏂囧瓧棰滆壊, 榛樿鍊?#FFFFFF','touch_style_tfff','color','#FFFFFF',''),
(16,1,22,'甯歌鏂囧瓧鑹?,'甯歌鏂囧瓧棰滆壊, 榛樿鍊?#333333','touch_style_t333','color','#333333',''),
(17,1,23,'涓瓑鏂囧瓧鑹?,'涓瓑鏂囧瓧棰滆壊, 榛樿鍊?#666666','touch_style_t666','color','#666666',''),
(18,1,24,'娴呰壊鏂囧瓧鑹?,'娴呰壊鏂囧瓧棰滆壊, 榛樿鍊?#999999','touch_style_t999','color','#999999',''),
(19,1,25,'杈呭姪鏂囧瓧鑹?,'杈呭姪鏂囧瓧棰滆壊, 榛樿鍊?#777777','touch_style_t777','color','#777777',''),
(20,1,26,'杈呭姪鏂囧瓧鑹?,'杈呭姪鏂囧瓧棰滆壊, 榛樿鍊?#888888','touch_style_t888','color','#888888',''),
(21,1,27,'杈呭姪鏂囧瓧鑹?,'杈呭姪鏂囧瓧棰滆壊, 榛樿鍊?#AAAAAA','touch_style_taaa','color','#AAAAAA',''),
(22,1,28,'杈呭姪鏂囧瓧鑹?,'杈呭姪鏂囧瓧棰滆壊, 榛樿鍊?#BBBBBB','touch_style_tbbb','color','#BBBBBB',''),
(23,1,29,'杈呭姪鏂囧瓧鑹?,'杈呭姪鏂囧瓧棰滆壊, 榛樿鍊?#CCCCCC','touch_style_tccc','color','#CCCCCC',''),
(24,1,30,'杈呭姪鏂囧瓧鑹?,'杈呭姪鏂囧瓧棰滆壊, 榛樿鍊?#DDDDDD','touch_style_tddd','color','#DDDDDD',''),
(25,1,31,'杈呭姪鏂囧瓧鑹?,'杈呭姪鏂囧瓧棰滆壊, 榛樿鍊?#7DA0CC','touch_style_tnnn','color','#7DA0CC',''),
(26,1,32,'杈呭姪鏂囧瓧鑹?,'杈呭姪鏂囧瓧棰滆壊, 榛樿鍊?#FF9C00','touch_style_tlight','color','#FF9C00',''),
(27,1,33,'杈呭姪鏂囧瓧鑹?,'杈呭姪鏂囧瓧棰滆壊, 榛樿鍊?#FF5656','touch_style_ta','color','#FF5656',''),
(28,1,34,'杈呭姪鏂囧瓧鑹?,'杈呭姪鏂囧瓧棰滆壊, 榛樿鍊?#7CBE00','touch_style_tv','color','#7CBE00',''),
(29,1,35,'鑷畾涔塁SS','','touch_style_add','styleTitle','',''),
(30,1,36,'闄勫姞鑷畾涔塁SS','闄勫姞鑷畾涔塁SS瀵归〉闈㈣繘琛屼釜鎬у寲璁剧疆','touch_style_addcss','textarea','',''),
(31,2,0,'鐣岄潰璁剧疆','','template','stylePage','',''),
(32,2,10,'妯＄増澶撮儴','','template_top','styleTitle','',''),
(33,2,11,'鎼滅储妗?,'瀹氫箟椤堕儴鎼滅储妗嗙殑寮€鍏?,'is_search','radio','1',''),
(34,2,12,'椤堕儴瀵艰埅鍥哄畾','褰撳悜涓婃粴鍔ㄥ睆骞曟椂锛屼富瀵艰埅灏嗗嵆鏃跺嚭鐜板湪灞忓箷鐨勯《閮?,'is_fixtop','radio','1',''),
(35,2,13,'瀵艰埅鏄剧ず鏁伴噺','瀵艰埅鏈€澶氭樉绀哄嚑涓悗鏄剧ず鈥滄洿澶氣€?,'top_navnum','number','8',''),
(36,2,14,'瀵艰埅鑷敱瀹藉害','閫夋嫨鈥滄槸鈥濆鑸樉绀鸿嚜鐢卞搴︼紝閫夋嫨鈥滃惁鈥濆鑸樉绀哄浐瀹氬搴?,'top_nav_widthauto','radio','0',''),
(37,2,15,'瀵艰埅鑳屾櫙棰滆壊','閰嶈壊瀵艰埅鑳屾櫙棰滆壊锛岀暀绌轰负榛樿鐨勭櫧鑹?,'top_nav_bgc','color','',''),
(38,2,16,'瀵艰埅鑳屾櫙娣辫壊','璺熶笂闈㈢殑瀵艰埅鑳屾櫙棰滆壊涓€璧蜂娇鐢紝濡傛灉璁剧疆鐨勬槸娣辫壊鑳屾櫙棰滆壊锛屽苟閫夋嫨浜嗏€滃鑸儗鏅繁鑹测€濅负鏄紝瀵艰埅鏂囧瓧棰滆壊鍒欎細鏄剧ず涓虹櫧鑹?,'top_nav_dark','radio','0',''),
(39,2,17,'蹇嵎瀵艰埅','鏄惁鏄剧ず鐢ㄦ埛鍚嶄笅鎷変腑鐨勫揩鎹峰鑸?,'top_fastnav','radio','1',''),
(40,2,20,'渚ц竟宸ュ叿鏍?,'','sider_tool','styleTitle','',''),
(41,2,22,'寰俊瀹㈡湇','','sider_wechat','radio','1',''),
(42,2,23,'寰俊瀹㈡湇浜岀淮鐮?,'','sider_wechat_qrcode','uploadimage','template/discuzx5/static/images/wechat.jpg',''),
(43,2,24,'寰俊瀹㈡湇鏂囧瓧璇存槑','','sider_wechat_txt','text','鍏虫敞鍏紬鍙?,''),
(44,2,25,'蹇€熷彂甯?,'','sider_fastpost','radio','1',''),
(45,2,30,'妯＄増搴曢儴','','template_bottom','styleTitle','',''),
(46,2,31,'搴曢儴鑳屾櫙棰滆壊','璁剧疆搴曢儴鐨勮儗鏅鑹?,'bottom_bgc','color','#333333',''),
(47,2,32,'搴曢儴鑳屾櫙娣辫壊','璺熶笂闈㈢殑搴曢儴鑳屾櫙棰滆壊涓€璧蜂娇鐢紝濡傛灉璁剧疆鐨勬槸娣辫壊鑳屾櫙棰滆壊锛屽苟閫夋嫨浜嗏€滃簳閮ㄨ儗鏅繁鑹测€濅负鏄紝搴曢儴鏂囧瓧棰滆壊鍒欎細鏄剧ず涓虹櫧鑹?,'bottom_dark','radio','1',''),
(48,2,33,'浜岀淮鐮?,'涓嶈缃皢鏄剧ず榛樿鍥剧墖锛屽鏋滀簩缁寸爜鍜屼簩缁寸爜鏂囧瓧閮戒笉璁剧疆锛屽皢涓嶆樉绀?,'bottom_qrcode','uploadimage','template/discuzx5/static/images/ewm_a.jpg',''),
(49,2,34,'浜岀淮鐮佹枃瀛?,'涓嶈缃皢鏄剧ず榛樿鏂囧瓧锛屽鏋滀簩缁寸爜鍜屼簩缁寸爜鏂囧瓧閮戒笉璁剧疆锛屽皢涓嶆樉绀?,'bottom_qrcodetxt','text','鍏虫敞鍏紬鍙?,''),
(50,2,35,'搴曢儴鏂囧瓧淇℃伅','','bottom_txt','text','鐩稿叧渚垫潈銆佷妇鎶ャ€佹姇璇夊強寤鸿绛夛紝璇峰彂 E-mail锛歛dmin@discuz.vip',''),
(51,2,100,'鍚庡彴璁剧疆','','admin','stylePage','',''),
(52,2,101,'鍚庡彴涓昏壊璋?,'','admin_color','color','#00b96b','');
/*!40000 ALTER TABLE `pre_common_stylevar_extra` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_common_syscache`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_common_syscache` (
  `cname` varchar(32) NOT NULL,
  `ctype` tinyint(3) unsigned NOT NULL,
  `dateline` int(10) unsigned NOT NULL,
  `data` mediumblob NOT NULL,
  PRIMARY KEY (`cname`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_common_syscache` WRITE;
/*!40000 ALTER TABLE `pre_common_syscache` DISABLE KEYS */;
INSERT INTO `pre_common_syscache` VALUES
('admin',1,1784453715,'a:2:{s:9:\"component\";a:3:{s:14:\"component_size\";a:2:{s:4:\"name\";s:9:\"瀹归噺鍊糪";s:5:\"class\";s:21:\"\\admin\\component_size\";}s:14:\"component_perm\";a:2:{s:4:\"name\";s:15:\"鏉冮檺閫夋嫨鍣╘";s:5:\"class\";s:21:\"\\admin\\component_perm\";}s:14:\"component_list\";a:2:{s:4:\"name\";s:15:\"鑷畾涔夊垪琛╘";s:5:\"class\";s:21:\"\\admin\\component_list\";}}s:8:\"subperms\";a:0:{}}'),
('admingroup_1',1,1784453715,'a:61:{s:8:\"admingid\";s:1:\"1\";s:13:\"alloweditpost\";s:1:\"1\";s:13:\"alloweditpoll\";s:1:\"1\";s:16:\"allowstickthread\";s:1:\"3\";s:12:\"allowmodpost\";s:1:\"1\";s:12:\"allowdelpost\";s:1:\"1\";s:14:\"allowmassprune\";s:1:\"1\";s:11:\"allowrefund\";s:1:\"1\";s:15:\"allowcensorword\";s:1:\"1\";s:11:\"allowviewip\";s:1:\"1\";s:10:\"allowbanip\";s:1:\"1\";s:13:\"allowedituser\";s:1:\"1\";s:12:\"allowmoduser\";s:1:\"1\";s:12:\"allowbanuser\";s:1:\"1\";s:17:\"allowbanvisituser\";s:1:\"1\";s:17:\"allowpostannounce\";s:1:\"1\";s:12:\"allowviewlog\";s:1:\"1\";s:12:\"allowbanpost\";s:1:\"1\";s:20:\"supe_allowpushthread\";s:1:\"1\";s:20:\"allowhighlightthread\";s:1:\"1\";s:15:\"allowlivethread\";s:1:\"1\";s:17:\"allowdigestthread\";s:1:\"3\";s:20:\"allowrecommendthread\";s:1:\"1\";s:15:\"allowbumpthread\";s:1:\"1\";s:16:\"allowclosethread\";s:1:\"1\";s:15:\"allowmovethread\";s:1:\"1\";s:19:\"allowedittypethread\";s:1:\"1\";s:16:\"allowstampthread\";s:1:\"1\";s:14:\"allowstamplist\";s:1:\"1\";s:15:\"allowcopythread\";s:1:\"1\";s:16:\"allowmergethread\";s:1:\"1\";s:16:\"allowsplitthread\";s:1:\"1\";s:17:\"allowrepairthread\";s:1:\"1\";s:13:\"allowwarnpost\";s:1:\"1\";s:15:\"allowviewreport\";s:1:\"1\";s:14:\"alloweditforum\";s:1:\"1\";s:17:\"allowremovereward\";s:1:\"1\";s:14:\"allowedittrade\";s:1:\"1\";s:17:\"alloweditactivity\";s:1:\"1\";s:15:\"allowstickreply\";s:1:\"1\";s:18:\"allowmanagearticle\";s:1:\"1\";s:13:\"allowaddtopic\";s:1:\"1\";s:16:\"allowmanagetopic\";s:1:\"1\";s:8:\"allowdiy\";s:1:\"1\";s:17:\"allowclearrecycle\";s:1:\"1\";s:14:\"allowmanagetag\";s:1:\"1\";s:16:\"alloweditusertag\";s:1:\"0\";s:10:\"managefeed\";s:1:\"1\";s:11:\"managedoing\";s:1:\"1\";s:11:\"manageshare\";s:1:\"1\";s:10:\"manageblog\";s:1:\"1\";s:11:\"managealbum\";s:1:\"1\";s:13:\"managecomment\";s:1:\"1\";s:14:\"managemagiclog\";s:1:\"1\";s:12:\"managereport\";s:1:\"1\";s:13:\"managehotuser\";s:1:\"1\";s:17:\"managedefaultuser\";s:1:\"1\";s:11:\"managemagic\";s:1:\"1\";s:11:\"manageclick\";s:1:\"1\";s:21:\"allowmanagecollection\";s:1:\"1\";s:13:\"allowmakehtml\";s:1:\"1\";}'),
('admingroup_16',1,1784453715,'a:61:{s:8:\"admingid\";s:2:\"16\";s:13:\"alloweditpost\";s:1:\"0\";s:13:\"alloweditpoll\";s:1:\"0\";s:16:\"allowstickthread\";s:1:\"1\";s:12:\"allowmodpost\";s:1:\"0\";s:12:\"allowdelpost\";s:1:\"0\";s:14:\"allowmassprune\";s:1:\"0\";s:11:\"allowrefund\";s:1:\"0\";s:15:\"allowcensorword\";s:1:\"0\";s:11:\"allowviewip\";s:1:\"0\";s:10:\"allowbanip\";s:1:\"0\";s:13:\"allowedituser\";s:1:\"0\";s:12:\"allowmoduser\";s:1:\"0\";s:12:\"allowbanuser\";s:1:\"0\";s:17:\"allowbanvisituser\";s:1:\"0\";s:17:\"allowpostannounce\";s:1:\"0\";s:12:\"allowviewlog\";s:1:\"0\";s:12:\"allowbanpost\";s:1:\"1\";s:20:\"supe_allowpushthread\";s:1:\"0\";s:20:\"allowhighlightthread\";s:1:\"1\";s:15:\"allowlivethread\";s:1:\"0\";s:17:\"allowdigestthread\";s:1:\"1\";s:20:\"allowrecommendthread\";s:1:\"1\";s:15:\"allowbumpthread\";s:1:\"1\";s:16:\"allowclosethread\";s:1:\"1\";s:15:\"allowmovethread\";s:1:\"0\";s:19:\"allowedittypethread\";s:1:\"0\";s:16:\"allowstampthread\";s:1:\"1\";s:14:\"allowstamplist\";s:1:\"0\";s:15:\"allowcopythread\";s:1:\"0\";s:16:\"allowmergethread\";s:1:\"0\";s:16:\"allowsplitthread\";s:1:\"0\";s:17:\"allowrepairthread\";s:1:\"0\";s:13:\"allowwarnpost\";s:1:\"1\";s:15:\"allowviewreport\";s:1:\"1\";s:14:\"alloweditforum\";s:1:\"0\";s:17:\"allowremovereward\";s:1:\"0\";s:14:\"allowedittrade\";s:1:\"0\";s:17:\"alloweditactivity\";s:1:\"0\";s:15:\"allowstickreply\";s:1:\"0\";s:18:\"allowmanagearticle\";s:1:\"0\";s:13:\"allowaddtopic\";s:1:\"0\";s:16:\"allowmanagetopic\";s:1:\"0\";s:8:\"allowdiy\";s:1:\"0\";s:17:\"allowclearrecycle\";s:1:\"0\";s:14:\"allowmanagetag\";s:1:\"0\";s:16:\"alloweditusertag\";s:1:\"0\";s:10:\"managefeed\";s:1:\"0\";s:11:\"managedoing\";s:1:\"0\";s:11:\"manageshare\";s:1:\"0\";s:10:\"manageblog\";s:1:\"0\";s:11:\"managealbum\";s:1:\"0\";s:13:\"managecomment\";s:1:\"0\";s:14:\"managemagiclog\";s:1:\"0\";s:12:\"managereport\";s:1:\"0\";s:13:\"managehotuser\";s:1:\"0\";s:17:\"managedefaultuser\";s:1:\"0\";s:11:\"managemagic\";s:1:\"0\";s:11:\"manageclick\";s:1:\"0\";s:21:\"allowmanagecollection\";s:1:\"0\";s:13:\"allowmakehtml\";s:1:\"0\";}'),
('admingroup_17',1,1784453715,'a:61:{s:8:\"admingid\";s:2:\"17\";s:13:\"alloweditpost\";s:1:\"1\";s:13:\"alloweditpoll\";s:1:\"0\";s:16:\"allowstickthread\";s:1:\"2\";s:12:\"allowmodpost\";s:1:\"1\";s:12:\"allowdelpost\";s:1:\"0\";s:14:\"allowmassprune\";s:1:\"0\";s:11:\"allowrefund\";s:1:\"1\";s:15:\"allowcensorword\";s:1:\"0\";s:11:\"allowviewip\";s:1:\"1\";s:10:\"allowbanip\";s:1:\"0\";s:13:\"allowedituser\";s:1:\"0\";s:12:\"allowmoduser\";s:1:\"0\";s:12:\"allowbanuser\";s:1:\"0\";s:17:\"allowbanvisituser\";s:1:\"0\";s:17:\"allowpostannounce\";s:1:\"1\";s:12:\"allowviewlog\";s:1:\"1\";s:12:\"allowbanpost\";s:1:\"1\";s:20:\"supe_allowpushthread\";s:1:\"0\";s:20:\"allowhighlightthread\";s:1:\"1\";s:15:\"allowlivethread\";s:1:\"0\";s:17:\"allowdigestthread\";s:1:\"3\";s:20:\"allowrecommendthread\";s:1:\"1\";s:15:\"allowbumpthread\";s:1:\"1\";s:16:\"allowclosethread\";s:1:\"1\";s:15:\"allowmovethread\";s:1:\"1\";s:19:\"allowedittypethread\";s:1:\"1\";s:16:\"allowstampthread\";s:1:\"1\";s:14:\"allowstamplist\";s:1:\"0\";s:15:\"allowcopythread\";s:1:\"1\";s:16:\"allowmergethread\";s:1:\"1\";s:16:\"allowsplitthread\";s:1:\"1\";s:17:\"allowrepairthread\";s:1:\"1\";s:13:\"allowwarnpost\";s:1:\"1\";s:15:\"allowviewreport\";s:1:\"1\";s:14:\"alloweditforum\";s:1:\"1\";s:17:\"allowremovereward\";s:1:\"1\";s:14:\"allowedittrade\";s:1:\"0\";s:17:\"alloweditactivity\";s:1:\"0\";s:15:\"allowstickreply\";s:1:\"0\";s:18:\"allowmanagearticle\";s:1:\"0\";s:13:\"allowaddtopic\";s:1:\"0\";s:16:\"allowmanagetopic\";s:1:\"0\";s:8:\"allowdiy\";s:1:\"0\";s:17:\"allowclearrecycle\";s:1:\"0\";s:14:\"allowmanagetag\";s:1:\"0\";s:16:\"alloweditusertag\";s:1:\"0\";s:10:\"managefeed\";s:1:\"0\";s:11:\"managedoing\";s:1:\"0\";s:11:\"manageshare\";s:1:\"0\";s:10:\"manageblog\";s:1:\"0\";s:11:\"managealbum\";s:1:\"0\";s:13:\"managecomment\";s:1:\"0\";s:14:\"managemagiclog\";s:1:\"0\";s:12:\"managereport\";s:1:\"0\";s:13:\"managehotuser\";s:1:\"0\";s:17:\"managedefaultuser\";s:1:\"0\";s:11:\"managemagic\";s:1:\"0\";s:11:\"manageclick\";s:1:\"0\";s:21:\"allowmanagecollection\";s:1:\"0\";s:13:\"allowmakehtml\";s:1:\"0\";}'),
('admingroup_18',1,1784453715,'a:61:{s:8:\"admingid\";s:2:\"18\";s:13:\"alloweditpost\";s:1:\"0\";s:13:\"alloweditpoll\";s:1:\"0\";s:16:\"allowstickthread\";s:1:\"0\";s:12:\"allowmodpost\";s:1:\"0\";s:12:\"allowdelpost\";s:1:\"0\";s:14:\"allowmassprune\";s:1:\"0\";s:11:\"allowrefund\";s:1:\"0\";s:15:\"allowcensorword\";s:1:\"0\";s:11:\"allowviewip\";s:1:\"1\";s:10:\"allowbanip\";s:1:\"0\";s:13:\"allowedituser\";s:1:\"0\";s:12:\"allowmoduser\";s:1:\"0\";s:12:\"allowbanuser\";s:1:\"0\";s:17:\"allowbanvisituser\";s:1:\"0\";s:17:\"allowpostannounce\";s:1:\"0\";s:12:\"allowviewlog\";s:1:\"0\";s:12:\"allowbanpost\";s:1:\"0\";s:20:\"supe_allowpushthread\";s:1:\"1\";s:20:\"allowhighlightthread\";s:1:\"0\";s:15:\"allowlivethread\";s:1:\"0\";s:17:\"allowdigestthread\";s:1:\"0\";s:20:\"allowrecommendthread\";s:1:\"0\";s:15:\"allowbumpthread\";s:1:\"0\";s:16:\"allowclosethread\";s:1:\"0\";s:15:\"allowmovethread\";s:1:\"0\";s:19:\"allowedittypethread\";s:1:\"0\";s:16:\"allowstampthread\";s:1:\"0\";s:14:\"allowstamplist\";s:1:\"0\";s:15:\"allowcopythread\";s:1:\"0\";s:16:\"allowmergethread\";s:1:\"0\";s:16:\"allowsplitthread\";s:1:\"0\";s:17:\"allowrepairthread\";s:1:\"0\";s:13:\"allowwarnpost\";s:1:\"0\";s:15:\"allowviewreport\";s:1:\"0\";s:14:\"alloweditforum\";s:1:\"0\";s:17:\"allowremovereward\";s:1:\"0\";s:14:\"allowedittrade\";s:1:\"0\";s:17:\"alloweditactivity\";s:1:\"0\";s:15:\"allowstickreply\";s:1:\"0\";s:18:\"allowmanagearticle\";s:1:\"0\";s:13:\"allowaddtopic\";s:1:\"0\";s:16:\"allowmanagetopic\";s:1:\"0\";s:8:\"allowdiy\";s:1:\"0\";s:17:\"allowclearrecycle\";s:1:\"0\";s:14:\"allowmanagetag\";s:1:\"0\";s:16:\"alloweditusertag\";s:1:\"0\";s:10:\"managefeed\";s:1:\"0\";s:11:\"managedoing\";s:1:\"0\";s:11:\"manageshare\";s:1:\"0\";s:10:\"manageblog\";s:1:\"0\";s:11:\"managealbum\";s:1:\"0\";s:13:\"managecomment\";s:1:\"0\";s:14:\"managemagiclog\";s:1:\"0\";s:12:\"managereport\";s:1:\"0\";s:13:\"managehotuser\";s:1:\"0\";s:17:\"managedefaultuser\";s:1:\"0\";s:11:\"managemagic\";s:1:\"0\";s:11:\"manageclick\";s:1:\"0\";s:21:\"allowmanagecollection\";s:1:\"0\";s:13:\"allowmakehtml\";s:1:\"0\";}'),
('admingroup_19',1,1784453715,'a:61:{s:8:\"admingid\";s:2:\"19\";s:13:\"alloweditpost\";s:1:\"0\";s:13:\"alloweditpoll\";s:1:\"0\";s:16:\"allowstickthread\";s:1:\"0\";s:12:\"allowmodpost\";s:1:\"1\";s:12:\"allowdelpost\";s:1:\"0\";s:14:\"allowmassprune\";s:1:\"0\";s:11:\"allowrefund\";s:1:\"0\";s:15:\"allowcensorword\";s:1:\"0\";s:11:\"allowviewip\";s:1:\"1\";s:10:\"allowbanip\";s:1:\"1\";s:13:\"allowedituser\";s:1:\"0\";s:12:\"allowmoduser\";s:1:\"1\";s:12:\"allowbanuser\";s:1:\"1\";s:17:\"allowbanvisituser\";s:1:\"0\";s:17:\"allowpostannounce\";s:1:\"0\";s:12:\"allowviewlog\";s:1:\"0\";s:12:\"allowbanpost\";s:1:\"0\";s:20:\"supe_allowpushthread\";s:1:\"0\";s:20:\"allowhighlightthread\";s:1:\"0\";s:15:\"allowlivethread\";s:1:\"0\";s:17:\"allowdigestthread\";s:1:\"0\";s:20:\"allowrecommendthread\";s:1:\"0\";s:15:\"allowbumpthread\";s:1:\"0\";s:16:\"allowclosethread\";s:1:\"0\";s:15:\"allowmovethread\";s:1:\"0\";s:19:\"allowedittypethread\";s:1:\"0\";s:16:\"allowstampthread\";s:1:\"0\";s:14:\"allowstamplist\";s:1:\"0\";s:15:\"allowcopythread\";s:1:\"0\";s:16:\"allowmergethread\";s:1:\"0\";s:16:\"allowsplitthread\";s:1:\"0\";s:17:\"allowrepairthread\";s:1:\"0\";s:13:\"allowwarnpost\";s:1:\"0\";s:15:\"allowviewreport\";s:1:\"0\";s:14:\"alloweditforum\";s:1:\"0\";s:17:\"allowremovereward\";s:1:\"0\";s:14:\"allowedittrade\";s:1:\"0\";s:17:\"alloweditactivity\";s:1:\"0\";s:15:\"allowstickreply\";s:1:\"0\";s:18:\"allowmanagearticle\";s:1:\"0\";s:13:\"allowaddtopic\";s:1:\"0\";s:16:\"allowmanagetopic\";s:1:\"0\";s:8:\"allowdiy\";s:1:\"0\";s:17:\"allowclearrecycle\";s:1:\"0\";s:14:\"allowmanagetag\";s:1:\"0\";s:16:\"alloweditusertag\";s:1:\"0\";s:10:\"managefeed\";s:1:\"0\";s:11:\"managedoing\";s:1:\"0\";s:11:\"manageshare\";s:1:\"0\";s:10:\"manageblog\";s:1:\"0\";s:11:\"managealbum\";s:1:\"0\";s:13:\"managecomment\";s:1:\"0\";s:14:\"managemagiclog\";s:1:\"0\";s:12:\"managereport\";s:1:\"0\";s:13:\"managehotuser\";s:1:\"0\";s:17:\"managedefaultuser\";s:1:\"0\";s:11:\"managemagic\";s:1:\"0\";s:11:\"manageclick\";s:1:\"0\";s:21:\"allowmanagecollection\";s:1:\"0\";s:13:\"allowmakehtml\";s:1:\"0\";}'),
('admingroup_2',1,1784453715,'a:61:{s:8:\"admingid\";s:1:\"2\";s:13:\"alloweditpost\";s:1:\"1\";s:13:\"alloweditpoll\";s:1:\"0\";s:16:\"allowstickthread\";s:1:\"2\";s:12:\"allowmodpost\";s:1:\"1\";s:12:\"allowdelpost\";s:1:\"1\";s:14:\"allowmassprune\";s:1:\"1\";s:11:\"allowrefund\";s:1:\"1\";s:15:\"allowcensorword\";s:1:\"1\";s:11:\"allowviewip\";s:1:\"1\";s:10:\"allowbanip\";s:1:\"1\";s:13:\"allowedituser\";s:1:\"1\";s:12:\"allowmoduser\";s:1:\"1\";s:12:\"allowbanuser\";s:1:\"1\";s:17:\"allowbanvisituser\";s:1:\"1\";s:17:\"allowpostannounce\";s:1:\"1\";s:12:\"allowviewlog\";s:1:\"1\";s:12:\"allowbanpost\";s:1:\"1\";s:20:\"supe_allowpushthread\";s:1:\"0\";s:20:\"allowhighlightthread\";s:1:\"1\";s:15:\"allowlivethread\";s:1:\"1\";s:17:\"allowdigestthread\";s:1:\"3\";s:20:\"allowrecommendthread\";s:1:\"1\";s:15:\"allowbumpthread\";s:1:\"1\";s:16:\"allowclosethread\";s:1:\"1\";s:15:\"allowmovethread\";s:1:\"1\";s:19:\"allowedittypethread\";s:1:\"1\";s:16:\"allowstampthread\";s:1:\"1\";s:14:\"allowstamplist\";s:1:\"1\";s:15:\"allowcopythread\";s:1:\"1\";s:16:\"allowmergethread\";s:1:\"1\";s:16:\"allowsplitthread\";s:1:\"1\";s:17:\"allowrepairthread\";s:1:\"1\";s:13:\"allowwarnpost\";s:1:\"1\";s:15:\"allowviewreport\";s:1:\"1\";s:14:\"alloweditforum\";s:1:\"1\";s:17:\"allowremovereward\";s:1:\"1\";s:14:\"allowedittrade\";s:1:\"0\";s:17:\"alloweditactivity\";s:1:\"0\";s:15:\"allowstickreply\";s:1:\"0\";s:18:\"allowmanagearticle\";s:1:\"0\";s:13:\"allowaddtopic\";s:1:\"0\";s:16:\"allowmanagetopic\";s:1:\"0\";s:8:\"allowdiy\";s:1:\"0\";s:17:\"allowclearrecycle\";s:1:\"0\";s:14:\"allowmanagetag\";s:1:\"0\";s:16:\"alloweditusertag\";s:1:\"0\";s:10:\"managefeed\";s:1:\"0\";s:11:\"managedoing\";s:1:\"0\";s:11:\"manageshare\";s:1:\"0\";s:10:\"manageblog\";s:1:\"0\";s:11:\"managealbum\";s:1:\"0\";s:13:\"managecomment\";s:1:\"0\";s:14:\"managemagiclog\";s:1:\"0\";s:12:\"managereport\";s:1:\"0\";s:13:\"managehotuser\";s:1:\"0\";s:17:\"managedefaultuser\";s:1:\"0\";s:11:\"managemagic\";s:1:\"0\";s:11:\"manageclick\";s:1:\"0\";s:21:\"allowmanagecollection\";s:1:\"1\";s:13:\"allowmakehtml\";s:1:\"0\";}'),
('admingroup_3',1,1784453715,'a:61:{s:8:\"admingid\";s:1:\"3\";s:13:\"alloweditpost\";s:1:\"1\";s:13:\"alloweditpoll\";s:1:\"0\";s:16:\"allowstickthread\";s:1:\"1\";s:12:\"allowmodpost\";s:1:\"1\";s:12:\"allowdelpost\";s:1:\"1\";s:14:\"allowmassprune\";s:1:\"0\";s:11:\"allowrefund\";s:1:\"0\";s:15:\"allowcensorword\";s:1:\"0\";s:11:\"allowviewip\";s:1:\"1\";s:10:\"allowbanip\";s:1:\"0\";s:13:\"allowedituser\";s:1:\"0\";s:12:\"allowmoduser\";s:1:\"1\";s:12:\"allowbanuser\";s:1:\"1\";s:17:\"allowbanvisituser\";s:1:\"0\";s:17:\"allowpostannounce\";s:1:\"0\";s:12:\"allowviewlog\";s:1:\"1\";s:12:\"allowbanpost\";s:1:\"1\";s:20:\"supe_allowpushthread\";s:1:\"0\";s:20:\"allowhighlightthread\";s:1:\"1\";s:15:\"allowlivethread\";s:1:\"1\";s:17:\"allowdigestthread\";s:1:\"3\";s:20:\"allowrecommendthread\";s:1:\"1\";s:15:\"allowbumpthread\";s:1:\"1\";s:16:\"allowclosethread\";s:1:\"1\";s:15:\"allowmovethread\";s:1:\"1\";s:19:\"allowedittypethread\";s:1:\"1\";s:16:\"allowstampthread\";s:1:\"1\";s:14:\"allowstamplist\";s:1:\"1\";s:15:\"allowcopythread\";s:1:\"1\";s:16:\"allowmergethread\";s:1:\"1\";s:16:\"allowsplitthread\";s:1:\"1\";s:17:\"allowrepairthread\";s:1:\"1\";s:13:\"allowwarnpost\";s:1:\"1\";s:15:\"allowviewreport\";s:1:\"1\";s:14:\"alloweditforum\";s:1:\"1\";s:17:\"allowremovereward\";s:1:\"1\";s:14:\"allowedittrade\";s:1:\"0\";s:17:\"alloweditactivity\";s:1:\"0\";s:15:\"allowstickreply\";s:1:\"0\";s:18:\"allowmanagearticle\";s:1:\"0\";s:13:\"allowaddtopic\";s:1:\"0\";s:16:\"allowmanagetopic\";s:1:\"0\";s:8:\"allowdiy\";s:1:\"0\";s:17:\"allowclearrecycle\";s:1:\"0\";s:14:\"allowmanagetag\";s:1:\"0\";s:16:\"alloweditusertag\";s:1:\"0\";s:10:\"managefeed\";s:1:\"0\";s:11:\"managedoing\";s:1:\"0\";s:11:\"manageshare\";s:1:\"0\";s:10:\"manageblog\";s:1:\"0\";s:11:\"managealbum\";s:1:\"0\";s:13:\"managecomment\";s:1:\"0\";s:14:\"managemagiclog\";s:1:\"0\";s:12:\"managereport\";s:1:\"0\";s:13:\"managehotuser\";s:1:\"0\";s:17:\"managedefaultuser\";s:1:\"0\";s:11:\"managemagic\";s:1:\"0\";s:11:\"manageclick\";s:1:\"0\";s:21:\"allowmanagecollection\";s:1:\"0\";s:13:\"allowmakehtml\";s:1:\"0\";}'),
('adminlog',1,1784453715,'a:0:{}'),
('adminmenu',1,1784453715,'a:0:{}'),
('advs',1,1784453715,'a:4:{s:8:\"evalcode\";a:0:{}s:10:\"parameters\";a:0:{}s:4:\"code\";a:0:{}s:6:\"addons\";a:0:{}}'),
('albumcategory',1,1784453715,'a:0:{}'),
('announcements',1,1784453715,'a:0:{}'),
('announcements_forum',1,1784453715,'a:0:{}'),
('antitheft',1,1784453715,'a:1:{i:0;b:0;}'),
('attachtype',1,1784453715,'a:0:{}'),
('bbcodes',1,1784453715,'a:13:{i:1;a:2:{s:11:\"searcharray\";a:1:{i:0;s:23:\"/\\[qq\\](.*?)\\[\\/qq\\]/is\";}s:12:\"replacearray\";a:1:{i:0;s:167:\"<a href=\"https://wpa.qq.com/msgrd?v=3&uin=\\1&amp;site=[Discuz!]&amp;from=discuz&amp;menu=yes\" target=\"_blank\"><img src=\"static/image/common/qq_big.gif\" border=\"0\"></a>\";}}i:2;a:2:{s:11:\"searcharray\";a:1:{i:0;s:23:\"/\\[qq\\](.*?)\\[\\/qq\\]/is\";}s:12:\"replacearray\";a:1:{i:0;s:167:\"<a href=\"https://wpa.qq.com/msgrd?v=3&uin=\\1&amp;site=[Discuz!]&amp;from=discuz&amp;menu=yes\" target=\"_blank\"><img src=\"static/image/common/qq_big.gif\" border=\"0\"></a>\";}}i:3;a:2:{s:11:\"searcharray\";a:1:{i:0;s:23:\"/\\[qq\\](.*?)\\[\\/qq\\]/is\";}s:12:\"replacearray\";a:1:{i:0;s:167:\"<a href=\"https://wpa.qq.com/msgrd?v=3&uin=\\1&amp;site=[Discuz!]&amp;from=discuz&amp;menu=yes\" target=\"_blank\"><img src=\"static/image/common/qq_big.gif\" border=\"0\"></a>\";}}i:10;a:2:{s:11:\"searcharray\";a:1:{i:0;s:23:\"/\\[qq\\](.*?)\\[\\/qq\\]/is\";}s:12:\"replacearray\";a:1:{i:0;s:167:\"<a href=\"https://wpa.qq.com/msgrd?v=3&uin=\\1&amp;site=[Discuz!]&amp;from=discuz&amp;menu=yes\" target=\"_blank\"><img src=\"static/image/common/qq_big.gif\" border=\"0\"></a>\";}}i:11;a:2:{s:11:\"searcharray\";a:1:{i:0;s:23:\"/\\[qq\\](.*?)\\[\\/qq\\]/is\";}s:12:\"replacearray\";a:1:{i:0;s:167:\"<a href=\"https://wpa.qq.com/msgrd?v=3&uin=\\1&amp;site=[Discuz!]&amp;from=discuz&amp;menu=yes\" target=\"_blank\"><img src=\"static/image/common/qq_big.gif\" border=\"0\"></a>\";}}i:12;a:2:{s:11:\"searcharray\";a:1:{i:0;s:23:\"/\\[qq\\](.*?)\\[\\/qq\\]/is\";}s:12:\"replacearray\";a:1:{i:0;s:167:\"<a href=\"https://wpa.qq.com/msgrd?v=3&uin=\\1&amp;site=[Discuz!]&amp;from=discuz&amp;menu=yes\" target=\"_blank\"><img src=\"static/image/common/qq_big.gif\" border=\"0\"></a>\";}}i:13;a:2:{s:11:\"searcharray\";a:1:{i:0;s:23:\"/\\[qq\\](.*?)\\[\\/qq\\]/is\";}s:12:\"replacearray\";a:1:{i:0;s:167:\"<a href=\"https://wpa.qq.com/msgrd?v=3&uin=\\1&amp;site=[Discuz!]&amp;from=discuz&amp;menu=yes\" target=\"_blank\"><img src=\"static/image/common/qq_big.gif\" border=\"0\"></a>\";}}i:14;a:2:{s:11:\"searcharray\";a:1:{i:0;s:23:\"/\\[qq\\](.*?)\\[\\/qq\\]/is\";}s:12:\"replacearray\";a:1:{i:0;s:167:\"<a href=\"https://wpa.qq.com/msgrd?v=3&uin=\\1&amp;site=[Discuz!]&amp;from=discuz&amp;menu=yes\" target=\"_blank\"><img src=\"static/image/common/qq_big.gif\" border=\"0\"></a>\";}}i:15;a:2:{s:11:\"searcharray\";a:1:{i:0;s:23:\"/\\[qq\\](.*?)\\[\\/qq\\]/is\";}s:12:\"replacearray\";a:1:{i:0;s:167:\"<a href=\"https://wpa.qq.com/msgrd?v=3&uin=\\1&amp;site=[Discuz!]&amp;from=discuz&amp;menu=yes\" target=\"_blank\"><img src=\"static/image/common/qq_big.gif\" border=\"0\"></a>\";}}i:16;a:2:{s:11:\"searcharray\";a:1:{i:0;s:23:\"/\\[qq\\](.*?)\\[\\/qq\\]/is\";}s:12:\"replacearray\";a:1:{i:0;s:167:\"<a href=\"https://wpa.qq.com/msgrd?v=3&uin=\\1&amp;site=[Discuz!]&amp;from=discuz&amp;menu=yes\" target=\"_blank\"><img src=\"static/image/common/qq_big.gif\" border=\"0\"></a>\";}}i:17;a:2:{s:11:\"searcharray\";a:1:{i:0;s:23:\"/\\[qq\\](.*?)\\[\\/qq\\]/is\";}s:12:\"replacearray\";a:1:{i:0;s:167:\"<a href=\"https://wpa.qq.com/msgrd?v=3&uin=\\1&amp;site=[Discuz!]&amp;from=discuz&amp;menu=yes\" target=\"_blank\"><img src=\"static/image/common/qq_big.gif\" border=\"0\"></a>\";}}i:18;a:2:{s:11:\"searcharray\";a:1:{i:0;s:23:\"/\\[qq\\](.*?)\\[\\/qq\\]/is\";}s:12:\"replacearray\";a:1:{i:0;s:167:\"<a href=\"https://wpa.qq.com/msgrd?v=3&uin=\\1&amp;site=[Discuz!]&amp;from=discuz&amp;menu=yes\" target=\"_blank\"><img src=\"static/image/common/qq_big.gif\" border=\"0\"></a>\";}}i:19;a:2:{s:11:\"searcharray\";a:1:{i:0;s:23:\"/\\[qq\\](.*?)\\[\\/qq\\]/is\";}s:12:\"replacearray\";a:1:{i:0;s:167:\"<a href=\"https://wpa.qq.com/msgrd?v=3&uin=\\1&amp;site=[Discuz!]&amp;from=discuz&amp;menu=yes\" target=\"_blank\"><img src=\"static/image/common/qq_big.gif\" border=\"0\"></a>\";}}}'),
('bbcodes_display',1,1784453715,'a:13:{i:1;a:1:{s:2:\"qq\";a:12:{s:2:\"id\";s:1:\"2\";s:9:\"available\";s:1:\"2\";s:4:\"icon\";s:29:\"static/image/common/bb_qq.gif\";s:11:\"replacement\";s:168:\"<a href=\"https://wpa.qq.com/msgrd?v=3&uin={1}&amp;site=[Discuz!]&amp;from=discuz&amp;menu=yes\" target=\"_blank\"><img src=\"static/image/common/qq_big.gif\" border=\"0\"></a>\";s:7:\"example\";s:15:\"[qq]688888[/qq]\";s:11:\"explanation\";s:67:\"鏄剧ず QQ 鍦ㄧ嚎鐘舵€侊紝鐐硅繖涓浘鏍囧彲浠ュ拰浠栵紙濂癸級鑱婂ぉ\";s:6:\"params\";s:1:\"1\";s:6:\"prompt\";s:215:\"璇疯緭鍏?QQ 鍙风爜:<a href=\"\" class=\"xi2\" onclick=\"this.href=\\\'https://wp.qq.com/set.html?from=discuz&uin=\\\'+$(\\\'e_cst1_qq_param_1\\\').value\" target=\"_blank\" style=\"float:right;\">璁剧疆QQ鍦ㄧ嚎鐘舵€?nbsp;&nbsp;</a>\";s:4:\"nest\";s:1:\"1\";s:12:\"displayorder\";s:2:\"21\";s:4:\"perm\";a:13:{i:0;s:1:\"1\";i:1;s:1:\"2\";i:2;s:1:\"3\";i:3;s:2:\"10\";i:4;s:2:\"11\";i:5;s:2:\"12\";i:6;s:2:\"13\";i:7;s:2:\"14\";i:8;s:2:\"15\";i:9;s:2:\"16\";i:10;s:2:\"17\";i:11;s:2:\"18\";i:12;s:2:\"19\";}s:1:\"i\";i:1;}}i:2;a:1:{s:2:\"qq\";a:12:{s:2:\"id\";s:1:\"2\";s:9:\"available\";s:1:\"2\";s:4:\"icon\";s:29:\"static/image/common/bb_qq.gif\";s:11:\"replacement\";s:168:\"<a href=\"https://wpa.qq.com/msgrd?v=3&uin={1}&amp;site=[Discuz!]&amp;from=discuz&amp;menu=yes\" target=\"_blank\"><img src=\"static/image/common/qq_big.gif\" border=\"0\"></a>\";s:7:\"example\";s:15:\"[qq]688888[/qq]\";s:11:\"explanation\";s:67:\"鏄剧ず QQ 鍦ㄧ嚎鐘舵€侊紝鐐硅繖涓浘鏍囧彲浠ュ拰浠栵紙濂癸級鑱婂ぉ\";s:6:\"params\";s:1:\"1\";s:6:\"prompt\";s:215:\"璇疯緭鍏?QQ 鍙风爜:<a href=\"\" class=\"xi2\" onclick=\"this.href=\\\'https://wp.qq.com/set.html?from=discuz&uin=\\\'+$(\\\'e_cst1_qq_param_1\\\').value\" target=\"_blank\" style=\"float:right;\">璁剧疆QQ鍦ㄧ嚎鐘舵€?nbsp;&nbsp;</a>\";s:4:\"nest\";s:1:\"1\";s:12:\"displayorder\";s:2:\"21\";s:4:\"perm\";a:13:{i:0;s:1:\"1\";i:1;s:1:\"2\";i:2;s:1:\"3\";i:3;s:2:\"10\";i:4;s:2:\"11\";i:5;s:2:\"12\";i:6;s:2:\"13\";i:7;s:2:\"14\";i:8;s:2:\"15\";i:9;s:2:\"16\";i:10;s:2:\"17\";i:11;s:2:\"18\";i:12;s:2:\"19\";}s:1:\"i\";i:1;}}i:3;a:1:{s:2:\"qq\";a:12:{s:2:\"id\";s:1:\"2\";s:9:\"available\";s:1:\"2\";s:4:\"icon\";s:29:\"static/image/common/bb_qq.gif\";s:11:\"replacement\";s:168:\"<a href=\"https://wpa.qq.com/msgrd?v=3&uin={1}&amp;site=[Discuz!]&amp;from=discuz&amp;menu=yes\" target=\"_blank\"><img src=\"static/image/common/qq_big.gif\" border=\"0\"></a>\";s:7:\"example\";s:15:\"[qq]688888[/qq]\";s:11:\"explanation\";s:67:\"鏄剧ず QQ 鍦ㄧ嚎鐘舵€侊紝鐐硅繖涓浘鏍囧彲浠ュ拰浠栵紙濂癸級鑱婂ぉ\";s:6:\"params\";s:1:\"1\";s:6:\"prompt\";s:215:\"璇疯緭鍏?QQ 鍙风爜:<a href=\"\" class=\"xi2\" onclick=\"this.href=\\\'https://wp.qq.com/set.html?from=discuz&uin=\\\'+$(\\\'e_cst1_qq_param_1\\\').value\" target=\"_blank\" style=\"float:right;\">璁剧疆QQ鍦ㄧ嚎鐘舵€?nbsp;&nbsp;</a>\";s:4:\"nest\";s:1:\"1\";s:12:\"displayorder\";s:2:\"21\";s:4:\"perm\";a:13:{i:0;s:1:\"1\";i:1;s:1:\"2\";i:2;s:1:\"3\";i:3;s:2:\"10\";i:4;s:2:\"11\";i:5;s:2:\"12\";i:6;s:2:\"13\";i:7;s:2:\"14\";i:8;s:2:\"15\";i:9;s:2:\"16\";i:10;s:2:\"17\";i:11;s:2:\"18\";i:12;s:2:\"19\";}s:1:\"i\";i:1;}}i:10;a:1:{s:2:\"qq\";a:12:{s:2:\"id\";s:1:\"2\";s:9:\"available\";s:1:\"2\";s:4:\"icon\";s:29:\"static/image/common/bb_qq.gif\";s:11:\"replacement\";s:168:\"<a href=\"https://wpa.qq.com/msgrd?v=3&uin={1}&amp;site=[Discuz!]&amp;from=discuz&amp;menu=yes\" target=\"_blank\"><img src=\"static/image/common/qq_big.gif\" border=\"0\"></a>\";s:7:\"example\";s:15:\"[qq]688888[/qq]\";s:11:\"explanation\";s:67:\"鏄剧ず QQ 鍦ㄧ嚎鐘舵€侊紝鐐硅繖涓浘鏍囧彲浠ュ拰浠栵紙濂癸級鑱婂ぉ\";s:6:\"params\";s:1:\"1\";s:6:\"prompt\";s:215:\"璇疯緭鍏?QQ 鍙风爜:<a href=\"\" class=\"xi2\" onclick=\"this.href=\\\'https://wp.qq.com/set.html?from=discuz&uin=\\\'+$(\\\'e_cst1_qq_param_1\\\').value\" target=\"_blank\" style=\"float:right;\">璁剧疆QQ鍦ㄧ嚎鐘舵€?nbsp;&nbsp;</a>\";s:4:\"nest\";s:1:\"1\";s:12:\"displayorder\";s:2:\"21\";s:4:\"perm\";a:13:{i:0;s:1:\"1\";i:1;s:1:\"2\";i:2;s:1:\"3\";i:3;s:2:\"10\";i:4;s:2:\"11\";i:5;s:2:\"12\";i:6;s:2:\"13\";i:7;s:2:\"14\";i:8;s:2:\"15\";i:9;s:2:\"16\";i:10;s:2:\"17\";i:11;s:2:\"18\";i:12;s:2:\"19\";}s:1:\"i\";i:1;}}i:11;a:1:{s:2:\"qq\";a:12:{s:2:\"id\";s:1:\"2\";s:9:\"available\";s:1:\"2\";s:4:\"icon\";s:29:\"static/image/common/bb_qq.gif\";s:11:\"replacement\";s:168:\"<a href=\"https://wpa.qq.com/msgrd?v=3&uin={1}&amp;site=[Discuz!]&amp;from=discuz&amp;menu=yes\" target=\"_blank\"><img src=\"static/image/common/qq_big.gif\" border=\"0\"></a>\";s:7:\"example\";s:15:\"[qq]688888[/qq]\";s:11:\"explanation\";s:67:\"鏄剧ず QQ 鍦ㄧ嚎鐘舵€侊紝鐐硅繖涓浘鏍囧彲浠ュ拰浠栵紙濂癸級鑱婂ぉ\";s:6:\"params\";s:1:\"1\";s:6:\"prompt\";s:215:\"璇疯緭鍏?QQ 鍙风爜:<a href=\"\" class=\"xi2\" onclick=\"this.href=\\\'https://wp.qq.com/set.html?from=discuz&uin=\\\'+$(\\\'e_cst1_qq_param_1\\\').value\" target=\"_blank\" style=\"float:right;\">璁剧疆QQ鍦ㄧ嚎鐘舵€?nbsp;&nbsp;</a>\";s:4:\"nest\";s:1:\"1\";s:12:\"displayorder\";s:2:\"21\";s:4:\"perm\";a:13:{i:0;s:1:\"1\";i:1;s:1:\"2\";i:2;s:1:\"3\";i:3;s:2:\"10\";i:4;s:2:\"11\";i:5;s:2:\"12\";i:6;s:2:\"13\";i:7;s:2:\"14\";i:8;s:2:\"15\";i:9;s:2:\"16\";i:10;s:2:\"17\";i:11;s:2:\"18\";i:12;s:2:\"19\";}s:1:\"i\";i:1;}}i:12;a:1:{s:2:\"qq\";a:12:{s:2:\"id\";s:1:\"2\";s:9:\"available\";s:1:\"2\";s:4:\"icon\";s:29:\"static/image/common/bb_qq.gif\";s:11:\"replacement\";s:168:\"<a href=\"https://wpa.qq.com/msgrd?v=3&uin={1}&amp;site=[Discuz!]&amp;from=discuz&amp;menu=yes\" target=\"_blank\"><img src=\"static/image/common/qq_big.gif\" border=\"0\"></a>\";s:7:\"example\";s:15:\"[qq]688888[/qq]\";s:11:\"explanation\";s:67:\"鏄剧ず QQ 鍦ㄧ嚎鐘舵€侊紝鐐硅繖涓浘鏍囧彲浠ュ拰浠栵紙濂癸級鑱婂ぉ\";s:6:\"params\";s:1:\"1\";s:6:\"prompt\";s:215:\"璇疯緭鍏?QQ 鍙风爜:<a href=\"\" class=\"xi2\" onclick=\"this.href=\\\'https://wp.qq.com/set.html?from=discuz&uin=\\\'+$(\\\'e_cst1_qq_param_1\\\').value\" target=\"_blank\" style=\"float:right;\">璁剧疆QQ鍦ㄧ嚎鐘舵€?nbsp;&nbsp;</a>\";s:4:\"nest\";s:1:\"1\";s:12:\"displayorder\";s:2:\"21\";s:4:\"perm\";a:13:{i:0;s:1:\"1\";i:1;s:1:\"2\";i:2;s:1:\"3\";i:3;s:2:\"10\";i:4;s:2:\"11\";i:5;s:2:\"12\";i:6;s:2:\"13\";i:7;s:2:\"14\";i:8;s:2:\"15\";i:9;s:2:\"16\";i:10;s:2:\"17\";i:11;s:2:\"18\";i:12;s:2:\"19\";}s:1:\"i\";i:1;}}i:13;a:1:{s:2:\"qq\";a:12:{s:2:\"id\";s:1:\"2\";s:9:\"available\";s:1:\"2\";s:4:\"icon\";s:29:\"static/image/common/bb_qq.gif\";s:11:\"replacement\";s:168:\"<a href=\"https://wpa.qq.com/msgrd?v=3&uin={1}&amp;site=[Discuz!]&amp;from=discuz&amp;menu=yes\" target=\"_blank\"><img src=\"static/image/common/qq_big.gif\" border=\"0\"></a>\";s:7:\"example\";s:15:\"[qq]688888[/qq]\";s:11:\"explanation\";s:67:\"鏄剧ず QQ 鍦ㄧ嚎鐘舵€侊紝鐐硅繖涓浘鏍囧彲浠ュ拰浠栵紙濂癸級鑱婂ぉ\";s:6:\"params\";s:1:\"1\";s:6:\"prompt\";s:215:\"璇疯緭鍏?QQ 鍙风爜:<a href=\"\" class=\"xi2\" onclick=\"this.href=\\\'https://wp.qq.com/set.html?from=discuz&uin=\\\'+$(\\\'e_cst1_qq_param_1\\\').value\" target=\"_blank\" style=\"float:right;\">璁剧疆QQ鍦ㄧ嚎鐘舵€?nbsp;&nbsp;</a>\";s:4:\"nest\";s:1:\"1\";s:12:\"displayorder\";s:2:\"21\";s:4:\"perm\";a:13:{i:0;s:1:\"1\";i:1;s:1:\"2\";i:2;s:1:\"3\";i:3;s:2:\"10\";i:4;s:2:\"11\";i:5;s:2:\"12\";i:6;s:2:\"13\";i:7;s:2:\"14\";i:8;s:2:\"15\";i:9;s:2:\"16\";i:10;s:2:\"17\";i:11;s:2:\"18\";i:12;s:2:\"19\";}s:1:\"i\";i:1;}}i:14;a:1:{s:2:\"qq\";a:12:{s:2:\"id\";s:1:\"2\";s:9:\"available\";s:1:\"2\";s:4:\"icon\";s:29:\"static/image/common/bb_qq.gif\";s:11:\"replacement\";s:168:\"<a href=\"https://wpa.qq.com/msgrd?v=3&uin={1}&amp;site=[Discuz!]&amp;from=discuz&amp;menu=yes\" target=\"_blank\"><img src=\"static/image/common/qq_big.gif\" border=\"0\"></a>\";s:7:\"example\";s:15:\"[qq]688888[/qq]\";s:11:\"explanation\";s:67:\"鏄剧ず QQ 鍦ㄧ嚎鐘舵€侊紝鐐硅繖涓浘鏍囧彲浠ュ拰浠栵紙濂癸級鑱婂ぉ\";s:6:\"params\";s:1:\"1\";s:6:\"prompt\";s:215:\"璇疯緭鍏?QQ 鍙风爜:<a href=\"\" class=\"xi2\" onclick=\"this.href=\\\'https://wp.qq.com/set.html?from=discuz&uin=\\\'+$(\\\'e_cst1_qq_param_1\\\').value\" target=\"_blank\" style=\"float:right;\">璁剧疆QQ鍦ㄧ嚎鐘舵€?nbsp;&nbsp;</a>\";s:4:\"nest\";s:1:\"1\";s:12:\"displayorder\";s:2:\"21\";s:4:\"perm\";a:13:{i:0;s:1:\"1\";i:1;s:1:\"2\";i:2;s:1:\"3\";i:3;s:2:\"10\";i:4;s:2:\"11\";i:5;s:2:\"12\";i:6;s:2:\"13\";i:7;s:2:\"14\";i:8;s:2:\"15\";i:9;s:2:\"16\";i:10;s:2:\"17\";i:11;s:2:\"18\";i:12;s:2:\"19\";}s:1:\"i\";i:1;}}i:15;a:1:{s:2:\"qq\";a:12:{s:2:\"id\";s:1:\"2\";s:9:\"available\";s:1:\"2\";s:4:\"icon\";s:29:\"static/image/common/bb_qq.gif\";s:11:\"replacement\";s:168:\"<a href=\"https://wpa.qq.com/msgrd?v=3&uin={1}&amp;site=[Discuz!]&amp;from=discuz&amp;menu=yes\" target=\"_blank\"><img src=\"static/image/common/qq_big.gif\" border=\"0\"></a>\";s:7:\"example\";s:15:\"[qq]688888[/qq]\";s:11:\"explanation\";s:67:\"鏄剧ず QQ 鍦ㄧ嚎鐘舵€侊紝鐐硅繖涓浘鏍囧彲浠ュ拰浠栵紙濂癸級鑱婂ぉ\";s:6:\"params\";s:1:\"1\";s:6:\"prompt\";s:215:\"璇疯緭鍏?QQ 鍙风爜:<a href=\"\" class=\"xi2\" onclick=\"this.href=\\\'https://wp.qq.com/set.html?from=discuz&uin=\\\'+$(\\\'e_cst1_qq_param_1\\\').value\" target=\"_blank\" style=\"float:right;\">璁剧疆QQ鍦ㄧ嚎鐘舵€?nbsp;&nbsp;</a>\";s:4:\"nest\";s:1:\"1\";s:12:\"displayorder\";s:2:\"21\";s:4:\"perm\";a:13:{i:0;s:1:\"1\";i:1;s:1:\"2\";i:2;s:1:\"3\";i:3;s:2:\"10\";i:4;s:2:\"11\";i:5;s:2:\"12\";i:6;s:2:\"13\";i:7;s:2:\"14\";i:8;s:2:\"15\";i:9;s:2:\"16\";i:10;s:2:\"17\";i:11;s:2:\"18\";i:12;s:2:\"19\";}s:1:\"i\";i:1;}}i:16;a:1:{s:2:\"qq\";a:12:{s:2:\"id\";s:1:\"2\";s:9:\"available\";s:1:\"2\";s:4:\"icon\";s:29:\"static/image/common/bb_qq.gif\";s:11:\"replacement\";s:168:\"<a href=\"https://wpa.qq.com/msgrd?v=3&uin={1}&amp;site=[Discuz!]&amp;from=discuz&amp;menu=yes\" target=\"_blank\"><img src=\"static/image/common/qq_big.gif\" border=\"0\"></a>\";s:7:\"example\";s:15:\"[qq]688888[/qq]\";s:11:\"explanation\";s:67:\"鏄剧ず QQ 鍦ㄧ嚎鐘舵€侊紝鐐硅繖涓浘鏍囧彲浠ュ拰浠栵紙濂癸級鑱婂ぉ\";s:6:\"params\";s:1:\"1\";s:6:\"prompt\";s:215:\"璇疯緭鍏?QQ 鍙风爜:<a href=\"\" class=\"xi2\" onclick=\"this.href=\\\'https://wp.qq.com/set.html?from=discuz&uin=\\\'+$(\\\'e_cst1_qq_param_1\\\').value\" target=\"_blank\" style=\"float:right;\">璁剧疆QQ鍦ㄧ嚎鐘舵€?nbsp;&nbsp;</a>\";s:4:\"nest\";s:1:\"1\";s:12:\"displayorder\";s:2:\"21\";s:4:\"perm\";a:13:{i:0;s:1:\"1\";i:1;s:1:\"2\";i:2;s:1:\"3\";i:3;s:2:\"10\";i:4;s:2:\"11\";i:5;s:2:\"12\";i:6;s:2:\"13\";i:7;s:2:\"14\";i:8;s:2:\"15\";i:9;s:2:\"16\";i:10;s:2:\"17\";i:11;s:2:\"18\";i:12;s:2:\"19\";}s:1:\"i\";i:1;}}i:17;a:1:{s:2:\"qq\";a:12:{s:2:\"id\";s:1:\"2\";s:9:\"available\";s:1:\"2\";s:4:\"icon\";s:29:\"static/image/common/bb_qq.gif\";s:11:\"replacement\";s:168:\"<a href=\"https://wpa.qq.com/msgrd?v=3&uin={1}&amp;site=[Discuz!]&amp;from=discuz&amp;menu=yes\" target=\"_blank\"><img src=\"static/image/common/qq_big.gif\" border=\"0\"></a>\";s:7:\"example\";s:15:\"[qq]688888[/qq]\";s:11:\"explanation\";s:67:\"鏄剧ず QQ 鍦ㄧ嚎鐘舵€侊紝鐐硅繖涓浘鏍囧彲浠ュ拰浠栵紙濂癸級鑱婂ぉ\";s:6:\"params\";s:1:\"1\";s:6:\"prompt\";s:215:\"璇疯緭鍏?QQ 鍙风爜:<a href=\"\" class=\"xi2\" onclick=\"this.href=\\\'https://wp.qq.com/set.html?from=discuz&uin=\\\'+$(\\\'e_cst1_qq_param_1\\\').value\" target=\"_blank\" style=\"float:right;\">璁剧疆QQ鍦ㄧ嚎鐘舵€?nbsp;&nbsp;</a>\";s:4:\"nest\";s:1:\"1\";s:12:\"displayorder\";s:2:\"21\";s:4:\"perm\";a:13:{i:0;s:1:\"1\";i:1;s:1:\"2\";i:2;s:1:\"3\";i:3;s:2:\"10\";i:4;s:2:\"11\";i:5;s:2:\"12\";i:6;s:2:\"13\";i:7;s:2:\"14\";i:8;s:2:\"15\";i:9;s:2:\"16\";i:10;s:2:\"17\";i:11;s:2:\"18\";i:12;s:2:\"19\";}s:1:\"i\";i:1;}}i:18;a:1:{s:2:\"qq\";a:12:{s:2:\"id\";s:1:\"2\";s:9:\"available\";s:1:\"2\";s:4:\"icon\";s:29:\"static/image/common/bb_qq.gif\";s:11:\"replacement\";s:168:\"<a href=\"https://wpa.qq.com/msgrd?v=3&uin={1}&amp;site=[Discuz!]&amp;from=discuz&amp;menu=yes\" target=\"_blank\"><img src=\"static/image/common/qq_big.gif\" border=\"0\"></a>\";s:7:\"example\";s:15:\"[qq]688888[/qq]\";s:11:\"explanation\";s:67:\"鏄剧ず QQ 鍦ㄧ嚎鐘舵€侊紝鐐硅繖涓浘鏍囧彲浠ュ拰浠栵紙濂癸級鑱婂ぉ\";s:6:\"params\";s:1:\"1\";s:6:\"prompt\";s:215:\"璇疯緭鍏?QQ 鍙风爜:<a href=\"\" class=\"xi2\" onclick=\"this.href=\\\'https://wp.qq.com/set.html?from=discuz&uin=\\\'+$(\\\'e_cst1_qq_param_1\\\').value\" target=\"_blank\" style=\"float:right;\">璁剧疆QQ鍦ㄧ嚎鐘舵€?nbsp;&nbsp;</a>\";s:4:\"nest\";s:1:\"1\";s:12:\"displayorder\";s:2:\"21\";s:4:\"perm\";a:13:{i:0;s:1:\"1\";i:1;s:1:\"2\";i:2;s:1:\"3\";i:3;s:2:\"10\";i:4;s:2:\"11\";i:5;s:2:\"12\";i:6;s:2:\"13\";i:7;s:2:\"14\";i:8;s:2:\"15\";i:9;s:2:\"16\";i:10;s:2:\"17\";i:11;s:2:\"18\";i:12;s:2:\"19\";}s:1:\"i\";i:1;}}i:19;a:1:{s:2:\"qq\";a:12:{s:2:\"id\";s:1:\"2\";s:9:\"available\";s:1:\"2\";s:4:\"icon\";s:29:\"static/image/common/bb_qq.gif\";s:11:\"replacement\";s:168:\"<a href=\"https://wpa.qq.com/msgrd?v=3&uin={1}&amp;site=[Discuz!]&amp;from=discuz&amp;menu=yes\" target=\"_blank\"><img src=\"static/image/common/qq_big.gif\" border=\"0\"></a>\";s:7:\"example\";s:15:\"[qq]688888[/qq]\";s:11:\"explanation\";s:67:\"鏄剧ず QQ 鍦ㄧ嚎鐘舵€侊紝鐐硅繖涓浘鏍囧彲浠ュ拰浠栵紙濂癸級鑱婂ぉ\";s:6:\"params\";s:1:\"1\";s:6:\"prompt\";s:215:\"璇疯緭鍏?QQ 鍙风爜:<a href=\"\" class=\"xi2\" onclick=\"this.href=\\\'https://wp.qq.com/set.html?from=discuz&uin=\\\'+$(\\\'e_cst1_qq_param_1\\\').value\" target=\"_blank\" style=\"float:right;\">璁剧疆QQ鍦ㄧ嚎鐘舵€?nbsp;&nbsp;</a>\";s:4:\"nest\";s:1:\"1\";s:12:\"displayorder\";s:2:\"21\";s:4:\"perm\";a:13:{i:0;s:1:\"1\";i:1;s:1:\"2\";i:2;s:1:\"3\";i:3;s:2:\"10\";i:4;s:2:\"11\";i:5;s:2:\"12\";i:6;s:2:\"13\";i:7;s:2:\"14\";i:8;s:2:\"15\";i:9;s:2:\"16\";i:10;s:2:\"17\";i:11;s:2:\"18\";i:12;s:2:\"19\";}s:1:\"i\";i:1;}}}'),
('blockclass',1,1784453715,'a:7:{s:5:\"forum\";a:2:{s:4:\"name\";s:9:\"璁哄潧绫籠";s:4:\"subs\";a:4:{s:12:\"forum_thread\";a:3:{s:4:\"name\";s:12:\"甯栧瓙妯″潡\";s:6:\"fields\";a:29:{s:2:\"id\";a:3:{s:4:\"name\";s:8:\"鏁版嵁ID\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:3:\"int\";}s:3:\"url\";a:3:{s:4:\"name\";s:9:\"甯栧瓙URL\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:5:\"title\";a:3:{s:4:\"name\";s:12:\"甯栧瓙鏍囬\";s:8:\"formtype\";s:5:\"title\";s:8:\"datatype\";s:5:\"title\";}s:3:\"pic\";a:3:{s:4:\"name\";s:12:\"闄勪欢鍥剧墖\";s:8:\"formtype\";s:3:\"pic\";s:8:\"datatype\";s:3:\"pic\";}s:7:\"summary\";a:3:{s:4:\"name\";s:12:\"甯栧瓙鍐呭\";s:8:\"formtype\";s:7:\"summary\";s:8:\"datatype\";s:7:\"summary\";}s:6:\"author\";a:3:{s:4:\"name\";s:6:\"妤间富\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:8:\"authorid\";a:3:{s:4:\"name\";s:9:\"妤间富UID\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:3:\"int\";}s:6:\"avatar\";a:3:{s:4:\"name\";s:12:\"妤间富澶村儚\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:13:\"avatar_middle\";a:3:{s:4:\"name\";s:17:\"妤间富澶村儚(涓?\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:10:\"avatar_big\";a:3:{s:4:\"name\";s:17:\"妤间富澶村儚(澶?\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:9:\"avatarimg\";a:3:{s:4:\"name\";s:12:\"妤间富澶村儚\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:16:\"avatarimg_middle\";a:3:{s:4:\"name\";s:17:\"妤间富澶村儚(涓?\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:13:\"avatarimg_big\";a:3:{s:4:\"name\";s:17:\"妤间富澶村儚(澶?\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:8:\"forumurl\";a:3:{s:4:\"name\";s:9:\"鐗堝潡URL\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:9:\"forumname\";a:3:{s:4:\"name\";s:12:\"鐗堝潡鍚嶇О\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:8:\"typename\";a:3:{s:4:\"name\";s:18:\"涓婚鍒嗙被鍚嶇О\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:8:\"typeicon\";a:3:{s:4:\"name\";s:18:\"涓婚鍒嗙被鍥炬爣\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:7:\"typeurl\";a:3:{s:4:\"name\";s:15:\"涓婚鍒嗙被URL\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:8:\"sortname\";a:3:{s:4:\"name\";s:18:\"鍒嗙被淇℃伅鍚嶇О\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:7:\"sorturl\";a:3:{s:4:\"name\";s:15:\"鍒嗙被淇℃伅URL\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:5:\"posts\";a:3:{s:4:\"name\";s:12:\"鎬诲彂甯栨暟\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:3:\"int\";}s:10:\"todayposts\";a:3:{s:4:\"name\";s:15:\"浠婃棩鍙戝笘鏁癨";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:3:\"int\";}s:10:\"lastposter\";a:3:{s:4:\"name\";s:18:\"鏈€鍚庡洖澶嶄綔鑰匼";s:8:\"formtype\";s:6:\"string\";s:8:\"datatype\";s:6:\"string\";}s:8:\"lastpost\";a:3:{s:4:\"name\";s:18:\"鏈€鍚庡洖澶嶆椂闂碶";s:8:\"formtype\";s:4:\"date\";s:8:\"datatype\";s:4:\"date\";}s:8:\"dateline\";a:3:{s:4:\"name\";s:12:\"鍙戝笘鏃堕棿\";s:8:\"formtype\";s:4:\"date\";s:8:\"datatype\";s:4:\"date\";}s:7:\"replies\";a:3:{s:4:\"name\";s:9:\"鍥炲鏁癨";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:3:\"int\";}s:5:\"views\";a:3:{s:4:\"name\";s:12:\"鎬绘祻瑙堟暟\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:3:\"int\";}s:5:\"heats\";a:3:{s:4:\"name\";s:9:\"鐑害鍊糪";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:3:\"int\";}s:10:\"recommends\";a:3:{s:4:\"name\";s:9:\"鎺ㄨ崘鏁癨";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:3:\"int\";}}s:6:\"script\";a:7:{s:6:\"thread\";s:15:\"楂樼骇鑷畾涔塡";s:9:\"threadnew\";s:9:\"鏈€鏂板笘\";s:13:\"threadspecial\";s:15:\"鐗规畩涓婚甯朶";s:12:\"threaddigest\";s:9:\"绮惧崕甯朶";s:9:\"threadhot\";s:9:\"鐑棬甯朶";s:15:\"threadspecified\";s:12:\"鎸囧畾甯栧瓙\";s:11:\"threadstick\";s:9:\"缃《甯朶";}}s:11:\"forum_trade\";a:3:{s:4:\"name\";s:12:\"鍟嗗搧妯″潡\";s:6:\"fields\";a:9:{s:2:\"id\";a:3:{s:4:\"name\";s:8:\"鏁版嵁ID\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:3:\"int\";}s:3:\"url\";a:3:{s:4:\"name\";s:12:\"鍟嗗搧閾炬帴\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:5:\"title\";a:3:{s:4:\"name\";s:12:\"鍟嗗搧鍚嶇О\";s:8:\"formtype\";s:5:\"title\";s:8:\"datatype\";s:5:\"title\";}s:3:\"pic\";a:3:{s:4:\"name\";s:18:\"鍟嗗搧鍥剧墖鍦板潃\";s:8:\"formtype\";s:3:\"pic\";s:8:\"datatype\";s:3:\"pic\";}s:7:\"summary\";a:3:{s:4:\"name\";s:12:\"鍟嗗搧璇存槑\";s:8:\"formtype\";s:7:\"summary\";s:8:\"datatype\";s:7:\"summary\";}s:10:\"totalitems\";a:3:{s:4:\"name\";s:21:\"鍟嗗搧绱鍞嚭鏁癨";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:3:\"int\";}s:6:\"author\";a:3:{s:4:\"name\";s:12:\"鍟嗗搧鍗栧\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:4:\"text\";}s:8:\"authorid\";a:3:{s:4:\"name\";s:15:\"鍟嗗搧鍗栧UID\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:3:\"int\";}s:5:\"price\";a:3:{s:4:\"name\";s:12:\"鍟嗗搧浠锋牸\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:4:\"text\";}}s:6:\"script\";a:4:{s:8:\"tradenew\";s:9:\"鏂板晢鍝乗";s:14:\"tradespecified\";s:12:\"鎸囧畾鍟嗗搧\";s:8:\"tradehot\";s:12:\"鐑棬鍟嗗搧\";s:5:\"trade\";s:15:\"楂樼骇鑷畾涔塡";}}s:14:\"forum_activity\";a:3:{s:4:\"name\";s:12:\"娲诲姩妯″潡\";s:6:\"fields\";a:15:{s:2:\"id\";a:3:{s:4:\"name\";s:8:\"鏁版嵁ID\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:3:\"int\";}s:3:\"url\";a:3:{s:4:\"name\";s:12:\"娲诲姩甯朥RL\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:5:\"title\";a:3:{s:4:\"name\";s:12:\"娲诲姩鏍囬\";s:8:\"formtype\";s:5:\"title\";s:8:\"datatype\";s:5:\"title\";}s:3:\"pic\";a:3:{s:4:\"name\";s:12:\"涓婚鍥剧墖\";s:8:\"formtype\";s:3:\"pic\";s:8:\"datatype\";s:3:\"pic\";}s:7:\"summary\";a:3:{s:4:\"name\";s:12:\"娲诲姩浠嬬粛\";s:8:\"formtype\";s:7:\"summary\";s:8:\"datatype\";s:7:\"summary\";}s:4:\"time\";a:3:{s:4:\"name\";s:12:\"娲诲姩鏃堕棿\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:4:\"text\";}s:10:\"expiration\";a:3:{s:4:\"name\";s:18:\"鎶ュ悕鎴鏃堕棿\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:4:\"text\";}s:6:\"author\";a:3:{s:4:\"name\";s:9:\"鍙戣捣浜篭";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:4:\"text\";}s:8:\"authorid\";a:3:{s:4:\"name\";s:12:\"鍙戣捣浜篣ID\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:3:\"int\";}s:4:\"cost\";a:3:{s:4:\"name\";s:12:\"姣忎汉鑺遍攢\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:3:\"int\";}s:5:\"place\";a:3:{s:4:\"name\";s:12:\"娲诲姩鍦扮偣\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:4:\"text\";}s:5:\"class\";a:3:{s:4:\"name\";s:12:\"娲诲姩绫诲瀷\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:4:\"text\";}s:6:\"gender\";a:3:{s:4:\"name\";s:12:\"鎬у埆瑕佹眰\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:4:\"text\";}s:6:\"number\";a:3:{s:4:\"name\";s:12:\"闇€瑕佷汉鏁癨";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:3:\"int\";}s:11:\"applynumber\";a:3:{s:4:\"name\";s:15:\"宸叉姤鍚嶄汉鏁癨";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:3:\"int\";}}s:6:\"script\";a:3:{s:8:\"activity\";s:15:\"楂樼骇鑷畾涔塡";s:11:\"activitynew\";s:12:\"鏈€鏂版椿鍔╘";s:12:\"activitycity\";s:12:\"鍚屽煄娲诲姩\";}}s:11:\"forum_forum\";a:3:{s:4:\"name\";s:12:\"鐗堝潡妯″潡\";s:6:\"fields\";a:8:{s:2:\"id\";a:3:{s:4:\"name\";s:8:\"鏁版嵁ID\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:3:\"int\";}s:3:\"url\";a:3:{s:4:\"name\";s:12:\"鐗堝潡閾炬帴\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:5:\"title\";a:3:{s:4:\"name\";s:12:\"鐗堝潡鍚嶇О\";s:8:\"formtype\";s:5:\"title\";s:8:\"datatype\";s:5:\"title\";}s:7:\"summary\";a:3:{s:4:\"name\";s:12:\"鐗堝潡浠嬬粛\";s:8:\"formtype\";s:7:\"summary\";s:8:\"datatype\";s:7:\"summary\";}s:4:\"icon\";a:3:{s:4:\"name\";s:12:\"鐗堝潡鍥炬爣\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:5:\"posts\";a:3:{s:4:\"name\";s:15:\"鐗堝潡甯栧瓙鏁癨";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:3:\"int\";}s:7:\"threads\";a:3:{s:4:\"name\";s:15:\"鐗堝潡璇濋鏁癨";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:3:\"int\";}s:10:\"todayposts\";a:3:{s:4:\"name\";s:21:\"鐗堝潡浠婃棩鏂板笘鏁癨";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:3:\"int\";}}s:6:\"script\";a:1:{s:5:\"forum\";s:12:\"璁哄潧鐗堝潡\";}}}}s:5:\"group\";a:2:{s:4:\"name\";s:9:\"鍦堝瓙绫籠";s:4:\"subs\";a:4:{s:11:\"group_group\";a:3:{s:4:\"name\";s:12:\"鍦堝瓙妯″潡\";s:6:\"fields\";a:16:{s:2:\"id\";a:3:{s:4:\"name\";s:8:\"鏁版嵁ID\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:3:\"int\";}s:3:\"url\";a:3:{s:4:\"name\";s:12:\"鍦堝瓙閾炬帴\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:5:\"title\";a:3:{s:4:\"name\";s:12:\"鍦堝瓙鍚嶇О\";s:8:\"formtype\";s:5:\"title\";s:8:\"datatype\";s:5:\"title\";}s:3:\"pic\";a:3:{s:4:\"name\";s:12:\"鍦堝瓙鍥剧墖\";s:8:\"formtype\";s:3:\"pic\";s:8:\"datatype\";s:3:\"pic\";}s:7:\"summary\";a:3:{s:4:\"name\";s:12:\"鍦堝瓙浠嬬粛\";s:8:\"formtype\";s:7:\"summary\";s:8:\"datatype\";s:7:\"summary\";}s:4:\"icon\";a:3:{s:4:\"name\";s:12:\"鍦堝瓙鍥炬爣\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:11:\"foundername\";a:3:{s:4:\"name\";s:9:\"鍒涘浜篭";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:10:\"founderuid\";a:3:{s:4:\"name\";s:12:\"鍒涘浜篣ID\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:3:\"int\";}s:5:\"posts\";a:3:{s:4:\"name\";s:12:\"鎬诲彂甯栨暟\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:3:\"int\";}s:10:\"todayposts\";a:3:{s:4:\"name\";s:15:\"浠婃棩鍙戝笘鏁癨";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:3:\"int\";}s:7:\"threads\";a:3:{s:4:\"name\";s:12:\"鎬昏瘽棰樻暟\";s:8:\"formtype\";s:4:\"date\";s:8:\"datatype\";s:3:\"int\";}s:9:\"membernum\";a:3:{s:4:\"name\";s:9:\"鎴愬憳鏁癨";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:3:\"int\";}s:8:\"dateline\";a:3:{s:4:\"name\";s:12:\"鍒涘缓鏃堕棿\";s:8:\"formtype\";s:4:\"date\";s:8:\"datatype\";s:4:\"date\";}s:5:\"level\";a:3:{s:4:\"name\";s:12:\"鍦堝瓙绛夌骇\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:3:\"int\";}s:13:\"commoncredits\";a:3:{s:4:\"name\";s:18:\"鍦堝瓙鍏叡绉垎\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:3:\"int\";}s:8:\"activity\";a:3:{s:4:\"name\";s:15:\"鍦堝瓙娲昏穬搴";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:3:\"int\";}}s:6:\"script\";a:4:{s:14:\"groupspecified\";s:12:\"鎸囧畾鍦堝瓙\";s:8:\"grouphot\";s:12:\"鐑棬鍦堝瓙\";s:8:\"groupnew\";s:12:\"鏈€鏂板湀瀛怽";s:5:\"group\";s:15:\"楂樼骇鑷畾涔塡";}}s:14:\"group_activity\";a:3:{s:4:\"name\";s:12:\"鍦堝瓙娲诲姩\";s:6:\"fields\";a:15:{s:2:\"id\";a:3:{s:4:\"name\";s:8:\"鏁版嵁ID\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:3:\"int\";}s:3:\"url\";a:3:{s:4:\"name\";s:12:\"娲诲姩甯朥RL\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:5:\"title\";a:3:{s:4:\"name\";s:12:\"娲诲姩鏍囬\";s:8:\"formtype\";s:5:\"title\";s:8:\"datatype\";s:5:\"title\";}s:3:\"pic\";a:3:{s:4:\"name\";s:12:\"涓婚鍥剧墖\";s:8:\"formtype\";s:3:\"pic\";s:8:\"datatype\";s:3:\"pic\";}s:7:\"summary\";a:3:{s:4:\"name\";s:12:\"娲诲姩浠嬬粛\";s:8:\"formtype\";s:7:\"summary\";s:8:\"datatype\";s:7:\"summary\";}s:4:\"time\";a:3:{s:4:\"name\";s:12:\"娲诲姩鏃堕棿\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:4:\"text\";}s:10:\"expiration\";a:3:{s:4:\"name\";s:18:\"鎶ュ悕鎴鏃堕棿\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:4:\"text\";}s:6:\"author\";a:3:{s:4:\"name\";s:9:\"鍙戣捣浜篭";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:4:\"text\";}s:8:\"authorid\";a:3:{s:4:\"name\";s:12:\"鍙戣捣浜篣ID\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:3:\"int\";}s:4:\"cost\";a:3:{s:4:\"name\";s:12:\"姣忎汉鑺遍攢\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:3:\"int\";}s:5:\"place\";a:3:{s:4:\"name\";s:12:\"娲诲姩鍦扮偣\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:4:\"text\";}s:5:\"class\";a:3:{s:4:\"name\";s:12:\"娲诲姩绫诲瀷\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:4:\"text\";}s:6:\"gender\";a:3:{s:4:\"name\";s:12:\"鎬у埆瑕佹眰\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:4:\"text\";}s:6:\"number\";a:3:{s:4:\"name\";s:12:\"闇€瑕佷汉鏁癨";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:3:\"int\";}s:11:\"applynumber\";a:3:{s:4:\"name\";s:15:\"宸叉姤鍚嶄汉鏁癨";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:3:\"int\";}}s:6:\"script\";a:3:{s:13:\"groupactivity\";s:12:\"鍦堝瓙娲诲姩\";s:16:\"groupactivitynew\";s:12:\"鏈€鏂版椿鍔╘";s:17:\"groupactivitycity\";s:12:\"鍚屽煄娲诲姩\";}}s:11:\"group_trade\";a:3:{s:4:\"name\";s:12:\"鍦堝瓙鍟嗗搧\";s:6:\"fields\";a:9:{s:2:\"id\";a:3:{s:4:\"name\";s:8:\"鏁版嵁ID\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:3:\"int\";}s:3:\"url\";a:3:{s:4:\"name\";s:12:\"鍟嗗搧閾炬帴\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:5:\"title\";a:3:{s:4:\"name\";s:12:\"鍟嗗搧鍚嶇О\";s:8:\"formtype\";s:5:\"title\";s:8:\"datatype\";s:5:\"title\";}s:3:\"pic\";a:3:{s:4:\"name\";s:18:\"鍟嗗搧鍥剧墖鍦板潃\";s:8:\"formtype\";s:3:\"pic\";s:8:\"datatype\";s:3:\"pic\";}s:7:\"summary\";a:3:{s:4:\"name\";s:12:\"鍟嗗搧璇存槑\";s:8:\"formtype\";s:7:\"summary\";s:8:\"datatype\";s:7:\"summary\";}s:10:\"totalitems\";a:3:{s:4:\"name\";s:21:\"鍟嗗搧绱鍞嚭鏁癨";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:3:\"int\";}s:6:\"author\";a:3:{s:4:\"name\";s:12:\"鍟嗗搧鍗栧\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:4:\"text\";}s:8:\"authorid\";a:3:{s:4:\"name\";s:15:\"鍟嗗搧鍗栧UID\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:3:\"int\";}s:5:\"price\";a:3:{s:4:\"name\";s:12:\"鍟嗗搧浠锋牸\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:4:\"text\";}}s:6:\"script\";a:4:{s:19:\"grouptradespecified\";s:12:\"鎸囧畾鍟嗗搧\";s:13:\"grouptradenew\";s:9:\"鏂板晢鍝乗";s:13:\"grouptradehot\";s:12:\"鐑棬鍟嗗搧\";s:10:\"grouptrade\";s:15:\"楂樼骇鑷畾涔塡";}}s:12:\"group_thread\";a:3:{s:4:\"name\";s:12:\"鍦堝瓙甯栧瓙\";s:6:\"fields\";a:23:{s:2:\"id\";a:3:{s:4:\"name\";s:8:\"鏁版嵁ID\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:3:\"int\";}s:3:\"url\";a:3:{s:4:\"name\";s:12:\"甯栧瓙閾炬帴\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:5:\"title\";a:3:{s:4:\"name\";s:12:\"甯栧瓙鏍囬\";s:8:\"formtype\";s:5:\"title\";s:8:\"datatype\";s:5:\"title\";}s:3:\"pic\";a:3:{s:4:\"name\";s:12:\"闄勪欢鍥剧墖\";s:8:\"formtype\";s:3:\"pic\";s:8:\"datatype\";s:3:\"pic\";}s:7:\"summary\";a:3:{s:4:\"name\";s:12:\"甯栧瓙鍐呭\";s:8:\"formtype\";s:7:\"summary\";s:8:\"datatype\";s:7:\"summary\";}s:6:\"author\";a:3:{s:4:\"name\";s:6:\"妤间富\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:8:\"authorid\";a:3:{s:4:\"name\";s:9:\"妤间富UID\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:3:\"int\";}s:6:\"avatar\";a:3:{s:4:\"name\";s:12:\"妤间富澶村儚\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:13:\"avatar_middle\";a:3:{s:4:\"name\";s:17:\"妤间富澶村儚(涓?\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:10:\"avatar_big\";a:3:{s:4:\"name\";s:17:\"妤间富澶村儚(澶?\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:9:\"avatarimg\";a:3:{s:4:\"name\";s:12:\"妤间富澶村儚\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:16:\"avatarimg_middle\";a:3:{s:4:\"name\";s:17:\"妤间富澶村儚(涓?\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:13:\"avatarimg_big\";a:3:{s:4:\"name\";s:17:\"妤间富澶村儚(澶?\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:5:\"posts\";a:3:{s:4:\"name\";s:18:\"涓婚甯栧瓙鎬绘暟\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:3:\"int\";}s:10:\"todayposts\";a:3:{s:4:\"name\";s:21:\"涓婚浠婃棩甯栧瓙鏁癨";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:3:\"int\";}s:8:\"lastpost\";a:3:{s:4:\"name\";s:24:\"涓婚鏈€鍚庡彂甯栨椂闂碶";s:8:\"formtype\";s:4:\"date\";s:8:\"datatype\";s:4:\"date\";}s:8:\"dateline\";a:3:{s:4:\"name\";s:18:\"涓婚鍙戝竷鏃堕棿\";s:8:\"formtype\";s:4:\"date\";s:8:\"datatype\";s:4:\"date\";}s:7:\"replies\";a:3:{s:4:\"name\";s:15:\"涓婚鍥炲鏁癨";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:3:\"int\";}s:5:\"views\";a:3:{s:4:\"name\";s:15:\"涓婚鏌ョ湅鏁癨";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:3:\"int\";}s:5:\"heats\";a:3:{s:4:\"name\";s:12:\"涓婚鐑害\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:3:\"int\";}s:10:\"recommends\";a:3:{s:4:\"name\";s:15:\"涓婚鎺ㄨ崘鏁癨";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:3:\"int\";}s:9:\"groupname\";a:3:{s:4:\"name\";s:12:\"鍦堝瓙鍚嶇О\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:8:\"groupurl\";a:3:{s:4:\"name\";s:12:\"鍦堝瓙閾炬帴\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}}s:6:\"script\";a:5:{s:18:\"groupthreadspecial\";s:12:\"鐗规畩涓婚\";s:11:\"groupthread\";s:15:\"楂樼骇鑷畾涔塡";s:14:\"groupthreadnew\";s:9:\"鏂颁富棰榎";s:20:\"groupthreadspecified\";s:12:\"鎸囧畾涓婚\";s:14:\"groupthreadhot\";s:12:\"鐑棬涓婚\";}}}}s:4:\"html\";a:2:{s:4:\"name\";s:9:\"灞曠ず绫籠";s:4:\"subs\";a:3:{s:9:\"html_html\";a:3:{s:4:\"name\";s:12:\"闈欐€佹ā鍧梊";s:6:\"fields\";a:0:{}s:6:\"script\";a:13:{s:3:\"api\";s:10:\"HTTP鎺ュ彛\";s:4:\"sort\";s:12:\"鍒嗙被淇℃伅\";s:8:\"witframe\";s:9:\"浜戞彃浠禱";s:6:\"google\";s:6:\"GOOGLE\";s:9:\"forumtree\";s:12:\"鐗堝潡鍒楄〃\";s:6:\"banner\";s:12:\"鍥剧墖妯箙\";s:5:\"blank\";s:13:\"鑷畾涔塇TML\";s:3:\"adv\";s:12:\"绔欑偣骞垮憡\";s:4:\"stat\";s:12:\"鏁版嵁缁熻\";s:6:\"search\";s:9:\"鎼滅储鏉";s:5:\"vedio\";s:12:\"缃戠粶瑙嗛\";s:10:\"friendlink\";s:12:\"鍙嬫儏閾炬帴\";s:4:\"line\";s:9:\"鍒嗗壊绾縗";}}s:12:\"html_misctag\";a:3:{s:4:\"name\";s:12:\"鏍囩妯″潡\";s:6:\"fields\";a:7:{s:2:\"id\";a:3:{s:4:\"name\";s:8:\"鏍囩ID\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:3:\"int\";}s:3:\"url\";a:3:{s:4:\"name\";s:12:\"鏍囩閾炬帴\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:5:\"title\";a:3:{s:4:\"name\";s:12:\"鏍囩鍚嶇О\";s:8:\"formtype\";s:5:\"title\";s:8:\"datatype\";s:5:\"title\";}s:13:\"related_count\";a:3:{s:4:\"name\";s:15:\"鍏宠仈鏁版嵁閲廫";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:3:\"int\";}s:9:\"hot_score\";a:3:{s:4:\"name\";s:6:\"鐑害\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:10:\"size_level\";a:3:{s:4:\"name\";s:18:\"鏍囩澶у皬绛夌骇\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:3:\"int\";}s:11:\"color_level\";a:3:{s:4:\"name\";s:18:\"鏍囩棰滆壊绛夌骇\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:3:\"int\";}}s:6:\"script\";a:1:{s:7:\"misctag\";s:12:\"缃戠珯鏍囩\";}}s:17:\"html_announcement\";a:3:{s:4:\"name\";s:12:\"鍏憡妯″潡\";s:6:\"fields\";a:5:{s:3:\"url\";a:3:{s:4:\"name\";s:12:\"鍏憡閾炬帴\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:5:\"title\";a:3:{s:4:\"name\";s:12:\"鍏憡鏍囬\";s:8:\"formtype\";s:5:\"title\";s:8:\"datatype\";s:5:\"title\";}s:7:\"summary\";a:3:{s:4:\"name\";s:12:\"鍏憡鍐呭\";s:8:\"formtype\";s:7:\"summary\";s:8:\"datatype\";s:7:\"summary\";}s:9:\"starttime\";a:3:{s:4:\"name\";s:12:\"寮€濮嬫椂闂碶";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:4:\"date\";}s:7:\"endtime\";a:3:{s:4:\"name\";s:12:\"缁撴潫鏃堕棿\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:4:\"date\";}}s:6:\"script\";a:1:{s:12:\"announcement\";s:12:\"绔欑偣鍏憡\";}}}}s:6:\"member\";a:2:{s:4:\"name\";s:9:\"浼氬憳绫籠";s:4:\"subs\";a:1:{s:13:\"member_member\";a:3:{s:4:\"name\";s:12:\"浼氬憳妯″潡\";s:6:\"fields\";a:62:{s:2:\"id\";a:3:{s:4:\"name\";s:8:\"鏁版嵁ID\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:3:\"int\";}s:3:\"url\";a:3:{s:4:\"name\";s:12:\"绌洪棿閾炬帴\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:3:\"uid\";a:3:{s:4:\"name\";s:9:\"鐢ㄦ埛UID\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:3:\"int\";}s:5:\"title\";a:3:{s:4:\"name\";s:9:\"鐢ㄦ埛鍚峔";s:8:\"formtype\";s:5:\"title\";s:8:\"datatype\";s:5:\"title\";}s:6:\"avatar\";a:3:{s:4:\"name\";s:12:\"鐢ㄦ埛澶村儚\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:13:\"avatar_middle\";a:3:{s:4:\"name\";s:17:\"鐢ㄦ埛澶村儚(涓?\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:10:\"avatar_big\";a:3:{s:4:\"name\";s:17:\"鐢ㄦ埛澶村儚(澶?\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:9:\"avatarimg\";a:3:{s:4:\"name\";s:12:\"鐢ㄦ埛澶村儚\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:16:\"avatarimg_middle\";a:3:{s:4:\"name\";s:17:\"鐢ㄦ埛澶村儚(涓?\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:13:\"avatarimg_big\";a:3:{s:4:\"name\";s:17:\"鐢ㄦ埛澶村儚(澶?\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:7:\"regdate\";a:3:{s:4:\"name\";s:12:\"娉ㄥ唽鏃堕棿\";s:8:\"formtype\";s:4:\"date\";s:8:\"datatype\";s:4:\"date\";}s:5:\"posts\";a:3:{s:4:\"name\";s:9:\"鍙戝笘鏁癨";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:3:\"int\";}s:7:\"threads\";a:3:{s:4:\"name\";s:9:\"涓婚鏁癨";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:3:\"int\";}s:11:\"digestposts\";a:3:{s:4:\"name\";s:12:\"绮惧崕甯栨暟\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:3:\"int\";}s:7:\"credits\";a:3:{s:4:\"name\";s:9:\"绉垎鏁癨";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:3:\"int\";}s:6:\"reason\";a:3:{s:4:\"name\";s:12:\"鎺ㄨ崘鍘熷洜\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:9:\"unitprice\";a:3:{s:4:\"name\";s:24:\"绔熶环鍗曟璁块棶鍗曚环\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:3:\"int\";}s:10:\"showcredit\";a:3:{s:4:\"name\";s:15:\"绔熶环鎬荤Н鍒哱";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:3:\"int\";}s:8:\"shownote\";a:3:{s:4:\"name\";s:18:\"绔熶环涓婃瀹ｈ█\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:11:\"extcredits1\";a:3:{s:4:\"name\";s:6:\"濞佹湜\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:3:\"int\";}s:11:\"extcredits2\";a:3:{s:4:\"name\";s:6:\"閲戦挶\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:3:\"int\";}s:11:\"extcredits3\";a:3:{s:4:\"name\";s:6:\"璐＄尞\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:3:\"int\";}s:2:\"qq\";a:3:{s:4:\"name\";s:2:\"QQ\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:3:\"bio\";a:3:{s:4:\"name\";s:12:\"鑷垜浠嬬粛\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:3:\"msn\";a:3:{s:4:\"name\";s:3:\"MSN\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:4:\"site\";a:3:{s:4:\"name\";s:12:\"涓汉涓婚〉\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:6:\"alipay\";a:3:{s:4:\"name\";s:9:\"鏀粯瀹漒";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:6:\"fields\";a:3:{s:4:\"name\";s:21:\"鏇村鑷畾涔夎祫鏂橽";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:6:\"gender\";a:3:{s:4:\"name\";s:6:\"鎬у埆\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:6:\"idcard\";a:3:{s:4:\"name\";s:9:\"璇佷欢鍙穃";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:6:\"mobile\";a:3:{s:4:\"name\";s:6:\"鎵嬫満\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:6:\"taobao\";a:3:{s:4:\"name\";s:12:\"闃块噷鏃烘椇\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:6:\"zodiac\";a:3:{s:4:\"name\";s:6:\"鐢熻倴\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:7:\"address\";a:3:{s:4:\"name\";s:12:\"閭瘎鍦板潃\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:7:\"company\";a:3:{s:4:\"name\";s:6:\"鍏徃\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:7:\"revenue\";a:3:{s:4:\"name\";s:9:\"骞存敹鍏";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:7:\"zipcode\";a:3:{s:4:\"name\";s:6:\"閭紪\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:8:\"birthday\";a:3:{s:4:\"name\";s:6:\"鐢熸棩\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:8:\"interest\";a:3:{s:4:\"name\";s:12:\"鍏磋叮鐖卞ソ\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:8:\"position\";a:3:{s:4:\"name\";s:6:\"鑱屼綅\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:8:\"realname\";a:3:{s:4:\"name\";s:12:\"鐪熷疄濮撳悕\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:9:\"birthcity\";a:3:{s:4:\"name\";s:9:\"鍑虹敓鍦癨";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:9:\"birthdist\";a:3:{s:4:\"name\";s:9:\"鍑虹敓鍘縗";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:9:\"birthyear\";a:3:{s:4:\"name\";s:12:\"鍑虹敓骞翠唤\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:9:\"bloodtype\";a:3:{s:4:\"name\";s:6:\"琛€鍨媆";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:9:\"education\";a:3:{s:4:\"name\";s:6:\"瀛﹀巻\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:9:\"telephone\";a:3:{s:4:\"name\";s:12:\"鍥哄畾鐢佃瘽\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:10:\"birthmonth\";a:3:{s:4:\"name\";s:12:\"鍑虹敓鏈堜唤\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:10:\"idcardtype\";a:3:{s:4:\"name\";s:12:\"璇佷欢绫诲瀷\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:10:\"lookingfor\";a:3:{s:4:\"name\";s:12:\"浜ゅ弸鐩殑\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:10:\"occupation\";a:3:{s:4:\"name\";s:6:\"鑱屼笟\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:10:\"residecity\";a:3:{s:4:\"name\";s:9:\"灞呬綇鍦癨";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:10:\"residedist\";a:3:{s:4:\"name\";s:9:\"灞呬綇鍘縗";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:12:\"birthcountry\";a:3:{s:4:\"name\";s:12:\"鍑虹敓鍥藉\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:13:\"birthprovince\";a:3:{s:4:\"name\";s:12:\"鍑虹敓鐪佷唤\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:13:\"constellation\";a:3:{s:4:\"name\";s:6:\"鏄熷骇\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:13:\"residecountry\";a:3:{s:4:\"name\";s:12:\"灞呬綇鍥藉\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:14:\"birthcommunity\";a:3:{s:4:\"name\";s:12:\"鍑虹敓灏忓尯\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:14:\"graduateschool\";a:3:{s:4:\"name\";s:12:\"姣曚笟瀛︽牎\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:14:\"resideprovince\";a:3:{s:4:\"name\";s:12:\"灞呬綇鐪佷唤\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:15:\"affectivestatus\";a:3:{s:4:\"name\";s:12:\"鎯呮劅鐘舵€乗";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:15:\"residecommunity\";a:3:{s:4:\"name\";s:12:\"灞呬綇灏忓尯\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}}s:6:\"script\";a:7:{s:11:\"memberposts\";s:12:\"鍙戝笘鎺掕\";s:9:\"membernew\";s:9:\"鏂颁細鍛榎";s:6:\"member\";s:15:\"楂樼骇鑷畾涔塡";s:12:\"membercredit\";s:12:\"绉垎鎺掕\";s:13:\"memberspecial\";s:12:\"鐗规畩浼氬憳\";s:15:\"memberspecified\";s:12:\"鎸囧畾鐢ㄦ埛\";s:10:\"membershow\";s:12:\"绔炰环鎺掕\";}}}}s:5:\"other\";a:2:{s:4:\"name\";s:9:\"鍏跺畠绫籠";s:4:\"subs\";a:2:{s:15:\"other_otherstat\";a:3:{s:4:\"name\";s:12:\"缁熻妯″潡\";s:6:\"fields\";a:26:{s:5:\"posts\";a:3:{s:4:\"name\";s:12:\"鍙戝笘鎬绘暟\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:3:\"int\";}s:11:\"posts_title\";a:3:{s:4:\"name\";s:15:\"甯栧瓙鏄剧ず鍚峔";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:6:\"groups\";a:3:{s:4:\"name\";s:12:\"鍦堝瓙鎬绘暟\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:3:\"int\";}s:12:\"groups_title\";a:3:{s:4:\"name\";s:15:\"鍦堝瓙鏄剧ず鍚峔";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:7:\"members\";a:3:{s:4:\"name\";s:12:\"浼氬憳鎬绘暟\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:3:\"int\";}s:13:\"members_title\";a:3:{s:4:\"name\";s:15:\"浼氬憳鏄剧ず鍚峔";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:13:\"groupnewposts\";a:3:{s:4:\"name\";s:18:\"鍦堝瓙浠婃棩鍙戝笘\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:3:\"int\";}s:19:\"groupnewposts_title\";a:3:{s:4:\"name\";s:21:\"浠婃棩鍙戝笘鏄剧ず鍚峔";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:11:\"bbsnewposts\";a:3:{s:4:\"name\";s:21:\"璁哄潧浠婃棩鍙戝笘鏁癨";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:3:\"int\";}s:17:\"bbsnewposts_title\";a:3:{s:4:\"name\";s:21:\"浠婃棩鍙戝笘鏄剧ず鍚峔";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:12:\"bbslastposts\";a:3:{s:4:\"name\";s:21:\"璁哄潧鏄ㄦ棩鍙戝笘鏁癨";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:3:\"int\";}s:18:\"bbslastposts_title\";a:3:{s:4:\"name\";s:21:\"鏄ㄦ棩鍙戝笘鏄剧ず鍚峔";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:13:\"onlinemembers\";a:3:{s:4:\"name\";s:21:\"褰撳墠鍦ㄧ嚎浼氬憳鏁癨";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:3:\"int\";}s:19:\"onlinemembers_title\";a:3:{s:4:\"name\";s:27:\"褰撳墠鍦ㄧ嚎浼氬憳鏄剧ず鍚峔";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:10:\"maxmembers\";a:3:{s:4:\"name\";s:27:\"鍘嗗彶鏈€楂樺湪绾夸細鍛樻暟\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:3:\"int\";}s:16:\"maxmembers_title\";a:3:{s:4:\"name\";s:27:\"鍘嗗彶鏈€楂樺湪绾挎樉绀哄悕\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:6:\"doings\";a:3:{s:4:\"name\";s:9:\"鍔ㄦ€佹暟\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:3:\"int\";}s:12:\"doings_title\";a:3:{s:4:\"name\";s:15:\"鍔ㄦ€佹樉绀哄悕\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:5:\"blogs\";a:3:{s:4:\"name\";s:9:\"鏃ュ織鏁癨";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:3:\"int\";}s:11:\"blogs_title\";a:3:{s:4:\"name\";s:15:\"鏃ュ織鏄剧ず鍚峔";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:6:\"albums\";a:3:{s:4:\"name\";s:9:\"鐩稿唽鏁癨";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:3:\"int\";}s:12:\"albums_title\";a:3:{s:4:\"name\";s:15:\"鐩稿唽鏄剧ず鍚峔";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:4:\"pics\";a:3:{s:4:\"name\";s:9:\"鍥剧墖鏁癨";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:3:\"int\";}s:10:\"pics_title\";a:3:{s:4:\"name\";s:15:\"鍥剧墖鏄剧ず鍚峔";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:6:\"shares\";a:3:{s:4:\"name\";s:9:\"鍒嗕韩鏁癨";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:3:\"int\";}s:12:\"shares_title\";a:3:{s:4:\"name\";s:15:\"鍒嗕韩鏄剧ず鍚峔";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}}s:6:\"script\";a:1:{s:9:\"otherstat\";s:15:\"楂樼骇鑷畾涔塡";}}s:21:\"other_otherfriendlink\";a:3:{s:4:\"name\";s:12:\"鍙嬫儏閾炬帴\";s:6:\"fields\";a:4:{s:3:\"url\";a:3:{s:4:\"name\";s:9:\"绔欑偣URL\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:5:\"title\";a:3:{s:4:\"name\";s:12:\"绔欑偣鍚嶇О\";s:8:\"formtype\";s:5:\"title\";s:8:\"datatype\";s:5:\"title\";}s:3:\"pic\";a:3:{s:4:\"name\";s:10:\"绔欑偣LOGO\";s:8:\"formtype\";s:3:\"pic\";s:8:\"datatype\";s:3:\"pic\";}s:7:\"summary\";a:3:{s:4:\"name\";s:12:\"绔欑偣绠€浠媆";s:8:\"formtype\";s:7:\"summary\";s:8:\"datatype\";s:7:\"summary\";}}s:6:\"script\";a:1:{s:15:\"otherfriendlink\";s:15:\"楂樼骇鑷畾涔塡";}}}}s:6:\"portal\";a:2:{s:4:\"name\";s:9:\"闂ㄦ埛绫籠";s:4:\"subs\";a:3:{s:14:\"portal_article\";a:3:{s:4:\"name\";s:12:\"鏂囩珷妯″潡\";s:6:\"fields\";a:21:{s:2:\"id\";a:3:{s:4:\"name\";s:8:\"鏁版嵁ID\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:3:\"int\";}s:3:\"uid\";a:3:{s:4:\"name\";s:9:\"浣滆€匲ID\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:3:\"int\";}s:8:\"username\";a:3:{s:4:\"name\";s:9:\"浣滆€呭悕\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:6:\"avatar\";a:3:{s:4:\"name\";s:12:\"鐢ㄦ埛澶村儚\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:13:\"avatar_middle\";a:3:{s:4:\"name\";s:17:\"鐢ㄦ埛澶村儚(涓?\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:10:\"avatar_big\";a:3:{s:4:\"name\";s:17:\"鐢ㄦ埛澶村儚(澶?\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:9:\"avatarimg\";a:3:{s:4:\"name\";s:12:\"鐢ㄦ埛澶村儚\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:16:\"avatarimg_middle\";a:3:{s:4:\"name\";s:17:\"鐢ㄦ埛澶村儚(涓?\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:13:\"avatarimg_big\";a:3:{s:4:\"name\";s:17:\"鐢ㄦ埛澶村儚(澶?\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:3:\"url\";a:3:{s:4:\"name\";s:12:\"鏂囩珷閾炬帴\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:5:\"title\";a:3:{s:4:\"name\";s:12:\"鏂囩珷鏍囬\";s:8:\"formtype\";s:5:\"title\";s:8:\"datatype\";s:5:\"title\";}s:7:\"fromurl\";a:3:{s:4:\"name\";s:12:\"鏉ユ簮鍦板潃\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:4:\"from\";a:3:{s:4:\"name\";s:12:\"鏂囩珷鏉ユ簮\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:3:\"pic\";a:3:{s:4:\"name\";s:12:\"鏂囩珷灏侀潰\";s:8:\"formtype\";s:3:\"pic\";s:8:\"datatype\";s:3:\"pic\";}s:7:\"summary\";a:3:{s:4:\"name\";s:12:\"鏂囩珷绠€浠媆";s:8:\"formtype\";s:7:\"summary\";s:8:\"datatype\";s:7:\"summary\";}s:8:\"dateline\";a:3:{s:4:\"name\";s:12:\"鍙戝竷鏃堕棿\";s:8:\"formtype\";s:4:\"date\";s:8:\"datatype\";s:4:\"date\";}s:6:\"caturl\";a:3:{s:4:\"name\";s:12:\"鏍忕洰閾炬帴\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:7:\"catname\";a:3:{s:4:\"name\";s:12:\"鏍忕洰鍚嶇О\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:8:\"articles\";a:3:{s:4:\"name\";s:9:\"鏂囩珷鏁癨";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:3:\"int\";}s:7:\"viewnum\";a:3:{s:4:\"name\";s:9:\"鏌ョ湅鏁癨";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:3:\"int\";}s:10:\"commentnum\";a:3:{s:4:\"name\";s:9:\"璇勮鏁癨";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:3:\"int\";}}s:6:\"script\";a:4:{s:16:\"articlespecified\";s:12:\"鎸囧畾鏂囩珷\";s:7:\"article\";s:15:\"楂樼骇鑷畾涔塡";s:10:\"articlenew\";s:12:\"鏈€鏂版枃绔燶";s:10:\"articlehot\";s:12:\"鐑棬鏂囩珷\";}}s:12:\"portal_topic\";a:3:{s:4:\"name\";s:12:\"涓撻妯″潡\";s:6:\"fields\";a:9:{s:2:\"id\";a:3:{s:4:\"name\";s:8:\"鏁版嵁ID\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:3:\"int\";}s:3:\"url\";a:3:{s:4:\"name\";s:12:\"涓撻閾炬帴\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:5:\"title\";a:3:{s:4:\"name\";s:12:\"涓撻鍚嶇О\";s:8:\"formtype\";s:5:\"title\";s:8:\"datatype\";s:5:\"title\";}s:3:\"pic\";a:3:{s:4:\"name\";s:12:\"涓撻灏侀潰\";s:8:\"formtype\";s:3:\"pic\";s:8:\"datatype\";s:3:\"pic\";}s:7:\"summary\";a:3:{s:4:\"name\";s:12:\"涓撻浠嬬粛\";s:8:\"formtype\";s:7:\"summary\";s:8:\"datatype\";s:7:\"summary\";}s:3:\"uid\";a:3:{s:4:\"name\";s:12:\"鍒涘缓鑰匲ID\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:3:\"int\";}s:8:\"username\";a:3:{s:4:\"name\";s:9:\"鍒涘缓鑰匼";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:8:\"dateline\";a:3:{s:4:\"name\";s:12:\"鍒涘缓鏃堕棿\";s:8:\"formtype\";s:4:\"date\";s:8:\"datatype\";s:4:\"date\";}s:7:\"viewnum\";a:3:{s:4:\"name\";s:9:\"鏌ョ湅鏁癨";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:3:\"int\";}}s:6:\"script\";a:4:{s:5:\"topic\";s:15:\"楂樼骇鑷畾涔塡";s:8:\"topicnew\";s:12:\"鏈€鏂颁笓棰榎";s:14:\"topicspecified\";s:12:\"鎸囧畾涓撻\";s:8:\"topichot\";s:12:\"鐑棬涓撻\";}}s:15:\"portal_category\";a:3:{s:4:\"name\";s:12:\"鏂囩珷鏍忕洰\";s:6:\"fields\";a:4:{s:2:\"id\";a:3:{s:4:\"name\";s:8:\"鏁版嵁ID\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:3:\"int\";}s:3:\"url\";a:3:{s:4:\"name\";s:12:\"鏍忕洰閾炬帴\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:5:\"title\";a:3:{s:4:\"name\";s:12:\"鏍忕洰鍚嶇О\";s:8:\"formtype\";s:5:\"title\";s:8:\"datatype\";s:5:\"title\";}s:8:\"articles\";a:3:{s:4:\"name\";s:9:\"鏂囩珷鏁癨";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:3:\"int\";}}s:6:\"script\";a:1:{s:14:\"portalcategory\";s:12:\"鏂囩珷鏍忕洰\";}}}}s:5:\"space\";a:2:{s:4:\"name\";s:9:\"绌洪棿绫籠";s:4:\"subs\";a:4:{s:9:\"space_pic\";a:3:{s:4:\"name\";s:12:\"鍥剧墖妯″潡\";s:6:\"fields\";a:17:{s:2:\"id\";a:3:{s:4:\"name\";s:8:\"鏁版嵁ID\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:3:\"int\";}s:3:\"url\";a:3:{s:4:\"name\";s:12:\"鍥剧墖閾炬帴\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:5:\"title\";a:3:{s:4:\"name\";s:12:\"鍥剧墖鍚嶇О\";s:8:\"formtype\";s:5:\"title\";s:8:\"datatype\";s:5:\"title\";}s:3:\"pic\";a:3:{s:4:\"name\";s:12:\"鍥剧墖鍦板潃\";s:8:\"formtype\";s:3:\"pic\";s:8:\"datatype\";s:3:\"pic\";}s:7:\"summary\";a:3:{s:4:\"name\";s:12:\"鍥剧墖璇存槑\";s:8:\"formtype\";s:7:\"summary\";s:8:\"datatype\";s:7:\"summary\";}s:3:\"uid\";a:3:{s:4:\"name\";s:9:\"鐢ㄦ埛UID\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:3:\"int\";}s:8:\"username\";a:3:{s:4:\"name\";s:9:\"鐢ㄦ埛鍚峔";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:8:\"dateline\";a:3:{s:4:\"name\";s:12:\"涓婁紶鏃堕棿\";s:8:\"formtype\";s:4:\"date\";s:8:\"datatype\";s:4:\"date\";}s:7:\"viewnum\";a:3:{s:4:\"name\";s:9:\"鏌ョ湅鏁癨";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:3:\"int\";}s:6:\"click1\";a:3:{s:4:\"name\";s:10:\"琛ㄦ€侀」1\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:3:\"int\";}s:6:\"click2\";a:3:{s:4:\"name\";s:10:\"琛ㄦ€侀」2\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:3:\"int\";}s:6:\"click3\";a:3:{s:4:\"name\";s:10:\"琛ㄦ€侀」3\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:3:\"int\";}s:6:\"click4\";a:3:{s:4:\"name\";s:10:\"琛ㄦ€侀」4\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:3:\"int\";}s:6:\"click5\";a:3:{s:4:\"name\";s:10:\"琛ㄦ€侀」5\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:3:\"int\";}s:6:\"click6\";a:3:{s:4:\"name\";s:10:\"琛ㄦ€侀」6\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:3:\"int\";}s:6:\"click7\";a:3:{s:4:\"name\";s:10:\"琛ㄦ€侀」7\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:3:\"int\";}s:6:\"click8\";a:3:{s:4:\"name\";s:10:\"琛ㄦ€侀」8\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:3:\"int\";}}s:6:\"script\";a:4:{s:3:\"pic\";s:15:\"楂樼骇鑷畾涔塡";s:12:\"picspecified\";s:12:\"鎸囧畾鍥剧墖\";s:6:\"picnew\";s:12:\"鏈€鏂板浘鐗嘰";s:6:\"pichot\";s:12:\"鐑棬鍥剧墖\";}}s:11:\"space_doing\";a:3:{s:4:\"name\";s:12:\"璁板綍妯″潡\";s:6:\"fields\";a:13:{s:2:\"id\";a:3:{s:4:\"name\";s:8:\"鏁版嵁ID\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:3:\"int\";}s:3:\"url\";a:3:{s:4:\"name\";s:12:\"璁板綍閾炬帴\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:5:\"title\";a:3:{s:4:\"name\";s:12:\"璁板綍鍐呭\";s:8:\"formtype\";s:5:\"title\";s:8:\"datatype\";s:5:\"title\";}s:3:\"uid\";a:3:{s:4:\"name\";s:9:\"鐢ㄦ埛UID\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:3:\"pic\";}s:8:\"username\";a:3:{s:4:\"name\";s:9:\"鐢ㄦ埛鍚峔";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:6:\"avatar\";a:3:{s:4:\"name\";s:12:\"鐢ㄦ埛澶村儚\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:13:\"avatar_middle\";a:3:{s:4:\"name\";s:17:\"鐢ㄦ埛澶村儚(涓?\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:10:\"avatar_big\";a:3:{s:4:\"name\";s:17:\"鐢ㄦ埛澶村儚(澶?\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:9:\"avatarimg\";a:3:{s:4:\"name\";s:12:\"鐢ㄦ埛澶村儚\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:16:\"avatarimg_middle\";a:3:{s:4:\"name\";s:17:\"鐢ㄦ埛澶村儚(涓?\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:13:\"avatarimg_big\";a:3:{s:4:\"name\";s:17:\"鐢ㄦ埛澶村儚(澶?\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:8:\"dateline\";a:3:{s:4:\"name\";s:12:\"鍙戝竷鏃堕棿\";s:8:\"formtype\";s:4:\"date\";s:8:\"datatype\";s:4:\"date\";}s:8:\"replynum\";a:3:{s:4:\"name\";s:9:\"鍥炲鏁癨";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:3:\"int\";}}s:6:\"script\";a:3:{s:8:\"doinghot\";s:12:\"鐑棬璁板綍\";s:5:\"doing\";s:15:\"楂樼骇鑷畾涔塡";s:8:\"doingnew\";s:12:\"鏈€鏂拌褰昞";}}s:10:\"space_blog\";a:3:{s:4:\"name\";s:12:\"鏃ュ織妯″潡\";s:6:\"fields\";a:24:{s:2:\"id\";a:3:{s:4:\"name\";s:8:\"鏁版嵁ID\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:3:\"int\";}s:3:\"url\";a:3:{s:4:\"name\";s:12:\"鏃ュ織閾炬帴\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:5:\"title\";a:3:{s:4:\"name\";s:12:\"鏃ュ織鏍囬\";s:8:\"formtype\";s:5:\"title\";s:8:\"datatype\";s:5:\"title\";}s:7:\"summary\";a:3:{s:4:\"name\";s:12:\"鏃ュ織绠€浠媆";s:8:\"formtype\";s:7:\"summary\";s:8:\"datatype\";s:7:\"summary\";}s:3:\"pic\";a:3:{s:4:\"name\";s:12:\"鏃ュ織鍥剧墖\";s:8:\"formtype\";s:3:\"pic\";s:8:\"datatype\";s:3:\"pic\";}s:8:\"dateline\";a:3:{s:4:\"name\";s:12:\"鍙戝竷鏃堕棿\";s:8:\"formtype\";s:4:\"date\";s:8:\"datatype\";s:4:\"date\";}s:3:\"uid\";a:3:{s:4:\"name\";s:9:\"浣滆€匲ID\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:3:\"int\";}s:8:\"username\";a:3:{s:4:\"name\";s:9:\"浣滆€呭悕\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:6:\"avatar\";a:3:{s:4:\"name\";s:12:\"鐢ㄦ埛澶村儚\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:13:\"avatar_middle\";a:3:{s:4:\"name\";s:17:\"鐢ㄦ埛澶村儚(涓?\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:10:\"avatar_big\";a:3:{s:4:\"name\";s:17:\"鐢ㄦ埛澶村儚(澶?\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:9:\"avatarimg\";a:3:{s:4:\"name\";s:12:\"鐢ㄦ埛澶村儚\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:16:\"avatarimg_middle\";a:3:{s:4:\"name\";s:17:\"鐢ㄦ埛澶村儚(涓?\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:13:\"avatarimg_big\";a:3:{s:4:\"name\";s:17:\"鐢ㄦ埛澶村儚(澶?\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:8:\"replynum\";a:3:{s:4:\"name\";s:9:\"璇勮鏁癨";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:3:\"int\";}s:7:\"viewnum\";a:3:{s:4:\"name\";s:9:\"娴忚鏁癨";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:3:\"int\";}s:6:\"click1\";a:3:{s:4:\"name\";s:10:\"琛ㄦ€侀」1\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:3:\"int\";}s:6:\"click2\";a:3:{s:4:\"name\";s:10:\"琛ㄦ€侀」2\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:3:\"int\";}s:6:\"click3\";a:3:{s:4:\"name\";s:10:\"琛ㄦ€侀」3\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:3:\"int\";}s:6:\"click4\";a:3:{s:4:\"name\";s:10:\"琛ㄦ€侀」4\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:3:\"int\";}s:6:\"click5\";a:3:{s:4:\"name\";s:10:\"琛ㄦ€侀」5\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:3:\"int\";}s:6:\"click6\";a:3:{s:4:\"name\";s:10:\"琛ㄦ€侀」6\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:3:\"int\";}s:6:\"click7\";a:3:{s:4:\"name\";s:10:\"琛ㄦ€侀」7\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:3:\"int\";}s:6:\"click8\";a:3:{s:4:\"name\";s:10:\"琛ㄦ€侀」8\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:3:\"int\";}}s:6:\"script\";a:4:{s:7:\"blognew\";s:12:\"鏈€鏂版棩蹇梊";s:13:\"blogspecified\";s:12:\"鎸囧畾鏃ュ織\";s:7:\"bloghot\";s:12:\"鐑棬鏃ュ織\";s:4:\"blog\";s:15:\"楂樼骇鑷畾涔塡";}}s:11:\"space_album\";a:3:{s:4:\"name\";s:12:\"鐩稿唽妯″潡\";s:6:\"fields\";a:9:{s:2:\"id\";a:3:{s:4:\"name\";s:8:\"鏁版嵁ID\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:3:\"int\";}s:3:\"url\";a:3:{s:4:\"name\";s:12:\"鐩稿唽閾炬帴\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:5:\"title\";a:3:{s:4:\"name\";s:12:\"鐩稿唽鍚嶇О\";s:8:\"formtype\";s:5:\"title\";s:8:\"datatype\";s:5:\"title\";}s:3:\"pic\";a:3:{s:4:\"name\";s:12:\"鐩稿唽灏侀潰\";s:8:\"formtype\";s:3:\"pic\";s:8:\"datatype\";s:3:\"pic\";}s:3:\"uid\";a:3:{s:4:\"name\";s:9:\"鐢ㄦ埛UID\";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:3:\"int\";}s:8:\"username\";a:3:{s:4:\"name\";s:9:\"鐢ㄦ埛鍚峔";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:6:\"string\";}s:8:\"dateline\";a:3:{s:4:\"name\";s:12:\"鍒涘缓鏃ユ湡\";s:8:\"formtype\";s:4:\"date\";s:8:\"datatype\";s:4:\"date\";}s:10:\"updatetime\";a:3:{s:4:\"name\";s:12:\"鏇存柊鏃ユ湡\";s:8:\"formtype\";s:4:\"date\";s:8:\"datatype\";s:4:\"date\";}s:6:\"picnum\";a:3:{s:4:\"name\";s:9:\"鐓х墖鏁癨";s:8:\"formtype\";s:4:\"text\";s:8:\"datatype\";s:3:\"int\";}}s:6:\"script\";a:3:{s:8:\"albumnew\";s:12:\"鏈€鏂扮浉鍐孿";s:14:\"albumspecified\";s:12:\"鎸囧畾鐩稿唽\";s:5:\"album\";s:15:\"楂樼骇鑷畾涔塡";}}}}}'),
('blockconvert',1,1784453715,'a:4:{s:5:\"forum\";a:4:{s:12:\"forum_thread\";a:3:{s:14:\"portal_article\";a:4:{s:4:\"name\";s:12:\"鏂囩珷妯″潡\";s:6:\"script\";s:7:\"article\";s:10:\"searchkeys\";a:7:{i:0;s:6:\"author\";i:1;s:8:\"authorid\";i:2;s:8:\"forumurl\";i:3;s:9:\"forumname\";i:4;s:5:\"posts\";i:5;s:5:\"views\";i:6;s:7:\"replies\";}s:11:\"replacekeys\";a:7:{i:0;s:8:\"username\";i:1;s:3:\"uid\";i:2;s:6:\"caturl\";i:3;s:7:\"catname\";i:4;s:8:\"articles\";i:5;s:7:\"viewnum\";i:6;s:10:\"commentnum\";}}s:10:\"space_blog\";a:4:{s:4:\"name\";s:12:\"鏃ュ織妯″潡\";s:6:\"script\";s:4:\"blog\";s:10:\"searchkeys\";a:4:{i:0;s:6:\"author\";i:1;s:8:\"authorid\";i:2;s:5:\"views\";i:3;s:7:\"replies\";}s:11:\"replacekeys\";a:4:{i:0;s:8:\"username\";i:1;s:3:\"uid\";i:2;s:7:\"viewnum\";i:3;s:8:\"replynum\";}}s:12:\"group_thread\";a:4:{s:4:\"name\";s:12:\"鍦堝瓙甯栧瓙\";s:6:\"script\";s:11:\"groupthread\";s:10:\"searchkeys\";a:2:{i:0;s:9:\"forumname\";i:1;s:8:\"forumurl\";}s:11:\"replacekeys\";a:2:{i:0;s:9:\"groupname\";i:1;s:8:\"groupurl\";}}}s:11:\"forum_trade\";a:1:{s:11:\"group_trade\";a:4:{s:4:\"name\";s:12:\"鍦堝瓙鍟嗗搧\";s:6:\"script\";s:10:\"grouptrade\";s:10:\"searchkeys\";a:0:{}s:11:\"replacekeys\";a:0:{}}}s:14:\"forum_activity\";a:1:{s:14:\"group_activity\";a:4:{s:4:\"name\";s:12:\"鍦堝瓙娲诲姩\";s:6:\"script\";s:13:\"groupactivity\";s:10:\"searchkeys\";a:0:{}s:11:\"replacekeys\";a:0:{}}}s:11:\"forum_forum\";a:2:{s:11:\"group_group\";a:4:{s:4:\"name\";s:12:\"鍦堝瓙妯″潡\";s:6:\"script\";s:5:\"group\";s:10:\"searchkeys\";a:0:{}s:11:\"replacekeys\";a:0:{}}s:15:\"portal_category\";a:4:{s:4:\"name\";s:12:\"鏂囩珷鏍忕洰\";s:6:\"script\";s:14:\"portalcategory\";s:10:\"searchkeys\";a:1:{i:0;s:7:\"threads\";}s:11:\"replacekeys\";a:1:{i:0;s:8:\"articles\";}}}}s:5:\"group\";a:4:{s:11:\"group_group\";a:2:{s:11:\"forum_forum\";a:4:{s:4:\"name\";s:12:\"鐗堝潡妯″潡\";s:6:\"script\";s:5:\"forum\";s:10:\"searchkeys\";a:0:{}s:11:\"replacekeys\";a:0:{}}s:15:\"portal_category\";a:4:{s:4:\"name\";s:12:\"鏂囩珷鏍忕洰\";s:6:\"script\";s:14:\"portalcategory\";s:10:\"searchkeys\";a:1:{i:0;s:7:\"threads\";}s:11:\"replacekeys\";a:1:{i:0;s:8:\"articles\";}}}s:14:\"group_activity\";a:1:{s:14:\"forum_activity\";a:4:{s:4:\"name\";s:12:\"娲诲姩妯″潡\";s:6:\"script\";s:8:\"activity\";s:10:\"searchkeys\";a:0:{}s:11:\"replacekeys\";a:0:{}}}s:11:\"group_trade\";a:1:{s:11:\"forum_trade\";a:4:{s:4:\"name\";s:12:\"鍟嗗搧妯″潡\";s:6:\"script\";s:5:\"trade\";s:10:\"searchkeys\";a:0:{}s:11:\"replacekeys\";a:0:{}}}s:12:\"group_thread\";a:3:{s:14:\"portal_article\";a:4:{s:4:\"name\";s:12:\"鏂囩珷妯″潡\";s:6:\"script\";s:7:\"article\";s:10:\"searchkeys\";a:7:{i:0;s:6:\"author\";i:1;s:8:\"authorid\";i:2;s:8:\"groupurl\";i:3;s:9:\"groupname\";i:4;s:5:\"posts\";i:5;s:5:\"views\";i:6;s:7:\"replies\";}s:11:\"replacekeys\";a:7:{i:0;s:8:\"username\";i:1;s:3:\"uid\";i:2;s:6:\"caturl\";i:3;s:7:\"catname\";i:4;s:8:\"articles\";i:5;s:7:\"viewnum\";i:6;s:10:\"commentnum\";}}s:10:\"space_blog\";a:4:{s:4:\"name\";s:12:\"鏃ュ織妯″潡\";s:6:\"script\";s:4:\"blog\";s:10:\"searchkeys\";a:4:{i:0;s:6:\"author\";i:1;s:8:\"authorid\";i:2;s:5:\"views\";i:3;s:7:\"replies\";}s:11:\"replacekeys\";a:4:{i:0;s:8:\"username\";i:1;s:3:\"uid\";i:2;s:7:\"viewnum\";i:3;s:8:\"replynum\";}}s:12:\"forum_thread\";a:4:{s:4:\"name\";s:12:\"甯栧瓙妯″潡\";s:6:\"script\";s:6:\"thread\";s:11:\"replacekeys\";a:2:{i:0;s:9:\"forumname\";i:1;s:8:\"forumurl\";}s:10:\"searchkeys\";a:2:{i:0;s:9:\"groupname\";i:1;s:8:\"groupurl\";}}}}s:6:\"portal\";a:2:{s:14:\"portal_article\";a:3:{s:12:\"forum_thread\";a:4:{s:4:\"name\";s:12:\"甯栧瓙妯″潡\";s:6:\"script\";s:6:\"thread\";s:10:\"searchkeys\";a:7:{i:0;s:8:\"username\";i:1;s:3:\"uid\";i:2;s:6:\"caturl\";i:3;s:7:\"catname\";i:4;s:8:\"articles\";i:5;s:7:\"viewnum\";i:6;s:10:\"commentnum\";}s:11:\"replacekeys\";a:7:{i:0;s:6:\"author\";i:1;s:8:\"authorid\";i:2;s:8:\"forumurl\";i:3;s:9:\"forumname\";i:4;s:5:\"posts\";i:5;s:5:\"views\";i:6;s:7:\"replies\";}}s:12:\"group_thread\";a:4:{s:4:\"name\";s:12:\"鍦堝瓙甯栧瓙\";s:6:\"script\";s:11:\"groupthread\";s:10:\"searchkeys\";a:7:{i:0;s:8:\"username\";i:1;s:3:\"uid\";i:2;s:6:\"caturl\";i:3;s:7:\"catname\";i:4;s:8:\"articles\";i:5;s:7:\"viewnum\";i:6;s:10:\"commentnum\";}s:11:\"replacekeys\";a:7:{i:0;s:6:\"author\";i:1;s:8:\"authorid\";i:2;s:8:\"groupurl\";i:3;s:9:\"groupname\";i:4;s:5:\"posts\";i:5;s:5:\"views\";i:6;s:7:\"replies\";}}s:10:\"space_blog\";a:4:{s:4:\"name\";s:12:\"鏃ュ織妯″潡\";s:6:\"script\";s:4:\"blog\";s:10:\"searchkeys\";a:1:{i:0;s:10:\"commentnum\";}s:11:\"replacekeys\";a:1:{i:0;s:8:\"replynum\";}}}s:15:\"portal_category\";a:2:{s:11:\"forum_forum\";a:4:{s:4:\"name\";s:12:\"鐗堝潡妯″潡\";s:6:\"script\";s:5:\"forum\";s:10:\"searchkeys\";a:1:{i:0;s:8:\"articles\";}s:11:\"replacekeys\";a:1:{i:0;s:7:\"threads\";}}s:11:\"group_group\";a:4:{s:4:\"name\";s:12:\"鍦堝瓙妯″潡\";s:6:\"script\";s:5:\"group\";s:10:\"searchkeys\";a:1:{i:0;s:8:\"articles\";}s:11:\"replacekeys\";a:1:{i:0;s:7:\"threads\";}}}}s:5:\"space\";a:1:{s:10:\"space_blog\";a:3:{s:12:\"forum_thread\";a:4:{s:4:\"name\";s:12:\"甯栧瓙妯″潡\";s:6:\"script\";s:6:\"thread\";s:10:\"searchkeys\";a:4:{i:0;s:8:\"username\";i:1;s:3:\"uid\";i:2;s:7:\"viewnum\";i:3;s:8:\"replynum\";}s:11:\"replacekeys\";a:4:{i:0;s:6:\"author\";i:1;s:8:\"authorid\";i:2;s:5:\"views\";i:3;s:7:\"replies\";}}s:12:\"group_thread\";a:4:{s:4:\"name\";s:12:\"鍦堝瓙甯栧瓙\";s:6:\"script\";s:11:\"groupthread\";s:10:\"searchkeys\";a:4:{i:0;s:8:\"username\";i:1;s:3:\"uid\";i:2;s:7:\"viewnum\";i:3;s:8:\"replynum\";}s:11:\"replacekeys\";a:4:{i:0;s:6:\"author\";i:1;s:8:\"authorid\";i:2;s:5:\"views\";i:3;s:7:\"replies\";}}s:14:\"portal_article\";a:4:{s:4:\"name\";s:12:\"鏂囩珷妯″潡\";s:6:\"script\";s:7:\"article\";s:10:\"searchkeys\";a:1:{i:0;s:8:\"replynum\";}s:11:\"replacekeys\";a:1:{i:0;s:10:\"commentnum\";}}}}}'),
('blockindex',1,1784453715,'a:3:{s:4:\"path\";a:8:{s:32:\"30afeffe3164bf5a01db8da559b617e9\";s:38:\"/app/public//source/class/block/forum/\";s:32:\"bdcbfe2fbc536a43ca98c893180eae32\";s:38:\"/app/public//source/class/block/group/\";s:32:\"59101d99b91d5be1d896c55af301a0b6\";s:37:\"/app/public//source/class/block/html/\";s:32:\"3e1eb1d84de9563d9d521741769935e2\";s:39:\"/app/public//source/class/block/member/\";s:32:\"830789094c40a1d620eb1fa01cbba794\";s:38:\"/app/public//source/class/block/other/\";s:32:\"f3d0463a004b668642e6f45ff8fbf2cf\";s:39:\"/app/public//source/class/block/portal/\";s:32:\"9a2f7b5b9a7bd44b52560ec763bb4cce\";s:38:\"/app/public//source/class/block/space/\";s:32:\"2467369e5ac5d4d6de0e2dfa8446c2a8\";s:36:\"/app/public//source/class/block/xml/\";}s:5:\"class\";a:21:{s:12:\"forum_thread\";s:5:\"forum\";s:11:\"forum_trade\";s:5:\"forum\";s:14:\"forum_activity\";s:5:\"forum\";s:11:\"forum_forum\";s:5:\"forum\";s:11:\"group_group\";s:5:\"group\";s:14:\"group_activity\";s:5:\"group\";s:11:\"group_trade\";s:5:\"group\";s:12:\"group_thread\";s:5:\"group\";s:9:\"html_html\";s:4:\"html\";s:12:\"html_misctag\";s:4:\"html\";s:17:\"html_announcement\";s:4:\"html\";s:13:\"member_member\";s:6:\"member\";s:15:\"other_otherstat\";s:5:\"other\";s:21:\"other_otherfriendlink\";s:5:\"other\";s:14:\"portal_article\";s:6:\"portal\";s:12:\"portal_topic\";s:6:\"portal\";s:15:\"portal_category\";s:6:\"portal\";s:9:\"space_pic\";s:5:\"space\";s:11:\"space_doing\";s:5:\"space\";s:10:\"space_blog\";s:5:\"space\";s:11:\"space_album\";s:5:\"space\";}s:3:\"sub\";a:78:{s:19:\"forum_thread.thread\";s:32:\"30afeffe3164bf5a01db8da559b617e9\";s:20:\"forum_trade.tradenew\";s:32:\"30afeffe3164bf5a01db8da559b617e9\";s:23:\"forum_activity.activity\";s:32:\"30afeffe3164bf5a01db8da559b617e9\";s:22:\"forum_thread.threadnew\";s:32:\"30afeffe3164bf5a01db8da559b617e9\";s:17:\"forum_forum.forum\";s:32:\"30afeffe3164bf5a01db8da559b617e9\";s:26:\"forum_thread.threadspecial\";s:32:\"30afeffe3164bf5a01db8da559b617e9\";s:25:\"forum_thread.threaddigest\";s:32:\"30afeffe3164bf5a01db8da559b617e9\";s:22:\"forum_thread.threadhot\";s:32:\"30afeffe3164bf5a01db8da559b617e9\";s:26:\"forum_trade.tradespecified\";s:32:\"30afeffe3164bf5a01db8da559b617e9\";s:28:\"forum_thread.threadspecified\";s:32:\"30afeffe3164bf5a01db8da559b617e9\";s:26:\"forum_activity.activitynew\";s:32:\"30afeffe3164bf5a01db8da559b617e9\";s:27:\"forum_activity.activitycity\";s:32:\"30afeffe3164bf5a01db8da559b617e9\";s:24:\"forum_thread.threadstick\";s:32:\"30afeffe3164bf5a01db8da559b617e9\";s:20:\"forum_trade.tradehot\";s:32:\"30afeffe3164bf5a01db8da559b617e9\";s:17:\"forum_trade.trade\";s:32:\"30afeffe3164bf5a01db8da559b617e9\";s:26:\"group_group.groupspecified\";s:32:\"bdcbfe2fbc536a43ca98c893180eae32\";s:28:\"group_activity.groupactivity\";s:32:\"bdcbfe2fbc536a43ca98c893180eae32\";s:31:\"group_trade.grouptradespecified\";s:32:\"bdcbfe2fbc536a43ca98c893180eae32\";s:25:\"group_trade.grouptradenew\";s:32:\"bdcbfe2fbc536a43ca98c893180eae32\";s:31:\"group_thread.groupthreadspecial\";s:32:\"bdcbfe2fbc536a43ca98c893180eae32\";s:24:\"group_thread.groupthread\";s:32:\"bdcbfe2fbc536a43ca98c893180eae32\";s:31:\"group_activity.groupactivitynew\";s:32:\"bdcbfe2fbc536a43ca98c893180eae32\";s:25:\"group_trade.grouptradehot\";s:32:\"bdcbfe2fbc536a43ca98c893180eae32\";s:20:\"group_group.grouphot\";s:32:\"bdcbfe2fbc536a43ca98c893180eae32\";s:27:\"group_thread.groupthreadnew\";s:32:\"bdcbfe2fbc536a43ca98c893180eae32\";s:22:\"group_trade.grouptrade\";s:32:\"bdcbfe2fbc536a43ca98c893180eae32\";s:20:\"group_group.groupnew\";s:32:\"bdcbfe2fbc536a43ca98c893180eae32\";s:33:\"group_thread.groupthreadspecified\";s:32:\"bdcbfe2fbc536a43ca98c893180eae32\";s:17:\"group_group.group\";s:32:\"bdcbfe2fbc536a43ca98c893180eae32\";s:27:\"group_thread.groupthreadhot\";s:32:\"bdcbfe2fbc536a43ca98c893180eae32\";s:32:\"group_activity.groupactivitycity\";s:32:\"bdcbfe2fbc536a43ca98c893180eae32\";s:13:\"html_html.api\";s:32:\"59101d99b91d5be1d896c55af301a0b6\";s:14:\"html_html.sort\";s:32:\"59101d99b91d5be1d896c55af301a0b6\";s:18:\"html_html.witframe\";s:32:\"59101d99b91d5be1d896c55af301a0b6\";s:16:\"html_html.google\";s:32:\"59101d99b91d5be1d896c55af301a0b6\";s:19:\"html_html.forumtree\";s:32:\"59101d99b91d5be1d896c55af301a0b6\";s:16:\"html_html.banner\";s:32:\"59101d99b91d5be1d896c55af301a0b6\";s:15:\"html_html.blank\";s:32:\"59101d99b91d5be1d896c55af301a0b6\";s:20:\"html_misctag.misctag\";s:32:\"59101d99b91d5be1d896c55af301a0b6\";s:30:\"html_announcement.announcement\";s:32:\"59101d99b91d5be1d896c55af301a0b6\";s:13:\"html_html.adv\";s:32:\"59101d99b91d5be1d896c55af301a0b6\";s:14:\"html_html.stat\";s:32:\"59101d99b91d5be1d896c55af301a0b6\";s:16:\"html_html.search\";s:32:\"59101d99b91d5be1d896c55af301a0b6\";s:15:\"html_html.vedio\";s:32:\"59101d99b91d5be1d896c55af301a0b6\";s:20:\"html_html.friendlink\";s:32:\"59101d99b91d5be1d896c55af301a0b6\";s:14:\"html_html.line\";s:32:\"59101d99b91d5be1d896c55af301a0b6\";s:25:\"member_member.memberposts\";s:32:\"3e1eb1d84de9563d9d521741769935e2\";s:23:\"member_member.membernew\";s:32:\"3e1eb1d84de9563d9d521741769935e2\";s:20:\"member_member.member\";s:32:\"3e1eb1d84de9563d9d521741769935e2\";s:26:\"member_member.membercredit\";s:32:\"3e1eb1d84de9563d9d521741769935e2\";s:27:\"member_member.memberspecial\";s:32:\"3e1eb1d84de9563d9d521741769935e2\";s:29:\"member_member.memberspecified\";s:32:\"3e1eb1d84de9563d9d521741769935e2\";s:24:\"member_member.membershow\";s:32:\"3e1eb1d84de9563d9d521741769935e2\";s:25:\"other_otherstat.otherstat\";s:32:\"830789094c40a1d620eb1fa01cbba794\";s:37:\"other_otherfriendlink.otherfriendlink\";s:32:\"830789094c40a1d620eb1fa01cbba794\";s:31:\"portal_article.articlespecified\";s:32:\"f3d0463a004b668642e6f45ff8fbf2cf\";s:22:\"portal_article.article\";s:32:\"f3d0463a004b668642e6f45ff8fbf2cf\";s:18:\"portal_topic.topic\";s:32:\"f3d0463a004b668642e6f45ff8fbf2cf\";s:30:\"portal_category.portalcategory\";s:32:\"f3d0463a004b668642e6f45ff8fbf2cf\";s:21:\"portal_topic.topicnew\";s:32:\"f3d0463a004b668642e6f45ff8fbf2cf\";s:27:\"portal_topic.topicspecified\";s:32:\"f3d0463a004b668642e6f45ff8fbf2cf\";s:25:\"portal_article.articlenew\";s:32:\"f3d0463a004b668642e6f45ff8fbf2cf\";s:21:\"portal_topic.topichot\";s:32:\"f3d0463a004b668642e6f45ff8fbf2cf\";s:25:\"portal_article.articlehot\";s:32:\"f3d0463a004b668642e6f45ff8fbf2cf\";s:13:\"space_pic.pic\";s:32:\"9a2f7b5b9a7bd44b52560ec763bb4cce\";s:20:\"space_doing.doinghot\";s:32:\"9a2f7b5b9a7bd44b52560ec763bb4cce\";s:18:\"space_blog.blognew\";s:32:\"9a2f7b5b9a7bd44b52560ec763bb4cce\";s:22:\"space_pic.picspecified\";s:32:\"9a2f7b5b9a7bd44b52560ec763bb4cce\";s:17:\"space_doing.doing\";s:32:\"9a2f7b5b9a7bd44b52560ec763bb4cce\";s:20:\"space_doing.doingnew\";s:32:\"9a2f7b5b9a7bd44b52560ec763bb4cce\";s:20:\"space_album.albumnew\";s:32:\"9a2f7b5b9a7bd44b52560ec763bb4cce\";s:16:\"space_pic.picnew\";s:32:\"9a2f7b5b9a7bd44b52560ec763bb4cce\";s:24:\"space_blog.blogspecified\";s:32:\"9a2f7b5b9a7bd44b52560ec763bb4cce\";s:26:\"space_album.albumspecified\";s:32:\"9a2f7b5b9a7bd44b52560ec763bb4cce\";s:18:\"space_blog.bloghot\";s:32:\"9a2f7b5b9a7bd44b52560ec763bb4cce\";s:15:\"space_blog.blog\";s:32:\"9a2f7b5b9a7bd44b52560ec763bb4cce\";s:17:\"space_album.album\";s:32:\"9a2f7b5b9a7bd44b52560ec763bb4cce\";s:16:\"space_pic.pichot\";s:32:\"9a2f7b5b9a7bd44b52560ec763bb4cce\";}}'),
('blogcategory',1,1784453715,'a:0:{}'),
('censor',1,1784453715,'a:3:{s:6:\"filter\";a:0:{}s:6:\"banned\";a:0:{}s:3:\"mod\";a:0:{}}'),
('click',1,1784453715,'a:3:{s:6:\"blogid\";a:5:{i:1;a:6:{s:7:\"clickid\";s:1:\"1\";s:4:\"name\";s:6:\"璺繃\";s:4:\"icon\";s:28:\"static/image/click/luguo.gif\";s:6:\"idtype\";s:6:\"blogid\";s:9:\"available\";s:1:\"1\";s:12:\"displayorder\";s:1:\"0\";}i:2;a:6:{s:7:\"clickid\";s:1:\"5\";s:4:\"name\";s:6:\"楦¤泲\";s:4:\"icon\";s:28:\"static/image/click/jidan.gif\";s:6:\"idtype\";s:6:\"blogid\";s:9:\"available\";s:1:\"1\";s:12:\"displayorder\";s:1:\"0\";}i:3;a:6:{s:7:\"clickid\";s:1:\"4\";s:4:\"name\";s:6:\"椴滆姳\";s:4:\"icon\";s:30:\"static/image/click/xianhua.gif\";s:6:\"idtype\";s:6:\"blogid\";s:9:\"available\";s:1:\"1\";s:12:\"displayorder\";s:1:\"0\";}i:4;a:6:{s:7:\"clickid\";s:1:\"3\";s:4:\"name\";s:6:\"鎻℃墜\";s:4:\"icon\";s:29:\"static/image/click/woshou.gif\";s:6:\"idtype\";s:6:\"blogid\";s:9:\"available\";s:1:\"1\";s:12:\"displayorder\";s:1:\"0\";}i:5;a:6:{s:7:\"clickid\";s:1:\"2\";s:4:\"name\";s:6:\"闆蜂汉\";s:4:\"icon\";s:29:\"static/image/click/leiren.gif\";s:6:\"idtype\";s:6:\"blogid\";s:9:\"available\";s:1:\"1\";s:12:\"displayorder\";s:1:\"0\";}}s:3:\"aid\";a:5:{i:1;a:6:{s:7:\"clickid\";s:2:\"14\";s:4:\"name\";s:6:\"椴滆姳\";s:4:\"icon\";s:30:\"static/image/click/xianhua.gif\";s:6:\"idtype\";s:3:\"aid\";s:9:\"available\";s:1:\"1\";s:12:\"displayorder\";s:1:\"0\";}i:2;a:6:{s:7:\"clickid\";s:2:\"13\";s:4:\"name\";s:6:\"鎻℃墜\";s:4:\"icon\";s:29:\"static/image/click/woshou.gif\";s:6:\"idtype\";s:3:\"aid\";s:9:\"available\";s:1:\"1\";s:12:\"displayorder\";s:1:\"0\";}i:3;a:6:{s:7:\"clickid\";s:2:\"12\";s:4:\"name\";s:6:\"闆蜂汉\";s:4:\"icon\";s:29:\"static/image/click/leiren.gif\";s:6:\"idtype\";s:3:\"aid\";s:9:\"available\";s:1:\"1\";s:12:\"displayorder\";s:1:\"0\";}i:4;a:6:{s:7:\"clickid\";s:2:\"11\";s:4:\"name\";s:6:\"璺繃\";s:4:\"icon\";s:28:\"static/image/click/luguo.gif\";s:6:\"idtype\";s:3:\"aid\";s:9:\"available\";s:1:\"1\";s:12:\"displayorder\";s:1:\"0\";}i:5;a:6:{s:7:\"clickid\";s:2:\"15\";s:4:\"name\";s:6:\"楦¤泲\";s:4:\"icon\";s:28:\"static/image/click/jidan.gif\";s:6:\"idtype\";s:3:\"aid\";s:9:\"available\";s:1:\"1\";s:12:\"displayorder\";s:1:\"0\";}}s:5:\"picid\";a:5:{i:1;a:6:{s:7:\"clickid\";s:2:\"10\";s:4:\"name\";s:6:\"楦¤泲\";s:4:\"icon\";s:28:\"static/image/click/jidan.gif\";s:6:\"idtype\";s:5:\"picid\";s:9:\"available\";s:1:\"1\";s:12:\"displayorder\";s:1:\"0\";}i:2;a:6:{s:7:\"clickid\";s:1:\"9\";s:4:\"name\";s:6:\"椴滆姳\";s:4:\"icon\";s:30:\"static/image/click/xianhua.gif\";s:6:\"idtype\";s:5:\"picid\";s:9:\"available\";s:1:\"1\";s:12:\"displayorder\";s:1:\"0\";}i:3;a:6:{s:7:\"clickid\";s:1:\"8\";s:4:\"name\";s:6:\"闆蜂汉\";s:4:\"icon\";s:29:\"static/image/click/leiren.gif\";s:6:\"idtype\";s:5:\"picid\";s:9:\"available\";s:1:\"1\";s:12:\"displayorder\";s:1:\"0\";}i:4;a:6:{s:7:\"clickid\";s:1:\"7\";s:4:\"name\";s:6:\"閰锋瘷\";s:4:\"icon\";s:27:\"static/image/click/kubi.gif\";s:6:\"idtype\";s:5:\"picid\";s:9:\"available\";s:1:\"1\";s:12:\"displayorder\";s:1:\"0\";}i:5;a:6:{s:7:\"clickid\";s:1:\"6\";s:4:\"name\";s:6:\"婕備寒\";s:4:\"icon\";s:32:\"static/image/click/piaoliang.gif\";s:6:\"idtype\";s:5:\"picid\";s:9:\"available\";s:1:\"1\";s:12:\"displayorder\";s:1:\"0\";}}}'),
('creditrule',1,1784453715,'a:28:{s:4:\"post\";a:17:{s:3:\"rid\";s:1:\"1\";s:8:\"rulename\";s:12:\"鍙戣〃涓婚\";s:6:\"action\";s:4:\"post\";s:9:\"cycletype\";s:1:\"4\";s:9:\"cycletime\";s:1:\"0\";s:9:\"rewardnum\";s:1:\"0\";s:8:\"norepeat\";s:1:\"0\";s:11:\"extcredits1\";s:1:\"0\";s:11:\"extcredits2\";s:1:\"2\";s:11:\"extcredits3\";s:1:\"0\";s:11:\"extcredits4\";s:1:\"0\";s:11:\"extcredits5\";s:1:\"0\";s:11:\"extcredits6\";s:1:\"0\";s:11:\"extcredits7\";s:1:\"0\";s:11:\"extcredits8\";s:1:\"0\";s:4:\"fids\";s:0:\"\";s:11:\"rulenameuni\";s:36:\"%E5%8F%91%E8%A1%A8%E4%B8%BB%E9%A2%98\";}s:5:\"reply\";a:17:{s:3:\"rid\";s:1:\"2\";s:8:\"rulename\";s:12:\"鍙戣〃鍥炲\";s:6:\"action\";s:5:\"reply\";s:9:\"cycletype\";s:1:\"4\";s:9:\"cycletime\";s:1:\"0\";s:9:\"rewardnum\";s:1:\"0\";s:8:\"norepeat\";s:1:\"0\";s:11:\"extcredits1\";s:1:\"0\";s:11:\"extcredits2\";s:1:\"1\";s:11:\"extcredits3\";s:1:\"0\";s:11:\"extcredits4\";s:1:\"0\";s:11:\"extcredits5\";s:1:\"0\";s:11:\"extcredits6\";s:1:\"0\";s:11:\"extcredits7\";s:1:\"0\";s:11:\"extcredits8\";s:1:\"0\";s:4:\"fids\";s:0:\"\";s:11:\"rulenameuni\";s:36:\"%E5%8F%91%E8%A1%A8%E5%9B%9E%E5%A4%8D\";}s:6:\"digest\";a:17:{s:3:\"rid\";s:1:\"3\";s:8:\"rulename\";s:9:\"鍔犵簿鍗嶾";s:6:\"action\";s:6:\"digest\";s:9:\"cycletype\";s:1:\"4\";s:9:\"cycletime\";s:1:\"0\";s:9:\"rewardnum\";s:1:\"0\";s:8:\"norepeat\";s:1:\"0\";s:11:\"extcredits1\";s:1:\"0\";s:11:\"extcredits2\";s:1:\"5\";s:11:\"extcredits3\";s:1:\"0\";s:11:\"extcredits4\";s:1:\"0\";s:11:\"extcredits5\";s:1:\"0\";s:11:\"extcredits6\";s:1:\"0\";s:11:\"extcredits7\";s:1:\"0\";s:11:\"extcredits8\";s:1:\"0\";s:4:\"fids\";s:0:\"\";s:11:\"rulenameuni\";s:27:\"%E5%8A%A0%E7%B2%BE%E5%8D%8E\";}s:10:\"postattach\";a:17:{s:3:\"rid\";s:1:\"4\";s:8:\"rulename\";s:12:\"涓婁紶闄勪欢\";s:6:\"action\";s:10:\"postattach\";s:9:\"cycletype\";s:1:\"4\";s:9:\"cycletime\";s:1:\"0\";s:9:\"rewardnum\";s:1:\"0\";s:8:\"norepeat\";s:1:\"0\";s:11:\"extcredits1\";s:1:\"0\";s:11:\"extcredits2\";s:1:\"0\";s:11:\"extcredits3\";s:1:\"0\";s:11:\"extcredits4\";s:1:\"0\";s:11:\"extcredits5\";s:1:\"0\";s:11:\"extcredits6\";s:1:\"0\";s:11:\"extcredits7\";s:1:\"0\";s:11:\"extcredits8\";s:1:\"0\";s:4:\"fids\";s:0:\"\";s:11:\"rulenameuni\";s:36:\"%E4%B8%8A%E4%BC%A0%E9%99%84%E4%BB%B6\";}s:9:\"getattach\";a:17:{s:3:\"rid\";s:1:\"5\";s:8:\"rulename\";s:12:\"涓嬭浇闄勪欢\";s:6:\"action\";s:9:\"getattach\";s:9:\"cycletype\";s:1:\"4\";s:9:\"cycletime\";s:1:\"0\";s:9:\"rewardnum\";s:1:\"0\";s:8:\"norepeat\";s:1:\"0\";s:11:\"extcredits1\";s:1:\"0\";s:11:\"extcredits2\";s:1:\"0\";s:11:\"extcredits3\";s:1:\"0\";s:11:\"extcredits4\";s:1:\"0\";s:11:\"extcredits5\";s:1:\"0\";s:11:\"extcredits6\";s:1:\"0\";s:11:\"extcredits7\";s:1:\"0\";s:11:\"extcredits8\";s:1:\"0\";s:4:\"fids\";s:0:\"\";s:11:\"rulenameuni\";s:36:\"%E4%B8%8B%E8%BD%BD%E9%99%84%E4%BB%B6\";}s:6:\"sendpm\";a:17:{s:3:\"rid\";s:1:\"6\";s:8:\"rulename\";s:12:\"鍙戠煭娑堟伅\";s:6:\"action\";s:6:\"sendpm\";s:9:\"cycletype\";s:1:\"4\";s:9:\"cycletime\";s:1:\"0\";s:9:\"rewardnum\";s:1:\"0\";s:8:\"norepeat\";s:1:\"0\";s:11:\"extcredits1\";s:1:\"0\";s:11:\"extcredits2\";s:1:\"0\";s:11:\"extcredits3\";s:1:\"0\";s:11:\"extcredits4\";s:1:\"0\";s:11:\"extcredits5\";s:1:\"0\";s:11:\"extcredits6\";s:1:\"0\";s:11:\"extcredits7\";s:1:\"0\";s:11:\"extcredits8\";s:1:\"0\";s:4:\"fids\";s:0:\"\";s:11:\"rulenameuni\";s:36:\"%E5%8F%91%E7%9F%AD%E6%B6%88%E6%81%AF\";}s:6:\"search\";a:17:{s:3:\"rid\";s:1:\"7\";s:8:\"rulename\";s:6:\"鎼滅储\";s:6:\"action\";s:6:\"search\";s:9:\"cycletype\";s:1:\"4\";s:9:\"cycletime\";s:1:\"0\";s:9:\"rewardnum\";s:1:\"0\";s:8:\"norepeat\";s:1:\"0\";s:11:\"extcredits1\";s:1:\"0\";s:11:\"extcredits2\";s:1:\"0\";s:11:\"extcredits3\";s:1:\"0\";s:11:\"extcredits4\";s:1:\"0\";s:11:\"extcredits5\";s:1:\"0\";s:11:\"extcredits6\";s:1:\"0\";s:11:\"extcredits7\";s:1:\"0\";s:11:\"extcredits8\";s:1:\"0\";s:4:\"fids\";s:0:\"\";s:11:\"rulenameuni\";s:18:\"%E6%90%9C%E7%B4%A2\";}s:15:\"promotion_visit\";a:17:{s:3:\"rid\";s:1:\"8\";s:8:\"rulename\";s:12:\"璁块棶鎺ㄥ箍\";s:6:\"action\";s:15:\"promotion_visit\";s:9:\"cycletype\";s:1:\"1\";s:9:\"cycletime\";s:1:\"0\";s:9:\"rewardnum\";s:1:\"1\";s:8:\"norepeat\";s:1:\"0\";s:11:\"extcredits1\";s:1:\"0\";s:11:\"extcredits2\";s:1:\"0\";s:11:\"extcredits3\";s:1:\"0\";s:11:\"extcredits4\";s:1:\"0\";s:11:\"extcredits5\";s:1:\"0\";s:11:\"extcredits6\";s:1:\"0\";s:11:\"extcredits7\";s:1:\"0\";s:11:\"extcredits8\";s:1:\"0\";s:4:\"fids\";s:0:\"\";s:11:\"rulenameuni\";s:36:\"%E8%AE%BF%E9%97%AE%E6%8E%A8%E5%B9%BF\";}s:18:\"promotion_register\";a:17:{s:3:\"rid\";s:1:\"9\";s:8:\"rulename\";s:12:\"娉ㄥ唽鎺ㄥ箍\";s:6:\"action\";s:18:\"promotion_register\";s:9:\"cycletype\";s:1:\"1\";s:9:\"cycletime\";s:1:\"0\";s:9:\"rewardnum\";s:1:\"1\";s:8:\"norepeat\";s:1:\"0\";s:11:\"extcredits1\";s:1:\"0\";s:11:\"extcredits2\";s:1:\"0\";s:11:\"extcredits3\";s:1:\"0\";s:11:\"extcredits4\";s:1:\"0\";s:11:\"extcredits5\";s:1:\"0\";s:11:\"extcredits6\";s:1:\"0\";s:11:\"extcredits7\";s:1:\"0\";s:11:\"extcredits8\";s:1:\"0\";s:4:\"fids\";s:0:\"\";s:11:\"rulenameuni\";s:36:\"%E6%B3%A8%E5%86%8C%E6%8E%A8%E5%B9%BF\";}s:13:\"tradefinished\";a:17:{s:3:\"rid\";s:2:\"10\";s:8:\"rulename\";s:12:\"鎴愬姛浜ゆ槗\";s:6:\"action\";s:13:\"tradefinished\";s:9:\"cycletype\";s:1:\"4\";s:9:\"cycletime\";s:1:\"0\";s:9:\"rewardnum\";s:1:\"0\";s:8:\"norepeat\";s:1:\"0\";s:11:\"extcredits1\";s:1:\"0\";s:11:\"extcredits2\";s:1:\"0\";s:11:\"extcredits3\";s:1:\"0\";s:11:\"extcredits4\";s:1:\"0\";s:11:\"extcredits5\";s:1:\"0\";s:11:\"extcredits6\";s:1:\"0\";s:11:\"extcredits7\";s:1:\"0\";s:11:\"extcredits8\";s:1:\"0\";s:4:\"fids\";s:0:\"\";s:11:\"rulenameuni\";s:36:\"%E6%88%90%E5%8A%9F%E4%BA%A4%E6%98%93\";}s:9:\"realemail\";a:17:{s:3:\"rid\";s:2:\"11\";s:8:\"rulename\";s:12:\"閭璁よ瘉\";s:6:\"action\";s:9:\"realemail\";s:9:\"cycletype\";s:1:\"0\";s:9:\"cycletime\";s:1:\"0\";s:9:\"rewardnum\";s:1:\"1\";s:8:\"norepeat\";s:1:\"0\";s:11:\"extcredits1\";s:1:\"0\";s:11:\"extcredits2\";s:2:\"10\";s:11:\"extcredits3\";s:1:\"0\";s:11:\"extcredits4\";s:1:\"0\";s:11:\"extcredits5\";s:1:\"0\";s:11:\"extcredits6\";s:1:\"0\";s:11:\"extcredits7\";s:1:\"0\";s:11:\"extcredits8\";s:1:\"0\";s:4:\"fids\";s:0:\"\";s:11:\"rulenameuni\";s:36:\"%E9%82%AE%E7%AE%B1%E8%AE%A4%E8%AF%81\";}s:9:\"setavatar\";a:17:{s:3:\"rid\";s:2:\"12\";s:8:\"rulename\";s:12:\"璁剧疆澶村儚\";s:6:\"action\";s:9:\"setavatar\";s:9:\"cycletype\";s:1:\"0\";s:9:\"cycletime\";s:1:\"0\";s:9:\"rewardnum\";s:1:\"1\";s:8:\"norepeat\";s:1:\"0\";s:11:\"extcredits1\";s:1:\"0\";s:11:\"extcredits2\";s:1:\"5\";s:11:\"extcredits3\";s:1:\"0\";s:11:\"extcredits4\";s:1:\"0\";s:11:\"extcredits5\";s:1:\"0\";s:11:\"extcredits6\";s:1:\"0\";s:11:\"extcredits7\";s:1:\"0\";s:11:\"extcredits8\";s:1:\"0\";s:4:\"fids\";s:0:\"\";s:11:\"rulenameuni\";s:36:\"%E8%AE%BE%E7%BD%AE%E5%A4%B4%E5%83%8F\";}s:7:\"hotinfo\";a:17:{s:3:\"rid\";s:2:\"14\";s:8:\"rulename\";s:12:\"鐑偣淇℃伅\";s:6:\"action\";s:7:\"hotinfo\";s:9:\"cycletype\";s:1:\"4\";s:9:\"cycletime\";s:1:\"0\";s:9:\"rewardnum\";s:1:\"0\";s:8:\"norepeat\";s:1:\"0\";s:11:\"extcredits1\";s:1:\"0\";s:11:\"extcredits2\";s:1:\"0\";s:11:\"extcredits3\";s:1:\"0\";s:11:\"extcredits4\";s:1:\"0\";s:11:\"extcredits5\";s:1:\"0\";s:11:\"extcredits6\";s:1:\"0\";s:11:\"extcredits7\";s:1:\"0\";s:11:\"extcredits8\";s:1:\"0\";s:4:\"fids\";s:0:\"\";s:11:\"rulenameuni\";s:36:\"%E7%83%AD%E7%82%B9%E4%BF%A1%E6%81%AF\";}s:8:\"daylogin\";a:17:{s:3:\"rid\";s:2:\"15\";s:8:\"rulename\";s:12:\"姣忓ぉ鐧诲綍\";s:6:\"action\";s:8:\"daylogin\";s:9:\"cycletype\";s:1:\"1\";s:9:\"cycletime\";s:1:\"0\";s:9:\"rewardnum\";s:1:\"1\";s:8:\"norepeat\";s:1:\"0\";s:11:\"extcredits1\";s:1:\"0\";s:11:\"extcredits2\";s:1:\"2\";s:11:\"extcredits3\";s:1:\"0\";s:11:\"extcredits4\";s:1:\"0\";s:11:\"extcredits5\";s:1:\"0\";s:11:\"extcredits6\";s:1:\"0\";s:11:\"extcredits7\";s:1:\"0\";s:11:\"extcredits8\";s:1:\"0\";s:4:\"fids\";s:0:\"\";s:11:\"rulenameuni\";s:36:\"%E6%AF%8F%E5%A4%A9%E7%99%BB%E5%BD%95\";}s:5:\"visit\";a:17:{s:3:\"rid\";s:2:\"16\";s:8:\"rulename\";s:18:\"璁块棶鍒汉绌洪棿\";s:6:\"action\";s:5:\"visit\";s:9:\"cycletype\";s:1:\"1\";s:9:\"cycletime\";s:1:\"0\";s:9:\"rewardnum\";s:2:\"10\";s:8:\"norepeat\";s:1:\"2\";s:11:\"extcredits1\";s:1:\"0\";s:11:\"extcredits2\";s:1:\"0\";s:11:\"extcredits3\";s:1:\"0\";s:11:\"extcredits4\";s:1:\"0\";s:11:\"extcredits5\";s:1:\"0\";s:11:\"extcredits6\";s:1:\"0\";s:11:\"extcredits7\";s:1:\"0\";s:11:\"extcredits8\";s:1:\"0\";s:4:\"fids\";s:0:\"\";s:11:\"rulenameuni\";s:54:\"%E8%AE%BF%E9%97%AE%E5%88%AB%E4%BA%BA%E7%A9%BA%E9%97%B4\";}s:4:\"poke\";a:17:{s:3:\"rid\";s:2:\"17\";s:8:\"rulename\";s:9:\"鎵撴嫑鍛糪";s:6:\"action\";s:4:\"poke\";s:9:\"cycletype\";s:1:\"1\";s:9:\"cycletime\";s:1:\"0\";s:9:\"rewardnum\";s:2:\"10\";s:8:\"norepeat\";s:1:\"2\";s:11:\"extcredits1\";s:1:\"0\";s:11:\"extcredits2\";s:1:\"0\";s:11:\"extcredits3\";s:1:\"0\";s:11:\"extcredits4\";s:1:\"0\";s:11:\"extcredits5\";s:1:\"0\";s:11:\"extcredits6\";s:1:\"0\";s:11:\"extcredits7\";s:1:\"0\";s:11:\"extcredits8\";s:1:\"0\";s:4:\"fids\";s:0:\"\";s:11:\"rulenameuni\";s:27:\"%E6%89%93%E6%8B%9B%E5%91%BC\";}s:9:\"guestbook\";a:17:{s:3:\"rid\";s:2:\"18\";s:8:\"rulename\";s:6:\"鐣欒█\";s:6:\"action\";s:9:\"guestbook\";s:9:\"cycletype\";s:1:\"1\";s:9:\"cycletime\";s:1:\"0\";s:9:\"rewardnum\";s:2:\"20\";s:8:\"norepeat\";s:1:\"2\";s:11:\"extcredits1\";s:1:\"0\";s:11:\"extcredits2\";s:1:\"1\";s:11:\"extcredits3\";s:1:\"0\";s:11:\"extcredits4\";s:1:\"0\";s:11:\"extcredits5\";s:1:\"0\";s:11:\"extcredits6\";s:1:\"0\";s:11:\"extcredits7\";s:1:\"0\";s:11:\"extcredits8\";s:1:\"0\";s:4:\"fids\";s:0:\"\";s:11:\"rulenameuni\";s:18:\"%E7%95%99%E8%A8%80\";}s:12:\"getguestbook\";a:17:{s:3:\"rid\";s:2:\"19\";s:8:\"rulename\";s:9:\"琚暀瑷€\";s:6:\"action\";s:12:\"getguestbook\";s:9:\"cycletype\";s:1:\"1\";s:9:\"cycletime\";s:1:\"0\";s:9:\"rewardnum\";s:1:\"5\";s:8:\"norepeat\";s:1:\"2\";s:11:\"extcredits1\";s:1:\"0\";s:11:\"extcredits2\";s:1:\"1\";s:11:\"extcredits3\";s:1:\"0\";s:11:\"extcredits4\";s:1:\"0\";s:11:\"extcredits5\";s:1:\"0\";s:11:\"extcredits6\";s:1:\"0\";s:11:\"extcredits7\";s:1:\"0\";s:11:\"extcredits8\";s:1:\"0\";s:4:\"fids\";s:0:\"\";s:11:\"rulenameuni\";s:27:\"%E8%A2%AB%E7%95%99%E8%A8%80\";}s:5:\"doing\";a:17:{s:3:\"rid\";s:2:\"20\";s:8:\"rulename\";s:12:\"鍙戣〃璁板綍\";s:6:\"action\";s:5:\"doing\";s:9:\"cycletype\";s:1:\"1\";s:9:\"cycletime\";s:1:\"0\";s:9:\"rewardnum\";s:1:\"5\";s:8:\"norepeat\";s:1:\"0\";s:11:\"extcredits1\";s:1:\"0\";s:11:\"extcredits2\";s:1:\"1\";s:11:\"extcredits3\";s:1:\"0\";s:11:\"extcredits4\";s:1:\"0\";s:11:\"extcredits5\";s:1:\"0\";s:11:\"extcredits6\";s:1:\"0\";s:11:\"extcredits7\";s:1:\"0\";s:11:\"extcredits8\";s:1:\"0\";s:4:\"fids\";s:0:\"\";s:11:\"rulenameuni\";s:36:\"%E5%8F%91%E8%A1%A8%E8%AE%B0%E5%BD%95\";}s:11:\"publishblog\";a:17:{s:3:\"rid\";s:2:\"21\";s:8:\"rulename\";s:12:\"鍙戣〃鏃ュ織\";s:6:\"action\";s:11:\"publishblog\";s:9:\"cycletype\";s:1:\"1\";s:9:\"cycletime\";s:1:\"0\";s:9:\"rewardnum\";s:1:\"3\";s:8:\"norepeat\";s:1:\"0\";s:11:\"extcredits1\";s:1:\"0\";s:11:\"extcredits2\";s:1:\"2\";s:11:\"extcredits3\";s:1:\"0\";s:11:\"extcredits4\";s:1:\"0\";s:11:\"extcredits5\";s:1:\"0\";s:11:\"extcredits6\";s:1:\"0\";s:11:\"extcredits7\";s:1:\"0\";s:11:\"extcredits8\";s:1:\"0\";s:4:\"fids\";s:0:\"\";s:11:\"rulenameuni\";s:36:\"%E5%8F%91%E8%A1%A8%E6%97%A5%E5%BF%97\";}s:8:\"joinpoll\";a:17:{s:3:\"rid\";s:2:\"22\";s:8:\"rulename\";s:12:\"鍙備笌鎶曠エ\";s:6:\"action\";s:8:\"joinpoll\";s:9:\"cycletype\";s:1:\"1\";s:9:\"cycletime\";s:1:\"0\";s:9:\"rewardnum\";s:2:\"10\";s:8:\"norepeat\";s:1:\"1\";s:11:\"extcredits1\";s:1:\"0\";s:11:\"extcredits2\";s:1:\"1\";s:11:\"extcredits3\";s:1:\"0\";s:11:\"extcredits4\";s:1:\"0\";s:11:\"extcredits5\";s:1:\"0\";s:11:\"extcredits6\";s:1:\"0\";s:11:\"extcredits7\";s:1:\"0\";s:11:\"extcredits8\";s:1:\"0\";s:4:\"fids\";s:0:\"\";s:11:\"rulenameuni\";s:36:\"%E5%8F%82%E4%B8%8E%E6%8A%95%E7%A5%A8\";}s:11:\"createshare\";a:17:{s:3:\"rid\";s:2:\"23\";s:8:\"rulename\";s:12:\"鍙戣捣鍒嗕韩\";s:6:\"action\";s:11:\"createshare\";s:9:\"cycletype\";s:1:\"1\";s:9:\"cycletime\";s:1:\"0\";s:9:\"rewardnum\";s:1:\"3\";s:8:\"norepeat\";s:1:\"0\";s:11:\"extcredits1\";s:1:\"0\";s:11:\"extcredits2\";s:1:\"1\";s:11:\"extcredits3\";s:1:\"0\";s:11:\"extcredits4\";s:1:\"0\";s:11:\"extcredits5\";s:1:\"0\";s:11:\"extcredits6\";s:1:\"0\";s:11:\"extcredits7\";s:1:\"0\";s:11:\"extcredits8\";s:1:\"0\";s:4:\"fids\";s:0:\"\";s:11:\"rulenameuni\";s:36:\"%E5%8F%91%E8%B5%B7%E5%88%86%E4%BA%AB\";}s:7:\"comment\";a:17:{s:3:\"rid\";s:2:\"24\";s:8:\"rulename\";s:6:\"璇勮\";s:6:\"action\";s:7:\"comment\";s:9:\"cycletype\";s:1:\"1\";s:9:\"cycletime\";s:1:\"0\";s:9:\"rewardnum\";s:2:\"40\";s:8:\"norepeat\";s:1:\"1\";s:11:\"extcredits1\";s:1:\"0\";s:11:\"extcredits2\";s:1:\"1\";s:11:\"extcredits3\";s:1:\"0\";s:11:\"extcredits4\";s:1:\"0\";s:11:\"extcredits5\";s:1:\"0\";s:11:\"extcredits6\";s:1:\"0\";s:11:\"extcredits7\";s:1:\"0\";s:11:\"extcredits8\";s:1:\"0\";s:4:\"fids\";s:0:\"\";s:11:\"rulenameuni\";s:18:\"%E8%AF%84%E8%AE%BA\";}s:10:\"getcomment\";a:17:{s:3:\"rid\";s:2:\"25\";s:8:\"rulename\";s:9:\"琚瘎璁篭";s:6:\"action\";s:10:\"getcomment\";s:9:\"cycletype\";s:1:\"1\";s:9:\"cycletime\";s:1:\"0\";s:9:\"rewardnum\";s:2:\"20\";s:8:\"norepeat\";s:1:\"1\";s:11:\"extcredits1\";s:1:\"0\";s:11:\"extcredits2\";s:1:\"2\";s:11:\"extcredits3\";s:1:\"0\";s:11:\"extcredits4\";s:1:\"0\";s:11:\"extcredits5\";s:1:\"0\";s:11:\"extcredits6\";s:1:\"0\";s:11:\"extcredits7\";s:1:\"0\";s:11:\"extcredits8\";s:1:\"0\";s:4:\"fids\";s:0:\"\";s:11:\"rulenameuni\";s:27:\"%E8%A2%AB%E8%AF%84%E8%AE%BA\";}s:5:\"click\";a:17:{s:3:\"rid\";s:2:\"28\";s:8:\"rulename\";s:12:\"淇℃伅琛ㄦ€乗";s:6:\"action\";s:5:\"click\";s:9:\"cycletype\";s:1:\"1\";s:9:\"cycletime\";s:1:\"0\";s:9:\"rewardnum\";s:2:\"10\";s:8:\"norepeat\";s:1:\"1\";s:11:\"extcredits1\";s:1:\"0\";s:11:\"extcredits2\";s:1:\"0\";s:11:\"extcredits3\";s:1:\"0\";s:11:\"extcredits4\";s:1:\"0\";s:11:\"extcredits5\";s:1:\"0\";s:11:\"extcredits6\";s:1:\"0\";s:11:\"extcredits7\";s:1:\"0\";s:11:\"extcredits8\";s:1:\"0\";s:4:\"fids\";s:0:\"\";s:11:\"rulenameuni\";s:36:\"%E4%BF%A1%E6%81%AF%E8%A1%A8%E6%80%81\";}s:12:\"modifydomain\";a:17:{s:3:\"rid\";s:2:\"29\";s:8:\"rulename\";s:12:\"淇敼鍩熷悕\";s:6:\"action\";s:12:\"modifydomain\";s:9:\"cycletype\";s:1:\"0\";s:9:\"cycletime\";s:1:\"0\";s:9:\"rewardnum\";s:1:\"1\";s:8:\"norepeat\";s:1:\"0\";s:11:\"extcredits1\";s:1:\"0\";s:11:\"extcredits2\";s:1:\"0\";s:11:\"extcredits3\";s:1:\"0\";s:11:\"extcredits4\";s:1:\"0\";s:11:\"extcredits5\";s:1:\"0\";s:11:\"extcredits6\";s:1:\"0\";s:11:\"extcredits7\";s:1:\"0\";s:11:\"extcredits8\";s:1:\"0\";s:4:\"fids\";s:0:\"\";s:11:\"rulenameuni\";s:36:\"%E4%BF%AE%E6%94%B9%E5%9F%9F%E5%90%8D\";}s:13:\"portalcomment\";a:17:{s:3:\"rid\";s:2:\"30\";s:8:\"rulename\";s:12:\"鏂囩珷璇勮\";s:6:\"action\";s:13:\"portalcomment\";s:9:\"cycletype\";s:1:\"1\";s:9:\"cycletime\";s:1:\"0\";s:9:\"rewardnum\";s:2:\"40\";s:8:\"norepeat\";s:1:\"1\";s:11:\"extcredits1\";s:1:\"0\";s:11:\"extcredits2\";s:1:\"1\";s:11:\"extcredits3\";s:1:\"0\";s:11:\"extcredits4\";s:1:\"0\";s:11:\"extcredits5\";s:1:\"0\";s:11:\"extcredits6\";s:1:\"0\";s:11:\"extcredits7\";s:1:\"0\";s:11:\"extcredits8\";s:1:\"0\";s:4:\"fids\";s:0:\"\";s:11:\"rulenameuni\";s:36:\"%E6%96%87%E7%AB%A0%E8%AF%84%E8%AE%BA\";}s:18:\"followedcollection\";a:17:{s:3:\"rid\";s:2:\"31\";s:8:\"rulename\";s:18:\"娣樹笓杈戣璁㈤槄\";s:6:\"action\";s:18:\"followedcollection\";s:9:\"cycletype\";s:1:\"1\";s:9:\"cycletime\";s:1:\"0\";s:9:\"rewardnum\";s:1:\"3\";s:8:\"norepeat\";s:1:\"0\";s:11:\"extcredits1\";s:1:\"0\";s:11:\"extcredits2\";s:1:\"1\";s:11:\"extcredits3\";s:1:\"0\";s:11:\"extcredits4\";s:1:\"0\";s:11:\"extcredits5\";s:1:\"0\";s:11:\"extcredits6\";s:1:\"0\";s:11:\"extcredits7\";s:1:\"0\";s:11:\"extcredits8\";s:1:\"0\";s:4:\"fids\";s:0:\"\";s:11:\"rulenameuni\";s:54:\"%E6%B7%98%E4%B8%93%E8%BE%91%E8%A2%AB%E8%AE%A2%E9%98%85\";}}'),
('creditrule_sub',1,1784453715,'a:0:{}'),
('cronnextrun',0,1784453715,'1784457315'),
('custominfo',1,1784453715,'a:4:{s:9:\"fieldsadd\";s:0:\"\";s:7:\"setting\";a:1:{s:4:\"menu\";a:6:{s:5:\"posts\";s:0:\"\";s:6:\"digest\";s:0:\"\";s:7:\"credits\";s:0:\"\";s:8:\"readperm\";s:0:\"\";s:7:\"regtime\";s:0:\"\";s:8:\"lastdate\";s:0:\"\";}}s:7:\"profile\";a:0:{}s:6:\"postno\";a:5:{i:0;s:12:\"<sup>#</sup>\";i:1;s:6:\"妤间富\";i:2;s:6:\"娌欏彂\";i:3;s:6:\"鏉垮嚦\";i:4;s:6:\"鍦版澘\";}}'),
('diytemplatename',1,1784453715,'a:0:{}'),
('domain',1,1784453715,'a:5:{s:12:\"defaultindex\";s:9:\"forum.php\";s:10:\"holddomain\";s:24:\"www|*blog*|*space*|*bbs*\";s:4:\"list\";a:0:{}s:3:\"app\";a:5:{s:6:\"portal\";s:0:\"\";s:5:\"forum\";s:0:\"\";s:5:\"group\";s:0:\"\";s:4:\"home\";s:0:\"\";s:7:\"default\";s:0:\"\";}s:4:\"root\";a:5:{s:4:\"home\";s:0:\"\";s:5:\"group\";s:0:\"\";s:5:\"forum\";s:0:\"\";s:5:\"topic\";s:0:\"\";s:7:\"channel\";s:0:\"\";}}'),
('domainwhitelist',1,1784453715,'a:0:{}'),
('fields_optional',1,1784453715,'a:40:{s:13:\"field_address\";a:17:{s:7:\"fieldid\";s:7:\"address\";s:9:\"available\";s:1:\"1\";s:9:\"invisible\";s:1:\"1\";s:10:\"needverify\";s:1:\"0\";s:5:\"title\";s:12:\"閭瘎鍦板潃\";s:11:\"description\";s:0:\"\";s:12:\"displayorder\";s:1:\"0\";s:8:\"required\";s:1:\"0\";s:12:\"unchangeable\";s:1:\"0\";s:10:\"showincard\";s:1:\"0\";s:12:\"showinthread\";s:1:\"0\";s:14:\"showinregister\";s:1:\"0\";s:11:\"allowsearch\";s:1:\"0\";s:8:\"formtype\";s:4:\"text\";s:4:\"size\";s:1:\"0\";s:8:\"validate\";s:0:\"\";s:7:\"encrypt\";s:1:\"0\";}s:21:\"field_affectivestatus\";a:17:{s:7:\"fieldid\";s:15:\"affectivestatus\";s:9:\"available\";s:1:\"1\";s:9:\"invisible\";s:1:\"1\";s:10:\"needverify\";s:1:\"0\";s:5:\"title\";s:12:\"鎯呮劅鐘舵€乗";s:11:\"description\";s:0:\"\";s:12:\"displayorder\";s:1:\"0\";s:8:\"required\";s:1:\"0\";s:12:\"unchangeable\";s:1:\"0\";s:10:\"showincard\";s:1:\"0\";s:12:\"showinthread\";s:1:\"0\";s:14:\"showinregister\";s:1:\"0\";s:11:\"allowsearch\";s:1:\"0\";s:8:\"formtype\";s:4:\"text\";s:4:\"size\";s:1:\"0\";s:8:\"validate\";s:0:\"\";s:7:\"encrypt\";s:1:\"0\";}s:12:\"field_alipay\";a:17:{s:7:\"fieldid\";s:6:\"alipay\";s:9:\"available\";s:1:\"1\";s:9:\"invisible\";s:1:\"1\";s:10:\"needverify\";s:1:\"0\";s:5:\"title\";s:9:\"鏀粯瀹漒";s:11:\"description\";s:0:\"\";s:12:\"displayorder\";s:1:\"0\";s:8:\"required\";s:1:\"0\";s:12:\"unchangeable\";s:1:\"0\";s:10:\"showincard\";s:1:\"0\";s:12:\"showinthread\";s:1:\"0\";s:14:\"showinregister\";s:1:\"0\";s:11:\"allowsearch\";s:1:\"0\";s:8:\"formtype\";s:4:\"text\";s:4:\"size\";s:1:\"0\";s:8:\"validate\";s:0:\"\";s:7:\"encrypt\";s:1:\"0\";}s:9:\"field_bio\";a:17:{s:7:\"fieldid\";s:3:\"bio\";s:9:\"available\";s:1:\"1\";s:9:\"invisible\";s:1:\"1\";s:10:\"needverify\";s:1:\"0\";s:5:\"title\";s:12:\"鑷垜浠嬬粛\";s:11:\"description\";s:0:\"\";s:12:\"displayorder\";s:1:\"0\";s:8:\"required\";s:1:\"0\";s:12:\"unchangeable\";s:1:\"0\";s:10:\"showincard\";s:1:\"0\";s:12:\"showinthread\";s:1:\"0\";s:14:\"showinregister\";s:1:\"0\";s:11:\"allowsearch\";s:1:\"0\";s:8:\"formtype\";s:8:\"textarea\";s:4:\"size\";s:1:\"0\";s:8:\"validate\";s:0:\"\";s:7:\"encrypt\";s:1:\"0\";}s:15:\"field_birthcity\";a:17:{s:7:\"fieldid\";s:9:\"birthcity\";s:9:\"available\";s:1:\"1\";s:9:\"invisible\";s:1:\"0\";s:10:\"needverify\";s:1:\"0\";s:5:\"title\";s:9:\"鍑虹敓鍦癨";s:11:\"description\";s:0:\"\";s:12:\"displayorder\";s:1:\"0\";s:8:\"required\";s:1:\"0\";s:12:\"unchangeable\";s:1:\"0\";s:10:\"showincard\";s:1:\"0\";s:12:\"showinthread\";s:1:\"0\";s:14:\"showinregister\";s:1:\"0\";s:11:\"allowsearch\";s:1:\"0\";s:8:\"formtype\";s:6:\"select\";s:4:\"size\";s:1:\"0\";s:8:\"validate\";s:0:\"\";s:7:\"encrypt\";s:1:\"0\";}s:20:\"field_birthcommunity\";a:17:{s:7:\"fieldid\";s:14:\"birthcommunity\";s:9:\"available\";s:1:\"1\";s:9:\"invisible\";s:1:\"0\";s:10:\"needverify\";s:1:\"0\";s:5:\"title\";s:12:\"鍑虹敓灏忓尯\";s:11:\"description\";s:0:\"\";s:12:\"displayorder\";s:1:\"0\";s:8:\"required\";s:1:\"0\";s:12:\"unchangeable\";s:1:\"0\";s:10:\"showincard\";s:1:\"0\";s:12:\"showinthread\";s:1:\"0\";s:14:\"showinregister\";s:1:\"0\";s:11:\"allowsearch\";s:1:\"0\";s:8:\"formtype\";s:6:\"select\";s:4:\"size\";s:1:\"0\";s:8:\"validate\";s:0:\"\";s:7:\"encrypt\";s:1:\"0\";}s:18:\"field_birthcountry\";a:17:{s:7:\"fieldid\";s:12:\"birthcountry\";s:9:\"available\";s:1:\"1\";s:9:\"invisible\";s:1:\"0\";s:10:\"needverify\";s:1:\"0\";s:5:\"title\";s:12:\"鍑虹敓鍥藉\";s:11:\"description\";s:0:\"\";s:12:\"displayorder\";s:1:\"0\";s:8:\"required\";s:1:\"0\";s:12:\"unchangeable\";s:1:\"0\";s:10:\"showincard\";s:1:\"0\";s:12:\"showinthread\";s:1:\"0\";s:14:\"showinregister\";s:1:\"0\";s:11:\"allowsearch\";s:1:\"0\";s:8:\"formtype\";s:6:\"select\";s:4:\"size\";s:1:\"0\";s:8:\"validate\";s:0:\"\";s:7:\"encrypt\";s:1:\"0\";}s:14:\"field_birthday\";a:17:{s:7:\"fieldid\";s:8:\"birthday\";s:9:\"available\";s:1:\"1\";s:9:\"invisible\";s:1:\"0\";s:10:\"needverify\";s:1:\"0\";s:5:\"title\";s:6:\"鐢熸棩\";s:11:\"description\";s:0:\"\";s:12:\"displayorder\";s:1:\"0\";s:8:\"required\";s:1:\"0\";s:12:\"unchangeable\";s:1:\"0\";s:10:\"showincard\";s:1:\"0\";s:12:\"showinthread\";s:1:\"0\";s:14:\"showinregister\";s:1:\"0\";s:11:\"allowsearch\";s:1:\"0\";s:8:\"formtype\";s:6:\"select\";s:4:\"size\";s:1:\"0\";s:8:\"validate\";s:0:\"\";s:7:\"encrypt\";s:1:\"0\";}s:15:\"field_birthdist\";a:17:{s:7:\"fieldid\";s:9:\"birthdist\";s:9:\"available\";s:1:\"1\";s:9:\"invisible\";s:1:\"0\";s:10:\"needverify\";s:1:\"0\";s:5:\"title\";s:9:\"鍑虹敓鍘縗";s:11:\"description\";s:19:\"鍑虹敓琛屾斂鍖?鍘縗";s:12:\"displayorder\";s:1:\"0\";s:8:\"required\";s:1:\"0\";s:12:\"unchangeable\";s:1:\"0\";s:10:\"showincard\";s:1:\"0\";s:12:\"showinthread\";s:1:\"0\";s:14:\"showinregister\";s:1:\"0\";s:11:\"allowsearch\";s:1:\"0\";s:8:\"formtype\";s:6:\"select\";s:4:\"size\";s:1:\"0\";s:8:\"validate\";s:0:\"\";s:7:\"encrypt\";s:1:\"0\";}s:16:\"field_birthmonth\";a:17:{s:7:\"fieldid\";s:10:\"birthmonth\";s:9:\"available\";s:1:\"1\";s:9:\"invisible\";s:1:\"0\";s:10:\"needverify\";s:1:\"0\";s:5:\"title\";s:12:\"鍑虹敓鏈堜唤\";s:11:\"description\";s:0:\"\";s:12:\"displayorder\";s:1:\"0\";s:8:\"required\";s:1:\"0\";s:12:\"unchangeable\";s:1:\"0\";s:10:\"showincard\";s:1:\"0\";s:12:\"showinthread\";s:1:\"0\";s:14:\"showinregister\";s:1:\"0\";s:11:\"allowsearch\";s:1:\"0\";s:8:\"formtype\";s:6:\"select\";s:4:\"size\";s:1:\"0\";s:8:\"validate\";s:0:\"\";s:7:\"encrypt\";s:1:\"0\";}s:19:\"field_birthprovince\";a:17:{s:7:\"fieldid\";s:13:\"birthprovince\";s:9:\"available\";s:1:\"1\";s:9:\"invisible\";s:1:\"0\";s:10:\"needverify\";s:1:\"0\";s:5:\"title\";s:12:\"鍑虹敓鐪佷唤\";s:11:\"description\";s:0:\"\";s:12:\"displayorder\";s:1:\"0\";s:8:\"required\";s:1:\"0\";s:12:\"unchangeable\";s:1:\"0\";s:10:\"showincard\";s:1:\"0\";s:12:\"showinthread\";s:1:\"0\";s:14:\"showinregister\";s:1:\"0\";s:11:\"allowsearch\";s:1:\"0\";s:8:\"formtype\";s:6:\"select\";s:4:\"size\";s:1:\"0\";s:8:\"validate\";s:0:\"\";s:7:\"encrypt\";s:1:\"0\";}s:15:\"field_birthyear\";a:17:{s:7:\"fieldid\";s:9:\"birthyear\";s:9:\"available\";s:1:\"1\";s:9:\"invisible\";s:1:\"0\";s:10:\"needverify\";s:1:\"0\";s:5:\"title\";s:12:\"鍑虹敓骞翠唤\";s:11:\"description\";s:0:\"\";s:12:\"displayorder\";s:1:\"0\";s:8:\"required\";s:1:\"0\";s:12:\"unchangeable\";s:1:\"0\";s:10:\"showincard\";s:1:\"0\";s:12:\"showinthread\";s:1:\"0\";s:14:\"showinregister\";s:1:\"0\";s:11:\"allowsearch\";s:1:\"1\";s:8:\"formtype\";s:6:\"select\";s:4:\"size\";s:1:\"0\";s:8:\"validate\";s:0:\"\";s:7:\"encrypt\";s:1:\"0\";}s:15:\"field_bloodtype\";a:17:{s:7:\"fieldid\";s:9:\"bloodtype\";s:9:\"available\";s:1:\"1\";s:9:\"invisible\";s:1:\"1\";s:10:\"needverify\";s:1:\"0\";s:5:\"title\";s:6:\"琛€鍨媆";s:11:\"description\";s:0:\"\";s:12:\"displayorder\";s:1:\"0\";s:8:\"required\";s:1:\"0\";s:12:\"unchangeable\";s:1:\"0\";s:10:\"showincard\";s:1:\"0\";s:12:\"showinthread\";s:1:\"0\";s:14:\"showinregister\";s:1:\"0\";s:11:\"allowsearch\";s:1:\"0\";s:8:\"formtype\";s:6:\"select\";s:4:\"size\";s:1:\"0\";s:8:\"validate\";s:0:\"\";s:7:\"encrypt\";s:1:\"0\";}s:13:\"field_company\";a:17:{s:7:\"fieldid\";s:7:\"company\";s:9:\"available\";s:1:\"1\";s:9:\"invisible\";s:1:\"0\";s:10:\"needverify\";s:1:\"0\";s:5:\"title\";s:6:\"鍏徃\";s:11:\"description\";s:0:\"\";s:12:\"displayorder\";s:1:\"0\";s:8:\"required\";s:1:\"0\";s:12:\"unchangeable\";s:1:\"0\";s:10:\"showincard\";s:1:\"0\";s:12:\"showinthread\";s:1:\"0\";s:14:\"showinregister\";s:1:\"0\";s:11:\"allowsearch\";s:1:\"0\";s:8:\"formtype\";s:4:\"text\";s:4:\"size\";s:1:\"0\";s:8:\"validate\";s:0:\"\";s:7:\"encrypt\";s:1:\"0\";}s:19:\"field_constellation\";a:17:{s:7:\"fieldid\";s:13:\"constellation\";s:9:\"available\";s:1:\"1\";s:9:\"invisible\";s:1:\"1\";s:10:\"needverify\";s:1:\"0\";s:5:\"title\";s:6:\"鏄熷骇\";s:11:\"description\";s:32:\"鏄熷骇(鏍规嵁鐢熸棩鑷姩璁＄畻)\";s:12:\"displayorder\";s:1:\"0\";s:8:\"required\";s:1:\"0\";s:12:\"unchangeable\";s:1:\"0\";s:10:\"showincard\";s:1:\"0\";s:12:\"showinthread\";s:1:\"0\";s:14:\"showinregister\";s:1:\"0\";s:11:\"allowsearch\";s:1:\"0\";s:8:\"formtype\";s:4:\"text\";s:4:\"size\";s:1:\"0\";s:8:\"validate\";s:0:\"\";s:7:\"encrypt\";s:1:\"0\";}s:15:\"field_education\";a:17:{s:7:\"fieldid\";s:9:\"education\";s:9:\"available\";s:1:\"1\";s:9:\"invisible\";s:1:\"0\";s:10:\"needverify\";s:1:\"0\";s:5:\"title\";s:6:\"瀛﹀巻\";s:11:\"description\";s:0:\"\";s:12:\"displayorder\";s:1:\"0\";s:8:\"required\";s:1:\"0\";s:12:\"unchangeable\";s:1:\"0\";s:10:\"showincard\";s:1:\"0\";s:12:\"showinthread\";s:1:\"0\";s:14:\"showinregister\";s:1:\"0\";s:11:\"allowsearch\";s:1:\"0\";s:8:\"formtype\";s:6:\"select\";s:4:\"size\";s:1:\"0\";s:8:\"validate\";s:0:\"\";s:7:\"encrypt\";s:1:\"0\";}s:12:\"field_fields\";a:17:{s:7:\"fieldid\";s:6:\"fields\";s:9:\"available\";s:1:\"1\";s:9:\"invisible\";s:1:\"1\";s:10:\"needverify\";s:1:\"0\";s:5:\"title\";s:21:\"鏇村鑷畾涔夎祫鏂橽";s:11:\"description\";s:0:\"\";s:12:\"displayorder\";s:1:\"0\";s:8:\"required\";s:1:\"0\";s:12:\"unchangeable\";s:1:\"0\";s:10:\"showincard\";s:1:\"0\";s:12:\"showinthread\";s:1:\"0\";s:14:\"showinregister\";s:1:\"0\";s:11:\"allowsearch\";s:1:\"0\";s:8:\"formtype\";s:4:\"json\";s:4:\"size\";s:1:\"0\";s:8:\"validate\";s:0:\"\";s:7:\"encrypt\";s:1:\"0\";}s:12:\"field_gender\";a:17:{s:7:\"fieldid\";s:6:\"gender\";s:9:\"available\";s:1:\"1\";s:9:\"invisible\";s:1:\"0\";s:10:\"needverify\";s:1:\"0\";s:5:\"title\";s:6:\"鎬у埆\";s:11:\"description\";s:0:\"\";s:12:\"displayorder\";s:1:\"0\";s:8:\"required\";s:1:\"0\";s:12:\"unchangeable\";s:1:\"0\";s:10:\"showincard\";s:1:\"0\";s:12:\"showinthread\";s:1:\"0\";s:14:\"showinregister\";s:1:\"0\";s:11:\"allowsearch\";s:1:\"1\";s:8:\"formtype\";s:6:\"select\";s:4:\"size\";s:1:\"0\";s:8:\"validate\";s:0:\"\";s:7:\"encrypt\";s:1:\"0\";}s:20:\"field_graduateschool\";a:17:{s:7:\"fieldid\";s:14:\"graduateschool\";s:9:\"available\";s:1:\"1\";s:9:\"invisible\";s:1:\"0\";s:10:\"needverify\";s:1:\"0\";s:5:\"title\";s:12:\"姣曚笟瀛︽牎\";s:11:\"description\";s:0:\"\";s:12:\"displayorder\";s:1:\"0\";s:8:\"required\";s:1:\"0\";s:12:\"unchangeable\";s:1:\"0\";s:10:\"showincard\";s:1:\"0\";s:12:\"showinthread\";s:1:\"0\";s:14:\"showinregister\";s:1:\"0\";s:11:\"allowsearch\";s:1:\"0\";s:8:\"formtype\";s:4:\"text\";s:4:\"size\";s:1:\"0\";s:8:\"validate\";s:0:\"\";s:7:\"encrypt\";s:1:\"0\";}s:12:\"field_idcard\";a:17:{s:7:\"fieldid\";s:6:\"idcard\";s:9:\"available\";s:1:\"1\";s:9:\"invisible\";s:1:\"1\";s:10:\"needverify\";s:1:\"0\";s:5:\"title\";s:9:\"璇佷欢鍙穃";s:11:\"description\";s:0:\"\";s:12:\"displayorder\";s:1:\"0\";s:8:\"required\";s:1:\"0\";s:12:\"unchangeable\";s:1:\"0\";s:10:\"showincard\";s:1:\"0\";s:12:\"showinthread\";s:1:\"0\";s:14:\"showinregister\";s:1:\"0\";s:11:\"allowsearch\";s:1:\"0\";s:8:\"formtype\";s:4:\"text\";s:4:\"size\";s:1:\"0\";s:8:\"validate\";s:0:\"\";s:7:\"encrypt\";s:1:\"0\";}s:16:\"field_idcardtype\";a:17:{s:7:\"fieldid\";s:10:\"idcardtype\";s:9:\"available\";s:1:\"1\";s:9:\"invisible\";s:1:\"1\";s:10:\"needverify\";s:1:\"0\";s:5:\"title\";s:12:\"璇佷欢绫诲瀷\";s:11:\"description\";s:29:\"韬唤璇?鎶ょ収 椹鹃┒璇佺瓑\";s:12:\"displayorder\";s:1:\"0\";s:8:\"required\";s:1:\"0\";s:12:\"unchangeable\";s:1:\"0\";s:10:\"showincard\";s:1:\"0\";s:12:\"showinthread\";s:1:\"0\";s:14:\"showinregister\";s:1:\"0\";s:11:\"allowsearch\";s:1:\"0\";s:8:\"formtype\";s:6:\"select\";s:4:\"size\";s:1:\"0\";s:8:\"validate\";s:0:\"\";s:7:\"encrypt\";s:1:\"0\";}s:14:\"field_interest\";a:17:{s:7:\"fieldid\";s:8:\"interest\";s:9:\"available\";s:1:\"1\";s:9:\"invisible\";s:1:\"0\";s:10:\"needverify\";s:1:\"0\";s:5:\"title\";s:12:\"鍏磋叮鐖卞ソ\";s:11:\"description\";s:0:\"\";s:12:\"displayorder\";s:1:\"0\";s:8:\"required\";s:1:\"0\";s:12:\"unchangeable\";s:1:\"0\";s:10:\"showincard\";s:1:\"0\";s:12:\"showinthread\";s:1:\"0\";s:14:\"showinregister\";s:1:\"0\";s:11:\"allowsearch\";s:1:\"0\";s:8:\"formtype\";s:8:\"textarea\";s:4:\"size\";s:1:\"0\";s:8:\"validate\";s:0:\"\";s:7:\"encrypt\";s:1:\"0\";}s:16:\"field_lookingfor\";a:17:{s:7:\"fieldid\";s:10:\"lookingfor\";s:9:\"available\";s:1:\"1\";s:9:\"invisible\";s:1:\"0\";s:10:\"needverify\";s:1:\"0\";s:5:\"title\";s:12:\"浜ゅ弸鐩殑\";s:11:\"description\";s:39:\"甯屾湜鍦ㄧ綉绔欐壘鍒颁粈涔堟牱鐨勬湅鍙媆";s:12:\"displayorder\";s:1:\"0\";s:8:\"required\";s:1:\"0\";s:12:\"unchangeable\";s:1:\"0\";s:10:\"showincard\";s:1:\"0\";s:12:\"showinthread\";s:1:\"0\";s:14:\"showinregister\";s:1:\"0\";s:11:\"allowsearch\";s:1:\"0\";s:8:\"formtype\";s:4:\"text\";s:4:\"size\";s:1:\"0\";s:8:\"validate\";s:0:\"\";s:7:\"encrypt\";s:1:\"0\";}s:12:\"field_mobile\";a:17:{s:7:\"fieldid\";s:6:\"mobile\";s:9:\"available\";s:1:\"1\";s:9:\"invisible\";s:1:\"1\";s:10:\"needverify\";s:1:\"0\";s:5:\"title\";s:6:\"鎵嬫満\";s:11:\"description\";s:0:\"\";s:12:\"displayorder\";s:1:\"0\";s:8:\"required\";s:1:\"0\";s:12:\"unchangeable\";s:1:\"0\";s:10:\"showincard\";s:1:\"0\";s:12:\"showinthread\";s:1:\"0\";s:14:\"showinregister\";s:1:\"0\";s:11:\"allowsearch\";s:1:\"0\";s:8:\"formtype\";s:4:\"text\";s:4:\"size\";s:1:\"0\";s:8:\"validate\";s:0:\"\";s:7:\"encrypt\";s:1:\"0\";}s:9:\"field_msn\";a:17:{s:7:\"fieldid\";s:3:\"msn\";s:9:\"available\";s:1:\"1\";s:9:\"invisible\";s:1:\"1\";s:10:\"needverify\";s:1:\"0\";s:5:\"title\";s:3:\"MSN\";s:11:\"description\";s:0:\"\";s:12:\"displayorder\";s:1:\"0\";s:8:\"required\";s:1:\"0\";s:12:\"unchangeable\";s:1:\"0\";s:10:\"showincard\";s:1:\"0\";s:12:\"showinthread\";s:1:\"0\";s:14:\"showinregister\";s:1:\"0\";s:11:\"allowsearch\";s:1:\"0\";s:8:\"formtype\";s:4:\"text\";s:4:\"size\";s:1:\"0\";s:8:\"validate\";s:0:\"\";s:7:\"encrypt\";s:1:\"0\";}s:16:\"field_occupation\";a:17:{s:7:\"fieldid\";s:10:\"occupation\";s:9:\"available\";s:1:\"1\";s:9:\"invisible\";s:1:\"0\";s:10:\"needverify\";s:1:\"0\";s:5:\"title\";s:6:\"鑱屼笟\";s:11:\"description\";s:0:\"\";s:12:\"displayorder\";s:1:\"0\";s:8:\"required\";s:1:\"0\";s:12:\"unchangeable\";s:1:\"0\";s:10:\"showincard\";s:1:\"0\";s:12:\"showinthread\";s:1:\"0\";s:14:\"showinregister\";s:1:\"0\";s:11:\"allowsearch\";s:1:\"0\";s:8:\"formtype\";s:4:\"text\";s:4:\"size\";s:1:\"0\";s:8:\"validate\";s:0:\"\";s:7:\"encrypt\";s:1:\"0\";}s:14:\"field_position\";a:17:{s:7:\"fieldid\";s:8:\"position\";s:9:\"available\";s:1:\"1\";s:9:\"invisible\";s:1:\"0\";s:10:\"needverify\";s:1:\"0\";s:5:\"title\";s:6:\"鑱屼綅\";s:11:\"description\";s:0:\"\";s:12:\"displayorder\";s:1:\"0\";s:8:\"required\";s:1:\"0\";s:12:\"unchangeable\";s:1:\"0\";s:10:\"showincard\";s:1:\"0\";s:12:\"showinthread\";s:1:\"0\";s:14:\"showinregister\";s:1:\"0\";s:11:\"allowsearch\";s:1:\"0\";s:8:\"formtype\";s:4:\"text\";s:4:\"size\";s:1:\"0\";s:8:\"validate\";s:0:\"\";s:7:\"encrypt\";s:1:\"0\";}s:8:\"field_qq\";a:17:{s:7:\"fieldid\";s:2:\"qq\";s:9:\"available\";s:1:\"1\";s:9:\"invisible\";s:1:\"1\";s:10:\"needverify\";s:1:\"0\";s:5:\"title\";s:2:\"QQ\";s:11:\"description\";s:0:\"\";s:12:\"displayorder\";s:1:\"0\";s:8:\"required\";s:1:\"0\";s:12:\"unchangeable\";s:1:\"0\";s:10:\"showincard\";s:1:\"0\";s:12:\"showinthread\";s:1:\"0\";s:14:\"showinregister\";s:1:\"0\";s:11:\"allowsearch\";s:1:\"0\";s:8:\"formtype\";s:4:\"text\";s:4:\"size\";s:1:\"0\";s:8:\"validate\";s:0:\"\";s:7:\"encrypt\";s:1:\"0\";}s:14:\"field_realname\";a:17:{s:7:\"fieldid\";s:8:\"realname\";s:9:\"available\";s:1:\"1\";s:9:\"invisible\";s:1:\"0\";s:10:\"needverify\";s:1:\"0\";s:5:\"title\";s:12:\"鐪熷疄濮撳悕\";s:11:\"description\";s:0:\"\";s:12:\"displayorder\";s:1:\"0\";s:8:\"required\";s:1:\"0\";s:12:\"unchangeable\";s:1:\"0\";s:10:\"showincard\";s:1:\"0\";s:12:\"showinthread\";s:1:\"0\";s:14:\"showinregister\";s:1:\"0\";s:11:\"allowsearch\";s:1:\"1\";s:8:\"formtype\";s:4:\"text\";s:4:\"size\";s:1:\"0\";s:8:\"validate\";s:0:\"\";s:7:\"encrypt\";s:1:\"0\";}s:16:\"field_residecity\";a:17:{s:7:\"fieldid\";s:10:\"residecity\";s:9:\"available\";s:1:\"1\";s:9:\"invisible\";s:1:\"0\";s:10:\"needverify\";s:1:\"0\";s:5:\"title\";s:9:\"灞呬綇鍦癨";s:11:\"description\";s:0:\"\";s:12:\"displayorder\";s:1:\"0\";s:8:\"required\";s:1:\"0\";s:12:\"unchangeable\";s:1:\"0\";s:10:\"showincard\";s:1:\"0\";s:12:\"showinthread\";s:1:\"0\";s:14:\"showinregister\";s:1:\"0\";s:11:\"allowsearch\";s:1:\"0\";s:8:\"formtype\";s:6:\"select\";s:4:\"size\";s:1:\"0\";s:8:\"validate\";s:0:\"\";s:7:\"encrypt\";s:1:\"0\";}s:21:\"field_residecommunity\";a:17:{s:7:\"fieldid\";s:15:\"residecommunity\";s:9:\"available\";s:1:\"1\";s:9:\"invisible\";s:1:\"0\";s:10:\"needverify\";s:1:\"0\";s:5:\"title\";s:12:\"灞呬綇灏忓尯\";s:11:\"description\";s:0:\"\";s:12:\"displayorder\";s:1:\"0\";s:8:\"required\";s:1:\"0\";s:12:\"unchangeable\";s:1:\"0\";s:10:\"showincard\";s:1:\"0\";s:12:\"showinthread\";s:1:\"0\";s:14:\"showinregister\";s:1:\"0\";s:11:\"allowsearch\";s:1:\"0\";s:8:\"formtype\";s:6:\"select\";s:4:\"size\";s:1:\"0\";s:8:\"validate\";s:0:\"\";s:7:\"encrypt\";s:1:\"0\";}s:19:\"field_residecountry\";a:17:{s:7:\"fieldid\";s:13:\"residecountry\";s:9:\"available\";s:1:\"1\";s:9:\"invisible\";s:1:\"0\";s:10:\"needverify\";s:1:\"0\";s:5:\"title\";s:12:\"灞呬綇鍥藉\";s:11:\"description\";s:0:\"\";s:12:\"displayorder\";s:1:\"0\";s:8:\"required\";s:1:\"0\";s:12:\"unchangeable\";s:1:\"0\";s:10:\"showincard\";s:1:\"0\";s:12:\"showinthread\";s:1:\"0\";s:14:\"showinregister\";s:1:\"0\";s:11:\"allowsearch\";s:1:\"0\";s:8:\"formtype\";s:6:\"select\";s:4:\"size\";s:1:\"0\";s:8:\"validate\";s:0:\"\";s:7:\"encrypt\";s:1:\"0\";}s:16:\"field_residedist\";a:17:{s:7:\"fieldid\";s:10:\"residedist\";s:9:\"available\";s:1:\"1\";s:9:\"invisible\";s:1:\"0\";s:10:\"needverify\";s:1:\"0\";s:5:\"title\";s:9:\"灞呬綇鍘縗";s:11:\"description\";s:19:\"灞呬綇琛屾斂鍖?鍘縗";s:12:\"displayorder\";s:1:\"0\";s:8:\"required\";s:1:\"0\";s:12:\"unchangeable\";s:1:\"0\";s:10:\"showincard\";s:1:\"0\";s:12:\"showinthread\";s:1:\"0\";s:14:\"showinregister\";s:1:\"0\";s:11:\"allowsearch\";s:1:\"0\";s:8:\"formtype\";s:6:\"select\";s:4:\"size\";s:1:\"0\";s:8:\"validate\";s:0:\"\";s:7:\"encrypt\";s:1:\"0\";}s:20:\"field_resideprovince\";a:17:{s:7:\"fieldid\";s:14:\"resideprovince\";s:9:\"available\";s:1:\"1\";s:9:\"invisible\";s:1:\"0\";s:10:\"needverify\";s:1:\"0\";s:5:\"title\";s:12:\"灞呬綇鐪佷唤\";s:11:\"description\";s:0:\"\";s:12:\"displayorder\";s:1:\"0\";s:8:\"required\";s:1:\"0\";s:12:\"unchangeable\";s:1:\"0\";s:10:\"showincard\";s:1:\"0\";s:12:\"showinthread\";s:1:\"0\";s:14:\"showinregister\";s:1:\"0\";s:11:\"allowsearch\";s:1:\"0\";s:8:\"formtype\";s:6:\"select\";s:4:\"size\";s:1:\"0\";s:8:\"validate\";s:0:\"\";s:7:\"encrypt\";s:1:\"0\";}s:13:\"field_revenue\";a:17:{s:7:\"fieldid\";s:7:\"revenue\";s:9:\"available\";s:1:\"1\";s:9:\"invisible\";s:1:\"1\";s:10:\"needverify\";s:1:\"0\";s:5:\"title\";s:9:\"骞存敹鍏";s:11:\"description\";s:10:\"鍗曚綅 鍏僜";s:12:\"displayorder\";s:1:\"0\";s:8:\"required\";s:1:\"0\";s:12:\"unchangeable\";s:1:\"0\";s:10:\"showincard\";s:1:\"0\";s:12:\"showinthread\";s:1:\"0\";s:14:\"showinregister\";s:1:\"0\";s:11:\"allowsearch\";s:1:\"0\";s:8:\"formtype\";s:4:\"text\";s:4:\"size\";s:1:\"0\";s:8:\"validate\";s:0:\"\";s:7:\"encrypt\";s:1:\"0\";}s:10:\"field_site\";a:17:{s:7:\"fieldid\";s:4:\"site\";s:9:\"available\";s:1:\"1\";s:9:\"invisible\";s:1:\"0\";s:10:\"needverify\";s:1:\"0\";s:5:\"title\";s:12:\"涓汉涓婚〉\";s:11:\"description\";s:0:\"\";s:12:\"displayorder\";s:1:\"0\";s:8:\"required\";s:1:\"0\";s:12:\"unchangeable\";s:1:\"0\";s:10:\"showincard\";s:1:\"0\";s:12:\"showinthread\";s:1:\"0\";s:14:\"showinregister\";s:1:\"0\";s:11:\"allowsearch\";s:1:\"0\";s:8:\"formtype\";s:4:\"text\";s:4:\"size\";s:1:\"0\";s:8:\"validate\";s:0:\"\";s:7:\"encrypt\";s:1:\"0\";}s:12:\"field_taobao\";a:17:{s:7:\"fieldid\";s:6:\"taobao\";s:9:\"available\";s:1:\"1\";s:9:\"invisible\";s:1:\"1\";s:10:\"needverify\";s:1:\"0\";s:5:\"title\";s:12:\"闃块噷鏃烘椇\";s:11:\"description\";s:0:\"\";s:12:\"displayorder\";s:1:\"0\";s:8:\"required\";s:1:\"0\";s:12:\"unchangeable\";s:1:\"0\";s:10:\"showincard\";s:1:\"0\";s:12:\"showinthread\";s:1:\"0\";s:14:\"showinregister\";s:1:\"0\";s:11:\"allowsearch\";s:1:\"0\";s:8:\"formtype\";s:4:\"text\";s:4:\"size\";s:1:\"0\";s:8:\"validate\";s:0:\"\";s:7:\"encrypt\";s:1:\"0\";}s:15:\"field_telephone\";a:17:{s:7:\"fieldid\";s:9:\"telephone\";s:9:\"available\";s:1:\"1\";s:9:\"invisible\";s:1:\"1\";s:10:\"needverify\";s:1:\"0\";s:5:\"title\";s:12:\"鍥哄畾鐢佃瘽\";s:11:\"description\";s:0:\"\";s:12:\"displayorder\";s:1:\"0\";s:8:\"required\";s:1:\"0\";s:12:\"unchangeable\";s:1:\"0\";s:10:\"showincard\";s:1:\"0\";s:12:\"showinthread\";s:1:\"0\";s:14:\"showinregister\";s:1:\"0\";s:11:\"allowsearch\";s:1:\"0\";s:8:\"formtype\";s:4:\"text\";s:4:\"size\";s:1:\"0\";s:8:\"validate\";s:0:\"\";s:7:\"encrypt\";s:1:\"0\";}s:13:\"field_zipcode\";a:17:{s:7:\"fieldid\";s:7:\"zipcode\";s:9:\"available\";s:1:\"1\";s:9:\"invisible\";s:1:\"1\";s:10:\"needverify\";s:1:\"0\";s:5:\"title\";s:6:\"閭紪\";s:11:\"description\";s:0:\"\";s:12:\"displayorder\";s:1:\"0\";s:8:\"required\";s:1:\"0\";s:12:\"unchangeable\";s:1:\"0\";s:10:\"showincard\";s:1:\"0\";s:12:\"showinthread\";s:1:\"0\";s:14:\"showinregister\";s:1:\"0\";s:11:\"allowsearch\";s:1:\"0\";s:8:\"formtype\";s:4:\"text\";s:4:\"size\";s:1:\"0\";s:8:\"validate\";s:0:\"\";s:7:\"encrypt\";s:1:\"0\";}s:12:\"field_zodiac\";a:17:{s:7:\"fieldid\";s:6:\"zodiac\";s:9:\"available\";s:1:\"1\";s:9:\"invisible\";s:1:\"1\";s:10:\"needverify\";s:1:\"0\";s:5:\"title\";s:6:\"鐢熻倴\";s:11:\"description\";s:32:\"鐢熻倴(鏍规嵁鐢熸棩鑷姩璁＄畻)\";s:12:\"displayorder\";s:1:\"0\";s:8:\"required\";s:1:\"0\";s:12:\"unchangeable\";s:1:\"0\";s:10:\"showincard\";s:1:\"0\";s:12:\"showinthread\";s:1:\"0\";s:14:\"showinregister\";s:1:\"0\";s:11:\"allowsearch\";s:1:\"0\";s:8:\"formtype\";s:4:\"text\";s:4:\"size\";s:1:\"0\";s:8:\"validate\";s:0:\"\";s:7:\"encrypt\";s:1:\"0\";}}'),
('fields_register',1,1784453715,'a:0:{}'),
('fields_required',1,1784453715,'a:0:{}'),
('focus',1,1784453715,'a:3:{s:5:\"title\";s:12:\"绔欓暱鎺ㄨ崘\";s:6:\"cookie\";i:1;s:4:\"data\";a:0:{}}'),
('forumlinks',1,1784453715,'a:3:{i:0;s:308:\"<li class=\"lk_logo mbm bbda cl\"><img src=\"static/image/common/logo_88_31.gif\" border=\"0\" alt=\"Discuz! 瀹樻柟璁哄潧\" /><div class=\"lk_content z\"><h5><a href=\"https://www.discuz.vip/\" target=\"_blank\">Discuz! 瀹樻柟璁哄潧</a></h5><p>鎻愪緵鏈€鏂?Discuz! 浜у搧鏂伴椈銆佽蒋浠朵笅杞戒笌鎶€鏈氦娴?/p></div></li>\";i:1;s:0:\"\";i:2;s:115:\"<li><a href=\"https://addon.dismall.com/\" target=\"_blank\" title=\"Discuz! 搴旂敤涓績\">Discuz! 搴旂敤涓績</a></li>\";}'),
('forumrecommend',1,1784453715,'a:0:{}'),
('forums',1,1784453715,'a:2:{i:1;a:15:{s:3:\"fid\";s:1:\"1\";s:4:\"type\";s:5:\"group\";s:4:\"name\";s:7:\"Discuz!\";s:3:\"fup\";s:1:\"0\";s:8:\"viewperm\";s:0:\"\";s:8:\"postperm\";s:0:\"\";s:7:\"orderby\";s:8:\"lastpost\";s:7:\"ascdesc\";s:4:\"DESC\";s:6:\"status\";s:1:\"1\";s:5:\"extra\";a:0:{}s:6:\"plugin\";N;s:5:\"style\";N;s:7:\"archive\";s:1:\"0\";s:6:\"domain\";s:0:\"\";s:12:\"havepassword\";i:0;}i:2;a:18:{s:3:\"fid\";s:1:\"2\";s:4:\"type\";s:5:\"forum\";s:4:\"name\";s:12:\"榛樿鐗堝潡\";s:3:\"fup\";s:1:\"1\";s:8:\"viewperm\";s:0:\"\";s:8:\"postperm\";s:0:\"\";s:7:\"orderby\";s:8:\"lastpost\";s:7:\"ascdesc\";s:4:\"DESC\";s:5:\"users\";N;s:6:\"status\";s:1:\"1\";s:5:\"extra\";a:0:{}s:6:\"plugin\";N;s:5:\"style\";N;s:16:\"allowpostspecial\";s:6:\"000001\";s:11:\"commentitem\";s:0:\"\";s:7:\"archive\";s:1:\"0\";s:6:\"domain\";s:0:\"\";s:12:\"havepassword\";i:0;}}'),
('forumstick',1,1784453715,'a:0:{}'),
('globalstick',1,1784453715,'a:1:{s:6:\"global\";a:2:{s:4:\"tids\";s:0:\"\";s:5:\"count\";i:0;}}'),
('groupicon',1,1784453715,'a:4:{i:1;s:36:\"static/image/common/online_admin.gif\";i:2;s:39:\"static/image/common/online_supermod.gif\";i:3;s:40:\"static/image/common/online_moderator.gif\";i:0;s:37:\"static/image/common/online_member.gif\";}'),
('grouplevels',1,1784453715,'a:3:{i:1;a:9:{s:7:\"levelid\";s:1:\"1\";s:4:\"type\";s:7:\"default\";s:10:\"leveltitle\";s:9:\"鏅€氱骇\";s:13:\"creditshigher\";s:10:\"-999999999\";s:12:\"creditslower\";s:3:\"500\";s:4:\"icon\";s:0:\"\";s:13:\"creditspolicy\";a:2:{s:4:\"post\";s:1:\"1\";s:5:\"reply\";s:1:\"1\";}s:10:\"postpolicy\";a:11:{s:13:\"alloweditpost\";s:1:\"1\";s:10:\"recyclebin\";s:1:\"1\";s:12:\"allowsmilies\";s:1:\"1\";s:9:\"allowhtml\";i:0;s:11:\"allowbbcode\";s:1:\"1\";s:14:\"allowanonymous\";i:0;s:6:\"jammer\";i:0;s:12:\"allowimgcode\";s:1:\"1\";s:14:\"allowmediacode\";s:1:\"1\";s:16:\"allowpostspecial\";i:31;s:16:\"attachextensions\";s:7:\"jpg,gif\";}s:13:\"specialswitch\";a:5:{s:15:\"allowchangename\";s:1:\"1\";s:15:\"allowchangetype\";s:1:\"1\";s:15:\"allowclosegroup\";s:1:\"1\";s:15:\"allowthreadtype\";s:1:\"1\";s:13:\"membermaximum\";s:0:\"\";}}i:2;a:9:{s:7:\"levelid\";s:1:\"2\";s:4:\"type\";s:7:\"default\";s:10:\"leveltitle\";s:6:\"涓骇\";s:13:\"creditshigher\";s:3:\"500\";s:12:\"creditslower\";s:4:\"3000\";s:4:\"icon\";s:0:\"\";s:13:\"creditspolicy\";a:2:{s:4:\"post\";s:1:\"1\";s:5:\"reply\";s:1:\"1\";}s:10:\"postpolicy\";a:11:{s:13:\"alloweditpost\";s:1:\"1\";s:10:\"recyclebin\";s:1:\"1\";s:12:\"allowsmilies\";s:1:\"1\";s:9:\"allowhtml\";i:0;s:11:\"allowbbcode\";s:1:\"1\";s:14:\"allowanonymous\";i:0;s:6:\"jammer\";i:0;s:12:\"allowimgcode\";s:1:\"1\";s:14:\"allowmediacode\";s:1:\"1\";s:16:\"allowpostspecial\";i:31;s:16:\"attachextensions\";s:11:\"jpg,gif,rar\";}s:13:\"specialswitch\";b:0;}i:3;a:9:{s:7:\"levelid\";s:1:\"3\";s:4:\"type\";s:7:\"default\";s:10:\"leveltitle\";s:6:\"楂樼骇\";s:13:\"creditshigher\";s:4:\"3000\";s:12:\"creditslower\";s:9:\"999999999\";s:4:\"icon\";s:0:\"\";s:13:\"creditspolicy\";a:2:{s:4:\"post\";s:1:\"1\";s:5:\"reply\";s:1:\"1\";}s:10:\"postpolicy\";a:11:{s:13:\"alloweditpost\";s:1:\"1\";s:10:\"recyclebin\";s:1:\"1\";s:12:\"allowsmilies\";s:1:\"1\";s:9:\"allowhtml\";s:1:\"0\";s:11:\"allowbbcode\";s:1:\"1\";s:14:\"allowanonymous\";s:1:\"0\";s:6:\"jammer\";s:1:\"1\";s:12:\"allowimgcode\";s:1:\"1\";s:14:\"allowmediacode\";s:1:\"1\";s:16:\"allowpostspecial\";i:31;s:16:\"attachextensions\";s:31:\"jpg,gif,png,bmp,rar,doc,txt,zip\";}s:13:\"specialswitch\";b:0;}}'),
('groupreadaccess',1,1784453715,'a:14:{i:0;a:3:{s:7:\"groupid\";s:1:\"7\";s:10:\"readaccess\";s:1:\"1\";s:10:\"grouptitle\";s:6:\"娓稿\";}i:1;a:3:{s:7:\"groupid\";s:2:\"10\";s:10:\"readaccess\";s:2:\"10\";s:10:\"grouptitle\";s:12:\"鏂版墜涓婅矾\";}i:2;a:3:{s:7:\"groupid\";s:2:\"11\";s:10:\"readaccess\";s:2:\"20\";s:10:\"grouptitle\";s:12:\"娉ㄥ唽浼氬憳\";}i:3;a:3:{s:7:\"groupid\";s:2:\"12\";s:10:\"readaccess\";s:2:\"30\";s:10:\"grouptitle\";s:12:\"涓骇浼氬憳\";}i:4;a:3:{s:7:\"groupid\";s:2:\"13\";s:10:\"readaccess\";s:2:\"50\";s:10:\"grouptitle\";s:12:\"楂樼骇浼氬憳\";}i:5;a:3:{s:7:\"groupid\";s:2:\"14\";s:10:\"readaccess\";s:2:\"70\";s:10:\"grouptitle\";s:12:\"閲戠墝浼氬憳\";}i:6;a:3:{s:7:\"groupid\";s:2:\"15\";s:10:\"readaccess\";s:2:\"90\";s:10:\"grouptitle\";s:12:\"璁哄潧鍏冭€乗";}i:7;a:3:{s:7:\"groupid\";s:2:\"16\";s:10:\"readaccess\";s:3:\"100\";s:10:\"grouptitle\";s:12:\"瀹炰範鐗堜富\";}i:8;a:3:{s:7:\"groupid\";s:2:\"19\";s:10:\"readaccess\";s:3:\"100\";s:10:\"grouptitle\";s:9:\"瀹℃牳鍛榎";}i:9;a:3:{s:7:\"groupid\";s:1:\"3\";s:10:\"readaccess\";s:3:\"100\";s:10:\"grouptitle\";s:6:\"鐗堜富\";}i:10;a:3:{s:7:\"groupid\";s:1:\"2\";s:10:\"readaccess\";s:3:\"150\";s:10:\"grouptitle\";s:12:\"瓒呯骇鐗堜富\";}i:11;a:3:{s:7:\"groupid\";s:2:\"17\";s:10:\"readaccess\";s:3:\"150\";s:10:\"grouptitle\";s:12:\"缃戠珯缂栬緫\";}i:12;a:3:{s:7:\"groupid\";s:2:\"18\";s:10:\"readaccess\";s:3:\"200\";s:10:\"grouptitle\";s:15:\"淇℃伅鐩戝療鍛榎";}i:13;a:3:{s:7:\"groupid\";s:1:\"1\";s:10:\"readaccess\";s:3:\"200\";s:10:\"grouptitle\";s:9:\"绠＄悊鍛榎";}}'),
('grouptype',1,1784453715,'a:2:{s:5:\"first\";a:0:{}s:6:\"second\";a:0:{}}'),
('heats',1,1784453715,'a:0:{}'),
('ipctrl',1,1784453715,'a:2:{s:9:\"ipregctrl\";s:0:\"\";s:13:\"ipverifywhite\";s:0:\"\";}'),
('magics',1,1784453715,'a:0:{}'),
('medals',1,1784453715,'a:0:{}'),
('modreasons',1,1784453715,'a:9:{i:0;s:11:\"骞垮憡/SPAM\";i:1;s:12:\"鎭舵剰鐏屾按\";i:2;s:12:\"杩濊鍐呭\";i:3;s:12:\"鏂囦笉瀵归\";i:4;s:12:\"閲嶅鍙戝笘\";i:5;s:0:\"\";i:6;s:12:\"鎴戝緢璧炲悓\";i:7;s:12:\"绮惧搧鏂囩珷\";i:8;s:12:\"鍘熷垱鍐呭\";}'),
('onlinelist',1,1784453715,'a:5:{s:6:\"legend\";s:333:\"<img src=\"static/image/common/online_admin.gif\" /> 绠＄悊鍛?&nbsp; &nbsp; &nbsp; <img src=\"static/image/common/online_supermod.gif\" /> 瓒呯骇鐗堜富 &nbsp; &nbsp; &nbsp; <img src=\"static/image/common/online_moderator.gif\" /> 鐗堜富 &nbsp; &nbsp; &nbsp; <img src=\"static/image/common/online_member.gif\" /> 浼氬憳 &nbsp; &nbsp; &nbsp; \";i:1;s:36:\"static/image/common/online_admin.gif\";i:2;s:39:\"static/image/common/online_supermod.gif\";i:3;s:40:\"static/image/common/online_moderator.gif\";i:0;s:37:\"static/image/common/online_member.gif\";}'),
('plugin',1,1784453715,'a:0:{}'),
('pluginsetting',1,1784453715,'a:0:{}'),
('portalcategory',1,1784453715,'a:0:{}'),
('postimg',1,1784453715,'a:2:{s:6:\"hrline\";a:15:{i:0;a:1:{s:3:\"url\";s:5:\"3.gif\";}i:1;a:1:{s:3:\"url\";s:9:\"line9.png\";}i:2;a:1:{s:3:\"url\";s:5:\"1.gif\";}i:3;a:1:{s:3:\"url\";s:9:\"line1.png\";}i:4;a:1:{s:3:\"url\";s:5:\"0.gif\";}i:5;a:1:{s:3:\"url\";s:5:\"2.gif\";}i:6;a:1:{s:3:\"url\";s:5:\"4.gif\";}i:7;a:1:{s:3:\"url\";s:9:\"line7.png\";}i:8;a:1:{s:3:\"url\";s:9:\"line4.png\";}i:9;a:1:{s:3:\"url\";s:9:\"line2.png\";}i:10;a:1:{s:3:\"url\";s:9:\"line6.png\";}i:11;a:1:{s:3:\"url\";s:9:\"line5.png\";}i:12;a:1:{s:3:\"url\";s:5:\"5.gif\";}i:13;a:1:{s:3:\"url\";s:9:\"line3.png\";}i:14;a:1:{s:3:\"url\";s:9:\"line8.png\";}}s:6:\"postbg\";a:14:{i:0;a:1:{s:3:\"url\";s:7:\"bg4.png\";}i:1;a:1:{s:3:\"url\";s:7:\"bg5.png\";}i:2;a:1:{s:3:\"url\";s:5:\"3.jpg\";}i:3;a:1:{s:3:\"url\";s:7:\"bg1.png\";}i:4;a:1:{s:3:\"url\";s:5:\"2.jpg\";}i:5;a:1:{s:3:\"url\";s:7:\"bg3.png\";}i:6;a:1:{s:3:\"url\";s:8:\"bg10.png\";}i:7;a:1:{s:3:\"url\";s:5:\"0.gif\";}i:8;a:1:{s:3:\"url\";s:7:\"bg8.png\";}i:9;a:1:{s:3:\"url\";s:7:\"bg7.png\";}i:10;a:1:{s:3:\"url\";s:7:\"bg9.png\";}i:11;a:1:{s:3:\"url\";s:7:\"bg6.png\";}i:12;a:1:{s:3:\"url\";s:7:\"bg2.png\";}i:13;a:1:{s:3:\"url\";s:5:\"1.jpg\";}}}'),
('posttable_info',0,1784453715,''),
('posttableids',0,1784453715,''),
('profilesetting',1,1784453715,'a:40:{s:2:\"qq\";a:18:{s:7:\"fieldid\";s:2:\"qq\";s:9:\"available\";s:1:\"1\";s:9:\"invisible\";s:1:\"1\";s:10:\"needverify\";s:1:\"0\";s:5:\"title\";s:2:\"QQ\";s:11:\"description\";s:0:\"\";s:12:\"displayorder\";s:1:\"0\";s:8:\"required\";s:1:\"0\";s:12:\"unchangeable\";s:1:\"0\";s:10:\"showincard\";s:1:\"0\";s:12:\"showinthread\";s:1:\"0\";s:14:\"showinregister\";s:1:\"0\";s:11:\"allowsearch\";s:1:\"0\";s:8:\"formtype\";s:4:\"text\";s:4:\"size\";s:1:\"0\";s:7:\"choices\";s:0:\"\";s:8:\"validate\";s:0:\"\";s:7:\"encrypt\";s:1:\"0\";}s:3:\"bio\";a:18:{s:7:\"fieldid\";s:3:\"bio\";s:9:\"available\";s:1:\"1\";s:9:\"invisible\";s:1:\"1\";s:10:\"needverify\";s:1:\"0\";s:5:\"title\";s:12:\"鑷垜浠嬬粛\";s:11:\"description\";s:0:\"\";s:12:\"displayorder\";s:1:\"0\";s:8:\"required\";s:1:\"0\";s:12:\"unchangeable\";s:1:\"0\";s:10:\"showincard\";s:1:\"0\";s:12:\"showinthread\";s:1:\"0\";s:14:\"showinregister\";s:1:\"0\";s:11:\"allowsearch\";s:1:\"0\";s:8:\"formtype\";s:8:\"textarea\";s:4:\"size\";s:1:\"0\";s:7:\"choices\";s:0:\"\";s:8:\"validate\";s:0:\"\";s:7:\"encrypt\";s:1:\"0\";}s:3:\"msn\";a:18:{s:7:\"fieldid\";s:3:\"msn\";s:9:\"available\";s:1:\"1\";s:9:\"invisible\";s:1:\"1\";s:10:\"needverify\";s:1:\"0\";s:5:\"title\";s:3:\"MSN\";s:11:\"description\";s:0:\"\";s:12:\"displayorder\";s:1:\"0\";s:8:\"required\";s:1:\"0\";s:12:\"unchangeable\";s:1:\"0\";s:10:\"showincard\";s:1:\"0\";s:12:\"showinthread\";s:1:\"0\";s:14:\"showinregister\";s:1:\"0\";s:11:\"allowsearch\";s:1:\"0\";s:8:\"formtype\";s:4:\"text\";s:4:\"size\";s:1:\"0\";s:7:\"choices\";s:0:\"\";s:8:\"validate\";s:0:\"\";s:7:\"encrypt\";s:1:\"0\";}s:4:\"site\";a:18:{s:7:\"fieldid\";s:4:\"site\";s:9:\"available\";s:1:\"1\";s:9:\"invisible\";s:1:\"0\";s:10:\"needverify\";s:1:\"0\";s:5:\"title\";s:12:\"涓汉涓婚〉\";s:11:\"description\";s:0:\"\";s:12:\"displayorder\";s:1:\"0\";s:8:\"required\";s:1:\"0\";s:12:\"unchangeable\";s:1:\"0\";s:10:\"showincard\";s:1:\"0\";s:12:\"showinthread\";s:1:\"0\";s:14:\"showinregister\";s:1:\"0\";s:11:\"allowsearch\";s:1:\"0\";s:8:\"formtype\";s:4:\"text\";s:4:\"size\";s:1:\"0\";s:7:\"choices\";s:0:\"\";s:8:\"validate\";s:0:\"\";s:7:\"encrypt\";s:1:\"0\";}s:6:\"alipay\";a:18:{s:7:\"fieldid\";s:6:\"alipay\";s:9:\"available\";s:1:\"1\";s:9:\"invisible\";s:1:\"1\";s:10:\"needverify\";s:1:\"0\";s:5:\"title\";s:9:\"鏀粯瀹漒";s:11:\"description\";s:0:\"\";s:12:\"displayorder\";s:1:\"0\";s:8:\"required\";s:1:\"0\";s:12:\"unchangeable\";s:1:\"0\";s:10:\"showincard\";s:1:\"0\";s:12:\"showinthread\";s:1:\"0\";s:14:\"showinregister\";s:1:\"0\";s:11:\"allowsearch\";s:1:\"0\";s:8:\"formtype\";s:4:\"text\";s:4:\"size\";s:1:\"0\";s:7:\"choices\";s:0:\"\";s:8:\"validate\";s:0:\"\";s:7:\"encrypt\";s:1:\"0\";}s:6:\"fields\";a:18:{s:7:\"fieldid\";s:6:\"fields\";s:9:\"available\";s:1:\"1\";s:9:\"invisible\";s:1:\"1\";s:10:\"needverify\";s:1:\"0\";s:5:\"title\";s:21:\"鏇村鑷畾涔夎祫鏂橽";s:11:\"description\";s:0:\"\";s:12:\"displayorder\";s:1:\"0\";s:8:\"required\";s:1:\"0\";s:12:\"unchangeable\";s:1:\"0\";s:10:\"showincard\";s:1:\"0\";s:12:\"showinthread\";s:1:\"0\";s:14:\"showinregister\";s:1:\"0\";s:11:\"allowsearch\";s:1:\"0\";s:8:\"formtype\";s:4:\"json\";s:4:\"size\";s:1:\"0\";s:7:\"choices\";s:0:\"\";s:8:\"validate\";s:0:\"\";s:7:\"encrypt\";s:1:\"0\";}s:6:\"gender\";a:18:{s:7:\"fieldid\";s:6:\"gender\";s:9:\"available\";s:1:\"1\";s:9:\"invisible\";s:1:\"0\";s:10:\"needverify\";s:1:\"0\";s:5:\"title\";s:6:\"鎬у埆\";s:11:\"description\";s:0:\"\";s:12:\"displayorder\";s:1:\"0\";s:8:\"required\";s:1:\"0\";s:12:\"unchangeable\";s:1:\"0\";s:10:\"showincard\";s:1:\"0\";s:12:\"showinthread\";s:1:\"0\";s:14:\"showinregister\";s:1:\"0\";s:11:\"allowsearch\";s:1:\"1\";s:8:\"formtype\";s:6:\"select\";s:4:\"size\";s:1:\"0\";s:7:\"choices\";s:0:\"\";s:8:\"validate\";s:0:\"\";s:7:\"encrypt\";s:1:\"0\";}s:6:\"idcard\";a:18:{s:7:\"fieldid\";s:6:\"idcard\";s:9:\"available\";s:1:\"1\";s:9:\"invisible\";s:1:\"1\";s:10:\"needverify\";s:1:\"0\";s:5:\"title\";s:9:\"璇佷欢鍙穃";s:11:\"description\";s:0:\"\";s:12:\"displayorder\";s:1:\"0\";s:8:\"required\";s:1:\"0\";s:12:\"unchangeable\";s:1:\"0\";s:10:\"showincard\";s:1:\"0\";s:12:\"showinthread\";s:1:\"0\";s:14:\"showinregister\";s:1:\"0\";s:11:\"allowsearch\";s:1:\"0\";s:8:\"formtype\";s:4:\"text\";s:4:\"size\";s:1:\"0\";s:7:\"choices\";s:0:\"\";s:8:\"validate\";s:0:\"\";s:7:\"encrypt\";s:1:\"0\";}s:6:\"mobile\";a:18:{s:7:\"fieldid\";s:6:\"mobile\";s:9:\"available\";s:1:\"1\";s:9:\"invisible\";s:1:\"1\";s:10:\"needverify\";s:1:\"0\";s:5:\"title\";s:6:\"鎵嬫満\";s:11:\"description\";s:0:\"\";s:12:\"displayorder\";s:1:\"0\";s:8:\"required\";s:1:\"0\";s:12:\"unchangeable\";s:1:\"0\";s:10:\"showincard\";s:1:\"0\";s:12:\"showinthread\";s:1:\"0\";s:14:\"showinregister\";s:1:\"0\";s:11:\"allowsearch\";s:1:\"0\";s:8:\"formtype\";s:4:\"text\";s:4:\"size\";s:1:\"0\";s:7:\"choices\";s:0:\"\";s:8:\"validate\";s:0:\"\";s:7:\"encrypt\";s:1:\"0\";}s:6:\"taobao\";a:18:{s:7:\"fieldid\";s:6:\"taobao\";s:9:\"available\";s:1:\"1\";s:9:\"invisible\";s:1:\"1\";s:10:\"needverify\";s:1:\"0\";s:5:\"title\";s:12:\"闃块噷鏃烘椇\";s:11:\"description\";s:0:\"\";s:12:\"displayorder\";s:1:\"0\";s:8:\"required\";s:1:\"0\";s:12:\"unchangeable\";s:1:\"0\";s:10:\"showincard\";s:1:\"0\";s:12:\"showinthread\";s:1:\"0\";s:14:\"showinregister\";s:1:\"0\";s:11:\"allowsearch\";s:1:\"0\";s:8:\"formtype\";s:4:\"text\";s:4:\"size\";s:1:\"0\";s:7:\"choices\";s:0:\"\";s:8:\"validate\";s:0:\"\";s:7:\"encrypt\";s:1:\"0\";}s:6:\"zodiac\";a:18:{s:7:\"fieldid\";s:6:\"zodiac\";s:9:\"available\";s:1:\"1\";s:9:\"invisible\";s:1:\"1\";s:10:\"needverify\";s:1:\"0\";s:5:\"title\";s:6:\"鐢熻倴\";s:11:\"description\";s:32:\"鐢熻倴(鏍规嵁鐢熸棩鑷姩璁＄畻)\";s:12:\"displayorder\";s:1:\"0\";s:8:\"required\";s:1:\"0\";s:12:\"unchangeable\";s:1:\"0\";s:10:\"showincard\";s:1:\"0\";s:12:\"showinthread\";s:1:\"0\";s:14:\"showinregister\";s:1:\"0\";s:11:\"allowsearch\";s:1:\"0\";s:8:\"formtype\";s:4:\"text\";s:4:\"size\";s:1:\"0\";s:7:\"choices\";s:0:\"\";s:8:\"validate\";s:0:\"\";s:7:\"encrypt\";s:1:\"0\";}s:7:\"address\";a:18:{s:7:\"fieldid\";s:7:\"address\";s:9:\"available\";s:1:\"1\";s:9:\"invisible\";s:1:\"1\";s:10:\"needverify\";s:1:\"0\";s:5:\"title\";s:12:\"閭瘎鍦板潃\";s:11:\"description\";s:0:\"\";s:12:\"displayorder\";s:1:\"0\";s:8:\"required\";s:1:\"0\";s:12:\"unchangeable\";s:1:\"0\";s:10:\"showincard\";s:1:\"0\";s:12:\"showinthread\";s:1:\"0\";s:14:\"showinregister\";s:1:\"0\";s:11:\"allowsearch\";s:1:\"0\";s:8:\"formtype\";s:4:\"text\";s:4:\"size\";s:1:\"0\";s:7:\"choices\";s:0:\"\";s:8:\"validate\";s:0:\"\";s:7:\"encrypt\";s:1:\"0\";}s:7:\"company\";a:18:{s:7:\"fieldid\";s:7:\"company\";s:9:\"available\";s:1:\"1\";s:9:\"invisible\";s:1:\"0\";s:10:\"needverify\";s:1:\"0\";s:5:\"title\";s:6:\"鍏徃\";s:11:\"description\";s:0:\"\";s:12:\"displayorder\";s:1:\"0\";s:8:\"required\";s:1:\"0\";s:12:\"unchangeable\";s:1:\"0\";s:10:\"showincard\";s:1:\"0\";s:12:\"showinthread\";s:1:\"0\";s:14:\"showinregister\";s:1:\"0\";s:11:\"allowsearch\";s:1:\"0\";s:8:\"formtype\";s:4:\"text\";s:4:\"size\";s:1:\"0\";s:7:\"choices\";s:0:\"\";s:8:\"validate\";s:0:\"\";s:7:\"encrypt\";s:1:\"0\";}s:7:\"revenue\";a:18:{s:7:\"fieldid\";s:7:\"revenue\";s:9:\"available\";s:1:\"1\";s:9:\"invisible\";s:1:\"1\";s:10:\"needverify\";s:1:\"0\";s:5:\"title\";s:9:\"骞存敹鍏";s:11:\"description\";s:10:\"鍗曚綅 鍏僜";s:12:\"displayorder\";s:1:\"0\";s:8:\"required\";s:1:\"0\";s:12:\"unchangeable\";s:1:\"0\";s:10:\"showincard\";s:1:\"0\";s:12:\"showinthread\";s:1:\"0\";s:14:\"showinregister\";s:1:\"0\";s:11:\"allowsearch\";s:1:\"0\";s:8:\"formtype\";s:4:\"text\";s:4:\"size\";s:1:\"0\";s:7:\"choices\";s:0:\"\";s:8:\"validate\";s:0:\"\";s:7:\"encrypt\";s:1:\"0\";}s:7:\"zipcode\";a:18:{s:7:\"fieldid\";s:7:\"zipcode\";s:9:\"available\";s:1:\"1\";s:9:\"invisible\";s:1:\"1\";s:10:\"needverify\";s:1:\"0\";s:5:\"title\";s:6:\"閭紪\";s:11:\"description\";s:0:\"\";s:12:\"displayorder\";s:1:\"0\";s:8:\"required\";s:1:\"0\";s:12:\"unchangeable\";s:1:\"0\";s:10:\"showincard\";s:1:\"0\";s:12:\"showinthread\";s:1:\"0\";s:14:\"showinregister\";s:1:\"0\";s:11:\"allowsearch\";s:1:\"0\";s:8:\"formtype\";s:4:\"text\";s:4:\"size\";s:1:\"0\";s:7:\"choices\";s:0:\"\";s:8:\"validate\";s:0:\"\";s:7:\"encrypt\";s:1:\"0\";}s:8:\"birthday\";a:18:{s:7:\"fieldid\";s:8:\"birthday\";s:9:\"available\";s:1:\"1\";s:9:\"invisible\";s:1:\"0\";s:10:\"needverify\";s:1:\"0\";s:5:\"title\";s:6:\"鐢熸棩\";s:11:\"description\";s:0:\"\";s:12:\"displayorder\";s:1:\"0\";s:8:\"required\";s:1:\"0\";s:12:\"unchangeable\";s:1:\"0\";s:10:\"showincard\";s:1:\"0\";s:12:\"showinthread\";s:1:\"0\";s:14:\"showinregister\";s:1:\"0\";s:11:\"allowsearch\";s:1:\"0\";s:8:\"formtype\";s:6:\"select\";s:4:\"size\";s:1:\"0\";s:7:\"choices\";s:0:\"\";s:8:\"validate\";s:0:\"\";s:7:\"encrypt\";s:1:\"0\";}s:8:\"interest\";a:18:{s:7:\"fieldid\";s:8:\"interest\";s:9:\"available\";s:1:\"1\";s:9:\"invisible\";s:1:\"0\";s:10:\"needverify\";s:1:\"0\";s:5:\"title\";s:12:\"鍏磋叮鐖卞ソ\";s:11:\"description\";s:0:\"\";s:12:\"displayorder\";s:1:\"0\";s:8:\"required\";s:1:\"0\";s:12:\"unchangeable\";s:1:\"0\";s:10:\"showincard\";s:1:\"0\";s:12:\"showinthread\";s:1:\"0\";s:14:\"showinregister\";s:1:\"0\";s:11:\"allowsearch\";s:1:\"0\";s:8:\"formtype\";s:8:\"textarea\";s:4:\"size\";s:1:\"0\";s:7:\"choices\";s:0:\"\";s:8:\"validate\";s:0:\"\";s:7:\"encrypt\";s:1:\"0\";}s:8:\"position\";a:18:{s:7:\"fieldid\";s:8:\"position\";s:9:\"available\";s:1:\"1\";s:9:\"invisible\";s:1:\"0\";s:10:\"needverify\";s:1:\"0\";s:5:\"title\";s:6:\"鑱屼綅\";s:11:\"description\";s:0:\"\";s:12:\"displayorder\";s:1:\"0\";s:8:\"required\";s:1:\"0\";s:12:\"unchangeable\";s:1:\"0\";s:10:\"showincard\";s:1:\"0\";s:12:\"showinthread\";s:1:\"0\";s:14:\"showinregister\";s:1:\"0\";s:11:\"allowsearch\";s:1:\"0\";s:8:\"formtype\";s:4:\"text\";s:4:\"size\";s:1:\"0\";s:7:\"choices\";s:0:\"\";s:8:\"validate\";s:0:\"\";s:7:\"encrypt\";s:1:\"0\";}s:8:\"realname\";a:18:{s:7:\"fieldid\";s:8:\"realname\";s:9:\"available\";s:1:\"1\";s:9:\"invisible\";s:1:\"0\";s:10:\"needverify\";s:1:\"0\";s:5:\"title\";s:12:\"鐪熷疄濮撳悕\";s:11:\"description\";s:0:\"\";s:12:\"displayorder\";s:1:\"0\";s:8:\"required\";s:1:\"0\";s:12:\"unchangeable\";s:1:\"0\";s:10:\"showincard\";s:1:\"0\";s:12:\"showinthread\";s:1:\"0\";s:14:\"showinregister\";s:1:\"0\";s:11:\"allowsearch\";s:1:\"1\";s:8:\"formtype\";s:4:\"text\";s:4:\"size\";s:1:\"0\";s:7:\"choices\";s:0:\"\";s:8:\"validate\";s:0:\"\";s:7:\"encrypt\";s:1:\"0\";}s:9:\"birthcity\";a:18:{s:7:\"fieldid\";s:9:\"birthcity\";s:9:\"available\";s:1:\"1\";s:9:\"invisible\";s:1:\"0\";s:10:\"needverify\";s:1:\"0\";s:5:\"title\";s:9:\"鍑虹敓鍦癨";s:11:\"description\";s:0:\"\";s:12:\"displayorder\";s:1:\"0\";s:8:\"required\";s:1:\"0\";s:12:\"unchangeable\";s:1:\"0\";s:10:\"showincard\";s:1:\"0\";s:12:\"showinthread\";s:1:\"0\";s:14:\"showinregister\";s:1:\"0\";s:11:\"allowsearch\";s:1:\"0\";s:8:\"formtype\";s:6:\"select\";s:4:\"size\";s:1:\"0\";s:7:\"choices\";s:0:\"\";s:8:\"validate\";s:0:\"\";s:7:\"encrypt\";s:1:\"0\";}s:9:\"birthdist\";a:18:{s:7:\"fieldid\";s:9:\"birthdist\";s:9:\"available\";s:1:\"1\";s:9:\"invisible\";s:1:\"0\";s:10:\"needverify\";s:1:\"0\";s:5:\"title\";s:9:\"鍑虹敓鍘縗";s:11:\"description\";s:19:\"鍑虹敓琛屾斂鍖?鍘縗";s:12:\"displayorder\";s:1:\"0\";s:8:\"required\";s:1:\"0\";s:12:\"unchangeable\";s:1:\"0\";s:10:\"showincard\";s:1:\"0\";s:12:\"showinthread\";s:1:\"0\";s:14:\"showinregister\";s:1:\"0\";s:11:\"allowsearch\";s:1:\"0\";s:8:\"formtype\";s:6:\"select\";s:4:\"size\";s:1:\"0\";s:7:\"choices\";s:0:\"\";s:8:\"validate\";s:0:\"\";s:7:\"encrypt\";s:1:\"0\";}s:9:\"birthyear\";a:18:{s:7:\"fieldid\";s:9:\"birthyear\";s:9:\"available\";s:1:\"1\";s:9:\"invisible\";s:1:\"0\";s:10:\"needverify\";s:1:\"0\";s:5:\"title\";s:12:\"鍑虹敓骞翠唤\";s:11:\"description\";s:0:\"\";s:12:\"displayorder\";s:1:\"0\";s:8:\"required\";s:1:\"0\";s:12:\"unchangeable\";s:1:\"0\";s:10:\"showincard\";s:1:\"0\";s:12:\"showinthread\";s:1:\"0\";s:14:\"showinregister\";s:1:\"0\";s:11:\"allowsearch\";s:1:\"1\";s:8:\"formtype\";s:6:\"select\";s:4:\"size\";s:1:\"0\";s:7:\"choices\";s:0:\"\";s:8:\"validate\";s:0:\"\";s:7:\"encrypt\";s:1:\"0\";}s:9:\"bloodtype\";a:18:{s:7:\"fieldid\";s:9:\"bloodtype\";s:9:\"available\";s:1:\"1\";s:9:\"invisible\";s:1:\"1\";s:10:\"needverify\";s:1:\"0\";s:5:\"title\";s:6:\"琛€鍨媆";s:11:\"description\";s:0:\"\";s:12:\"displayorder\";s:1:\"0\";s:8:\"required\";s:1:\"0\";s:12:\"unchangeable\";s:1:\"0\";s:10:\"showincard\";s:1:\"0\";s:12:\"showinthread\";s:1:\"0\";s:14:\"showinregister\";s:1:\"0\";s:11:\"allowsearch\";s:1:\"0\";s:8:\"formtype\";s:6:\"select\";s:4:\"size\";s:1:\"0\";s:7:\"choices\";s:19:\"A\r\nB\r\nAB\r\nO\r\n鍏跺畠\";s:8:\"validate\";s:0:\"\";s:7:\"encrypt\";s:1:\"0\";}s:9:\"education\";a:18:{s:7:\"fieldid\";s:9:\"education\";s:9:\"available\";s:1:\"1\";s:9:\"invisible\";s:1:\"0\";s:10:\"needverify\";s:1:\"0\";s:5:\"title\";s:6:\"瀛﹀巻\";s:11:\"description\";s:0:\"\";s:12:\"displayorder\";s:1:\"0\";s:8:\"required\";s:1:\"0\";s:12:\"unchangeable\";s:1:\"0\";s:10:\"showincard\";s:1:\"0\";s:12:\"showinthread\";s:1:\"0\";s:14:\"showinregister\";s:1:\"0\";s:11:\"allowsearch\";s:1:\"0\";s:8:\"formtype\";s:6:\"select\";s:4:\"size\";s:1:\"0\";s:7:\"choices\";s:54:\"鍗氬＋\r\n纭曞＋\r\n鏈\r\n涓撶\r\n涓\r\n灏忓\r\n鍏跺畠\";s:8:\"validate\";s:0:\"\";s:7:\"encrypt\";s:1:\"0\";}s:9:\"telephone\";a:18:{s:7:\"fieldid\";s:9:\"telephone\";s:9:\"available\";s:1:\"1\";s:9:\"invisible\";s:1:\"1\";s:10:\"needverify\";s:1:\"0\";s:5:\"title\";s:12:\"鍥哄畾鐢佃瘽\";s:11:\"description\";s:0:\"\";s:12:\"displayorder\";s:1:\"0\";s:8:\"required\";s:1:\"0\";s:12:\"unchangeable\";s:1:\"0\";s:10:\"showincard\";s:1:\"0\";s:12:\"showinthread\";s:1:\"0\";s:14:\"showinregister\";s:1:\"0\";s:11:\"allowsearch\";s:1:\"0\";s:8:\"formtype\";s:4:\"text\";s:4:\"size\";s:1:\"0\";s:7:\"choices\";s:0:\"\";s:8:\"validate\";s:0:\"\";s:7:\"encrypt\";s:1:\"0\";}s:10:\"birthmonth\";a:18:{s:7:\"fieldid\";s:10:\"birthmonth\";s:9:\"available\";s:1:\"1\";s:9:\"invisible\";s:1:\"0\";s:10:\"needverify\";s:1:\"0\";s:5:\"title\";s:12:\"鍑虹敓鏈堜唤\";s:11:\"description\";s:0:\"\";s:12:\"displayorder\";s:1:\"0\";s:8:\"required\";s:1:\"0\";s:12:\"unchangeable\";s:1:\"0\";s:10:\"showincard\";s:1:\"0\";s:12:\"showinthread\";s:1:\"0\";s:14:\"showinregister\";s:1:\"0\";s:11:\"allowsearch\";s:1:\"0\";s:8:\"formtype\";s:6:\"select\";s:4:\"size\";s:1:\"0\";s:7:\"choices\";s:0:\"\";s:8:\"validate\";s:0:\"\";s:7:\"encrypt\";s:1:\"0\";}s:10:\"idcardtype\";a:18:{s:7:\"fieldid\";s:10:\"idcardtype\";s:9:\"available\";s:1:\"1\";s:9:\"invisible\";s:1:\"1\";s:10:\"needverify\";s:1:\"0\";s:5:\"title\";s:12:\"璇佷欢绫诲瀷\";s:11:\"description\";s:29:\"韬唤璇?鎶ょ収 椹鹃┒璇佺瓑\";s:12:\"displayorder\";s:1:\"0\";s:8:\"required\";s:1:\"0\";s:12:\"unchangeable\";s:1:\"0\";s:10:\"showincard\";s:1:\"0\";s:12:\"showinthread\";s:1:\"0\";s:14:\"showinregister\";s:1:\"0\";s:11:\"allowsearch\";s:1:\"0\";s:8:\"formtype\";s:6:\"select\";s:4:\"size\";s:1:\"0\";s:7:\"choices\";s:28:\"韬唤璇乗r\n鎶ょ収\r\n椹鹃┒璇乗";s:8:\"validate\";s:0:\"\";s:7:\"encrypt\";s:1:\"0\";}s:10:\"lookingfor\";a:18:{s:7:\"fieldid\";s:10:\"lookingfor\";s:9:\"available\";s:1:\"1\";s:9:\"invisible\";s:1:\"0\";s:10:\"needverify\";s:1:\"0\";s:5:\"title\";s:12:\"浜ゅ弸鐩殑\";s:11:\"description\";s:39:\"甯屾湜鍦ㄧ綉绔欐壘鍒颁粈涔堟牱鐨勬湅鍙媆";s:12:\"displayorder\";s:1:\"0\";s:8:\"required\";s:1:\"0\";s:12:\"unchangeable\";s:1:\"0\";s:10:\"showincard\";s:1:\"0\";s:12:\"showinthread\";s:1:\"0\";s:14:\"showinregister\";s:1:\"0\";s:11:\"allowsearch\";s:1:\"0\";s:8:\"formtype\";s:4:\"text\";s:4:\"size\";s:1:\"0\";s:7:\"choices\";s:0:\"\";s:8:\"validate\";s:0:\"\";s:7:\"encrypt\";s:1:\"0\";}s:10:\"occupation\";a:18:{s:7:\"fieldid\";s:10:\"occupation\";s:9:\"available\";s:1:\"1\";s:9:\"invisible\";s:1:\"0\";s:10:\"needverify\";s:1:\"0\";s:5:\"title\";s:6:\"鑱屼笟\";s:11:\"description\";s:0:\"\";s:12:\"displayorder\";s:1:\"0\";s:8:\"required\";s:1:\"0\";s:12:\"unchangeable\";s:1:\"0\";s:10:\"showincard\";s:1:\"0\";s:12:\"showinthread\";s:1:\"0\";s:14:\"showinregister\";s:1:\"0\";s:11:\"allowsearch\";s:1:\"0\";s:8:\"formtype\";s:4:\"text\";s:4:\"size\";s:1:\"0\";s:7:\"choices\";s:0:\"\";s:8:\"validate\";s:0:\"\";s:7:\"encrypt\";s:1:\"0\";}s:10:\"residecity\";a:18:{s:7:\"fieldid\";s:10:\"residecity\";s:9:\"available\";s:1:\"1\";s:9:\"invisible\";s:1:\"0\";s:10:\"needverify\";s:1:\"0\";s:5:\"title\";s:9:\"灞呬綇鍦癨";s:11:\"description\";s:0:\"\";s:12:\"displayorder\";s:1:\"0\";s:8:\"required\";s:1:\"0\";s:12:\"unchangeable\";s:1:\"0\";s:10:\"showincard\";s:1:\"0\";s:12:\"showinthread\";s:1:\"0\";s:14:\"showinregister\";s:1:\"0\";s:11:\"allowsearch\";s:1:\"0\";s:8:\"formtype\";s:6:\"select\";s:4:\"size\";s:1:\"0\";s:7:\"choices\";s:0:\"\";s:8:\"validate\";s:0:\"\";s:7:\"encrypt\";s:1:\"0\";}s:10:\"residedist\";a:18:{s:7:\"fieldid\";s:10:\"residedist\";s:9:\"available\";s:1:\"1\";s:9:\"invisible\";s:1:\"0\";s:10:\"needverify\";s:1:\"0\";s:5:\"title\";s:9:\"灞呬綇鍘縗";s:11:\"description\";s:19:\"灞呬綇琛屾斂鍖?鍘縗";s:12:\"displayorder\";s:1:\"0\";s:8:\"required\";s:1:\"0\";s:12:\"unchangeable\";s:1:\"0\";s:10:\"showincard\";s:1:\"0\";s:12:\"showinthread\";s:1:\"0\";s:14:\"showinregister\";s:1:\"0\";s:11:\"allowsearch\";s:1:\"0\";s:8:\"formtype\";s:6:\"select\";s:4:\"size\";s:1:\"0\";s:7:\"choices\";s:0:\"\";s:8:\"validate\";s:0:\"\";s:7:\"encrypt\";s:1:\"0\";}s:12:\"birthcountry\";a:18:{s:7:\"fieldid\";s:12:\"birthcountry\";s:9:\"available\";s:1:\"1\";s:9:\"invisible\";s:1:\"0\";s:10:\"needverify\";s:1:\"0\";s:5:\"title\";s:12:\"鍑虹敓鍥藉\";s:11:\"description\";s:0:\"\";s:12:\"displayorder\";s:1:\"0\";s:8:\"required\";s:1:\"0\";s:12:\"unchangeable\";s:1:\"0\";s:10:\"showincard\";s:1:\"0\";s:12:\"showinthread\";s:1:\"0\";s:14:\"showinregister\";s:1:\"0\";s:11:\"allowsearch\";s:1:\"0\";s:8:\"formtype\";s:6:\"select\";s:4:\"size\";s:1:\"0\";s:7:\"choices\";s:0:\"\";s:8:\"validate\";s:0:\"\";s:7:\"encrypt\";s:1:\"0\";}s:13:\"birthprovince\";a:18:{s:7:\"fieldid\";s:13:\"birthprovince\";s:9:\"available\";s:1:\"1\";s:9:\"invisible\";s:1:\"0\";s:10:\"needverify\";s:1:\"0\";s:5:\"title\";s:12:\"鍑虹敓鐪佷唤\";s:11:\"description\";s:0:\"\";s:12:\"displayorder\";s:1:\"0\";s:8:\"required\";s:1:\"0\";s:12:\"unchangeable\";s:1:\"0\";s:10:\"showincard\";s:1:\"0\";s:12:\"showinthread\";s:1:\"0\";s:14:\"showinregister\";s:1:\"0\";s:11:\"allowsearch\";s:1:\"0\";s:8:\"formtype\";s:6:\"select\";s:4:\"size\";s:1:\"0\";s:7:\"choices\";s:0:\"\";s:8:\"validate\";s:0:\"\";s:7:\"encrypt\";s:1:\"0\";}s:13:\"constellation\";a:18:{s:7:\"fieldid\";s:13:\"constellation\";s:9:\"available\";s:1:\"1\";s:9:\"invisible\";s:1:\"1\";s:10:\"needverify\";s:1:\"0\";s:5:\"title\";s:6:\"鏄熷骇\";s:11:\"description\";s:32:\"鏄熷骇(鏍规嵁鐢熸棩鑷姩璁＄畻)\";s:12:\"displayorder\";s:1:\"0\";s:8:\"required\";s:1:\"0\";s:12:\"unchangeable\";s:1:\"0\";s:10:\"showincard\";s:1:\"0\";s:12:\"showinthread\";s:1:\"0\";s:14:\"showinregister\";s:1:\"0\";s:11:\"allowsearch\";s:1:\"0\";s:8:\"formtype\";s:4:\"text\";s:4:\"size\";s:1:\"0\";s:7:\"choices\";s:0:\"\";s:8:\"validate\";s:0:\"\";s:7:\"encrypt\";s:1:\"0\";}s:13:\"residecountry\";a:18:{s:7:\"fieldid\";s:13:\"residecountry\";s:9:\"available\";s:1:\"1\";s:9:\"invisible\";s:1:\"0\";s:10:\"needverify\";s:1:\"0\";s:5:\"title\";s:12:\"灞呬綇鍥藉\";s:11:\"description\";s:0:\"\";s:12:\"displayorder\";s:1:\"0\";s:8:\"required\";s:1:\"0\";s:12:\"unchangeable\";s:1:\"0\";s:10:\"showincard\";s:1:\"0\";s:12:\"showinthread\";s:1:\"0\";s:14:\"showinregister\";s:1:\"0\";s:11:\"allowsearch\";s:1:\"0\";s:8:\"formtype\";s:6:\"select\";s:4:\"size\";s:1:\"0\";s:7:\"choices\";s:0:\"\";s:8:\"validate\";s:0:\"\";s:7:\"encrypt\";s:1:\"0\";}s:14:\"birthcommunity\";a:18:{s:7:\"fieldid\";s:14:\"birthcommunity\";s:9:\"available\";s:1:\"1\";s:9:\"invisible\";s:1:\"0\";s:10:\"needverify\";s:1:\"0\";s:5:\"title\";s:12:\"鍑虹敓灏忓尯\";s:11:\"description\";s:0:\"\";s:12:\"displayorder\";s:1:\"0\";s:8:\"required\";s:1:\"0\";s:12:\"unchangeable\";s:1:\"0\";s:10:\"showincard\";s:1:\"0\";s:12:\"showinthread\";s:1:\"0\";s:14:\"showinregister\";s:1:\"0\";s:11:\"allowsearch\";s:1:\"0\";s:8:\"formtype\";s:6:\"select\";s:4:\"size\";s:1:\"0\";s:7:\"choices\";s:0:\"\";s:8:\"validate\";s:0:\"\";s:7:\"encrypt\";s:1:\"0\";}s:14:\"graduateschool\";a:18:{s:7:\"fieldid\";s:14:\"graduateschool\";s:9:\"available\";s:1:\"1\";s:9:\"invisible\";s:1:\"0\";s:10:\"needverify\";s:1:\"0\";s:5:\"title\";s:12:\"姣曚笟瀛︽牎\";s:11:\"description\";s:0:\"\";s:12:\"displayorder\";s:1:\"0\";s:8:\"required\";s:1:\"0\";s:12:\"unchangeable\";s:1:\"0\";s:10:\"showincard\";s:1:\"0\";s:12:\"showinthread\";s:1:\"0\";s:14:\"showinregister\";s:1:\"0\";s:11:\"allowsearch\";s:1:\"0\";s:8:\"formtype\";s:4:\"text\";s:4:\"size\";s:1:\"0\";s:7:\"choices\";s:0:\"\";s:8:\"validate\";s:0:\"\";s:7:\"encrypt\";s:1:\"0\";}s:14:\"resideprovince\";a:18:{s:7:\"fieldid\";s:14:\"resideprovince\";s:9:\"available\";s:1:\"1\";s:9:\"invisible\";s:1:\"0\";s:10:\"needverify\";s:1:\"0\";s:5:\"title\";s:12:\"灞呬綇鐪佷唤\";s:11:\"description\";s:0:\"\";s:12:\"displayorder\";s:1:\"0\";s:8:\"required\";s:1:\"0\";s:12:\"unchangeable\";s:1:\"0\";s:10:\"showincard\";s:1:\"0\";s:12:\"showinthread\";s:1:\"0\";s:14:\"showinregister\";s:1:\"0\";s:11:\"allowsearch\";s:1:\"0\";s:8:\"formtype\";s:6:\"select\";s:4:\"size\";s:1:\"0\";s:7:\"choices\";s:0:\"\";s:8:\"validate\";s:0:\"\";s:7:\"encrypt\";s:1:\"0\";}s:15:\"affectivestatus\";a:18:{s:7:\"fieldid\";s:15:\"affectivestatus\";s:9:\"available\";s:1:\"1\";s:9:\"invisible\";s:1:\"1\";s:10:\"needverify\";s:1:\"0\";s:5:\"title\";s:12:\"鎯呮劅鐘舵€乗";s:11:\"description\";s:0:\"\";s:12:\"displayorder\";s:1:\"0\";s:8:\"required\";s:1:\"0\";s:12:\"unchangeable\";s:1:\"0\";s:10:\"showincard\";s:1:\"0\";s:12:\"showinthread\";s:1:\"0\";s:14:\"showinregister\";s:1:\"0\";s:11:\"allowsearch\";s:1:\"0\";s:8:\"formtype\";s:4:\"text\";s:4:\"size\";s:1:\"0\";s:7:\"choices\";s:0:\"\";s:8:\"validate\";s:0:\"\";s:7:\"encrypt\";s:1:\"0\";}s:15:\"residecommunity\";a:18:{s:7:\"fieldid\";s:15:\"residecommunity\";s:9:\"available\";s:1:\"1\";s:9:\"invisible\";s:1:\"0\";s:10:\"needverify\";s:1:\"0\";s:5:\"title\";s:12:\"灞呬綇灏忓尯\";s:11:\"description\";s:0:\"\";s:12:\"displayorder\";s:1:\"0\";s:8:\"required\";s:1:\"0\";s:12:\"unchangeable\";s:1:\"0\";s:10:\"showincard\";s:1:\"0\";s:12:\"showinthread\";s:1:\"0\";s:14:\"showinregister\";s:1:\"0\";s:11:\"allowsearch\";s:1:\"0\";s:8:\"formtype\";s:6:\"select\";s:4:\"size\";s:1:\"0\";s:7:\"choices\";s:0:\"\";s:8:\"validate\";s:0:\"\";s:7:\"encrypt\";s:1:\"0\";}}'),
('relatedlink',1,1784453715,'a:0:{}'),
('secqaa',1,1784453715,'a:0:{}'),
('setting',1,1784453715,'a:514:{s:11:\"accessemail\";s:0:\"\";s:12:\"accountguard\";a:3:{s:12:\"loginpwcheck\";s:1:\"0\";s:14:\"loginoutofdate\";s:1:\"0\";s:17:\"loginoutofdatenum\";s:0:\"\";}s:14:\"activitycredit\";s:1:\"1\";s:14:\"activityextnum\";s:1:\"0\";s:13:\"activityfield\";s:88:\"a:3:{s:8:\"realname\";s:12:\"鐪熷疄濮撳悕\";s:6:\"mobile\";s:6:\"鎵嬫満\";s:2:\"qq\";s:5:\"QQ鍙穃";}\";s:15:\"activityforumid\";s:1:\"0\";s:10:\"activitypp\";s:1:\"8\";s:12:\"activitytype\";s:68:\"鏈嬪弸鑱氫細\r\n鍑哄閮婃父\r\n鑷┚鍑鸿\r\n鍏泭娲诲姩\r\n绾夸笂娲诲姩\";s:10:\"adminemail\";s:15:\"admin@admin.com\";s:13:\"adminipaccess\";s:0:\"\";s:16:\"adminnotifytypes\";s:231:\"verifythread,verifypost,verifyuser,verifyblog,verifydoing,verifypic,verifyshare,verifycommontes,verifyrecycle,verifyrecyclepost,verifyarticle,verifyacommont,verifymedal,verify_1,verify_2,verify_3,verify_4,verify_5,verify_6,verify_7\";s:13:\"advexpiration\";a:3:{s:5:\"allow\";b:0;s:3:\"day\";s:0:\"\";s:5:\"users\";s:0:\"\";}s:7:\"advtype\";a:1:{i:0;s:6:\"custom\";}s:21:\"albumcategoryrequired\";s:1:\"0\";s:17:\"albumcategorystat\";s:1:\"0\";s:11:\"albumstatus\";s:1:\"0\";s:14:\"allowattachurl\";s:1:\"0\";s:11:\"allowdomain\";s:1:\"0\";s:13:\"alloweditpost\";s:1:\"0\";s:14:\"allowfastreply\";s:1:\"0\";s:16:\"allowgroupdomain\";s:1:\"0\";s:21:\"allowmoderatingthread\";s:1:\"1\";s:16:\"allowpostcomment\";a:2:{i:0;s:1:\"1\";i:1;s:1:\"2\";}s:21:\"allowquickviewprofile\";s:1:\"1\";s:12:\"allowreplybg\";s:1:\"0\";s:16:\"allowspacedomain\";s:1:\"0\";s:17:\"allowswitcheditor\";s:1:\"1\";s:19:\"allowviewuserthread\";s:0:\"\";s:14:\"allowwidthauto\";s:1:\"0\";s:13:\"anonymoustext\";s:6:\"鍖垮悕\";s:9:\"antitheft\";a:2:{s:5:\"allow\";i:0;s:3:\"max\";i:200;}s:8:\"archiver\";s:1:\"1\";s:16:\"archiverredirect\";s:1:\"0\";s:12:\"article_tags\";a:8:{i:1;s:6:\"鍘熷垱\";i:2;s:6:\"鐑偣\";i:3;s:6:\"缁勫浘\";i:4;s:6:\"鐖嗘枡\";i:5;s:6:\"澶存潯\";i:6;s:6:\"骞荤伅\";i:7;s:6:\"婊氬姩\";i:8;s:6:\"鎺ㄨ崘\";}s:9:\"at_anyone\";s:1:\"0\";s:16:\"attachbanperiods\";s:0:\"\";s:9:\"attachdir\";s:30:\"/app/public/./data/attachment/\";s:12:\"attachexpire\";s:0:\"\";s:13:\"attachimgpost\";s:1:\"1\";s:14:\"attachrefcheck\";s:1:\"0\";s:10:\"attachsave\";s:1:\"3\";s:9:\"attachurl\";s:16:\"data/attachment/\";s:7:\"authkey\";s:0:\"\";s:12:\"authoronleft\";s:1:\"1\";s:12:\"autoidselect\";s:1:\"0\";s:12:\"avatarmethod\";s:1:\"0\";s:14:\"bannedmessages\";s:1:\"1\";s:8:\"bbclosed\";s:0:\"\";s:6:\"bbname\";s:7:\"Discuz!\";s:7:\"bbrules\";s:1:\"0\";s:12:\"bbrulesforce\";s:1:\"0\";s:10:\"bbrulestxt\";s:0:\"\";s:10:\"bdaystatus\";s:1:\"0\";s:11:\"binddomains\";s:6:\"a:0:{}\";s:23:\"blockmaxaggregationitem\";s:5:\"20000\";s:20:\"blogcategoryrequired\";s:1:\"0\";s:16:\"blogcategorystat\";s:1:\"0\";s:14:\"blogrecyclebin\";s:1:\"0\";s:10:\"blogstatus\";s:1:\"0\";s:13:\"boardlicensed\";s:1:\"0\";s:14:\"cacheindexlife\";s:1:\"0\";s:14:\"cachethreaddir\";s:16:\"data/threadcache\";s:15:\"cachethreadlife\";s:1:\"0\";s:4:\"card\";a:1:{s:4:\"open\";s:1:\"0\";}s:11:\"censoremail\";s:0:\"\";s:10:\"censoruser\";s:0:\"\";s:12:\"change_email\";s:1:\"0\";s:16:\"change_secmobile\";s:1:\"0\";s:17:\"chatpmrefreshtime\";s:1:\"8\";s:14:\"close_leftinfo\";s:1:\"0\";s:23:\"close_leftinfo_userctrl\";s:1:\"0\";s:21:\"closedallowactivation\";s:1:\"0\";s:17:\"closeforumorderby\";s:1:\"0\";s:13:\"collectionnum\";s:2:\"10\";s:19:\"collectionrecommend\";s:71:\"a:3:{s:5:\"ctids\";N;s:13:\"autorecommend\";i:0;s:14:\"adminrecommend\";i:0;}\";s:22:\"collectionrecommendnum\";s:1:\"0\";s:16:\"collectionstatus\";s:1:\"0\";s:23:\"collectionteamworkernum\";s:1:\"3\";s:16:\"commentfirstpost\";s:1:\"1\";s:11:\"commentitem\";a:6:{i:0;s:0:\"\";i:1;s:0:\"\";i:2;s:0:\"\";i:3;s:0:\"\";i:4;s:0:\"\";i:5;s:0:\"\";}s:13:\"commentnumber\";s:1:\"5\";s:15:\"commentpostself\";s:1:\"0\";s:7:\"connect\";a:19:{s:5:\"allow\";s:1:\"1\";s:4:\"feed\";a:2:{s:5:\"allow\";s:1:\"1\";s:5:\"group\";s:1:\"0\";}s:1:\"t\";a:2:{s:5:\"allow\";s:1:\"1\";s:5:\"group\";s:1:\"0\";}s:10:\"like_allow\";s:1:\"1\";s:7:\"like_qq\";s:0:\"\";s:10:\"turl_allow\";s:1:\"1\";s:7:\"turl_qq\";s:0:\"\";s:8:\"like_url\";s:0:\"\";s:17:\"register_birthday\";s:1:\"0\";s:15:\"register_gender\";s:1:\"0\";s:17:\"register_uinlimit\";s:0:\"\";s:21:\"register_rewardcredit\";s:1:\"1\";s:18:\"register_addcredit\";s:0:\"\";s:16:\"register_groupid\";s:1:\"0\";s:18:\"register_regverify\";s:1:\"1\";s:15:\"register_invite\";s:1:\"0\";s:10:\"newbiespan\";s:0:\"\";s:9:\"turl_code\";s:0:\"\";s:13:\"mblog_app_key\";s:3:\"abc\";}s:12:\"creditnotice\";s:1:\"1\";s:14:\"creditsformula\";s:112:\"$member[\'posts\']+$member[\'digestposts\']*5+$member[\'extcredits1\']*2+$member[\'extcredits2\']+$member[\'extcredits3\']\";s:17:\"creditsformulaexp\";s:122:\"<u>{credits_CREDITS}</u>=<u>{credits_POSTS}</u>+<u>{credits_DIGESTPOSTS}</u>*5+<u>濞佹湜</u>*2+<u>閲戦挶</u>+<u>璐＄尞</u>\";s:13:\"creditspolicy\";a:12:{s:4:\"post\";a:0:{}s:5:\"reply\";a:0:{}s:6:\"digest\";a:1:{i:1;i:10;}s:10:\"postattach\";a:0:{}s:9:\"getattach\";a:0:{}s:6:\"sendpm\";a:0:{}s:6:\"search\";a:0:{}s:15:\"promotion_visit\";b:0;s:18:\"promotion_register\";b:0;s:13:\"tradefinished\";a:0:{}s:8:\"votepoll\";a:0:{}s:10:\"lowerlimit\";a:0:{}}s:19:\"creditspolicymobile\";s:1:\"0\";s:10:\"creditstax\";s:3:\"0.2\";s:12:\"creditstrans\";s:1:\"2\";s:8:\"csspathv\";s:11:\"data/cache/\";s:8:\"darkroom\";s:1:\"1\";s:11:\"dateconvert\";s:1:\"1\";s:10:\"dateformat\";s:5:\"Y-n-j\";s:13:\"debateforumid\";s:1:\"0\";s:5:\"debug\";s:1:\"1\";s:17:\"defaulteditormode\";s:1:\"1\";s:12:\"defaultindex\";s:9:\"forum.php\";s:14:\"delayviewcount\";s:1:\"0\";s:12:\"deletereason\";s:0:\"\";s:15:\"disableipnotice\";s:1:\"0\";s:13:\"disallowfloat\";s:9:\"newthread\";s:14:\"disfixedavatar\";s:1:\"0\";s:23:\"disfixednv_forumdisplay\";s:1:\"0\";s:21:\"disfixednv_forumindex\";s:1:\"0\";s:21:\"disfixednv_viewthread\";s:1:\"0\";s:11:\"doingstatus\";s:1:\"0\";s:6:\"domain\";a:5:{s:12:\"defaultindex\";s:9:\"forum.php\";s:10:\"holddomain\";s:24:\"www|*blog*|*space*|*bbs*\";s:4:\"list\";a:0:{}s:3:\"app\";a:5:{s:6:\"portal\";s:0:\"\";s:5:\"forum\";s:0:\"\";s:5:\"group\";s:0:\"\";s:4:\"home\";s:0:\"\";s:7:\"default\";s:0:\"\";}s:4:\"root\";a:5:{s:4:\"home\";s:0:\"\";s:5:\"group\";s:0:\"\";s:5:\"forum\";s:0:\"\";s:5:\"topic\";s:0:\"\";s:7:\"channel\";s:0:\"\";}}s:10:\"domainroot\";s:0:\"\";s:7:\"doublee\";s:1:\"1\";s:12:\"dupkarmarate\";s:1:\"0\";s:6:\"dynavt\";s:1:\"1\";s:10:\"ec_account\";s:0:\"\";s:11:\"ec_contract\";s:0:\"\";s:9:\"ec_credit\";a:2:{s:18:\"maxcreditspermonth\";i:6;s:4:\"rank\";a:15:{i:1;i:4;i:2;i:11;i:3;i:41;i:4;i:91;i:5;i:151;i:6;i:251;i:7;i:501;i:8;i:1001;i:9;i:2001;i:10;i:5001;i:11;i:10001;i:12;i:20001;i:13;i:50001;i:14;i:100001;i:15;i:200001;}}s:13:\"ec_maxcredits\";s:4:\"1000\";s:21:\"ec_maxcreditspermonth\";s:1:\"0\";s:13:\"ec_mincredits\";s:1:\"0\";s:8:\"ec_ratio\";s:1:\"0\";s:8:\"editedby\";s:1:\"1\";s:10:\"editorfids\";s:17:\"a:1:{i:0;s:0:\"\";}\";s:13:\"editorgroupid\";s:17:\"a:1:{i:0;s:0:\"\";}\";s:14:\"editormodetype\";s:1:\"0\";s:13:\"editoroptions\";s:1:\"6\";s:10:\"editperdel\";s:1:\"0\";s:13:\"edittimelimit\";s:0:\"\";s:18:\"exchangemincredits\";s:3:\"100\";s:10:\"extcredits\";a:3:{i:1;a:7:{s:3:\"img\";s:0:\"\";s:5:\"title\";s:6:\"濞佹湜\";s:4:\"unit\";s:0:\"\";s:5:\"ratio\";i:0;s:12:\"showinthread\";N;s:15:\"allowexchangein\";N;s:16:\"allowexchangeout\";N;}i:2;a:7:{s:3:\"img\";s:0:\"\";s:5:\"title\";s:6:\"閲戦挶\";s:4:\"unit\";s:0:\"\";s:5:\"ratio\";i:0;s:12:\"showinthread\";N;s:15:\"allowexchangein\";N;s:16:\"allowexchangeout\";N;}i:3;a:7:{s:3:\"img\";s:0:\"\";s:5:\"title\";s:6:\"璐＄尞\";s:4:\"unit\";s:0:\"\";s:5:\"ratio\";i:0;s:12:\"showinthread\";N;s:15:\"allowexchangein\";N;s:16:\"allowexchangeout\";N;}}s:8:\"fastpost\";s:1:\"1\";s:11:\"fastsmilies\";s:1:\"1\";s:14:\"favoritestatus\";s:1:\"0\";s:7:\"feedday\";s:1:\"7\";s:10:\"feedhotday\";s:1:\"2\";s:10:\"feedhotmin\";s:1:\"3\";s:10:\"feedhotnum\";s:1:\"3\";s:10:\"feedmaxnum\";s:3:\"100\";s:10:\"feedstatus\";s:1:\"0\";s:15:\"feedtargetblank\";s:1:\"1\";s:14:\"filterednovote\";s:1:\"1\";s:9:\"floodctrl\";s:2:\"15\";s:5:\"focus\";a:0:{}s:15:\"followaddnotice\";s:1:\"0\";s:15:\"followretainday\";s:1:\"7\";s:12:\"followstatus\";s:1:\"0\";s:14:\"forumallowside\";s:1:\"0\";s:25:\"forumdisplaythreadpreview\";s:1:\"1\";s:12:\"forumdomains\";s:6:\"a:0:{}\";s:9:\"forumjump\";s:1:\"0\";s:15:\"forumlinkstatus\";s:1:\"1\";s:13:\"forumpicstyle\";s:70:\"a:3:{s:10:\"thumbwidth\";i:0;s:11:\"thumbheight\";i:0;s:8:\"thumbnum\";i:0;}\";s:14:\"forumseparator\";s:1:\"1\";s:11:\"forumstatus\";s:1:\"1\";s:17:\"forumstickthreads\";s:6:\"a:0:{}\";s:7:\"frameon\";s:1:\"0\";s:10:\"framewidth\";s:3:\"180\";s:14:\"friendgroupnum\";s:1:\"8\";s:12:\"friendstatus\";s:1:\"0\";s:3:\"ftp\";a:11:{s:2:\"on\";s:1:\"0\";s:3:\"ssl\";s:1:\"0\";s:4:\"host\";s:0:\"\";s:4:\"port\";s:2:\"21\";s:8:\"username\";s:0:\"\";s:8:\"password\";s:0:\"\";s:9:\"attachdir\";s:1:\".\";s:9:\"attachurl\";s:1:\"/\";s:7:\"hideurl\";s:1:\"0\";s:7:\"timeout\";s:1:\"0\";s:6:\"connid\";i:0;}s:11:\"globalstick\";s:1:\"1\";s:4:\"grid\";a:8:{s:8:\"showgrid\";s:1:\"0\";s:8:\"gridtype\";s:1:\"0\";s:8:\"textleng\";s:2:\"30\";s:4:\"fids\";a:1:{i:0;i:0;}s:9:\"highlight\";s:1:\"1\";s:11:\"targetblank\";s:1:\"1\";s:8:\"showtips\";s:1:\"1\";s:9:\"cachelife\";s:3:\"600\";}s:19:\"group_admingroupids\";s:18:\"a:1:{i:1;s:1:\"1\";}\";s:15:\"group_allowfeed\";s:1:\"1\";s:17:\"group_description\";s:0:\"\";s:18:\"group_imgsizelimit\";s:3:\"512\";s:14:\"group_keywords\";s:0:\"\";s:15:\"group_recommend\";s:6:\"a:0:{}\";s:14:\"group_userperm\";s:684:\"a:22:{s:16:\"allowstickthread\";s:1:\"1\";s:15:\"allowbumpthread\";s:1:\"1\";s:20:\"allowhighlightthread\";s:1:\"1\";s:16:\"allowstampthread\";s:1:\"1\";s:16:\"allowclosethread\";s:1:\"1\";s:16:\"allowmergethread\";s:1:\"1\";s:16:\"allowsplitthread\";s:1:\"1\";s:17:\"allowrepairthread\";s:1:\"1\";s:11:\"allowrefund\";s:1:\"1\";s:13:\"alloweditpoll\";s:1:\"1\";s:17:\"allowremovereward\";s:1:\"1\";s:17:\"alloweditactivity\";s:1:\"1\";s:14:\"allowedittrade\";s:1:\"1\";s:17:\"allowdigestthread\";s:1:\"3\";s:13:\"alloweditpost\";s:1:\"1\";s:13:\"allowwarnpost\";s:1:\"1\";s:12:\"allowbanpost\";s:1:\"1\";s:12:\"allowdelpost\";s:1:\"1\";s:13:\"allowupbanner\";s:1:\"1\";s:15:\"disablepostctrl\";s:1:\"0\";s:11:\"allowviewip\";s:1:\"1\";s:15:\"allowlivethread\";s:1:\"1\";}\";s:8:\"groupmod\";s:1:\"0\";s:11:\"groupstatus\";s:1:\"0\";s:17:\"guesttipsinthread\";a:2:{s:4:\"flag\";i:0;s:4:\"text\";s:0:\"\";}s:14:\"guestviewthumb\";a:3:{s:4:\"flag\";i:0;s:5:\"width\";i:100;s:6:\"height\";i:100;}s:5:\"guide\";s:51:\"a:2:{s:5:\"hotdt\";i:604800;s:8:\"digestdt\";i:604800;}\";s:11:\"guidestatus\";s:1:\"0\";s:10:\"heatthread\";a:5:{s:4:\"type\";s:1:\"2\";s:5:\"reply\";i:5;s:9:\"recommend\";i:3;s:6:\"period\";s:2:\"15\";s:10:\"iconlevels\";a:3:{i:2;s:3:\"200\";i:1;s:3:\"100\";i:0;s:2:\"50\";}}s:14:\"hideattachdown\";s:1:\"0\";s:14:\"hideattachtips\";s:1:\"0\";s:16:\"hidefilteredpost\";s:1:\"0\";s:11:\"hideprivate\";s:1:\"1\";s:12:\"historyposts\";s:3:\"0	7\";s:10:\"holddomain\";s:24:\"www|*blog*|*space*|*bbs*\";s:13:\"homepagestyle\";s:1:\"0\";s:10:\"homestatus\";s:1:\"0\";s:9:\"homestyle\";s:1:\"0\";s:8:\"hottopic\";s:2:\"10\";s:3:\"icp\";s:0:\"\";s:8:\"imagelib\";s:1:\"0\";s:14:\"imagelistthumb\";s:1:\"0\";s:13:\"imagemaxwidth\";i:600;s:8:\"indexhot\";a:7:{s:6:\"status\";s:1:\"0\";s:5:\"limit\";s:2:\"10\";s:4:\"days\";s:1:\"7\";s:10:\"expiration\";s:3:\"900\";s:10:\"messagecut\";s:3:\"200\";s:5:\"width\";i:100;s:6:\"height\";i:70;}s:9:\"indextype\";s:8:\"classics\";s:14:\"infosidestatus\";b:0;s:11:\"initcredits\";s:17:\"0,0,0,0,0,0,0,0,0\";s:12:\"inviteconfig\";a:1:{s:16:\"invitecodeprompt\";s:0:\"\";}s:8:\"ipaccess\";s:0:\"\";s:13:\"ipregctrltime\";s:2:\"72\";s:11:\"jscachelife\";s:4:\"1800\";s:12:\"jsdateformat\";s:0:\"\";s:6:\"jspath\";s:11:\"data/cache/\";s:12:\"jsrefdomains\";s:0:\"\";s:8:\"jsstatus\";s:1:\"0\";s:14:\"karmaratelimit\";s:1:\"0\";s:8:\"lazyload\";s:1:\"0\";s:12:\"leftsideopen\";s:1:\"0\";s:13:\"leftsidewidth\";s:1:\"0\";s:3:\"log\";a:14:{s:13:\"clearlogstime\";s:1:\"0\";s:14:\"clearlogstypes\";a:2:{i:0;s:2:\"cp\";i:1;s:5:\"error\";}s:7:\"illegal\";s:1:\"1\";s:3:\"ban\";s:1:\"1\";s:4:\"mods\";s:1:\"1\";s:3:\"sms\";s:1:\"1\";s:5:\"login\";s:1:\"1\";s:2:\"cp\";s:1:\"1\";s:5:\"modcp\";s:1:\"1\";s:5:\"error\";s:1:\"1\";s:8:\"sendmail\";s:1:\"1\";s:4:\"SMTP\";s:1:\"1\";s:4:\"rate\";s:1:\"1\";s:3:\"pmt\";s:1:\"1\";}s:11:\"losslessdel\";s:3:\"365\";s:13:\"magicdiscount\";s:2:\"85\";s:11:\"magicmarket\";s:1:\"1\";s:11:\"magicstatus\";s:1:\"0\";s:4:\"mail\";s:545:\"a:17:{s:8:\"mailsend\";s:1:\"1\";s:6:\"server\";s:13:\"smtp.21cn.com\";s:4:\"port\";s:2:\"25\";s:4:\"auth\";s:1:\"1\";s:4:\"from\";s:26:\"Discuz <username@21cn.com>\";s:13:\"auth_username\";s:17:\"username@21cn.com\";s:13:\"auth_password\";s:8:\"password\";s:13:\"maildelimiter\";s:1:\"0\";s:12:\"mailusername\";s:1:\"1\";s:15:\"sendmail_silent\";s:1:\"1\";s:15:\"emailcodestatus\";s:1:\"0\";s:22:\"emailcodedefaultlength\";s:1:\"6\";s:16:\"emailverifylimit\";s:1:\"5\";s:14:\"emailtimelimit\";s:5:\"86400\";s:13:\"emailnumlimit\";s:1:\"5\";s:13:\"emailinterval\";s:3:\"300\";s:13:\"emailglblimit\";s:4:\"1000\";}\";s:14:\"maxavatarpixel\";s:3:\"120\";s:13:\"maxavatarsize\";s:5:\"20000\";s:8:\"maxbdays\";s:1:\"0\";s:13:\"maxchargespan\";s:1:\"0\";s:12:\"maxfavorites\";s:3:\"100\";s:15:\"maxincperthread\";s:1:\"0\";s:13:\"maxmagicprice\";s:2:\"50\";s:17:\"maxmodworksmonths\";s:1:\"3\";s:13:\"maxonlinelist\";s:1:\"0\";s:7:\"maxpage\";s:3:\"100\";s:14:\"maxpolloptions\";s:2:\"20\";s:11:\"maxpostsize\";s:5:\"10000\";s:10:\"maxsigrows\";s:3:\"100\";s:10:\"maxsmilies\";s:2:\"10\";s:14:\"maxsubjectsize\";s:2:\"80\";s:11:\"medalstatus\";s:1:\"0\";s:14:\"membermaxpages\";s:3:\"100\";s:13:\"memberperpage\";s:2:\"25\";s:13:\"memliststatus\";s:1:\"1\";s:6:\"memory\";a:16:{s:13:\"common_member\";i:0;s:19:\"common_member_count\";i:0;s:20:\"common_member_status\";i:0;s:21:\"common_member_profile\";i:0;s:24:\"common_member_field_home\";i:0;s:25:\"common_member_field_forum\";i:0;s:20:\"common_member_verify\";i:0;s:12:\"forum_thread\";i:172800;s:25:\"forum_thread_forumdisplay\";i:300;s:23:\"forum_collectionrelated\";i:0;s:15:\"forum_postcache\";i:300;s:16:\"forum_collection\";i:300;s:11:\"home_follow\";i:86400;s:10:\"forumindex\";i:30;s:8:\"diyblock\";i:300;s:14:\"diyblockoutput\";i:30;}s:11:\"minpostsize\";s:2:\"10\";s:18:\"minpostsize_mobile\";s:1:\"0\";s:14:\"minsubjectsize\";s:1:\"1\";s:6:\"mobile\";a:13:{s:11:\"allowmobile\";i:1;s:9:\"allowmnew\";i:0;s:13:\"mobileforward\";i:1;s:14:\"mobileregister\";i:1;s:13:\"mobileseccode\";i:0;s:16:\"mobilesimpletype\";i:0;s:15:\"mobilecachetime\";i:0;s:14:\"mobilecomefrom\";s:0:\"\";s:13:\"mobilepreview\";i:0;s:6:\"legacy\";i:1;s:3:\"wml\";i:0;s:6:\"portal\";a:1:{s:6:\"catnav\";i:0;}s:5:\"forum\";a:6:{s:5:\"index\";i:0;s:8:\"statshow\";i:0;s:13:\"displayorder3\";i:1;s:12:\"topicperpage\";i:20;s:11:\"postperpage\";i:10;s:9:\"forumview\";i:0;}}s:8:\"modasban\";s:1:\"1\";s:9:\"moddetail\";s:1:\"0\";s:10:\"moddisplay\";s:4:\"flat\";s:12:\"modratelimit\";s:1:\"0\";s:17:\"modreasons_public\";s:1:\"0\";s:14:\"moduser_public\";s:1:\"0\";s:13:\"modworkstatus\";s:1:\"1\";s:3:\"mps\";s:0:\"\";s:10:\"msgforward\";s:504:\"a:3:{s:11:\"refreshtime\";i:2;s:5:\"quick\";i:1;s:8:\"messages\";a:14:{i:0;s:19:\"thread_poll_succeed\";i:1;s:19:\"thread_rate_succeed\";i:2;s:23:\"usergroups_join_succeed\";i:3;s:23:\"usergroups_exit_succeed\";i:4;s:25:\"usergroups_update_succeed\";i:5;s:20:\"buddy_update_succeed\";i:6;s:17:\"post_edit_succeed\";i:7;s:18:\"post_reply_succeed\";i:8;s:24:\"post_edit_delete_succeed\";i:9;s:22:\"post_newthread_succeed\";i:10;s:13:\"admin_succeed\";i:11;s:17:\"pm_delete_succeed\";i:12;s:15:\"search_redirect\";i:13;s:10:\"do_success\";}}\";s:3:\"msn\";s:0:\"\";s:19:\"my_closecheckupdate\";s:0:\"\";s:5:\"my_ip\";s:0:\"\";s:14:\"my_search_data\";b:0;s:9:\"my_siteid\";s:0:\"\";s:10:\"my_sitekey\";s:0:\"\";s:11:\"navsubhover\";s:1:\"0\";s:11:\"need_avatar\";s:1:\"0\";s:10:\"need_email\";s:1:\"0\";s:14:\"need_secmobile\";s:1:\"0\";s:11:\"networkpage\";s:1:\"0\";s:6:\"newbie\";s:2:\"20\";s:10:\"newbiespan\";s:1:\"2\";s:11:\"newbietasks\";s:0:\"\";s:16:\"newbietaskupdate\";s:0:\"\";s:14:\"newspaceavatar\";s:1:\"0\";s:14:\"nocacheheaders\";s:1:\"0\";s:14:\"nofilteredpost\";s:1:\"0\";s:11:\"notifyusers\";s:83:\"a:1:{i:1;a:2:{s:8:\"username\";s:5:\"admin\";s:5:\"types\";s:20:\"11111111111111111111\";}}\";s:10:\"nsprofiles\";s:1:\"1\";s:10:\"numbercard\";s:74:\"a:1:{s:3:\"row\";a:3:{i:1;s:7:\"threads\";i:2;s:5:\"posts\";i:3;s:7:\"credits\";}}\";s:10:\"oltimespan\";s:2:\"10\";s:20:\"onlineguestsmultiple\";s:2:\"10\";s:10:\"onlinehold\";i:900;s:12:\"onlinerecord\";s:12:\"7	1269749404\";s:18:\"onlyacceptfriendpm\";s:1:\"0\";s:13:\"optimizeviews\";s:1:\"0\";s:13:\"outlandverify\";s:1:\"0\";s:12:\"pmreportuser\";s:1:\"1\";s:11:\"pollforumid\";s:1:\"0\";s:27:\"portalarticleimgthumbclosed\";s:1:\"0\";s:12:\"portalstatus\";s:1:\"0\";s:10:\"postappend\";s:1:\"0\";s:14:\"postbanperiods\";s:0:\"\";s:14:\"postignorearea\";s:0:\"\";s:12:\"postignoreip\";s:0:\"\";s:14:\"postmodperiods\";s:0:\"\";s:11:\"postperpage\";s:2:\"10\";s:14:\"preventrefresh\";s:1:\"1\";s:7:\"privacy\";a:2:{s:4:\"view\";a:8:{s:5:\"index\";i:0;s:6:\"friend\";i:0;s:4:\"wall\";i:0;s:4:\"home\";i:0;s:5:\"doing\";i:0;s:4:\"blog\";i:0;s:5:\"album\";i:0;s:5:\"share\";i:0;}s:4:\"feed\";a:5:{s:5:\"doing\";i:1;s:4:\"blog\";i:1;s:6:\"upload\";i:1;s:4:\"poll\";i:1;s:9:\"newthread\";i:1;}}s:12:\"profilegroup\";a:5:{s:4:\"base\";a:4:{s:9:\"available\";i:1;s:12:\"displayorder\";i:0;s:5:\"title\";s:12:\"鍩烘湰璧勬枡\";s:5:\"field\";a:17:{s:6:\"gender\";s:6:\"gender\";s:8:\"birthday\";s:8:\"birthday\";s:8:\"realname\";s:8:\"realname\";s:9:\"birthcity\";s:9:\"birthcity\";s:9:\"bloodtype\";s:9:\"bloodtype\";s:10:\"lookingfor\";s:10:\"lookingfor\";s:10:\"residecity\";s:10:\"residecity\";s:10:\"residedist\";s:10:\"residedist\";s:15:\"affectivestatus\";s:15:\"affectivestatus\";s:6:\"field1\";s:6:\"field1\";s:6:\"field2\";s:6:\"field2\";s:6:\"field3\";s:6:\"field3\";s:6:\"field4\";s:6:\"field4\";s:6:\"field5\";s:6:\"field5\";s:6:\"field6\";s:6:\"field6\";s:6:\"field7\";s:6:\"field7\";s:6:\"field8\";s:6:\"field8\";}}s:7:\"contact\";a:4:{s:5:\"title\";s:12:\"鑱旂郴鏂瑰紡\";s:9:\"available\";s:1:\"1\";s:12:\"displayorder\";s:1:\"1\";s:5:\"field\";a:7:{s:2:\"qq\";s:2:\"qq\";s:3:\"msn\";s:3:\"msn\";s:6:\"mobile\";s:6:\"mobile\";s:6:\"taobao\";s:6:\"taobao\";s:9:\"telephone\";s:9:\"telephone\";s:3:\"icq\";s:3:\"icq\";s:5:\"yahoo\";s:5:\"yahoo\";}}s:3:\"edu\";a:4:{s:9:\"available\";i:1;s:12:\"displayorder\";i:2;s:5:\"title\";s:12:\"鏁欒偛鎯呭喌\";s:5:\"field\";a:2:{s:9:\"education\";s:9:\"education\";s:14:\"graduateschool\";s:14:\"graduateschool\";}}s:4:\"work\";a:4:{s:9:\"available\";i:1;s:12:\"displayorder\";i:3;s:5:\"title\";s:12:\"宸ヤ綔鎯呭喌\";s:5:\"field\";a:4:{s:7:\"company\";s:7:\"company\";s:7:\"revenue\";s:7:\"revenue\";s:8:\"position\";s:8:\"position\";s:10:\"occupation\";s:10:\"occupation\";}}s:4:\"info\";a:4:{s:5:\"title\";s:12:\"涓汉淇℃伅\";s:9:\"available\";s:1:\"1\";s:12:\"displayorder\";s:1:\"4\";s:5:\"field\";a:10:{s:3:\"bio\";s:3:\"bio\";s:4:\"site\";s:4:\"site\";s:6:\"idcard\";s:6:\"idcard\";s:7:\"address\";s:7:\"address\";s:7:\"zipcode\";s:7:\"zipcode\";s:8:\"interest\";s:8:\"interest\";s:10:\"idcardtype\";s:10:\"idcardtype\";s:7:\"sightml\";s:7:\"sightml\";s:12:\"customstatus\";s:12:\"customstatus\";s:10:\"timeoffset\";s:10:\"timeoffset\";}}}s:15:\"profilegroupnew\";s:0:\"\";s:14:\"profilehistory\";s:1:\"0\";s:11:\"pvfrequence\";s:2:\"60\";s:8:\"pwlength\";s:1:\"6\";s:5:\"qihoo\";a:9:{s:6:\"status\";i:0;s:9:\"searchbox\";i:6;s:7:\"summary\";i:1;s:6:\"jammer\";i:1;s:9:\"maxtopics\";i:10;s:8:\"keywords\";s:0:\"\";s:10:\"adminemail\";s:0:\"\";s:8:\"validity\";i:1;s:14:\"relatedthreads\";a:6:{s:6:\"bbsnum\";i:0;s:6:\"webnum\";i:0;s:4:\"type\";a:3:{s:4:\"blog\";s:4:\"blog\";s:4:\"news\";s:4:\"news\";s:3:\"bbs\";s:3:\"bbs\";}s:6:\"banurl\";s:0:\"\";s:8:\"position\";i:1;s:8:\"validity\";i:1;}}s:8:\"ranklist\";a:12:{s:6:\"status\";s:1:\"1\";s:10:\"membershow\";s:1:\"1\";s:10:\"cache_time\";s:1:\"1\";s:12:\"index_select\";s:8:\"thisweek\";s:6:\"member\";a:3:{s:9:\"available\";s:1:\"1\";s:10:\"cache_time\";s:1:\"5\";s:8:\"show_num\";s:2:\"20\";}s:6:\"thread\";a:3:{s:9:\"available\";s:1:\"1\";s:10:\"cache_time\";s:1:\"5\";s:8:\"show_num\";s:2:\"20\";}s:4:\"blog\";a:3:{s:9:\"available\";s:1:\"1\";s:10:\"cache_time\";s:1:\"5\";s:8:\"show_num\";s:2:\"20\";}s:4:\"poll\";a:3:{s:9:\"available\";s:1:\"1\";s:10:\"cache_time\";s:1:\"5\";s:8:\"show_num\";s:2:\"20\";}s:8:\"activity\";a:3:{s:9:\"available\";s:1:\"1\";s:10:\"cache_time\";s:1:\"5\";s:8:\"show_num\";s:2:\"20\";}s:7:\"picture\";a:3:{s:9:\"available\";s:1:\"1\";s:10:\"cache_time\";s:1:\"5\";s:8:\"show_num\";s:2:\"20\";}s:5:\"forum\";a:3:{s:9:\"available\";s:1:\"1\";s:10:\"cache_time\";s:1:\"5\";s:8:\"show_num\";s:2:\"20\";}s:5:\"group\";a:3:{s:9:\"available\";s:1:\"1\";s:10:\"cache_time\";s:1:\"5\";s:8:\"show_num\";s:2:\"20\";}}s:14:\"rankliststatus\";s:1:\"0\";s:9:\"ratelogon\";s:1:\"1\";s:13:\"ratelogrecord\";s:2:\"20\";s:8:\"realname\";s:1:\"0\";s:15:\"recommendthread\";a:1:{s:5:\"allow\";i:0;}s:15:\"regclosemessage\";s:0:\"\";s:10:\"regconnect\";s:1:\"1\";s:7:\"regctrl\";s:1:\"0\";s:8:\"regemail\";s:1:\"1\";s:12:\"regfloodctrl\";s:1:\"0\";s:8:\"reginput\";a:4:{s:8:\"username\";s:6:\"R2uy4s\";s:8:\"password\";s:6:\"aDLRD4\";s:9:\"password2\";s:6:\"TqdEa4\";s:5:\"email\";s:6:\"AuUeUq\";}s:11:\"reglinkname\";s:12:\"绔嬪嵆娉ㄥ唽\";s:7:\"regname\";s:8:\"register\";s:9:\"regstatus\";s:1:\"1\";s:9:\"regverify\";s:1:\"0\";s:17:\"relatedlinkstatus\";s:1:\"0\";s:10:\"relatedtag\";b:0;s:9:\"relatenum\";s:2:\"10\";s:10:\"relatetime\";s:2:\"60\";s:11:\"repliesrank\";s:1:\"0\";s:14:\"report_receive\";s:62:\"a:2:{s:9:\"adminuser\";a:1:{i:0;s:1:\"1\";}s:12:\"supmoderator\";N;}\";s:13:\"report_reward\";s:35:\"a:2:{s:3:\"min\";i:-3;s:3:\"max\";i:3;}\";s:16:\"rewardexpiration\";s:2:\"30\";s:13:\"rewardforumid\";s:1:\"0\";s:17:\"rewritecompatible\";s:0:\"\";s:12:\"rewriteguest\";s:1:\"0\";s:13:\"rewritemobile\";s:1:\"0\";s:11:\"rewriterule\";b:0;s:13:\"rewritestatus\";b:0;s:13:\"robotarchiver\";s:1:\"0\";s:9:\"rssstatus\";s:1:\"1\";s:6:\"rssttl\";s:2:\"60\";s:9:\"runwizard\";s:1:\"1\";s:6:\"search\";a:6:{s:6:\"portal\";a:4:{s:6:\"status\";i:1;s:10:\"searchctrl\";i:10;s:6:\"maxspm\";i:10;s:16:\"maxsearchresults\";i:500;}s:5:\"forum\";a:4:{s:6:\"status\";i:1;s:10:\"searchctrl\";i:10;s:6:\"maxspm\";i:10;s:16:\"maxsearchresults\";i:500;}s:4:\"blog\";a:4:{s:6:\"status\";i:1;s:10:\"searchctrl\";i:10;s:6:\"maxspm\";i:10;s:16:\"maxsearchresults\";i:500;}s:5:\"album\";a:4:{s:6:\"status\";i:1;s:10:\"searchctrl\";i:10;s:6:\"maxspm\";i:10;s:16:\"maxsearchresults\";i:500;}s:5:\"group\";a:4:{s:6:\"status\";i:1;s:10:\"searchctrl\";i:10;s:6:\"maxspm\";i:10;s:16:\"maxsearchresults\";i:500;}s:10:\"collection\";a:4:{s:6:\"status\";i:1;s:10:\"searchctrl\";i:10;s:6:\"maxspm\";i:10;s:16:\"maxsearchresults\";i:500;}}s:16:\"searchbanperiods\";s:0:\"\";s:11:\"seccodedata\";a:14:{s:4:\"type\";s:1:\"0\";s:5:\"width\";s:3:\"100\";s:6:\"height\";s:2:\"30\";s:13:\"shuffer_order\";s:1:\"0\";s:7:\"scatter\";s:1:\"0\";s:10:\"background\";s:1:\"0\";s:10:\"adulterate\";s:1:\"1\";s:3:\"ttf\";s:1:\"0\";s:5:\"angle\";s:1:\"0\";s:7:\"warping\";s:1:\"0\";s:5:\"color\";s:1:\"1\";s:4:\"size\";s:1:\"1\";s:6:\"shadow\";s:1:\"0\";s:8:\"animator\";s:1:\"0\";}s:13:\"seccodestatus\";s:2:\"16\";s:8:\"seclevel\";s:1:\"1\";s:14:\"secmobilelogin\";s:1:\"0\";s:6:\"secqaa\";a:5:{s:8:\"statuses\";a:2:{i:0;s:8:\"register\";i:1;s:5:\"login\";}s:8:\"minposts\";i:0;s:4:\"perm\";s:0:\"\";s:9:\"allowcode\";i:1;s:7:\"allowqa\";i:0;}s:14:\"security_email\";s:1:\"1\";s:15:\"security_logoff\";s:1:\"0\";s:15:\"security_mobile\";s:1:\"0\";s:17:\"security_password\";s:1:\"1\";s:17:\"security_question\";s:1:\"1\";s:15:\"security_rename\";s:1:\"1\";s:15:\"security_verify\";s:6:\"a:0:{}\";s:11:\"sendmailday\";s:1:\"0\";s:15:\"sendregisterurl\";s:1:\"0\";s:14:\"seodescription\";b:0;s:7:\"seohead\";s:0:\"\";s:14:\"seohead_mobile\";s:0:\"\";s:11:\"seokeywords\";b:0;s:8:\"seotitle\";a:4:{s:6:\"portal\";s:6:\"闂ㄦ埛\";s:5:\"forum\";s:6:\"璁哄潧\";s:5:\"group\";s:6:\"鍦堝瓙\";s:4:\"home\";s:6:\"瀹跺洯\";}s:11:\"sharestatus\";s:1:\"0\";s:16:\"showallfriendnum\";s:1:\"8\";s:11:\"showavatars\";s:1:\"1\";s:9:\"showemail\";s:0:\"\";s:8:\"showexif\";s:1:\"0\";s:9:\"showfjump\";s:1:\"1\";s:20:\"showfollowcollection\";s:1:\"8\";s:10:\"showimages\";s:1:\"1\";s:11:\"shownewuser\";s:1:\"0\";s:12:\"showsettings\";s:1:\"7\";s:14:\"showsignatures\";s:1:\"1\";s:10:\"showsignin\";s:1:\"1\";s:12:\"showusercard\";s:1:\"1\";s:11:\"sigimgclick\";s:1:\"0\";s:11:\"sigviewcond\";s:1:\"0\";s:10:\"simplemode\";s:1:\"0\";s:7:\"site_qq\";s:0:\"\";s:11:\"sitemessage\";a:5:{s:4:\"time\";i:3000;s:8:\"register\";a:0:{}s:5:\"login\";a:0:{}s:9:\"newthread\";a:0:{}s:5:\"reply\";a:0:{}}s:8:\"sitename\";s:9:\"Discuz! X\";s:12:\"siteuniqueid\";s:16:\"DX0HTR9P773bH0b0\";s:7:\"siteurl\";s:23:\"https://www.discuz.vip/\";s:10:\"sitevipkey\";s:1:\"1\";s:6:\"smcols\";s:1:\"8\";s:6:\"smrows\";s:1:\"5\";s:12:\"smsdefaultcc\";s:2:\"86\";s:16:\"smsdefaultlength\";s:1:\"4\";s:11:\"smsglblimit\";s:4:\"1000\";s:11:\"smsinterval\";s:3:\"300\";s:11:\"smsmillimit\";s:2:\"20\";s:11:\"smsnumlimit\";s:1:\"5\";s:9:\"smsstatus\";s:1:\"0\";s:12:\"smstimelimit\";s:5:\"86400\";s:7:\"smthumb\";s:2:\"20\";s:12:\"sourceheight\";s:0:\"\";s:11:\"sourcewidth\";s:0:\"\";s:9:\"spacedata\";a:11:{s:9:\"cachelife\";s:3:\"900\";s:14:\"limitmythreads\";s:1:\"5\";s:14:\"limitmyreplies\";s:1:\"5\";s:14:\"limitmyrewards\";s:1:\"5\";s:13:\"limitmytrades\";s:1:\"5\";s:13:\"limitmyvideos\";s:1:\"0\";s:12:\"limitmyblogs\";s:1:\"8\";s:14:\"limitmyfriends\";s:1:\"0\";s:16:\"limitmyfavforums\";s:1:\"5\";s:17:\"limitmyfavthreads\";s:1:\"0\";s:10:\"textlength\";s:3:\"300\";}s:11:\"spacestatus\";s:1:\"1\";s:10:\"srchcensor\";s:1:\"1\";s:15:\"srchhotkeywords\";a:3:{i:0;s:6:\"娲诲姩\";i:1;s:6:\"浜ゅ弸\";i:2;s:6:\"discuz\";}s:14:\"stamplistlevel\";s:1:\"3\";s:13:\"starthreshold\";s:1:\"2\";s:8:\"statcode\";s:0:\"\";s:14:\"statscachelife\";s:3:\"180\";s:10:\"statstatus\";s:0:\"\";s:8:\"strongpw\";b:0;s:7:\"styleid\";s:1:\"2\";s:8:\"styleid1\";s:1:\"1\";s:8:\"styleid2\";s:1:\"1\";s:8:\"styleid3\";s:1:\"1\";s:9:\"stylejump\";s:1:\"1\";s:14:\"subforumsindex\";s:1:\"0\";s:10:\"submitlock\";s:1:\"0\";s:15:\"switchwidthauto\";s:1:\"1\";s:9:\"tagstatus\";s:1:\"1\";s:11:\"targetblank\";s:1:\"0\";s:10:\"taskstatus\";s:1:\"0\";s:9:\"tasktypes\";s:241:\"a:3:{s:9:\"promotion\";a:2:{s:4:\"name\";s:18:\"缃戠珯鎺ㄥ箍浠诲姟\";s:7:\"version\";s:3:\"1.0\";}s:4:\"gift\";a:2:{s:4:\"name\";s:15:\"绾㈠寘绫讳换鍔";s:7:\"version\";s:3:\"1.0\";}s:6:\"avatar\";a:2:{s:4:\"name\";s:15:\"澶村儚绫讳换鍔";s:7:\"version\";s:3:\"1.0\";}}\";s:15:\"threadblacklist\";s:1:\"1\";s:15:\"threadfilternum\";s:2:\"10\";s:15:\"threadguestlite\";s:1:\"0\";s:16:\"threadhotreplies\";s:1:\"0\";s:14:\"threadmaxpages\";s:4:\"1000\";s:12:\"threadsticky\";a:3:{i:0;s:12:\"鍏ㄥ眬缃《\";i:1;s:12:\"鍒嗙被缃《\";i:2;s:12:\"鏈増缃《\";}s:11:\"thumbheight\";s:3:\"300\";s:12:\"thumbquality\";s:3:\"100\";s:11:\"thumbsource\";s:1:\"0\";s:11:\"thumbstatus\";s:0:\"\";s:10:\"thumbwidth\";s:3:\"400\";s:10:\"timeformat\";s:3:\"H:i\";s:10:\"timeoffset\";s:1:\"8\";s:12:\"topcachetime\";s:2:\"60\";s:12:\"topicperpage\";s:2:\"20\";s:12:\"tradeforumid\";s:1:\"0\";s:18:\"transfermincredits\";s:4:\"1000\";s:2:\"uc\";a:1:{s:7:\"addfeed\";i:1;}s:12:\"ucactivation\";s:1:\"1\";s:8:\"uidlogin\";s:1:\"0\";s:10:\"updatestat\";s:1:\"1\";s:14:\"userdateformat\";s:26:\"Y-n-j\r\nY/n/j\r\nj-n-Y\r\nj/n/Y\";s:11:\"userreasons\";s:58:\"寰堢粰鍔?\r\n绁為┈閮芥槸娴簯\r\n璧炰竴涓?\r\n灞卞\r\n娣″畾\";s:12:\"userstatusby\";s:1:\"1\";s:6:\"verify\";a:7:{i:6;a:6:{s:5:\"title\";s:12:\"瀹炲悕璁よ瘉\";s:9:\"available\";s:1:\"0\";s:8:\"showicon\";s:1:\"0\";s:12:\"viewrealname\";s:1:\"0\";s:5:\"field\";a:1:{s:8:\"realname\";s:8:\"realname\";}s:4:\"icon\";b:0;}s:7:\"enabled\";b:0;i:1;a:1:{s:4:\"icon\";s:0:\"\";}i:2;a:1:{s:4:\"icon\";s:0:\"\";}i:3;a:1:{s:4:\"icon\";s:0:\"\";}i:4;a:1:{s:4:\"icon\";s:0:\"\";}i:5;a:1:{s:4:\"icon\";s:0:\"\";}}s:16:\"video_allowalbum\";s:1:\"0\";s:15:\"video_allowblog\";s:1:\"0\";s:18:\"video_allowcomment\";s:1:\"0\";s:16:\"video_allowdoing\";s:1:\"1\";s:17:\"video_allowfriend\";s:1:\"1\";s:15:\"video_allowpoke\";s:1:\"1\";s:16:\"video_allowshare\";s:1:\"0\";s:20:\"video_allowviewspace\";s:1:\"1\";s:15:\"video_allowwall\";s:1:\"1\";s:14:\"viewthreadtags\";s:3:\"100\";s:15:\"visitbanperiods\";s:0:\"\";s:13:\"visitedforums\";s:2:\"10\";s:14:\"visitedthreads\";s:1:\"0\";s:14:\"vtonlinestatus\";s:1:\"1\";s:10:\"wallstatus\";s:1:\"0\";s:10:\"wapcharset\";s:1:\"0\";s:13:\"wapdateformat\";s:3:\"n/j\";s:6:\"wapmps\";s:3:\"500\";s:6:\"wapppp\";s:1:\"5\";s:11:\"wapregister\";s:1:\"0\";s:9:\"wapstatus\";s:1:\"0\";s:6:\"waptpp\";s:2:\"10\";s:17:\"warningexpiration\";s:2:\"30\";s:12:\"warninglimit\";s:1:\"3\";s:18:\"watermarkminheight\";s:67:\"a:3:{s:6:\"portal\";s:1:\"0\";s:5:\"forum\";s:1:\"0\";s:5:\"album\";s:1:\"0\";}\";s:17:\"watermarkminwidth\";s:67:\"a:3:{s:6:\"portal\";s:1:\"0\";s:5:\"forum\";s:1:\"0\";s:5:\"album\";s:1:\"0\";}\";s:16:\"watermarkquality\";s:62:\"a:3:{s:6:\"portal\";s:2:\"90\";s:5:\"forum\";i:90;s:5:\"album\";i:90;}\";s:15:\"watermarkstatus\";s:67:\"a:3:{s:6:\"portal\";s:1:\"0\";s:5:\"forum\";s:1:\"0\";s:5:\"album\";s:1:\"0\";}\";s:13:\"watermarktext\";a:12:{s:4:\"text\";a:3:{s:6:\"portal\";s:0:\"\";s:5:\"forum\";s:0:\"\";s:5:\"album\";s:0:\"\";}s:8:\"fontpath\";a:3:{s:6:\"portal\";s:0:\"\";s:5:\"forum\";s:0:\"\";s:5:\"album\";s:0:\"\";}s:4:\"size\";a:3:{s:6:\"portal\";s:0:\"\";s:5:\"forum\";s:0:\"\";s:5:\"album\";s:0:\"\";}s:5:\"angle\";a:3:{s:6:\"portal\";s:0:\"\";s:5:\"forum\";s:0:\"\";s:5:\"album\";s:0:\"\";}s:5:\"color\";a:3:{s:6:\"portal\";s:0:\"\";s:5:\"forum\";s:0:\"\";s:5:\"album\";s:0:\"\";}s:7:\"shadowx\";a:3:{s:6:\"portal\";s:0:\"\";s:5:\"forum\";s:0:\"\";s:5:\"album\";s:0:\"\";}s:7:\"shadowy\";a:3:{s:6:\"portal\";s:0:\"\";s:5:\"forum\";s:0:\"\";s:5:\"album\";s:0:\"\";}s:11:\"shadowcolor\";a:3:{s:6:\"portal\";s:0:\"\";s:5:\"forum\";s:0:\"\";s:5:\"album\";s:0:\"\";}s:10:\"translatex\";a:3:{s:6:\"portal\";s:0:\"\";s:5:\"forum\";s:0:\"\";s:5:\"album\";s:0:\"\";}s:10:\"translatey\";a:3:{s:6:\"portal\";s:0:\"\";s:5:\"forum\";s:0:\"\";s:5:\"album\";s:0:\"\";}s:5:\"skewx\";a:3:{s:6:\"portal\";s:0:\"\";s:5:\"forum\";s:0:\"\";s:5:\"album\";s:0:\"\";}s:5:\"skewy\";a:3:{s:6:\"portal\";s:0:\"\";s:5:\"forum\";s:0:\"\";s:5:\"album\";s:0:\"\";}}s:14:\"watermarktrans\";s:62:\"a:3:{s:6:\"portal\";s:2:\"50\";s:5:\"forum\";i:50;s:5:\"album\";i:50;}\";s:13:\"watermarktype\";a:3:{s:6:\"portal\";s:3:\"png\";s:5:\"forum\";s:3:\"png\";s:5:\"album\";s:3:\"png\";}s:10:\"welcomemsg\";s:1:\"1\";s:15:\"welcomemsgtitle\";s:67:\"{username}锛屾偍濂斤紝鎰熻阿鎮ㄧ殑娉ㄥ唽锛岃闃呰浠ヤ笅鍐呭銆俓";s:13:\"welcomemsgtxt\";s:213:\"灏婃暚鐨剓username}锛屾偍宸茬粡娉ㄥ唽鎴愪负{sitename}鐨勪細鍛橈紝璇锋偍鍦ㄥ彂琛ㄨ█璁烘椂锛岄伒瀹堝綋鍦版硶寰嬫硶瑙勩€俓r\n濡傛灉鎮ㄦ湁浠€涔堢枒闂彲浠ヨ仈绯荤鐞嗗憳锛孍mail: {adminemail}銆俓r\n\r\n\r\n{bbname}\r\n{time}\";s:19:\"whosonline_contract\";s:1:\"0\";s:16:\"whosonlinestatus\";s:1:\"3\";s:10:\"zoomstatus\";s:1:\"1\";s:14:\"newusergroupid\";s:2:\"10\";s:18:\"buyusergroupexists\";s:1:\"0\";s:9:\"forumfids\";a:0:{}s:7:\"version\";s:4:\"X5.0\";s:13:\"cachethreadon\";i:0;s:8:\"iconfont\";s:21:\"static/js/iconfont.js\";s:6:\"styles\";a:2:{i:1;s:12:\"榛樿椋庢牸\";i:2;s:8:\"X5妯＄増\";}s:11:\"creditnames\";s:29:\"1|濞佹湜|,2|閲戦挶|,3|璐＄尞|\";s:17:\"creditstransextra\";a:13:{i:1;s:1:\"2\";i:2;s:1:\"2\";i:3;s:1:\"2\";i:4;s:1:\"2\";i:5;s:1:\"2\";i:6;s:1:\"2\";i:7;s:1:\"2\";i:8;s:1:\"2\";i:9;s:1:\"2\";i:10;s:1:\"2\";i:11;s:1:\"2\";i:12;s:1:\"2\";i:13;s:1:\"2\";}s:14:\"exchangestatus\";b:0;s:14:\"transferstatus\";b:1;s:10:\"ucenterurl\";s:1:\".\";s:9:\"avatarurl\";s:13:\"./data/avatar\";s:10:\"avatarpath\";s:12:\"data/avatar/\";s:13:\"defaultavatar\";s:26:\"./data/avatar/noavatar.svg\";s:9:\"tradeopen\";i:1;s:7:\"plugins\";a:2:{s:9:\"available\";a:0:{}s:4:\"func\";a:0:{}}s:11:\"pluginlinks\";a:0:{}s:10:\"hookscript\";a:0:{}s:16:\"hookscriptmobile\";a:0:{}s:13:\"threadplugins\";a:0:{}s:11:\"specialicon\";a:0:{}s:4:\"navs\";a:4:{i:2;a:7:{s:7:\"navname\";s:6:\"璁哄潧\";s:8:\"filename\";s:9:\"forum.php\";s:9:\"available\";s:1:\"1\";s:4:\"data\";a:19:{s:2:\"id\";s:1:\"2\";s:8:\"parentid\";s:1:\"0\";s:4:\"name\";s:6:\"璁哄潧\";s:5:\"title\";s:3:\"BBS\";s:3:\"url\";s:9:\"forum.php\";s:10:\"identifier\";s:1:\"2\";s:6:\"target\";s:1:\"0\";s:4:\"type\";s:1:\"0\";s:9:\"available\";s:1:\"1\";s:12:\"displayorder\";s:1:\"2\";s:9:\"highlight\";s:1:\"0\";s:5:\"level\";s:1:\"0\";s:7:\"subtype\";s:1:\"0\";s:7:\"subcols\";s:1:\"0\";s:4:\"icon\";s:0:\"\";s:7:\"subname\";s:0:\"\";s:6:\"suburl\";s:0:\"\";s:7:\"navtype\";s:1:\"0\";s:4:\"logo\";s:0:\"\";}s:5:\"navid\";s:8:\"mn_forum\";s:5:\"level\";s:1:\"0\";s:3:\"nav\";s:91:\"id=\"mn_forum\" ><a href=\"forum.php\" hidefocus=\"true\" title=\"BBS\"  >璁哄潧<span>BBS</span></a\";}i:6;a:3:{s:7:\"navname\";s:6:\"鎻掍欢\";s:8:\"filename\";s:1:\"#\";s:9:\"available\";i:0;}i:7;a:7:{s:7:\"navname\";s:6:\"甯姪\";s:8:\"filename\";s:16:\"misc.php?mod=faq\";s:9:\"available\";s:1:\"0\";s:4:\"data\";a:19:{s:2:\"id\";s:1:\"6\";s:8:\"parentid\";s:1:\"0\";s:4:\"name\";s:6:\"甯姪\";s:5:\"title\";s:4:\"Help\";s:3:\"url\";s:16:\"misc.php?mod=faq\";s:10:\"identifier\";s:1:\"7\";s:6:\"target\";s:1:\"0\";s:4:\"type\";s:1:\"0\";s:9:\"available\";s:1:\"0\";s:12:\"displayorder\";s:2:\"10\";s:9:\"highlight\";s:1:\"0\";s:5:\"level\";s:1:\"0\";s:7:\"subtype\";s:1:\"0\";s:7:\"subcols\";s:1:\"0\";s:4:\"icon\";s:0:\"\";s:7:\"subname\";s:0:\"\";s:6:\"suburl\";s:0:\"\";s:7:\"navtype\";s:1:\"0\";s:4:\"logo\";s:0:\"\";}s:5:\"navid\";s:8:\"mn_N0a2c\";s:5:\"level\";s:1:\"0\";s:3:\"nav\";s:100:\"id=\"mn_N0a2c\" ><a href=\"misc.php?mod=faq\" hidefocus=\"true\" title=\"Help\"  >甯姪<span>Help</span></a\";}i:16;a:7:{s:7:\"navname\";s:6:\"棣栭〉\";s:8:\"filename\";s:32:\"forum.php?mod=forumdisplay&fid=0\";s:9:\"available\";s:1:\"0\";s:4:\"data\";a:19:{s:2:\"id\";s:2:\"15\";s:8:\"parentid\";s:1:\"0\";s:4:\"name\";s:6:\"棣栭〉\";s:5:\"title\";s:5:\"Index\";s:3:\"url\";s:32:\"forum.php?mod=forumdisplay&fid=0\";s:10:\"identifier\";s:2:\"16\";s:6:\"target\";s:1:\"0\";s:4:\"type\";s:1:\"0\";s:9:\"available\";s:1:\"0\";s:12:\"displayorder\";s:2:\"16\";s:9:\"highlight\";s:1:\"0\";s:5:\"level\";s:1:\"0\";s:7:\"subtype\";s:1:\"0\";s:7:\"subcols\";s:1:\"0\";s:4:\"icon\";s:0:\"\";s:7:\"subname\";s:0:\"\";s:6:\"suburl\";s:0:\"\";s:7:\"navtype\";s:1:\"0\";s:4:\"logo\";s:0:\"\";}s:5:\"navid\";s:11:\"mn_forum_16\";s:5:\"level\";s:1:\"0\";s:3:\"nav\";s:121:\"id=\"mn_forum_16\" ><a href=\"forum.php?mod=forumdisplay&fid=0\" hidefocus=\"true\" title=\"Index\"  >棣栭〉<span>Index</span></a\";}}s:7:\"subnavs\";a:0:{}s:8:\"menunavs\";s:0:\"\";s:6:\"navmns\";a:2:{s:8:\"misc.php\";a:1:{i:0;a:2:{i:0;a:1:{s:3:\"mod\";s:3:\"faq\";}i:1;s:8:\"mn_N0a2c\";}}s:9:\"forum.php\";a:1:{i:0;a:2:{i:0;a:2:{s:3:\"mod\";s:12:\"forumdisplay\";s:3:\"fid\";s:1:\"0\";}i:1;s:11:\"mn_forum_16\";}}}s:5:\"navmn\";a:1:{s:9:\"forum.php\";s:8:\"mn_forum\";}s:6:\"navdms\";a:0:{}s:8:\"navlogos\";N;s:10:\"footernavs\";a:5:{s:4:\"stat\";a:7:{s:4:\"data\";a:19:{s:2:\"id\";s:2:\"16\";s:8:\"parentid\";s:1:\"0\";s:4:\"name\";s:12:\"绔欑偣缁熻\";s:5:\"title\";s:0:\"\";s:3:\"url\";s:17:\"misc.php?mod=stat\";s:10:\"identifier\";s:4:\"stat\";s:6:\"target\";s:1:\"0\";s:4:\"type\";s:1:\"0\";s:9:\"available\";s:1:\"1\";s:12:\"displayorder\";s:1:\"1\";s:9:\"highlight\";s:1:\"0\";s:5:\"level\";s:1:\"0\";s:7:\"subtype\";s:1:\"0\";s:7:\"subcols\";s:1:\"0\";s:4:\"icon\";s:0:\"\";s:7:\"subname\";s:0:\"\";s:6:\"suburl\";s:0:\"\";s:7:\"navtype\";s:1:\"1\";s:4:\"logo\";s:0:\"\";}s:9:\"available\";s:1:\"1\";s:7:\"navname\";s:12:\"绔欑偣缁熻\";s:4:\"code\";s:45:\"<a href=\"misc.php?mod=stat\" >绔欑偣缁熻</a>\";s:4:\"type\";s:1:\"0\";s:5:\"level\";s:1:\"0\";s:2:\"id\";s:4:\"stat\";}s:6:\"report\";a:7:{s:4:\"data\";a:19:{s:2:\"id\";s:2:\"17\";s:8:\"parentid\";s:1:\"0\";s:4:\"name\";s:6:\"涓炬姤\";s:5:\"title\";s:0:\"\";s:3:\"url\";s:1:\"#\";s:10:\"identifier\";s:6:\"report\";s:6:\"target\";s:1:\"0\";s:4:\"type\";s:1:\"0\";s:9:\"available\";s:1:\"1\";s:12:\"displayorder\";s:1:\"2\";s:9:\"highlight\";s:1:\"0\";s:5:\"level\";s:1:\"0\";s:7:\"subtype\";s:1:\"0\";s:7:\"subcols\";s:1:\"0\";s:4:\"icon\";s:0:\"\";s:7:\"subname\";s:0:\"\";s:6:\"suburl\";s:0:\"\";s:7:\"navtype\";s:1:\"1\";s:4:\"logo\";s:0:\"\";}s:9:\"available\";s:1:\"1\";s:7:\"navname\";s:6:\"涓炬姤\";s:4:\"code\";s:121:\"<a href=\"javascript:;\"  onclick=\"showWindow(\'miscreport\', \'misc.php?mod=report&url=\'+REPORTURL);return false;\">涓炬姤</a>\";s:4:\"type\";s:1:\"0\";s:5:\"level\";s:1:\"0\";s:2:\"id\";s:6:\"report\";}s:8:\"archiver\";a:7:{s:4:\"data\";a:19:{s:2:\"id\";s:2:\"18\";s:8:\"parentid\";s:1:\"0\";s:4:\"name\";s:8:\"Archiver\";s:5:\"title\";s:0:\"\";s:3:\"url\";s:9:\"archiver/\";s:10:\"identifier\";s:8:\"archiver\";s:6:\"target\";s:1:\"0\";s:4:\"type\";s:1:\"0\";s:9:\"available\";s:1:\"1\";s:12:\"displayorder\";s:1:\"3\";s:9:\"highlight\";s:1:\"0\";s:5:\"level\";s:1:\"0\";s:7:\"subtype\";s:1:\"0\";s:7:\"subcols\";s:1:\"0\";s:4:\"icon\";s:0:\"\";s:7:\"subname\";s:0:\"\";s:6:\"suburl\";s:0:\"\";s:7:\"navtype\";s:1:\"1\";s:4:\"logo\";s:0:\"\";}s:9:\"available\";s:1:\"1\";s:7:\"navname\";s:8:\"Archiver\";s:4:\"code\";s:33:\"<a href=\"archiver/\" >Archiver</a>\";s:4:\"type\";s:1:\"0\";s:5:\"level\";s:1:\"0\";s:2:\"id\";s:8:\"archiver\";}s:6:\"mobile\";a:7:{s:4:\"data\";a:19:{s:2:\"id\";s:2:\"19\";s:8:\"parentid\";s:1:\"0\";s:4:\"name\";s:9:\"鎵嬫満鐗圽";s:5:\"title\";s:0:\"\";s:3:\"url\";s:24:\"forum.php?showmobile=yes\";s:10:\"identifier\";s:6:\"mobile\";s:6:\"target\";s:1:\"0\";s:4:\"type\";s:1:\"0\";s:9:\"available\";s:1:\"1\";s:12:\"displayorder\";s:1:\"3\";s:9:\"highlight\";s:1:\"0\";s:5:\"level\";s:1:\"0\";s:7:\"subtype\";s:1:\"0\";s:7:\"subcols\";s:1:\"0\";s:4:\"icon\";s:0:\"\";s:7:\"subname\";s:0:\"\";s:6:\"suburl\";s:0:\"\";s:7:\"navtype\";s:1:\"1\";s:4:\"logo\";s:0:\"\";}s:9:\"available\";s:1:\"1\";s:7:\"navname\";s:9:\"鎵嬫満鐗圽";s:4:\"code\";s:49:\"<a href=\"forum.php?showmobile=yes\" >鎵嬫満鐗?/a>\";s:4:\"type\";s:1:\"0\";s:5:\"level\";s:1:\"0\";s:2:\"id\";s:6:\"mobile\";}s:8:\"darkroom\";a:7:{s:4:\"data\";a:19:{s:2:\"id\";s:2:\"20\";s:8:\"parentid\";s:1:\"0\";s:4:\"name\";s:9:\"灏忛粦灞媆";s:5:\"title\";s:0:\"\";s:3:\"url\";s:21:\"misc.php?mod=darkroom\";s:10:\"identifier\";s:8:\"darkroom\";s:6:\"target\";s:1:\"0\";s:4:\"type\";s:1:\"0\";s:9:\"available\";s:1:\"1\";s:12:\"displayorder\";s:1:\"3\";s:9:\"highlight\";s:1:\"0\";s:5:\"level\";s:1:\"0\";s:7:\"subtype\";s:1:\"0\";s:7:\"subcols\";s:1:\"0\";s:4:\"icon\";s:0:\"\";s:7:\"subname\";s:0:\"\";s:6:\"suburl\";s:0:\"\";s:7:\"navtype\";s:1:\"1\";s:4:\"logo\";s:0:\"\";}s:9:\"available\";s:1:\"1\";s:7:\"navname\";s:9:\"灏忛粦灞媆";s:4:\"code\";s:46:\"<a href=\"misc.php?mod=darkroom\" >灏忛粦灞?/a>\";s:4:\"type\";s:1:\"0\";s:5:\"level\";s:1:\"0\";s:2:\"id\";s:8:\"darkroom\";}}s:9:\"spacenavs\";a:0:{}s:6:\"mynavs\";a:1:{s:6:\"thread\";a:6:{s:4:\"data\";a:19:{s:2:\"id\";s:2:\"22\";s:8:\"parentid\";s:1:\"0\";s:4:\"name\";s:6:\"甯栧瓙\";s:5:\"title\";s:0:\"\";s:3:\"url\";s:36:\"home.php?mod=space&do=thread&view=me\";s:10:\"identifier\";s:6:\"thread\";s:6:\"target\";s:1:\"0\";s:4:\"type\";s:1:\"0\";s:9:\"available\";s:1:\"1\";s:12:\"displayorder\";s:1:\"2\";s:9:\"highlight\";s:1:\"0\";s:5:\"level\";s:1:\"0\";s:7:\"subtype\";s:1:\"0\";s:7:\"subcols\";s:1:\"0\";s:4:\"icon\";s:30:\"{STATICURL}image/app/forum.svg\";s:7:\"subname\";s:0:\"\";s:6:\"suburl\";s:0:\"\";s:7:\"navtype\";s:1:\"3\";s:4:\"logo\";s:0:\"\";}s:9:\"available\";s:1:\"1\";s:7:\"navname\";s:6:\"甯栧瓙\";s:4:\"code\";s:148:\"<a href=\"home.php?mod=space&do=thread&view=me\" style=\"background-image:url(https://localhost:8080/static/image/app/forum.svg) !important\">甯栧瓙</a>\";s:5:\"level\";s:1:\"0\";s:4:\"icon\";s:91:\" style=\"background-image:url(https://localhost:8080/static/image/app/forum.svg) !important\"\";}}s:7:\"topnavs\";a:0:{}s:11:\"profilenode\";a:2:{s:8:\"template\";a:1:{i:0;a:2:{s:4:\"left\";s:187:\"{U6UhID1c}\r\n{wxPs88kP}\r\n{uPDwlheD}\r\n{HQLlNpKX}\r\n{JigAGLHI}\r\n{ti37C8I2}\r\n<dl class=\"pil cl\">\r\n	<dt>{eS0DF03n}</dt><dd>{tr3euJWB}</dd>\r\n</dl>\r\n{gQyiFZk9}\r\n<dl class=\"pil cl\">{PX8d2wFx}</dl>\";s:3:\"top\";s:62:\"<dl class=\"cl\">\r\n<dt>{f6Ev2LY6}</dt><dd>{FfQo3otK}</dd>\r\n</dl>\";}}s:4:\"code\";a:1:{i:0;a:2:{s:4:\"left\";a:10:{s:10:\"{U6UhID1c}\";a:4:{i:0;s:10:\"numbercard\";i:1;s:0:\"\";i:2;s:0:\"\";i:3;s:0:\"\";}s:10:\"{wxPs88kP}\";a:4:{i:0;s:9:\"groupicon\";i:1;s:3:\"<p>\";i:2;s:4:\"</p>\";i:3;s:0:\"\";}s:10:\"{uPDwlheD}\";a:4:{i:0;s:11:\"authortitle\";i:1;s:7:\"<p><em>\";i:2;s:9:\"</em></p>\";i:3;s:0:\"\";}s:10:\"{HQLlNpKX}\";a:4:{i:0;s:12:\"customstatus\";i:1;s:15:\"<p class=\"xg1\">\";i:2;s:4:\"</p>\";i:3;s:0:\"\";}s:10:\"{JigAGLHI}\";a:4:{i:0;s:4:\"star\";i:1;s:3:\"<p>\";i:2;s:4:\"</p>\";i:3;s:0:\"\";}s:10:\"{ti37C8I2}\";a:4:{i:0;s:15:\"upgradeprogress\";i:1;s:3:\"<p>\";i:2;s:4:\"</p>\";i:3;s:0:\"\";}s:10:\"{eS0DF03n}\";a:4:{i:0;s:8:\"baseinfo\";i:1;s:0:\"\";i:2;s:0:\"\";i:3;s:9:\"credits,1\";}s:10:\"{tr3euJWB}\";a:4:{i:0;s:8:\"baseinfo\";i:1;s:0:\"\";i:2;s:0:\"\";i:3;s:9:\"credits,0\";}s:10:\"{gQyiFZk9}\";a:4:{i:0;s:5:\"medal\";i:1;s:19:\"<p class=\"md_ctrl\">\";i:2;s:4:\"</p>\";i:3;s:0:\"\";}s:10:\"{PX8d2wFx}\";a:4:{i:0;s:8:\"baseinfo\";i:1;s:0:\"\";i:2;s:0:\"\";i:3;s:10:\"field_qq,0\";}}s:3:\"top\";a:2:{s:10:\"{f6Ev2LY6}\";a:4:{i:0;s:8:\"baseinfo\";i:1;s:0:\"\";i:2;s:0:\"\";i:3;s:9:\"credits,1\";}s:10:\"{FfQo3otK}\";a:4:{i:0;s:8:\"baseinfo\";i:1;s:0:\"\";i:2;s:0:\"\";i:3;s:9:\"credits,0\";}}}}}s:9:\"mfindnavs\";a:7:{s:6:\"search\";a:8:{s:4:\"data\";a:19:{s:2:\"id\";s:2:\"41\";s:8:\"parentid\";s:1:\"0\";s:4:\"name\";s:6:\"鎼滅储\";s:5:\"title\";s:0:\"\";s:3:\"url\";s:20:\"search.php?mod=forum\";s:10:\"identifier\";s:6:\"search\";s:6:\"target\";s:1:\"0\";s:4:\"type\";s:1:\"0\";s:9:\"available\";s:1:\"1\";s:12:\"displayorder\";s:1:\"1\";s:9:\"highlight\";s:1:\"0\";s:5:\"level\";s:1:\"0\";s:7:\"subtype\";s:1:\"0\";s:7:\"subcols\";s:1:\"0\";s:4:\"icon\";s:0:\"\";s:7:\"subname\";s:0:\"\";s:6:\"suburl\";s:0:\"\";s:7:\"navtype\";s:1:\"5\";s:4:\"logo\";s:0:\"\";}s:9:\"available\";s:1:\"1\";s:7:\"navname\";s:6:\"鎼滅储\";s:3:\"url\";s:20:\"search.php?mod=forum\";s:4:\"name\";s:6:\"鎼滅储\";s:4:\"type\";s:1:\"0\";s:5:\"level\";s:1:\"0\";s:2:\"id\";s:6:\"search\";}s:12:\"announcement\";a:8:{s:4:\"data\";a:19:{s:2:\"id\";s:2:\"42\";s:8:\"parentid\";s:1:\"0\";s:4:\"name\";s:6:\"鍏憡\";s:5:\"title\";s:0:\"\";s:3:\"url\";s:26:\"forum.php?mod=announcement\";s:10:\"identifier\";s:12:\"announcement\";s:6:\"target\";s:1:\"0\";s:4:\"type\";s:1:\"0\";s:9:\"available\";s:1:\"1\";s:12:\"displayorder\";s:1:\"1\";s:9:\"highlight\";s:1:\"0\";s:5:\"level\";s:1:\"0\";s:7:\"subtype\";s:1:\"0\";s:7:\"subcols\";s:1:\"0\";s:4:\"icon\";s:0:\"\";s:7:\"subname\";s:0:\"\";s:6:\"suburl\";s:0:\"\";s:7:\"navtype\";s:1:\"5\";s:4:\"logo\";s:0:\"\";}s:9:\"available\";s:1:\"1\";s:7:\"navname\";s:6:\"鍏憡\";s:3:\"url\";s:26:\"forum.php?mod=announcement\";s:4:\"name\";s:6:\"鍏憡\";s:4:\"type\";s:1:\"0\";s:5:\"level\";s:1:\"0\";s:2:\"id\";s:12:\"announcement\";}s:5:\"guide\";a:8:{s:4:\"data\";a:19:{s:2:\"id\";s:2:\"43\";s:8:\"parentid\";s:1:\"0\";s:4:\"name\";s:6:\"瀵艰\";s:5:\"title\";s:0:\"\";s:3:\"url\";s:34:\"forum.php?mod=guide&view=newthread\";s:10:\"identifier\";s:5:\"guide\";s:6:\"target\";s:1:\"0\";s:4:\"type\";s:1:\"0\";s:9:\"available\";s:1:\"0\";s:12:\"displayorder\";s:1:\"1\";s:9:\"highlight\";s:1:\"0\";s:5:\"level\";s:1:\"0\";s:7:\"subtype\";s:1:\"0\";s:7:\"subcols\";s:1:\"0\";s:4:\"icon\";s:0:\"\";s:7:\"subname\";s:0:\"\";s:6:\"suburl\";s:0:\"\";s:7:\"navtype\";s:1:\"5\";s:4:\"logo\";s:0:\"\";}s:9:\"available\";s:1:\"0\";s:7:\"navname\";s:6:\"瀵艰\";s:3:\"url\";s:34:\"forum.php?mod=guide&view=newthread\";s:4:\"name\";s:6:\"瀵艰\";s:4:\"type\";s:1:\"0\";s:5:\"level\";s:1:\"0\";s:2:\"id\";s:5:\"guide\";}s:6:\"portal\";a:8:{s:4:\"data\";a:19:{s:2:\"id\";s:2:\"44\";s:8:\"parentid\";s:1:\"0\";s:4:\"name\";s:6:\"璧勮\";s:5:\"title\";s:0:\"\";s:3:\"url\";s:27:\"portal.php?mod=list&catid=1\";s:10:\"identifier\";s:6:\"portal\";s:6:\"target\";s:1:\"0\";s:4:\"type\";s:1:\"0\";s:9:\"available\";s:1:\"0\";s:12:\"displayorder\";s:1:\"1\";s:9:\"highlight\";s:1:\"0\";s:5:\"level\";s:1:\"0\";s:7:\"subtype\";s:1:\"0\";s:7:\"subcols\";s:1:\"0\";s:4:\"icon\";s:0:\"\";s:7:\"subname\";s:0:\"\";s:6:\"suburl\";s:0:\"\";s:7:\"navtype\";s:1:\"5\";s:4:\"logo\";s:0:\"\";}s:9:\"available\";s:1:\"0\";s:7:\"navname\";s:6:\"璧勮\";s:3:\"url\";s:27:\"portal.php?mod=list&catid=1\";s:4:\"name\";s:6:\"璧勮\";s:4:\"type\";s:1:\"0\";s:5:\"level\";s:1:\"0\";s:2:\"id\";s:6:\"portal\";}s:4:\"blog\";a:8:{s:4:\"data\";a:19:{s:2:\"id\";s:2:\"45\";s:8:\"parentid\";s:1:\"0\";s:4:\"name\";s:6:\"鏃ュ織\";s:5:\"title\";s:0:\"\";s:3:\"url\";s:26:\"home.php?mod=space&do=blog\";s:10:\"identifier\";s:4:\"blog\";s:6:\"target\";s:1:\"0\";s:4:\"type\";s:1:\"0\";s:9:\"available\";s:1:\"0\";s:12:\"displayorder\";s:1:\"1\";s:9:\"highlight\";s:1:\"0\";s:5:\"level\";s:1:\"0\";s:7:\"subtype\";s:1:\"0\";s:7:\"subcols\";s:1:\"0\";s:4:\"icon\";s:0:\"\";s:7:\"subname\";s:0:\"\";s:6:\"suburl\";s:0:\"\";s:7:\"navtype\";s:1:\"5\";s:4:\"logo\";s:0:\"\";}s:9:\"available\";s:1:\"0\";s:7:\"navname\";s:6:\"鏃ュ織\";s:3:\"url\";s:26:\"home.php?mod=space&do=blog\";s:4:\"name\";s:6:\"鏃ュ織\";s:4:\"type\";s:1:\"0\";s:5:\"level\";s:1:\"0\";s:2:\"id\";s:4:\"blog\";}s:5:\"share\";a:8:{s:4:\"data\";a:19:{s:2:\"id\";s:2:\"46\";s:8:\"parentid\";s:1:\"0\";s:4:\"name\";s:6:\"鍒嗕韩\";s:5:\"title\";s:0:\"\";s:3:\"url\";s:27:\"home.php?mod=space&do=share\";s:10:\"identifier\";s:5:\"share\";s:6:\"target\";s:1:\"0\";s:4:\"type\";s:1:\"0\";s:9:\"available\";s:1:\"0\";s:12:\"displayorder\";s:1:\"1\";s:9:\"highlight\";s:1:\"0\";s:5:\"level\";s:1:\"0\";s:7:\"subtype\";s:1:\"0\";s:7:\"subcols\";s:1:\"0\";s:4:\"icon\";s:0:\"\";s:7:\"subname\";s:0:\"\";s:6:\"suburl\";s:0:\"\";s:7:\"navtype\";s:1:\"5\";s:4:\"logo\";s:0:\"\";}s:9:\"available\";s:1:\"0\";s:7:\"navname\";s:6:\"鍒嗕韩\";s:3:\"url\";s:27:\"home.php?mod=space&do=share\";s:4:\"name\";s:6:\"鍒嗕韩\";s:4:\"type\";s:1:\"0\";s:5:\"level\";s:1:\"0\";s:2:\"id\";s:5:\"share\";}s:8:\"ranklist\";a:8:{s:4:\"data\";a:19:{s:2:\"id\";s:2:\"47\";s:8:\"parentid\";s:1:\"0\";s:4:\"name\";s:9:\"鎺掕姒淺";s:5:\"title\";s:0:\"\";s:3:\"url\";s:21:\"misc.php?mod=ranklist\";s:10:\"identifier\";s:8:\"ranklist\";s:6:\"target\";s:1:\"0\";s:4:\"type\";s:1:\"0\";s:9:\"available\";s:1:\"0\";s:12:\"displayorder\";s:1:\"1\";s:9:\"highlight\";s:1:\"0\";s:5:\"level\";s:1:\"0\";s:7:\"subtype\";s:1:\"0\";s:7:\"subcols\";s:1:\"0\";s:4:\"icon\";s:0:\"\";s:7:\"subname\";s:0:\"\";s:6:\"suburl\";s:0:\"\";s:7:\"navtype\";s:1:\"5\";s:4:\"logo\";s:0:\"\";}s:9:\"available\";s:1:\"0\";s:7:\"navname\";s:9:\"鎺掕姒淺";s:3:\"url\";s:21:\"misc.php?mod=ranklist\";s:4:\"name\";s:9:\"鎺掕姒淺";s:4:\"type\";s:1:\"0\";s:5:\"level\";s:1:\"0\";s:2:\"id\";s:8:\"ranklist\";}}s:5:\"mnavs\";a:5:{i:148;a:10:{s:4:\"data\";a:19:{s:2:\"id\";s:2:\"48\";s:8:\"parentid\";s:1:\"0\";s:4:\"name\";s:6:\"棣栭〉\";s:5:\"title\";s:0:\"\";s:3:\"url\";s:9:\"index.php\";s:10:\"identifier\";s:0:\"\";s:6:\"target\";s:1:\"0\";s:4:\"type\";s:1:\"1\";s:9:\"available\";s:1:\"1\";s:12:\"displayorder\";s:1:\"1\";s:9:\"highlight\";s:1:\"0\";s:5:\"level\";s:1:\"0\";s:7:\"subtype\";s:1:\"0\";s:7:\"subcols\";s:1:\"0\";s:4:\"icon\";s:35:\"/static/image/mobile/touch/home.svg\";s:7:\"subname\";s:0:\"\";s:6:\"suburl\";s:0:\"\";s:7:\"navtype\";s:1:\"6\";s:4:\"logo\";s:0:\"\";}s:9:\"available\";s:1:\"1\";s:7:\"navname\";s:6:\"棣栭〉\";s:3:\"url\";s:9:\"index.php\";s:4:\"name\";s:6:\"棣栭〉\";s:4:\"type\";s:1:\"1\";s:5:\"level\";s:1:\"0\";s:2:\"id\";s:0:\"\";s:4:\"icon\";s:85:\"<img src=\"https://localhost:8080//static/image/mobile/touch/home.svg\" alt=\"棣栭〉\" />\";s:7:\"is_post\";b:0;}i:149;a:10:{s:4:\"data\";a:19:{s:2:\"id\";s:2:\"49\";s:8:\"parentid\";s:1:\"0\";s:4:\"name\";s:6:\"璁哄潧\";s:5:\"title\";s:0:\"\";s:3:\"url\";s:9:\"forum.php\";s:10:\"identifier\";s:0:\"\";s:6:\"target\";s:1:\"0\";s:4:\"type\";s:1:\"1\";s:9:\"available\";s:1:\"1\";s:12:\"displayorder\";s:1:\"2\";s:9:\"highlight\";s:1:\"0\";s:5:\"level\";s:1:\"0\";s:7:\"subtype\";s:1:\"0\";s:7:\"subcols\";s:1:\"0\";s:4:\"icon\";s:36:\"/static/image/mobile/touch/forum.svg\";s:7:\"subname\";s:0:\"\";s:6:\"suburl\";s:0:\"\";s:7:\"navtype\";s:1:\"6\";s:4:\"logo\";s:0:\"\";}s:9:\"available\";s:1:\"1\";s:7:\"navname\";s:6:\"璁哄潧\";s:3:\"url\";s:9:\"forum.php\";s:4:\"name\";s:6:\"璁哄潧\";s:4:\"type\";s:1:\"1\";s:5:\"level\";s:1:\"0\";s:2:\"id\";s:0:\"\";s:4:\"icon\";s:86:\"<img src=\"https://localhost:8080//static/image/mobile/touch/forum.svg\" alt=\"璁哄潧\" />\";s:7:\"is_post\";b:0;}i:150;a:10:{s:4:\"data\";a:19:{s:2:\"id\";s:2:\"50\";s:8:\"parentid\";s:1:\"0\";s:4:\"name\";s:6:\"鍙戝竷\";s:5:\"title\";s:0:\"\";s:3:\"url\";s:29:\"forum.php?mod=misc&action=nav\";s:10:\"identifier\";s:4:\"post\";s:6:\"target\";s:1:\"0\";s:4:\"type\";s:1:\"1\";s:9:\"available\";s:1:\"1\";s:12:\"displayorder\";s:1:\"3\";s:9:\"highlight\";s:1:\"0\";s:5:\"level\";s:1:\"0\";s:7:\"subtype\";s:1:\"0\";s:7:\"subcols\";s:1:\"0\";s:4:\"icon\";s:39:\"/static/image/mobile/touch/plus_btn.svg\";s:7:\"subname\";s:0:\"\";s:6:\"suburl\";s:0:\"\";s:7:\"navtype\";s:1:\"6\";s:4:\"logo\";s:0:\"\";}s:9:\"available\";s:1:\"1\";s:7:\"navname\";s:6:\"鍙戝竷\";s:3:\"url\";s:29:\"forum.php?mod=misc&action=nav\";s:4:\"name\";s:6:\"鍙戝竷\";s:4:\"type\";s:1:\"1\";s:5:\"level\";s:1:\"0\";s:2:\"id\";s:4:\"post\";s:4:\"icon\";s:89:\"<img src=\"https://localhost:8080//static/image/mobile/touch/plus_btn.svg\" alt=\"鍙戝竷\" />\";s:7:\"is_post\";b:1;}i:151;a:10:{s:4:\"data\";a:19:{s:2:\"id\";s:2:\"51\";s:8:\"parentid\";s:1:\"0\";s:4:\"name\";s:6:\"鍙戠幇\";s:5:\"title\";s:0:\"\";s:3:\"url\";s:18:\"forum.php?mod=find\";s:10:\"identifier\";s:0:\"\";s:6:\"target\";s:1:\"0\";s:4:\"type\";s:1:\"1\";s:9:\"available\";s:1:\"1\";s:12:\"displayorder\";s:1:\"4\";s:9:\"highlight\";s:1:\"0\";s:5:\"level\";s:1:\"0\";s:7:\"subtype\";s:1:\"0\";s:7:\"subcols\";s:1:\"0\";s:4:\"icon\";s:38:\"/static/image/mobile/touch/explore.svg\";s:7:\"subname\";s:0:\"\";s:6:\"suburl\";s:0:\"\";s:7:\"navtype\";s:1:\"6\";s:4:\"logo\";s:0:\"\";}s:9:\"available\";s:1:\"1\";s:7:\"navname\";s:6:\"鍙戠幇\";s:3:\"url\";s:18:\"forum.php?mod=find\";s:4:\"name\";s:6:\"鍙戠幇\";s:4:\"type\";s:1:\"1\";s:5:\"level\";s:1:\"0\";s:2:\"id\";s:0:\"\";s:4:\"icon\";s:88:\"<img src=\"https://localhost:8080//static/image/mobile/touch/explore.svg\" alt=\"鍙戠幇\" />\";s:7:\"is_post\";b:0;}i:152;a:10:{s:4:\"data\";a:19:{s:2:\"id\";s:2:\"52\";s:8:\"parentid\";s:1:\"0\";s:4:\"name\";s:6:\"鎴戠殑\";s:5:\"title\";s:0:\"\";s:3:\"url\";s:18:\"home.php?mod=space\";s:10:\"identifier\";s:0:\"\";s:6:\"target\";s:1:\"0\";s:4:\"type\";s:1:\"1\";s:9:\"available\";s:1:\"1\";s:12:\"displayorder\";s:1:\"5\";s:9:\"highlight\";s:1:\"0\";s:5:\"level\";s:1:\"0\";s:7:\"subtype\";s:1:\"0\";s:7:\"subcols\";s:1:\"0\";s:4:\"icon\";s:36:\"/static/image/mobile/touch/space.svg\";s:7:\"subname\";s:0:\"\";s:6:\"suburl\";s:0:\"\";s:7:\"navtype\";s:1:\"6\";s:4:\"logo\";s:0:\"\";}s:9:\"available\";s:1:\"1\";s:7:\"navname\";s:6:\"鎴戠殑\";s:3:\"url\";s:18:\"home.php?mod=space\";s:4:\"name\";s:6:\"鎴戠殑\";s:4:\"type\";s:1:\"1\";s:5:\"level\";s:1:\"0\";s:2:\"id\";s:0:\"\";s:4:\"icon\";s:86:\"<img src=\"https://localhost:8080//static/image/mobile/touch/space.svg\" alt=\"鎴戠殑\" />\";s:7:\"is_post\";b:0;}}s:13:\"allowsynlogin\";i:0;s:9:\"ucappopen\";a:0:{}s:5:\"ucapp\";a:0:{}s:9:\"uchomeurl\";s:0:\"\";s:9:\"discuzurl\";s:22:\"https://localhost:8080\";s:8:\"homeshow\";s:1:\"0\";s:7:\"csspath\";s:17:\"data/cache/style_\";s:16:\"witframe_plugins\";a:0:{}s:6:\"output\";a:2:{s:3:\"str\";a:0:{}s:4:\"preg\";a:0:{}}s:8:\"parseflv\";a:7:{s:7:\"youtube\";a:1:{i:0;s:18:\"youtube.com/watch?\";}s:4:\"wasu\";a:1:{i:0;s:7:\"wasu.cn\";}s:5:\"acfun\";a:2:{i:0;s:8:\"acfun.cn\";i:1;s:8:\"acfun.tv\";}s:6:\"ixigua\";a:1:{i:0;s:11:\"ixigua.com/\";}s:5:\"youku\";a:1:{i:0;s:19:\"v.youku.com/v_show/\";}s:2:\"qq\";a:2:{i:0;s:16:\"v.qq.com/x/page/\";i:1;s:17:\"v.qq.com/x/cover/\";}s:8:\"bilibili\";a:4:{i:0;s:19:\"bilibili.com/video/\";i:1;s:18:\"bilibili.tv/video/\";i:2;s:6:\"acg.tv\";i:3;s:6:\"b23.tv\";}}s:5:\"mpsid\";s:0:\"\";s:13:\"securesiteurl\";s:23:\"https://localhost:8080/\";s:5:\"cells\";a:4:{s:3:\"tpl\";a:1:{i:2;a:2:{s:20:\"forum/portal/navlist\";s:338:\"<div class=\"portal_nav\">\r\n<div class=\"portal_nav_container\">\r\n	<ul class=\" cl\">\r\n	{cell forum/portal/navlist/loop_start}\r\n	<li{cell forum/portal/navlist/current_class}><a href=\"{cell forum/portal/navlist/url}\" ajaxtarget=\"threadlist\">{cell forum/portal/navlist/name}</a></li>\r\n	{cell forum/portal/navlist/loop_end}\r\n	</ul>\r\n</div>\r\n</div>\";s:23:\"forum/portal/threadlist\";s:900:\"<div class=\"timeline_entry_list\">\r\n<div class=\"timeline_entry_list_container\">\r\n<div class=\"tb-c threadlist\">\r\n	<!--Ajax:InnerStart-->\r\n	<div class=\"forumportal_listc cl\">\r\n		<ul>\r\n		    {cell forum/threadlist/loop_start}		    \r\n				<li class=\"kmlist\">\r\n					\r\n					{cell forum/threadlist/subject}\r\n					<div class=\"kmtxt\">{cell forum/threadlist/message}</div>\r\n					{cell forum/threadlist/image}\r\n					<div class=\"kmfoot\">\r\n						{cell forum/threadlist/author_avatar}{cell forum/threadlist/replies}{cell forum/threadlist/views}\r\n						{cell forum/threadlist/author}{cell forum/threadlist/dateline}{cell forum/threadlist/forum_name}\r\n					</div>\r\n				</li>\r\n		    {cell forum/threadlist/loop_end}\r\n		</ul>\r\n	</div>\r\n	<div id=\"threadlistAppend\" class=\"forumportal_pages cl\">\r\n		<!--Ajax:Clear-->{cell forum/threadlist/nextpage}<!--Ajax:/Clear-->\r\n	</div>\r\n	<!--Ajax:InnerEnd-->\r\n</div>\r\n</div>\r\n</div>\";}}s:4:\"used\";a:1:{i:2;a:2:{s:20:\"forum/portal/navlist\";a:0:{}s:23:\"forum/portal/threadlist\";a:4:{s:7:\"message\";i:1;s:5:\"image\";i:1;s:4:\"page\";i:0;s:8:\"nextpage\";i:1;}}}s:4:\"tplM\";a:1:{i:2;a:2:{s:20:\"forum/portal/navlist\";s:336:\"<div class=\"dhnav_box\"><div id=\"dhnav\"><div id=\"dhnav_li\"><ul class=\"flex-box\">\r\n{cell forum/portal/navlist/loop_start}\r\n<li{cell forum/portal/navlist/current_class}><a href=\"{cell forum/portal/navlist/url}\" ajaxtarget=\"threadlist\">{cell forum/portal/navlist/name}</a></li>\r\n{cell forum/portal/navlist/loop_end}\r\n</ul></div></div></div>\";s:23:\"forum/portal/threadlist\";s:894:\"<div class=\"threadlist_box mt10 cl\">\r\n	<div class=\"threadlist cl\">\r\n		<ul>\r\n		{cell forum/threadlist/loop_start}\r\n			<li class=\"list\">\r\n				<div class=\"threadlist_top cl\">\r\n					{cell forum/threadlist/author_avatar}\r\n					<div class=\"muser\">\r\n						<h3>{cell forum/threadlist/author}</h3>\r\n						<span class=\"mtime\">{cell forum/threadlist/dateline}</span>\r\n					</div>\r\n				</div>\r\n				{cell forum/threadlist/subject}\r\n				<a href=\"{cell forum/threadlist/url}\"><div class=\"threadlist_mes cl\">{cell forum/threadlist/message}</div></a>\r\n				{cell forum/threadlist/image}\r\n				<div class=\"threadlist_foot cl\">\r\n					<ul>\r\n						<li><i class=\"dm-eye-fill\"></i>{cell forum/threadlist/views}</li>\r\n						<li><i class=\"dm-chat-s-fill\"></i>{cell forum/threadlist/replies}</li>\r\n					</ul>\r\n				</div>\r\n			</li>\r\n		{cell forum/threadlist/loop_end}\r\n		</ul>\r\n	</div>\r\n</div>\r\n{cell forum/threadlist/page}\";}}s:5:\"usedM\";a:1:{i:2;a:2:{s:20:\"forum/portal/navlist\";a:0:{}s:23:\"forum/portal/threadlist\";a:4:{s:7:\"message\";i:1;s:5:\"image\";i:1;s:4:\"page\";i:1;s:8:\"nextpage\";i:0;}}}}s:4:\"i18n\";a:2:{s:7:\"SC_UTF8\";s:33:\"/app/public/./source/i18n/SC_UTF8\";s:7:\"TC_UTF8\";s:33:\"/app/public/./source/i18n/TC_UTF8\";}s:8:\"i18nLang\";a:0:{}}'),
('smileycodes',1,1784453715,'a:156:{i:1;s:2:\":)\";i:2;s:2:\":(\";i:3;s:2:\":D\";i:4;s:3:\":\'(\";i:5;s:2:\":@\";i:6;s:2:\":o\";i:7;s:2:\":P\";i:8;s:2:\":$\";i:9;s:2:\";P\";i:10;s:2:\":L\";i:11;s:2:\":Q\";i:12;s:4:\":lol\";i:13;s:12:\":loveliness:\";i:14;s:6:\":funk:\";i:15;s:7:\":curse:\";i:16;s:7:\":dizzy:\";i:17;s:8:\":shutup:\";i:18;s:8:\":sleepy:\";i:19;s:5:\":hug:\";i:20;s:9:\":victory:\";i:21;s:6:\":time:\";i:22;s:6:\":kiss:\";i:23;s:10:\":handshake\";i:24;s:6:\":call:\";i:25;s:8:\"{:2_25:}\";i:26;s:8:\"{:2_26:}\";i:27;s:8:\"{:2_27:}\";i:28;s:8:\"{:2_28:}\";i:29;s:8:\"{:2_29:}\";i:30;s:8:\"{:2_30:}\";i:31;s:8:\"{:2_31:}\";i:32;s:8:\"{:2_32:}\";i:33;s:8:\"{:2_33:}\";i:34;s:8:\"{:2_34:}\";i:35;s:8:\"{:2_35:}\";i:36;s:8:\"{:2_36:}\";i:37;s:8:\"{:2_37:}\";i:38;s:8:\"{:2_38:}\";i:39;s:8:\"{:2_39:}\";i:40;s:8:\"{:2_40:}\";i:41;s:8:\"{:3_41:}\";i:42;s:8:\"{:3_42:}\";i:43;s:8:\"{:3_43:}\";i:44;s:8:\"{:3_44:}\";i:45;s:8:\"{:3_45:}\";i:46;s:8:\"{:3_46:}\";i:47;s:8:\"{:3_47:}\";i:48;s:8:\"{:3_48:}\";i:49;s:8:\"{:3_49:}\";i:50;s:8:\"{:3_50:}\";i:51;s:8:\"{:3_51:}\";i:52;s:8:\"{:3_52:}\";i:53;s:8:\"{:3_53:}\";i:54;s:8:\"{:3_54:}\";i:55;s:8:\"{:3_55:}\";i:56;s:8:\"{:3_56:}\";i:57;s:8:\"{:3_57:}\";i:58;s:8:\"{:3_58:}\";i:59;s:8:\"{:3_59:}\";i:60;s:8:\"{:3_60:}\";i:61;s:8:\"{:3_61:}\";i:62;s:8:\"{:3_62:}\";i:63;s:8:\"{:3_63:}\";i:64;s:8:\"{:3_64:}\";i:86;s:8:\":kelian:\";i:87;s:8:\":haqian:\";i:88;s:8:\":woshou:\";i:89;s:7:\":aixin:\";i:90;s:13:\":zuohengheng:\";i:91;s:9:\":weixiao:\";i:92;s:10:\":jingkong:\";i:93;s:8:\":tiaopi:\";i:94;s:9:\":touxiao:\";i:95;s:9:\":youling:\";i:96;s:8:\":caidao:\";i:97;s:7:\":cahan:\";i:98;s:7:\":hecai:\";i:99;s:6:\":keai:\";i:100;s:6:\":ciya:\";i:101;s:8:\":saorao:\";i:102;s:8:\":jingxi:\";i:103;s:4:\":ku:\";i:104;s:8:\":piezui:\";i:105;s:4:\":se:\";i:106;s:5:\":xia:\";i:107;s:9:\":yinxian:\";i:108;s:8:\":zhouma:\";i:110;s:7:\":kulou:\";i:111;s:4:\":xu:\";i:112;s:8:\":jingya:\";i:113;s:6:\":doge:\";i:114;s:7:\":bizui:\";i:115;s:9:\":yangtuo:\";i:116;s:11:\":shouqiang:\";i:117;s:9:\":baoquan:\";i:118;s:5:\":yun:\";i:119;s:8:\":lanqiu:\";i:120;s:7:\":zhemo:\";i:121;s:9:\":guzhang:\";i:122;s:9:\":shengli:\";i:123;s:9:\":zaijian:\";i:124;s:8:\":dabing:\";i:125;s:6:\":deyi:\";i:126;s:9:\":hanxiao:\";i:127;s:5:\":kun:\";i:128;s:7:\":hexie:\";i:129;s:6:\":daku:\";i:130;s:10:\":wozuimei:\";i:131;s:8:\":xiaoku:\";i:132;s:7:\":xigua:\";i:133;s:10:\":huaixiao:\";i:134;s:8:\":liulei:\";i:135;s:9:\":lenghan:\";i:136;s:9:\":qiudale:\";i:137;s:12:\":zhayanjian:\";i:138;s:8:\":qiaoda:\";i:139;s:8:\":baojin:\";i:140;s:4:\":OK:\";i:141;s:12:\":xiaojiujie:\";i:142;s:8:\":gouyin:\";i:143;s:13:\":youhengheng:\";i:144;s:8:\":tuosai:\";i:145;s:8:\":nanguo:\";i:146;s:9:\":quantou:\";i:147;s:8:\":haixiu:\";i:148;s:7:\":koubi:\";i:149;s:7:\":qiang:\";i:150;s:7:\":pijiu:\";i:151;s:7:\":bishi:\";i:152;s:7:\":yiwen:\";i:153;s:8:\":liuhan:\";i:154;s:7:\":wunai:\";i:155;s:6:\":aini:\";i:156;s:14:\":bangbangtang:\";i:157;s:8:\":penxue:\";i:158;s:9:\":haobang:\";i:159;s:8:\":qinqin:\";i:160;s:12:\":xiaoyanger:\";i:161;s:8:\":fendou:\";i:162;s:7:\":ganga:\";i:163;s:7:\":shuai:\";i:164;s:7:\":juhua:\";i:165;s:8:\":baiyan:\";i:166;s:6:\":fanu:\";i:167;s:5:\":jie:\";i:168;s:5:\":chi:\";i:169;s:10:\":kuaikule:\";i:170;s:11:\":zhuakuang:\";i:171;s:6:\":shui:\";i:172;s:5:\":dan:\";i:173;s:7:\":aoman:\";i:174;s:7:\":fadai:\";i:175;s:8:\":leiben:\";i:176;s:4:\":tu:\";i:177;s:7:\":weiqu:\";i:178;s:12:\":xieyanxiao:\";}'),
('smileytypes',1,1784453715,'a:4:{i:1;a:5:{s:9:\"available\";s:1:\"1\";s:4:\"name\";s:6:\"榛樿\";s:4:\"type\";s:6:\"smiley\";s:12:\"displayorder\";s:1:\"1\";s:9:\"directory\";s:7:\"default\";}i:2;a:5:{s:9:\"available\";s:1:\"1\";s:4:\"name\";s:6:\"閰风尨\";s:4:\"type\";s:6:\"smiley\";s:12:\"displayorder\";s:1:\"2\";s:9:\"directory\";s:10:\"coolmonkey\";}i:3;a:5:{s:9:\"available\";s:1:\"1\";s:4:\"name\";s:9:\"鍛嗗憜鐢穃";s:4:\"type\";s:6:\"smiley\";s:12:\"displayorder\";s:1:\"3\";s:9:\"directory\";s:8:\"grapeman\";}i:4;a:5:{s:9:\"available\";s:1:\"1\";s:4:\"name\";s:2:\"QQ\";s:4:\"type\";s:6:\"smiley\";s:12:\"displayorder\";s:1:\"4\";s:9:\"directory\";s:2:\"qq\";}}'),
('smilies',1,1784453715,'a:3:{s:11:\"searcharray\";a:156:{i:156;s:18:\"/\\:bangbangtang\\:/\";i:90;s:17:\"/\\:zuohengheng\\:/\";i:143;s:17:\"/\\:youhengheng\\:/\";i:13;s:16:\"/\\:loveliness\\:/\";i:137;s:16:\"/\\:zhayanjian\\:/\";i:141;s:16:\"/\\:xiaojiujie\\:/\";i:160;s:16:\"/\\:xiaoyanger\\:/\";i:178;s:16:\"/\\:xieyanxiao\\:/\";i:116;s:15:\"/\\:shouqiang\\:/\";i:170;s:15:\"/\\:zhuakuang\\:/\";i:23;s:13:\"/\\:handshake/\";i:92;s:14:\"/\\:jingkong\\:/\";i:130;s:14:\"/\\:wozuimei\\:/\";i:133;s:14:\"/\\:huaixiao\\:/\";i:169;s:14:\"/\\:kuaikule\\:/\";i:20;s:13:\"/\\:victory\\:/\";i:91;s:13:\"/\\:weixiao\\:/\";i:94;s:13:\"/\\:touxiao\\:/\";i:95;s:13:\"/\\:youling\\:/\";i:107;s:13:\"/\\:yinxian\\:/\";i:115;s:13:\"/\\:yangtuo\\:/\";i:117;s:13:\"/\\:baoquan\\:/\";i:121;s:13:\"/\\:guzhang\\:/\";i:122;s:13:\"/\\:shengli\\:/\";i:123;s:13:\"/\\:zaijian\\:/\";i:126;s:13:\"/\\:hanxiao\\:/\";i:135;s:13:\"/\\:lenghan\\:/\";i:136;s:13:\"/\\:qiudale\\:/\";i:146;s:13:\"/\\:quantou\\:/\";i:158;s:13:\"/\\:haobang\\:/\";i:17;s:12:\"/\\:shutup\\:/\";i:18;s:12:\"/\\:sleepy\\:/\";i:25;s:14:\"/\\{\\:2_25\\:\\}/\";i:26;s:14:\"/\\{\\:2_26\\:\\}/\";i:27;s:14:\"/\\{\\:2_27\\:\\}/\";i:28;s:14:\"/\\{\\:2_28\\:\\}/\";i:29;s:14:\"/\\{\\:2_29\\:\\}/\";i:30;s:14:\"/\\{\\:2_30\\:\\}/\";i:31;s:14:\"/\\{\\:2_31\\:\\}/\";i:32;s:14:\"/\\{\\:2_32\\:\\}/\";i:33;s:14:\"/\\{\\:2_33\\:\\}/\";i:34;s:14:\"/\\{\\:2_34\\:\\}/\";i:35;s:14:\"/\\{\\:2_35\\:\\}/\";i:36;s:14:\"/\\{\\:2_36\\:\\}/\";i:37;s:14:\"/\\{\\:2_37\\:\\}/\";i:38;s:14:\"/\\{\\:2_38\\:\\}/\";i:39;s:14:\"/\\{\\:2_39\\:\\}/\";i:40;s:14:\"/\\{\\:2_40\\:\\}/\";i:41;s:14:\"/\\{\\:3_41\\:\\}/\";i:42;s:14:\"/\\{\\:3_42\\:\\}/\";i:43;s:14:\"/\\{\\:3_43\\:\\}/\";i:44;s:14:\"/\\{\\:3_44\\:\\}/\";i:45;s:14:\"/\\{\\:3_45\\:\\}/\";i:46;s:14:\"/\\{\\:3_46\\:\\}/\";i:47;s:14:\"/\\{\\:3_47\\:\\}/\";i:48;s:14:\"/\\{\\:3_48\\:\\}/\";i:49;s:14:\"/\\{\\:3_49\\:\\}/\";i:50;s:14:\"/\\{\\:3_50\\:\\}/\";i:51;s:14:\"/\\{\\:3_51\\:\\}/\";i:52;s:14:\"/\\{\\:3_52\\:\\}/\";i:53;s:14:\"/\\{\\:3_53\\:\\}/\";i:54;s:14:\"/\\{\\:3_54\\:\\}/\";i:55;s:14:\"/\\{\\:3_55\\:\\}/\";i:56;s:14:\"/\\{\\:3_56\\:\\}/\";i:57;s:14:\"/\\{\\:3_57\\:\\}/\";i:58;s:14:\"/\\{\\:3_58\\:\\}/\";i:59;s:14:\"/\\{\\:3_59\\:\\}/\";i:60;s:14:\"/\\{\\:3_60\\:\\}/\";i:61;s:14:\"/\\{\\:3_61\\:\\}/\";i:62;s:14:\"/\\{\\:3_62\\:\\}/\";i:63;s:14:\"/\\{\\:3_63\\:\\}/\";i:64;s:14:\"/\\{\\:3_64\\:\\}/\";i:86;s:12:\"/\\:kelian\\:/\";i:87;s:12:\"/\\:haqian\\:/\";i:88;s:12:\"/\\:woshou\\:/\";i:93;s:12:\"/\\:tiaopi\\:/\";i:96;s:12:\"/\\:caidao\\:/\";i:101;s:12:\"/\\:saorao\\:/\";i:102;s:12:\"/\\:jingxi\\:/\";i:104;s:12:\"/\\:piezui\\:/\";i:108;s:12:\"/\\:zhouma\\:/\";i:112;s:12:\"/\\:jingya\\:/\";i:119;s:12:\"/\\:lanqiu\\:/\";i:124;s:12:\"/\\:dabing\\:/\";i:131;s:12:\"/\\:xiaoku\\:/\";i:134;s:12:\"/\\:liulei\\:/\";i:138;s:12:\"/\\:qiaoda\\:/\";i:139;s:12:\"/\\:baojin\\:/\";i:142;s:12:\"/\\:gouyin\\:/\";i:144;s:12:\"/\\:tuosai\\:/\";i:145;s:12:\"/\\:nanguo\\:/\";i:147;s:12:\"/\\:haixiu\\:/\";i:153;s:12:\"/\\:liuhan\\:/\";i:157;s:12:\"/\\:penxue\\:/\";i:159;s:12:\"/\\:qinqin\\:/\";i:161;s:12:\"/\\:fendou\\:/\";i:165;s:12:\"/\\:baiyan\\:/\";i:175;s:12:\"/\\:leiben\\:/\";i:15;s:11:\"/\\:curse\\:/\";i:16;s:11:\"/\\:dizzy\\:/\";i:89;s:11:\"/\\:aixin\\:/\";i:97;s:11:\"/\\:cahan\\:/\";i:98;s:11:\"/\\:hecai\\:/\";i:110;s:11:\"/\\:kulou\\:/\";i:114;s:11:\"/\\:bizui\\:/\";i:120;s:11:\"/\\:zhemo\\:/\";i:128;s:11:\"/\\:hexie\\:/\";i:132;s:11:\"/\\:xigua\\:/\";i:148;s:11:\"/\\:koubi\\:/\";i:149;s:11:\"/\\:qiang\\:/\";i:150;s:11:\"/\\:pijiu\\:/\";i:151;s:11:\"/\\:bishi\\:/\";i:152;s:11:\"/\\:yiwen\\:/\";i:154;s:11:\"/\\:wunai\\:/\";i:162;s:11:\"/\\:ganga\\:/\";i:163;s:11:\"/\\:shuai\\:/\";i:164;s:11:\"/\\:juhua\\:/\";i:173;s:11:\"/\\:aoman\\:/\";i:174;s:11:\"/\\:fadai\\:/\";i:177;s:11:\"/\\:weiqu\\:/\";i:14;s:10:\"/\\:funk\\:/\";i:21;s:10:\"/\\:time\\:/\";i:22;s:10:\"/\\:kiss\\:/\";i:24;s:10:\"/\\:call\\:/\";i:99;s:10:\"/\\:keai\\:/\";i:100;s:10:\"/\\:ciya\\:/\";i:113;s:10:\"/\\:doge\\:/\";i:125;s:10:\"/\\:deyi\\:/\";i:129;s:10:\"/\\:daku\\:/\";i:155;s:10:\"/\\:aini\\:/\";i:166;s:10:\"/\\:fanu\\:/\";i:171;s:10:\"/\\:shui\\:/\";i:19;s:9:\"/\\:hug\\:/\";i:106;s:9:\"/\\:xia\\:/\";i:118;s:9:\"/\\:yun\\:/\";i:127;s:9:\"/\\:kun\\:/\";i:167;s:9:\"/\\:jie\\:/\";i:168;s:9:\"/\\:chi\\:/\";i:172;s:9:\"/\\:dan\\:/\";i:12;s:7:\"/\\:lol/\";i:103;s:8:\"/\\:ku\\:/\";i:105;s:8:\"/\\:se\\:/\";i:111;s:8:\"/\\:xu\\:/\";i:140;s:8:\"/\\:OK\\:/\";i:176;s:8:\"/\\:tu\\:/\";i:4;s:7:\"/\\:\'\\(/\";i:1;s:6:\"/\\:\\)/\";i:2;s:6:\"/\\:\\(/\";i:3;s:5:\"/\\:D/\";i:5;s:5:\"/\\:@/\";i:6;s:5:\"/\\:o/\";i:7;s:5:\"/\\:P/\";i:8;s:6:\"/\\:\\$/\";i:9;s:4:\"/;P/\";i:10;s:5:\"/\\:L/\";i:11;s:5:\"/\\:Q/\";}s:12:\"replacearray\";a:156:{i:156;s:16:\"bangbangtang.gif\";i:90;s:15:\"zuohengheng.gif\";i:143;s:15:\"youhengheng.gif\";i:13;s:14:\"loveliness.gif\";i:137;s:14:\"zhayanjian.gif\";i:141;s:14:\"xiaojiujie.gif\";i:160;s:14:\"xiaoyanger.gif\";i:178;s:14:\"xieyanxiao.gif\";i:116;s:13:\"shouqiang.gif\";i:170;s:13:\"zhuakuang.gif\";i:23;s:13:\"handshake.gif\";i:92;s:12:\"jingkong.gif\";i:130;s:12:\"wozuimei.gif\";i:133;s:12:\"huaixiao.gif\";i:169;s:12:\"kuaikule.gif\";i:20;s:11:\"victory.gif\";i:91;s:11:\"weixiao.gif\";i:94;s:11:\"touxiao.gif\";i:95;s:11:\"youling.gif\";i:107;s:11:\"yinxian.gif\";i:115;s:11:\"yangtuo.gif\";i:117;s:11:\"baoquan.gif\";i:121;s:11:\"guzhang.gif\";i:122;s:11:\"shengli.gif\";i:123;s:11:\"zaijian.gif\";i:126;s:11:\"hanxiao.gif\";i:135;s:11:\"lenghan.gif\";i:136;s:11:\"qiudale.gif\";i:146;s:11:\"quantou.gif\";i:158;s:11:\"haobang.gif\";i:17;s:10:\"shutup.gif\";i:18;s:10:\"sleepy.gif\";i:25;s:6:\"01.gif\";i:26;s:6:\"02.gif\";i:27;s:6:\"03.gif\";i:28;s:6:\"04.gif\";i:29;s:6:\"05.gif\";i:30;s:6:\"06.gif\";i:31;s:6:\"07.gif\";i:32;s:6:\"08.gif\";i:33;s:6:\"09.gif\";i:34;s:6:\"10.gif\";i:35;s:6:\"11.gif\";i:36;s:6:\"12.gif\";i:37;s:6:\"13.gif\";i:38;s:6:\"14.gif\";i:39;s:6:\"15.gif\";i:40;s:6:\"16.gif\";i:41;s:6:\"01.gif\";i:42;s:6:\"02.gif\";i:43;s:6:\"03.gif\";i:44;s:6:\"04.gif\";i:45;s:6:\"05.gif\";i:46;s:6:\"06.gif\";i:47;s:6:\"07.gif\";i:48;s:6:\"08.gif\";i:49;s:6:\"09.gif\";i:50;s:6:\"10.gif\";i:51;s:6:\"11.gif\";i:52;s:6:\"12.gif\";i:53;s:6:\"13.gif\";i:54;s:6:\"14.gif\";i:55;s:6:\"15.gif\";i:56;s:6:\"16.gif\";i:57;s:6:\"17.gif\";i:58;s:6:\"18.gif\";i:59;s:6:\"19.gif\";i:60;s:6:\"20.gif\";i:61;s:6:\"21.gif\";i:62;s:6:\"22.gif\";i:63;s:6:\"23.gif\";i:64;s:6:\"24.gif\";i:86;s:10:\"kelian.gif\";i:87;s:10:\"haqian.gif\";i:88;s:10:\"woshou.gif\";i:93;s:10:\"tiaopi.gif\";i:96;s:10:\"caidao.gif\";i:101;s:10:\"saorao.gif\";i:102;s:10:\"jingxi.gif\";i:104;s:10:\"piezui.gif\";i:108;s:10:\"zhouma.gif\";i:112;s:10:\"jingya.gif\";i:119;s:10:\"lanqiu.gif\";i:124;s:10:\"dabing.gif\";i:131;s:10:\"xiaoku.gif\";i:134;s:10:\"liulei.gif\";i:138;s:10:\"qiaoda.gif\";i:139;s:10:\"baojin.gif\";i:142;s:10:\"gouyin.gif\";i:144;s:10:\"tuosai.gif\";i:145;s:10:\"nanguo.gif\";i:147;s:10:\"haixiu.gif\";i:153;s:10:\"liuhan.gif\";i:157;s:10:\"penxue.gif\";i:159;s:10:\"qinqin.gif\";i:161;s:10:\"fendou.gif\";i:165;s:10:\"baiyan.gif\";i:175;s:10:\"leiben.gif\";i:15;s:9:\"curse.gif\";i:16;s:9:\"dizzy.gif\";i:89;s:9:\"aixin.gif\";i:97;s:9:\"cahan.gif\";i:98;s:9:\"hecai.gif\";i:110;s:9:\"kulou.gif\";i:114;s:9:\"bizui.gif\";i:120;s:9:\"zhemo.gif\";i:128;s:9:\"hexie.gif\";i:132;s:9:\"xigua.gif\";i:148;s:9:\"koubi.gif\";i:149;s:9:\"qiang.gif\";i:150;s:9:\"pijiu.gif\";i:151;s:9:\"bishi.gif\";i:152;s:9:\"yiwen.gif\";i:154;s:9:\"wunai.gif\";i:162;s:9:\"ganga.gif\";i:163;s:9:\"shuai.gif\";i:164;s:9:\"juhua.gif\";i:173;s:9:\"aoman.gif\";i:174;s:9:\"fadai.gif\";i:177;s:9:\"weiqu.gif\";i:14;s:8:\"funk.gif\";i:21;s:8:\"time.gif\";i:22;s:8:\"kiss.gif\";i:24;s:8:\"call.gif\";i:99;s:8:\"keai.gif\";i:100;s:8:\"ciya.gif\";i:113;s:8:\"doge.gif\";i:125;s:8:\"deyi.gif\";i:129;s:8:\"daku.gif\";i:155;s:8:\"aini.gif\";i:166;s:8:\"fanu.gif\";i:171;s:8:\"shui.gif\";i:19;s:7:\"hug.gif\";i:106;s:7:\"xia.gif\";i:118;s:7:\"yun.gif\";i:127;s:7:\"kun.gif\";i:167;s:7:\"jie.gif\";i:168;s:7:\"chi.gif\";i:172;s:7:\"dan.gif\";i:12;s:7:\"lol.gif\";i:103;s:6:\"ku.gif\";i:105;s:6:\"se.gif\";i:111;s:6:\"xu.gif\";i:140;s:6:\"OK.gif\";i:176;s:6:\"tu.gif\";i:4;s:7:\"cry.gif\";i:1;s:9:\"smile.gif\";i:2;s:7:\"sad.gif\";i:3;s:11:\"biggrin.gif\";i:5;s:9:\"huffy.gif\";i:6;s:11:\"shocked.gif\";i:7;s:10:\"tongue.gif\";i:8;s:7:\"shy.gif\";i:9;s:10:\"titter.gif\";i:10;s:9:\"sweat.gif\";i:11;s:7:\"mad.gif\";}s:9:\"typearray\";a:156:{i:156;s:1:\"4\";i:90;s:1:\"4\";i:143;s:1:\"4\";i:13;s:1:\"1\";i:137;s:1:\"4\";i:141;s:1:\"4\";i:160;s:1:\"4\";i:178;s:1:\"4\";i:116;s:1:\"4\";i:170;s:1:\"4\";i:23;s:1:\"1\";i:92;s:1:\"4\";i:130;s:1:\"4\";i:133;s:1:\"4\";i:169;s:1:\"4\";i:20;s:1:\"1\";i:91;s:1:\"4\";i:94;s:1:\"4\";i:95;s:1:\"4\";i:107;s:1:\"4\";i:115;s:1:\"4\";i:117;s:1:\"4\";i:121;s:1:\"4\";i:122;s:1:\"4\";i:123;s:1:\"4\";i:126;s:1:\"4\";i:135;s:1:\"4\";i:136;s:1:\"4\";i:146;s:1:\"4\";i:158;s:1:\"4\";i:17;s:1:\"1\";i:18;s:1:\"1\";i:25;s:1:\"2\";i:26;s:1:\"2\";i:27;s:1:\"2\";i:28;s:1:\"2\";i:29;s:1:\"2\";i:30;s:1:\"2\";i:31;s:1:\"2\";i:32;s:1:\"2\";i:33;s:1:\"2\";i:34;s:1:\"2\";i:35;s:1:\"2\";i:36;s:1:\"2\";i:37;s:1:\"2\";i:38;s:1:\"2\";i:39;s:1:\"2\";i:40;s:1:\"2\";i:41;s:1:\"3\";i:42;s:1:\"3\";i:43;s:1:\"3\";i:44;s:1:\"3\";i:45;s:1:\"3\";i:46;s:1:\"3\";i:47;s:1:\"3\";i:48;s:1:\"3\";i:49;s:1:\"3\";i:50;s:1:\"3\";i:51;s:1:\"3\";i:52;s:1:\"3\";i:53;s:1:\"3\";i:54;s:1:\"3\";i:55;s:1:\"3\";i:56;s:1:\"3\";i:57;s:1:\"3\";i:58;s:1:\"3\";i:59;s:1:\"3\";i:60;s:1:\"3\";i:61;s:1:\"3\";i:62;s:1:\"3\";i:63;s:1:\"3\";i:64;s:1:\"3\";i:86;s:1:\"4\";i:87;s:1:\"4\";i:88;s:1:\"4\";i:93;s:1:\"4\";i:96;s:1:\"4\";i:101;s:1:\"4\";i:102;s:1:\"4\";i:104;s:1:\"4\";i:108;s:1:\"4\";i:112;s:1:\"4\";i:119;s:1:\"4\";i:124;s:1:\"4\";i:131;s:1:\"4\";i:134;s:1:\"4\";i:138;s:1:\"4\";i:139;s:1:\"4\";i:142;s:1:\"4\";i:144;s:1:\"4\";i:145;s:1:\"4\";i:147;s:1:\"4\";i:153;s:1:\"4\";i:157;s:1:\"4\";i:159;s:1:\"4\";i:161;s:1:\"4\";i:165;s:1:\"4\";i:175;s:1:\"4\";i:15;s:1:\"1\";i:16;s:1:\"1\";i:89;s:1:\"4\";i:97;s:1:\"4\";i:98;s:1:\"4\";i:110;s:1:\"4\";i:114;s:1:\"4\";i:120;s:1:\"4\";i:128;s:1:\"4\";i:132;s:1:\"4\";i:148;s:1:\"4\";i:149;s:1:\"4\";i:150;s:1:\"4\";i:151;s:1:\"4\";i:152;s:1:\"4\";i:154;s:1:\"4\";i:162;s:1:\"4\";i:163;s:1:\"4\";i:164;s:1:\"4\";i:173;s:1:\"4\";i:174;s:1:\"4\";i:177;s:1:\"4\";i:14;s:1:\"1\";i:21;s:1:\"1\";i:22;s:1:\"1\";i:24;s:1:\"1\";i:99;s:1:\"4\";i:100;s:1:\"4\";i:113;s:1:\"4\";i:125;s:1:\"4\";i:129;s:1:\"4\";i:155;s:1:\"4\";i:166;s:1:\"4\";i:171;s:1:\"4\";i:19;s:1:\"1\";i:106;s:1:\"4\";i:118;s:1:\"4\";i:127;s:1:\"4\";i:167;s:1:\"4\";i:168;s:1:\"4\";i:172;s:1:\"4\";i:12;s:1:\"1\";i:103;s:1:\"4\";i:105;s:1:\"4\";i:111;s:1:\"4\";i:140;s:1:\"4\";i:176;s:1:\"4\";i:4;s:1:\"1\";i:1;s:1:\"1\";i:2;s:1:\"1\";i:3;s:1:\"1\";i:5;s:1:\"1\";i:6;s:1:\"1\";i:7;s:1:\"1\";i:8;s:1:\"1\";i:9;s:1:\"1\";i:10;s:1:\"1\";i:11;s:1:\"1\";}}'),
('stamps',1,1784453715,'a:21:{i:0;a:4:{s:3:\"url\";s:7:\"001.gif\";s:4:\"text\";s:6:\"绮惧崕\";s:4:\"type\";s:5:\"stamp\";s:4:\"icon\";i:0;}i:1;a:4:{s:3:\"url\";s:7:\"002.gif\";s:4:\"text\";s:6:\"鐑笘\";s:4:\"type\";s:5:\"stamp\";s:4:\"icon\";i:0;}i:2;a:4:{s:3:\"url\";s:7:\"003.gif\";s:4:\"text\";s:6:\"缇庡浘\";s:4:\"type\";s:5:\"stamp\";s:4:\"icon\";i:0;}i:3;a:4:{s:3:\"url\";s:7:\"004.gif\";s:4:\"text\";s:6:\"浼樼\";s:4:\"type\";s:5:\"stamp\";s:4:\"icon\";i:0;}i:4;a:4:{s:3:\"url\";s:7:\"005.gif\";s:4:\"text\";s:6:\"缃《\";s:4:\"type\";s:5:\"stamp\";s:4:\"icon\";i:0;}i:5;a:4:{s:3:\"url\";s:7:\"006.gif\";s:4:\"text\";s:6:\"鎺ㄨ崘\";s:4:\"type\";s:5:\"stamp\";s:4:\"icon\";i:0;}i:6;a:4:{s:3:\"url\";s:7:\"007.gif\";s:4:\"text\";s:6:\"鍘熷垱\";s:4:\"type\";s:5:\"stamp\";s:4:\"icon\";i:0;}i:7;a:4:{s:3:\"url\";s:7:\"008.gif\";s:4:\"text\";s:12:\"鐗堜富鎺ㄨ崘\";s:4:\"type\";s:5:\"stamp\";s:4:\"icon\";i:0;}i:8;a:4:{s:3:\"url\";s:7:\"009.gif\";s:4:\"text\";s:6:\"鐖嗘枡\";s:4:\"type\";s:5:\"stamp\";s:4:\"icon\";i:0;}i:9;a:4:{s:3:\"url\";s:13:\"001.small.gif\";s:4:\"text\";s:6:\"绮惧崕\";s:4:\"type\";s:9:\"stamplist\";s:4:\"icon\";i:1;}i:10;a:4:{s:3:\"url\";s:13:\"002.small.gif\";s:4:\"text\";s:6:\"鐑笘\";s:4:\"type\";s:9:\"stamplist\";s:4:\"icon\";i:1;}i:11;a:4:{s:3:\"url\";s:13:\"003.small.gif\";s:4:\"text\";s:6:\"缇庡浘\";s:4:\"type\";s:9:\"stamplist\";s:4:\"icon\";i:1;}i:12;a:4:{s:3:\"url\";s:13:\"004.small.gif\";s:4:\"text\";s:6:\"浼樼\";s:4:\"type\";s:9:\"stamplist\";s:4:\"icon\";i:1;}i:13;a:4:{s:3:\"url\";s:13:\"005.small.gif\";s:4:\"text\";s:6:\"缃《\";s:4:\"type\";s:9:\"stamplist\";s:4:\"icon\";i:1;}i:14;a:4:{s:3:\"url\";s:13:\"006.small.gif\";s:4:\"text\";s:6:\"鎺ㄨ崘\";s:4:\"type\";s:9:\"stamplist\";s:4:\"icon\";i:1;}i:15;a:4:{s:3:\"url\";s:13:\"007.small.gif\";s:4:\"text\";s:6:\"鍘熷垱\";s:4:\"type\";s:9:\"stamplist\";s:4:\"icon\";i:1;}i:16;a:4:{s:3:\"url\";s:13:\"008.small.gif\";s:4:\"text\";s:12:\"鐗堜富鎺ㄨ崘\";s:4:\"type\";s:9:\"stamplist\";s:4:\"icon\";i:1;}i:17;a:4:{s:3:\"url\";s:13:\"009.small.gif\";s:4:\"text\";s:6:\"鐖嗘枡\";s:4:\"type\";s:9:\"stamplist\";s:4:\"icon\";i:1;}i:18;a:4:{s:3:\"url\";s:13:\"010.small.gif\";s:4:\"text\";s:12:\"缂栬緫閲囩敤\";s:4:\"type\";s:9:\"stamplist\";s:4:\"icon\";i:1;}i:19;a:4:{s:3:\"url\";s:7:\"010.gif\";s:4:\"text\";s:12:\"缂栬緫閲囩敤\";s:4:\"type\";s:5:\"stamp\";s:4:\"icon\";i:0;}i:20;a:4:{s:3:\"url\";s:13:\"011.small.gif\";s:4:\"text\";s:9:\"鏂颁汉甯朶";s:4:\"type\";s:9:\"stamplist\";s:4:\"icon\";i:1;}}'),
('stamptypeid',1,1784453715,'a:2:{i:0;s:1:\"8\";i:4;s:2:\"19\";}'),
('style_1',1,1784453715,'a:104:{s:7:\"styleid\";s:1:\"1\";s:4:\"name\";s:12:\"榛樿椋庢牸\";s:9:\"available\";s:0:\"\";s:10:\"templateid\";s:1:\"1\";s:8:\"extstyle\";a:5:{i:0;a:3:{i:0;s:27:\"./template/default/style/t1\";i:1;s:3:\"绾";i:2;s:7:\"#BA350F\";}i:1;a:3:{i:0;s:27:\"./template/default/style/t2\";i:1;s:3:\"闈抃";i:2;s:7:\"#429296\";}i:2;a:3:{i:0;s:27:\"./template/default/style/t3\";i:1;s:3:\"姗橽";i:2;s:7:\"#FE9500\";}i:3;a:3:{i:0;s:27:\"./template/default/style/t4\";i:1;s:3:\"绱玕";i:2;s:7:\"#9934B7\";}i:4;a:3:{i:0;s:27:\"./template/default/style/t5\";i:1;s:3:\"钃漒";i:2;s:7:\"#0053B9\";}}s:7:\"version\";s:5:\"1.0.0\";s:7:\"tplname\";s:18:\"榛樿妯℃澘濂楃郴\";s:9:\"directory\";s:18:\"./template/default\";s:9:\"copyright\";s:7:\"Discuz!\";s:6:\"tpldir\";s:18:\"./template/default\";s:16:\"menuhoverbgcolor\";s:7:\"#004FA0\";s:14:\"menucurbgcolor\";s:7:\"#005AB4\";s:9:\"lightlink\";s:4:\"#FFF\";s:12:\"floatbgcolor\";s:4:\"#FFF\";s:15:\"dropmenubgcolor\";s:7:\"#FEFEFE\";s:16:\"floatmaskbgcolor\";s:4:\"#000\";s:14:\"dropmenuborder\";s:4:\"#DDD\";s:9:\"specialbg\";s:7:\"#E5EDF2\";s:13:\"specialborder\";s:7:\"#C2D5E3\";s:8:\"commonbg\";s:7:\"#F2F2F2\";s:12:\"commonborder\";s:7:\"#CDCDCD\";s:7:\"inputbg\";s:4:\"#FFF\";s:7:\"stypeid\";s:1:\"1\";s:20:\"inputborderdarkcolor\";s:7:\"#848484\";s:13:\"headerbgcolor\";s:0:\"\";s:12:\"headerborder\";s:1:\"0\";s:11:\"sidebgcolor\";s:7:\"#E8F0F7\";s:11:\"msgfontsize\";s:4:\"14px\";s:7:\"bgcolor\";s:4:\"#FFF\";s:10:\"noticetext\";s:7:\"#F26C4F\";s:13:\"highlightlink\";s:4:\"#369\";s:4:\"link\";s:4:\"#333\";s:9:\"lighttext\";s:4:\"#999\";s:7:\"midtext\";s:4:\"#666\";s:9:\"tabletext\";s:4:\"#444\";s:10:\"smfontsize\";s:6:\"0.83em\";s:15:\"threadtitlefont\";s:45:\"Tahoma,Helvetica,\'Microsoft Yahei\',sans-serif\";s:19:\"threadtitlefontsize\";s:4:\"14px\";s:6:\"smfont\";s:27:\"Tahoma,Helvetica,sans-serif\";s:12:\"titlebgcolor\";s:7:\"#E5EDF2\";s:8:\"fontsize\";s:8:\"12px/1.5\";s:4:\"font\";s:45:\"Tahoma,Helvetica,\'Microsoft Yahei\',sans-serif\";s:11:\"styleimgdir\";s:19:\"static/image/common\";s:6:\"imgdir\";s:19:\"static/image/common\";s:8:\"boardimg\";s:28:\"static/image/common/logo.svg\";s:9:\"searchimg\";s:31:\"static/image/common/logo_sc.svg\";s:8:\"touchimg\";s:30:\"static/image/common/logo_m.svg\";s:10:\"headertext\";s:4:\"#444\";s:10:\"footertext\";s:4:\"#666\";s:11:\"menubgcolor\";s:7:\"#2B7ACD\";s:8:\"menutext\";s:4:\"#FFF\";s:13:\"menuhovertext\";s:4:\"#FFF\";s:6:\"wrapbg\";s:4:\"#FFF\";s:15:\"wrapbordercolor\";s:4:\"#CCC\";s:12:\"contentwidth\";s:5:\"630px\";s:15:\"contentseparate\";s:7:\"#C2D5E3\";s:11:\"inputborder\";s:7:\"#E0E0E0\";s:15:\"menuhoverbgcode\";s:19:\"background: #004FA0\";s:13:\"menucurbgcode\";s:19:\"background: #005AB4\";s:11:\"floatbgcode\";s:16:\"background: #FFF\";s:14:\"dropmenubgcode\";s:19:\"background: #FEFEFE\";s:15:\"floatmaskbgcode\";s:16:\"background: #000\";s:12:\"headerbgcode\";s:0:\"\";s:10:\"sidebgcode\";s:19:\"background: #E8F0F7\";s:6:\"bgcode\";s:16:\"background: #FFF\";s:11:\"titlebgcode\";s:19:\"background: #E5EDF2\";s:10:\"menubgcode\";s:19:\"background: #2B7ACD\";s:9:\"boardlogo\";s:100:\"<img src=\"static/image/common/logo.svg\" alt=\"Discuz!\" class=\"boardlogo\" id=\"boardlogo\" border=\"0\" />\";s:10:\"searchlogo\";s:105:\"<img src=\"static/image/common/logo_sc.svg\" alt=\"Discuz!\" class=\"searchlogo\" id=\"searchlogo\" border=\"0\" />\";s:9:\"touchlogo\";s:102:\"<img src=\"static/image/common/logo_m.svg\" alt=\"Discuz!\" class=\"touchlogo\" id=\"touchlogo\" border=\"0\" />\";s:4:\"bold\";s:4:\"bold\";s:15:\"defaultextstyle\";s:0:\"\";s:12:\"templatelang\";b:0;s:11:\"touch_style\";s:0:\"\";s:17:\"touch_style_color\";s:7:\"#2B7ACD\";s:15:\"touch_style_bg2\";s:7:\"#FF5656\";s:18:\"touch_style_border\";s:7:\"#EDEDED\";s:15:\"touch_style_bgs\";s:0:\"\";s:18:\"touch_style_bodybg\";s:7:\"#EEEEEE\";s:15:\"touch_style_bg0\";s:7:\"#FFFFFF\";s:15:\"touch_style_bg1\";s:7:\"#333333\";s:15:\"touch_style_bg3\";s:7:\"#FF9900\";s:15:\"touch_style_bg4\";s:7:\"#B3CC0D\";s:15:\"touch_style_bg5\";s:7:\"#F3F3F3\";s:15:\"touch_style_bg6\";s:7:\"#CCCCCC\";s:15:\"touch_style_bgn\";s:7:\"#A0C8EA\";s:16:\"touch_style_text\";s:0:\"\";s:16:\"touch_style_tfff\";s:7:\"#FFFFFF\";s:16:\"touch_style_t333\";s:7:\"#333333\";s:16:\"touch_style_t666\";s:7:\"#666666\";s:16:\"touch_style_t999\";s:7:\"#999999\";s:16:\"touch_style_t777\";s:7:\"#777777\";s:16:\"touch_style_t888\";s:7:\"#888888\";s:16:\"touch_style_taaa\";s:7:\"#AAAAAA\";s:16:\"touch_style_tbbb\";s:7:\"#BBBBBB\";s:16:\"touch_style_tccc\";s:7:\"#CCCCCC\";s:16:\"touch_style_tddd\";s:7:\"#DDDDDD\";s:16:\"touch_style_tnnn\";s:7:\"#7DA0CC\";s:18:\"touch_style_tlight\";s:7:\"#FF9C00\";s:14:\"touch_style_ta\";s:7:\"#FF5656\";s:14:\"touch_style_tv\";s:7:\"#7CBE00\";s:15:\"touch_style_add\";s:0:\"\";s:18:\"touch_style_addcss\";s:0:\"\";s:7:\"verhash\";s:3:\"K2J\";}'),
('style_2',1,1784453715,'a:99:{s:7:\"styleid\";s:1:\"2\";s:4:\"name\";s:8:\"X5妯＄増\";s:9:\"available\";s:0:\"\";s:10:\"templateid\";s:1:\"2\";s:8:\"extstyle\";s:0:\"\";s:7:\"version\";s:0:\"\";s:7:\"tplname\";s:8:\"X5妯＄増\";s:9:\"directory\";s:19:\"./template/discuzx5\";s:9:\"copyright\";s:7:\"Discuz!\";s:6:\"tpldir\";s:19:\"./template/discuzx5\";s:16:\"menuhoverbgcolor\";s:7:\"#0051CC\";s:14:\"menucurbgcolor\";s:7:\"#0051CC\";s:9:\"lightlink\";s:4:\"#ccc\";s:12:\"floatbgcolor\";s:4:\"#FFF\";s:15:\"dropmenubgcolor\";s:7:\"#FEFEFE\";s:16:\"floatmaskbgcolor\";s:4:\"#000\";s:14:\"dropmenuborder\";s:7:\"#eeeeee\";s:9:\"specialbg\";s:7:\"#d6e4ff\";s:13:\"specialborder\";s:7:\"#d6e4ff\";s:8:\"commonbg\";s:7:\"#F9f9f9\";s:12:\"commonborder\";s:7:\"#eeeeee\";s:7:\"inputbg\";s:4:\"#FFF\";s:7:\"stypeid\";s:1:\"1\";s:20:\"inputborderdarkcolor\";s:7:\"#e3e3e3\";s:13:\"headerbgcolor\";s:0:\"\";s:12:\"headerborder\";s:1:\"0\";s:11:\"sidebgcolor\";s:7:\"#E8F0F7\";s:11:\"msgfontsize\";s:4:\"14px\";s:7:\"bgcolor\";s:7:\"#F7F9FA\";s:10:\"noticetext\";s:7:\"#F26C4F\";s:13:\"highlightlink\";s:7:\"#0066ff\";s:4:\"link\";s:4:\"#333\";s:9:\"lighttext\";s:4:\"#999\";s:7:\"midtext\";s:4:\"#666\";s:9:\"tabletext\";s:4:\"#333\";s:10:\"smfontsize\";s:6:\"0.83em\";s:15:\"threadtitlefont\";s:45:\"Tahoma,Helvetica,\'Microsoft Yahei\',sans-serif\";s:19:\"threadtitlefontsize\";s:4:\"16px\";s:6:\"smfont\";s:27:\"Tahoma,Helvetica,sans-serif\";s:12:\"titlebgcolor\";s:7:\"#E5EDF2\";s:8:\"fontsize\";s:8:\"12px/1.5\";s:4:\"font\";s:45:\"Tahoma,Helvetica,\'Microsoft Yahei\',sans-serif\";s:11:\"styleimgdir\";s:25:\"template/discuzx5/static/\";s:6:\"imgdir\";s:19:\"static/image/common\";s:8:\"boardimg\";s:34:\"template/discuzx5/static//logo.png\";s:9:\"searchimg\";s:31:\"static/image/common/logo_sc.svg\";s:8:\"touchimg\";s:30:\"static/image/common/logo_m.svg\";s:10:\"headertext\";s:4:\"#444\";s:10:\"footertext\";s:4:\"#666\";s:11:\"menubgcolor\";s:7:\"#0066FF\";s:8:\"menutext\";s:7:\"#0066ff\";s:13:\"menuhovertext\";s:4:\"#FFF\";s:6:\"wrapbg\";s:4:\"#FFF\";s:15:\"wrapbordercolor\";s:4:\"#CCC\";s:12:\"contentwidth\";s:5:\"630px\";s:15:\"contentseparate\";s:7:\"#d6e4ff\";s:11:\"inputborder\";s:7:\"#e3e3e3\";s:5:\"btnbg\";s:7:\"#d6e4ff\";s:6:\"btntxt\";s:7:\"#0066ff\";s:6:\"btnbga\";s:7:\"#0066ff\";s:7:\"btntxta\";s:7:\"#ffffff\";s:15:\"menuhoverbgcode\";s:19:\"background: #0051CC\";s:13:\"menucurbgcode\";s:19:\"background: #0051CC\";s:11:\"floatbgcode\";s:16:\"background: #FFF\";s:14:\"dropmenubgcode\";s:19:\"background: #FEFEFE\";s:15:\"floatmaskbgcode\";s:16:\"background: #000\";s:12:\"headerbgcode\";s:0:\"\";s:10:\"sidebgcode\";s:19:\"background: #E8F0F7\";s:6:\"bgcode\";s:19:\"background: #F7F9FA\";s:11:\"titlebgcode\";s:19:\"background: #E5EDF2\";s:10:\"menubgcode\";s:19:\"background: #0066FF\";s:9:\"boardlogo\";s:106:\"<img src=\"template/discuzx5/static//logo.png\" alt=\"Discuz!\" class=\"boardlogo\" id=\"boardlogo\" border=\"0\" />\";s:10:\"searchlogo\";s:105:\"<img src=\"static/image/common/logo_sc.svg\" alt=\"Discuz!\" class=\"searchlogo\" id=\"searchlogo\" border=\"0\" />\";s:9:\"touchlogo\";s:102:\"<img src=\"static/image/common/logo_m.svg\" alt=\"Discuz!\" class=\"touchlogo\" id=\"touchlogo\" border=\"0\" />\";s:4:\"bold\";s:4:\"bold\";s:12:\"templatelang\";b:1;s:8:\"template\";s:0:\"\";s:12:\"template_top\";s:0:\"\";s:9:\"is_search\";s:1:\"1\";s:9:\"is_fixtop\";s:1:\"1\";s:10:\"top_navnum\";s:1:\"8\";s:17:\"top_nav_widthauto\";s:1:\"0\";s:11:\"top_nav_bgc\";s:0:\"\";s:12:\"top_nav_dark\";s:1:\"0\";s:11:\"top_fastnav\";s:1:\"1\";s:10:\"sider_tool\";s:0:\"\";s:12:\"sider_wechat\";s:1:\"1\";s:19:\"sider_wechat_qrcode\";s:42:\"template/discuzx5/static/images/wechat.jpg\";s:16:\"sider_wechat_txt\";s:15:\"鍏虫敞鍏紬鍙穃";s:14:\"sider_fastpost\";s:1:\"1\";s:15:\"template_bottom\";s:0:\"\";s:10:\"bottom_bgc\";s:7:\"#333333\";s:11:\"bottom_dark\";s:1:\"1\";s:13:\"bottom_qrcode\";s:41:\"template/discuzx5/static/images/ewm_a.jpg\";s:16:\"bottom_qrcodetxt\";s:15:\"鍏虫敞鍏紬鍙穃";s:10:\"bottom_txt\";s:77:\"鐩稿叧渚垫潈銆佷妇鎶ャ€佹姇璇夊強寤鸿绛夛紝璇峰彂 E-mail锛歛dmin@discuz.vip\";s:5:\"admin\";s:0:\"\";s:11:\"admin_color\";s:7:\"#00b96b\";s:7:\"verhash\";s:3:\"wnz\";}'),
('style_default',1,1784453715,'a:99:{s:7:\"styleid\";s:1:\"2\";s:4:\"name\";s:8:\"X5妯＄増\";s:9:\"available\";s:0:\"\";s:10:\"templateid\";s:1:\"2\";s:8:\"extstyle\";s:0:\"\";s:7:\"version\";s:0:\"\";s:7:\"tplname\";s:8:\"X5妯＄増\";s:9:\"directory\";s:19:\"./template/discuzx5\";s:9:\"copyright\";s:7:\"Discuz!\";s:6:\"tpldir\";s:19:\"./template/discuzx5\";s:16:\"menuhoverbgcolor\";s:7:\"#0051CC\";s:14:\"menucurbgcolor\";s:7:\"#0051CC\";s:9:\"lightlink\";s:4:\"#ccc\";s:12:\"floatbgcolor\";s:4:\"#FFF\";s:15:\"dropmenubgcolor\";s:7:\"#FEFEFE\";s:16:\"floatmaskbgcolor\";s:4:\"#000\";s:14:\"dropmenuborder\";s:7:\"#eeeeee\";s:9:\"specialbg\";s:7:\"#d6e4ff\";s:13:\"specialborder\";s:7:\"#d6e4ff\";s:8:\"commonbg\";s:7:\"#F9f9f9\";s:12:\"commonborder\";s:7:\"#eeeeee\";s:7:\"inputbg\";s:4:\"#FFF\";s:7:\"stypeid\";s:1:\"1\";s:20:\"inputborderdarkcolor\";s:7:\"#e3e3e3\";s:13:\"headerbgcolor\";s:0:\"\";s:12:\"headerborder\";s:1:\"0\";s:11:\"sidebgcolor\";s:7:\"#E8F0F7\";s:11:\"msgfontsize\";s:4:\"14px\";s:7:\"bgcolor\";s:7:\"#F7F9FA\";s:10:\"noticetext\";s:7:\"#F26C4F\";s:13:\"highlightlink\";s:7:\"#0066ff\";s:4:\"link\";s:4:\"#333\";s:9:\"lighttext\";s:4:\"#999\";s:7:\"midtext\";s:4:\"#666\";s:9:\"tabletext\";s:4:\"#333\";s:10:\"smfontsize\";s:6:\"0.83em\";s:15:\"threadtitlefont\";s:45:\"Tahoma,Helvetica,\'Microsoft Yahei\',sans-serif\";s:19:\"threadtitlefontsize\";s:4:\"16px\";s:6:\"smfont\";s:27:\"Tahoma,Helvetica,sans-serif\";s:12:\"titlebgcolor\";s:7:\"#E5EDF2\";s:8:\"fontsize\";s:8:\"12px/1.5\";s:4:\"font\";s:45:\"Tahoma,Helvetica,\'Microsoft Yahei\',sans-serif\";s:11:\"styleimgdir\";s:25:\"template/discuzx5/static/\";s:6:\"imgdir\";s:19:\"static/image/common\";s:8:\"boardimg\";s:34:\"template/discuzx5/static//logo.png\";s:9:\"searchimg\";s:31:\"static/image/common/logo_sc.svg\";s:8:\"touchimg\";s:30:\"static/image/common/logo_m.svg\";s:10:\"headertext\";s:4:\"#444\";s:10:\"footertext\";s:4:\"#666\";s:11:\"menubgcolor\";s:7:\"#0066FF\";s:8:\"menutext\";s:7:\"#0066ff\";s:13:\"menuhovertext\";s:4:\"#FFF\";s:6:\"wrapbg\";s:4:\"#FFF\";s:15:\"wrapbordercolor\";s:4:\"#CCC\";s:12:\"contentwidth\";s:5:\"630px\";s:15:\"contentseparate\";s:7:\"#d6e4ff\";s:11:\"inputborder\";s:7:\"#e3e3e3\";s:5:\"btnbg\";s:7:\"#d6e4ff\";s:6:\"btntxt\";s:7:\"#0066ff\";s:6:\"btnbga\";s:7:\"#0066ff\";s:7:\"btntxta\";s:7:\"#ffffff\";s:15:\"menuhoverbgcode\";s:19:\"background: #0051CC\";s:13:\"menucurbgcode\";s:19:\"background: #0051CC\";s:11:\"floatbgcode\";s:16:\"background: #FFF\";s:14:\"dropmenubgcode\";s:19:\"background: #FEFEFE\";s:15:\"floatmaskbgcode\";s:16:\"background: #000\";s:12:\"headerbgcode\";s:0:\"\";s:10:\"sidebgcode\";s:19:\"background: #E8F0F7\";s:6:\"bgcode\";s:19:\"background: #F7F9FA\";s:11:\"titlebgcode\";s:19:\"background: #E5EDF2\";s:10:\"menubgcode\";s:19:\"background: #0066FF\";s:9:\"boardlogo\";s:106:\"<img src=\"template/discuzx5/static//logo.png\" alt=\"Discuz!\" class=\"boardlogo\" id=\"boardlogo\" border=\"0\" />\";s:10:\"searchlogo\";s:105:\"<img src=\"static/image/common/logo_sc.svg\" alt=\"Discuz!\" class=\"searchlogo\" id=\"searchlogo\" border=\"0\" />\";s:9:\"touchlogo\";s:102:\"<img src=\"static/image/common/logo_m.svg\" alt=\"Discuz!\" class=\"touchlogo\" id=\"touchlogo\" border=\"0\" />\";s:4:\"bold\";s:4:\"bold\";s:12:\"templatelang\";b:1;s:8:\"template\";s:0:\"\";s:12:\"template_top\";s:0:\"\";s:9:\"is_search\";s:1:\"1\";s:9:\"is_fixtop\";s:1:\"1\";s:10:\"top_navnum\";s:1:\"8\";s:17:\"top_nav_widthauto\";s:1:\"0\";s:11:\"top_nav_bgc\";s:0:\"\";s:12:\"top_nav_dark\";s:1:\"0\";s:11:\"top_fastnav\";s:1:\"1\";s:10:\"sider_tool\";s:0:\"\";s:12:\"sider_wechat\";s:1:\"1\";s:19:\"sider_wechat_qrcode\";s:42:\"template/discuzx5/static/images/wechat.jpg\";s:16:\"sider_wechat_txt\";s:15:\"鍏虫敞鍏紬鍙穃";s:14:\"sider_fastpost\";s:1:\"1\";s:15:\"template_bottom\";s:0:\"\";s:10:\"bottom_bgc\";s:7:\"#333333\";s:11:\"bottom_dark\";s:1:\"1\";s:13:\"bottom_qrcode\";s:41:\"template/discuzx5/static/images/ewm_a.jpg\";s:16:\"bottom_qrcodetxt\";s:15:\"鍏虫敞鍏紬鍙穃";s:10:\"bottom_txt\";s:77:\"鐩稿叧渚垫潈銆佷妇鎶ャ€佹姇璇夊強寤鸿绛夛紝璇峰彂 E-mail锛歛dmin@discuz.vip\";s:5:\"admin\";s:0:\"\";s:11:\"admin_color\";s:7:\"#00b96b\";s:7:\"verhash\";s:3:\"wnz\";}'),
('styleconsts',1,1784453715,'a:2:{i:1;a:45:{s:18:\"{MENUHOVERBGCOLOR}\";s:7:\"#004FA0\";s:16:\"{MENUCURBGCOLOR}\";s:7:\"#005AB4\";s:11:\"{LIGHTLINK}\";s:4:\"#FFF\";s:14:\"{FLOATBGCOLOR}\";s:4:\"#FFF\";s:17:\"{DROPMENUBGCOLOR}\";s:7:\"#FEFEFE\";s:18:\"{FLOATMASKBGCOLOR}\";s:4:\"#000\";s:16:\"{DROPMENUBORDER}\";s:4:\"#DDD\";s:11:\"{SPECIALBG}\";s:7:\"#E5EDF2\";s:15:\"{SPECIALBORDER}\";s:7:\"#C2D5E3\";s:10:\"{COMMONBG}\";s:7:\"#F2F2F2\";s:14:\"{COMMONBORDER}\";s:7:\"#CDCDCD\";s:9:\"{INPUTBG}\";s:4:\"#FFF\";s:22:\"{INPUTBORDERDARKCOLOR}\";s:7:\"#848484\";s:15:\"{HEADERBGCOLOR}\";s:0:\"\";s:14:\"{HEADERBORDER}\";s:1:\"0\";s:13:\"{SIDEBGCOLOR}\";s:7:\"#E8F0F7\";s:13:\"{MSGFONTSIZE}\";s:4:\"14px\";s:9:\"{BGCOLOR}\";s:4:\"#FFF\";s:12:\"{NOTICETEXT}\";s:7:\"#F26C4F\";s:15:\"{HIGHLIGHTLINK}\";s:4:\"#369\";s:6:\"{LINK}\";s:4:\"#333\";s:11:\"{LIGHTTEXT}\";s:4:\"#999\";s:9:\"{MIDTEXT}\";s:4:\"#666\";s:11:\"{TABLETEXT}\";s:4:\"#444\";s:12:\"{SMFONTSIZE}\";s:6:\"0.83em\";s:17:\"{THREADTITLEFONT}\";s:45:\"Tahoma,Helvetica,\'Microsoft Yahei\',sans-serif\";s:21:\"{THREADTITLEFONTSIZE}\";s:4:\"14px\";s:8:\"{SMFONT}\";s:27:\"Tahoma,Helvetica,sans-serif\";s:14:\"{TITLEBGCOLOR}\";s:7:\"#E5EDF2\";s:10:\"{FONTSIZE}\";s:8:\"12px/1.5\";s:6:\"{FONT}\";s:45:\"Tahoma,Helvetica,\'Microsoft Yahei\',sans-serif\";s:13:\"{STYLEIMGDIR}\";s:0:\"\";s:10:\"{BOARDIMG}\";s:8:\"logo.svg\";s:11:\"{SEARCHIMG}\";s:11:\"logo_sc.svg\";s:10:\"{TOUCHIMG}\";s:10:\"logo_m.svg\";s:12:\"{HEADERTEXT}\";s:4:\"#444\";s:12:\"{FOOTERTEXT}\";s:4:\"#666\";s:13:\"{MENUBGCOLOR}\";s:7:\"#2B7ACD\";s:10:\"{MENUTEXT}\";s:4:\"#FFF\";s:15:\"{MENUHOVERTEXT}\";s:4:\"#FFF\";s:8:\"{WRAPBG}\";s:4:\"#FFF\";s:17:\"{WRAPBORDERCOLOR}\";s:4:\"#CCC\";s:14:\"{CONTENTWIDTH}\";s:5:\"630px\";s:17:\"{CONTENTSEPARATE}\";s:7:\"#C2D5E3\";s:13:\"{INPUTBORDER}\";s:7:\"#E0E0E0\";}i:2;a:49:{s:18:\"{MENUHOVERBGCOLOR}\";s:7:\"#0051cc\";s:16:\"{MENUCURBGCOLOR}\";s:7:\"#0051cc\";s:11:\"{LIGHTLINK}\";s:4:\"#ccc\";s:14:\"{FLOATBGCOLOR}\";s:4:\"#FFF\";s:17:\"{DROPMENUBGCOLOR}\";s:7:\"#FEFEFE\";s:18:\"{FLOATMASKBGCOLOR}\";s:4:\"#000\";s:16:\"{DROPMENUBORDER}\";s:7:\"#eeeeee\";s:11:\"{SPECIALBG}\";s:7:\"#d6e4ff\";s:15:\"{SPECIALBORDER}\";s:7:\"#d6e4ff\";s:10:\"{COMMONBG}\";s:7:\"#F9f9f9\";s:14:\"{COMMONBORDER}\";s:7:\"#eeeeee\";s:9:\"{INPUTBG}\";s:4:\"#FFF\";s:22:\"{INPUTBORDERDARKCOLOR}\";s:7:\"#e3e3e3\";s:15:\"{HEADERBGCOLOR}\";s:0:\"\";s:14:\"{HEADERBORDER}\";s:1:\"0\";s:13:\"{SIDEBGCOLOR}\";s:7:\"#E8F0F7\";s:13:\"{MSGFONTSIZE}\";s:4:\"14px\";s:9:\"{BGCOLOR}\";s:7:\"#f7f9fa\";s:12:\"{NOTICETEXT}\";s:7:\"#F26C4F\";s:15:\"{HIGHLIGHTLINK}\";s:7:\"#0066ff\";s:6:\"{LINK}\";s:4:\"#333\";s:11:\"{LIGHTTEXT}\";s:4:\"#999\";s:9:\"{MIDTEXT}\";s:4:\"#666\";s:11:\"{TABLETEXT}\";s:4:\"#333\";s:12:\"{SMFONTSIZE}\";s:6:\"0.83em\";s:17:\"{THREADTITLEFONT}\";s:45:\"Tahoma,Helvetica,\'Microsoft Yahei\',sans-serif\";s:21:\"{THREADTITLEFONTSIZE}\";s:4:\"16px\";s:8:\"{SMFONT}\";s:27:\"Tahoma,Helvetica,sans-serif\";s:14:\"{TITLEBGCOLOR}\";s:7:\"#E5EDF2\";s:10:\"{FONTSIZE}\";s:8:\"12px/1.5\";s:6:\"{FONT}\";s:45:\"Tahoma,Helvetica,\'Microsoft Yahei\',sans-serif\";s:13:\"{STYLEIMGDIR}\";s:25:\"template/discuzx5/static/\";s:10:\"{BOARDIMG}\";s:8:\"logo.png\";s:11:\"{SEARCHIMG}\";s:11:\"logo_sc.svg\";s:10:\"{TOUCHIMG}\";s:10:\"logo_m.svg\";s:12:\"{HEADERTEXT}\";s:4:\"#444\";s:12:\"{FOOTERTEXT}\";s:4:\"#666\";s:13:\"{MENUBGCOLOR}\";s:7:\"#0066ff\";s:10:\"{MENUTEXT}\";s:7:\"#0066ff\";s:15:\"{MENUHOVERTEXT}\";s:4:\"#FFF\";s:8:\"{WRAPBG}\";s:4:\"#FFF\";s:17:\"{WRAPBORDERCOLOR}\";s:4:\"#CCC\";s:14:\"{CONTENTWIDTH}\";s:5:\"630px\";s:17:\"{CONTENTSEPARATE}\";s:7:\"#d6e4ff\";s:13:\"{INPUTBORDER}\";s:7:\"#e3e3e3\";s:7:\"{BTNBG}\";s:7:\"#d6e4ff\";s:8:\"{BTNTXT}\";s:7:\"#0066ff\";s:8:\"{BTNBGA}\";s:7:\"#0066ff\";s:9:\"{BTNTXTA}\";s:7:\"#ffffff\";}}'),
('stylesetting',1,1784453715,'a:0:{}'),
('threadtable_info',0,1784453715,''),
('threadtableids',0,1784453715,''),
('usergroup_1',1,1784453715,'a:197:{s:7:\"groupid\";s:1:\"1\";s:8:\"radminid\";s:1:\"1\";s:10:\"grouptitle\";s:9:\"绠＄悊鍛榎";s:5:\"stars\";s:1:\"9\";s:5:\"color\";s:0:\"\";s:4:\"icon\";s:0:\"\";s:10:\"allowvisit\";s:1:\"1\";s:11:\"allowsendpm\";s:1:\"1\";s:11:\"allowinvite\";s:1:\"1\";s:15:\"allowmailinvite\";s:1:\"1\";s:11:\"allowfollow\";s:1:\"1\";s:12:\"maxinvitenum\";s:1:\"0\";s:11:\"inviteprice\";s:1:\"0\";s:12:\"maxinviteday\";s:2:\"10\";s:9:\"upgroupid\";s:1:\"0\";s:14:\"creditsformula\";s:0:\"\";s:10:\"readaccess\";s:3:\"200\";s:9:\"allowpost\";s:1:\"1\";s:10:\"allowreply\";s:1:\"1\";s:13:\"allowpostpoll\";s:1:\"1\";s:15:\"allowpostreward\";s:1:\"1\";s:14:\"allowposttrade\";s:1:\"1\";s:17:\"allowpostactivity\";s:1:\"1\";s:15:\"allowdirectpost\";s:1:\"3\";s:14:\"allowgetattach\";s:1:\"1\";s:13:\"allowgetimage\";s:1:\"1\";s:15:\"allowpostattach\";s:1:\"1\";s:14:\"allowpostimage\";s:1:\"1\";s:9:\"allowvote\";s:1:\"1\";s:11:\"allowsearch\";s:3:\"127\";s:12:\"allowcstatus\";s:1:\"1\";s:14:\"allowinvisible\";s:1:\"1\";s:13:\"allowtransfer\";s:1:\"1\";s:16:\"allowsetreadperm\";s:1:\"1\";s:18:\"allowsetattachperm\";s:1:\"1\";s:12:\"allowposttag\";s:1:\"1\";s:13:\"allowhidecode\";s:1:\"1\";s:9:\"allowhtml\";s:1:\"1\";s:14:\"allowanonymous\";s:1:\"1\";s:14:\"allowsigbbcode\";s:1:\"1\";s:15:\"allowsigimgcode\";s:1:\"1\";s:11:\"allowmagics\";s:1:\"2\";s:17:\"disableperiodctrl\";s:1:\"1\";s:8:\"reasonpm\";s:1:\"0\";s:8:\"maxprice\";s:2:\"30\";s:10:\"maxsigsize\";s:3:\"500\";s:13:\"maxattachsize\";s:7:\"2048000\";s:13:\"maxsizeperday\";s:1:\"0\";s:17:\"maxthreadsperhour\";s:1:\"0\";s:15:\"maxpostsperhour\";s:1:\"0\";s:16:\"attachextensions\";s:0:\"\";s:9:\"raterange\";a:0:{}s:11:\"loginreward\";s:0:\"\";s:13:\"mintradeprice\";s:1:\"1\";s:13:\"maxtradeprice\";s:1:\"0\";s:14:\"minrewardprice\";s:1:\"1\";s:14:\"maxrewardprice\";s:1:\"0\";s:14:\"magicsdiscount\";s:1:\"0\";s:15:\"maxmagicsweight\";s:3:\"200\";s:15:\"allowpostdebate\";s:1:\"1\";s:10:\"tradestick\";s:1:\"5\";s:6:\"exempt\";s:3:\"255\";s:12:\"maxattachnum\";s:1:\"0\";s:12:\"allowposturl\";s:1:\"3\";s:14:\"allowrecommend\";s:1:\"1\";s:18:\"allowpostrushreply\";s:1:\"1\";s:12:\"maxfriendnum\";s:1:\"0\";s:12:\"maxspacesize\";s:1:\"0\";s:12:\"allowcomment\";s:1:\"1\";s:15:\"allowcommentmod\";s:1:\"0\";s:19:\"allowcommentarticle\";s:4:\"1000\";s:22:\"allowcommentarticlemod\";s:1:\"0\";s:14:\"searchinterval\";s:1:\"0\";s:12:\"searchignore\";s:1:\"0\";s:9:\"allowblog\";s:1:\"1\";s:10:\"allowdoing\";s:1:\"1\";s:11:\"allowupload\";s:1:\"1\";s:10:\"allowshare\";s:1:\"1\";s:12:\"allowblogmod\";s:1:\"0\";s:13:\"allowdoingmod\";s:1:\"0\";s:14:\"allowuploadmod\";s:1:\"0\";s:13:\"allowsharemod\";s:1:\"0\";s:8:\"allowcss\";s:1:\"0\";s:9:\"allowpoke\";s:1:\"1\";s:11:\"allowfriend\";s:1:\"1\";s:10:\"allowclick\";s:1:\"1\";s:10:\"allowmagic\";s:1:\"0\";s:9:\"allowstat\";s:1:\"1\";s:13:\"allowstatdata\";s:1:\"1\";s:13:\"magicdiscount\";s:1:\"0\";s:12:\"domainlength\";s:1:\"5\";s:7:\"seccode\";s:1:\"1\";s:15:\"disablepostctrl\";s:1:\"1\";s:15:\"allowbuildgroup\";s:2:\"30\";s:20:\"allowgroupdirectpost\";s:1:\"3\";s:17:\"allowgroupposturl\";s:1:\"3\";s:13:\"edittimelimit\";s:1:\"0\";s:16:\"allowpostarticle\";s:1:\"1\";s:17:\"allowdownlocalimg\";s:1:\"1\";s:18:\"allowdownremoteimg\";s:1:\"1\";s:19:\"allowpostarticlemod\";s:1:\"0\";s:17:\"allowspacediyhtml\";s:1:\"1\";s:19:\"allowspacediybbcode\";s:1:\"1\";s:20:\"allowspacediyimgcode\";s:1:\"1\";s:16:\"allowcommentpost\";s:1:\"3\";s:16:\"allowcommentitem\";s:1:\"1\";s:17:\"allowcommentreply\";s:1:\"0\";s:16:\"allowreplycredit\";s:1:\"1\";s:12:\"ignorecensor\";s:1:\"1\";s:14:\"allowsendallpm\";s:1:\"1\";s:17:\"allowsendpmmaxnum\";s:1:\"0\";s:12:\"maximagesize\";s:1:\"0\";s:14:\"allowmediacode\";s:1:\"1\";s:14:\"allowbegincode\";s:1:\"1\";s:7:\"allowat\";s:2:\"50\";s:9:\"allowsave\";s:1:\"1\";s:14:\"allowsavereply\";s:1:\"1\";s:12:\"allowsavenum\";s:1:\"0\";s:19:\"allowsetpublishdate\";s:1:\"0\";s:21:\"allowfollowcollection\";s:2:\"30\";s:22:\"allowcommentcollection\";s:1:\"1\";s:21:\"allowcreatecollection\";s:1:\"5\";s:12:\"forcesecques\";s:1:\"0\";s:10:\"forcelogin\";s:1:\"0\";s:7:\"closead\";s:1:\"0\";s:17:\"buildgroupcredits\";s:1:\"0\";s:15:\"allowimgcontent\";s:1:\"0\";s:17:\"allowavatarupload\";s:1:\"1\";s:16:\"allowviewprofile\";s:1:\"1\";s:6:\"fields\";a:0:{}s:13:\"alloweditpost\";s:1:\"1\";s:13:\"alloweditpoll\";s:1:\"1\";s:16:\"allowstickthread\";s:1:\"3\";s:12:\"allowmodpost\";s:1:\"1\";s:12:\"allowdelpost\";s:1:\"1\";s:14:\"allowmassprune\";s:1:\"1\";s:11:\"allowrefund\";s:1:\"1\";s:15:\"allowcensorword\";s:1:\"1\";s:11:\"allowviewip\";s:1:\"1\";s:10:\"allowbanip\";s:1:\"1\";s:13:\"allowedituser\";s:1:\"1\";s:12:\"allowmoduser\";s:1:\"1\";s:12:\"allowbanuser\";s:1:\"1\";s:17:\"allowbanvisituser\";s:1:\"1\";s:17:\"allowpostannounce\";s:1:\"1\";s:12:\"allowviewlog\";s:1:\"1\";s:12:\"allowbanpost\";s:1:\"1\";s:20:\"supe_allowpushthread\";s:1:\"1\";s:20:\"allowhighlightthread\";s:1:\"1\";s:15:\"allowlivethread\";s:1:\"1\";s:17:\"allowdigestthread\";s:1:\"3\";s:20:\"allowrecommendthread\";s:1:\"1\";s:15:\"allowbumpthread\";s:1:\"1\";s:16:\"allowclosethread\";s:1:\"1\";s:15:\"allowmovethread\";s:1:\"1\";s:19:\"allowedittypethread\";s:1:\"1\";s:16:\"allowstampthread\";s:1:\"1\";s:14:\"allowstamplist\";s:1:\"1\";s:15:\"allowcopythread\";s:1:\"1\";s:16:\"allowmergethread\";s:1:\"1\";s:16:\"allowsplitthread\";s:1:\"1\";s:17:\"allowrepairthread\";s:1:\"1\";s:13:\"allowwarnpost\";s:1:\"1\";s:15:\"allowviewreport\";s:1:\"1\";s:14:\"alloweditforum\";s:1:\"1\";s:17:\"allowremovereward\";s:1:\"1\";s:14:\"allowedittrade\";s:1:\"1\";s:17:\"alloweditactivity\";s:1:\"1\";s:15:\"allowstickreply\";s:1:\"1\";s:18:\"allowmanagearticle\";s:1:\"1\";s:13:\"allowaddtopic\";s:1:\"1\";s:16:\"allowmanagetopic\";s:1:\"1\";s:8:\"allowdiy\";s:1:\"1\";s:17:\"allowclearrecycle\";s:1:\"1\";s:14:\"allowmanagetag\";s:1:\"1\";s:16:\"alloweditusertag\";s:1:\"0\";s:10:\"managefeed\";s:1:\"1\";s:11:\"managedoing\";s:1:\"1\";s:11:\"manageshare\";s:1:\"1\";s:10:\"manageblog\";s:1:\"1\";s:11:\"managealbum\";s:1:\"1\";s:13:\"managecomment\";s:1:\"1\";s:14:\"managemagiclog\";s:1:\"1\";s:12:\"managereport\";s:1:\"1\";s:13:\"managehotuser\";s:1:\"1\";s:17:\"managedefaultuser\";s:1:\"1\";s:11:\"managemagic\";s:1:\"1\";s:11:\"manageclick\";s:1:\"1\";s:21:\"allowmanagecollection\";s:1:\"1\";s:13:\"allowmakehtml\";s:1:\"1\";s:9:\"grouptype\";s:6:\"system\";s:11:\"grouppublic\";b:0;s:18:\"groupcreditshigher\";s:1:\"0\";s:17:\"groupcreditslower\";s:1:\"0\";s:17:\"allowthreadplugin\";a:0:{}s:6:\"plugin\";N;s:5:\"style\";N;}'),
('usergroup_10',1,1784453715,'a:137:{s:7:\"groupid\";s:2:\"10\";s:8:\"radminid\";s:1:\"0\";s:10:\"grouptitle\";s:12:\"鏂版墜涓婅矾\";s:5:\"stars\";s:1:\"1\";s:5:\"color\";s:0:\"\";s:4:\"icon\";s:0:\"\";s:10:\"allowvisit\";s:1:\"1\";s:11:\"allowsendpm\";s:1:\"1\";s:11:\"allowinvite\";s:1:\"0\";s:15:\"allowmailinvite\";s:1:\"0\";s:11:\"allowfollow\";s:1:\"0\";s:12:\"maxinvitenum\";s:1:\"0\";s:11:\"inviteprice\";s:1:\"0\";s:12:\"maxinviteday\";s:2:\"10\";s:9:\"upgroupid\";s:1:\"0\";s:14:\"creditsformula\";s:0:\"\";s:10:\"readaccess\";s:2:\"10\";s:9:\"allowpost\";s:1:\"1\";s:10:\"allowreply\";s:1:\"1\";s:13:\"allowpostpoll\";s:1:\"1\";s:15:\"allowpostreward\";s:1:\"1\";s:14:\"allowposttrade\";s:1:\"1\";s:17:\"allowpostactivity\";s:1:\"1\";s:15:\"allowdirectpost\";s:1:\"0\";s:14:\"allowgetattach\";s:1:\"1\";s:13:\"allowgetimage\";s:1:\"1\";s:15:\"allowpostattach\";s:1:\"1\";s:14:\"allowpostimage\";s:1:\"1\";s:9:\"allowvote\";s:1:\"1\";s:11:\"allowsearch\";s:2:\"95\";s:12:\"allowcstatus\";s:1:\"0\";s:14:\"allowinvisible\";s:1:\"0\";s:13:\"allowtransfer\";s:1:\"0\";s:16:\"allowsetreadperm\";s:1:\"0\";s:18:\"allowsetattachperm\";s:1:\"0\";s:12:\"allowposttag\";s:1:\"0\";s:13:\"allowhidecode\";s:1:\"0\";s:9:\"allowhtml\";s:1:\"0\";s:14:\"allowanonymous\";s:1:\"0\";s:14:\"allowsigbbcode\";s:1:\"1\";s:15:\"allowsigimgcode\";s:1:\"0\";s:11:\"allowmagics\";s:1:\"1\";s:17:\"disableperiodctrl\";s:1:\"0\";s:8:\"reasonpm\";s:1:\"0\";s:8:\"maxprice\";s:1:\"0\";s:10:\"maxsigsize\";s:2:\"80\";s:13:\"maxattachsize\";s:7:\"1024000\";s:13:\"maxsizeperday\";s:1:\"0\";s:17:\"maxthreadsperhour\";s:1:\"0\";s:15:\"maxpostsperhour\";s:1:\"0\";s:16:\"attachextensions\";s:55:\"chm, pdf, zip, rar, tar, gz, bzip2, gif, jpg, jpeg, png\";s:9:\"raterange\";a:0:{}s:11:\"loginreward\";s:0:\"\";s:13:\"mintradeprice\";s:1:\"1\";s:13:\"maxtradeprice\";s:1:\"0\";s:14:\"minrewardprice\";s:1:\"1\";s:14:\"maxrewardprice\";s:1:\"0\";s:14:\"magicsdiscount\";s:1:\"0\";s:15:\"maxmagicsweight\";s:2:\"40\";s:15:\"allowpostdebate\";s:1:\"1\";s:10:\"tradestick\";s:1:\"5\";s:6:\"exempt\";s:1:\"0\";s:12:\"maxattachnum\";s:1:\"0\";s:12:\"allowposturl\";s:1:\"3\";s:14:\"allowrecommend\";s:1:\"1\";s:18:\"allowpostrushreply\";s:1:\"0\";s:12:\"maxfriendnum\";s:1:\"0\";s:12:\"maxspacesize\";s:1:\"0\";s:12:\"allowcomment\";s:1:\"1\";s:15:\"allowcommentmod\";s:1:\"0\";s:19:\"allowcommentarticle\";s:4:\"1000\";s:22:\"allowcommentarticlemod\";s:1:\"0\";s:14:\"searchinterval\";s:1:\"0\";s:12:\"searchignore\";s:1:\"0\";s:9:\"allowblog\";s:1:\"1\";s:10:\"allowdoing\";s:1:\"1\";s:11:\"allowupload\";s:1:\"1\";s:10:\"allowshare\";s:1:\"1\";s:12:\"allowblogmod\";s:1:\"0\";s:13:\"allowdoingmod\";s:1:\"0\";s:14:\"allowuploadmod\";s:1:\"0\";s:13:\"allowsharemod\";s:1:\"0\";s:8:\"allowcss\";s:1:\"0\";s:9:\"allowpoke\";s:1:\"1\";s:11:\"allowfriend\";s:1:\"1\";s:10:\"allowclick\";s:1:\"1\";s:10:\"allowmagic\";s:1:\"0\";s:9:\"allowstat\";s:1:\"0\";s:13:\"allowstatdata\";s:1:\"0\";s:13:\"magicdiscount\";s:1:\"0\";s:12:\"domainlength\";s:1:\"5\";s:7:\"seccode\";s:1:\"1\";s:15:\"disablepostctrl\";s:1:\"0\";s:15:\"allowbuildgroup\";s:1:\"5\";s:20:\"allowgroupdirectpost\";s:1:\"3\";s:17:\"allowgroupposturl\";s:1:\"0\";s:13:\"edittimelimit\";s:1:\"0\";s:16:\"allowpostarticle\";s:1:\"0\";s:17:\"allowdownlocalimg\";s:1:\"0\";s:18:\"allowdownremoteimg\";s:1:\"0\";s:19:\"allowpostarticlemod\";s:1:\"0\";s:17:\"allowspacediyhtml\";s:1:\"0\";s:19:\"allowspacediybbcode\";s:1:\"1\";s:20:\"allowspacediyimgcode\";s:1:\"1\";s:16:\"allowcommentpost\";s:1:\"2\";s:16:\"allowcommentitem\";s:1:\"1\";s:17:\"allowcommentreply\";s:1:\"0\";s:16:\"allowreplycredit\";s:1:\"0\";s:12:\"ignorecensor\";s:1:\"0\";s:14:\"allowsendallpm\";s:1:\"0\";s:17:\"allowsendpmmaxnum\";s:1:\"0\";s:12:\"maximagesize\";s:1:\"0\";s:14:\"allowmediacode\";s:1:\"0\";s:14:\"allowbegincode\";s:1:\"0\";s:7:\"allowat\";s:1:\"0\";s:9:\"allowsave\";s:1:\"1\";s:14:\"allowsavereply\";s:1:\"1\";s:12:\"allowsavenum\";s:1:\"0\";s:19:\"allowsetpublishdate\";s:1:\"0\";s:21:\"allowfollowcollection\";s:2:\"30\";s:22:\"allowcommentcollection\";s:1:\"1\";s:21:\"allowcreatecollection\";s:1:\"5\";s:12:\"forcesecques\";s:1:\"0\";s:10:\"forcelogin\";s:1:\"0\";s:7:\"closead\";s:1:\"0\";s:17:\"buildgroupcredits\";s:1:\"0\";s:15:\"allowimgcontent\";s:1:\"0\";s:17:\"allowavatarupload\";s:1:\"1\";s:16:\"allowviewprofile\";s:1:\"1\";s:6:\"fields\";a:0:{}s:9:\"grouptype\";s:6:\"member\";s:11:\"grouppublic\";b:0;s:18:\"groupcreditshigher\";s:1:\"0\";s:17:\"groupcreditslower\";s:2:\"50\";s:17:\"allowthreadplugin\";a:0:{}s:6:\"plugin\";N;s:5:\"style\";N;}'),
('usergroup_11',1,1784453715,'a:137:{s:7:\"groupid\";s:2:\"11\";s:8:\"radminid\";s:1:\"0\";s:10:\"grouptitle\";s:12:\"娉ㄥ唽浼氬憳\";s:5:\"stars\";s:1:\"2\";s:5:\"color\";s:0:\"\";s:4:\"icon\";s:0:\"\";s:10:\"allowvisit\";s:1:\"1\";s:11:\"allowsendpm\";s:1:\"1\";s:11:\"allowinvite\";s:1:\"0\";s:15:\"allowmailinvite\";s:1:\"0\";s:11:\"allowfollow\";s:1:\"0\";s:12:\"maxinvitenum\";s:1:\"0\";s:11:\"inviteprice\";s:1:\"0\";s:12:\"maxinviteday\";s:2:\"10\";s:9:\"upgroupid\";s:1:\"0\";s:14:\"creditsformula\";s:0:\"\";s:10:\"readaccess\";s:2:\"20\";s:9:\"allowpost\";s:1:\"1\";s:10:\"allowreply\";s:1:\"1\";s:13:\"allowpostpoll\";s:1:\"1\";s:15:\"allowpostreward\";s:1:\"1\";s:14:\"allowposttrade\";s:1:\"1\";s:17:\"allowpostactivity\";s:1:\"1\";s:15:\"allowdirectpost\";s:1:\"0\";s:14:\"allowgetattach\";s:1:\"1\";s:13:\"allowgetimage\";s:1:\"1\";s:15:\"allowpostattach\";s:1:\"1\";s:14:\"allowpostimage\";s:1:\"1\";s:9:\"allowvote\";s:1:\"1\";s:11:\"allowsearch\";s:2:\"95\";s:12:\"allowcstatus\";s:1:\"0\";s:14:\"allowinvisible\";s:1:\"0\";s:13:\"allowtransfer\";s:1:\"0\";s:16:\"allowsetreadperm\";s:1:\"0\";s:18:\"allowsetattachperm\";s:1:\"0\";s:12:\"allowposttag\";s:1:\"0\";s:13:\"allowhidecode\";s:1:\"0\";s:9:\"allowhtml\";s:1:\"0\";s:14:\"allowanonymous\";s:1:\"0\";s:14:\"allowsigbbcode\";s:1:\"1\";s:15:\"allowsigimgcode\";s:1:\"0\";s:11:\"allowmagics\";s:1:\"1\";s:17:\"disableperiodctrl\";s:1:\"0\";s:8:\"reasonpm\";s:1:\"0\";s:8:\"maxprice\";s:1:\"0\";s:10:\"maxsigsize\";s:3:\"100\";s:13:\"maxattachsize\";s:7:\"1024000\";s:13:\"maxsizeperday\";s:1:\"0\";s:17:\"maxthreadsperhour\";s:1:\"0\";s:15:\"maxpostsperhour\";s:1:\"0\";s:16:\"attachextensions\";s:55:\"chm, pdf, zip, rar, tar, gz, bzip2, gif, jpg, jpeg, png\";s:9:\"raterange\";a:0:{}s:11:\"loginreward\";s:0:\"\";s:13:\"mintradeprice\";s:1:\"1\";s:13:\"maxtradeprice\";s:1:\"0\";s:14:\"minrewardprice\";s:1:\"1\";s:14:\"maxrewardprice\";s:1:\"0\";s:14:\"magicsdiscount\";s:1:\"0\";s:15:\"maxmagicsweight\";s:2:\"60\";s:15:\"allowpostdebate\";s:1:\"1\";s:10:\"tradestick\";s:1:\"5\";s:6:\"exempt\";s:1:\"0\";s:12:\"maxattachnum\";s:1:\"0\";s:12:\"allowposturl\";s:1:\"3\";s:14:\"allowrecommend\";s:1:\"1\";s:18:\"allowpostrushreply\";s:1:\"0\";s:12:\"maxfriendnum\";s:1:\"0\";s:12:\"maxspacesize\";s:1:\"0\";s:12:\"allowcomment\";s:1:\"1\";s:15:\"allowcommentmod\";s:1:\"0\";s:19:\"allowcommentarticle\";s:4:\"1000\";s:22:\"allowcommentarticlemod\";s:1:\"0\";s:14:\"searchinterval\";s:1:\"0\";s:12:\"searchignore\";s:1:\"0\";s:9:\"allowblog\";s:1:\"1\";s:10:\"allowdoing\";s:1:\"1\";s:11:\"allowupload\";s:1:\"1\";s:10:\"allowshare\";s:1:\"1\";s:12:\"allowblogmod\";s:1:\"0\";s:13:\"allowdoingmod\";s:1:\"0\";s:14:\"allowuploadmod\";s:1:\"0\";s:13:\"allowsharemod\";s:1:\"0\";s:8:\"allowcss\";s:1:\"0\";s:9:\"allowpoke\";s:1:\"1\";s:11:\"allowfriend\";s:1:\"1\";s:10:\"allowclick\";s:1:\"1\";s:10:\"allowmagic\";s:1:\"0\";s:9:\"allowstat\";s:1:\"0\";s:13:\"allowstatdata\";s:1:\"0\";s:13:\"magicdiscount\";s:1:\"0\";s:12:\"domainlength\";s:1:\"5\";s:7:\"seccode\";s:1:\"1\";s:15:\"disablepostctrl\";s:1:\"0\";s:15:\"allowbuildgroup\";s:1:\"5\";s:20:\"allowgroupdirectpost\";s:1:\"3\";s:17:\"allowgroupposturl\";s:1:\"0\";s:13:\"edittimelimit\";s:1:\"0\";s:16:\"allowpostarticle\";s:1:\"0\";s:17:\"allowdownlocalimg\";s:1:\"0\";s:18:\"allowdownremoteimg\";s:1:\"0\";s:19:\"allowpostarticlemod\";s:1:\"0\";s:17:\"allowspacediyhtml\";s:1:\"0\";s:19:\"allowspacediybbcode\";s:1:\"1\";s:20:\"allowspacediyimgcode\";s:1:\"1\";s:16:\"allowcommentpost\";s:1:\"2\";s:16:\"allowcommentitem\";s:1:\"1\";s:17:\"allowcommentreply\";s:1:\"0\";s:16:\"allowreplycredit\";s:1:\"1\";s:12:\"ignorecensor\";s:1:\"0\";s:14:\"allowsendallpm\";s:1:\"0\";s:17:\"allowsendpmmaxnum\";s:1:\"0\";s:12:\"maximagesize\";s:1:\"0\";s:14:\"allowmediacode\";s:1:\"0\";s:14:\"allowbegincode\";s:1:\"0\";s:7:\"allowat\";s:1:\"0\";s:9:\"allowsave\";s:1:\"1\";s:14:\"allowsavereply\";s:1:\"1\";s:12:\"allowsavenum\";s:1:\"0\";s:19:\"allowsetpublishdate\";s:1:\"0\";s:21:\"allowfollowcollection\";s:2:\"30\";s:22:\"allowcommentcollection\";s:1:\"1\";s:21:\"allowcreatecollection\";s:1:\"5\";s:12:\"forcesecques\";s:1:\"0\";s:10:\"forcelogin\";s:1:\"0\";s:7:\"closead\";s:1:\"0\";s:17:\"buildgroupcredits\";s:1:\"0\";s:15:\"allowimgcontent\";s:1:\"0\";s:17:\"allowavatarupload\";s:1:\"1\";s:16:\"allowviewprofile\";s:1:\"1\";s:6:\"fields\";a:0:{}s:9:\"grouptype\";s:6:\"member\";s:11:\"grouppublic\";b:0;s:18:\"groupcreditshigher\";s:2:\"50\";s:17:\"groupcreditslower\";s:3:\"200\";s:17:\"allowthreadplugin\";a:0:{}s:6:\"plugin\";N;s:5:\"style\";N;}'),
('usergroup_12',1,1784453715,'a:137:{s:7:\"groupid\";s:2:\"12\";s:8:\"radminid\";s:1:\"0\";s:10:\"grouptitle\";s:12:\"涓骇浼氬憳\";s:5:\"stars\";s:1:\"3\";s:5:\"color\";s:0:\"\";s:4:\"icon\";s:0:\"\";s:10:\"allowvisit\";s:1:\"1\";s:11:\"allowsendpm\";s:1:\"1\";s:11:\"allowinvite\";s:1:\"0\";s:15:\"allowmailinvite\";s:1:\"0\";s:11:\"allowfollow\";s:1:\"0\";s:12:\"maxinvitenum\";s:1:\"0\";s:11:\"inviteprice\";s:1:\"0\";s:12:\"maxinviteday\";s:2:\"10\";s:9:\"upgroupid\";s:1:\"0\";s:14:\"creditsformula\";s:0:\"\";s:10:\"readaccess\";s:2:\"30\";s:9:\"allowpost\";s:1:\"1\";s:10:\"allowreply\";s:1:\"1\";s:13:\"allowpostpoll\";s:1:\"1\";s:15:\"allowpostreward\";s:1:\"1\";s:14:\"allowposttrade\";s:1:\"1\";s:17:\"allowpostactivity\";s:1:\"1\";s:15:\"allowdirectpost\";s:1:\"0\";s:14:\"allowgetattach\";s:1:\"1\";s:13:\"allowgetimage\";s:1:\"1\";s:15:\"allowpostattach\";s:1:\"1\";s:14:\"allowpostimage\";s:1:\"1\";s:9:\"allowvote\";s:1:\"1\";s:11:\"allowsearch\";s:2:\"95\";s:12:\"allowcstatus\";s:1:\"0\";s:14:\"allowinvisible\";s:1:\"0\";s:13:\"allowtransfer\";s:1:\"0\";s:16:\"allowsetreadperm\";s:1:\"0\";s:18:\"allowsetattachperm\";s:1:\"0\";s:12:\"allowposttag\";s:1:\"0\";s:13:\"allowhidecode\";s:1:\"0\";s:9:\"allowhtml\";s:1:\"0\";s:14:\"allowanonymous\";s:1:\"0\";s:14:\"allowsigbbcode\";s:1:\"1\";s:15:\"allowsigimgcode\";s:1:\"0\";s:11:\"allowmagics\";s:1:\"1\";s:17:\"disableperiodctrl\";s:1:\"0\";s:8:\"reasonpm\";s:1:\"0\";s:8:\"maxprice\";s:1:\"0\";s:10:\"maxsigsize\";s:3:\"150\";s:13:\"maxattachsize\";s:7:\"1024000\";s:13:\"maxsizeperday\";s:1:\"0\";s:17:\"maxthreadsperhour\";s:1:\"0\";s:15:\"maxpostsperhour\";s:1:\"0\";s:16:\"attachextensions\";s:55:\"chm, pdf, zip, rar, tar, gz, bzip2, gif, jpg, jpeg, png\";s:9:\"raterange\";a:0:{}s:11:\"loginreward\";s:0:\"\";s:13:\"mintradeprice\";s:1:\"1\";s:13:\"maxtradeprice\";s:1:\"0\";s:14:\"minrewardprice\";s:1:\"1\";s:14:\"maxrewardprice\";s:1:\"0\";s:14:\"magicsdiscount\";s:1:\"0\";s:15:\"maxmagicsweight\";s:2:\"80\";s:15:\"allowpostdebate\";s:1:\"1\";s:10:\"tradestick\";s:1:\"5\";s:6:\"exempt\";s:1:\"0\";s:12:\"maxattachnum\";s:1:\"0\";s:12:\"allowposturl\";s:1:\"3\";s:14:\"allowrecommend\";s:1:\"1\";s:18:\"allowpostrushreply\";s:1:\"0\";s:12:\"maxfriendnum\";s:1:\"0\";s:12:\"maxspacesize\";s:1:\"0\";s:12:\"allowcomment\";s:1:\"1\";s:15:\"allowcommentmod\";s:1:\"0\";s:19:\"allowcommentarticle\";s:4:\"1000\";s:22:\"allowcommentarticlemod\";s:1:\"0\";s:14:\"searchinterval\";s:1:\"0\";s:12:\"searchignore\";s:1:\"0\";s:9:\"allowblog\";s:1:\"1\";s:10:\"allowdoing\";s:1:\"1\";s:11:\"allowupload\";s:1:\"1\";s:10:\"allowshare\";s:1:\"1\";s:12:\"allowblogmod\";s:1:\"0\";s:13:\"allowdoingmod\";s:1:\"0\";s:14:\"allowuploadmod\";s:1:\"0\";s:13:\"allowsharemod\";s:1:\"0\";s:8:\"allowcss\";s:1:\"0\";s:9:\"allowpoke\";s:1:\"1\";s:11:\"allowfriend\";s:1:\"1\";s:10:\"allowclick\";s:1:\"1\";s:10:\"allowmagic\";s:1:\"0\";s:9:\"allowstat\";s:1:\"0\";s:13:\"allowstatdata\";s:1:\"0\";s:13:\"magicdiscount\";s:1:\"0\";s:12:\"domainlength\";s:1:\"5\";s:7:\"seccode\";s:1:\"1\";s:15:\"disablepostctrl\";s:1:\"0\";s:15:\"allowbuildgroup\";s:1:\"5\";s:20:\"allowgroupdirectpost\";s:1:\"3\";s:17:\"allowgroupposturl\";s:1:\"0\";s:13:\"edittimelimit\";s:1:\"0\";s:16:\"allowpostarticle\";s:1:\"0\";s:17:\"allowdownlocalimg\";s:1:\"0\";s:18:\"allowdownremoteimg\";s:1:\"0\";s:19:\"allowpostarticlemod\";s:1:\"0\";s:17:\"allowspacediyhtml\";s:1:\"0\";s:19:\"allowspacediybbcode\";s:1:\"1\";s:20:\"allowspacediyimgcode\";s:1:\"1\";s:16:\"allowcommentpost\";s:1:\"2\";s:16:\"allowcommentitem\";s:1:\"1\";s:17:\"allowcommentreply\";s:1:\"0\";s:16:\"allowreplycredit\";s:1:\"1\";s:12:\"ignorecensor\";s:1:\"0\";s:14:\"allowsendallpm\";s:1:\"0\";s:17:\"allowsendpmmaxnum\";s:1:\"0\";s:12:\"maximagesize\";s:1:\"0\";s:14:\"allowmediacode\";s:1:\"0\";s:14:\"allowbegincode\";s:1:\"0\";s:7:\"allowat\";s:1:\"0\";s:9:\"allowsave\";s:1:\"1\";s:14:\"allowsavereply\";s:1:\"1\";s:12:\"allowsavenum\";s:1:\"0\";s:19:\"allowsetpublishdate\";s:1:\"0\";s:21:\"allowfollowcollection\";s:2:\"30\";s:22:\"allowcommentcollection\";s:1:\"1\";s:21:\"allowcreatecollection\";s:1:\"5\";s:12:\"forcesecques\";s:1:\"0\";s:10:\"forcelogin\";s:1:\"0\";s:7:\"closead\";s:1:\"0\";s:17:\"buildgroupcredits\";s:1:\"0\";s:15:\"allowimgcontent\";s:1:\"0\";s:17:\"allowavatarupload\";s:1:\"1\";s:16:\"allowviewprofile\";s:1:\"1\";s:6:\"fields\";a:0:{}s:9:\"grouptype\";s:6:\"member\";s:11:\"grouppublic\";b:0;s:18:\"groupcreditshigher\";s:3:\"200\";s:17:\"groupcreditslower\";s:3:\"500\";s:17:\"allowthreadplugin\";a:0:{}s:6:\"plugin\";N;s:5:\"style\";N;}'),
('usergroup_13',1,1784453715,'a:137:{s:7:\"groupid\";s:2:\"13\";s:8:\"radminid\";s:1:\"0\";s:10:\"grouptitle\";s:12:\"楂樼骇浼氬憳\";s:5:\"stars\";s:1:\"4\";s:5:\"color\";s:0:\"\";s:4:\"icon\";s:0:\"\";s:10:\"allowvisit\";s:1:\"1\";s:11:\"allowsendpm\";s:1:\"1\";s:11:\"allowinvite\";s:1:\"0\";s:15:\"allowmailinvite\";s:1:\"0\";s:11:\"allowfollow\";s:1:\"1\";s:12:\"maxinvitenum\";s:1:\"0\";s:11:\"inviteprice\";s:1:\"0\";s:12:\"maxinviteday\";s:2:\"10\";s:9:\"upgroupid\";s:1:\"0\";s:14:\"creditsformula\";s:0:\"\";s:10:\"readaccess\";s:2:\"50\";s:9:\"allowpost\";s:1:\"1\";s:10:\"allowreply\";s:1:\"1\";s:13:\"allowpostpoll\";s:1:\"1\";s:15:\"allowpostreward\";s:1:\"1\";s:14:\"allowposttrade\";s:1:\"1\";s:17:\"allowpostactivity\";s:1:\"1\";s:15:\"allowdirectpost\";s:1:\"0\";s:14:\"allowgetattach\";s:1:\"1\";s:13:\"allowgetimage\";s:1:\"1\";s:15:\"allowpostattach\";s:1:\"1\";s:14:\"allowpostimage\";s:1:\"1\";s:9:\"allowvote\";s:1:\"1\";s:11:\"allowsearch\";s:2:\"95\";s:12:\"allowcstatus\";s:1:\"1\";s:14:\"allowinvisible\";s:1:\"0\";s:13:\"allowtransfer\";s:1:\"0\";s:16:\"allowsetreadperm\";s:1:\"0\";s:18:\"allowsetattachperm\";s:1:\"0\";s:12:\"allowposttag\";s:1:\"0\";s:13:\"allowhidecode\";s:1:\"0\";s:9:\"allowhtml\";s:1:\"0\";s:14:\"allowanonymous\";s:1:\"0\";s:14:\"allowsigbbcode\";s:1:\"1\";s:15:\"allowsigimgcode\";s:1:\"0\";s:11:\"allowmagics\";s:1:\"2\";s:17:\"disableperiodctrl\";s:1:\"0\";s:8:\"reasonpm\";s:1:\"0\";s:8:\"maxprice\";s:1:\"0\";s:10:\"maxsigsize\";s:3:\"200\";s:13:\"maxattachsize\";s:7:\"2048000\";s:13:\"maxsizeperday\";s:1:\"0\";s:17:\"maxthreadsperhour\";s:1:\"0\";s:15:\"maxpostsperhour\";s:1:\"0\";s:16:\"attachextensions\";s:55:\"chm, pdf, zip, rar, tar, gz, bzip2, gif, jpg, jpeg, png\";s:9:\"raterange\";a:0:{}s:11:\"loginreward\";s:0:\"\";s:13:\"mintradeprice\";s:1:\"1\";s:13:\"maxtradeprice\";s:1:\"0\";s:14:\"minrewardprice\";s:1:\"1\";s:14:\"maxrewardprice\";s:1:\"0\";s:14:\"magicsdiscount\";s:1:\"0\";s:15:\"maxmagicsweight\";s:3:\"100\";s:15:\"allowpostdebate\";s:1:\"1\";s:10:\"tradestick\";s:1:\"5\";s:6:\"exempt\";s:1:\"0\";s:12:\"maxattachnum\";s:1:\"0\";s:12:\"allowposturl\";s:1:\"3\";s:14:\"allowrecommend\";s:1:\"1\";s:18:\"allowpostrushreply\";s:1:\"0\";s:12:\"maxfriendnum\";s:1:\"0\";s:12:\"maxspacesize\";s:1:\"0\";s:12:\"allowcomment\";s:1:\"1\";s:15:\"allowcommentmod\";s:1:\"0\";s:19:\"allowcommentarticle\";s:4:\"1000\";s:22:\"allowcommentarticlemod\";s:1:\"0\";s:14:\"searchinterval\";s:1:\"0\";s:12:\"searchignore\";s:1:\"0\";s:9:\"allowblog\";s:1:\"1\";s:10:\"allowdoing\";s:1:\"1\";s:11:\"allowupload\";s:1:\"1\";s:10:\"allowshare\";s:1:\"1\";s:12:\"allowblogmod\";s:1:\"0\";s:13:\"allowdoingmod\";s:1:\"0\";s:14:\"allowuploadmod\";s:1:\"0\";s:13:\"allowsharemod\";s:1:\"0\";s:8:\"allowcss\";s:1:\"0\";s:9:\"allowpoke\";s:1:\"1\";s:11:\"allowfriend\";s:1:\"1\";s:10:\"allowclick\";s:1:\"1\";s:10:\"allowmagic\";s:1:\"0\";s:9:\"allowstat\";s:1:\"0\";s:13:\"allowstatdata\";s:1:\"0\";s:13:\"magicdiscount\";s:1:\"0\";s:12:\"domainlength\";s:1:\"5\";s:7:\"seccode\";s:1:\"1\";s:15:\"disablepostctrl\";s:1:\"0\";s:15:\"allowbuildgroup\";s:2:\"10\";s:20:\"allowgroupdirectpost\";s:1:\"3\";s:17:\"allowgroupposturl\";s:1:\"0\";s:13:\"edittimelimit\";s:1:\"0\";s:16:\"allowpostarticle\";s:1:\"0\";s:17:\"allowdownlocalimg\";s:1:\"0\";s:18:\"allowdownremoteimg\";s:1:\"0\";s:19:\"allowpostarticlemod\";s:1:\"0\";s:17:\"allowspacediyhtml\";s:1:\"0\";s:19:\"allowspacediybbcode\";s:1:\"1\";s:20:\"allowspacediyimgcode\";s:1:\"1\";s:16:\"allowcommentpost\";s:1:\"2\";s:16:\"allowcommentitem\";s:1:\"1\";s:17:\"allowcommentreply\";s:1:\"0\";s:16:\"allowreplycredit\";s:1:\"1\";s:12:\"ignorecensor\";s:1:\"0\";s:14:\"allowsendallpm\";s:1:\"0\";s:17:\"allowsendpmmaxnum\";s:1:\"0\";s:12:\"maximagesize\";s:1:\"0\";s:14:\"allowmediacode\";s:1:\"0\";s:14:\"allowbegincode\";s:1:\"0\";s:7:\"allowat\";s:1:\"0\";s:9:\"allowsave\";s:1:\"1\";s:14:\"allowsavereply\";s:1:\"1\";s:12:\"allowsavenum\";s:1:\"0\";s:19:\"allowsetpublishdate\";s:1:\"0\";s:21:\"allowfollowcollection\";s:2:\"30\";s:22:\"allowcommentcollection\";s:1:\"1\";s:21:\"allowcreatecollection\";s:1:\"5\";s:12:\"forcesecques\";s:1:\"0\";s:10:\"forcelogin\";s:1:\"0\";s:7:\"closead\";s:1:\"0\";s:17:\"buildgroupcredits\";s:1:\"0\";s:15:\"allowimgcontent\";s:1:\"0\";s:17:\"allowavatarupload\";s:1:\"1\";s:16:\"allowviewprofile\";s:1:\"1\";s:6:\"fields\";a:0:{}s:9:\"grouptype\";s:6:\"member\";s:11:\"grouppublic\";b:0;s:18:\"groupcreditshigher\";s:3:\"500\";s:17:\"groupcreditslower\";s:4:\"1000\";s:17:\"allowthreadplugin\";a:0:{}s:6:\"plugin\";N;s:5:\"style\";N;}'),
('usergroup_14',1,1784453715,'a:137:{s:7:\"groupid\";s:2:\"14\";s:8:\"radminid\";s:1:\"0\";s:10:\"grouptitle\";s:12:\"閲戠墝浼氬憳\";s:5:\"stars\";s:1:\"6\";s:5:\"color\";s:0:\"\";s:4:\"icon\";s:0:\"\";s:10:\"allowvisit\";s:1:\"1\";s:11:\"allowsendpm\";s:1:\"1\";s:11:\"allowinvite\";s:1:\"0\";s:15:\"allowmailinvite\";s:1:\"0\";s:11:\"allowfollow\";s:1:\"1\";s:12:\"maxinvitenum\";s:1:\"0\";s:11:\"inviteprice\";s:1:\"0\";s:12:\"maxinviteday\";s:2:\"10\";s:9:\"upgroupid\";s:1:\"0\";s:14:\"creditsformula\";s:0:\"\";s:10:\"readaccess\";s:2:\"70\";s:9:\"allowpost\";s:1:\"1\";s:10:\"allowreply\";s:1:\"1\";s:13:\"allowpostpoll\";s:1:\"1\";s:15:\"allowpostreward\";s:1:\"1\";s:14:\"allowposttrade\";s:1:\"1\";s:17:\"allowpostactivity\";s:1:\"1\";s:15:\"allowdirectpost\";s:1:\"0\";s:14:\"allowgetattach\";s:1:\"1\";s:13:\"allowgetimage\";s:1:\"1\";s:15:\"allowpostattach\";s:1:\"1\";s:14:\"allowpostimage\";s:1:\"1\";s:9:\"allowvote\";s:1:\"1\";s:11:\"allowsearch\";s:2:\"95\";s:12:\"allowcstatus\";s:1:\"1\";s:14:\"allowinvisible\";s:1:\"0\";s:13:\"allowtransfer\";s:1:\"0\";s:16:\"allowsetreadperm\";s:1:\"1\";s:18:\"allowsetattachperm\";s:1:\"1\";s:12:\"allowposttag\";s:1:\"0\";s:13:\"allowhidecode\";s:1:\"0\";s:9:\"allowhtml\";s:1:\"0\";s:14:\"allowanonymous\";s:1:\"0\";s:14:\"allowsigbbcode\";s:1:\"1\";s:15:\"allowsigimgcode\";s:1:\"1\";s:11:\"allowmagics\";s:1:\"2\";s:17:\"disableperiodctrl\";s:1:\"0\";s:8:\"reasonpm\";s:1:\"0\";s:8:\"maxprice\";s:1:\"0\";s:10:\"maxsigsize\";s:3:\"300\";s:13:\"maxattachsize\";s:7:\"2048000\";s:13:\"maxsizeperday\";s:1:\"0\";s:17:\"maxthreadsperhour\";s:1:\"0\";s:15:\"maxpostsperhour\";s:1:\"0\";s:16:\"attachextensions\";s:55:\"chm, pdf, zip, rar, tar, gz, bzip2, gif, jpg, jpeg, png\";s:9:\"raterange\";a:0:{}s:11:\"loginreward\";s:0:\"\";s:13:\"mintradeprice\";s:1:\"1\";s:13:\"maxtradeprice\";s:1:\"0\";s:14:\"minrewardprice\";s:1:\"1\";s:14:\"maxrewardprice\";s:1:\"0\";s:14:\"magicsdiscount\";s:1:\"0\";s:15:\"maxmagicsweight\";s:3:\"120\";s:15:\"allowpostdebate\";s:1:\"1\";s:10:\"tradestick\";s:1:\"5\";s:6:\"exempt\";s:1:\"0\";s:12:\"maxattachnum\";s:1:\"0\";s:12:\"allowposturl\";s:1:\"3\";s:14:\"allowrecommend\";s:1:\"1\";s:18:\"allowpostrushreply\";s:1:\"0\";s:12:\"maxfriendnum\";s:1:\"0\";s:12:\"maxspacesize\";s:1:\"0\";s:12:\"allowcomment\";s:1:\"1\";s:15:\"allowcommentmod\";s:1:\"0\";s:19:\"allowcommentarticle\";s:4:\"1000\";s:22:\"allowcommentarticlemod\";s:1:\"0\";s:14:\"searchinterval\";s:1:\"0\";s:12:\"searchignore\";s:1:\"0\";s:9:\"allowblog\";s:1:\"1\";s:10:\"allowdoing\";s:1:\"1\";s:11:\"allowupload\";s:1:\"1\";s:10:\"allowshare\";s:1:\"1\";s:12:\"allowblogmod\";s:1:\"0\";s:13:\"allowdoingmod\";s:1:\"0\";s:14:\"allowuploadmod\";s:1:\"0\";s:13:\"allowsharemod\";s:1:\"0\";s:8:\"allowcss\";s:1:\"0\";s:9:\"allowpoke\";s:1:\"1\";s:11:\"allowfriend\";s:1:\"1\";s:10:\"allowclick\";s:1:\"1\";s:10:\"allowmagic\";s:1:\"0\";s:9:\"allowstat\";s:1:\"0\";s:13:\"allowstatdata\";s:1:\"0\";s:13:\"magicdiscount\";s:1:\"0\";s:12:\"domainlength\";s:1:\"5\";s:7:\"seccode\";s:1:\"1\";s:15:\"disablepostctrl\";s:1:\"0\";s:15:\"allowbuildgroup\";s:2:\"10\";s:20:\"allowgroupdirectpost\";s:1:\"3\";s:17:\"allowgroupposturl\";s:1:\"0\";s:13:\"edittimelimit\";s:1:\"0\";s:16:\"allowpostarticle\";s:1:\"0\";s:17:\"allowdownlocalimg\";s:1:\"0\";s:18:\"allowdownremoteimg\";s:1:\"0\";s:19:\"allowpostarticlemod\";s:1:\"0\";s:17:\"allowspacediyhtml\";s:1:\"0\";s:19:\"allowspacediybbcode\";s:1:\"1\";s:20:\"allowspacediyimgcode\";s:1:\"1\";s:16:\"allowcommentpost\";s:1:\"2\";s:16:\"allowcommentitem\";s:1:\"1\";s:17:\"allowcommentreply\";s:1:\"0\";s:16:\"allowreplycredit\";s:1:\"1\";s:12:\"ignorecensor\";s:1:\"0\";s:14:\"allowsendallpm\";s:1:\"0\";s:17:\"allowsendpmmaxnum\";s:1:\"0\";s:12:\"maximagesize\";s:1:\"0\";s:14:\"allowmediacode\";s:1:\"0\";s:14:\"allowbegincode\";s:1:\"0\";s:7:\"allowat\";s:1:\"0\";s:9:\"allowsave\";s:1:\"1\";s:14:\"allowsavereply\";s:1:\"1\";s:12:\"allowsavenum\";s:1:\"0\";s:19:\"allowsetpublishdate\";s:1:\"0\";s:21:\"allowfollowcollection\";s:2:\"30\";s:22:\"allowcommentcollection\";s:1:\"1\";s:21:\"allowcreatecollection\";s:1:\"5\";s:12:\"forcesecques\";s:1:\"0\";s:10:\"forcelogin\";s:1:\"0\";s:7:\"closead\";s:1:\"0\";s:17:\"buildgroupcredits\";s:1:\"0\";s:15:\"allowimgcontent\";s:1:\"0\";s:17:\"allowavatarupload\";s:1:\"1\";s:16:\"allowviewprofile\";s:1:\"1\";s:6:\"fields\";a:0:{}s:9:\"grouptype\";s:6:\"member\";s:11:\"grouppublic\";b:0;s:18:\"groupcreditshigher\";s:4:\"1000\";s:17:\"groupcreditslower\";s:4:\"3000\";s:17:\"allowthreadplugin\";a:0:{}s:6:\"plugin\";N;s:5:\"style\";N;}'),
('usergroup_15',1,1784453715,'a:137:{s:7:\"groupid\";s:2:\"15\";s:8:\"radminid\";s:1:\"0\";s:10:\"grouptitle\";s:12:\"璁哄潧鍏冭€乗";s:5:\"stars\";s:1:\"8\";s:5:\"color\";s:0:\"\";s:4:\"icon\";s:0:\"\";s:10:\"allowvisit\";s:1:\"1\";s:11:\"allowsendpm\";s:1:\"1\";s:11:\"allowinvite\";s:1:\"0\";s:15:\"allowmailinvite\";s:1:\"0\";s:11:\"allowfollow\";s:1:\"1\";s:12:\"maxinvitenum\";s:1:\"0\";s:11:\"inviteprice\";s:1:\"0\";s:12:\"maxinviteday\";s:2:\"10\";s:9:\"upgroupid\";s:1:\"0\";s:14:\"creditsformula\";s:0:\"\";s:10:\"readaccess\";s:2:\"90\";s:9:\"allowpost\";s:1:\"1\";s:10:\"allowreply\";s:1:\"1\";s:13:\"allowpostpoll\";s:1:\"1\";s:15:\"allowpostreward\";s:1:\"1\";s:14:\"allowposttrade\";s:1:\"1\";s:17:\"allowpostactivity\";s:1:\"1\";s:15:\"allowdirectpost\";s:1:\"0\";s:14:\"allowgetattach\";s:1:\"1\";s:13:\"allowgetimage\";s:1:\"1\";s:15:\"allowpostattach\";s:1:\"1\";s:14:\"allowpostimage\";s:1:\"1\";s:9:\"allowvote\";s:1:\"1\";s:11:\"allowsearch\";s:2:\"95\";s:12:\"allowcstatus\";s:1:\"1\";s:14:\"allowinvisible\";s:1:\"1\";s:13:\"allowtransfer\";s:1:\"0\";s:16:\"allowsetreadperm\";s:1:\"1\";s:18:\"allowsetattachperm\";s:1:\"1\";s:12:\"allowposttag\";s:1:\"0\";s:13:\"allowhidecode\";s:1:\"0\";s:9:\"allowhtml\";s:1:\"0\";s:14:\"allowanonymous\";s:1:\"1\";s:14:\"allowsigbbcode\";s:1:\"1\";s:15:\"allowsigimgcode\";s:1:\"1\";s:11:\"allowmagics\";s:1:\"2\";s:17:\"disableperiodctrl\";s:1:\"0\";s:8:\"reasonpm\";s:1:\"0\";s:8:\"maxprice\";s:1:\"0\";s:10:\"maxsigsize\";s:3:\"500\";s:13:\"maxattachsize\";s:7:\"2048000\";s:13:\"maxsizeperday\";s:1:\"0\";s:17:\"maxthreadsperhour\";s:1:\"0\";s:15:\"maxpostsperhour\";s:1:\"0\";s:16:\"attachextensions\";s:55:\"chm, pdf, zip, rar, tar, gz, bzip2, gif, jpg, jpeg, png\";s:9:\"raterange\";a:0:{}s:11:\"loginreward\";s:0:\"\";s:13:\"mintradeprice\";s:1:\"1\";s:13:\"maxtradeprice\";s:1:\"0\";s:14:\"minrewardprice\";s:1:\"1\";s:14:\"maxrewardprice\";s:1:\"0\";s:14:\"magicsdiscount\";s:1:\"0\";s:15:\"maxmagicsweight\";s:3:\"140\";s:15:\"allowpostdebate\";s:1:\"1\";s:10:\"tradestick\";s:1:\"5\";s:6:\"exempt\";s:1:\"0\";s:12:\"maxattachnum\";s:1:\"0\";s:12:\"allowposturl\";s:1:\"3\";s:14:\"allowrecommend\";s:1:\"1\";s:18:\"allowpostrushreply\";s:1:\"0\";s:12:\"maxfriendnum\";s:1:\"0\";s:12:\"maxspacesize\";s:1:\"0\";s:12:\"allowcomment\";s:1:\"1\";s:15:\"allowcommentmod\";s:1:\"0\";s:19:\"allowcommentarticle\";s:4:\"1000\";s:22:\"allowcommentarticlemod\";s:1:\"0\";s:14:\"searchinterval\";s:1:\"0\";s:12:\"searchignore\";s:1:\"0\";s:9:\"allowblog\";s:1:\"1\";s:10:\"allowdoing\";s:1:\"1\";s:11:\"allowupload\";s:1:\"1\";s:10:\"allowshare\";s:1:\"1\";s:12:\"allowblogmod\";s:1:\"0\";s:13:\"allowdoingmod\";s:1:\"0\";s:14:\"allowuploadmod\";s:1:\"0\";s:13:\"allowsharemod\";s:1:\"0\";s:8:\"allowcss\";s:1:\"0\";s:9:\"allowpoke\";s:1:\"1\";s:11:\"allowfriend\";s:1:\"1\";s:10:\"allowclick\";s:1:\"1\";s:10:\"allowmagic\";s:1:\"0\";s:9:\"allowstat\";s:1:\"0\";s:13:\"allowstatdata\";s:1:\"0\";s:13:\"magicdiscount\";s:1:\"0\";s:12:\"domainlength\";s:1:\"5\";s:7:\"seccode\";s:1:\"1\";s:15:\"disablepostctrl\";s:1:\"0\";s:15:\"allowbuildgroup\";s:2:\"10\";s:20:\"allowgroupdirectpost\";s:1:\"3\";s:17:\"allowgroupposturl\";s:1:\"0\";s:13:\"edittimelimit\";s:1:\"0\";s:16:\"allowpostarticle\";s:1:\"0\";s:17:\"allowdownlocalimg\";s:1:\"0\";s:18:\"allowdownremoteimg\";s:1:\"0\";s:19:\"allowpostarticlemod\";s:1:\"0\";s:17:\"allowspacediyhtml\";s:1:\"0\";s:19:\"allowspacediybbcode\";s:1:\"1\";s:20:\"allowspacediyimgcode\";s:1:\"1\";s:16:\"allowcommentpost\";s:1:\"2\";s:16:\"allowcommentitem\";s:1:\"1\";s:17:\"allowcommentreply\";s:1:\"0\";s:16:\"allowreplycredit\";s:1:\"1\";s:12:\"ignorecensor\";s:1:\"0\";s:14:\"allowsendallpm\";s:1:\"0\";s:17:\"allowsendpmmaxnum\";s:1:\"0\";s:12:\"maximagesize\";s:1:\"0\";s:14:\"allowmediacode\";s:1:\"0\";s:14:\"allowbegincode\";s:1:\"0\";s:7:\"allowat\";s:1:\"0\";s:9:\"allowsave\";s:1:\"1\";s:14:\"allowsavereply\";s:1:\"1\";s:12:\"allowsavenum\";s:1:\"0\";s:19:\"allowsetpublishdate\";s:1:\"0\";s:21:\"allowfollowcollection\";s:2:\"30\";s:22:\"allowcommentcollection\";s:1:\"1\";s:21:\"allowcreatecollection\";s:1:\"5\";s:12:\"forcesecques\";s:1:\"0\";s:10:\"forcelogin\";s:1:\"0\";s:7:\"closead\";s:1:\"0\";s:17:\"buildgroupcredits\";s:1:\"0\";s:15:\"allowimgcontent\";s:1:\"0\";s:17:\"allowavatarupload\";s:1:\"1\";s:16:\"allowviewprofile\";s:1:\"1\";s:6:\"fields\";a:0:{}s:9:\"grouptype\";s:6:\"member\";s:11:\"grouppublic\";b:0;s:18:\"groupcreditshigher\";s:4:\"3000\";s:17:\"groupcreditslower\";s:7:\"9999999\";s:17:\"allowthreadplugin\";a:0:{}s:6:\"plugin\";N;s:5:\"style\";N;}'),
('usergroup_16',1,1784453715,'a:197:{s:7:\"groupid\";s:2:\"16\";s:8:\"radminid\";s:1:\"3\";s:10:\"grouptitle\";s:12:\"瀹炰範鐗堜富\";s:5:\"stars\";s:1:\"7\";s:5:\"color\";s:0:\"\";s:4:\"icon\";s:0:\"\";s:10:\"allowvisit\";s:1:\"1\";s:11:\"allowsendpm\";s:1:\"1\";s:11:\"allowinvite\";s:1:\"0\";s:15:\"allowmailinvite\";s:1:\"0\";s:11:\"allowfollow\";s:1:\"1\";s:12:\"maxinvitenum\";s:1:\"0\";s:11:\"inviteprice\";s:1:\"0\";s:12:\"maxinviteday\";s:2:\"10\";s:9:\"upgroupid\";s:1:\"0\";s:14:\"creditsformula\";s:0:\"\";s:10:\"readaccess\";s:3:\"100\";s:9:\"allowpost\";s:1:\"1\";s:10:\"allowreply\";s:1:\"1\";s:13:\"allowpostpoll\";s:1:\"1\";s:15:\"allowpostreward\";s:1:\"1\";s:14:\"allowposttrade\";s:1:\"1\";s:17:\"allowpostactivity\";s:1:\"1\";s:15:\"allowdirectpost\";s:1:\"1\";s:14:\"allowgetattach\";s:1:\"1\";s:13:\"allowgetimage\";s:1:\"1\";s:15:\"allowpostattach\";s:1:\"1\";s:14:\"allowpostimage\";s:1:\"1\";s:9:\"allowvote\";s:1:\"1\";s:11:\"allowsearch\";s:2:\"95\";s:12:\"allowcstatus\";s:1:\"1\";s:14:\"allowinvisible\";s:1:\"0\";s:13:\"allowtransfer\";s:1:\"1\";s:16:\"allowsetreadperm\";s:1:\"1\";s:18:\"allowsetattachperm\";s:1:\"1\";s:12:\"allowposttag\";s:1:\"0\";s:13:\"allowhidecode\";s:1:\"1\";s:9:\"allowhtml\";s:1:\"0\";s:14:\"allowanonymous\";s:1:\"0\";s:14:\"allowsigbbcode\";s:1:\"1\";s:15:\"allowsigimgcode\";s:1:\"1\";s:11:\"allowmagics\";s:1:\"2\";s:17:\"disableperiodctrl\";s:1:\"1\";s:8:\"reasonpm\";s:1:\"0\";s:8:\"maxprice\";s:2:\"10\";s:10:\"maxsigsize\";s:3:\"200\";s:13:\"maxattachsize\";s:7:\"2048000\";s:13:\"maxsizeperday\";s:1:\"0\";s:17:\"maxthreadsperhour\";s:1:\"0\";s:15:\"maxpostsperhour\";s:1:\"0\";s:16:\"attachextensions\";s:55:\"chm, pdf, zip, rar, tar, gz, bzip2, gif, jpg, jpeg, png\";s:9:\"raterange\";a:0:{}s:11:\"loginreward\";s:0:\"\";s:13:\"mintradeprice\";s:1:\"1\";s:13:\"maxtradeprice\";s:1:\"0\";s:14:\"minrewardprice\";s:1:\"1\";s:14:\"maxrewardprice\";s:1:\"0\";s:14:\"magicsdiscount\";s:1:\"0\";s:15:\"maxmagicsweight\";s:3:\"160\";s:15:\"allowpostdebate\";s:1:\"1\";s:10:\"tradestick\";s:1:\"5\";s:6:\"exempt\";s:3:\"188\";s:12:\"maxattachnum\";s:1:\"0\";s:12:\"allowposturl\";s:1:\"3\";s:14:\"allowrecommend\";s:1:\"0\";s:18:\"allowpostrushreply\";s:1:\"0\";s:12:\"maxfriendnum\";s:1:\"0\";s:12:\"maxspacesize\";s:1:\"0\";s:12:\"allowcomment\";s:1:\"1\";s:15:\"allowcommentmod\";s:1:\"0\";s:19:\"allowcommentarticle\";s:4:\"1000\";s:22:\"allowcommentarticlemod\";s:1:\"0\";s:14:\"searchinterval\";s:1:\"0\";s:12:\"searchignore\";s:1:\"0\";s:9:\"allowblog\";s:1:\"1\";s:10:\"allowdoing\";s:1:\"1\";s:11:\"allowupload\";s:1:\"1\";s:10:\"allowshare\";s:1:\"1\";s:12:\"allowblogmod\";s:1:\"0\";s:13:\"allowdoingmod\";s:1:\"0\";s:14:\"allowuploadmod\";s:1:\"0\";s:13:\"allowsharemod\";s:1:\"0\";s:8:\"allowcss\";s:1:\"0\";s:9:\"allowpoke\";s:1:\"1\";s:11:\"allowfriend\";s:1:\"1\";s:10:\"allowclick\";s:1:\"1\";s:10:\"allowmagic\";s:1:\"0\";s:9:\"allowstat\";s:1:\"0\";s:13:\"allowstatdata\";s:1:\"0\";s:13:\"magicdiscount\";s:1:\"0\";s:12:\"domainlength\";s:1:\"5\";s:7:\"seccode\";s:1:\"1\";s:15:\"disablepostctrl\";s:1:\"0\";s:15:\"allowbuildgroup\";s:2:\"15\";s:20:\"allowgroupdirectpost\";s:1:\"3\";s:17:\"allowgroupposturl\";s:1:\"0\";s:13:\"edittimelimit\";s:1:\"0\";s:16:\"allowpostarticle\";s:1:\"0\";s:17:\"allowdownlocalimg\";s:1:\"0\";s:18:\"allowdownremoteimg\";s:1:\"0\";s:19:\"allowpostarticlemod\";s:1:\"0\";s:17:\"allowspacediyhtml\";s:1:\"0\";s:19:\"allowspacediybbcode\";s:1:\"1\";s:20:\"allowspacediyimgcode\";s:1:\"1\";s:16:\"allowcommentpost\";s:1:\"2\";s:16:\"allowcommentitem\";s:1:\"1\";s:17:\"allowcommentreply\";s:1:\"0\";s:16:\"allowreplycredit\";s:1:\"1\";s:12:\"ignorecensor\";s:1:\"0\";s:14:\"allowsendallpm\";s:1:\"0\";s:17:\"allowsendpmmaxnum\";s:1:\"0\";s:12:\"maximagesize\";s:1:\"0\";s:14:\"allowmediacode\";s:1:\"0\";s:14:\"allowbegincode\";s:1:\"0\";s:7:\"allowat\";s:1:\"0\";s:9:\"allowsave\";s:1:\"1\";s:14:\"allowsavereply\";s:1:\"1\";s:12:\"allowsavenum\";s:1:\"0\";s:19:\"allowsetpublishdate\";s:1:\"0\";s:21:\"allowfollowcollection\";s:2:\"30\";s:22:\"allowcommentcollection\";s:1:\"1\";s:21:\"allowcreatecollection\";s:1:\"5\";s:12:\"forcesecques\";s:1:\"0\";s:10:\"forcelogin\";s:1:\"0\";s:7:\"closead\";s:1:\"0\";s:17:\"buildgroupcredits\";s:1:\"0\";s:15:\"allowimgcontent\";s:1:\"0\";s:17:\"allowavatarupload\";s:1:\"1\";s:16:\"allowviewprofile\";s:1:\"1\";s:6:\"fields\";a:0:{}s:13:\"alloweditpost\";s:1:\"0\";s:13:\"alloweditpoll\";s:1:\"0\";s:16:\"allowstickthread\";s:1:\"1\";s:12:\"allowmodpost\";s:1:\"0\";s:12:\"allowdelpost\";s:1:\"0\";s:14:\"allowmassprune\";s:1:\"0\";s:11:\"allowrefund\";s:1:\"0\";s:15:\"allowcensorword\";s:1:\"0\";s:11:\"allowviewip\";s:1:\"0\";s:10:\"allowbanip\";s:1:\"0\";s:13:\"allowedituser\";s:1:\"0\";s:12:\"allowmoduser\";s:1:\"0\";s:12:\"allowbanuser\";s:1:\"0\";s:17:\"allowbanvisituser\";s:1:\"0\";s:17:\"allowpostannounce\";s:1:\"0\";s:12:\"allowviewlog\";s:1:\"0\";s:12:\"allowbanpost\";s:1:\"1\";s:20:\"supe_allowpushthread\";s:1:\"0\";s:20:\"allowhighlightthread\";s:1:\"1\";s:15:\"allowlivethread\";s:1:\"0\";s:17:\"allowdigestthread\";s:1:\"1\";s:20:\"allowrecommendthread\";s:1:\"1\";s:15:\"allowbumpthread\";s:1:\"1\";s:16:\"allowclosethread\";s:1:\"1\";s:15:\"allowmovethread\";s:1:\"0\";s:19:\"allowedittypethread\";s:1:\"0\";s:16:\"allowstampthread\";s:1:\"1\";s:14:\"allowstamplist\";s:1:\"0\";s:15:\"allowcopythread\";s:1:\"0\";s:16:\"allowmergethread\";s:1:\"0\";s:16:\"allowsplitthread\";s:1:\"0\";s:17:\"allowrepairthread\";s:1:\"0\";s:13:\"allowwarnpost\";s:1:\"1\";s:15:\"allowviewreport\";s:1:\"1\";s:14:\"alloweditforum\";s:1:\"0\";s:17:\"allowremovereward\";s:1:\"0\";s:14:\"allowedittrade\";s:1:\"0\";s:17:\"alloweditactivity\";s:1:\"0\";s:15:\"allowstickreply\";s:1:\"0\";s:18:\"allowmanagearticle\";s:1:\"0\";s:13:\"allowaddtopic\";s:1:\"0\";s:16:\"allowmanagetopic\";s:1:\"0\";s:8:\"allowdiy\";s:1:\"0\";s:17:\"allowclearrecycle\";s:1:\"0\";s:14:\"allowmanagetag\";s:1:\"0\";s:16:\"alloweditusertag\";s:1:\"0\";s:10:\"managefeed\";s:1:\"0\";s:11:\"managedoing\";s:1:\"0\";s:11:\"manageshare\";s:1:\"0\";s:10:\"manageblog\";s:1:\"0\";s:11:\"managealbum\";s:1:\"0\";s:13:\"managecomment\";s:1:\"0\";s:14:\"managemagiclog\";s:1:\"0\";s:12:\"managereport\";s:1:\"0\";s:13:\"managehotuser\";s:1:\"0\";s:17:\"managedefaultuser\";s:1:\"0\";s:11:\"managemagic\";s:1:\"0\";s:11:\"manageclick\";s:1:\"0\";s:21:\"allowmanagecollection\";s:1:\"0\";s:13:\"allowmakehtml\";s:1:\"0\";s:9:\"grouptype\";s:7:\"special\";s:11:\"grouppublic\";b:0;s:18:\"groupcreditshigher\";s:1:\"0\";s:17:\"groupcreditslower\";s:1:\"0\";s:17:\"allowthreadplugin\";a:0:{}s:6:\"plugin\";N;s:5:\"style\";N;}'),
('usergroup_17',1,1784453715,'a:197:{s:7:\"groupid\";s:2:\"17\";s:8:\"radminid\";s:1:\"2\";s:10:\"grouptitle\";s:12:\"缃戠珯缂栬緫\";s:5:\"stars\";s:1:\"8\";s:5:\"color\";s:0:\"\";s:4:\"icon\";s:0:\"\";s:10:\"allowvisit\";s:1:\"1\";s:11:\"allowsendpm\";s:1:\"1\";s:11:\"allowinvite\";s:1:\"0\";s:15:\"allowmailinvite\";s:1:\"0\";s:11:\"allowfollow\";s:1:\"1\";s:12:\"maxinvitenum\";s:1:\"0\";s:11:\"inviteprice\";s:1:\"0\";s:12:\"maxinviteday\";s:2:\"10\";s:9:\"upgroupid\";s:1:\"0\";s:14:\"creditsformula\";s:0:\"\";s:10:\"readaccess\";s:3:\"150\";s:9:\"allowpost\";s:1:\"1\";s:10:\"allowreply\";s:1:\"1\";s:13:\"allowpostpoll\";s:1:\"1\";s:15:\"allowpostreward\";s:1:\"1\";s:14:\"allowposttrade\";s:1:\"1\";s:17:\"allowpostactivity\";s:1:\"1\";s:15:\"allowdirectpost\";s:1:\"3\";s:14:\"allowgetattach\";s:1:\"1\";s:13:\"allowgetimage\";s:1:\"1\";s:15:\"allowpostattach\";s:1:\"1\";s:14:\"allowpostimage\";s:1:\"1\";s:9:\"allowvote\";s:1:\"1\";s:11:\"allowsearch\";s:2:\"95\";s:12:\"allowcstatus\";s:1:\"1\";s:14:\"allowinvisible\";s:1:\"1\";s:13:\"allowtransfer\";s:1:\"1\";s:16:\"allowsetreadperm\";s:1:\"1\";s:18:\"allowsetattachperm\";s:1:\"1\";s:12:\"allowposttag\";s:1:\"0\";s:13:\"allowhidecode\";s:1:\"1\";s:9:\"allowhtml\";s:1:\"0\";s:14:\"allowanonymous\";s:1:\"0\";s:14:\"allowsigbbcode\";s:1:\"1\";s:15:\"allowsigimgcode\";s:1:\"1\";s:11:\"allowmagics\";s:1:\"2\";s:17:\"disableperiodctrl\";s:1:\"1\";s:8:\"reasonpm\";s:1:\"0\";s:8:\"maxprice\";s:2:\"20\";s:10:\"maxsigsize\";s:3:\"300\";s:13:\"maxattachsize\";s:7:\"2048000\";s:13:\"maxsizeperday\";s:1:\"0\";s:17:\"maxthreadsperhour\";s:1:\"0\";s:15:\"maxpostsperhour\";s:1:\"0\";s:16:\"attachextensions\";s:55:\"chm, pdf, zip, rar, tar, gz, bzip2, gif, jpg, jpeg, png\";s:9:\"raterange\";a:0:{}s:11:\"loginreward\";s:0:\"\";s:13:\"mintradeprice\";s:1:\"1\";s:13:\"maxtradeprice\";s:1:\"0\";s:14:\"minrewardprice\";s:1:\"1\";s:14:\"maxrewardprice\";s:1:\"0\";s:14:\"magicsdiscount\";s:1:\"0\";s:15:\"maxmagicsweight\";s:3:\"180\";s:15:\"allowpostdebate\";s:1:\"1\";s:10:\"tradestick\";s:1:\"5\";s:6:\"exempt\";s:3:\"255\";s:12:\"maxattachnum\";s:1:\"0\";s:12:\"allowposturl\";s:1:\"3\";s:14:\"allowrecommend\";s:1:\"0\";s:18:\"allowpostrushreply\";s:1:\"0\";s:12:\"maxfriendnum\";s:1:\"0\";s:12:\"maxspacesize\";s:1:\"0\";s:12:\"allowcomment\";s:1:\"1\";s:15:\"allowcommentmod\";s:1:\"0\";s:19:\"allowcommentarticle\";s:4:\"1000\";s:22:\"allowcommentarticlemod\";s:1:\"0\";s:14:\"searchinterval\";s:1:\"0\";s:12:\"searchignore\";s:1:\"0\";s:9:\"allowblog\";s:1:\"1\";s:10:\"allowdoing\";s:1:\"1\";s:11:\"allowupload\";s:1:\"1\";s:10:\"allowshare\";s:1:\"1\";s:12:\"allowblogmod\";s:1:\"0\";s:13:\"allowdoingmod\";s:1:\"0\";s:14:\"allowuploadmod\";s:1:\"0\";s:13:\"allowsharemod\";s:1:\"0\";s:8:\"allowcss\";s:1:\"0\";s:9:\"allowpoke\";s:1:\"1\";s:11:\"allowfriend\";s:1:\"1\";s:10:\"allowclick\";s:1:\"1\";s:10:\"allowmagic\";s:1:\"0\";s:9:\"allowstat\";s:1:\"0\";s:13:\"allowstatdata\";s:1:\"0\";s:13:\"magicdiscount\";s:1:\"0\";s:12:\"domainlength\";s:1:\"5\";s:7:\"seccode\";s:1:\"1\";s:15:\"disablepostctrl\";s:1:\"1\";s:15:\"allowbuildgroup\";s:2:\"15\";s:20:\"allowgroupdirectpost\";s:1:\"3\";s:17:\"allowgroupposturl\";s:1:\"0\";s:13:\"edittimelimit\";s:1:\"0\";s:16:\"allowpostarticle\";s:1:\"0\";s:17:\"allowdownlocalimg\";s:1:\"0\";s:18:\"allowdownremoteimg\";s:1:\"0\";s:19:\"allowpostarticlemod\";s:1:\"0\";s:17:\"allowspacediyhtml\";s:1:\"0\";s:19:\"allowspacediybbcode\";s:1:\"1\";s:20:\"allowspacediyimgcode\";s:1:\"1\";s:16:\"allowcommentpost\";s:1:\"2\";s:16:\"allowcommentitem\";s:1:\"1\";s:17:\"allowcommentreply\";s:1:\"0\";s:16:\"allowreplycredit\";s:1:\"1\";s:12:\"ignorecensor\";s:1:\"0\";s:14:\"allowsendallpm\";s:1:\"0\";s:17:\"allowsendpmmaxnum\";s:1:\"0\";s:12:\"maximagesize\";s:1:\"0\";s:14:\"allowmediacode\";s:1:\"0\";s:14:\"allowbegincode\";s:1:\"0\";s:7:\"allowat\";s:1:\"0\";s:9:\"allowsave\";s:1:\"1\";s:14:\"allowsavereply\";s:1:\"1\";s:12:\"allowsavenum\";s:1:\"0\";s:19:\"allowsetpublishdate\";s:1:\"0\";s:21:\"allowfollowcollection\";s:2:\"30\";s:22:\"allowcommentcollection\";s:1:\"1\";s:21:\"allowcreatecollection\";s:1:\"5\";s:12:\"forcesecques\";s:1:\"0\";s:10:\"forcelogin\";s:1:\"0\";s:7:\"closead\";s:1:\"0\";s:17:\"buildgroupcredits\";s:1:\"0\";s:15:\"allowimgcontent\";s:1:\"0\";s:17:\"allowavatarupload\";s:1:\"1\";s:16:\"allowviewprofile\";s:1:\"1\";s:6:\"fields\";a:0:{}s:13:\"alloweditpost\";s:1:\"1\";s:13:\"alloweditpoll\";s:1:\"0\";s:16:\"allowstickthread\";s:1:\"2\";s:12:\"allowmodpost\";s:1:\"1\";s:12:\"allowdelpost\";s:1:\"0\";s:14:\"allowmassprune\";s:1:\"0\";s:11:\"allowrefund\";s:1:\"1\";s:15:\"allowcensorword\";s:1:\"0\";s:11:\"allowviewip\";s:1:\"1\";s:10:\"allowbanip\";s:1:\"0\";s:13:\"allowedituser\";s:1:\"0\";s:12:\"allowmoduser\";s:1:\"0\";s:12:\"allowbanuser\";s:1:\"0\";s:17:\"allowbanvisituser\";s:1:\"0\";s:17:\"allowpostannounce\";s:1:\"1\";s:12:\"allowviewlog\";s:1:\"1\";s:12:\"allowbanpost\";s:1:\"1\";s:20:\"supe_allowpushthread\";s:1:\"0\";s:20:\"allowhighlightthread\";s:1:\"1\";s:15:\"allowlivethread\";s:1:\"0\";s:17:\"allowdigestthread\";s:1:\"3\";s:20:\"allowrecommendthread\";s:1:\"1\";s:15:\"allowbumpthread\";s:1:\"1\";s:16:\"allowclosethread\";s:1:\"1\";s:15:\"allowmovethread\";s:1:\"1\";s:19:\"allowedittypethread\";s:1:\"1\";s:16:\"allowstampthread\";s:1:\"1\";s:14:\"allowstamplist\";s:1:\"0\";s:15:\"allowcopythread\";s:1:\"1\";s:16:\"allowmergethread\";s:1:\"1\";s:16:\"allowsplitthread\";s:1:\"1\";s:17:\"allowrepairthread\";s:1:\"1\";s:13:\"allowwarnpost\";s:1:\"1\";s:15:\"allowviewreport\";s:1:\"1\";s:14:\"alloweditforum\";s:1:\"1\";s:17:\"allowremovereward\";s:1:\"1\";s:14:\"allowedittrade\";s:1:\"0\";s:17:\"alloweditactivity\";s:1:\"0\";s:15:\"allowstickreply\";s:1:\"0\";s:18:\"allowmanagearticle\";s:1:\"0\";s:13:\"allowaddtopic\";s:1:\"0\";s:16:\"allowmanagetopic\";s:1:\"0\";s:8:\"allowdiy\";s:1:\"0\";s:17:\"allowclearrecycle\";s:1:\"0\";s:14:\"allowmanagetag\";s:1:\"0\";s:16:\"alloweditusertag\";s:1:\"0\";s:10:\"managefeed\";s:1:\"0\";s:11:\"managedoing\";s:1:\"0\";s:11:\"manageshare\";s:1:\"0\";s:10:\"manageblog\";s:1:\"0\";s:11:\"managealbum\";s:1:\"0\";s:13:\"managecomment\";s:1:\"0\";s:14:\"managemagiclog\";s:1:\"0\";s:12:\"managereport\";s:1:\"0\";s:13:\"managehotuser\";s:1:\"0\";s:17:\"managedefaultuser\";s:1:\"0\";s:11:\"managemagic\";s:1:\"0\";s:11:\"manageclick\";s:1:\"0\";s:21:\"allowmanagecollection\";s:1:\"0\";s:13:\"allowmakehtml\";s:1:\"0\";s:9:\"grouptype\";s:7:\"special\";s:11:\"grouppublic\";b:0;s:18:\"groupcreditshigher\";s:1:\"0\";s:17:\"groupcreditslower\";s:1:\"0\";s:17:\"allowthreadplugin\";a:0:{}s:6:\"plugin\";N;s:5:\"style\";N;}'),
('usergroup_18',1,1784453715,'a:197:{s:7:\"groupid\";s:2:\"18\";s:8:\"radminid\";s:1:\"1\";s:10:\"grouptitle\";s:15:\"淇℃伅鐩戝療鍛榎";s:5:\"stars\";s:1:\"9\";s:5:\"color\";s:0:\"\";s:4:\"icon\";s:0:\"\";s:10:\"allowvisit\";s:1:\"1\";s:11:\"allowsendpm\";s:1:\"1\";s:11:\"allowinvite\";s:1:\"0\";s:15:\"allowmailinvite\";s:1:\"0\";s:11:\"allowfollow\";s:1:\"1\";s:12:\"maxinvitenum\";s:1:\"0\";s:11:\"inviteprice\";s:1:\"0\";s:12:\"maxinviteday\";s:2:\"10\";s:9:\"upgroupid\";s:1:\"0\";s:14:\"creditsformula\";s:0:\"\";s:10:\"readaccess\";s:3:\"200\";s:9:\"allowpost\";s:1:\"1\";s:10:\"allowreply\";s:1:\"1\";s:13:\"allowpostpoll\";s:1:\"1\";s:15:\"allowpostreward\";s:1:\"1\";s:14:\"allowposttrade\";s:1:\"1\";s:17:\"allowpostactivity\";s:1:\"1\";s:15:\"allowdirectpost\";s:1:\"3\";s:14:\"allowgetattach\";s:1:\"1\";s:13:\"allowgetimage\";s:1:\"1\";s:15:\"allowpostattach\";s:1:\"1\";s:14:\"allowpostimage\";s:1:\"1\";s:9:\"allowvote\";s:1:\"1\";s:11:\"allowsearch\";s:2:\"95\";s:12:\"allowcstatus\";s:1:\"0\";s:14:\"allowinvisible\";s:1:\"1\";s:13:\"allowtransfer\";s:1:\"1\";s:16:\"allowsetreadperm\";s:1:\"1\";s:18:\"allowsetattachperm\";s:1:\"1\";s:12:\"allowposttag\";s:1:\"0\";s:13:\"allowhidecode\";s:1:\"1\";s:9:\"allowhtml\";s:1:\"1\";s:14:\"allowanonymous\";s:1:\"1\";s:14:\"allowsigbbcode\";s:1:\"1\";s:15:\"allowsigimgcode\";s:1:\"1\";s:11:\"allowmagics\";s:1:\"2\";s:17:\"disableperiodctrl\";s:1:\"0\";s:8:\"reasonpm\";s:1:\"0\";s:8:\"maxprice\";s:2:\"30\";s:10:\"maxsigsize\";s:3:\"500\";s:13:\"maxattachsize\";s:1:\"0\";s:13:\"maxsizeperday\";s:1:\"0\";s:17:\"maxthreadsperhour\";s:1:\"0\";s:15:\"maxpostsperhour\";s:1:\"1\";s:16:\"attachextensions\";s:0:\"\";s:9:\"raterange\";a:0:{}s:11:\"loginreward\";s:0:\"\";s:13:\"mintradeprice\";s:1:\"1\";s:13:\"maxtradeprice\";s:1:\"0\";s:14:\"minrewardprice\";s:1:\"1\";s:14:\"maxrewardprice\";s:1:\"0\";s:14:\"magicsdiscount\";s:1:\"0\";s:15:\"maxmagicsweight\";s:3:\"200\";s:15:\"allowpostdebate\";s:1:\"1\";s:10:\"tradestick\";s:1:\"5\";s:6:\"exempt\";s:3:\"255\";s:12:\"maxattachnum\";s:1:\"0\";s:12:\"allowposturl\";s:1:\"3\";s:14:\"allowrecommend\";s:1:\"3\";s:18:\"allowpostrushreply\";s:1:\"0\";s:12:\"maxfriendnum\";s:1:\"0\";s:12:\"maxspacesize\";s:1:\"0\";s:12:\"allowcomment\";s:1:\"1\";s:15:\"allowcommentmod\";s:1:\"0\";s:19:\"allowcommentarticle\";s:4:\"1000\";s:22:\"allowcommentarticlemod\";s:1:\"0\";s:14:\"searchinterval\";s:1:\"0\";s:12:\"searchignore\";s:1:\"0\";s:9:\"allowblog\";s:1:\"1\";s:10:\"allowdoing\";s:1:\"1\";s:11:\"allowupload\";s:1:\"1\";s:10:\"allowshare\";s:1:\"1\";s:12:\"allowblogmod\";s:1:\"0\";s:13:\"allowdoingmod\";s:1:\"0\";s:14:\"allowuploadmod\";s:1:\"0\";s:13:\"allowsharemod\";s:1:\"0\";s:8:\"allowcss\";s:1:\"0\";s:9:\"allowpoke\";s:1:\"1\";s:11:\"allowfriend\";s:1:\"1\";s:10:\"allowclick\";s:1:\"1\";s:10:\"allowmagic\";s:1:\"0\";s:9:\"allowstat\";s:1:\"0\";s:13:\"allowstatdata\";s:1:\"0\";s:13:\"magicdiscount\";s:1:\"0\";s:12:\"domainlength\";s:2:\"15\";s:7:\"seccode\";s:1:\"1\";s:15:\"disablepostctrl\";s:1:\"1\";s:15:\"allowbuildgroup\";s:1:\"5\";s:20:\"allowgroupdirectpost\";s:1:\"3\";s:17:\"allowgroupposturl\";s:1:\"0\";s:13:\"edittimelimit\";s:1:\"0\";s:16:\"allowpostarticle\";s:1:\"0\";s:17:\"allowdownlocalimg\";s:1:\"0\";s:18:\"allowdownremoteimg\";s:1:\"0\";s:19:\"allowpostarticlemod\";s:1:\"0\";s:17:\"allowspacediyhtml\";s:1:\"0\";s:19:\"allowspacediybbcode\";s:1:\"1\";s:20:\"allowspacediyimgcode\";s:1:\"1\";s:16:\"allowcommentpost\";s:1:\"2\";s:16:\"allowcommentitem\";s:1:\"1\";s:17:\"allowcommentreply\";s:1:\"0\";s:16:\"allowreplycredit\";s:1:\"1\";s:12:\"ignorecensor\";s:1:\"0\";s:14:\"allowsendallpm\";s:1:\"0\";s:17:\"allowsendpmmaxnum\";s:1:\"0\";s:12:\"maximagesize\";s:1:\"0\";s:14:\"allowmediacode\";s:1:\"0\";s:14:\"allowbegincode\";s:1:\"0\";s:7:\"allowat\";s:1:\"0\";s:9:\"allowsave\";s:1:\"1\";s:14:\"allowsavereply\";s:1:\"1\";s:12:\"allowsavenum\";s:1:\"0\";s:19:\"allowsetpublishdate\";s:1:\"0\";s:21:\"allowfollowcollection\";s:2:\"30\";s:22:\"allowcommentcollection\";s:1:\"1\";s:21:\"allowcreatecollection\";s:1:\"5\";s:12:\"forcesecques\";s:1:\"0\";s:10:\"forcelogin\";s:1:\"0\";s:7:\"closead\";s:1:\"0\";s:17:\"buildgroupcredits\";s:1:\"0\";s:15:\"allowimgcontent\";s:1:\"0\";s:17:\"allowavatarupload\";s:1:\"1\";s:16:\"allowviewprofile\";s:1:\"1\";s:6:\"fields\";a:0:{}s:13:\"alloweditpost\";s:1:\"0\";s:13:\"alloweditpoll\";s:1:\"0\";s:16:\"allowstickthread\";s:1:\"0\";s:12:\"allowmodpost\";s:1:\"0\";s:12:\"allowdelpost\";s:1:\"0\";s:14:\"allowmassprune\";s:1:\"0\";s:11:\"allowrefund\";s:1:\"0\";s:15:\"allowcensorword\";s:1:\"0\";s:11:\"allowviewip\";s:1:\"1\";s:10:\"allowbanip\";s:1:\"0\";s:13:\"allowedituser\";s:1:\"0\";s:12:\"allowmoduser\";s:1:\"0\";s:12:\"allowbanuser\";s:1:\"0\";s:17:\"allowbanvisituser\";s:1:\"0\";s:17:\"allowpostannounce\";s:1:\"0\";s:12:\"allowviewlog\";s:1:\"0\";s:12:\"allowbanpost\";s:1:\"0\";s:20:\"supe_allowpushthread\";s:1:\"1\";s:20:\"allowhighlightthread\";s:1:\"0\";s:15:\"allowlivethread\";s:1:\"0\";s:17:\"allowdigestthread\";s:1:\"0\";s:20:\"allowrecommendthread\";s:1:\"0\";s:15:\"allowbumpthread\";s:1:\"0\";s:16:\"allowclosethread\";s:1:\"0\";s:15:\"allowmovethread\";s:1:\"0\";s:19:\"allowedittypethread\";s:1:\"0\";s:16:\"allowstampthread\";s:1:\"0\";s:14:\"allowstamplist\";s:1:\"0\";s:15:\"allowcopythread\";s:1:\"0\";s:16:\"allowmergethread\";s:1:\"0\";s:16:\"allowsplitthread\";s:1:\"0\";s:17:\"allowrepairthread\";s:1:\"0\";s:13:\"allowwarnpost\";s:1:\"0\";s:15:\"allowviewreport\";s:1:\"0\";s:14:\"alloweditforum\";s:1:\"0\";s:17:\"allowremovereward\";s:1:\"0\";s:14:\"allowedittrade\";s:1:\"0\";s:17:\"alloweditactivity\";s:1:\"0\";s:15:\"allowstickreply\";s:1:\"0\";s:18:\"allowmanagearticle\";s:1:\"0\";s:13:\"allowaddtopic\";s:1:\"0\";s:16:\"allowmanagetopic\";s:1:\"0\";s:8:\"allowdiy\";s:1:\"0\";s:17:\"allowclearrecycle\";s:1:\"0\";s:14:\"allowmanagetag\";s:1:\"0\";s:16:\"alloweditusertag\";s:1:\"0\";s:10:\"managefeed\";s:1:\"0\";s:11:\"managedoing\";s:1:\"0\";s:11:\"manageshare\";s:1:\"0\";s:10:\"manageblog\";s:1:\"0\";s:11:\"managealbum\";s:1:\"0\";s:13:\"managecomment\";s:1:\"0\";s:14:\"managemagiclog\";s:1:\"0\";s:12:\"managereport\";s:1:\"0\";s:13:\"managehotuser\";s:1:\"0\";s:17:\"managedefaultuser\";s:1:\"0\";s:11:\"managemagic\";s:1:\"0\";s:11:\"manageclick\";s:1:\"0\";s:21:\"allowmanagecollection\";s:1:\"0\";s:13:\"allowmakehtml\";s:1:\"0\";s:9:\"grouptype\";s:7:\"special\";s:11:\"grouppublic\";b:0;s:18:\"groupcreditshigher\";s:1:\"0\";s:17:\"groupcreditslower\";s:1:\"0\";s:17:\"allowthreadplugin\";a:0:{}s:6:\"plugin\";N;s:5:\"style\";N;}'),
('usergroup_19',1,1784453715,'a:197:{s:7:\"groupid\";s:2:\"19\";s:8:\"radminid\";s:1:\"3\";s:10:\"grouptitle\";s:9:\"瀹℃牳鍛榎";s:5:\"stars\";s:1:\"7\";s:5:\"color\";s:0:\"\";s:4:\"icon\";s:0:\"\";s:10:\"allowvisit\";s:1:\"1\";s:11:\"allowsendpm\";s:1:\"1\";s:11:\"allowinvite\";s:1:\"0\";s:15:\"allowmailinvite\";s:1:\"0\";s:11:\"allowfollow\";s:1:\"1\";s:12:\"maxinvitenum\";s:1:\"0\";s:11:\"inviteprice\";s:1:\"0\";s:12:\"maxinviteday\";s:2:\"10\";s:9:\"upgroupid\";s:1:\"0\";s:14:\"creditsformula\";s:0:\"\";s:10:\"readaccess\";s:3:\"100\";s:9:\"allowpost\";s:1:\"1\";s:10:\"allowreply\";s:1:\"1\";s:13:\"allowpostpoll\";s:1:\"1\";s:15:\"allowpostreward\";s:1:\"1\";s:14:\"allowposttrade\";s:1:\"1\";s:17:\"allowpostactivity\";s:1:\"1\";s:15:\"allowdirectpost\";s:1:\"1\";s:14:\"allowgetattach\";s:1:\"1\";s:13:\"allowgetimage\";s:1:\"1\";s:15:\"allowpostattach\";s:1:\"1\";s:14:\"allowpostimage\";s:1:\"1\";s:9:\"allowvote\";s:1:\"1\";s:11:\"allowsearch\";s:2:\"95\";s:12:\"allowcstatus\";s:1:\"1\";s:14:\"allowinvisible\";s:1:\"0\";s:13:\"allowtransfer\";s:1:\"1\";s:16:\"allowsetreadperm\";s:1:\"1\";s:18:\"allowsetattachperm\";s:1:\"1\";s:12:\"allowposttag\";s:1:\"0\";s:13:\"allowhidecode\";s:1:\"1\";s:9:\"allowhtml\";s:1:\"0\";s:14:\"allowanonymous\";s:1:\"0\";s:14:\"allowsigbbcode\";s:1:\"1\";s:15:\"allowsigimgcode\";s:1:\"1\";s:11:\"allowmagics\";s:1:\"2\";s:17:\"disableperiodctrl\";s:1:\"1\";s:8:\"reasonpm\";s:1:\"0\";s:8:\"maxprice\";s:2:\"10\";s:10:\"maxsigsize\";s:3:\"200\";s:13:\"maxattachsize\";s:7:\"2048000\";s:13:\"maxsizeperday\";s:1:\"0\";s:17:\"maxthreadsperhour\";s:1:\"0\";s:15:\"maxpostsperhour\";s:1:\"0\";s:16:\"attachextensions\";s:55:\"chm, pdf, zip, rar, tar, gz, bzip2, gif, jpg, jpeg, png\";s:9:\"raterange\";a:0:{}s:11:\"loginreward\";s:0:\"\";s:13:\"mintradeprice\";s:1:\"1\";s:13:\"maxtradeprice\";s:1:\"0\";s:14:\"minrewardprice\";s:1:\"1\";s:14:\"maxrewardprice\";s:1:\"0\";s:14:\"magicsdiscount\";s:1:\"0\";s:15:\"maxmagicsweight\";s:3:\"160\";s:15:\"allowpostdebate\";s:1:\"1\";s:10:\"tradestick\";s:1:\"5\";s:6:\"exempt\";s:3:\"188\";s:12:\"maxattachnum\";s:1:\"0\";s:12:\"allowposturl\";s:1:\"3\";s:14:\"allowrecommend\";s:1:\"0\";s:18:\"allowpostrushreply\";s:1:\"0\";s:12:\"maxfriendnum\";s:1:\"0\";s:12:\"maxspacesize\";s:1:\"0\";s:12:\"allowcomment\";s:1:\"1\";s:15:\"allowcommentmod\";s:1:\"0\";s:19:\"allowcommentarticle\";s:4:\"1000\";s:22:\"allowcommentarticlemod\";s:1:\"0\";s:14:\"searchinterval\";s:1:\"0\";s:12:\"searchignore\";s:1:\"0\";s:9:\"allowblog\";s:1:\"1\";s:10:\"allowdoing\";s:1:\"1\";s:11:\"allowupload\";s:1:\"1\";s:10:\"allowshare\";s:1:\"1\";s:12:\"allowblogmod\";s:1:\"0\";s:13:\"allowdoingmod\";s:1:\"0\";s:14:\"allowuploadmod\";s:1:\"0\";s:13:\"allowsharemod\";s:1:\"0\";s:8:\"allowcss\";s:1:\"0\";s:9:\"allowpoke\";s:1:\"1\";s:11:\"allowfriend\";s:1:\"1\";s:10:\"allowclick\";s:1:\"1\";s:10:\"allowmagic\";s:1:\"0\";s:9:\"allowstat\";s:1:\"0\";s:13:\"allowstatdata\";s:1:\"0\";s:13:\"magicdiscount\";s:1:\"0\";s:12:\"domainlength\";s:1:\"5\";s:7:\"seccode\";s:1:\"1\";s:15:\"disablepostctrl\";s:1:\"1\";s:15:\"allowbuildgroup\";s:2:\"15\";s:20:\"allowgroupdirectpost\";s:1:\"3\";s:17:\"allowgroupposturl\";s:1:\"0\";s:13:\"edittimelimit\";s:1:\"0\";s:16:\"allowpostarticle\";s:1:\"0\";s:17:\"allowdownlocalimg\";s:1:\"0\";s:18:\"allowdownremoteimg\";s:1:\"0\";s:19:\"allowpostarticlemod\";s:1:\"0\";s:17:\"allowspacediyhtml\";s:1:\"0\";s:19:\"allowspacediybbcode\";s:1:\"1\";s:20:\"allowspacediyimgcode\";s:1:\"1\";s:16:\"allowcommentpost\";s:1:\"2\";s:16:\"allowcommentitem\";s:1:\"1\";s:17:\"allowcommentreply\";s:1:\"0\";s:16:\"allowreplycredit\";s:1:\"1\";s:12:\"ignorecensor\";s:1:\"0\";s:14:\"allowsendallpm\";s:1:\"0\";s:17:\"allowsendpmmaxnum\";s:1:\"0\";s:12:\"maximagesize\";s:1:\"0\";s:14:\"allowmediacode\";s:1:\"0\";s:14:\"allowbegincode\";s:1:\"0\";s:7:\"allowat\";s:1:\"0\";s:9:\"allowsave\";s:1:\"1\";s:14:\"allowsavereply\";s:1:\"1\";s:12:\"allowsavenum\";s:1:\"0\";s:19:\"allowsetpublishdate\";s:1:\"0\";s:21:\"allowfollowcollection\";s:2:\"30\";s:22:\"allowcommentcollection\";s:1:\"1\";s:21:\"allowcreatecollection\";s:1:\"5\";s:12:\"forcesecques\";s:1:\"0\";s:10:\"forcelogin\";s:1:\"0\";s:7:\"closead\";s:1:\"0\";s:17:\"buildgroupcredits\";s:1:\"0\";s:15:\"allowimgcontent\";s:1:\"0\";s:17:\"allowavatarupload\";s:1:\"1\";s:16:\"allowviewprofile\";s:1:\"1\";s:6:\"fields\";a:0:{}s:13:\"alloweditpost\";s:1:\"0\";s:13:\"alloweditpoll\";s:1:\"0\";s:16:\"allowstickthread\";s:1:\"0\";s:12:\"allowmodpost\";s:1:\"1\";s:12:\"allowdelpost\";s:1:\"0\";s:14:\"allowmassprune\";s:1:\"0\";s:11:\"allowrefund\";s:1:\"0\";s:15:\"allowcensorword\";s:1:\"0\";s:11:\"allowviewip\";s:1:\"1\";s:10:\"allowbanip\";s:1:\"1\";s:13:\"allowedituser\";s:1:\"0\";s:12:\"allowmoduser\";s:1:\"1\";s:12:\"allowbanuser\";s:1:\"1\";s:17:\"allowbanvisituser\";s:1:\"0\";s:17:\"allowpostannounce\";s:1:\"0\";s:12:\"allowviewlog\";s:1:\"0\";s:12:\"allowbanpost\";s:1:\"0\";s:20:\"supe_allowpushthread\";s:1:\"0\";s:20:\"allowhighlightthread\";s:1:\"0\";s:15:\"allowlivethread\";s:1:\"0\";s:17:\"allowdigestthread\";s:1:\"0\";s:20:\"allowrecommendthread\";s:1:\"0\";s:15:\"allowbumpthread\";s:1:\"0\";s:16:\"allowclosethread\";s:1:\"0\";s:15:\"allowmovethread\";s:1:\"0\";s:19:\"allowedittypethread\";s:1:\"0\";s:16:\"allowstampthread\";s:1:\"0\";s:14:\"allowstamplist\";s:1:\"0\";s:15:\"allowcopythread\";s:1:\"0\";s:16:\"allowmergethread\";s:1:\"0\";s:16:\"allowsplitthread\";s:1:\"0\";s:17:\"allowrepairthread\";s:1:\"0\";s:13:\"allowwarnpost\";s:1:\"0\";s:15:\"allowviewreport\";s:1:\"0\";s:14:\"alloweditforum\";s:1:\"0\";s:17:\"allowremovereward\";s:1:\"0\";s:14:\"allowedittrade\";s:1:\"0\";s:17:\"alloweditactivity\";s:1:\"0\";s:15:\"allowstickreply\";s:1:\"0\";s:18:\"allowmanagearticle\";s:1:\"0\";s:13:\"allowaddtopic\";s:1:\"0\";s:16:\"allowmanagetopic\";s:1:\"0\";s:8:\"allowdiy\";s:1:\"0\";s:17:\"allowclearrecycle\";s:1:\"0\";s:14:\"allowmanagetag\";s:1:\"0\";s:16:\"alloweditusertag\";s:1:\"0\";s:10:\"managefeed\";s:1:\"0\";s:11:\"managedoing\";s:1:\"0\";s:11:\"manageshare\";s:1:\"0\";s:10:\"manageblog\";s:1:\"0\";s:11:\"managealbum\";s:1:\"0\";s:13:\"managecomment\";s:1:\"0\";s:14:\"managemagiclog\";s:1:\"0\";s:12:\"managereport\";s:1:\"0\";s:13:\"managehotuser\";s:1:\"0\";s:17:\"managedefaultuser\";s:1:\"0\";s:11:\"managemagic\";s:1:\"0\";s:11:\"manageclick\";s:1:\"0\";s:21:\"allowmanagecollection\";s:1:\"0\";s:13:\"allowmakehtml\";s:1:\"0\";s:9:\"grouptype\";s:7:\"special\";s:11:\"grouppublic\";b:0;s:18:\"groupcreditshigher\";s:1:\"0\";s:17:\"groupcreditslower\";s:1:\"0\";s:17:\"allowthreadplugin\";a:0:{}s:6:\"plugin\";N;s:5:\"style\";N;}'),
('usergroup_2',1,1784453715,'a:197:{s:7:\"groupid\";s:1:\"2\";s:8:\"radminid\";s:1:\"2\";s:10:\"grouptitle\";s:12:\"瓒呯骇鐗堜富\";s:5:\"stars\";s:1:\"8\";s:5:\"color\";s:0:\"\";s:4:\"icon\";s:0:\"\";s:10:\"allowvisit\";s:1:\"1\";s:11:\"allowsendpm\";s:1:\"1\";s:11:\"allowinvite\";s:1:\"1\";s:15:\"allowmailinvite\";s:1:\"1\";s:11:\"allowfollow\";s:1:\"1\";s:12:\"maxinvitenum\";s:1:\"0\";s:11:\"inviteprice\";s:1:\"0\";s:12:\"maxinviteday\";s:2:\"10\";s:9:\"upgroupid\";s:1:\"0\";s:14:\"creditsformula\";s:0:\"\";s:10:\"readaccess\";s:3:\"150\";s:9:\"allowpost\";s:1:\"1\";s:10:\"allowreply\";s:1:\"1\";s:13:\"allowpostpoll\";s:1:\"1\";s:15:\"allowpostreward\";s:1:\"1\";s:14:\"allowposttrade\";s:1:\"1\";s:17:\"allowpostactivity\";s:1:\"1\";s:15:\"allowdirectpost\";s:1:\"3\";s:14:\"allowgetattach\";s:1:\"1\";s:13:\"allowgetimage\";s:1:\"1\";s:15:\"allowpostattach\";s:1:\"1\";s:14:\"allowpostimage\";s:1:\"1\";s:9:\"allowvote\";s:1:\"1\";s:11:\"allowsearch\";s:2:\"95\";s:12:\"allowcstatus\";s:1:\"1\";s:14:\"allowinvisible\";s:1:\"1\";s:13:\"allowtransfer\";s:1:\"1\";s:16:\"allowsetreadperm\";s:1:\"1\";s:18:\"allowsetattachperm\";s:1:\"1\";s:12:\"allowposttag\";s:1:\"0\";s:13:\"allowhidecode\";s:1:\"1\";s:9:\"allowhtml\";s:1:\"0\";s:14:\"allowanonymous\";s:1:\"1\";s:14:\"allowsigbbcode\";s:1:\"1\";s:15:\"allowsigimgcode\";s:1:\"1\";s:11:\"allowmagics\";s:1:\"2\";s:17:\"disableperiodctrl\";s:1:\"1\";s:8:\"reasonpm\";s:1:\"0\";s:8:\"maxprice\";s:2:\"20\";s:10:\"maxsigsize\";s:3:\"300\";s:13:\"maxattachsize\";s:7:\"2048000\";s:13:\"maxsizeperday\";s:1:\"0\";s:17:\"maxthreadsperhour\";s:1:\"0\";s:15:\"maxpostsperhour\";s:1:\"0\";s:16:\"attachextensions\";s:55:\"chm, pdf, zip, rar, tar, gz, bzip2, gif, jpg, jpeg, png\";s:9:\"raterange\";a:0:{}s:11:\"loginreward\";s:0:\"\";s:13:\"mintradeprice\";s:1:\"1\";s:13:\"maxtradeprice\";s:1:\"0\";s:14:\"minrewardprice\";s:1:\"1\";s:14:\"maxrewardprice\";s:1:\"0\";s:14:\"magicsdiscount\";s:1:\"0\";s:15:\"maxmagicsweight\";s:3:\"180\";s:15:\"allowpostdebate\";s:1:\"1\";s:10:\"tradestick\";s:1:\"5\";s:6:\"exempt\";s:3:\"255\";s:12:\"maxattachnum\";s:1:\"0\";s:12:\"allowposturl\";s:1:\"3\";s:14:\"allowrecommend\";s:1:\"1\";s:18:\"allowpostrushreply\";s:1:\"0\";s:12:\"maxfriendnum\";s:1:\"0\";s:12:\"maxspacesize\";s:1:\"0\";s:12:\"allowcomment\";s:1:\"1\";s:15:\"allowcommentmod\";s:1:\"0\";s:19:\"allowcommentarticle\";s:4:\"1000\";s:22:\"allowcommentarticlemod\";s:1:\"0\";s:14:\"searchinterval\";s:1:\"0\";s:12:\"searchignore\";s:1:\"0\";s:9:\"allowblog\";s:1:\"1\";s:10:\"allowdoing\";s:1:\"1\";s:11:\"allowupload\";s:1:\"1\";s:10:\"allowshare\";s:1:\"1\";s:12:\"allowblogmod\";s:1:\"0\";s:13:\"allowdoingmod\";s:1:\"0\";s:14:\"allowuploadmod\";s:1:\"0\";s:13:\"allowsharemod\";s:1:\"0\";s:8:\"allowcss\";s:1:\"0\";s:9:\"allowpoke\";s:1:\"1\";s:11:\"allowfriend\";s:1:\"1\";s:10:\"allowclick\";s:1:\"1\";s:10:\"allowmagic\";s:1:\"0\";s:9:\"allowstat\";s:1:\"0\";s:13:\"allowstatdata\";s:1:\"0\";s:13:\"magicdiscount\";s:1:\"0\";s:12:\"domainlength\";s:1:\"5\";s:7:\"seccode\";s:1:\"1\";s:15:\"disablepostctrl\";s:1:\"1\";s:15:\"allowbuildgroup\";s:2:\"20\";s:20:\"allowgroupdirectpost\";s:1:\"3\";s:17:\"allowgroupposturl\";s:1:\"0\";s:13:\"edittimelimit\";s:1:\"0\";s:16:\"allowpostarticle\";s:1:\"0\";s:17:\"allowdownlocalimg\";s:1:\"0\";s:18:\"allowdownremoteimg\";s:1:\"0\";s:19:\"allowpostarticlemod\";s:1:\"0\";s:17:\"allowspacediyhtml\";s:1:\"0\";s:19:\"allowspacediybbcode\";s:1:\"1\";s:20:\"allowspacediyimgcode\";s:1:\"1\";s:16:\"allowcommentpost\";s:1:\"2\";s:16:\"allowcommentitem\";s:1:\"1\";s:17:\"allowcommentreply\";s:1:\"0\";s:16:\"allowreplycredit\";s:1:\"1\";s:12:\"ignorecensor\";s:1:\"1\";s:14:\"allowsendallpm\";s:1:\"0\";s:17:\"allowsendpmmaxnum\";s:1:\"0\";s:12:\"maximagesize\";s:1:\"0\";s:14:\"allowmediacode\";s:1:\"0\";s:14:\"allowbegincode\";s:1:\"0\";s:7:\"allowat\";s:1:\"0\";s:9:\"allowsave\";s:1:\"1\";s:14:\"allowsavereply\";s:1:\"1\";s:12:\"allowsavenum\";s:1:\"0\";s:19:\"allowsetpublishdate\";s:1:\"0\";s:21:\"allowfollowcollection\";s:2:\"30\";s:22:\"allowcommentcollection\";s:1:\"1\";s:21:\"allowcreatecollection\";s:1:\"5\";s:12:\"forcesecques\";s:1:\"0\";s:10:\"forcelogin\";s:1:\"0\";s:7:\"closead\";s:1:\"0\";s:17:\"buildgroupcredits\";s:1:\"0\";s:15:\"allowimgcontent\";s:1:\"0\";s:17:\"allowavatarupload\";s:1:\"1\";s:16:\"allowviewprofile\";s:1:\"1\";s:6:\"fields\";a:0:{}s:13:\"alloweditpost\";s:1:\"1\";s:13:\"alloweditpoll\";s:1:\"0\";s:16:\"allowstickthread\";s:1:\"2\";s:12:\"allowmodpost\";s:1:\"1\";s:12:\"allowdelpost\";s:1:\"1\";s:14:\"allowmassprune\";s:1:\"1\";s:11:\"allowrefund\";s:1:\"1\";s:15:\"allowcensorword\";s:1:\"1\";s:11:\"allowviewip\";s:1:\"1\";s:10:\"allowbanip\";s:1:\"1\";s:13:\"allowedituser\";s:1:\"1\";s:12:\"allowmoduser\";s:1:\"1\";s:12:\"allowbanuser\";s:1:\"1\";s:17:\"allowbanvisituser\";s:1:\"1\";s:17:\"allowpostannounce\";s:1:\"1\";s:12:\"allowviewlog\";s:1:\"1\";s:12:\"allowbanpost\";s:1:\"1\";s:20:\"supe_allowpushthread\";s:1:\"0\";s:20:\"allowhighlightthread\";s:1:\"1\";s:15:\"allowlivethread\";s:1:\"1\";s:17:\"allowdigestthread\";s:1:\"3\";s:20:\"allowrecommendthread\";s:1:\"1\";s:15:\"allowbumpthread\";s:1:\"1\";s:16:\"allowclosethread\";s:1:\"1\";s:15:\"allowmovethread\";s:1:\"1\";s:19:\"allowedittypethread\";s:1:\"1\";s:16:\"allowstampthread\";s:1:\"1\";s:14:\"allowstamplist\";s:1:\"1\";s:15:\"allowcopythread\";s:1:\"1\";s:16:\"allowmergethread\";s:1:\"1\";s:16:\"allowsplitthread\";s:1:\"1\";s:17:\"allowrepairthread\";s:1:\"1\";s:13:\"allowwarnpost\";s:1:\"1\";s:15:\"allowviewreport\";s:1:\"1\";s:14:\"alloweditforum\";s:1:\"1\";s:17:\"allowremovereward\";s:1:\"1\";s:14:\"allowedittrade\";s:1:\"0\";s:17:\"alloweditactivity\";s:1:\"0\";s:15:\"allowstickreply\";s:1:\"0\";s:18:\"allowmanagearticle\";s:1:\"0\";s:13:\"allowaddtopic\";s:1:\"0\";s:16:\"allowmanagetopic\";s:1:\"0\";s:8:\"allowdiy\";s:1:\"0\";s:17:\"allowclearrecycle\";s:1:\"0\";s:14:\"allowmanagetag\";s:1:\"0\";s:16:\"alloweditusertag\";s:1:\"0\";s:10:\"managefeed\";s:1:\"0\";s:11:\"managedoing\";s:1:\"0\";s:11:\"manageshare\";s:1:\"0\";s:10:\"manageblog\";s:1:\"0\";s:11:\"managealbum\";s:1:\"0\";s:13:\"managecomment\";s:1:\"0\";s:14:\"managemagiclog\";s:1:\"0\";s:12:\"managereport\";s:1:\"0\";s:13:\"managehotuser\";s:1:\"0\";s:17:\"managedefaultuser\";s:1:\"0\";s:11:\"managemagic\";s:1:\"0\";s:11:\"manageclick\";s:1:\"0\";s:21:\"allowmanagecollection\";s:1:\"1\";s:13:\"allowmakehtml\";s:1:\"0\";s:9:\"grouptype\";s:6:\"system\";s:11:\"grouppublic\";b:0;s:18:\"groupcreditshigher\";s:1:\"0\";s:17:\"groupcreditslower\";s:1:\"0\";s:17:\"allowthreadplugin\";a:0:{}s:6:\"plugin\";N;s:5:\"style\";N;}'),
('usergroup_3',1,1784453715,'a:197:{s:7:\"groupid\";s:1:\"3\";s:8:\"radminid\";s:1:\"3\";s:10:\"grouptitle\";s:6:\"鐗堜富\";s:5:\"stars\";s:1:\"7\";s:5:\"color\";s:0:\"\";s:4:\"icon\";s:0:\"\";s:10:\"allowvisit\";s:1:\"1\";s:11:\"allowsendpm\";s:1:\"1\";s:11:\"allowinvite\";s:1:\"1\";s:15:\"allowmailinvite\";s:1:\"1\";s:11:\"allowfollow\";s:1:\"1\";s:12:\"maxinvitenum\";s:1:\"0\";s:11:\"inviteprice\";s:1:\"0\";s:12:\"maxinviteday\";s:2:\"10\";s:9:\"upgroupid\";s:1:\"0\";s:14:\"creditsformula\";s:0:\"\";s:10:\"readaccess\";s:3:\"100\";s:9:\"allowpost\";s:1:\"1\";s:10:\"allowreply\";s:1:\"1\";s:13:\"allowpostpoll\";s:1:\"1\";s:15:\"allowpostreward\";s:1:\"1\";s:14:\"allowposttrade\";s:1:\"1\";s:17:\"allowpostactivity\";s:1:\"1\";s:15:\"allowdirectpost\";s:1:\"1\";s:14:\"allowgetattach\";s:1:\"1\";s:13:\"allowgetimage\";s:1:\"1\";s:15:\"allowpostattach\";s:1:\"1\";s:14:\"allowpostimage\";s:1:\"1\";s:9:\"allowvote\";s:1:\"1\";s:11:\"allowsearch\";s:2:\"95\";s:12:\"allowcstatus\";s:1:\"1\";s:14:\"allowinvisible\";s:1:\"0\";s:13:\"allowtransfer\";s:1:\"1\";s:16:\"allowsetreadperm\";s:1:\"1\";s:18:\"allowsetattachperm\";s:1:\"1\";s:12:\"allowposttag\";s:1:\"0\";s:13:\"allowhidecode\";s:1:\"1\";s:9:\"allowhtml\";s:1:\"0\";s:14:\"allowanonymous\";s:1:\"0\";s:14:\"allowsigbbcode\";s:1:\"1\";s:15:\"allowsigimgcode\";s:1:\"1\";s:11:\"allowmagics\";s:1:\"2\";s:17:\"disableperiodctrl\";s:1:\"1\";s:8:\"reasonpm\";s:1:\"0\";s:8:\"maxprice\";s:2:\"10\";s:10:\"maxsigsize\";s:3:\"200\";s:13:\"maxattachsize\";s:7:\"2048000\";s:13:\"maxsizeperday\";s:1:\"0\";s:17:\"maxthreadsperhour\";s:1:\"0\";s:15:\"maxpostsperhour\";s:1:\"0\";s:16:\"attachextensions\";s:55:\"chm, pdf, zip, rar, tar, gz, bzip2, gif, jpg, jpeg, png\";s:9:\"raterange\";a:0:{}s:11:\"loginreward\";s:0:\"\";s:13:\"mintradeprice\";s:1:\"1\";s:13:\"maxtradeprice\";s:1:\"0\";s:14:\"minrewardprice\";s:1:\"1\";s:14:\"maxrewardprice\";s:1:\"0\";s:14:\"magicsdiscount\";s:1:\"0\";s:15:\"maxmagicsweight\";s:3:\"160\";s:15:\"allowpostdebate\";s:1:\"1\";s:10:\"tradestick\";s:1:\"5\";s:6:\"exempt\";s:3:\"224\";s:12:\"maxattachnum\";s:1:\"0\";s:12:\"allowposturl\";s:1:\"3\";s:14:\"allowrecommend\";s:1:\"1\";s:18:\"allowpostrushreply\";s:1:\"0\";s:12:\"maxfriendnum\";s:1:\"0\";s:12:\"maxspacesize\";s:1:\"0\";s:12:\"allowcomment\";s:1:\"1\";s:15:\"allowcommentmod\";s:1:\"0\";s:19:\"allowcommentarticle\";s:4:\"1000\";s:22:\"allowcommentarticlemod\";s:1:\"0\";s:14:\"searchinterval\";s:1:\"0\";s:12:\"searchignore\";s:1:\"0\";s:9:\"allowblog\";s:1:\"1\";s:10:\"allowdoing\";s:1:\"1\";s:11:\"allowupload\";s:1:\"1\";s:10:\"allowshare\";s:1:\"1\";s:12:\"allowblogmod\";s:1:\"0\";s:13:\"allowdoingmod\";s:1:\"0\";s:14:\"allowuploadmod\";s:1:\"0\";s:13:\"allowsharemod\";s:1:\"0\";s:8:\"allowcss\";s:1:\"0\";s:9:\"allowpoke\";s:1:\"1\";s:11:\"allowfriend\";s:1:\"1\";s:10:\"allowclick\";s:1:\"1\";s:10:\"allowmagic\";s:1:\"0\";s:9:\"allowstat\";s:1:\"0\";s:13:\"allowstatdata\";s:1:\"0\";s:13:\"magicdiscount\";s:1:\"0\";s:12:\"domainlength\";s:1:\"5\";s:7:\"seccode\";s:1:\"1\";s:15:\"disablepostctrl\";s:1:\"1\";s:15:\"allowbuildgroup\";s:2:\"15\";s:20:\"allowgroupdirectpost\";s:1:\"3\";s:17:\"allowgroupposturl\";s:1:\"0\";s:13:\"edittimelimit\";s:1:\"0\";s:16:\"allowpostarticle\";s:1:\"0\";s:17:\"allowdownlocalimg\";s:1:\"0\";s:18:\"allowdownremoteimg\";s:1:\"0\";s:19:\"allowpostarticlemod\";s:1:\"0\";s:17:\"allowspacediyhtml\";s:1:\"0\";s:19:\"allowspacediybbcode\";s:1:\"1\";s:20:\"allowspacediyimgcode\";s:1:\"1\";s:16:\"allowcommentpost\";s:1:\"2\";s:16:\"allowcommentitem\";s:1:\"1\";s:17:\"allowcommentreply\";s:1:\"0\";s:16:\"allowreplycredit\";s:1:\"1\";s:12:\"ignorecensor\";s:1:\"1\";s:14:\"allowsendallpm\";s:1:\"0\";s:17:\"allowsendpmmaxnum\";s:1:\"0\";s:12:\"maximagesize\";s:1:\"0\";s:14:\"allowmediacode\";s:1:\"0\";s:14:\"allowbegincode\";s:1:\"0\";s:7:\"allowat\";s:1:\"0\";s:9:\"allowsave\";s:1:\"1\";s:14:\"allowsavereply\";s:1:\"1\";s:12:\"allowsavenum\";s:1:\"0\";s:19:\"allowsetpublishdate\";s:1:\"0\";s:21:\"allowfollowcollection\";s:2:\"30\";s:22:\"allowcommentcollection\";s:1:\"1\";s:21:\"allowcreatecollection\";s:1:\"5\";s:12:\"forcesecques\";s:1:\"0\";s:10:\"forcelogin\";s:1:\"0\";s:7:\"closead\";s:1:\"0\";s:17:\"buildgroupcredits\";s:1:\"0\";s:15:\"allowimgcontent\";s:1:\"0\";s:17:\"allowavatarupload\";s:1:\"1\";s:16:\"allowviewprofile\";s:1:\"1\";s:6:\"fields\";a:0:{}s:13:\"alloweditpost\";s:1:\"1\";s:13:\"alloweditpoll\";s:1:\"0\";s:16:\"allowstickthread\";s:1:\"1\";s:12:\"allowmodpost\";s:1:\"1\";s:12:\"allowdelpost\";s:1:\"1\";s:14:\"allowmassprune\";s:1:\"0\";s:11:\"allowrefund\";s:1:\"0\";s:15:\"allowcensorword\";s:1:\"0\";s:11:\"allowviewip\";s:1:\"1\";s:10:\"allowbanip\";s:1:\"0\";s:13:\"allowedituser\";s:1:\"0\";s:12:\"allowmoduser\";s:1:\"1\";s:12:\"allowbanuser\";s:1:\"1\";s:17:\"allowbanvisituser\";s:1:\"0\";s:17:\"allowpostannounce\";s:1:\"0\";s:12:\"allowviewlog\";s:1:\"1\";s:12:\"allowbanpost\";s:1:\"1\";s:20:\"supe_allowpushthread\";s:1:\"0\";s:20:\"allowhighlightthread\";s:1:\"1\";s:15:\"allowlivethread\";s:1:\"1\";s:17:\"allowdigestthread\";s:1:\"3\";s:20:\"allowrecommendthread\";s:1:\"1\";s:15:\"allowbumpthread\";s:1:\"1\";s:16:\"allowclosethread\";s:1:\"1\";s:15:\"allowmovethread\";s:1:\"1\";s:19:\"allowedittypethread\";s:1:\"1\";s:16:\"allowstampthread\";s:1:\"1\";s:14:\"allowstamplist\";s:1:\"1\";s:15:\"allowcopythread\";s:1:\"1\";s:16:\"allowmergethread\";s:1:\"1\";s:16:\"allowsplitthread\";s:1:\"1\";s:17:\"allowrepairthread\";s:1:\"1\";s:13:\"allowwarnpost\";s:1:\"1\";s:15:\"allowviewreport\";s:1:\"1\";s:14:\"alloweditforum\";s:1:\"1\";s:17:\"allowremovereward\";s:1:\"1\";s:14:\"allowedittrade\";s:1:\"0\";s:17:\"alloweditactivity\";s:1:\"0\";s:15:\"allowstickreply\";s:1:\"0\";s:18:\"allowmanagearticle\";s:1:\"0\";s:13:\"allowaddtopic\";s:1:\"0\";s:16:\"allowmanagetopic\";s:1:\"0\";s:8:\"allowdiy\";s:1:\"0\";s:17:\"allowclearrecycle\";s:1:\"0\";s:14:\"allowmanagetag\";s:1:\"0\";s:16:\"alloweditusertag\";s:1:\"0\";s:10:\"managefeed\";s:1:\"0\";s:11:\"managedoing\";s:1:\"0\";s:11:\"manageshare\";s:1:\"0\";s:10:\"manageblog\";s:1:\"0\";s:11:\"managealbum\";s:1:\"0\";s:13:\"managecomment\";s:1:\"0\";s:14:\"managemagiclog\";s:1:\"0\";s:12:\"managereport\";s:1:\"0\";s:13:\"managehotuser\";s:1:\"0\";s:17:\"managedefaultuser\";s:1:\"0\";s:11:\"managemagic\";s:1:\"0\";s:11:\"manageclick\";s:1:\"0\";s:21:\"allowmanagecollection\";s:1:\"0\";s:13:\"allowmakehtml\";s:1:\"0\";s:9:\"grouptype\";s:6:\"system\";s:11:\"grouppublic\";b:0;s:18:\"groupcreditshigher\";s:1:\"0\";s:17:\"groupcreditslower\";s:1:\"0\";s:17:\"allowthreadplugin\";a:0:{}s:6:\"plugin\";N;s:5:\"style\";N;}'),
('usergroup_4',1,1784453715,'a:137:{s:7:\"groupid\";s:1:\"4\";s:8:\"radminid\";s:1:\"0\";s:10:\"grouptitle\";s:12:\"绂佹鍙戣█\";s:5:\"stars\";s:1:\"0\";s:5:\"color\";s:0:\"\";s:4:\"icon\";s:0:\"\";s:10:\"allowvisit\";s:1:\"1\";s:11:\"allowsendpm\";s:1:\"1\";s:11:\"allowinvite\";s:1:\"0\";s:15:\"allowmailinvite\";s:1:\"0\";s:11:\"allowfollow\";s:1:\"0\";s:12:\"maxinvitenum\";s:1:\"0\";s:11:\"inviteprice\";s:1:\"0\";s:12:\"maxinviteday\";s:1:\"0\";s:9:\"upgroupid\";s:1:\"0\";s:14:\"creditsformula\";s:0:\"\";s:10:\"readaccess\";s:1:\"0\";s:9:\"allowpost\";s:1:\"0\";s:10:\"allowreply\";s:1:\"0\";s:13:\"allowpostpoll\";s:1:\"0\";s:15:\"allowpostreward\";s:1:\"0\";s:14:\"allowposttrade\";s:1:\"0\";s:17:\"allowpostactivity\";s:1:\"0\";s:15:\"allowdirectpost\";s:1:\"0\";s:14:\"allowgetattach\";s:1:\"0\";s:13:\"allowgetimage\";s:1:\"0\";s:15:\"allowpostattach\";s:1:\"0\";s:14:\"allowpostimage\";s:1:\"0\";s:9:\"allowvote\";s:1:\"0\";s:11:\"allowsearch\";s:1:\"0\";s:12:\"allowcstatus\";s:1:\"0\";s:14:\"allowinvisible\";s:1:\"0\";s:13:\"allowtransfer\";s:1:\"0\";s:16:\"allowsetreadperm\";s:1:\"0\";s:18:\"allowsetattachperm\";s:1:\"0\";s:12:\"allowposttag\";s:1:\"0\";s:13:\"allowhidecode\";s:1:\"0\";s:9:\"allowhtml\";s:1:\"0\";s:14:\"allowanonymous\";s:1:\"0\";s:14:\"allowsigbbcode\";s:1:\"0\";s:15:\"allowsigimgcode\";s:1:\"0\";s:11:\"allowmagics\";s:1:\"0\";s:17:\"disableperiodctrl\";s:1:\"0\";s:8:\"reasonpm\";s:1:\"0\";s:8:\"maxprice\";s:1:\"0\";s:10:\"maxsigsize\";s:1:\"0\";s:13:\"maxattachsize\";s:1:\"0\";s:13:\"maxsizeperday\";s:1:\"0\";s:17:\"maxthreadsperhour\";s:1:\"0\";s:15:\"maxpostsperhour\";s:1:\"0\";s:16:\"attachextensions\";s:0:\"\";s:9:\"raterange\";a:0:{}s:11:\"loginreward\";s:0:\"\";s:13:\"mintradeprice\";s:1:\"1\";s:13:\"maxtradeprice\";s:1:\"0\";s:14:\"minrewardprice\";s:1:\"1\";s:14:\"maxrewardprice\";s:1:\"0\";s:14:\"magicsdiscount\";s:1:\"0\";s:15:\"maxmagicsweight\";s:1:\"0\";s:15:\"allowpostdebate\";s:1:\"0\";s:10:\"tradestick\";s:1:\"5\";s:6:\"exempt\";s:1:\"0\";s:12:\"maxattachnum\";s:1:\"0\";s:12:\"allowposturl\";s:1:\"3\";s:14:\"allowrecommend\";s:1:\"1\";s:18:\"allowpostrushreply\";s:1:\"0\";s:12:\"maxfriendnum\";s:1:\"0\";s:12:\"maxspacesize\";s:1:\"0\";s:12:\"allowcomment\";s:1:\"0\";s:15:\"allowcommentmod\";s:1:\"0\";s:19:\"allowcommentarticle\";s:1:\"0\";s:22:\"allowcommentarticlemod\";s:1:\"0\";s:14:\"searchinterval\";s:1:\"0\";s:12:\"searchignore\";s:1:\"0\";s:9:\"allowblog\";s:1:\"0\";s:10:\"allowdoing\";s:1:\"0\";s:11:\"allowupload\";s:1:\"0\";s:10:\"allowshare\";s:1:\"0\";s:12:\"allowblogmod\";s:1:\"0\";s:13:\"allowdoingmod\";s:1:\"0\";s:14:\"allowuploadmod\";s:1:\"0\";s:13:\"allowsharemod\";s:1:\"0\";s:8:\"allowcss\";s:1:\"0\";s:9:\"allowpoke\";s:1:\"0\";s:11:\"allowfriend\";s:1:\"0\";s:10:\"allowclick\";s:1:\"0\";s:10:\"allowmagic\";s:1:\"0\";s:9:\"allowstat\";s:1:\"0\";s:13:\"allowstatdata\";s:1:\"0\";s:13:\"magicdiscount\";s:1:\"0\";s:12:\"domainlength\";s:1:\"0\";s:7:\"seccode\";s:1:\"1\";s:15:\"disablepostctrl\";s:1:\"0\";s:15:\"allowbuildgroup\";s:1:\"0\";s:20:\"allowgroupdirectpost\";s:1:\"3\";s:17:\"allowgroupposturl\";s:1:\"0\";s:13:\"edittimelimit\";s:1:\"0\";s:16:\"allowpostarticle\";s:1:\"0\";s:17:\"allowdownlocalimg\";s:1:\"0\";s:18:\"allowdownremoteimg\";s:1:\"0\";s:19:\"allowpostarticlemod\";s:1:\"0\";s:17:\"allowspacediyhtml\";s:1:\"0\";s:19:\"allowspacediybbcode\";s:1:\"0\";s:20:\"allowspacediyimgcode\";s:1:\"0\";s:16:\"allowcommentpost\";s:1:\"0\";s:16:\"allowcommentitem\";s:1:\"0\";s:17:\"allowcommentreply\";s:1:\"0\";s:16:\"allowreplycredit\";s:1:\"0\";s:12:\"ignorecensor\";s:1:\"0\";s:14:\"allowsendallpm\";s:1:\"0\";s:17:\"allowsendpmmaxnum\";s:1:\"0\";s:12:\"maximagesize\";s:1:\"0\";s:14:\"allowmediacode\";s:1:\"0\";s:14:\"allowbegincode\";s:1:\"0\";s:7:\"allowat\";s:1:\"0\";s:9:\"allowsave\";s:1:\"1\";s:14:\"allowsavereply\";s:1:\"1\";s:12:\"allowsavenum\";s:1:\"0\";s:19:\"allowsetpublishdate\";s:1:\"0\";s:21:\"allowfollowcollection\";s:1:\"0\";s:22:\"allowcommentcollection\";s:1:\"0\";s:21:\"allowcreatecollection\";s:1:\"0\";s:12:\"forcesecques\";s:1:\"0\";s:10:\"forcelogin\";s:1:\"0\";s:7:\"closead\";s:1:\"0\";s:17:\"buildgroupcredits\";s:1:\"0\";s:15:\"allowimgcontent\";s:1:\"0\";s:17:\"allowavatarupload\";s:1:\"0\";s:16:\"allowviewprofile\";s:1:\"0\";s:6:\"fields\";a:0:{}s:9:\"grouptype\";s:6:\"system\";s:11:\"grouppublic\";b:0;s:18:\"groupcreditshigher\";s:1:\"0\";s:17:\"groupcreditslower\";s:1:\"0\";s:17:\"allowthreadplugin\";a:0:{}s:6:\"plugin\";N;s:5:\"style\";N;}'),
('usergroup_5',1,1784453715,'a:137:{s:7:\"groupid\";s:1:\"5\";s:8:\"radminid\";s:1:\"0\";s:10:\"grouptitle\";s:12:\"绂佹璁块棶\";s:5:\"stars\";s:1:\"0\";s:5:\"color\";s:0:\"\";s:4:\"icon\";s:0:\"\";s:10:\"allowvisit\";s:1:\"0\";s:11:\"allowsendpm\";s:1:\"1\";s:11:\"allowinvite\";s:1:\"0\";s:15:\"allowmailinvite\";s:1:\"0\";s:11:\"allowfollow\";s:1:\"0\";s:12:\"maxinvitenum\";s:1:\"0\";s:11:\"inviteprice\";s:1:\"0\";s:12:\"maxinviteday\";s:1:\"0\";s:9:\"upgroupid\";s:1:\"0\";s:14:\"creditsformula\";s:0:\"\";s:10:\"readaccess\";s:1:\"0\";s:9:\"allowpost\";s:1:\"0\";s:10:\"allowreply\";s:1:\"0\";s:13:\"allowpostpoll\";s:1:\"0\";s:15:\"allowpostreward\";s:1:\"0\";s:14:\"allowposttrade\";s:1:\"0\";s:17:\"allowpostactivity\";s:1:\"0\";s:15:\"allowdirectpost\";s:1:\"0\";s:14:\"allowgetattach\";s:1:\"0\";s:13:\"allowgetimage\";s:1:\"0\";s:15:\"allowpostattach\";s:1:\"0\";s:14:\"allowpostimage\";s:1:\"0\";s:9:\"allowvote\";s:1:\"0\";s:11:\"allowsearch\";s:1:\"0\";s:12:\"allowcstatus\";s:1:\"0\";s:14:\"allowinvisible\";s:1:\"0\";s:13:\"allowtransfer\";s:1:\"0\";s:16:\"allowsetreadperm\";s:1:\"0\";s:18:\"allowsetattachperm\";s:1:\"0\";s:12:\"allowposttag\";s:1:\"0\";s:13:\"allowhidecode\";s:1:\"0\";s:9:\"allowhtml\";s:1:\"0\";s:14:\"allowanonymous\";s:1:\"0\";s:14:\"allowsigbbcode\";s:1:\"0\";s:15:\"allowsigimgcode\";s:1:\"0\";s:11:\"allowmagics\";s:1:\"0\";s:17:\"disableperiodctrl\";s:1:\"0\";s:8:\"reasonpm\";s:1:\"0\";s:8:\"maxprice\";s:1:\"0\";s:10:\"maxsigsize\";s:1:\"0\";s:13:\"maxattachsize\";s:1:\"0\";s:13:\"maxsizeperday\";s:1:\"0\";s:17:\"maxthreadsperhour\";s:1:\"0\";s:15:\"maxpostsperhour\";s:1:\"0\";s:16:\"attachextensions\";s:0:\"\";s:9:\"raterange\";a:0:{}s:11:\"loginreward\";s:0:\"\";s:13:\"mintradeprice\";s:1:\"1\";s:13:\"maxtradeprice\";s:1:\"0\";s:14:\"minrewardprice\";s:1:\"1\";s:14:\"maxrewardprice\";s:1:\"0\";s:14:\"magicsdiscount\";s:1:\"0\";s:15:\"maxmagicsweight\";s:1:\"0\";s:15:\"allowpostdebate\";s:1:\"0\";s:10:\"tradestick\";s:1:\"5\";s:6:\"exempt\";s:1:\"0\";s:12:\"maxattachnum\";s:1:\"0\";s:12:\"allowposturl\";s:1:\"3\";s:14:\"allowrecommend\";s:1:\"1\";s:18:\"allowpostrushreply\";s:1:\"0\";s:12:\"maxfriendnum\";s:1:\"0\";s:12:\"maxspacesize\";s:1:\"0\";s:12:\"allowcomment\";s:1:\"0\";s:15:\"allowcommentmod\";s:1:\"0\";s:19:\"allowcommentarticle\";s:1:\"0\";s:22:\"allowcommentarticlemod\";s:1:\"0\";s:14:\"searchinterval\";s:1:\"0\";s:12:\"searchignore\";s:1:\"0\";s:9:\"allowblog\";s:1:\"0\";s:10:\"allowdoing\";s:1:\"0\";s:11:\"allowupload\";s:1:\"0\";s:10:\"allowshare\";s:1:\"0\";s:12:\"allowblogmod\";s:1:\"0\";s:13:\"allowdoingmod\";s:1:\"0\";s:14:\"allowuploadmod\";s:1:\"0\";s:13:\"allowsharemod\";s:1:\"0\";s:8:\"allowcss\";s:1:\"0\";s:9:\"allowpoke\";s:1:\"0\";s:11:\"allowfriend\";s:1:\"0\";s:10:\"allowclick\";s:1:\"0\";s:10:\"allowmagic\";s:1:\"0\";s:9:\"allowstat\";s:1:\"0\";s:13:\"allowstatdata\";s:1:\"0\";s:13:\"magicdiscount\";s:1:\"0\";s:12:\"domainlength\";s:1:\"0\";s:7:\"seccode\";s:1:\"1\";s:15:\"disablepostctrl\";s:1:\"0\";s:15:\"allowbuildgroup\";s:1:\"0\";s:20:\"allowgroupdirectpost\";s:1:\"3\";s:17:\"allowgroupposturl\";s:1:\"0\";s:13:\"edittimelimit\";s:1:\"0\";s:16:\"allowpostarticle\";s:1:\"0\";s:17:\"allowdownlocalimg\";s:1:\"0\";s:18:\"allowdownremoteimg\";s:1:\"0\";s:19:\"allowpostarticlemod\";s:1:\"0\";s:17:\"allowspacediyhtml\";s:1:\"0\";s:19:\"allowspacediybbcode\";s:1:\"0\";s:20:\"allowspacediyimgcode\";s:1:\"0\";s:16:\"allowcommentpost\";s:1:\"0\";s:16:\"allowcommentitem\";s:1:\"0\";s:17:\"allowcommentreply\";s:1:\"0\";s:16:\"allowreplycredit\";s:1:\"0\";s:12:\"ignorecensor\";s:1:\"0\";s:14:\"allowsendallpm\";s:1:\"0\";s:17:\"allowsendpmmaxnum\";s:1:\"0\";s:12:\"maximagesize\";s:1:\"0\";s:14:\"allowmediacode\";s:1:\"0\";s:14:\"allowbegincode\";s:1:\"0\";s:7:\"allowat\";s:1:\"0\";s:9:\"allowsave\";s:1:\"1\";s:14:\"allowsavereply\";s:1:\"1\";s:12:\"allowsavenum\";s:1:\"0\";s:19:\"allowsetpublishdate\";s:1:\"0\";s:21:\"allowfollowcollection\";s:1:\"0\";s:22:\"allowcommentcollection\";s:1:\"0\";s:21:\"allowcreatecollection\";s:1:\"0\";s:12:\"forcesecques\";s:1:\"0\";s:10:\"forcelogin\";s:1:\"0\";s:7:\"closead\";s:1:\"0\";s:17:\"buildgroupcredits\";s:1:\"0\";s:15:\"allowimgcontent\";s:1:\"0\";s:17:\"allowavatarupload\";s:1:\"0\";s:16:\"allowviewprofile\";s:1:\"0\";s:6:\"fields\";a:0:{}s:9:\"grouptype\";s:6:\"system\";s:11:\"grouppublic\";b:0;s:18:\"groupcreditshigher\";s:1:\"0\";s:17:\"groupcreditslower\";s:1:\"0\";s:17:\"allowthreadplugin\";a:0:{}s:6:\"plugin\";N;s:5:\"style\";N;}'),
('usergroup_6',1,1784453715,'a:137:{s:7:\"groupid\";s:1:\"6\";s:8:\"radminid\";s:1:\"0\";s:10:\"grouptitle\";s:9:\"绂佹 IP\";s:5:\"stars\";s:1:\"0\";s:5:\"color\";s:0:\"\";s:4:\"icon\";s:0:\"\";s:10:\"allowvisit\";s:1:\"0\";s:11:\"allowsendpm\";s:1:\"1\";s:11:\"allowinvite\";s:1:\"0\";s:15:\"allowmailinvite\";s:1:\"0\";s:11:\"allowfollow\";s:1:\"0\";s:12:\"maxinvitenum\";s:1:\"0\";s:11:\"inviteprice\";s:1:\"0\";s:12:\"maxinviteday\";s:1:\"0\";s:9:\"upgroupid\";s:1:\"0\";s:14:\"creditsformula\";s:0:\"\";s:10:\"readaccess\";s:1:\"0\";s:9:\"allowpost\";s:1:\"0\";s:10:\"allowreply\";s:1:\"0\";s:13:\"allowpostpoll\";s:1:\"0\";s:15:\"allowpostreward\";s:1:\"0\";s:14:\"allowposttrade\";s:1:\"0\";s:17:\"allowpostactivity\";s:1:\"0\";s:15:\"allowdirectpost\";s:1:\"0\";s:14:\"allowgetattach\";s:1:\"0\";s:13:\"allowgetimage\";s:1:\"0\";s:15:\"allowpostattach\";s:1:\"0\";s:14:\"allowpostimage\";s:1:\"0\";s:9:\"allowvote\";s:1:\"0\";s:11:\"allowsearch\";s:1:\"0\";s:12:\"allowcstatus\";s:1:\"0\";s:14:\"allowinvisible\";s:1:\"0\";s:13:\"allowtransfer\";s:1:\"0\";s:16:\"allowsetreadperm\";s:1:\"0\";s:18:\"allowsetattachperm\";s:1:\"0\";s:12:\"allowposttag\";s:1:\"0\";s:13:\"allowhidecode\";s:1:\"0\";s:9:\"allowhtml\";s:1:\"0\";s:14:\"allowanonymous\";s:1:\"0\";s:14:\"allowsigbbcode\";s:1:\"0\";s:15:\"allowsigimgcode\";s:1:\"0\";s:11:\"allowmagics\";s:1:\"0\";s:17:\"disableperiodctrl\";s:1:\"0\";s:8:\"reasonpm\";s:1:\"0\";s:8:\"maxprice\";s:1:\"0\";s:10:\"maxsigsize\";s:1:\"0\";s:13:\"maxattachsize\";s:1:\"0\";s:13:\"maxsizeperday\";s:1:\"0\";s:17:\"maxthreadsperhour\";s:1:\"0\";s:15:\"maxpostsperhour\";s:1:\"0\";s:16:\"attachextensions\";s:0:\"\";s:9:\"raterange\";a:0:{}s:11:\"loginreward\";s:0:\"\";s:13:\"mintradeprice\";s:1:\"1\";s:13:\"maxtradeprice\";s:1:\"0\";s:14:\"minrewardprice\";s:1:\"1\";s:14:\"maxrewardprice\";s:1:\"0\";s:14:\"magicsdiscount\";s:1:\"0\";s:15:\"maxmagicsweight\";s:1:\"0\";s:15:\"allowpostdebate\";s:1:\"0\";s:10:\"tradestick\";s:1:\"5\";s:6:\"exempt\";s:1:\"0\";s:12:\"maxattachnum\";s:1:\"0\";s:12:\"allowposturl\";s:1:\"3\";s:14:\"allowrecommend\";s:1:\"1\";s:18:\"allowpostrushreply\";s:1:\"0\";s:12:\"maxfriendnum\";s:1:\"0\";s:12:\"maxspacesize\";s:1:\"0\";s:12:\"allowcomment\";s:1:\"0\";s:15:\"allowcommentmod\";s:1:\"0\";s:19:\"allowcommentarticle\";s:1:\"0\";s:22:\"allowcommentarticlemod\";s:1:\"0\";s:14:\"searchinterval\";s:1:\"0\";s:12:\"searchignore\";s:1:\"0\";s:9:\"allowblog\";s:1:\"0\";s:10:\"allowdoing\";s:1:\"0\";s:11:\"allowupload\";s:1:\"0\";s:10:\"allowshare\";s:1:\"0\";s:12:\"allowblogmod\";s:1:\"0\";s:13:\"allowdoingmod\";s:1:\"0\";s:14:\"allowuploadmod\";s:1:\"0\";s:13:\"allowsharemod\";s:1:\"0\";s:8:\"allowcss\";s:1:\"0\";s:9:\"allowpoke\";s:1:\"0\";s:11:\"allowfriend\";s:1:\"0\";s:10:\"allowclick\";s:1:\"0\";s:10:\"allowmagic\";s:1:\"0\";s:9:\"allowstat\";s:1:\"0\";s:13:\"allowstatdata\";s:1:\"0\";s:13:\"magicdiscount\";s:1:\"0\";s:12:\"domainlength\";s:1:\"0\";s:7:\"seccode\";s:1:\"1\";s:15:\"disablepostctrl\";s:1:\"0\";s:15:\"allowbuildgroup\";s:1:\"0\";s:20:\"allowgroupdirectpost\";s:1:\"3\";s:17:\"allowgroupposturl\";s:1:\"0\";s:13:\"edittimelimit\";s:1:\"0\";s:16:\"allowpostarticle\";s:1:\"0\";s:17:\"allowdownlocalimg\";s:1:\"0\";s:18:\"allowdownremoteimg\";s:1:\"0\";s:19:\"allowpostarticlemod\";s:1:\"0\";s:17:\"allowspacediyhtml\";s:1:\"0\";s:19:\"allowspacediybbcode\";s:1:\"0\";s:20:\"allowspacediyimgcode\";s:1:\"0\";s:16:\"allowcommentpost\";s:1:\"0\";s:16:\"allowcommentitem\";s:1:\"0\";s:17:\"allowcommentreply\";s:1:\"0\";s:16:\"allowreplycredit\";s:1:\"0\";s:12:\"ignorecensor\";s:1:\"0\";s:14:\"allowsendallpm\";s:1:\"0\";s:17:\"allowsendpmmaxnum\";s:1:\"0\";s:12:\"maximagesize\";s:1:\"0\";s:14:\"allowmediacode\";s:1:\"0\";s:14:\"allowbegincode\";s:1:\"0\";s:7:\"allowat\";s:1:\"0\";s:9:\"allowsave\";s:1:\"1\";s:14:\"allowsavereply\";s:1:\"1\";s:12:\"allowsavenum\";s:1:\"0\";s:19:\"allowsetpublishdate\";s:1:\"0\";s:21:\"allowfollowcollection\";s:1:\"0\";s:22:\"allowcommentcollection\";s:1:\"0\";s:21:\"allowcreatecollection\";s:1:\"0\";s:12:\"forcesecques\";s:1:\"0\";s:10:\"forcelogin\";s:1:\"0\";s:7:\"closead\";s:1:\"0\";s:17:\"buildgroupcredits\";s:1:\"0\";s:15:\"allowimgcontent\";s:1:\"0\";s:17:\"allowavatarupload\";s:1:\"0\";s:16:\"allowviewprofile\";s:1:\"0\";s:6:\"fields\";a:0:{}s:9:\"grouptype\";s:6:\"system\";s:11:\"grouppublic\";b:0;s:18:\"groupcreditshigher\";s:1:\"0\";s:17:\"groupcreditslower\";s:1:\"0\";s:17:\"allowthreadplugin\";a:0:{}s:6:\"plugin\";N;s:5:\"style\";N;}'),
('usergroup_7',1,1784453715,'a:137:{s:7:\"groupid\";s:1:\"7\";s:8:\"radminid\";s:1:\"0\";s:10:\"grouptitle\";s:6:\"娓稿\";s:5:\"stars\";s:1:\"0\";s:5:\"color\";s:0:\"\";s:4:\"icon\";s:0:\"\";s:10:\"allowvisit\";s:1:\"1\";s:11:\"allowsendpm\";s:1:\"1\";s:11:\"allowinvite\";s:1:\"0\";s:15:\"allowmailinvite\";s:1:\"0\";s:11:\"allowfollow\";s:1:\"0\";s:12:\"maxinvitenum\";s:1:\"0\";s:11:\"inviteprice\";s:1:\"0\";s:12:\"maxinviteday\";s:2:\"10\";s:9:\"upgroupid\";s:1:\"0\";s:14:\"creditsformula\";s:0:\"\";s:10:\"readaccess\";s:1:\"1\";s:9:\"allowpost\";s:1:\"0\";s:10:\"allowreply\";s:1:\"0\";s:13:\"allowpostpoll\";s:1:\"0\";s:15:\"allowpostreward\";s:1:\"0\";s:14:\"allowposttrade\";s:1:\"0\";s:17:\"allowpostactivity\";s:1:\"0\";s:15:\"allowdirectpost\";s:1:\"0\";s:14:\"allowgetattach\";s:1:\"0\";s:13:\"allowgetimage\";s:1:\"0\";s:15:\"allowpostattach\";s:1:\"0\";s:14:\"allowpostimage\";s:1:\"0\";s:9:\"allowvote\";s:1:\"0\";s:11:\"allowsearch\";s:2:\"19\";s:12:\"allowcstatus\";s:1:\"0\";s:14:\"allowinvisible\";s:1:\"0\";s:13:\"allowtransfer\";s:1:\"0\";s:16:\"allowsetreadperm\";s:1:\"0\";s:18:\"allowsetattachperm\";s:1:\"0\";s:12:\"allowposttag\";s:1:\"0\";s:13:\"allowhidecode\";s:1:\"0\";s:9:\"allowhtml\";s:1:\"0\";s:14:\"allowanonymous\";s:1:\"0\";s:14:\"allowsigbbcode\";s:1:\"0\";s:15:\"allowsigimgcode\";s:1:\"0\";s:11:\"allowmagics\";s:1:\"0\";s:17:\"disableperiodctrl\";s:1:\"0\";s:8:\"reasonpm\";s:1:\"0\";s:8:\"maxprice\";s:1:\"0\";s:10:\"maxsigsize\";s:1:\"0\";s:13:\"maxattachsize\";s:1:\"0\";s:13:\"maxsizeperday\";s:1:\"0\";s:17:\"maxthreadsperhour\";s:1:\"0\";s:15:\"maxpostsperhour\";s:1:\"0\";s:16:\"attachextensions\";s:19:\"gif, jpg, jpeg, png\";s:9:\"raterange\";a:0:{}s:11:\"loginreward\";s:0:\"\";s:13:\"mintradeprice\";s:1:\"1\";s:13:\"maxtradeprice\";s:1:\"0\";s:14:\"minrewardprice\";s:1:\"1\";s:14:\"maxrewardprice\";s:1:\"0\";s:14:\"magicsdiscount\";s:1:\"0\";s:15:\"maxmagicsweight\";s:1:\"0\";s:15:\"allowpostdebate\";s:1:\"0\";s:10:\"tradestick\";s:1:\"5\";s:6:\"exempt\";s:1:\"0\";s:12:\"maxattachnum\";s:1:\"0\";s:12:\"allowposturl\";s:1:\"3\";s:14:\"allowrecommend\";s:1:\"0\";s:18:\"allowpostrushreply\";s:1:\"0\";s:12:\"maxfriendnum\";s:1:\"0\";s:12:\"maxspacesize\";s:1:\"0\";s:12:\"allowcomment\";s:1:\"0\";s:15:\"allowcommentmod\";s:1:\"0\";s:19:\"allowcommentarticle\";s:1:\"0\";s:22:\"allowcommentarticlemod\";s:1:\"0\";s:14:\"searchinterval\";s:1:\"0\";s:12:\"searchignore\";s:1:\"0\";s:9:\"allowblog\";s:1:\"0\";s:10:\"allowdoing\";s:1:\"0\";s:11:\"allowupload\";s:1:\"0\";s:10:\"allowshare\";s:1:\"0\";s:12:\"allowblogmod\";s:1:\"0\";s:13:\"allowdoingmod\";s:1:\"0\";s:14:\"allowuploadmod\";s:1:\"0\";s:13:\"allowsharemod\";s:1:\"0\";s:8:\"allowcss\";s:1:\"0\";s:9:\"allowpoke\";s:1:\"0\";s:11:\"allowfriend\";s:1:\"0\";s:10:\"allowclick\";s:1:\"0\";s:10:\"allowmagic\";s:1:\"0\";s:9:\"allowstat\";s:1:\"0\";s:13:\"allowstatdata\";s:1:\"0\";s:13:\"magicdiscount\";s:1:\"0\";s:12:\"domainlength\";s:1:\"0\";s:7:\"seccode\";s:1:\"1\";s:15:\"disablepostctrl\";s:1:\"0\";s:15:\"allowbuildgroup\";s:1:\"0\";s:20:\"allowgroupdirectpost\";s:1:\"3\";s:17:\"allowgroupposturl\";s:1:\"0\";s:13:\"edittimelimit\";s:1:\"0\";s:16:\"allowpostarticle\";s:1:\"0\";s:17:\"allowdownlocalimg\";s:1:\"0\";s:18:\"allowdownremoteimg\";s:1:\"0\";s:19:\"allowpostarticlemod\";s:1:\"0\";s:17:\"allowspacediyhtml\";s:1:\"0\";s:19:\"allowspacediybbcode\";s:1:\"0\";s:20:\"allowspacediyimgcode\";s:1:\"0\";s:16:\"allowcommentpost\";s:1:\"0\";s:16:\"allowcommentitem\";s:1:\"0\";s:17:\"allowcommentreply\";s:1:\"0\";s:16:\"allowreplycredit\";s:1:\"0\";s:12:\"ignorecensor\";s:1:\"0\";s:14:\"allowsendallpm\";s:1:\"0\";s:17:\"allowsendpmmaxnum\";s:1:\"0\";s:12:\"maximagesize\";s:1:\"0\";s:14:\"allowmediacode\";s:1:\"0\";s:14:\"allowbegincode\";s:1:\"0\";s:7:\"allowat\";s:1:\"0\";s:9:\"allowsave\";s:1:\"1\";s:14:\"allowsavereply\";s:1:\"1\";s:12:\"allowsavenum\";s:1:\"0\";s:19:\"allowsetpublishdate\";s:1:\"0\";s:21:\"allowfollowcollection\";s:1:\"0\";s:22:\"allowcommentcollection\";s:1:\"0\";s:21:\"allowcreatecollection\";s:1:\"0\";s:12:\"forcesecques\";s:1:\"0\";s:10:\"forcelogin\";s:1:\"0\";s:7:\"closead\";s:1:\"0\";s:17:\"buildgroupcredits\";s:1:\"0\";s:15:\"allowimgcontent\";s:1:\"0\";s:17:\"allowavatarupload\";s:1:\"0\";s:16:\"allowviewprofile\";s:1:\"0\";s:6:\"fields\";a:0:{}s:9:\"grouptype\";s:6:\"system\";s:11:\"grouppublic\";b:0;s:18:\"groupcreditshigher\";s:1:\"0\";s:17:\"groupcreditslower\";s:1:\"0\";s:17:\"allowthreadplugin\";a:0:{}s:6:\"plugin\";N;s:5:\"style\";N;}'),
('usergroup_8',1,1784453715,'a:137:{s:7:\"groupid\";s:1:\"8\";s:8:\"radminid\";s:1:\"0\";s:10:\"grouptitle\";s:18:\"绛夊緟楠岃瘉浼氬憳\";s:5:\"stars\";s:1:\"0\";s:5:\"color\";s:0:\"\";s:4:\"icon\";s:0:\"\";s:10:\"allowvisit\";s:1:\"1\";s:11:\"allowsendpm\";s:1:\"1\";s:11:\"allowinvite\";s:1:\"0\";s:15:\"allowmailinvite\";s:1:\"0\";s:11:\"allowfollow\";s:1:\"0\";s:12:\"maxinvitenum\";s:1:\"0\";s:11:\"inviteprice\";s:1:\"0\";s:12:\"maxinviteday\";s:1:\"0\";s:9:\"upgroupid\";s:1:\"0\";s:14:\"creditsformula\";s:0:\"\";s:10:\"readaccess\";s:1:\"0\";s:9:\"allowpost\";s:1:\"0\";s:10:\"allowreply\";s:1:\"0\";s:13:\"allowpostpoll\";s:1:\"0\";s:15:\"allowpostreward\";s:1:\"0\";s:14:\"allowposttrade\";s:1:\"0\";s:17:\"allowpostactivity\";s:1:\"0\";s:15:\"allowdirectpost\";s:1:\"0\";s:14:\"allowgetattach\";s:1:\"0\";s:13:\"allowgetimage\";s:1:\"0\";s:15:\"allowpostattach\";s:1:\"0\";s:14:\"allowpostimage\";s:1:\"0\";s:9:\"allowvote\";s:1:\"0\";s:11:\"allowsearch\";s:1:\"2\";s:12:\"allowcstatus\";s:1:\"0\";s:14:\"allowinvisible\";s:1:\"0\";s:13:\"allowtransfer\";s:1:\"0\";s:16:\"allowsetreadperm\";s:1:\"0\";s:18:\"allowsetattachperm\";s:1:\"0\";s:12:\"allowposttag\";s:1:\"0\";s:13:\"allowhidecode\";s:1:\"0\";s:9:\"allowhtml\";s:1:\"0\";s:14:\"allowanonymous\";s:1:\"0\";s:14:\"allowsigbbcode\";s:1:\"1\";s:15:\"allowsigimgcode\";s:1:\"0\";s:11:\"allowmagics\";s:1:\"0\";s:17:\"disableperiodctrl\";s:1:\"0\";s:8:\"reasonpm\";s:1:\"0\";s:8:\"maxprice\";s:1:\"0\";s:10:\"maxsigsize\";s:2:\"50\";s:13:\"maxattachsize\";s:1:\"0\";s:13:\"maxsizeperday\";s:1:\"0\";s:17:\"maxthreadsperhour\";s:1:\"0\";s:15:\"maxpostsperhour\";s:1:\"0\";s:16:\"attachextensions\";s:0:\"\";s:9:\"raterange\";a:0:{}s:11:\"loginreward\";s:0:\"\";s:13:\"mintradeprice\";s:1:\"1\";s:13:\"maxtradeprice\";s:1:\"0\";s:14:\"minrewardprice\";s:1:\"1\";s:14:\"maxrewardprice\";s:1:\"0\";s:14:\"magicsdiscount\";s:1:\"0\";s:15:\"maxmagicsweight\";s:1:\"0\";s:15:\"allowpostdebate\";s:1:\"0\";s:10:\"tradestick\";s:1:\"5\";s:6:\"exempt\";s:1:\"0\";s:12:\"maxattachnum\";s:1:\"0\";s:12:\"allowposturl\";s:1:\"3\";s:14:\"allowrecommend\";s:1:\"1\";s:18:\"allowpostrushreply\";s:1:\"0\";s:12:\"maxfriendnum\";s:1:\"0\";s:12:\"maxspacesize\";s:1:\"0\";s:12:\"allowcomment\";s:1:\"0\";s:15:\"allowcommentmod\";s:1:\"0\";s:19:\"allowcommentarticle\";s:1:\"0\";s:22:\"allowcommentarticlemod\";s:1:\"0\";s:14:\"searchinterval\";s:1:\"0\";s:12:\"searchignore\";s:1:\"0\";s:9:\"allowblog\";s:1:\"0\";s:10:\"allowdoing\";s:1:\"0\";s:11:\"allowupload\";s:1:\"0\";s:10:\"allowshare\";s:1:\"0\";s:12:\"allowblogmod\";s:1:\"0\";s:13:\"allowdoingmod\";s:1:\"0\";s:14:\"allowuploadmod\";s:1:\"0\";s:13:\"allowsharemod\";s:1:\"0\";s:8:\"allowcss\";s:1:\"0\";s:9:\"allowpoke\";s:1:\"0\";s:11:\"allowfriend\";s:1:\"0\";s:10:\"allowclick\";s:1:\"0\";s:10:\"allowmagic\";s:1:\"0\";s:9:\"allowstat\";s:1:\"0\";s:13:\"allowstatdata\";s:1:\"0\";s:13:\"magicdiscount\";s:1:\"0\";s:12:\"domainlength\";s:1:\"0\";s:7:\"seccode\";s:1:\"1\";s:15:\"disablepostctrl\";s:1:\"0\";s:15:\"allowbuildgroup\";s:1:\"0\";s:20:\"allowgroupdirectpost\";s:1:\"3\";s:17:\"allowgroupposturl\";s:1:\"0\";s:13:\"edittimelimit\";s:1:\"0\";s:16:\"allowpostarticle\";s:1:\"0\";s:17:\"allowdownlocalimg\";s:1:\"0\";s:18:\"allowdownremoteimg\";s:1:\"0\";s:19:\"allowpostarticlemod\";s:1:\"0\";s:17:\"allowspacediyhtml\";s:1:\"0\";s:19:\"allowspacediybbcode\";s:1:\"0\";s:20:\"allowspacediyimgcode\";s:1:\"0\";s:16:\"allowcommentpost\";s:1:\"2\";s:16:\"allowcommentitem\";s:1:\"0\";s:17:\"allowcommentreply\";s:1:\"0\";s:16:\"allowreplycredit\";s:1:\"0\";s:12:\"ignorecensor\";s:1:\"0\";s:14:\"allowsendallpm\";s:1:\"0\";s:17:\"allowsendpmmaxnum\";s:1:\"0\";s:12:\"maximagesize\";s:1:\"0\";s:14:\"allowmediacode\";s:1:\"0\";s:14:\"allowbegincode\";s:1:\"0\";s:7:\"allowat\";s:1:\"0\";s:9:\"allowsave\";s:1:\"1\";s:14:\"allowsavereply\";s:1:\"1\";s:12:\"allowsavenum\";s:1:\"0\";s:19:\"allowsetpublishdate\";s:1:\"0\";s:21:\"allowfollowcollection\";s:1:\"0\";s:22:\"allowcommentcollection\";s:1:\"0\";s:21:\"allowcreatecollection\";s:1:\"0\";s:12:\"forcesecques\";s:1:\"0\";s:10:\"forcelogin\";s:1:\"0\";s:7:\"closead\";s:1:\"0\";s:17:\"buildgroupcredits\";s:1:\"0\";s:15:\"allowimgcontent\";s:1:\"0\";s:17:\"allowavatarupload\";s:1:\"0\";s:16:\"allowviewprofile\";s:1:\"0\";s:6:\"fields\";a:0:{}s:9:\"grouptype\";s:6:\"system\";s:11:\"grouppublic\";b:0;s:18:\"groupcreditshigher\";s:1:\"0\";s:17:\"groupcreditslower\";s:1:\"0\";s:17:\"allowthreadplugin\";a:0:{}s:6:\"plugin\";N;s:5:\"style\";N;}'),
('usergroup_9',1,1784453715,'a:137:{s:7:\"groupid\";s:1:\"9\";s:8:\"radminid\";s:1:\"0\";s:10:\"grouptitle\";s:12:\"闄愬埗浼氬憳\";s:5:\"stars\";s:1:\"0\";s:5:\"color\";s:0:\"\";s:4:\"icon\";s:0:\"\";s:10:\"allowvisit\";s:1:\"1\";s:11:\"allowsendpm\";s:1:\"1\";s:11:\"allowinvite\";s:1:\"0\";s:15:\"allowmailinvite\";s:1:\"0\";s:11:\"allowfollow\";s:1:\"0\";s:12:\"maxinvitenum\";s:1:\"0\";s:11:\"inviteprice\";s:1:\"0\";s:12:\"maxinviteday\";s:1:\"0\";s:9:\"upgroupid\";s:1:\"0\";s:14:\"creditsformula\";s:0:\"\";s:10:\"readaccess\";s:1:\"0\";s:9:\"allowpost\";s:1:\"0\";s:10:\"allowreply\";s:1:\"0\";s:13:\"allowpostpoll\";s:1:\"0\";s:15:\"allowpostreward\";s:1:\"0\";s:14:\"allowposttrade\";s:1:\"0\";s:17:\"allowpostactivity\";s:1:\"0\";s:15:\"allowdirectpost\";s:1:\"0\";s:14:\"allowgetattach\";s:1:\"0\";s:13:\"allowgetimage\";s:1:\"0\";s:15:\"allowpostattach\";s:1:\"0\";s:14:\"allowpostimage\";s:1:\"0\";s:9:\"allowvote\";s:1:\"0\";s:11:\"allowsearch\";s:1:\"0\";s:12:\"allowcstatus\";s:1:\"0\";s:14:\"allowinvisible\";s:1:\"0\";s:13:\"allowtransfer\";s:1:\"0\";s:16:\"allowsetreadperm\";s:1:\"0\";s:18:\"allowsetattachperm\";s:1:\"0\";s:12:\"allowposttag\";s:1:\"0\";s:13:\"allowhidecode\";s:1:\"0\";s:9:\"allowhtml\";s:1:\"0\";s:14:\"allowanonymous\";s:1:\"0\";s:14:\"allowsigbbcode\";s:1:\"0\";s:15:\"allowsigimgcode\";s:1:\"0\";s:11:\"allowmagics\";s:1:\"0\";s:17:\"disableperiodctrl\";s:1:\"0\";s:8:\"reasonpm\";s:1:\"0\";s:8:\"maxprice\";s:1:\"0\";s:10:\"maxsigsize\";s:1:\"0\";s:13:\"maxattachsize\";s:1:\"0\";s:13:\"maxsizeperday\";s:1:\"0\";s:17:\"maxthreadsperhour\";s:1:\"0\";s:15:\"maxpostsperhour\";s:1:\"0\";s:16:\"attachextensions\";s:45:\"chm,pdf,zip,rar,tar,gz,bzip2,gif,jpg,jpeg,png\";s:9:\"raterange\";a:0:{}s:11:\"loginreward\";s:0:\"\";s:13:\"mintradeprice\";s:1:\"1\";s:13:\"maxtradeprice\";s:1:\"0\";s:14:\"minrewardprice\";s:1:\"1\";s:14:\"maxrewardprice\";s:1:\"0\";s:14:\"magicsdiscount\";s:1:\"1\";s:15:\"maxmagicsweight\";s:1:\"0\";s:15:\"allowpostdebate\";s:1:\"0\";s:10:\"tradestick\";s:1:\"5\";s:6:\"exempt\";s:1:\"0\";s:12:\"maxattachnum\";s:1:\"0\";s:12:\"allowposturl\";s:1:\"0\";s:14:\"allowrecommend\";s:1:\"0\";s:18:\"allowpostrushreply\";s:1:\"0\";s:12:\"maxfriendnum\";s:1:\"0\";s:12:\"maxspacesize\";s:1:\"0\";s:12:\"allowcomment\";s:1:\"0\";s:15:\"allowcommentmod\";s:1:\"0\";s:19:\"allowcommentarticle\";s:1:\"0\";s:22:\"allowcommentarticlemod\";s:1:\"0\";s:14:\"searchinterval\";s:1:\"0\";s:12:\"searchignore\";s:1:\"0\";s:9:\"allowblog\";s:1:\"0\";s:10:\"allowdoing\";s:1:\"0\";s:11:\"allowupload\";s:1:\"0\";s:10:\"allowshare\";s:1:\"0\";s:12:\"allowblogmod\";s:1:\"0\";s:13:\"allowdoingmod\";s:1:\"0\";s:14:\"allowuploadmod\";s:1:\"0\";s:13:\"allowsharemod\";s:1:\"0\";s:8:\"allowcss\";s:1:\"0\";s:9:\"allowpoke\";s:1:\"0\";s:11:\"allowfriend\";s:1:\"0\";s:10:\"allowclick\";s:1:\"0\";s:10:\"allowmagic\";s:1:\"0\";s:9:\"allowstat\";s:1:\"0\";s:13:\"allowstatdata\";s:1:\"0\";s:13:\"magicdiscount\";s:1:\"0\";s:12:\"domainlength\";s:1:\"0\";s:7:\"seccode\";s:1:\"1\";s:15:\"disablepostctrl\";s:1:\"0\";s:15:\"allowbuildgroup\";s:1:\"0\";s:20:\"allowgroupdirectpost\";s:1:\"0\";s:17:\"allowgroupposturl\";s:1:\"0\";s:13:\"edittimelimit\";s:1:\"0\";s:16:\"allowpostarticle\";s:1:\"0\";s:17:\"allowdownlocalimg\";s:1:\"0\";s:18:\"allowdownremoteimg\";s:1:\"0\";s:19:\"allowpostarticlemod\";s:1:\"0\";s:17:\"allowspacediyhtml\";s:1:\"0\";s:19:\"allowspacediybbcode\";s:1:\"0\";s:20:\"allowspacediyimgcode\";s:1:\"0\";s:16:\"allowcommentpost\";s:1:\"0\";s:16:\"allowcommentitem\";s:1:\"1\";s:17:\"allowcommentreply\";s:1:\"0\";s:16:\"allowreplycredit\";s:1:\"0\";s:12:\"ignorecensor\";s:1:\"0\";s:14:\"allowsendallpm\";s:1:\"0\";s:17:\"allowsendpmmaxnum\";s:1:\"0\";s:12:\"maximagesize\";s:1:\"0\";s:14:\"allowmediacode\";s:1:\"0\";s:14:\"allowbegincode\";s:1:\"0\";s:7:\"allowat\";s:1:\"0\";s:9:\"allowsave\";s:1:\"1\";s:14:\"allowsavereply\";s:1:\"1\";s:12:\"allowsavenum\";s:1:\"0\";s:19:\"allowsetpublishdate\";s:1:\"0\";s:21:\"allowfollowcollection\";s:1:\"0\";s:22:\"allowcommentcollection\";s:1:\"0\";s:21:\"allowcreatecollection\";s:1:\"0\";s:12:\"forcesecques\";s:1:\"0\";s:10:\"forcelogin\";s:1:\"0\";s:7:\"closead\";s:1:\"0\";s:17:\"buildgroupcredits\";s:1:\"0\";s:15:\"allowimgcontent\";s:1:\"0\";s:17:\"allowavatarupload\";s:1:\"0\";s:16:\"allowviewprofile\";s:1:\"0\";s:6:\"fields\";a:0:{}s:9:\"grouptype\";s:6:\"member\";s:11:\"grouppublic\";b:0;s:18:\"groupcreditshigher\";s:8:\"-9999999\";s:17:\"groupcreditslower\";s:1:\"0\";s:17:\"allowthreadplugin\";a:0:{}s:6:\"plugin\";N;s:5:\"style\";N;}'),
('usergroups',1,1784453715,'a:19:{i:9;a:15:{s:9:\"upgroupid\";s:1:\"0\";s:4:\"type\";s:6:\"member\";s:10:\"grouptitle\";s:12:\"闄愬埗浼氬憳\";s:13:\"creditshigher\";s:8:\"-9999999\";s:12:\"creditslower\";s:1:\"0\";s:5:\"stars\";s:1:\"0\";s:5:\"color\";s:0:\"\";s:4:\"icon\";s:0:\"\";s:10:\"readaccess\";s:1:\"0\";s:14:\"allowgetattach\";s:1:\"0\";s:13:\"allowgetimage\";s:1:\"0\";s:14:\"allowmediacode\";s:1:\"0\";s:10:\"maxsigsize\";s:1:\"0\";s:14:\"allowbegincode\";s:1:\"0\";s:12:\"userstatusby\";i:1;}i:1;a:13:{s:9:\"upgroupid\";s:1:\"0\";s:4:\"type\";s:6:\"system\";s:10:\"grouptitle\";s:9:\"绠＄悊鍛榎";s:5:\"stars\";s:1:\"9\";s:5:\"color\";s:0:\"\";s:4:\"icon\";s:0:\"\";s:10:\"readaccess\";s:3:\"200\";s:14:\"allowgetattach\";s:1:\"1\";s:13:\"allowgetimage\";s:1:\"1\";s:14:\"allowmediacode\";s:1:\"1\";s:10:\"maxsigsize\";s:3:\"500\";s:14:\"allowbegincode\";s:1:\"1\";s:12:\"userstatusby\";i:1;}i:2;a:13:{s:9:\"upgroupid\";s:1:\"0\";s:4:\"type\";s:6:\"system\";s:10:\"grouptitle\";s:12:\"瓒呯骇鐗堜富\";s:5:\"stars\";s:1:\"8\";s:5:\"color\";s:0:\"\";s:4:\"icon\";s:0:\"\";s:10:\"readaccess\";s:3:\"150\";s:14:\"allowgetattach\";s:1:\"1\";s:13:\"allowgetimage\";s:1:\"1\";s:14:\"allowmediacode\";s:1:\"0\";s:10:\"maxsigsize\";s:3:\"300\";s:14:\"allowbegincode\";s:1:\"0\";s:12:\"userstatusby\";i:1;}i:3;a:13:{s:9:\"upgroupid\";s:1:\"0\";s:4:\"type\";s:6:\"system\";s:10:\"grouptitle\";s:6:\"鐗堜富\";s:5:\"stars\";s:1:\"7\";s:5:\"color\";s:0:\"\";s:4:\"icon\";s:0:\"\";s:10:\"readaccess\";s:3:\"100\";s:14:\"allowgetattach\";s:1:\"1\";s:13:\"allowgetimage\";s:1:\"1\";s:14:\"allowmediacode\";s:1:\"0\";s:10:\"maxsigsize\";s:3:\"200\";s:14:\"allowbegincode\";s:1:\"0\";s:12:\"userstatusby\";i:1;}i:4;a:13:{s:9:\"upgroupid\";s:1:\"0\";s:4:\"type\";s:6:\"system\";s:10:\"grouptitle\";s:12:\"绂佹鍙戣█\";s:5:\"stars\";s:1:\"0\";s:5:\"color\";s:0:\"\";s:4:\"icon\";s:0:\"\";s:10:\"readaccess\";s:1:\"0\";s:14:\"allowgetattach\";s:1:\"0\";s:13:\"allowgetimage\";s:1:\"0\";s:14:\"allowmediacode\";s:1:\"0\";s:10:\"maxsigsize\";s:1:\"0\";s:14:\"allowbegincode\";s:1:\"0\";s:12:\"userstatusby\";i:1;}i:5;a:13:{s:9:\"upgroupid\";s:1:\"0\";s:4:\"type\";s:6:\"system\";s:10:\"grouptitle\";s:12:\"绂佹璁块棶\";s:5:\"stars\";s:1:\"0\";s:5:\"color\";s:0:\"\";s:4:\"icon\";s:0:\"\";s:10:\"readaccess\";s:1:\"0\";s:14:\"allowgetattach\";s:1:\"0\";s:13:\"allowgetimage\";s:1:\"0\";s:14:\"allowmediacode\";s:1:\"0\";s:10:\"maxsigsize\";s:1:\"0\";s:14:\"allowbegincode\";s:1:\"0\";s:12:\"userstatusby\";i:1;}i:6;a:13:{s:9:\"upgroupid\";s:1:\"0\";s:4:\"type\";s:6:\"system\";s:10:\"grouptitle\";s:9:\"绂佹 IP\";s:5:\"stars\";s:1:\"0\";s:5:\"color\";s:0:\"\";s:4:\"icon\";s:0:\"\";s:10:\"readaccess\";s:1:\"0\";s:14:\"allowgetattach\";s:1:\"0\";s:13:\"allowgetimage\";s:1:\"0\";s:14:\"allowmediacode\";s:1:\"0\";s:10:\"maxsigsize\";s:1:\"0\";s:14:\"allowbegincode\";s:1:\"0\";s:12:\"userstatusby\";i:1;}i:7;a:13:{s:9:\"upgroupid\";s:1:\"0\";s:4:\"type\";s:6:\"system\";s:10:\"grouptitle\";s:6:\"娓稿\";s:5:\"stars\";s:1:\"0\";s:5:\"color\";s:0:\"\";s:4:\"icon\";s:0:\"\";s:10:\"readaccess\";s:1:\"1\";s:14:\"allowgetattach\";s:1:\"0\";s:13:\"allowgetimage\";s:1:\"0\";s:14:\"allowmediacode\";s:1:\"0\";s:10:\"maxsigsize\";s:1:\"0\";s:14:\"allowbegincode\";s:1:\"0\";s:12:\"userstatusby\";i:1;}i:8;a:13:{s:9:\"upgroupid\";s:1:\"0\";s:4:\"type\";s:6:\"system\";s:10:\"grouptitle\";s:18:\"绛夊緟楠岃瘉浼氬憳\";s:5:\"stars\";s:1:\"0\";s:5:\"color\";s:0:\"\";s:4:\"icon\";s:0:\"\";s:10:\"readaccess\";s:1:\"0\";s:14:\"allowgetattach\";s:1:\"0\";s:13:\"allowgetimage\";s:1:\"0\";s:14:\"allowmediacode\";s:1:\"0\";s:10:\"maxsigsize\";s:2:\"50\";s:14:\"allowbegincode\";s:1:\"0\";s:12:\"userstatusby\";i:1;}i:10;a:15:{s:9:\"upgroupid\";s:1:\"0\";s:4:\"type\";s:6:\"member\";s:10:\"grouptitle\";s:12:\"鏂版墜涓婅矾\";s:13:\"creditshigher\";s:1:\"0\";s:12:\"creditslower\";s:2:\"50\";s:5:\"stars\";s:1:\"1\";s:5:\"color\";s:0:\"\";s:4:\"icon\";s:0:\"\";s:10:\"readaccess\";s:2:\"10\";s:14:\"allowgetattach\";s:1:\"1\";s:13:\"allowgetimage\";s:1:\"1\";s:14:\"allowmediacode\";s:1:\"0\";s:10:\"maxsigsize\";s:2:\"80\";s:14:\"allowbegincode\";s:1:\"0\";s:12:\"userstatusby\";i:1;}i:16;a:13:{s:9:\"upgroupid\";s:1:\"0\";s:4:\"type\";s:7:\"special\";s:10:\"grouptitle\";s:12:\"瀹炰範鐗堜富\";s:5:\"stars\";s:1:\"7\";s:5:\"color\";s:0:\"\";s:4:\"icon\";s:0:\"\";s:10:\"readaccess\";s:3:\"100\";s:14:\"allowgetattach\";s:1:\"1\";s:13:\"allowgetimage\";s:1:\"1\";s:14:\"allowmediacode\";s:1:\"0\";s:10:\"maxsigsize\";s:3:\"200\";s:14:\"allowbegincode\";s:1:\"0\";s:12:\"userstatusby\";i:1;}i:17;a:13:{s:9:\"upgroupid\";s:1:\"0\";s:4:\"type\";s:7:\"special\";s:10:\"grouptitle\";s:12:\"缃戠珯缂栬緫\";s:5:\"stars\";s:1:\"8\";s:5:\"color\";s:0:\"\";s:4:\"icon\";s:0:\"\";s:10:\"readaccess\";s:3:\"150\";s:14:\"allowgetattach\";s:1:\"1\";s:13:\"allowgetimage\";s:1:\"1\";s:14:\"allowmediacode\";s:1:\"0\";s:10:\"maxsigsize\";s:3:\"300\";s:14:\"allowbegincode\";s:1:\"0\";s:12:\"userstatusby\";i:1;}i:18;a:13:{s:9:\"upgroupid\";s:1:\"0\";s:4:\"type\";s:7:\"special\";s:10:\"grouptitle\";s:15:\"淇℃伅鐩戝療鍛榎";s:5:\"stars\";s:1:\"9\";s:5:\"color\";s:0:\"\";s:4:\"icon\";s:0:\"\";s:10:\"readaccess\";s:3:\"200\";s:14:\"allowgetattach\";s:1:\"1\";s:13:\"allowgetimage\";s:1:\"1\";s:14:\"allowmediacode\";s:1:\"0\";s:10:\"maxsigsize\";s:3:\"500\";s:14:\"allowbegincode\";s:1:\"0\";s:12:\"userstatusby\";i:1;}i:19;a:13:{s:9:\"upgroupid\";s:1:\"0\";s:4:\"type\";s:7:\"special\";s:10:\"grouptitle\";s:9:\"瀹℃牳鍛榎";s:5:\"stars\";s:1:\"7\";s:5:\"color\";s:0:\"\";s:4:\"icon\";s:0:\"\";s:10:\"readaccess\";s:3:\"100\";s:14:\"allowgetattach\";s:1:\"1\";s:13:\"allowgetimage\";s:1:\"1\";s:14:\"allowmediacode\";s:1:\"0\";s:10:\"maxsigsize\";s:3:\"200\";s:14:\"allowbegincode\";s:1:\"0\";s:12:\"userstatusby\";i:1;}i:11;a:15:{s:9:\"upgroupid\";s:1:\"0\";s:4:\"type\";s:6:\"member\";s:10:\"grouptitle\";s:12:\"娉ㄥ唽浼氬憳\";s:13:\"creditshigher\";s:2:\"50\";s:12:\"creditslower\";s:3:\"200\";s:5:\"stars\";s:1:\"2\";s:5:\"color\";s:0:\"\";s:4:\"icon\";s:0:\"\";s:10:\"readaccess\";s:2:\"20\";s:14:\"allowgetattach\";s:1:\"1\";s:13:\"allowgetimage\";s:1:\"1\";s:14:\"allowmediacode\";s:1:\"0\";s:10:\"maxsigsize\";s:3:\"100\";s:14:\"allowbegincode\";s:1:\"0\";s:12:\"userstatusby\";i:1;}i:12;a:15:{s:9:\"upgroupid\";s:1:\"0\";s:4:\"type\";s:6:\"member\";s:10:\"grouptitle\";s:12:\"涓骇浼氬憳\";s:13:\"creditshigher\";s:3:\"200\";s:12:\"creditslower\";s:3:\"500\";s:5:\"stars\";s:1:\"3\";s:5:\"color\";s:0:\"\";s:4:\"icon\";s:0:\"\";s:10:\"readaccess\";s:2:\"30\";s:14:\"allowgetattach\";s:1:\"1\";s:13:\"allowgetimage\";s:1:\"1\";s:14:\"allowmediacode\";s:1:\"0\";s:10:\"maxsigsize\";s:3:\"150\";s:14:\"allowbegincode\";s:1:\"0\";s:12:\"userstatusby\";i:1;}i:13;a:15:{s:9:\"upgroupid\";s:1:\"0\";s:4:\"type\";s:6:\"member\";s:10:\"grouptitle\";s:12:\"楂樼骇浼氬憳\";s:13:\"creditshigher\";s:3:\"500\";s:12:\"creditslower\";s:4:\"1000\";s:5:\"stars\";s:1:\"4\";s:5:\"color\";s:0:\"\";s:4:\"icon\";s:0:\"\";s:10:\"readaccess\";s:2:\"50\";s:14:\"allowgetattach\";s:1:\"1\";s:13:\"allowgetimage\";s:1:\"1\";s:14:\"allowmediacode\";s:1:\"0\";s:10:\"maxsigsize\";s:3:\"200\";s:14:\"allowbegincode\";s:1:\"0\";s:12:\"userstatusby\";i:1;}i:14;a:15:{s:9:\"upgroupid\";s:1:\"0\";s:4:\"type\";s:6:\"member\";s:10:\"grouptitle\";s:12:\"閲戠墝浼氬憳\";s:13:\"creditshigher\";s:4:\"1000\";s:12:\"creditslower\";s:4:\"3000\";s:5:\"stars\";s:1:\"6\";s:5:\"color\";s:0:\"\";s:4:\"icon\";s:0:\"\";s:10:\"readaccess\";s:2:\"70\";s:14:\"allowgetattach\";s:1:\"1\";s:13:\"allowgetimage\";s:1:\"1\";s:14:\"allowmediacode\";s:1:\"0\";s:10:\"maxsigsize\";s:3:\"300\";s:14:\"allowbegincode\";s:1:\"0\";s:12:\"userstatusby\";i:1;}i:15;a:15:{s:9:\"upgroupid\";s:1:\"0\";s:4:\"type\";s:6:\"member\";s:10:\"grouptitle\";s:12:\"璁哄潧鍏冭€乗";s:13:\"creditshigher\";s:4:\"3000\";s:12:\"creditslower\";s:7:\"9999999\";s:5:\"stars\";s:1:\"8\";s:5:\"color\";s:0:\"\";s:4:\"icon\";s:0:\"\";s:10:\"readaccess\";s:2:\"90\";s:14:\"allowgetattach\";s:1:\"1\";s:13:\"allowgetimage\";s:1:\"1\";s:14:\"allowmediacode\";s:1:\"0\";s:10:\"maxsigsize\";s:3:\"500\";s:14:\"allowbegincode\";s:1:\"0\";s:12:\"userstatusby\";i:1;}}'),
('userreasons',1,1784453715,'a:5:{i:0;s:10:\"寰堢粰鍔?\";i:1;s:18:\"绁為┈閮芥槸娴簯\";i:2;s:10:\"璧炰竴涓?\";i:3;s:6:\"灞卞\";i:4;s:6:\"娣″畾\";}'),
('userstats',1,1784453715,'a:2:{s:12:\"totalmembers\";s:1:\"1\";s:10:\"newsetuser\";s:5:\"admin\";}');
/*!40000 ALTER TABLE `pre_common_syscache` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_common_tag`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_common_tag` (
  `tagid` mediumint(8) unsigned NOT NULL AUTO_INCREMENT,
  `tagname` char(50) NOT NULL DEFAULT '',
  `status` tinyint(1) NOT NULL DEFAULT 0,
  `related_count` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `hot_score` float NOT NULL DEFAULT 0,
  `created_at` int(10) unsigned NOT NULL DEFAULT 0,
  `updated_at` int(10) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`tagid`),
  KEY `tagname` (`tagname`),
  KEY `status` (`status`,`tagid`),
  KEY `idx_hot_score` (`hot_score`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_common_tag` WRITE;
/*!40000 ALTER TABLE `pre_common_tag` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_common_tag` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_common_tagitem`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_common_tagitem` (
  `tagid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `itemid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `idtype` char(10) NOT NULL DEFAULT '',
  `created_at` int(10) unsigned NOT NULL DEFAULT 0,
  UNIQUE KEY `item` (`tagid`,`itemid`,`idtype`),
  KEY `idtype` (`idtype`,`itemid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_common_tagitem` WRITE;
/*!40000 ALTER TABLE `pre_common_tagitem` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_common_tagitem` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_common_task`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_common_task` (
  `taskid` smallint(6) unsigned NOT NULL AUTO_INCREMENT,
  `relatedtaskid` smallint(6) unsigned NOT NULL DEFAULT 0,
  `exclusivetaskid` smallint(6) unsigned NOT NULL DEFAULT 0,
  `available` tinyint(1) NOT NULL DEFAULT 0,
  `name` varchar(50) NOT NULL DEFAULT '',
  `description` text NOT NULL,
  `icon` varchar(150) NOT NULL DEFAULT '',
  `applicants` int(10) unsigned NOT NULL DEFAULT 0,
  `achievers` int(10) unsigned NOT NULL DEFAULT 0,
  `tasklimits` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `applyperm` text NOT NULL,
  `scriptname` varchar(50) NOT NULL DEFAULT '',
  `starttime` int(10) unsigned NOT NULL DEFAULT 0,
  `endtime` int(10) unsigned NOT NULL DEFAULT 0,
  `period` int(10) unsigned NOT NULL DEFAULT 0,
  `periodtype` tinyint(1) NOT NULL DEFAULT 0,
  `reward` enum('credit','magic','medal','invite','group') NOT NULL DEFAULT 'credit',
  `prize` varchar(50) NOT NULL DEFAULT '',
  `bonus` int(10) NOT NULL DEFAULT 0,
  `displayorder` smallint(6) unsigned NOT NULL DEFAULT 0,
  `version` varchar(15) NOT NULL DEFAULT '',
  PRIMARY KEY (`taskid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_common_task` WRITE;
/*!40000 ALTER TABLE `pre_common_task` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_common_task` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_common_taskvar`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_common_taskvar` (
  `taskvarid` mediumint(8) unsigned NOT NULL AUTO_INCREMENT,
  `taskid` smallint(6) unsigned NOT NULL DEFAULT 0,
  `sort` enum('apply','complete') NOT NULL DEFAULT 'complete',
  `name` varchar(100) NOT NULL DEFAULT '',
  `description` varchar(255) NOT NULL DEFAULT '',
  `variable` varchar(40) NOT NULL DEFAULT '',
  `type` varchar(20) NOT NULL DEFAULT 'text',
  `value` text NOT NULL,
  PRIMARY KEY (`taskvarid`),
  KEY `taskid` (`taskid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_common_taskvar` WRITE;
/*!40000 ALTER TABLE `pre_common_taskvar` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_common_taskvar` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_common_template`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_common_template` (
  `templateid` smallint(6) unsigned NOT NULL AUTO_INCREMENT,
  `name` varchar(30) NOT NULL DEFAULT '',
  `directory` varchar(100) NOT NULL DEFAULT '',
  `copyright` varchar(100) NOT NULL DEFAULT '',
  PRIMARY KEY (`templateid`)
) ENGINE=InnoDB AUTO_INCREMENT=3 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_common_template` WRITE;
/*!40000 ALTER TABLE `pre_common_template` DISABLE KEYS */;
INSERT INTO `pre_common_template` VALUES
(1,'榛樿妯℃澘濂楃郴','./template/default','Discuz!'),
(2,'X5妯＄増','./template/discuzx5','Discuz!');
/*!40000 ALTER TABLE `pre_common_template` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_common_template_block`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_common_template_block` (
  `targettplname` varchar(100) NOT NULL DEFAULT '',
  `tpldirectory` varchar(80) NOT NULL DEFAULT '',
  `bid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`targettplname`,`tpldirectory`,`bid`),
  KEY `bid` (`bid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_common_template_block` WRITE;
/*!40000 ALTER TABLE `pre_common_template_block` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_common_template_block` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_common_template_permission`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_common_template_permission` (
  `targettplname` varchar(100) NOT NULL DEFAULT '',
  `uid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `allowmanage` tinyint(1) NOT NULL DEFAULT 0,
  `allowrecommend` tinyint(1) NOT NULL DEFAULT 0,
  `needverify` tinyint(1) NOT NULL DEFAULT 0,
  `inheritedtplname` varchar(255) NOT NULL DEFAULT '',
  PRIMARY KEY (`targettplname`,`uid`),
  KEY `uid` (`uid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_common_template_permission` WRITE;
/*!40000 ALTER TABLE `pre_common_template_permission` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_common_template_permission` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_common_uin_black`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_common_uin_black` (
  `uin` char(40) NOT NULL,
  `uid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`uin`),
  UNIQUE KEY `uid` (`uid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_common_uin_black` WRITE;
/*!40000 ALTER TABLE `pre_common_uin_black` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_common_uin_black` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_common_usergroup`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_common_usergroup` (
  `groupid` smallint(6) unsigned NOT NULL AUTO_INCREMENT,
  `radminid` tinyint(3) NOT NULL DEFAULT 0,
  `type` enum('system','special','member') NOT NULL DEFAULT 'member',
  `system` varchar(255) NOT NULL DEFAULT 'private',
  `grouptitle` varchar(255) NOT NULL DEFAULT '',
  `creditshigher` int(10) NOT NULL DEFAULT 0,
  `creditslower` int(10) NOT NULL DEFAULT 0,
  `stars` tinyint(3) NOT NULL DEFAULT 0,
  `color` varchar(255) NOT NULL DEFAULT '',
  `icon` varchar(255) NOT NULL DEFAULT '',
  `allowvisit` tinyint(1) NOT NULL DEFAULT 0,
  `allowsendpm` tinyint(1) NOT NULL DEFAULT 1,
  `allowinvite` tinyint(1) NOT NULL DEFAULT 0,
  `allowmailinvite` tinyint(1) NOT NULL DEFAULT 0,
  `allowfollow` tinyint(1) NOT NULL DEFAULT 0,
  `maxinvitenum` tinyint(3) unsigned NOT NULL DEFAULT 0,
  `inviteprice` smallint(6) unsigned NOT NULL DEFAULT 0,
  `maxinviteday` smallint(6) unsigned NOT NULL DEFAULT 0,
  `upgroupid` smallint(6) unsigned DEFAULT 0,
  `creditsformula` varchar(255) NOT NULL DEFAULT '',
  PRIMARY KEY (`groupid`),
  KEY `creditsrange` (`creditshigher`,`creditslower`),
  KEY `upgroupid` (`upgroupid`)
) ENGINE=InnoDB AUTO_INCREMENT=20 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_common_usergroup` WRITE;
/*!40000 ALTER TABLE `pre_common_usergroup` DISABLE KEYS */;
INSERT INTO `pre_common_usergroup` VALUES
(1,1,'system','private','绠＄悊鍛?,0,0,9,'','',1,1,1,1,1,0,0,10,0,''),
(2,2,'system','private','瓒呯骇鐗堜富',0,0,8,'','',1,1,1,1,1,0,0,10,0,''),
(3,3,'system','private','鐗堜富',0,0,7,'','',1,1,1,1,1,0,0,10,0,''),
(4,0,'system','private','绂佹鍙戣█',0,0,0,'','',1,1,0,0,0,0,0,0,0,''),
(5,0,'system','private','绂佹璁块棶',0,0,0,'','',0,1,0,0,0,0,0,0,0,''),
(6,0,'system','private','绂佹 IP',0,0,0,'','',0,1,0,0,0,0,0,0,0,''),
(7,0,'system','private','娓稿',0,0,0,'','',1,1,0,0,0,0,0,10,0,''),
(8,0,'system','private','绛夊緟楠岃瘉浼氬憳',0,0,0,'','',1,1,0,0,0,0,0,0,0,''),
(9,0,'member','private','闄愬埗浼氬憳',-9999999,0,0,'','',1,1,0,0,0,0,0,0,0,''),
(10,0,'member','private','鏂版墜涓婅矾',0,50,1,'','',1,1,0,0,0,0,0,10,0,''),
(11,0,'member','private','娉ㄥ唽浼氬憳',50,200,2,'','',1,1,0,0,0,0,0,10,0,''),
(12,0,'member','private','涓骇浼氬憳',200,500,3,'','',1,1,0,0,0,0,0,10,0,''),
(13,0,'member','private','楂樼骇浼氬憳',500,1000,4,'','',1,1,0,0,1,0,0,10,0,''),
(14,0,'member','private','閲戠墝浼氬憳',1000,3000,6,'','',1,1,0,0,1,0,0,10,0,''),
(15,0,'member','private','璁哄潧鍏冭€?,3000,9999999,8,'','',1,1,0,0,1,0,0,10,0,''),
(16,3,'special','private','瀹炰範鐗堜富',0,0,7,'','',1,1,0,0,1,0,0,10,0,''),
(17,2,'special','private','缃戠珯缂栬緫',0,0,8,'','',1,1,0,0,1,0,0,10,0,''),
(18,1,'special','private','淇℃伅鐩戝療鍛?,0,0,9,'','',1,1,0,0,1,0,0,10,0,''),
(19,3,'special','private','瀹℃牳鍛?,0,0,7,'','',1,1,0,0,1,0,0,10,0,'');
/*!40000 ALTER TABLE `pre_common_usergroup` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_common_usergroup_field`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_common_usergroup_field` (
  `groupid` smallint(6) unsigned NOT NULL,
  `readaccess` tinyint(3) unsigned NOT NULL DEFAULT 0,
  `allowpost` tinyint(1) NOT NULL DEFAULT 0,
  `allowreply` tinyint(1) NOT NULL DEFAULT 0,
  `allowpostpoll` tinyint(1) NOT NULL DEFAULT 0,
  `allowpostreward` tinyint(1) NOT NULL DEFAULT 0,
  `allowposttrade` tinyint(1) NOT NULL DEFAULT 0,
  `allowpostactivity` tinyint(1) NOT NULL DEFAULT 0,
  `allowdirectpost` tinyint(1) NOT NULL DEFAULT 0,
  `allowgetattach` tinyint(1) NOT NULL DEFAULT 0,
  `allowgetimage` tinyint(1) NOT NULL DEFAULT 0,
  `allowpostattach` tinyint(1) NOT NULL DEFAULT 0,
  `allowpostimage` tinyint(1) NOT NULL DEFAULT 0,
  `allowvote` tinyint(1) NOT NULL DEFAULT 0,
  `allowsearch` tinyint(1) NOT NULL DEFAULT 0,
  `allowcstatus` tinyint(1) NOT NULL DEFAULT 0,
  `allowinvisible` tinyint(1) NOT NULL DEFAULT 0,
  `allowtransfer` tinyint(1) NOT NULL DEFAULT 0,
  `allowsetreadperm` tinyint(1) NOT NULL DEFAULT 0,
  `allowsetattachperm` tinyint(1) NOT NULL DEFAULT 0,
  `allowposttag` tinyint(1) NOT NULL DEFAULT 0,
  `allowhidecode` tinyint(1) NOT NULL DEFAULT 0,
  `allowhtml` tinyint(1) NOT NULL DEFAULT 0,
  `allowanonymous` tinyint(1) NOT NULL DEFAULT 0,
  `allowsigbbcode` tinyint(1) NOT NULL DEFAULT 0,
  `allowsigimgcode` tinyint(1) NOT NULL DEFAULT 0,
  `allowmagics` tinyint(3) unsigned NOT NULL,
  `disableperiodctrl` tinyint(1) NOT NULL DEFAULT 0,
  `reasonpm` tinyint(1) NOT NULL DEFAULT 0,
  `maxprice` smallint(6) unsigned NOT NULL DEFAULT 0,
  `maxsigsize` smallint(6) unsigned NOT NULL DEFAULT 0,
  `maxattachsize` int(10) unsigned NOT NULL DEFAULT 0,
  `maxsizeperday` int(10) unsigned NOT NULL DEFAULT 0,
  `maxthreadsperhour` tinyint(3) unsigned NOT NULL DEFAULT 0,
  `maxpostsperhour` tinyint(3) unsigned NOT NULL DEFAULT 0,
  `attachextensions` char(100) NOT NULL DEFAULT '',
  `raterange` char(150) NOT NULL DEFAULT '',
  `loginreward` char(150) NOT NULL DEFAULT '',
  `mintradeprice` smallint(6) unsigned NOT NULL DEFAULT 1,
  `maxtradeprice` smallint(6) unsigned NOT NULL DEFAULT 0,
  `minrewardprice` smallint(6) unsigned NOT NULL DEFAULT 1,
  `maxrewardprice` smallint(6) unsigned NOT NULL DEFAULT 0,
  `magicsdiscount` tinyint(1) NOT NULL,
  `maxmagicsweight` smallint(6) unsigned NOT NULL,
  `allowpostdebate` tinyint(1) NOT NULL DEFAULT 0,
  `tradestick` tinyint(3) unsigned NOT NULL,
  `exempt` tinyint(3) unsigned NOT NULL,
  `maxattachnum` smallint(6) NOT NULL DEFAULT 0,
  `allowposturl` tinyint(1) NOT NULL DEFAULT 3,
  `allowrecommend` tinyint(3) unsigned NOT NULL DEFAULT 1,
  `allowpostrushreply` tinyint(1) NOT NULL DEFAULT 0,
  `maxfriendnum` smallint(6) unsigned NOT NULL DEFAULT 0,
  `maxspacesize` int(10) unsigned NOT NULL DEFAULT 0,
  `allowcomment` tinyint(1) NOT NULL DEFAULT 0,
  `allowcommentmod` tinyint(1) NOT NULL DEFAULT 0,
  `allowcommentarticle` smallint(6) NOT NULL DEFAULT 0,
  `allowcommentarticlemod` tinyint(1) NOT NULL DEFAULT 0,
  `searchinterval` smallint(6) unsigned NOT NULL DEFAULT 0,
  `searchignore` tinyint(1) NOT NULL DEFAULT 0,
  `allowblog` tinyint(1) NOT NULL DEFAULT 0,
  `allowdoing` tinyint(1) NOT NULL DEFAULT 0,
  `allowupload` tinyint(1) NOT NULL DEFAULT 0,
  `allowshare` tinyint(1) NOT NULL DEFAULT 0,
  `allowblogmod` tinyint(1) NOT NULL DEFAULT 0,
  `allowdoingmod` tinyint(1) NOT NULL DEFAULT 0,
  `allowuploadmod` tinyint(1) NOT NULL DEFAULT 0,
  `allowsharemod` tinyint(1) NOT NULL DEFAULT 0,
  `allowcss` tinyint(1) NOT NULL DEFAULT 0,
  `allowpoke` tinyint(1) NOT NULL DEFAULT 0,
  `allowfriend` tinyint(1) NOT NULL DEFAULT 0,
  `allowclick` tinyint(1) NOT NULL DEFAULT 0,
  `allowmagic` tinyint(1) NOT NULL DEFAULT 0,
  `allowstat` tinyint(1) NOT NULL DEFAULT 0,
  `allowstatdata` tinyint(1) NOT NULL DEFAULT 0,
  `magicdiscount` tinyint(1) NOT NULL DEFAULT 0,
  `domainlength` smallint(6) unsigned NOT NULL DEFAULT 0,
  `seccode` tinyint(1) NOT NULL DEFAULT 1,
  `disablepostctrl` tinyint(1) NOT NULL DEFAULT 0,
  `allowbuildgroup` tinyint(3) unsigned NOT NULL DEFAULT 0,
  `allowgroupdirectpost` tinyint(3) unsigned NOT NULL DEFAULT 0,
  `allowgroupposturl` tinyint(3) unsigned NOT NULL DEFAULT 0,
  `edittimelimit` int(10) unsigned NOT NULL DEFAULT 0,
  `allowpostarticle` tinyint(1) NOT NULL DEFAULT 0,
  `allowdownlocalimg` tinyint(1) NOT NULL DEFAULT 0,
  `allowdownremoteimg` tinyint(1) NOT NULL DEFAULT 0,
  `allowpostarticlemod` tinyint(1) NOT NULL DEFAULT 0,
  `allowspacediyhtml` tinyint(1) NOT NULL DEFAULT 0,
  `allowspacediybbcode` tinyint(1) NOT NULL DEFAULT 0,
  `allowspacediyimgcode` tinyint(1) NOT NULL DEFAULT 0,
  `allowcommentpost` tinyint(1) NOT NULL DEFAULT 2,
  `allowcommentitem` tinyint(1) NOT NULL DEFAULT 0,
  `allowcommentreply` tinyint(1) NOT NULL DEFAULT 0,
  `allowreplycredit` tinyint(1) NOT NULL DEFAULT 0,
  `ignorecensor` tinyint(1) NOT NULL DEFAULT 0,
  `allowsendallpm` tinyint(1) NOT NULL DEFAULT 0,
  `allowsendpmmaxnum` smallint(6) unsigned NOT NULL DEFAULT 0,
  `maximagesize` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `allowmediacode` tinyint(1) NOT NULL DEFAULT 0,
  `allowbegincode` tinyint(1) NOT NULL DEFAULT 0,
  `allowat` smallint(6) unsigned NOT NULL DEFAULT 0,
  `allowsave` tinyint(1) NOT NULL DEFAULT 1,
  `allowsavereply` tinyint(1) NOT NULL DEFAULT 1,
  `allowsavenum` int(10) unsigned NOT NULL DEFAULT 0,
  `allowsetpublishdate` tinyint(1) NOT NULL DEFAULT 0,
  `allowfollowcollection` tinyint(3) unsigned NOT NULL DEFAULT 0,
  `allowcommentcollection` tinyint(1) NOT NULL DEFAULT 0,
  `allowcreatecollection` smallint(6) unsigned NOT NULL DEFAULT 0,
  `forcesecques` tinyint(1) NOT NULL DEFAULT 0,
  `forcelogin` tinyint(3) unsigned NOT NULL DEFAULT 0,
  `closead` tinyint(1) NOT NULL DEFAULT 0,
  `buildgroupcredits` smallint(6) unsigned NOT NULL DEFAULT 0,
  `allowimgcontent` tinyint(1) NOT NULL DEFAULT 0,
  `allowavatarupload` tinyint(1) NOT NULL DEFAULT 0,
  `allowviewprofile` tinyint(1) NOT NULL DEFAULT 0,
  `fields` longtext CHARACTER SET utf8mb4 COLLATE utf8mb4_bin DEFAULT NULL CHECK (json_valid(`fields`)),
  PRIMARY KEY (`groupid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_common_usergroup_field` WRITE;
/*!40000 ALTER TABLE `pre_common_usergroup_field` DISABLE KEYS */;
INSERT INTO `pre_common_usergroup_field` VALUES
(1,200,1,1,1,1,1,1,3,1,1,1,1,1,127,1,1,1,1,1,1,1,1,1,1,1,2,1,0,30,500,2048000,0,0,0,'','','',1,0,1,0,0,200,1,5,255,0,3,1,1,0,0,1,0,1000,0,0,0,1,1,1,1,0,0,0,0,0,1,1,1,0,1,1,0,5,1,1,30,3,3,0,1,1,1,0,1,1,1,3,1,0,1,1,1,0,0,1,1,50,1,1,0,0,30,1,5,0,0,0,0,0,1,1,'{}'),
(2,150,1,1,1,1,1,1,3,1,1,1,1,1,95,1,1,1,1,1,0,1,0,1,1,1,2,1,0,20,300,2048000,0,0,0,'chm, pdf, zip, rar, tar, gz, bzip2, gif, jpg, jpeg, png','','',1,0,1,0,0,180,1,5,255,0,3,1,0,0,0,1,0,1000,0,0,0,1,1,1,1,0,0,0,0,0,1,1,1,0,0,0,0,5,1,1,20,3,0,0,0,0,0,0,0,1,1,2,1,0,1,1,0,0,0,0,0,0,1,1,0,0,30,1,5,0,0,0,0,0,1,1,'{}'),
(3,100,1,1,1,1,1,1,1,1,1,1,1,1,95,1,0,1,1,1,0,1,0,0,1,1,2,1,0,10,200,2048000,0,0,0,'chm, pdf, zip, rar, tar, gz, bzip2, gif, jpg, jpeg, png','','',1,0,1,0,0,160,1,5,224,0,3,1,0,0,0,1,0,1000,0,0,0,1,1,1,1,0,0,0,0,0,1,1,1,0,0,0,0,5,1,1,15,3,0,0,0,0,0,0,0,1,1,2,1,0,1,1,0,0,0,0,0,0,1,1,0,0,30,1,5,0,0,0,0,0,1,1,'{}'),
(4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,'','','',1,0,1,0,0,0,0,5,0,0,3,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,3,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,0,0,0,0,0,0,0,0,0,0,0,0,'{}'),
(5,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,'','','',1,0,1,0,0,0,0,5,0,0,3,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,3,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,0,0,0,0,0,0,0,0,0,0,0,0,'{}'),
(6,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,'','','',1,0,1,0,0,0,0,5,0,0,3,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,3,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,0,0,0,0,0,0,0,0,0,0,0,0,'{}'),
(7,1,0,0,0,0,0,0,0,0,0,0,0,0,19,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,'gif, jpg, jpeg, png','','',1,0,1,0,0,0,0,5,0,0,3,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,3,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,0,0,0,0,0,0,0,0,0,0,0,0,'{}'),
(8,0,0,0,0,0,0,0,0,0,0,0,0,0,2,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,50,0,0,0,0,'','','',1,0,1,0,0,0,0,5,0,0,3,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,3,0,0,0,0,0,0,0,0,0,2,0,0,0,0,0,0,0,0,0,0,1,1,0,0,0,0,0,0,0,0,0,0,0,0,'{}'),
(9,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,'chm,pdf,zip,rar,tar,gz,bzip2,gif,jpg,jpeg,png','','',1,0,1,0,1,0,0,5,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,1,1,0,0,0,0,0,0,0,0,0,0,0,0,'{}'),
(10,10,1,1,1,1,1,1,0,1,1,1,1,1,95,0,0,0,0,0,0,0,0,0,1,0,1,0,0,0,80,1024000,0,0,0,'chm, pdf, zip, rar, tar, gz, bzip2, gif, jpg, jpeg, png','','',1,0,1,0,0,40,1,5,0,0,3,1,0,0,0,1,0,1000,0,0,0,1,1,1,1,0,0,0,0,0,1,1,1,0,0,0,0,5,1,0,5,3,0,0,0,0,0,0,0,1,1,2,1,0,0,0,0,0,0,0,0,0,1,1,0,0,30,1,5,0,0,0,0,0,1,1,'{}'),
(11,20,1,1,1,1,1,1,0,1,1,1,1,1,95,0,0,0,0,0,0,0,0,0,1,0,1,0,0,0,100,1024000,0,0,0,'chm, pdf, zip, rar, tar, gz, bzip2, gif, jpg, jpeg, png','','',1,0,1,0,0,60,1,5,0,0,3,1,0,0,0,1,0,1000,0,0,0,1,1,1,1,0,0,0,0,0,1,1,1,0,0,0,0,5,1,0,5,3,0,0,0,0,0,0,0,1,1,2,1,0,1,0,0,0,0,0,0,0,1,1,0,0,30,1,5,0,0,0,0,0,1,1,'{}'),
(12,30,1,1,1,1,1,1,0,1,1,1,1,1,95,0,0,0,0,0,0,0,0,0,1,0,1,0,0,0,150,1024000,0,0,0,'chm, pdf, zip, rar, tar, gz, bzip2, gif, jpg, jpeg, png','','',1,0,1,0,0,80,1,5,0,0,3,1,0,0,0,1,0,1000,0,0,0,1,1,1,1,0,0,0,0,0,1,1,1,0,0,0,0,5,1,0,5,3,0,0,0,0,0,0,0,1,1,2,1,0,1,0,0,0,0,0,0,0,1,1,0,0,30,1,5,0,0,0,0,0,1,1,'{}'),
(13,50,1,1,1,1,1,1,0,1,1,1,1,1,95,1,0,0,0,0,0,0,0,0,1,0,2,0,0,0,200,2048000,0,0,0,'chm, pdf, zip, rar, tar, gz, bzip2, gif, jpg, jpeg, png','','',1,0,1,0,0,100,1,5,0,0,3,1,0,0,0,1,0,1000,0,0,0,1,1,1,1,0,0,0,0,0,1,1,1,0,0,0,0,5,1,0,10,3,0,0,0,0,0,0,0,1,1,2,1,0,1,0,0,0,0,0,0,0,1,1,0,0,30,1,5,0,0,0,0,0,1,1,'{}'),
(14,70,1,1,1,1,1,1,0,1,1,1,1,1,95,1,0,0,1,1,0,0,0,0,1,1,2,0,0,0,300,2048000,0,0,0,'chm, pdf, zip, rar, tar, gz, bzip2, gif, jpg, jpeg, png','','',1,0,1,0,0,120,1,5,0,0,3,1,0,0,0,1,0,1000,0,0,0,1,1,1,1,0,0,0,0,0,1,1,1,0,0,0,0,5,1,0,10,3,0,0,0,0,0,0,0,1,1,2,1,0,1,0,0,0,0,0,0,0,1,1,0,0,30,1,5,0,0,0,0,0,1,1,'{}'),
(15,90,1,1,1,1,1,1,0,1,1,1,1,1,95,1,1,0,1,1,0,0,0,1,1,1,2,0,0,0,500,2048000,0,0,0,'chm, pdf, zip, rar, tar, gz, bzip2, gif, jpg, jpeg, png','','',1,0,1,0,0,140,1,5,0,0,3,1,0,0,0,1,0,1000,0,0,0,1,1,1,1,0,0,0,0,0,1,1,1,0,0,0,0,5,1,0,10,3,0,0,0,0,0,0,0,1,1,2,1,0,1,0,0,0,0,0,0,0,1,1,0,0,30,1,5,0,0,0,0,0,1,1,'{}'),
(16,100,1,1,1,1,1,1,1,1,1,1,1,1,95,1,0,1,1,1,0,1,0,0,1,1,2,1,0,10,200,2048000,0,0,0,'chm, pdf, zip, rar, tar, gz, bzip2, gif, jpg, jpeg, png','','',1,0,1,0,0,160,1,5,188,0,3,0,0,0,0,1,0,1000,0,0,0,1,1,1,1,0,0,0,0,0,1,1,1,0,0,0,0,5,1,0,15,3,0,0,0,0,0,0,0,1,1,2,1,0,1,0,0,0,0,0,0,0,1,1,0,0,30,1,5,0,0,0,0,0,1,1,'{}'),
(17,150,1,1,1,1,1,1,3,1,1,1,1,1,95,1,1,1,1,1,0,1,0,0,1,1,2,1,0,20,300,2048000,0,0,0,'chm, pdf, zip, rar, tar, gz, bzip2, gif, jpg, jpeg, png','','',1,0,1,0,0,180,1,5,255,0,3,0,0,0,0,1,0,1000,0,0,0,1,1,1,1,0,0,0,0,0,1,1,1,0,0,0,0,5,1,1,15,3,0,0,0,0,0,0,0,1,1,2,1,0,1,0,0,0,0,0,0,0,1,1,0,0,30,1,5,0,0,0,0,0,1,1,'{}'),
(18,200,1,1,1,1,1,1,3,1,1,1,1,1,95,0,1,1,1,1,0,1,1,1,1,1,2,0,0,30,500,0,0,0,1,'','','',1,0,1,0,0,200,1,5,255,0,3,3,0,0,0,1,0,1000,0,0,0,1,1,1,1,0,0,0,0,0,1,1,1,0,0,0,0,15,1,1,5,3,0,0,0,0,0,0,0,1,1,2,1,0,1,0,0,0,0,0,0,0,1,1,0,0,30,1,5,0,0,0,0,0,1,1,'{}'),
(19,100,1,1,1,1,1,1,1,1,1,1,1,1,95,1,0,1,1,1,0,1,0,0,1,1,2,1,0,10,200,2048000,0,0,0,'chm, pdf, zip, rar, tar, gz, bzip2, gif, jpg, jpeg, png','','',1,0,1,0,0,160,1,5,188,0,3,0,0,0,0,1,0,1000,0,0,0,1,1,1,1,0,0,0,0,0,1,1,1,0,0,0,0,5,1,1,15,3,0,0,0,0,0,0,0,1,1,2,1,0,1,0,0,0,0,0,0,0,1,1,0,0,30,1,5,0,0,0,0,0,1,1,'{}');
/*!40000 ALTER TABLE `pre_common_usergroup_field` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_common_visit`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_common_visit` (
  `ip` varchar(45) NOT NULL DEFAULT '',
  `view` int(10) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`ip`),
  KEY `ip` (`ip`,`view`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_common_visit` WRITE;
/*!40000 ALTER TABLE `pre_common_visit` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_common_visit` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_common_word`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_common_word` (
  `id` smallint(6) unsigned NOT NULL AUTO_INCREMENT,
  `admin` varchar(50) NOT NULL DEFAULT '',
  `type` smallint(6) NOT NULL DEFAULT 1,
  `find` varchar(255) NOT NULL DEFAULT '',
  `replacement` varchar(255) NOT NULL DEFAULT '',
  `extra` varchar(255) NOT NULL DEFAULT '',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_common_word` WRITE;
/*!40000 ALTER TABLE `pre_common_word` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_common_word` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_common_word_type`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_common_word_type` (
  `id` smallint(6) unsigned NOT NULL AUTO_INCREMENT,
  `typename` varchar(15) NOT NULL DEFAULT '',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=3 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_common_word_type` WRITE;
/*!40000 ALTER TABLE `pre_common_word_type` DISABLE KEYS */;
INSERT INTO `pre_common_word_type` VALUES
(1,'鏀挎不'),
(2,'骞垮憡');
/*!40000 ALTER TABLE `pre_common_word_type` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_connect_disktask`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_connect_disktask` (
  `taskid` int(10) unsigned NOT NULL AUTO_INCREMENT,
  `aid` int(10) unsigned NOT NULL DEFAULT 0,
  `uid` int(10) unsigned NOT NULL DEFAULT 0,
  `openid` char(32) NOT NULL DEFAULT '',
  `filename` varchar(255) NOT NULL DEFAULT '',
  `verifycode` char(32) NOT NULL DEFAULT '',
  `status` smallint(6) unsigned NOT NULL DEFAULT 0,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  `downloadtime` int(10) unsigned NOT NULL DEFAULT 0,
  `extra` text DEFAULT NULL,
  PRIMARY KEY (`taskid`),
  KEY `openid` (`openid`),
  KEY `status` (`status`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_connect_disktask` WRITE;
/*!40000 ALTER TABLE `pre_connect_disktask` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_connect_disktask` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_connect_feedlog`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_connect_feedlog` (
  `flid` mediumint(8) unsigned NOT NULL AUTO_INCREMENT,
  `tid` int(10) unsigned NOT NULL DEFAULT 0,
  `uid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `publishtimes` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `lastpublished` int(10) unsigned NOT NULL DEFAULT 0,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  `status` tinyint(1) NOT NULL DEFAULT 1,
  PRIMARY KEY (`flid`),
  UNIQUE KEY `tid` (`tid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_connect_feedlog` WRITE;
/*!40000 ALTER TABLE `pre_connect_feedlog` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_connect_feedlog` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_forum_access`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_forum_access` (
  `uid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `fid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `allowview` tinyint(1) NOT NULL DEFAULT 0,
  `allowpost` tinyint(1) NOT NULL DEFAULT 0,
  `allowreply` tinyint(1) NOT NULL DEFAULT 0,
  `allowgetattach` tinyint(1) NOT NULL DEFAULT 0,
  `allowgetimage` tinyint(1) NOT NULL DEFAULT 0,
  `allowpostattach` tinyint(1) NOT NULL DEFAULT 0,
  `allowpostimage` tinyint(1) NOT NULL DEFAULT 0,
  `adminuser` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`uid`,`fid`),
  KEY `listorder` (`fid`,`dateline`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_forum_access` WRITE;
/*!40000 ALTER TABLE `pre_forum_access` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_forum_access` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_forum_activity`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_forum_activity` (
  `tid` int(10) unsigned NOT NULL DEFAULT 0,
  `uid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `aid` int(10) unsigned NOT NULL DEFAULT 0,
  `cost` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `starttimefrom` int(10) unsigned NOT NULL DEFAULT 0,
  `starttimeto` int(10) unsigned NOT NULL DEFAULT 0,
  `place` varchar(255) NOT NULL DEFAULT '',
  `class` varchar(255) NOT NULL DEFAULT '',
  `gender` tinyint(1) NOT NULL DEFAULT 0,
  `number` smallint(5) unsigned NOT NULL DEFAULT 0,
  `applynumber` smallint(5) unsigned NOT NULL DEFAULT 0,
  `expiration` int(10) unsigned NOT NULL DEFAULT 0,
  `ufield` text NOT NULL,
  `credit` smallint(6) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`tid`),
  KEY `uid` (`uid`,`starttimefrom`),
  KEY `starttimefrom` (`starttimefrom`),
  KEY `expiration` (`expiration`),
  KEY `applynumber` (`applynumber`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_forum_activity` WRITE;
/*!40000 ALTER TABLE `pre_forum_activity` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_forum_activity` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_forum_activityapply`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_forum_activityapply` (
  `applyid` int(10) unsigned NOT NULL AUTO_INCREMENT,
  `tid` int(10) unsigned NOT NULL DEFAULT 0,
  `username` varchar(255) NOT NULL DEFAULT '',
  `uid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `message` varchar(255) NOT NULL DEFAULT '',
  `verified` tinyint(1) NOT NULL DEFAULT 0,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  `payment` mediumint(8) NOT NULL DEFAULT 0,
  `ufielddata` text NOT NULL,
  PRIMARY KEY (`applyid`),
  KEY `uid` (`uid`),
  KEY `tid` (`tid`),
  KEY `dateline` (`tid`,`dateline`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_forum_activityapply` WRITE;
/*!40000 ALTER TABLE `pre_forum_activityapply` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_forum_activityapply` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_forum_announcement`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_forum_announcement` (
  `id` smallint(6) unsigned NOT NULL AUTO_INCREMENT,
  `author` varchar(50) NOT NULL DEFAULT '',
  `subject` varchar(255) NOT NULL DEFAULT '',
  `type` tinyint(1) NOT NULL DEFAULT 0,
  `displayorder` tinyint(3) NOT NULL DEFAULT 0,
  `starttime` int(10) unsigned NOT NULL DEFAULT 0,
  `endtime` int(10) unsigned NOT NULL DEFAULT 0,
  `message` text NOT NULL,
  `groups` text NOT NULL,
  PRIMARY KEY (`id`),
  KEY `timespan` (`starttime`,`endtime`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_forum_announcement` WRITE;
/*!40000 ALTER TABLE `pre_forum_announcement` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_forum_announcement` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_forum_attachment`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_forum_attachment` (
  `aid` int(10) unsigned NOT NULL AUTO_INCREMENT,
  `tid` int(10) unsigned NOT NULL DEFAULT 0,
  `pid` int(10) unsigned NOT NULL DEFAULT 0,
  `uid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `tableid` tinyint(3) unsigned NOT NULL DEFAULT 0,
  `downloads` mediumint(8) NOT NULL DEFAULT 0,
  PRIMARY KEY (`aid`),
  KEY `tid` (`tid`),
  KEY `pid` (`pid`),
  KEY `uid` (`uid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_forum_attachment` WRITE;
/*!40000 ALTER TABLE `pre_forum_attachment` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_forum_attachment` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_forum_attachment_0`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_forum_attachment_0` (
  `aid` int(10) unsigned NOT NULL,
  `tid` int(10) unsigned NOT NULL DEFAULT 0,
  `pid` int(10) unsigned NOT NULL DEFAULT 0,
  `uid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  `filename` varchar(255) NOT NULL DEFAULT '',
  `filesize` int(10) unsigned NOT NULL DEFAULT 0,
  `attachment` varchar(255) NOT NULL DEFAULT '',
  `remote` tinyint(1) NOT NULL DEFAULT 0,
  `description` varchar(255) NOT NULL,
  `readperm` tinyint(3) unsigned NOT NULL DEFAULT 0,
  `price` smallint(6) unsigned NOT NULL DEFAULT 0,
  `isimage` tinyint(1) NOT NULL DEFAULT 0,
  `width` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `height` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `thumb` tinyint(1) NOT NULL DEFAULT 0,
  `picid` mediumint(8) NOT NULL DEFAULT 0,
  PRIMARY KEY (`aid`),
  KEY `tid` (`tid`),
  KEY `pid` (`pid`),
  KEY `uid` (`uid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_forum_attachment_0` WRITE;
/*!40000 ALTER TABLE `pre_forum_attachment_0` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_forum_attachment_0` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_forum_attachment_1`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_forum_attachment_1` (
  `aid` int(10) unsigned NOT NULL,
  `tid` int(10) unsigned NOT NULL DEFAULT 0,
  `pid` int(10) unsigned NOT NULL DEFAULT 0,
  `uid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  `filename` varchar(255) NOT NULL DEFAULT '',
  `filesize` int(10) unsigned NOT NULL DEFAULT 0,
  `attachment` varchar(255) NOT NULL DEFAULT '',
  `remote` tinyint(1) NOT NULL DEFAULT 0,
  `description` varchar(255) NOT NULL,
  `readperm` tinyint(3) unsigned NOT NULL DEFAULT 0,
  `price` smallint(6) unsigned NOT NULL DEFAULT 0,
  `isimage` tinyint(1) NOT NULL DEFAULT 0,
  `width` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `height` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `thumb` tinyint(1) NOT NULL DEFAULT 0,
  `picid` mediumint(8) NOT NULL DEFAULT 0,
  PRIMARY KEY (`aid`),
  KEY `tid` (`tid`),
  KEY `pid` (`pid`),
  KEY `uid` (`uid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_forum_attachment_1` WRITE;
/*!40000 ALTER TABLE `pre_forum_attachment_1` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_forum_attachment_1` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_forum_attachment_2`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_forum_attachment_2` (
  `aid` int(10) unsigned NOT NULL,
  `tid` int(10) unsigned NOT NULL DEFAULT 0,
  `pid` int(10) unsigned NOT NULL DEFAULT 0,
  `uid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  `filename` varchar(255) NOT NULL DEFAULT '',
  `filesize` int(10) unsigned NOT NULL DEFAULT 0,
  `attachment` varchar(255) NOT NULL DEFAULT '',
  `remote` tinyint(1) NOT NULL DEFAULT 0,
  `description` varchar(255) NOT NULL,
  `readperm` tinyint(3) unsigned NOT NULL DEFAULT 0,
  `price` smallint(6) unsigned NOT NULL DEFAULT 0,
  `isimage` tinyint(1) NOT NULL DEFAULT 0,
  `width` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `height` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `thumb` tinyint(1) NOT NULL DEFAULT 0,
  `picid` mediumint(8) NOT NULL DEFAULT 0,
  PRIMARY KEY (`aid`),
  KEY `tid` (`tid`),
  KEY `pid` (`pid`),
  KEY `uid` (`uid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_forum_attachment_2` WRITE;
/*!40000 ALTER TABLE `pre_forum_attachment_2` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_forum_attachment_2` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_forum_attachment_3`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_forum_attachment_3` (
  `aid` int(10) unsigned NOT NULL,
  `tid` int(10) unsigned NOT NULL DEFAULT 0,
  `pid` int(10) unsigned NOT NULL DEFAULT 0,
  `uid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  `filename` varchar(255) NOT NULL DEFAULT '',
  `filesize` int(10) unsigned NOT NULL DEFAULT 0,
  `attachment` varchar(255) NOT NULL DEFAULT '',
  `remote` tinyint(1) NOT NULL DEFAULT 0,
  `description` varchar(255) NOT NULL,
  `readperm` tinyint(3) unsigned NOT NULL DEFAULT 0,
  `price` smallint(6) unsigned NOT NULL DEFAULT 0,
  `isimage` tinyint(1) NOT NULL DEFAULT 0,
  `width` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `height` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `thumb` tinyint(1) NOT NULL DEFAULT 0,
  `picid` mediumint(8) NOT NULL DEFAULT 0,
  PRIMARY KEY (`aid`),
  KEY `tid` (`tid`),
  KEY `pid` (`pid`),
  KEY `uid` (`uid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_forum_attachment_3` WRITE;
/*!40000 ALTER TABLE `pre_forum_attachment_3` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_forum_attachment_3` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_forum_attachment_4`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_forum_attachment_4` (
  `aid` int(10) unsigned NOT NULL,
  `tid` int(10) unsigned NOT NULL DEFAULT 0,
  `pid` int(10) unsigned NOT NULL DEFAULT 0,
  `uid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  `filename` varchar(255) NOT NULL DEFAULT '',
  `filesize` int(10) unsigned NOT NULL DEFAULT 0,
  `attachment` varchar(255) NOT NULL DEFAULT '',
  `remote` tinyint(1) NOT NULL DEFAULT 0,
  `description` varchar(255) NOT NULL,
  `readperm` tinyint(3) unsigned NOT NULL DEFAULT 0,
  `price` smallint(6) unsigned NOT NULL DEFAULT 0,
  `isimage` tinyint(1) NOT NULL DEFAULT 0,
  `width` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `height` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `thumb` tinyint(1) NOT NULL DEFAULT 0,
  `picid` mediumint(8) NOT NULL DEFAULT 0,
  PRIMARY KEY (`aid`),
  KEY `tid` (`tid`),
  KEY `pid` (`pid`),
  KEY `uid` (`uid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_forum_attachment_4` WRITE;
/*!40000 ALTER TABLE `pre_forum_attachment_4` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_forum_attachment_4` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_forum_attachment_5`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_forum_attachment_5` (
  `aid` int(10) unsigned NOT NULL,
  `tid` int(10) unsigned NOT NULL DEFAULT 0,
  `pid` int(10) unsigned NOT NULL DEFAULT 0,
  `uid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  `filename` varchar(255) NOT NULL DEFAULT '',
  `filesize` int(10) unsigned NOT NULL DEFAULT 0,
  `attachment` varchar(255) NOT NULL DEFAULT '',
  `remote` tinyint(1) NOT NULL DEFAULT 0,
  `description` varchar(255) NOT NULL,
  `readperm` tinyint(3) unsigned NOT NULL DEFAULT 0,
  `price` smallint(6) unsigned NOT NULL DEFAULT 0,
  `isimage` tinyint(1) NOT NULL DEFAULT 0,
  `width` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `height` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `thumb` tinyint(1) NOT NULL DEFAULT 0,
  `picid` mediumint(8) NOT NULL DEFAULT 0,
  PRIMARY KEY (`aid`),
  KEY `tid` (`tid`),
  KEY `pid` (`pid`),
  KEY `uid` (`uid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_forum_attachment_5` WRITE;
/*!40000 ALTER TABLE `pre_forum_attachment_5` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_forum_attachment_5` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_forum_attachment_6`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_forum_attachment_6` (
  `aid` int(10) unsigned NOT NULL,
  `tid` int(10) unsigned NOT NULL DEFAULT 0,
  `pid` int(10) unsigned NOT NULL DEFAULT 0,
  `uid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  `filename` varchar(255) NOT NULL DEFAULT '',
  `filesize` int(10) unsigned NOT NULL DEFAULT 0,
  `attachment` varchar(255) NOT NULL DEFAULT '',
  `remote` tinyint(1) NOT NULL DEFAULT 0,
  `description` varchar(255) NOT NULL,
  `readperm` tinyint(3) unsigned NOT NULL DEFAULT 0,
  `price` smallint(6) unsigned NOT NULL DEFAULT 0,
  `isimage` tinyint(1) NOT NULL DEFAULT 0,
  `width` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `height` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `thumb` tinyint(1) NOT NULL DEFAULT 0,
  `picid` mediumint(8) NOT NULL DEFAULT 0,
  PRIMARY KEY (`aid`),
  KEY `tid` (`tid`),
  KEY `pid` (`pid`),
  KEY `uid` (`uid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_forum_attachment_6` WRITE;
/*!40000 ALTER TABLE `pre_forum_attachment_6` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_forum_attachment_6` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_forum_attachment_7`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_forum_attachment_7` (
  `aid` int(10) unsigned NOT NULL,
  `tid` int(10) unsigned NOT NULL DEFAULT 0,
  `pid` int(10) unsigned NOT NULL DEFAULT 0,
  `uid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  `filename` varchar(255) NOT NULL DEFAULT '',
  `filesize` int(10) unsigned NOT NULL DEFAULT 0,
  `attachment` varchar(255) NOT NULL DEFAULT '',
  `remote` tinyint(1) NOT NULL DEFAULT 0,
  `description` varchar(255) NOT NULL,
  `readperm` tinyint(3) unsigned NOT NULL DEFAULT 0,
  `price` smallint(6) unsigned NOT NULL DEFAULT 0,
  `isimage` tinyint(1) NOT NULL DEFAULT 0,
  `width` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `height` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `thumb` tinyint(1) NOT NULL DEFAULT 0,
  `picid` mediumint(8) NOT NULL DEFAULT 0,
  PRIMARY KEY (`aid`),
  KEY `tid` (`tid`),
  KEY `pid` (`pid`),
  KEY `uid` (`uid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_forum_attachment_7` WRITE;
/*!40000 ALTER TABLE `pre_forum_attachment_7` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_forum_attachment_7` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_forum_attachment_8`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_forum_attachment_8` (
  `aid` int(10) unsigned NOT NULL,
  `tid` int(10) unsigned NOT NULL DEFAULT 0,
  `pid` int(10) unsigned NOT NULL DEFAULT 0,
  `uid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  `filename` varchar(255) NOT NULL DEFAULT '',
  `filesize` int(10) unsigned NOT NULL DEFAULT 0,
  `attachment` varchar(255) NOT NULL DEFAULT '',
  `remote` tinyint(1) NOT NULL DEFAULT 0,
  `description` varchar(255) NOT NULL,
  `readperm` tinyint(3) unsigned NOT NULL DEFAULT 0,
  `price` smallint(6) unsigned NOT NULL DEFAULT 0,
  `isimage` tinyint(1) NOT NULL DEFAULT 0,
  `width` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `height` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `thumb` tinyint(1) NOT NULL DEFAULT 0,
  `picid` mediumint(8) NOT NULL DEFAULT 0,
  PRIMARY KEY (`aid`),
  KEY `tid` (`tid`),
  KEY `pid` (`pid`),
  KEY `uid` (`uid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_forum_attachment_8` WRITE;
/*!40000 ALTER TABLE `pre_forum_attachment_8` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_forum_attachment_8` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_forum_attachment_9`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_forum_attachment_9` (
  `aid` int(10) unsigned NOT NULL,
  `tid` int(10) unsigned NOT NULL DEFAULT 0,
  `pid` int(10) unsigned NOT NULL DEFAULT 0,
  `uid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  `filename` varchar(255) NOT NULL DEFAULT '',
  `filesize` int(10) unsigned NOT NULL DEFAULT 0,
  `attachment` varchar(255) NOT NULL DEFAULT '',
  `remote` tinyint(1) NOT NULL DEFAULT 0,
  `description` varchar(255) NOT NULL,
  `readperm` tinyint(3) unsigned NOT NULL DEFAULT 0,
  `price` smallint(6) unsigned NOT NULL DEFAULT 0,
  `isimage` tinyint(1) NOT NULL DEFAULT 0,
  `width` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `height` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `thumb` tinyint(1) NOT NULL DEFAULT 0,
  `picid` mediumint(8) NOT NULL DEFAULT 0,
  PRIMARY KEY (`aid`),
  KEY `tid` (`tid`),
  KEY `pid` (`pid`),
  KEY `uid` (`uid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_forum_attachment_9` WRITE;
/*!40000 ALTER TABLE `pre_forum_attachment_9` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_forum_attachment_9` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_forum_attachment_exif`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_forum_attachment_exif` (
  `aid` int(10) unsigned NOT NULL,
  `exif` text NOT NULL,
  PRIMARY KEY (`aid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_forum_attachment_exif` WRITE;
/*!40000 ALTER TABLE `pre_forum_attachment_exif` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_forum_attachment_exif` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_forum_attachment_unused`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_forum_attachment_unused` (
  `aid` int(10) unsigned NOT NULL,
  `uid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  `filename` varchar(255) NOT NULL DEFAULT '',
  `filesize` int(10) unsigned NOT NULL DEFAULT 0,
  `attachment` varchar(255) NOT NULL DEFAULT '',
  `remote` tinyint(1) NOT NULL DEFAULT 0,
  `isimage` tinyint(1) NOT NULL DEFAULT 0,
  `width` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `height` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `thumb` tinyint(1) NOT NULL DEFAULT 0,
  PRIMARY KEY (`aid`),
  KEY `uid` (`uid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_forum_attachment_unused` WRITE;
/*!40000 ALTER TABLE `pre_forum_attachment_unused` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_forum_attachment_unused` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_forum_attachtype`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_forum_attachtype` (
  `id` smallint(6) unsigned NOT NULL AUTO_INCREMENT,
  `fid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `extension` char(12) NOT NULL DEFAULT '',
  `maxsize` int(10) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`id`),
  KEY `fid` (`fid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_forum_attachtype` WRITE;
/*!40000 ALTER TABLE `pre_forum_attachtype` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_forum_attachtype` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_forum_bbcode`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_forum_bbcode` (
  `id` mediumint(8) unsigned NOT NULL AUTO_INCREMENT,
  `available` tinyint(1) NOT NULL DEFAULT 0,
  `tag` varchar(100) NOT NULL DEFAULT '',
  `icon` varchar(255) NOT NULL,
  `replacement` text NOT NULL,
  `example` varchar(255) NOT NULL DEFAULT '',
  `explanation` text NOT NULL,
  `params` tinyint(3) unsigned NOT NULL DEFAULT 1,
  `prompt` text NOT NULL,
  `nest` tinyint(3) unsigned NOT NULL DEFAULT 1,
  `displayorder` tinyint(3) NOT NULL DEFAULT 0,
  `perm` text NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=5 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_forum_bbcode` WRITE;
/*!40000 ALTER TABLE `pre_forum_bbcode` DISABLE KEYS */;
INSERT INTO `pre_forum_bbcode` VALUES
(1,0,'fly','bb_fly.gif','<marquee width=\"90%\" scrollamount=\"3\">{1}</marquee>','[fly]This is sample text[/fly]','浣垮唴瀹规í鍚戞粴鍔紝杩欎釜鏁堟灉绫讳技 HTML 鐨?marquee 鏍囩锛屾敞鎰忥細杩欎釜鏁堟灉鍙湪 Internet Explorer 娴忚鍣ㄤ笅鏈夋晥銆?,1,'璇疯緭鍏ユ粴鍔ㄦ樉绀虹殑鏂囧瓧:',1,19,'1	2	3	12	13	14	15	16	17	18	19'),
(2,2,'qq','bb_qq.gif','<a href=\"https://wpa.qq.com/msgrd?v=3&uin={1}&amp;site=[Discuz!]&amp;from=discuz&amp;menu=yes\" target=\"_blank\"><img src=\"static/image/common/qq_big.gif\" border=\"0\"></a>','[qq]688888[/qq]','鏄剧ず QQ 鍦ㄧ嚎鐘舵€侊紝鐐硅繖涓浘鏍囧彲浠ュ拰浠栵紙濂癸級鑱婂ぉ',1,'璇疯緭鍏?QQ 鍙风爜:<a href=\"\" class=\"xi2\" onclick=\"this.href=\'https://wp.qq.com/set.html?from=discuz&uin=\'+$(\'e_cst1_qq_param_1\').value\" target=\"_blank\" style=\"float:right;\">璁剧疆QQ鍦ㄧ嚎鐘舵€?nbsp;&nbsp;</a>',1,21,'1	2	3	10	11	12	13	14	15	16	17	18	19'),
(3,0,'sup','bb_sup.gif','<sup>{1}</sup>','X[sup]2[/sup]','涓婃爣',1,'璇疯緭鍏ヤ笂鏍囨枃瀛楋細',1,22,'1	2	3	12	13	14	15	16	17	18	19'),
(4,0,'sub','bb_sub.gif','<sub>{1}</sub>','X[sub]2[/sub]','涓嬫爣',1,'璇疯緭鍏ヤ笅鏍囨枃瀛楋細',1,23,'1	2	3	12	13	14	15	16	17	18	19');
/*!40000 ALTER TABLE `pre_forum_bbcode` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_forum_collection`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_forum_collection` (
  `ctid` mediumint(8) unsigned NOT NULL AUTO_INCREMENT,
  `uid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `username` varchar(50) NOT NULL DEFAULT '',
  `name` varchar(50) NOT NULL DEFAULT '',
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  `follownum` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `threadnum` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `commentnum` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `desc` varchar(255) NOT NULL DEFAULT '',
  `lastupdate` int(10) unsigned NOT NULL DEFAULT 0,
  `rate` float NOT NULL DEFAULT 0,
  `ratenum` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `lastpost` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `lastsubject` varchar(255) NOT NULL DEFAULT '',
  `lastposttime` int(10) unsigned NOT NULL DEFAULT 0,
  `lastposter` varchar(50) NOT NULL DEFAULT '',
  `lastvisit` int(10) unsigned NOT NULL DEFAULT 0,
  `keyword` varchar(255) NOT NULL DEFAULT '',
  `cover` tinyint(1) unsigned NOT NULL DEFAULT 0,
  `icon` tinyint(1) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`ctid`),
  KEY `dateline` (`dateline`),
  KEY `hotcollection` (`threadnum`,`lastupdate`),
  KEY `follownum` (`follownum`),
  KEY `uid` (`uid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_forum_collection` WRITE;
/*!40000 ALTER TABLE `pre_forum_collection` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_forum_collection` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_forum_collectioncomment`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_forum_collectioncomment` (
  `cid` mediumint(8) unsigned NOT NULL AUTO_INCREMENT,
  `ctid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `uid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `username` varchar(50) NOT NULL DEFAULT '',
  `message` text NOT NULL,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  `useip` varchar(45) NOT NULL DEFAULT '',
  `port` smallint(6) unsigned NOT NULL DEFAULT 0,
  `rate` float NOT NULL DEFAULT 0,
  PRIMARY KEY (`cid`),
  KEY `ctid` (`ctid`,`dateline`),
  KEY `userrate` (`ctid`,`uid`,`rate`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_forum_collectioncomment` WRITE;
/*!40000 ALTER TABLE `pre_forum_collectioncomment` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_forum_collectioncomment` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_forum_collectionfollow`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_forum_collectionfollow` (
  `uid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `username` char(50) NOT NULL DEFAULT '',
  `ctid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  `lastvisit` int(10) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`uid`,`ctid`),
  KEY `ctid` (`ctid`,`dateline`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_forum_collectionfollow` WRITE;
/*!40000 ALTER TABLE `pre_forum_collectionfollow` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_forum_collectionfollow` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_forum_collectioninvite`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_forum_collectioninvite` (
  `ctid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `uid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`ctid`,`uid`),
  KEY `dateline` (`dateline`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_forum_collectioninvite` WRITE;
/*!40000 ALTER TABLE `pre_forum_collectioninvite` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_forum_collectioninvite` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_forum_collectionrelated`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_forum_collectionrelated` (
  `tid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `collection` text NOT NULL,
  PRIMARY KEY (`tid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_forum_collectionrelated` WRITE;
/*!40000 ALTER TABLE `pre_forum_collectionrelated` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_forum_collectionrelated` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_forum_collectionteamworker`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_forum_collectionteamworker` (
  `ctid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `uid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `name` varchar(50) NOT NULL DEFAULT '',
  `username` varchar(50) NOT NULL DEFAULT '',
  `lastvisit` int(10) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`ctid`,`uid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_forum_collectionteamworker` WRITE;
/*!40000 ALTER TABLE `pre_forum_collectionteamworker` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_forum_collectionteamworker` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_forum_collectionthread`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_forum_collectionthread` (
  `ctid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `tid` int(10) unsigned NOT NULL DEFAULT 0,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  `reason` varchar(255) NOT NULL DEFAULT '',
  PRIMARY KEY (`ctid`,`tid`),
  KEY `ctid` (`ctid`,`dateline`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_forum_collectionthread` WRITE;
/*!40000 ALTER TABLE `pre_forum_collectionthread` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_forum_collectionthread` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_forum_creditslog`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_forum_creditslog` (
  `uid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `fromto` char(50) NOT NULL DEFAULT '',
  `sendcredits` tinyint(1) NOT NULL DEFAULT 0,
  `receivecredits` tinyint(1) NOT NULL DEFAULT 0,
  `send` int(10) unsigned NOT NULL DEFAULT 0,
  `receive` int(10) unsigned NOT NULL DEFAULT 0,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  `operation` char(3) NOT NULL DEFAULT '',
  KEY `uid` (`uid`,`dateline`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_forum_creditslog` WRITE;
/*!40000 ALTER TABLE `pre_forum_creditslog` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_forum_creditslog` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_forum_debate`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_forum_debate` (
  `tid` int(10) unsigned NOT NULL DEFAULT 0,
  `uid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `starttime` int(10) unsigned NOT NULL DEFAULT 0,
  `endtime` int(10) unsigned NOT NULL DEFAULT 0,
  `affirmdebaters` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `negadebaters` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `affirmvotes` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `negavotes` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `umpire` varchar(50) NOT NULL DEFAULT '',
  `winner` tinyint(1) NOT NULL DEFAULT 0,
  `bestdebater` varchar(50) NOT NULL DEFAULT '',
  `affirmpoint` text NOT NULL,
  `negapoint` text NOT NULL,
  `umpirepoint` text NOT NULL,
  `affirmvoterids` text NOT NULL,
  `negavoterids` text NOT NULL,
  `affirmreplies` mediumint(8) unsigned NOT NULL,
  `negareplies` mediumint(8) unsigned NOT NULL,
  PRIMARY KEY (`tid`),
  KEY `uid` (`uid`,`starttime`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_forum_debate` WRITE;
/*!40000 ALTER TABLE `pre_forum_debate` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_forum_debate` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_forum_debatepost`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_forum_debatepost` (
  `pid` int(10) unsigned NOT NULL DEFAULT 0,
  `stand` tinyint(1) NOT NULL DEFAULT 0,
  `tid` int(10) unsigned NOT NULL DEFAULT 0,
  `uid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  `voters` mediumint(10) unsigned NOT NULL DEFAULT 0,
  `voterids` text NOT NULL,
  PRIMARY KEY (`pid`),
  KEY `pid` (`pid`,`stand`),
  KEY `tid` (`tid`,`uid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_forum_debatepost` WRITE;
/*!40000 ALTER TABLE `pre_forum_debatepost` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_forum_debatepost` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_forum_faq`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_forum_faq` (
  `id` smallint(6) NOT NULL AUTO_INCREMENT,
  `fpid` smallint(6) unsigned NOT NULL DEFAULT 0,
  `displayorder` tinyint(3) NOT NULL DEFAULT 0,
  `identifier` varchar(20) NOT NULL,
  `keyword` varchar(50) NOT NULL,
  `title` varchar(50) NOT NULL,
  `message` text NOT NULL,
  PRIMARY KEY (`id`),
  KEY `displayplay` (`displayorder`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_forum_faq` WRITE;
/*!40000 ALTER TABLE `pre_forum_faq` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_forum_faq` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_forum_filter_post`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_forum_filter_post` (
  `tid` int(10) unsigned NOT NULL DEFAULT 0,
  `pid` int(10) unsigned NOT NULL DEFAULT 0,
  `postlength` int(10) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`tid`,`pid`),
  KEY `tid` (`tid`,`postlength`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_forum_filter_post` WRITE;
/*!40000 ALTER TABLE `pre_forum_filter_post` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_forum_filter_post` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_forum_forum`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_forum_forum` (
  `fid` mediumint(8) unsigned NOT NULL AUTO_INCREMENT,
  `fup` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `type` enum('group','forum','sub') NOT NULL DEFAULT 'forum',
  `name` char(50) NOT NULL DEFAULT '',
  `status` tinyint(1) NOT NULL DEFAULT 0,
  `displayorder` smallint(6) NOT NULL DEFAULT 0,
  `styleid` smallint(6) unsigned NOT NULL DEFAULT 0,
  `threads` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `posts` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `todayposts` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `yesterdayposts` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `rank` smallint(6) unsigned NOT NULL DEFAULT 0,
  `oldrank` smallint(6) unsigned NOT NULL DEFAULT 0,
  `lastpost` char(110) NOT NULL DEFAULT '',
  `domain` char(15) NOT NULL DEFAULT '',
  `allowsmilies` tinyint(1) NOT NULL DEFAULT 0,
  `allowhtml` tinyint(1) NOT NULL DEFAULT 0,
  `allowbbcode` tinyint(1) NOT NULL DEFAULT 0,
  `allowimgcode` tinyint(1) NOT NULL DEFAULT 0,
  `allowmediacode` tinyint(1) NOT NULL DEFAULT 0,
  `allowanonymous` tinyint(1) NOT NULL DEFAULT 0,
  `allowpostspecial` smallint(6) unsigned NOT NULL DEFAULT 0,
  `allowspecialonly` tinyint(1) NOT NULL DEFAULT 0,
  `allowappend` tinyint(1) NOT NULL DEFAULT 0,
  `alloweditrules` tinyint(1) NOT NULL DEFAULT 0,
  `allowfeed` tinyint(1) NOT NULL DEFAULT 1,
  `allowside` tinyint(1) NOT NULL DEFAULT 0,
  `recyclebin` tinyint(1) NOT NULL DEFAULT 0,
  `modnewposts` tinyint(1) NOT NULL DEFAULT 0,
  `jammer` tinyint(1) NOT NULL DEFAULT 0,
  `disablewatermark` tinyint(1) NOT NULL DEFAULT 0,
  `inheritedmod` tinyint(1) NOT NULL DEFAULT 0,
  `autoclose` smallint(6) NOT NULL DEFAULT 0,
  `forumcolumns` tinyint(3) unsigned NOT NULL DEFAULT 0,
  `catforumcolumns` tinyint(3) unsigned NOT NULL DEFAULT 0,
  `threadcaches` tinyint(1) NOT NULL DEFAULT 0,
  `alloweditpost` tinyint(1) NOT NULL DEFAULT 1,
  `simple` smallint(6) NOT NULL DEFAULT 0,
  `modworks` tinyint(1) NOT NULL DEFAULT 0,
  `allowglobalstick` tinyint(1) NOT NULL DEFAULT 1,
  `level` smallint(6) NOT NULL DEFAULT 0,
  `commoncredits` int(10) unsigned NOT NULL DEFAULT 0,
  `archive` tinyint(1) NOT NULL DEFAULT 0,
  `recommend` smallint(6) unsigned NOT NULL DEFAULT 0,
  `favtimes` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `sharetimes` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `disablethumb` tinyint(1) NOT NULL DEFAULT 0,
  `disablecollect` tinyint(1) NOT NULL DEFAULT 0,
  `editormode` tinyint(1) NOT NULL DEFAULT -1,
  PRIMARY KEY (`fid`),
  KEY `forum` (`status`,`type`,`displayorder`),
  KEY `fup_type` (`fup`,`type`,`displayorder`),
  KEY `fup` (`fup`)
) ENGINE=InnoDB AUTO_INCREMENT=3 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_forum_forum` WRITE;
/*!40000 ALTER TABLE `pre_forum_forum` DISABLE KEYS */;
INSERT INTO `pre_forum_forum` VALUES
(1,0,'group','Discuz!',1,0,0,0,0,0,0,0,0,'','',0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,1,0,0,1,0,0,0,0,0,0,0,0,-1),
(2,1,'forum','榛樿鐗堝潡',1,0,0,0,0,0,0,0,0,'','',1,0,1,1,1,0,1,0,0,0,1,0,1,0,0,0,0,0,0,0,0,1,0,0,1,0,0,0,0,0,0,0,0,-1);
/*!40000 ALTER TABLE `pre_forum_forum` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_forum_forum_threadtable`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_forum_forum_threadtable` (
  `fid` smallint(6) unsigned NOT NULL,
  `threadtableid` smallint(6) unsigned NOT NULL,
  `threads` int(11) unsigned NOT NULL DEFAULT 0,
  `posts` int(11) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`fid`,`threadtableid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_forum_forum_threadtable` WRITE;
/*!40000 ALTER TABLE `pre_forum_forum_threadtable` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_forum_forum_threadtable` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_forum_forumfield`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_forum_forumfield` (
  `fid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `description` text NOT NULL,
  `password` varchar(12) NOT NULL DEFAULT '',
  `icon` varchar(255) NOT NULL DEFAULT '',
  `redirect` varchar(255) NOT NULL DEFAULT '',
  `attachextensions` varchar(255) NOT NULL DEFAULT '',
  `creditspolicy` mediumtext NOT NULL,
  `formulaperm` text NOT NULL,
  `moderators` text NOT NULL,
  `rules` text NOT NULL,
  `threadtypes` text NOT NULL,
  `threadsorts` text NOT NULL,
  `viewperm` text NOT NULL,
  `postperm` text NOT NULL,
  `replyperm` text NOT NULL,
  `getattachperm` text NOT NULL,
  `postattachperm` text NOT NULL,
  `postimageperm` text NOT NULL,
  `spviewperm` text NOT NULL,
  `seotitle` text NOT NULL,
  `keywords` text NOT NULL,
  `seodescription` text NOT NULL,
  `supe_pushsetting` text NOT NULL,
  `modrecommend` text NOT NULL,
  `threadplugin` text NOT NULL,
  `replybg` text NOT NULL,
  `extra` text NOT NULL,
  `jointype` tinyint(1) NOT NULL DEFAULT 0,
  `gviewperm` tinyint(1) NOT NULL DEFAULT 0,
  `membernum` smallint(6) unsigned NOT NULL DEFAULT 0,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  `lastupdate` int(10) unsigned NOT NULL DEFAULT 0,
  `activity` int(10) unsigned NOT NULL DEFAULT 0,
  `founderuid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `foundername` varchar(255) NOT NULL DEFAULT '',
  `banner` varchar(255) NOT NULL DEFAULT '',
  `groupnum` smallint(6) unsigned NOT NULL DEFAULT 0,
  `commentitem` text NOT NULL,
  `relatedgroup` text NOT NULL,
  `picstyle` tinyint(1) NOT NULL DEFAULT 0,
  `widthauto` tinyint(1) NOT NULL DEFAULT 0,
  `noantitheft` tinyint(1) NOT NULL DEFAULT 0,
  `noforumhidewater` tinyint(1) NOT NULL DEFAULT 0,
  `noforumrecommend` tinyint(1) NOT NULL DEFAULT 0,
  `livetid` int(10) unsigned NOT NULL DEFAULT 0,
  `price` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `fields` longtext CHARACTER SET utf8mb4 COLLATE utf8mb4_bin DEFAULT NULL CHECK (json_valid(`fields`)),
  PRIMARY KEY (`fid`),
  KEY `membernum` (`membernum`),
  KEY `dateline` (`dateline`),
  KEY `lastupdate` (`lastupdate`),
  KEY `activity` (`activity`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_forum_forumfield` WRITE;
/*!40000 ALTER TABLE `pre_forum_forumfield` DISABLE KEYS */;
INSERT INTO `pre_forum_forumfield` VALUES
(1,'','','','','','','','','','','','','','','','','','','','','','','','','','',0,0,0,0,0,0,0,'','',0,'','',0,0,0,0,0,0,0,'{}'),
(2,'','','','','','','','','','','','','','','','','','','','','','','','','','',0,0,0,0,0,0,0,'','',0,'','',0,0,0,0,0,0,0,'{}');
/*!40000 ALTER TABLE `pre_forum_forumfield` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_forum_forumrecommend`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_forum_forumrecommend` (
  `fid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `tid` int(10) unsigned NOT NULL,
  `typeid` smallint(6) NOT NULL,
  `displayorder` tinyint(1) NOT NULL,
  `subject` varchar(255) NOT NULL,
  `author` char(50) NOT NULL,
  `authorid` mediumint(8) NOT NULL,
  `moderatorid` mediumint(8) NOT NULL,
  `expiration` int(10) unsigned NOT NULL,
  `position` tinyint(1) NOT NULL DEFAULT 0,
  `highlight` tinyint(1) NOT NULL DEFAULT 0,
  `aid` int(10) unsigned NOT NULL DEFAULT 0,
  `filename` char(100) NOT NULL DEFAULT '',
  PRIMARY KEY (`tid`),
  KEY `displayorder` (`fid`,`displayorder`),
  KEY `position` (`position`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_forum_forumrecommend` WRITE;
/*!40000 ALTER TABLE `pre_forum_forumrecommend` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_forum_forumrecommend` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_forum_groupcreditslog`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_forum_groupcreditslog` (
  `fid` mediumint(8) unsigned NOT NULL,
  `uid` mediumint(8) unsigned NOT NULL,
  `logdate` int(10) NOT NULL DEFAULT 0,
  PRIMARY KEY (`fid`,`uid`,`logdate`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_forum_groupcreditslog` WRITE;
/*!40000 ALTER TABLE `pre_forum_groupcreditslog` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_forum_groupcreditslog` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_forum_groupfield`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_forum_groupfield` (
  `fid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `privacy` tinyint(1) NOT NULL DEFAULT 0,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  `type` varchar(100) NOT NULL,
  `data` text NOT NULL,
  UNIQUE KEY `types` (`fid`,`type`),
  KEY `fid` (`fid`),
  KEY `type` (`type`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_forum_groupfield` WRITE;
/*!40000 ALTER TABLE `pre_forum_groupfield` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_forum_groupfield` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_forum_groupinvite`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_forum_groupinvite` (
  `fid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `uid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `inviteuid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  UNIQUE KEY `ids` (`fid`,`inviteuid`),
  KEY `dateline` (`dateline`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_forum_groupinvite` WRITE;
/*!40000 ALTER TABLE `pre_forum_groupinvite` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_forum_groupinvite` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_forum_grouplevel`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_forum_grouplevel` (
  `levelid` smallint(6) unsigned NOT NULL AUTO_INCREMENT,
  `type` enum('special','default') NOT NULL DEFAULT 'default',
  `leveltitle` varchar(255) NOT NULL DEFAULT '',
  `creditshigher` int(10) NOT NULL DEFAULT 0,
  `creditslower` int(10) NOT NULL DEFAULT 0,
  `icon` varchar(255) NOT NULL DEFAULT '',
  `creditspolicy` text NOT NULL,
  `postpolicy` text NOT NULL,
  `specialswitch` text NOT NULL,
  PRIMARY KEY (`levelid`),
  KEY `creditsrange` (`creditshigher`,`creditslower`)
) ENGINE=InnoDB AUTO_INCREMENT=4 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_forum_grouplevel` WRITE;
/*!40000 ALTER TABLE `pre_forum_grouplevel` DISABLE KEYS */;
INSERT INTO `pre_forum_grouplevel` VALUES
(1,'default','鏅€氱骇',-999999999,500,'','a:2:{s:4:\"post\";s:1:\"1\";s:5:\"reply\";s:1:\"1\";}','a:11:{s:13:\"alloweditpost\";s:1:\"1\";s:10:\"recyclebin\";s:1:\"1\";s:12:\"allowsmilies\";s:1:\"1\";s:9:\"allowhtml\";i:0;s:11:\"allowbbcode\";s:1:\"1\";s:14:\"allowanonymous\";i:0;s:6:\"jammer\";i:0;s:12:\"allowimgcode\";s:1:\"1\";s:14:\"allowmediacode\";s:1:\"1\";s:16:\"allowpostspecial\";i:31;s:16:\"attachextensions\";s:7:\"jpg,gif\";}','a:5:{s:15:\"allowchangename\";s:1:\"1\";s:15:\"allowchangetype\";s:1:\"1\";s:15:\"allowclosegroup\";s:1:\"1\";s:15:\"allowthreadtype\";s:1:\"1\";s:13:\"membermaximum\";s:0:\"\";}'),
(2,'default','涓骇',500,3000,'','a:2:{s:4:\"post\";s:1:\"1\";s:5:\"reply\";s:1:\"1\";}','a:11:{s:13:\"alloweditpost\";s:1:\"1\";s:10:\"recyclebin\";s:1:\"1\";s:12:\"allowsmilies\";s:1:\"1\";s:9:\"allowhtml\";i:0;s:11:\"allowbbcode\";s:1:\"1\";s:14:\"allowanonymous\";i:0;s:6:\"jammer\";i:0;s:12:\"allowimgcode\";s:1:\"1\";s:14:\"allowmediacode\";s:1:\"1\";s:16:\"allowpostspecial\";i:31;s:16:\"attachextensions\";s:11:\"jpg,gif,rar\";}',''),
(3,'default','楂樼骇',3000,999999999,'','a:2:{s:4:\"post\";s:1:\"1\";s:5:\"reply\";s:1:\"1\";}','a:11:{s:13:\"alloweditpost\";s:1:\"1\";s:10:\"recyclebin\";s:1:\"1\";s:12:\"allowsmilies\";s:1:\"1\";s:9:\"allowhtml\";s:1:\"0\";s:11:\"allowbbcode\";s:1:\"1\";s:14:\"allowanonymous\";s:1:\"0\";s:6:\"jammer\";s:1:\"1\";s:12:\"allowimgcode\";s:1:\"1\";s:14:\"allowmediacode\";s:1:\"1\";s:16:\"allowpostspecial\";i:31;s:16:\"attachextensions\";s:31:\"jpg,gif,png,bmp,rar,doc,txt,zip\";}','');
/*!40000 ALTER TABLE `pre_forum_grouplevel` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_forum_groupuser`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_forum_groupuser` (
  `fid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `uid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `username` char(50) NOT NULL,
  `level` tinyint(3) unsigned NOT NULL DEFAULT 0,
  `threads` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `replies` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `joindateline` int(10) unsigned NOT NULL DEFAULT 0,
  `lastupdate` int(10) unsigned NOT NULL DEFAULT 0,
  `privacy` tinyint(1) NOT NULL DEFAULT 0,
  PRIMARY KEY (`fid`,`uid`),
  KEY `uid_lastupdate` (`uid`,`lastupdate`),
  KEY `userlist` (`fid`,`level`,`lastupdate`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_forum_groupuser` WRITE;
/*!40000 ALTER TABLE `pre_forum_groupuser` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_forum_groupuser` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_forum_hotreply_member`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_forum_hotreply_member` (
  `tid` int(10) unsigned NOT NULL DEFAULT 0,
  `pid` int(10) unsigned NOT NULL DEFAULT 0,
  `uid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `attitude` tinyint(1) NOT NULL DEFAULT 0,
  PRIMARY KEY (`pid`,`uid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_forum_hotreply_member` WRITE;
/*!40000 ALTER TABLE `pre_forum_hotreply_member` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_forum_hotreply_member` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_forum_hotreply_number`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_forum_hotreply_number` (
  `pid` int(10) unsigned NOT NULL DEFAULT 0,
  `tid` int(10) unsigned NOT NULL DEFAULT 0,
  `support` smallint(6) unsigned NOT NULL DEFAULT 0,
  `against` smallint(6) unsigned NOT NULL DEFAULT 0,
  `total` mediumint(8) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`pid`),
  KEY `tid` (`tid`,`total`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_forum_hotreply_number` WRITE;
/*!40000 ALTER TABLE `pre_forum_hotreply_number` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_forum_hotreply_number` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_forum_imagetype`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_forum_imagetype` (
  `typeid` smallint(6) unsigned NOT NULL AUTO_INCREMENT,
  `available` tinyint(1) NOT NULL DEFAULT 0,
  `name` char(20) NOT NULL,
  `type` enum('smiley','icon','avatar') NOT NULL DEFAULT 'smiley',
  `displayorder` tinyint(3) NOT NULL DEFAULT 0,
  `directory` char(100) NOT NULL,
  PRIMARY KEY (`typeid`)
) ENGINE=InnoDB AUTO_INCREMENT=5 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_forum_imagetype` WRITE;
/*!40000 ALTER TABLE `pre_forum_imagetype` DISABLE KEYS */;
INSERT INTO `pre_forum_imagetype` VALUES
(1,1,'榛樿','smiley',1,'default'),
(2,1,'閰风尨','smiley',2,'coolmonkey'),
(3,1,'鍛嗗憜鐢?,'smiley',3,'grapeman'),
(4,1,'QQ','smiley',4,'qq');
/*!40000 ALTER TABLE `pre_forum_imagetype` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_forum_medal`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_forum_medal` (
  `medalid` smallint(6) unsigned NOT NULL AUTO_INCREMENT,
  `name` varchar(50) NOT NULL DEFAULT '',
  `available` tinyint(1) NOT NULL DEFAULT 0,
  `image` varchar(255) NOT NULL DEFAULT '',
  `type` tinyint(1) NOT NULL DEFAULT 0,
  `displayorder` tinyint(3) NOT NULL DEFAULT 0,
  `description` varchar(255) NOT NULL,
  `expiration` smallint(6) unsigned NOT NULL DEFAULT 0,
  `permission` mediumtext NOT NULL,
  `credit` tinyint(3) unsigned NOT NULL DEFAULT 0,
  `price` mediumint(8) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`medalid`),
  KEY `displayorder` (`displayorder`),
  KEY `available` (`available`,`displayorder`)
) ENGINE=InnoDB AUTO_INCREMENT=11 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_forum_medal` WRITE;
/*!40000 ALTER TABLE `pre_forum_medal` DISABLE KEYS */;
INSERT INTO `pre_forum_medal` VALUES
(1,'鏈€浣虫柊浜?,0,'medal1.gif',0,0,'娉ㄥ唽璐﹀彿鍚庣Н鏋佸彂甯栫殑浼氬憳',0,'',0,0),
(2,'娲昏穬浼氬憳',0,'medal2.gif',0,0,'缁忓父鍙備笌鍚勭被璇濋鐨勮璁猴紝鍙戝笘鍐呭杈冩湁涓昏',0,'',0,0),
(3,'鐑績浼氬憳',0,'medal3.gif',0,0,'缁忓父甯姪鍏朵粬浼氬憳绛旂枒',0,'',0,0),
(4,'鎺ㄥ箍杈句汉',0,'medal4.gif',0,0,'绉瀬瀹ｄ紶鏈珯锛屼负鏈珯甯︽潵鏇村娉ㄥ唽浼氬憳',0,'',0,0),
(5,'瀹ｄ紶杈句汉',0,'medal5.gif',0,0,'绉瀬瀹ｄ紶鏈珯锛屼负鏈珯甯︽潵鏇村鐨勭敤鎴疯闂噺',0,'',0,0),
(6,'鐏屾按涔嬬帇',0,'medal6.gif',0,0,'缁忓父鍦ㄨ鍧涘彂甯栵紝涓斿彂甯栭噺杈冨ぇ',0,'',0,0),
(7,'绐佸嚭璐＄尞',0,'medal7.gif',0,0,'闀挎湡瀵硅鍧涚殑绻佽崳鑰屼笉鏂姫鍔涳紝鎴栧娆℃彁鍑哄缓璁炬€ф剰瑙?,0,'',0,0),
(8,'浼樼鐗堜富',0,'medal8.gif',0,0,'娲昏穬涓斿敖璐ｈ亴瀹堢殑鐗堜富',0,'',0,0),
(9,'鑽ｈ獕绠＄悊',0,'medal9.gif',0,0,'鏇剧粡涓鸿鍧涘仛鍑虹獊鍑鸿础鐚洰鍓嶅凡绂昏亴鐨勭増涓?,0,'',0,0),
(10,'璁哄潧鍏冭€?,0,'medal10.gif',0,0,'涓鸿鍧涘仛鍑虹獊鍑鸿础鐚殑浼氬憳',0,'',0,0);
/*!40000 ALTER TABLE `pre_forum_medal` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_forum_medallog`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_forum_medallog` (
  `id` mediumint(8) unsigned NOT NULL AUTO_INCREMENT,
  `uid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `medalid` smallint(6) unsigned NOT NULL DEFAULT 0,
  `type` tinyint(1) NOT NULL DEFAULT 0,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  `expiration` int(10) unsigned NOT NULL DEFAULT 0,
  `status` tinyint(1) NOT NULL DEFAULT 0,
  PRIMARY KEY (`id`),
  KEY `type` (`type`),
  KEY `status` (`status`,`expiration`),
  KEY `uid` (`uid`,`medalid`,`type`),
  KEY `dateline` (`dateline`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_forum_medallog` WRITE;
/*!40000 ALTER TABLE `pre_forum_medallog` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_forum_medallog` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_forum_memberrecommend`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_forum_memberrecommend` (
  `tid` int(10) unsigned NOT NULL,
  `recommenduid` mediumint(8) unsigned NOT NULL,
  `dateline` int(10) unsigned NOT NULL,
  KEY `tid` (`tid`),
  KEY `uid` (`recommenduid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_forum_memberrecommend` WRITE;
/*!40000 ALTER TABLE `pre_forum_memberrecommend` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_forum_memberrecommend` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_forum_moderator`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_forum_moderator` (
  `uid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `fid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `displayorder` tinyint(3) NOT NULL DEFAULT 0,
  `inherited` tinyint(1) NOT NULL DEFAULT 0,
  PRIMARY KEY (`uid`,`fid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_forum_moderator` WRITE;
/*!40000 ALTER TABLE `pre_forum_moderator` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_forum_moderator` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_forum_modwork`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_forum_modwork` (
  `uid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `modaction` char(3) NOT NULL DEFAULT '',
  `dateline` date NOT NULL DEFAULT '2006-01-01',
  `count` smallint(6) unsigned NOT NULL DEFAULT 0,
  `posts` smallint(6) unsigned NOT NULL DEFAULT 0,
  KEY `uid` (`uid`,`dateline`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_forum_modwork` WRITE;
/*!40000 ALTER TABLE `pre_forum_modwork` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_forum_modwork` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_forum_newthread`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_forum_newthread` (
  `tid` int(10) unsigned NOT NULL DEFAULT 0,
  `fid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`tid`),
  KEY `fid` (`fid`),
  KEY `dateline` (`dateline`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_forum_newthread` WRITE;
/*!40000 ALTER TABLE `pre_forum_newthread` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_forum_newthread` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_forum_onlinelist`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_forum_onlinelist` (
  `groupid` smallint(6) unsigned NOT NULL DEFAULT 0,
  `displayorder` tinyint(3) NOT NULL DEFAULT 0,
  `title` varchar(30) NOT NULL DEFAULT '',
  `url` varchar(30) NOT NULL DEFAULT ''
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_forum_onlinelist` WRITE;
/*!40000 ALTER TABLE `pre_forum_onlinelist` DISABLE KEYS */;
INSERT INTO `pre_forum_onlinelist` VALUES
(1,1,'绠＄悊鍛?,'online_admin.gif'),
(2,2,'瓒呯骇鐗堜富','online_supermod.gif'),
(3,3,'鐗堜富','online_moderator.gif'),
(0,4,'浼氬憳','online_member.gif');
/*!40000 ALTER TABLE `pre_forum_onlinelist` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_forum_order`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_forum_order` (
  `orderid` char(32) NOT NULL DEFAULT '',
  `status` char(3) NOT NULL DEFAULT '',
  `buyer` char(50) NOT NULL DEFAULT '',
  `admin` char(50) NOT NULL DEFAULT '',
  `uid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `amount` int(10) unsigned NOT NULL DEFAULT 0,
  `price` float(7,2) unsigned NOT NULL DEFAULT 0.00,
  `submitdate` int(10) unsigned NOT NULL DEFAULT 0,
  `confirmdate` int(10) unsigned NOT NULL DEFAULT 0,
  `email` varchar(255) NOT NULL DEFAULT '',
  `ip` varchar(45) NOT NULL DEFAULT '',
  `port` smallint(6) unsigned NOT NULL DEFAULT 0,
  UNIQUE KEY `orderid` (`orderid`),
  KEY `submitdate` (`submitdate`),
  KEY `uid` (`uid`,`submitdate`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_forum_order` WRITE;
/*!40000 ALTER TABLE `pre_forum_order` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_forum_order` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_forum_poll`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_forum_poll` (
  `tid` int(10) unsigned NOT NULL DEFAULT 0,
  `overt` tinyint(1) NOT NULL DEFAULT 0,
  `multiple` tinyint(1) NOT NULL DEFAULT 0,
  `visible` tinyint(1) NOT NULL DEFAULT 0,
  `maxchoices` tinyint(3) unsigned NOT NULL DEFAULT 0,
  `isimage` tinyint(1) NOT NULL DEFAULT 0,
  `expiration` int(10) unsigned NOT NULL DEFAULT 0,
  `pollpreview` varchar(255) NOT NULL DEFAULT '',
  `voters` mediumint(8) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`tid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_forum_poll` WRITE;
/*!40000 ALTER TABLE `pre_forum_poll` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_forum_poll` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_forum_polloption`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_forum_polloption` (
  `polloptionid` int(10) unsigned NOT NULL AUTO_INCREMENT,
  `tid` int(10) unsigned NOT NULL DEFAULT 0,
  `votes` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `displayorder` tinyint(3) NOT NULL DEFAULT 0,
  `polloption` varchar(80) NOT NULL DEFAULT '',
  `voterids` mediumtext NOT NULL,
  PRIMARY KEY (`polloptionid`),
  KEY `tid` (`tid`,`displayorder`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_forum_polloption` WRITE;
/*!40000 ALTER TABLE `pre_forum_polloption` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_forum_polloption` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_forum_polloption_image`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_forum_polloption_image` (
  `aid` int(10) unsigned NOT NULL AUTO_INCREMENT,
  `poid` int(10) unsigned NOT NULL DEFAULT 0,
  `tid` int(10) unsigned NOT NULL DEFAULT 0,
  `pid` int(10) unsigned NOT NULL DEFAULT 0,
  `uid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `filename` varchar(255) NOT NULL DEFAULT '',
  `filesize` int(10) unsigned NOT NULL DEFAULT 0,
  `attachment` varchar(255) NOT NULL DEFAULT '',
  `remote` tinyint(1) NOT NULL DEFAULT 0,
  `width` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `height` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `thumb` tinyint(1) NOT NULL DEFAULT 0,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`aid`),
  KEY `poid` (`poid`),
  KEY `tid` (`tid`),
  KEY `uid` (`uid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_forum_polloption_image` WRITE;
/*!40000 ALTER TABLE `pre_forum_polloption_image` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_forum_polloption_image` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_forum_pollvoter`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_forum_pollvoter` (
  `tid` int(10) unsigned NOT NULL DEFAULT 0,
  `uid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `username` varchar(50) NOT NULL DEFAULT '',
  `options` text NOT NULL,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  KEY `tid` (`tid`),
  KEY `uid` (`uid`,`dateline`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_forum_pollvoter` WRITE;
/*!40000 ALTER TABLE `pre_forum_pollvoter` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_forum_pollvoter` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_forum_post`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_forum_post` (
  `pid` int(10) unsigned NOT NULL,
  `fid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `tid` int(10) unsigned NOT NULL DEFAULT 0,
  `repid` int(10) unsigned NOT NULL DEFAULT 0,
  `first` tinyint(1) NOT NULL DEFAULT 0,
  `author` varchar(50) NOT NULL DEFAULT '',
  `authorid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `subject` varchar(255) NOT NULL DEFAULT '',
  `original` tinyint(1) NOT NULL DEFAULT 0,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  `lastupdate` int(10) unsigned NOT NULL DEFAULT 0,
  `updateuid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `premsg` text NOT NULL,
  `message` mediumtext NOT NULL,
  `content` longtext CHARACTER SET utf8mb4 COLLATE utf8mb4_bin DEFAULT NULL CHECK (json_valid(`content`)),
  `source` longtext CHARACTER SET utf8mb4 COLLATE utf8mb4_bin DEFAULT NULL CHECK (json_valid(`source`)),
  `useip` varchar(45) NOT NULL DEFAULT '',
  `port` smallint(6) unsigned NOT NULL DEFAULT 0,
  `invisible` tinyint(1) NOT NULL DEFAULT 0,
  `anonymous` tinyint(1) NOT NULL DEFAULT 0,
  `usesig` tinyint(1) NOT NULL DEFAULT 0,
  `htmlon` tinyint(1) NOT NULL DEFAULT 0,
  `bbcodeoff` tinyint(1) NOT NULL DEFAULT 0,
  `smileyoff` tinyint(1) NOT NULL DEFAULT 0,
  `parseurloff` tinyint(1) NOT NULL DEFAULT 0,
  `attachment` tinyint(1) NOT NULL DEFAULT 0,
  `rate` smallint(6) NOT NULL DEFAULT 0,
  `ratetimes` tinyint(3) unsigned NOT NULL DEFAULT 0,
  `status` int(10) NOT NULL DEFAULT 0,
  `tags` varchar(255) NOT NULL DEFAULT '',
  `comment` tinyint(1) NOT NULL DEFAULT 0,
  `replycredit` int(10) NOT NULL DEFAULT 0,
  `position` int(10) unsigned NOT NULL,
  `bestanswer` tinyint(1) NOT NULL,
  PRIMARY KEY (`tid`,`position`),
  UNIQUE KEY `pid` (`pid`),
  KEY `fid` (`fid`),
  KEY `authorid` (`authorid`,`invisible`),
  KEY `dateline` (`dateline`),
  KEY `invisible` (`invisible`),
  KEY `displayorder` (`tid`,`invisible`,`dateline`),
  KEY `first` (`tid`,`first`),
  KEY `bestanswer` (`bestanswer`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_forum_post` WRITE;
/*!40000 ALTER TABLE `pre_forum_post` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_forum_post` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_forum_post_history`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_forum_post_history` (
  `id` int(10) unsigned NOT NULL,
  `pid` int(10) unsigned NOT NULL,
  `dateline` int(10) unsigned NOT NULL,
  `subject` varchar(255) NOT NULL DEFAULT '',
  `message` mediumtext NOT NULL,
  PRIMARY KEY (`id`),
  KEY `pid` (`pid`,`dateline`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_forum_post_history` WRITE;
/*!40000 ALTER TABLE `pre_forum_post_history` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_forum_post_history` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_forum_post_location`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_forum_post_location` (
  `pid` int(10) unsigned NOT NULL DEFAULT 0,
  `tid` int(10) unsigned DEFAULT 0,
  `uid` mediumint(8) unsigned DEFAULT 0,
  `mapx` varchar(255) NOT NULL,
  `mapy` varchar(255) NOT NULL,
  `location` varchar(255) NOT NULL,
  PRIMARY KEY (`pid`),
  KEY `tid` (`tid`),
  KEY `uid` (`uid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_forum_post_location` WRITE;
/*!40000 ALTER TABLE `pre_forum_post_location` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_forum_post_location` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_forum_post_moderate`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_forum_post_moderate` (
  `id` int(10) unsigned NOT NULL DEFAULT 0,
  `status` tinyint(3) NOT NULL DEFAULT 0,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`id`),
  KEY `status` (`status`,`dateline`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_forum_post_moderate` WRITE;
/*!40000 ALTER TABLE `pre_forum_post_moderate` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_forum_post_moderate` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_forum_post_tableid`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_forum_post_tableid` (
  `pid` int(10) unsigned NOT NULL AUTO_INCREMENT,
  PRIMARY KEY (`pid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_forum_post_tableid` WRITE;
/*!40000 ALTER TABLE `pre_forum_post_tableid` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_forum_post_tableid` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_forum_postcache`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_forum_postcache` (
  `pid` int(10) unsigned NOT NULL,
  `comment` mediumtext NOT NULL,
  `rate` mediumtext NOT NULL,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`pid`),
  KEY `dateline` (`dateline`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_forum_postcache` WRITE;
/*!40000 ALTER TABLE `pre_forum_postcache` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_forum_postcache` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_forum_postcomment`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_forum_postcomment` (
  `id` int(10) unsigned NOT NULL AUTO_INCREMENT,
  `tid` int(10) unsigned NOT NULL DEFAULT 0,
  `pid` int(10) unsigned NOT NULL DEFAULT 0,
  `author` varchar(50) NOT NULL DEFAULT '',
  `authorid` mediumint(8) NOT NULL DEFAULT 0,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  `comment` varchar(255) NOT NULL DEFAULT '',
  `score` tinyint(1) NOT NULL DEFAULT 0,
  `useip` varchar(45) NOT NULL DEFAULT '',
  `port` smallint(6) unsigned NOT NULL DEFAULT 0,
  `rpid` int(10) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`id`),
  KEY `tid` (`tid`),
  KEY `authorid` (`authorid`),
  KEY `score` (`score`),
  KEY `rpid` (`rpid`),
  KEY `pid` (`pid`,`dateline`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_forum_postcomment` WRITE;
/*!40000 ALTER TABLE `pre_forum_postcomment` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_forum_postcomment` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_forum_poststick`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_forum_poststick` (
  `tid` int(10) unsigned NOT NULL,
  `pid` int(10) unsigned NOT NULL,
  `position` int(10) unsigned NOT NULL,
  `dateline` int(10) unsigned NOT NULL,
  PRIMARY KEY (`tid`,`pid`),
  KEY `dateline` (`tid`,`dateline`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_forum_poststick` WRITE;
/*!40000 ALTER TABLE `pre_forum_poststick` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_forum_poststick` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_forum_promotion`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_forum_promotion` (
  `ip` varchar(45) NOT NULL DEFAULT '',
  `port` smallint(6) unsigned NOT NULL DEFAULT 0,
  `uid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `username` char(50) NOT NULL DEFAULT '',
  PRIMARY KEY (`ip`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_forum_promotion` WRITE;
/*!40000 ALTER TABLE `pre_forum_promotion` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_forum_promotion` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_forum_ratelog`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_forum_ratelog` (
  `pid` int(10) unsigned NOT NULL DEFAULT 0,
  `uid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `username` char(50) NOT NULL DEFAULT '',
  `extcredits` tinyint(3) unsigned NOT NULL DEFAULT 0,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  `score` smallint(6) NOT NULL DEFAULT 0,
  `reason` char(40) NOT NULL DEFAULT '',
  KEY `pid` (`pid`,`dateline`),
  KEY `dateline` (`dateline`),
  KEY `uid` (`uid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_forum_ratelog` WRITE;
/*!40000 ALTER TABLE `pre_forum_ratelog` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_forum_ratelog` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_forum_relatedthread`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_forum_relatedthread` (
  `tid` int(10) unsigned NOT NULL DEFAULT 0,
  `type` enum('general','trade') NOT NULL DEFAULT 'general',
  `expiration` int(10) NOT NULL DEFAULT 0,
  `keywords` varchar(255) NOT NULL DEFAULT '',
  `relatedthreads` text NOT NULL,
  PRIMARY KEY (`tid`,`type`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_forum_relatedthread` WRITE;
/*!40000 ALTER TABLE `pre_forum_relatedthread` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_forum_relatedthread` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_forum_replycredit`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_forum_replycredit` (
  `tid` int(10) unsigned NOT NULL,
  `extcredits` int(10) unsigned NOT NULL DEFAULT 0,
  `extcreditstype` tinyint(1) NOT NULL DEFAULT 0,
  `times` int(10) unsigned NOT NULL DEFAULT 0,
  `membertimes` int(10) unsigned NOT NULL DEFAULT 0,
  `random` tinyint(1) NOT NULL DEFAULT 0,
  PRIMARY KEY (`tid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_forum_replycredit` WRITE;
/*!40000 ALTER TABLE `pre_forum_replycredit` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_forum_replycredit` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_forum_rsscache`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_forum_rsscache` (
  `lastupdate` int(10) unsigned NOT NULL DEFAULT 0,
  `fid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `tid` int(10) unsigned NOT NULL DEFAULT 0,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  `forum` char(50) NOT NULL DEFAULT '',
  `author` char(50) NOT NULL DEFAULT '',
  `subject` varchar(255) NOT NULL DEFAULT '',
  `description` char(255) NOT NULL DEFAULT '',
  `guidetype` char(10) NOT NULL DEFAULT '',
  UNIQUE KEY `tid` (`tid`),
  KEY `fid` (`fid`,`dateline`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_forum_rsscache` WRITE;
/*!40000 ALTER TABLE `pre_forum_rsscache` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_forum_rsscache` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_forum_sofa`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_forum_sofa` (
  `tid` int(10) unsigned NOT NULL DEFAULT 0,
  `fid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`tid`),
  KEY `ftid` (`fid`,`tid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_forum_sofa` WRITE;
/*!40000 ALTER TABLE `pre_forum_sofa` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_forum_sofa` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_forum_spacecache`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_forum_spacecache` (
  `uid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `variable` varchar(20) NOT NULL,
  `value` text NOT NULL,
  `expiration` int(10) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`uid`,`variable`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_forum_spacecache` WRITE;
/*!40000 ALTER TABLE `pre_forum_spacecache` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_forum_spacecache` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_forum_statlog`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_forum_statlog` (
  `logdate` date NOT NULL,
  `fid` mediumint(8) unsigned NOT NULL,
  `type` smallint(5) unsigned NOT NULL DEFAULT 0,
  `value` int(10) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`logdate`,`fid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_forum_statlog` WRITE;
/*!40000 ALTER TABLE `pre_forum_statlog` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_forum_statlog` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_forum_thread`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_forum_thread` (
  `tid` int(10) unsigned NOT NULL AUTO_INCREMENT,
  `fid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `posttableid` smallint(6) unsigned NOT NULL DEFAULT 0,
  `typeid` smallint(6) unsigned NOT NULL DEFAULT 0,
  `sortid` smallint(6) unsigned NOT NULL DEFAULT 0,
  `readperm` tinyint(3) unsigned NOT NULL DEFAULT 0,
  `price` smallint(6) NOT NULL DEFAULT 0,
  `author` char(50) NOT NULL DEFAULT '',
  `authorid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `subject` varchar(255) NOT NULL DEFAULT '',
  `summary` varchar(255) NOT NULL DEFAULT '',
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  `lastpost` int(10) unsigned NOT NULL DEFAULT 0,
  `lastposter` char(50) NOT NULL DEFAULT '',
  `views` int(10) unsigned NOT NULL DEFAULT 0,
  `replies` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `displayorder` tinyint(1) NOT NULL DEFAULT 0,
  `highlight` tinyint(1) NOT NULL DEFAULT 0,
  `digest` tinyint(1) NOT NULL DEFAULT 0,
  `rate` tinyint(1) NOT NULL DEFAULT 0,
  `special` tinyint(1) NOT NULL DEFAULT 0,
  `attachment` tinyint(1) NOT NULL DEFAULT 0,
  `moderated` tinyint(1) NOT NULL DEFAULT 0,
  `closed` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `stickreply` tinyint(1) NOT NULL DEFAULT 0,
  `recommends` smallint(6) NOT NULL DEFAULT 0,
  `recommend_add` smallint(6) NOT NULL DEFAULT 0,
  `recommend_sub` smallint(6) NOT NULL DEFAULT 0,
  `heats` int(10) unsigned NOT NULL DEFAULT 0,
  `status` smallint(6) unsigned NOT NULL DEFAULT 0,
  `isgroup` tinyint(1) NOT NULL DEFAULT 0,
  `favtimes` mediumint(8) NOT NULL DEFAULT 0,
  `sharetimes` mediumint(8) NOT NULL DEFAULT 0,
  `stamp` tinyint(3) NOT NULL DEFAULT -1,
  `icon` tinyint(3) NOT NULL DEFAULT -1,
  `pushedaid` mediumint(8) NOT NULL DEFAULT 0,
  `cover` smallint(6) NOT NULL DEFAULT 0,
  `replycredit` int(10) NOT NULL DEFAULT 0,
  `relatebytag` char(255) NOT NULL DEFAULT '0',
  `maxposition` int(10) unsigned NOT NULL DEFAULT 0,
  `bgcolor` char(8) NOT NULL DEFAULT '',
  `comments` int(10) unsigned NOT NULL DEFAULT 0,
  `hidden` smallint(6) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`tid`),
  KEY `digest` (`digest`),
  KEY `sortid` (`sortid`),
  KEY `displayorder` (`fid`,`displayorder`,`lastpost`),
  KEY `typeid` (`fid`,`typeid`,`displayorder`,`lastpost`),
  KEY `recommends` (`recommends`),
  KEY `heats` (`heats`),
  KEY `authorid` (`authorid`),
  KEY `isgroup` (`isgroup`,`lastpost`),
  KEY `special` (`special`),
  KEY `displayorder_dateline` (`fid`,`displayorder`,`dateline`),
  KEY `displayorder_replies` (`fid`,`displayorder`,`replies`),
  KEY `displayorder_views` (`fid`,`displayorder`,`views`),
  KEY `displayorder_recommends` (`fid`,`displayorder`,`recommends`),
  KEY `displayorder_heats` (`fid`,`displayorder`,`heats`),
  KEY `typeid_dateline` (`fid`,`typeid`,`displayorder`,`dateline`),
  KEY `typeid_replies` (`fid`,`typeid`,`displayorder`,`replies`),
  KEY `typeid_views` (`fid`,`typeid`,`displayorder`,`views`),
  KEY `typeid_recommends` (`fid`,`typeid`,`displayorder`,`recommends`),
  KEY `typeid_heats` (`fid`,`typeid`,`displayorder`,`heats`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_forum_thread` WRITE;
/*!40000 ALTER TABLE `pre_forum_thread` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_forum_thread` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_forum_thread_moderate`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_forum_thread_moderate` (
  `id` int(10) unsigned NOT NULL DEFAULT 0,
  `status` tinyint(3) NOT NULL DEFAULT 0,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`id`),
  KEY `status` (`status`,`dateline`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_forum_thread_moderate` WRITE;
/*!40000 ALTER TABLE `pre_forum_thread_moderate` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_forum_thread_moderate` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_forum_threadaddviews`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_forum_threadaddviews` (
  `tid` int(10) unsigned NOT NULL DEFAULT 0,
  `addviews` int(10) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`tid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_forum_threadaddviews` WRITE;
/*!40000 ALTER TABLE `pre_forum_threadaddviews` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_forum_threadaddviews` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_forum_threadcalendar`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_forum_threadcalendar` (
  `cid` mediumint(8) unsigned NOT NULL AUTO_INCREMENT,
  `fid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  `hotnum` int(10) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`cid`),
  KEY `fid` (`fid`,`dateline`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_forum_threadcalendar` WRITE;
/*!40000 ALTER TABLE `pre_forum_threadcalendar` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_forum_threadcalendar` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_forum_threadclass`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_forum_threadclass` (
  `typeid` mediumint(8) unsigned NOT NULL AUTO_INCREMENT,
  `fid` mediumint(8) unsigned NOT NULL,
  `name` varchar(255) NOT NULL,
  `displayorder` mediumint(9) NOT NULL,
  `icon` varchar(255) NOT NULL,
  `moderators` tinyint(1) NOT NULL DEFAULT 0,
  PRIMARY KEY (`typeid`),
  KEY `fid` (`fid`,`displayorder`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_forum_threadclass` WRITE;
/*!40000 ALTER TABLE `pre_forum_threadclass` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_forum_threadclass` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_forum_threadclosed`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_forum_threadclosed` (
  `tid` int(10) unsigned NOT NULL DEFAULT 0,
  `redirect` mediumint(8) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`tid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_forum_threadclosed` WRITE;
/*!40000 ALTER TABLE `pre_forum_threadclosed` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_forum_threadclosed` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_forum_threaddisablepos`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_forum_threaddisablepos` (
  `tid` int(10) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`tid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_forum_threaddisablepos` WRITE;
/*!40000 ALTER TABLE `pre_forum_threaddisablepos` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_forum_threaddisablepos` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_forum_threadhidelog`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_forum_threadhidelog` (
  `tid` int(10) unsigned NOT NULL DEFAULT 0,
  `uid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  UNIQUE KEY `uid` (`tid`,`uid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_forum_threadhidelog` WRITE;
/*!40000 ALTER TABLE `pre_forum_threadhidelog` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_forum_threadhidelog` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_forum_threadhot`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_forum_threadhot` (
  `cid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `fid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `tid` int(10) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`cid`,`tid`),
  KEY `fid` (`fid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_forum_threadhot` WRITE;
/*!40000 ALTER TABLE `pre_forum_threadhot` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_forum_threadhot` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_forum_threadimage`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_forum_threadimage` (
  `tid` int(10) unsigned NOT NULL DEFAULT 0,
  `attachment` varchar(255) NOT NULL DEFAULT '',
  `remote` tinyint(1) NOT NULL DEFAULT 0,
  KEY `tid` (`tid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_forum_threadimage` WRITE;
/*!40000 ALTER TABLE `pre_forum_threadimage` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_forum_threadimage` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_forum_threadmod`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_forum_threadmod` (
  `tid` int(10) unsigned NOT NULL DEFAULT 0,
  `uid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `username` char(50) NOT NULL DEFAULT '',
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  `expiration` int(10) unsigned NOT NULL DEFAULT 0,
  `action` char(5) NOT NULL DEFAULT '',
  `status` tinyint(1) NOT NULL DEFAULT 0,
  `magicid` smallint(6) unsigned NOT NULL,
  `stamp` tinyint(3) NOT NULL,
  `reason` char(40) NOT NULL DEFAULT '',
  KEY `tid` (`tid`,`dateline`),
  KEY `expiration` (`expiration`,`status`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_forum_threadmod` WRITE;
/*!40000 ALTER TABLE `pre_forum_threadmod` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_forum_threadmod` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_forum_threadpartake`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_forum_threadpartake` (
  `tid` int(10) unsigned NOT NULL DEFAULT 0,
  `uid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  KEY `tid` (`tid`,`uid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_forum_threadpartake` WRITE;
/*!40000 ALTER TABLE `pre_forum_threadpartake` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_forum_threadpartake` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_forum_threadpreview`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_forum_threadpreview` (
  `tid` int(10) unsigned NOT NULL DEFAULT 0,
  `relay` int(10) unsigned NOT NULL DEFAULT 0,
  `content` text NOT NULL,
  PRIMARY KEY (`tid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_forum_threadpreview` WRITE;
/*!40000 ALTER TABLE `pre_forum_threadpreview` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_forum_threadpreview` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_forum_threadprofile`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_forum_threadprofile` (
  `id` mediumint(8) unsigned NOT NULL AUTO_INCREMENT,
  `name` char(100) NOT NULL DEFAULT '',
  `template` text NOT NULL,
  `global` tinyint(1) NOT NULL DEFAULT 0,
  PRIMARY KEY (`id`),
  KEY `global` (`global`)
) ENGINE=InnoDB AUTO_INCREMENT=2 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_forum_threadprofile` WRITE;
/*!40000 ALTER TABLE `pre_forum_threadprofile` DISABLE KEYS */;
INSERT INTO `pre_forum_threadprofile` VALUES
(1,'榛樿鏂规','a:2:{s:4:\"left\";s:399:\"{numbercard}\r\n{groupicon}<p>{*}</p>{/groupicon}\r\n{authortitle}<p><em>{*}</em></p>{/authortitle}\r\n{customstatus}<p class=\"xg1\">{*}</p>{/customstatus}\r\n{star}<p>{*}</p>{/star}\r\n{upgradeprogress}<p>{*}</p>{/upgradeprogress}\r\n<dl class=\"pil cl\">\r\n	<dt>{baseinfo=credits,1}</dt><dd>{baseinfo=credits,0}</dd>\r\n</dl>\r\n{medal}<p class=\"md_ctrl\">{*}</p>{/medal}\r\n<dl class=\"pil cl\">{baseinfo=field_qq,0}</dl>\";s:3:\"top\";s:82:\"<dl class=\"cl\">\r\n<dt>{baseinfo=credits,1}</dt><dd>{baseinfo=credits,0}</dd>\r\n</dl>\";}',1);
/*!40000 ALTER TABLE `pre_forum_threadprofile` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_forum_threadprofile_group`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_forum_threadprofile_group` (
  `gid` mediumint(8) NOT NULL,
  `tpid` mediumint(8) unsigned NOT NULL,
  PRIMARY KEY (`gid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_forum_threadprofile_group` WRITE;
/*!40000 ALTER TABLE `pre_forum_threadprofile_group` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_forum_threadprofile_group` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_forum_threadrush`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_forum_threadrush` (
  `tid` int(10) unsigned NOT NULL DEFAULT 0,
  `stopfloor` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `starttimefrom` int(10) unsigned NOT NULL DEFAULT 0,
  `starttimeto` int(10) unsigned NOT NULL DEFAULT 0,
  `rewardfloor` text NOT NULL,
  `creditlimit` int(10) NOT NULL DEFAULT -996,
  `replylimit` smallint(6) NOT NULL DEFAULT 0,
  PRIMARY KEY (`tid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_forum_threadrush` WRITE;
/*!40000 ALTER TABLE `pre_forum_threadrush` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_forum_threadrush` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_forum_threadtype`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_forum_threadtype` (
  `typeid` smallint(6) unsigned NOT NULL AUTO_INCREMENT,
  `fid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `displayorder` smallint(6) NOT NULL DEFAULT 0,
  `name` varchar(255) NOT NULL DEFAULT '',
  `description` varchar(255) NOT NULL DEFAULT '',
  `icon` varchar(255) NOT NULL DEFAULT '',
  `special` smallint(6) NOT NULL DEFAULT 0,
  `modelid` smallint(6) unsigned NOT NULL DEFAULT 0,
  `expiration` tinyint(1) NOT NULL DEFAULT 0,
  `template` text NOT NULL,
  `stemplate` text NOT NULL,
  `ptemplate` text NOT NULL,
  `btemplate` text NOT NULL,
  `super` text NOT NULL,
  PRIMARY KEY (`typeid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_forum_threadtype` WRITE;
/*!40000 ALTER TABLE `pre_forum_threadtype` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_forum_threadtype` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_forum_trade`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_forum_trade` (
  `tid` int(10) unsigned NOT NULL,
  `pid` int(10) unsigned NOT NULL,
  `typeid` smallint(6) unsigned NOT NULL,
  `sellerid` mediumint(8) unsigned NOT NULL,
  `seller` char(50) NOT NULL,
  `account` char(50) NOT NULL,
  `tenpayaccount` char(20) NOT NULL DEFAULT '',
  `subject` char(100) NOT NULL,
  `price` decimal(8,2) NOT NULL,
  `amount` smallint(6) unsigned NOT NULL DEFAULT 1,
  `quality` tinyint(3) unsigned NOT NULL DEFAULT 0,
  `locus` char(20) NOT NULL,
  `transport` tinyint(1) NOT NULL DEFAULT 0,
  `ordinaryfee` smallint(4) unsigned NOT NULL DEFAULT 0,
  `expressfee` smallint(4) unsigned NOT NULL DEFAULT 0,
  `emsfee` smallint(4) unsigned NOT NULL DEFAULT 0,
  `itemtype` tinyint(1) NOT NULL DEFAULT 0,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  `expiration` int(10) unsigned NOT NULL DEFAULT 0,
  `lastbuyer` char(50) NOT NULL,
  `lastupdate` int(10) unsigned NOT NULL DEFAULT 0,
  `totalitems` smallint(5) unsigned NOT NULL DEFAULT 0,
  `tradesum` decimal(8,2) NOT NULL DEFAULT 0.00,
  `closed` tinyint(1) NOT NULL DEFAULT 0,
  `aid` int(10) unsigned NOT NULL,
  `displayorder` tinyint(1) NOT NULL,
  `costprice` decimal(8,2) NOT NULL,
  `credit` int(10) unsigned NOT NULL DEFAULT 0,
  `costcredit` int(10) unsigned NOT NULL DEFAULT 0,
  `credittradesum` int(10) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`tid`,`pid`),
  KEY `pid` (`pid`),
  KEY `sellerid` (`sellerid`),
  KEY `totalitems` (`totalitems`),
  KEY `tradesum` (`tradesum`),
  KEY `displayorder` (`tid`,`displayorder`),
  KEY `sellertrades` (`sellerid`,`tradesum`,`totalitems`),
  KEY `typeid` (`typeid`),
  KEY `credittradesum` (`credittradesum`),
  KEY `expiration` (`expiration`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_forum_trade` WRITE;
/*!40000 ALTER TABLE `pre_forum_trade` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_forum_trade` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_forum_tradecomment`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_forum_tradecomment` (
  `id` mediumint(8) NOT NULL AUTO_INCREMENT,
  `orderid` char(32) NOT NULL,
  `pid` int(10) unsigned NOT NULL,
  `type` tinyint(1) NOT NULL,
  `raterid` mediumint(8) unsigned NOT NULL,
  `rater` char(50) NOT NULL,
  `rateeid` mediumint(8) unsigned NOT NULL,
  `ratee` char(50) NOT NULL,
  `message` char(200) NOT NULL,
  `explanation` char(200) NOT NULL,
  `score` tinyint(1) NOT NULL,
  `dateline` int(10) unsigned NOT NULL,
  PRIMARY KEY (`id`),
  KEY `raterid` (`raterid`,`type`,`dateline`),
  KEY `rateeid` (`rateeid`,`type`,`dateline`),
  KEY `orderid` (`orderid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_forum_tradecomment` WRITE;
/*!40000 ALTER TABLE `pre_forum_tradecomment` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_forum_tradecomment` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_forum_tradelog`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_forum_tradelog` (
  `tid` int(10) unsigned NOT NULL,
  `pid` int(10) unsigned NOT NULL,
  `orderid` varchar(32) NOT NULL,
  `tradeno` varchar(32) NOT NULL,
  `paytype` tinyint(3) unsigned NOT NULL DEFAULT 0,
  `subject` varchar(100) NOT NULL,
  `price` decimal(8,2) NOT NULL DEFAULT 0.00,
  `quality` tinyint(3) unsigned NOT NULL DEFAULT 0,
  `itemtype` tinyint(1) NOT NULL DEFAULT 0,
  `number` smallint(5) unsigned NOT NULL DEFAULT 0,
  `tax` decimal(6,2) unsigned NOT NULL DEFAULT 0.00,
  `locus` varchar(100) NOT NULL,
  `sellerid` mediumint(8) unsigned NOT NULL,
  `seller` varchar(50) NOT NULL,
  `selleraccount` varchar(50) NOT NULL,
  `tenpayaccount` varchar(20) NOT NULL DEFAULT '0',
  `buyerid` mediumint(8) unsigned NOT NULL,
  `buyer` varchar(50) NOT NULL,
  `buyercontact` varchar(50) NOT NULL,
  `buyercredits` smallint(5) unsigned NOT NULL DEFAULT 0,
  `buyermsg` varchar(200) DEFAULT NULL,
  `status` tinyint(1) NOT NULL DEFAULT 0,
  `lastupdate` int(10) unsigned NOT NULL DEFAULT 0,
  `offline` tinyint(1) NOT NULL DEFAULT 0,
  `buyername` varchar(50) NOT NULL,
  `buyerzip` varchar(10) NOT NULL,
  `buyerphone` varchar(20) NOT NULL,
  `buyermobile` varchar(20) NOT NULL,
  `transport` tinyint(1) NOT NULL DEFAULT 0,
  `transportfee` smallint(6) unsigned NOT NULL DEFAULT 0,
  `baseprice` decimal(8,2) NOT NULL,
  `discount` tinyint(1) NOT NULL DEFAULT 0,
  `ratestatus` tinyint(1) NOT NULL DEFAULT 0,
  `message` text NOT NULL,
  `credit` int(10) unsigned NOT NULL DEFAULT 0,
  `basecredit` int(10) unsigned NOT NULL DEFAULT 0,
  UNIQUE KEY `orderid` (`orderid`),
  KEY `sellerid` (`sellerid`),
  KEY `buyerid` (`buyerid`),
  KEY `status` (`status`),
  KEY `buyerlog` (`buyerid`,`status`,`lastupdate`),
  KEY `sellerlog` (`sellerid`,`status`,`lastupdate`),
  KEY `tid` (`tid`,`pid`),
  KEY `pid` (`pid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_forum_tradelog` WRITE;
/*!40000 ALTER TABLE `pre_forum_tradelog` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_forum_tradelog` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_forum_typeoption`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_forum_typeoption` (
  `optionid` smallint(6) unsigned NOT NULL AUTO_INCREMENT,
  `classid` smallint(6) unsigned NOT NULL DEFAULT 0,
  `displayorder` tinyint(3) NOT NULL DEFAULT 0,
  `expiration` tinyint(1) NOT NULL,
  `protect` varchar(255) NOT NULL,
  `title` varchar(255) NOT NULL DEFAULT '',
  `description` varchar(255) NOT NULL DEFAULT '',
  `identifier` varchar(255) NOT NULL DEFAULT '',
  `type` varchar(255) NOT NULL DEFAULT '',
  `unit` varchar(255) NOT NULL,
  `rules` mediumtext NOT NULL,
  `permprompt` mediumtext NOT NULL,
  PRIMARY KEY (`optionid`),
  KEY `classid` (`classid`)
) ENGINE=InnoDB AUTO_INCREMENT=69 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_forum_typeoption` WRITE;
/*!40000 ALTER TABLE `pre_forum_typeoption` DISABLE KEYS */;
INSERT INTO `pre_forum_typeoption` VALUES
(1,0,0,0,'','閫氱敤绫?,'','','','','',''),
(2,0,0,0,'','鎴夸骇绫?,'','','','','',''),
(3,0,0,0,'','浜ゅ弸绫?,'','','','','',''),
(4,0,0,0,'','姹傝亴鎷涜仒绫?,'','','','','',''),
(5,0,0,0,'','浜ゆ槗绫?,'','','','','',''),
(6,0,0,0,'','浜掕仈缃戠被','','','','','',''),
(7,1,0,0,'','濮撳悕','','name','text','','',''),
(9,1,0,0,'','骞撮緞','','age','number','','',''),
(10,1,0,0,'','鍦板潃','','address','text','','',''),
(11,1,0,0,'','QQ','','qq','number','','',''),
(12,1,0,0,'','閭','','mail','email','','',''),
(13,1,0,0,'','鐢佃瘽','','phone','text','','',''),
(14,5,0,0,'','鍩硅璐圭敤','','teach_pay','text','','',''),
(15,5,0,0,'','鍩硅鏃堕棿','','teach_time','text','','',''),
(20,2,0,0,'','妤煎眰','','floor','number','','',''),
(21,2,0,0,'','浜ら€氱姸鍐?,'','traf','textarea','','',''),
(22,2,0,0,'','鍦板浘','','images','image','','',''),
(24,2,0,0,'','浠锋牸','','price','text','','',''),
(26,5,0,0,'','鍩硅鍚嶇О','','teach_name','text','','',''),
(28,3,0,0,'','韬珮','','heighth','number','','',''),
(29,3,0,0,'','浣撻噸','','weighth','number','','',''),
(33,1,0,0,'','鐓х墖','','photo','image','','',''),
(35,5,0,0,'','鏈嶅姟鏂瑰紡','','service_type','text','','',''),
(36,5,0,0,'','鏈嶅姟鏃堕棿','','service_time','text','','',''),
(37,5,0,0,'','鏈嶅姟璐圭敤','','service_pay','text','','',''),
(39,6,0,0,'','缃戝潃','','site_url','url','','',''),
(40,6,0,0,'','鐢靛瓙閭欢','','site_mail','email','','',''),
(42,6,0,0,'','缃戠珯鍚嶇О','','site_name','text','','',''),
(46,4,0,0,'','鑱屼綅','','recr_intend','text','','',''),
(47,4,0,0,'','宸ヤ綔鍦扮偣','','recr_palce','text','','',''),
(49,4,0,0,'','鏈夋晥鏈熻嚦','','recr_end','calendar','','',''),
(51,4,0,0,'','鍏徃鍚嶇О','','recr_com','text','','',''),
(52,4,0,0,'','骞撮緞瑕佹眰','','recr_age','text','','',''),
(54,4,0,0,'','涓撲笟','','recr_abli','text','','',''),
(55,5,0,0,'','濮嬪彂','','leaves','text','','',''),
(56,5,0,0,'','缁堢偣','','boundfor','text','','',''),
(57,6,0,0,'','Alexa鎺掑悕','','site_top','number','','',''),
(58,5,0,0,'','杞︽/鑸彮','','train_no','text','','',''),
(59,5,0,0,'','鏁伴噺','','trade_num','number','','',''),
(60,5,0,0,'','浠锋牸','','trade_price','text','','',''),
(61,5,0,0,'','鏈夋晥鏈熻嚦','','trade_end','calendar','','',''),
(63,1,0,0,'','璇︾粏鎻忚堪','','detail_content','textarea','','',''),
(64,1,0,0,'','绫嶈疮','','born_place','text','','',''),
(65,2,0,0,'','绉熼噾','','money','text','','',''),
(66,2,0,0,'','闈㈢Н','','acreage','text','','',''),
(67,5,0,0,'','鍙戣溅鏃堕棿','','time','calendar','','N;',''),
(68,1,0,0,'','鎵€鍦ㄥ湴','','now_place','text','','','');
/*!40000 ALTER TABLE `pre_forum_typeoption` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_forum_typeoptionvar`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_forum_typeoptionvar` (
  `sortid` smallint(6) unsigned NOT NULL DEFAULT 0,
  `tid` int(10) unsigned NOT NULL DEFAULT 0,
  `fid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `optionid` smallint(6) unsigned NOT NULL DEFAULT 0,
  `expiration` int(10) unsigned NOT NULL DEFAULT 0,
  `value` mediumtext NOT NULL,
  KEY `sortid` (`sortid`),
  KEY `tid` (`tid`),
  KEY `fid` (`fid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_forum_typeoptionvar` WRITE;
/*!40000 ALTER TABLE `pre_forum_typeoptionvar` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_forum_typeoptionvar` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_forum_typevar`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_forum_typevar` (
  `sortid` smallint(6) NOT NULL DEFAULT 0,
  `optionid` smallint(6) NOT NULL DEFAULT 0,
  `available` tinyint(1) NOT NULL DEFAULT 0,
  `required` tinyint(1) NOT NULL DEFAULT 0,
  `unchangeable` tinyint(1) NOT NULL DEFAULT 0,
  `search` tinyint(1) NOT NULL DEFAULT 0,
  `displayorder` tinyint(3) NOT NULL DEFAULT 0,
  `subjectshow` tinyint(1) NOT NULL DEFAULT 0,
  UNIQUE KEY `optionid` (`sortid`,`optionid`),
  KEY `sortid` (`sortid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_forum_typevar` WRITE;
/*!40000 ALTER TABLE `pre_forum_typevar` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_forum_typevar` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_forum_warning`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_forum_warning` (
  `wid` int(10) unsigned NOT NULL AUTO_INCREMENT,
  `pid` int(10) unsigned NOT NULL,
  `operatorid` mediumint(8) unsigned NOT NULL,
  `operator` char(50) NOT NULL,
  `authorid` mediumint(8) unsigned NOT NULL,
  `author` char(50) NOT NULL,
  `dateline` int(10) unsigned NOT NULL,
  `reason` char(40) NOT NULL,
  PRIMARY KEY (`wid`),
  UNIQUE KEY `pid` (`pid`),
  KEY `authorid` (`authorid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_forum_warning` WRITE;
/*!40000 ALTER TABLE `pre_forum_warning` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_forum_warning` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_home_album`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_home_album` (
  `albumid` mediumint(8) unsigned NOT NULL AUTO_INCREMENT,
  `albumname` varchar(50) NOT NULL DEFAULT '',
  `catid` smallint(6) unsigned NOT NULL DEFAULT 0,
  `uid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `username` varchar(50) NOT NULL DEFAULT '',
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  `updatetime` int(10) unsigned NOT NULL DEFAULT 0,
  `picnum` smallint(6) unsigned NOT NULL DEFAULT 0,
  `pic` varchar(255) NOT NULL DEFAULT '',
  `picflag` tinyint(1) NOT NULL DEFAULT 0,
  `friend` tinyint(1) NOT NULL DEFAULT 0,
  `password` varchar(10) NOT NULL DEFAULT '',
  `target_ids` text NOT NULL,
  `favtimes` mediumint(8) unsigned NOT NULL,
  `sharetimes` mediumint(8) unsigned NOT NULL,
  `depict` text NOT NULL,
  PRIMARY KEY (`albumid`),
  KEY `uid` (`uid`,`updatetime`),
  KEY `updatetime` (`updatetime`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_home_album` WRITE;
/*!40000 ALTER TABLE `pre_home_album` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_home_album` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_home_album_category`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_home_album_category` (
  `catid` mediumint(8) unsigned NOT NULL AUTO_INCREMENT,
  `upid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `catname` varchar(255) NOT NULL DEFAULT '',
  `num` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `displayorder` smallint(6) NOT NULL DEFAULT 0,
  PRIMARY KEY (`catid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_home_album_category` WRITE;
/*!40000 ALTER TABLE `pre_home_album_category` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_home_album_category` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_home_blacklist`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_home_blacklist` (
  `uid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `buid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`uid`,`buid`),
  KEY `uid` (`uid`,`dateline`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_home_blacklist` WRITE;
/*!40000 ALTER TABLE `pre_home_blacklist` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_home_blacklist` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_home_blog`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_home_blog` (
  `blogid` mediumint(8) unsigned NOT NULL AUTO_INCREMENT,
  `uid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `username` char(50) NOT NULL DEFAULT '',
  `subject` varchar(255) NOT NULL DEFAULT '',
  `classid` smallint(6) unsigned NOT NULL DEFAULT 0,
  `catid` smallint(6) unsigned NOT NULL DEFAULT 0,
  `viewnum` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `replynum` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `hot` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  `picflag` tinyint(1) NOT NULL DEFAULT 0,
  `noreply` tinyint(1) NOT NULL DEFAULT 0,
  `friend` tinyint(1) NOT NULL DEFAULT 0,
  `password` char(10) NOT NULL DEFAULT '',
  `favtimes` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `sharetimes` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `status` tinyint(1) NOT NULL DEFAULT 0,
  `click1` smallint(6) unsigned NOT NULL DEFAULT 0,
  `click2` smallint(6) unsigned NOT NULL DEFAULT 0,
  `click3` smallint(6) unsigned NOT NULL DEFAULT 0,
  `click4` smallint(6) unsigned NOT NULL DEFAULT 0,
  `click5` smallint(6) unsigned NOT NULL DEFAULT 0,
  `click6` smallint(6) unsigned NOT NULL DEFAULT 0,
  `click7` smallint(6) unsigned NOT NULL DEFAULT 0,
  `click8` smallint(6) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`blogid`),
  KEY `uid` (`uid`,`dateline`),
  KEY `hot` (`hot`),
  KEY `dateline` (`dateline`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_home_blog` WRITE;
/*!40000 ALTER TABLE `pre_home_blog` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_home_blog` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_home_blog_category`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_home_blog_category` (
  `catid` mediumint(8) unsigned NOT NULL AUTO_INCREMENT,
  `upid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `catname` varchar(255) NOT NULL DEFAULT '',
  `num` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `displayorder` smallint(6) NOT NULL DEFAULT 0,
  PRIMARY KEY (`catid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_home_blog_category` WRITE;
/*!40000 ALTER TABLE `pre_home_blog_category` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_home_blog_category` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_home_blog_moderate`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_home_blog_moderate` (
  `id` int(10) unsigned NOT NULL DEFAULT 0,
  `status` tinyint(3) NOT NULL DEFAULT 0,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`id`),
  KEY `status` (`status`,`dateline`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_home_blog_moderate` WRITE;
/*!40000 ALTER TABLE `pre_home_blog_moderate` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_home_blog_moderate` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_home_blogfield`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_home_blogfield` (
  `blogid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `uid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `pic` varchar(255) NOT NULL DEFAULT '',
  `tag` varchar(255) NOT NULL DEFAULT '',
  `message` mediumtext NOT NULL,
  `postip` varchar(255) NOT NULL DEFAULT '',
  `port` smallint(6) unsigned NOT NULL DEFAULT 0,
  `related` text NOT NULL,
  `relatedtime` int(10) unsigned NOT NULL DEFAULT 0,
  `target_ids` text NOT NULL,
  `hotuser` text NOT NULL,
  `magiccolor` tinyint(6) NOT NULL DEFAULT 0,
  `magicpaper` tinyint(6) NOT NULL DEFAULT 0,
  `pushedaid` mediumint(8) NOT NULL DEFAULT 0,
  PRIMARY KEY (`blogid`),
  KEY `uid` (`uid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_home_blogfield` WRITE;
/*!40000 ALTER TABLE `pre_home_blogfield` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_home_blogfield` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_home_class`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_home_class` (
  `classid` mediumint(8) unsigned NOT NULL AUTO_INCREMENT,
  `classname` char(40) NOT NULL DEFAULT '',
  `uid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`classid`),
  KEY `uid` (`uid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_home_class` WRITE;
/*!40000 ALTER TABLE `pre_home_class` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_home_class` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_home_click`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_home_click` (
  `clickid` smallint(6) unsigned NOT NULL AUTO_INCREMENT,
  `name` char(50) NOT NULL DEFAULT '',
  `icon` char(100) NOT NULL DEFAULT '',
  `idtype` char(15) NOT NULL DEFAULT '',
  `available` tinyint(1) NOT NULL DEFAULT 0,
  `displayorder` tinyint(6) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`clickid`),
  KEY `idtype` (`idtype`,`displayorder`)
) ENGINE=InnoDB AUTO_INCREMENT=16 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_home_click` WRITE;
/*!40000 ALTER TABLE `pre_home_click` DISABLE KEYS */;
INSERT INTO `pre_home_click` VALUES
(1,'璺繃','luguo.gif','blogid',1,0),
(2,'闆蜂汉','leiren.gif','blogid',1,0),
(3,'鎻℃墜','woshou.gif','blogid',1,0),
(4,'椴滆姳','xianhua.gif','blogid',1,0),
(5,'楦¤泲','jidan.gif','blogid',1,0),
(6,'婕備寒','piaoliang.gif','picid',1,0),
(7,'閰锋瘷','kubi.gif','picid',1,0),
(8,'闆蜂汉','leiren.gif','picid',1,0),
(9,'椴滆姳','xianhua.gif','picid',1,0),
(10,'楦¤泲','jidan.gif','picid',1,0),
(11,'璺繃','luguo.gif','aid',1,0),
(12,'闆蜂汉','leiren.gif','aid',1,0),
(13,'鎻℃墜','woshou.gif','aid',1,0),
(14,'椴滆姳','xianhua.gif','aid',1,0),
(15,'楦¤泲','jidan.gif','aid',1,0);
/*!40000 ALTER TABLE `pre_home_click` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_home_clickuser`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_home_clickuser` (
  `uid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `username` varchar(50) NOT NULL DEFAULT '',
  `id` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `idtype` varchar(15) NOT NULL DEFAULT '',
  `clickid` smallint(6) unsigned NOT NULL DEFAULT 0,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  KEY `id` (`id`,`idtype`,`dateline`),
  KEY `uid` (`uid`,`idtype`,`dateline`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_home_clickuser` WRITE;
/*!40000 ALTER TABLE `pre_home_clickuser` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_home_clickuser` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_home_comment`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_home_comment` (
  `cid` mediumint(8) unsigned NOT NULL AUTO_INCREMENT,
  `uid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `id` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `idtype` varchar(20) NOT NULL DEFAULT '',
  `authorid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `author` varchar(50) NOT NULL DEFAULT '',
  `ip` varchar(45) NOT NULL DEFAULT '',
  `port` smallint(6) unsigned NOT NULL DEFAULT 0,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  `message` text NOT NULL,
  `magicflicker` tinyint(1) NOT NULL DEFAULT 0,
  `status` tinyint(1) NOT NULL DEFAULT 0,
  PRIMARY KEY (`cid`),
  KEY `authorid` (`authorid`,`idtype`),
  KEY `id` (`id`,`idtype`,`dateline`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_home_comment` WRITE;
/*!40000 ALTER TABLE `pre_home_comment` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_home_comment` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_home_comment_moderate`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_home_comment_moderate` (
  `id` int(10) unsigned NOT NULL DEFAULT 0,
  `idtype` varchar(15) NOT NULL DEFAULT '',
  `status` tinyint(3) NOT NULL DEFAULT 0,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`id`),
  KEY `idtype` (`idtype`,`status`,`dateline`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_home_comment_moderate` WRITE;
/*!40000 ALTER TABLE `pre_home_comment_moderate` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_home_comment_moderate` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_home_docomment`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_home_docomment` (
  `id` int(10) unsigned NOT NULL AUTO_INCREMENT,
  `upid` int(10) unsigned NOT NULL DEFAULT 0,
  `doid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `uid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `username` varchar(50) NOT NULL DEFAULT '',
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  `message` text NOT NULL,
  `ip` varchar(45) NOT NULL DEFAULT '',
  `port` smallint(5) unsigned NOT NULL DEFAULT 0,
  `grade` smallint(5) unsigned NOT NULL DEFAULT 0,
  `replynum` int(10) unsigned NOT NULL,
  `recomends` int(10) unsigned NOT NULL,
  `status` tinyint(4) NOT NULL,
  `fields` longtext CHARACTER SET utf8mb4 COLLATE utf8mb4_bin NOT NULL CHECK (json_valid(`fields`)),
  PRIMARY KEY (`id`),
  KEY `doid` (`doid`,`dateline`),
  KEY `dateline` (`dateline`),
  KEY `status` (`status`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_home_docomment` WRITE;
/*!40000 ALTER TABLE `pre_home_docomment` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_home_docomment` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_home_docomment_recomend_log`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_home_docomment_recomend_log` (
  `id` int(10) unsigned NOT NULL AUTO_INCREMENT,
  `doid` int(10) unsigned NOT NULL DEFAULT 0,
  `docid` int(10) unsigned NOT NULL DEFAULT 0,
  `uid` int(10) unsigned NOT NULL DEFAULT 0,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`id`),
  UNIQUE KEY `docid_uid` (`docid`,`uid`),
  KEY `doid` (`doid`),
  KEY `docid` (`docid`),
  KEY `uid` (`uid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_home_docomment_recomend_log` WRITE;
/*!40000 ALTER TABLE `pre_home_docomment_recomend_log` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_home_docomment_recomend_log` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_home_doing`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_home_doing` (
  `doid` mediumint(8) unsigned NOT NULL AUTO_INCREMENT,
  `itemid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `type` varchar(30) NOT NULL DEFAULT '',
  `uid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `username` varchar(50) NOT NULL DEFAULT '',
  `from` varchar(20) NOT NULL DEFAULT '',
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  `body_template` text NOT NULL,
  `body_data` text NOT NULL,
  `message` text NOT NULL,
  `ip` varchar(45) NOT NULL DEFAULT '',
  `port` smallint(6) unsigned NOT NULL DEFAULT 0,
  `replynum` int(10) unsigned NOT NULL DEFAULT 0,
  `recomends` int(10) unsigned NOT NULL DEFAULT 0,
  `sharetimes` int(10) unsigned NOT NULL DEFAULT 0,
  `favtimes` int(10) unsigned NOT NULL DEFAULT 0,
  `status` tinyint(3) unsigned NOT NULL DEFAULT 0,
  `fields` longtext CHARACTER SET utf8mb4 COLLATE utf8mb4_bin DEFAULT NULL CHECK (json_valid(`fields`)),
  PRIMARY KEY (`doid`),
  KEY `uid` (`uid`,`dateline`),
  KEY `dateline` (`dateline`),
  KEY `type` (`type`),
  KEY `itemid` (`itemid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_home_doing` WRITE;
/*!40000 ALTER TABLE `pre_home_doing` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_home_doing` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_home_doing_attachment`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_home_doing_attachment` (
  `aid` int(10) unsigned NOT NULL AUTO_INCREMENT,
  `doid` int(10) unsigned NOT NULL DEFAULT 0,
  `uid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  `filename` varchar(255) NOT NULL DEFAULT '',
  `filesize` int(10) unsigned NOT NULL DEFAULT 0,
  `attachment` varchar(255) NOT NULL DEFAULT '',
  `remote` tinyint(1) NOT NULL DEFAULT 0,
  `isimage` tinyint(1) NOT NULL DEFAULT 0,
  `width` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `height` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `displayorder` int(11) NOT NULL,
  PRIMARY KEY (`aid`),
  KEY `uid` (`uid`),
  KEY `doid` (`doid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_home_doing_attachment` WRITE;
/*!40000 ALTER TABLE `pre_home_doing_attachment` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_home_doing_attachment` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_home_doing_moderate`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_home_doing_moderate` (
  `id` int(10) unsigned NOT NULL DEFAULT 0,
  `status` tinyint(3) NOT NULL DEFAULT 0,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`id`),
  KEY `status` (`status`,`dateline`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_home_doing_moderate` WRITE;
/*!40000 ALTER TABLE `pre_home_doing_moderate` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_home_doing_moderate` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_home_doing_recomend_log`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_home_doing_recomend_log` (
  `id` int(10) unsigned NOT NULL AUTO_INCREMENT,
  `doid` int(10) unsigned NOT NULL DEFAULT 0,
  `uid` int(10) unsigned NOT NULL DEFAULT 0,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`id`),
  UNIQUE KEY `doid_uid` (`doid`,`uid`),
  KEY `doid` (`doid`),
  KEY `uid` (`uid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_home_doing_recomend_log` WRITE;
/*!40000 ALTER TABLE `pre_home_doing_recomend_log` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_home_doing_recomend_log` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_home_favorite`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_home_favorite` (
  `favid` mediumint(8) unsigned NOT NULL AUTO_INCREMENT,
  `uid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `id` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `idtype` varchar(255) NOT NULL DEFAULT '',
  `spaceuid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `title` varchar(255) NOT NULL DEFAULT '',
  `description` text NOT NULL,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`favid`),
  KEY `idtype` (`id`,`idtype`(40)),
  KEY `uid` (`uid`,`idtype`(40),`dateline`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_home_favorite` WRITE;
/*!40000 ALTER TABLE `pre_home_favorite` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_home_favorite` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_home_feed`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_home_feed` (
  `feedid` int(10) unsigned NOT NULL AUTO_INCREMENT,
  `icon` varchar(30) NOT NULL DEFAULT '',
  `uid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `username` varchar(50) NOT NULL DEFAULT '',
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  `friend` tinyint(1) NOT NULL DEFAULT 0,
  `hash_template` varchar(32) NOT NULL DEFAULT '',
  `hash_data` varchar(32) NOT NULL DEFAULT '',
  `title_template` text NOT NULL,
  `title_data` text NOT NULL,
  `body_template` text NOT NULL,
  `body_data` text NOT NULL,
  `body_general` text NOT NULL,
  `image_1` varchar(255) NOT NULL DEFAULT '',
  `image_1_link` varchar(255) NOT NULL DEFAULT '',
  `image_2` varchar(255) NOT NULL DEFAULT '',
  `image_2_link` varchar(255) NOT NULL DEFAULT '',
  `image_3` varchar(255) NOT NULL DEFAULT '',
  `image_3_link` varchar(255) NOT NULL DEFAULT '',
  `image_4` varchar(255) NOT NULL DEFAULT '',
  `image_4_link` varchar(255) NOT NULL DEFAULT '',
  `target_ids` text NOT NULL,
  `id` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `idtype` varchar(15) NOT NULL DEFAULT '',
  `hot` mediumint(8) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`feedid`),
  KEY `uid` (`uid`,`dateline`),
  KEY `dateline` (`dateline`),
  KEY `hot` (`hot`),
  KEY `id` (`id`,`idtype`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_home_feed` WRITE;
/*!40000 ALTER TABLE `pre_home_feed` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_home_feed` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_home_follow`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_home_follow` (
  `uid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `username` char(50) NOT NULL DEFAULT '',
  `followuid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `fusername` char(50) NOT NULL DEFAULT '',
  `bkname` varchar(255) NOT NULL DEFAULT '',
  `status` tinyint(1) NOT NULL DEFAULT 0,
  `mutual` tinyint(1) NOT NULL DEFAULT 0,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`uid`,`followuid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_home_follow` WRITE;
/*!40000 ALTER TABLE `pre_home_follow` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_home_follow` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_home_follow_feed`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_home_follow_feed` (
  `feedid` mediumint(8) unsigned NOT NULL AUTO_INCREMENT,
  `uid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `username` varchar(50) NOT NULL DEFAULT '',
  `tid` int(10) unsigned NOT NULL DEFAULT 0,
  `note` text NOT NULL,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`feedid`),
  KEY `uid` (`uid`,`dateline`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_home_follow_feed` WRITE;
/*!40000 ALTER TABLE `pre_home_follow_feed` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_home_follow_feed` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_home_follow_feed_archiver`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_home_follow_feed_archiver` (
  `feedid` mediumint(8) unsigned NOT NULL AUTO_INCREMENT,
  `uid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `username` varchar(50) NOT NULL DEFAULT '',
  `tid` int(10) unsigned NOT NULL DEFAULT 0,
  `note` text NOT NULL,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`feedid`),
  KEY `uid` (`uid`,`dateline`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_home_follow_feed_archiver` WRITE;
/*!40000 ALTER TABLE `pre_home_follow_feed_archiver` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_home_follow_feed_archiver` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_home_friend`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_home_friend` (
  `uid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `fuid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `fusername` varchar(50) NOT NULL DEFAULT '',
  `gid` smallint(6) unsigned NOT NULL DEFAULT 0,
  `num` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  `note` varchar(255) NOT NULL DEFAULT '',
  PRIMARY KEY (`uid`,`fuid`),
  KEY `fuid` (`fuid`),
  KEY `uid` (`uid`,`num`,`dateline`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_home_friend` WRITE;
/*!40000 ALTER TABLE `pre_home_friend` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_home_friend` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_home_friend_request`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_home_friend_request` (
  `uid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `fuid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `fusername` char(50) NOT NULL DEFAULT '',
  `gid` smallint(6) unsigned NOT NULL DEFAULT 0,
  `note` char(60) NOT NULL DEFAULT '',
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`uid`,`fuid`),
  KEY `fuid` (`fuid`),
  KEY `dateline` (`uid`,`dateline`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_home_friend_request` WRITE;
/*!40000 ALTER TABLE `pre_home_friend_request` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_home_friend_request` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_home_friendlog`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_home_friendlog` (
  `uid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `fuid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `action` varchar(10) NOT NULL DEFAULT '',
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`uid`,`fuid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_home_friendlog` WRITE;
/*!40000 ALTER TABLE `pre_home_friendlog` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_home_friendlog` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_home_notification`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_home_notification` (
  `id` bigint(20) unsigned NOT NULL AUTO_INCREMENT,
  `uid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `type` varchar(20) NOT NULL DEFAULT '',
  `new` tinyint(1) NOT NULL DEFAULT 0,
  `authorid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `author` varchar(50) NOT NULL DEFAULT '',
  `note` text NOT NULL,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  `from_id` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `from_idtype` varchar(20) NOT NULL DEFAULT '',
  `from_num` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `category` tinyint(1) NOT NULL DEFAULT 0,
  PRIMARY KEY (`id`),
  KEY `uid` (`uid`,`new`),
  KEY `category` (`uid`,`category`,`dateline`),
  KEY `by_type` (`uid`,`type`,`dateline`),
  KEY `from_id` (`from_id`,`from_idtype`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_home_notification` WRITE;
/*!40000 ALTER TABLE `pre_home_notification` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_home_notification` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_home_pic`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_home_pic` (
  `picid` mediumint(8) NOT NULL AUTO_INCREMENT,
  `albumid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `uid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `username` varchar(50) NOT NULL DEFAULT '',
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  `postip` varchar(255) NOT NULL DEFAULT '',
  `port` smallint(6) unsigned NOT NULL DEFAULT 0,
  `filename` varchar(255) NOT NULL DEFAULT '',
  `title` varchar(255) NOT NULL DEFAULT '',
  `type` varchar(255) NOT NULL DEFAULT '',
  `size` int(10) unsigned NOT NULL DEFAULT 0,
  `filepath` varchar(255) NOT NULL DEFAULT '',
  `thumb` tinyint(1) NOT NULL DEFAULT 0,
  `remote` tinyint(1) NOT NULL DEFAULT 0,
  `hot` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `sharetimes` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `click1` smallint(6) unsigned NOT NULL DEFAULT 0,
  `click2` smallint(6) unsigned NOT NULL DEFAULT 0,
  `click3` smallint(6) unsigned NOT NULL DEFAULT 0,
  `click4` smallint(6) unsigned NOT NULL DEFAULT 0,
  `click5` smallint(6) unsigned NOT NULL DEFAULT 0,
  `click6` smallint(6) unsigned NOT NULL DEFAULT 0,
  `click7` smallint(6) unsigned NOT NULL DEFAULT 0,
  `click8` smallint(6) unsigned NOT NULL DEFAULT 0,
  `magicframe` tinyint(6) NOT NULL DEFAULT 0,
  `status` tinyint(3) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`picid`),
  KEY `uid` (`uid`),
  KEY `albumid` (`albumid`,`dateline`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_home_pic` WRITE;
/*!40000 ALTER TABLE `pre_home_pic` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_home_pic` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_home_pic_moderate`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_home_pic_moderate` (
  `id` int(10) unsigned NOT NULL DEFAULT 0,
  `status` tinyint(3) NOT NULL DEFAULT 0,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`id`),
  KEY `status` (`status`,`dateline`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_home_pic_moderate` WRITE;
/*!40000 ALTER TABLE `pre_home_pic_moderate` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_home_pic_moderate` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_home_picfield`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_home_picfield` (
  `picid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `hotuser` text NOT NULL,
  PRIMARY KEY (`picid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_home_picfield` WRITE;
/*!40000 ALTER TABLE `pre_home_picfield` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_home_picfield` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_home_poke`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_home_poke` (
  `uid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `fromuid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `fromusername` varchar(50) NOT NULL DEFAULT '',
  `note` varchar(255) NOT NULL DEFAULT '',
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  `iconid` smallint(6) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`uid`,`fromuid`),
  KEY `uid` (`uid`,`dateline`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_home_poke` WRITE;
/*!40000 ALTER TABLE `pre_home_poke` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_home_poke` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_home_pokearchive`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_home_pokearchive` (
  `pid` mediumint(8) NOT NULL AUTO_INCREMENT,
  `pokeuid` int(10) unsigned NOT NULL DEFAULT 0,
  `uid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `fromuid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `note` varchar(255) NOT NULL DEFAULT '',
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  `iconid` smallint(6) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`pid`),
  KEY `pokeuid` (`pokeuid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_home_pokearchive` WRITE;
/*!40000 ALTER TABLE `pre_home_pokearchive` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_home_pokearchive` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_home_share`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_home_share` (
  `sid` mediumint(8) unsigned NOT NULL AUTO_INCREMENT,
  `itemid` mediumint(8) unsigned NOT NULL,
  `type` varchar(30) NOT NULL DEFAULT '',
  `uid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `username` varchar(50) NOT NULL DEFAULT '',
  `fromuid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  `title_template` text NOT NULL,
  `body_template` text NOT NULL,
  `body_data` text NOT NULL,
  `body_general` text NOT NULL,
  `image` varchar(255) NOT NULL DEFAULT '',
  `image_link` varchar(255) NOT NULL DEFAULT '',
  `hot` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `hotuser` text NOT NULL,
  `status` tinyint(1) NOT NULL,
  PRIMARY KEY (`sid`),
  KEY `uid` (`uid`,`dateline`),
  KEY `hot` (`hot`),
  KEY `dateline` (`dateline`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_home_share` WRITE;
/*!40000 ALTER TABLE `pre_home_share` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_home_share` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_home_share_moderate`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_home_share_moderate` (
  `id` int(10) unsigned NOT NULL DEFAULT 0,
  `status` tinyint(3) NOT NULL DEFAULT 0,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`id`),
  KEY `status` (`status`,`dateline`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_home_share_moderate` WRITE;
/*!40000 ALTER TABLE `pre_home_share_moderate` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_home_share_moderate` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_home_show`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_home_show` (
  `uid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `username` varchar(50) NOT NULL DEFAULT '',
  `unitprice` int(10) unsigned NOT NULL DEFAULT 1,
  `credit` int(10) unsigned NOT NULL DEFAULT 0,
  `note` varchar(100) NOT NULL DEFAULT '',
  PRIMARY KEY (`uid`),
  KEY `unitprice` (`unitprice`),
  KEY `credit` (`credit`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_home_show` WRITE;
/*!40000 ALTER TABLE `pre_home_show` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_home_show` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_home_specialuser`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_home_specialuser` (
  `uid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `username` varchar(50) NOT NULL DEFAULT '',
  `status` tinyint(1) NOT NULL DEFAULT 0,
  `dateline` int(10) NOT NULL DEFAULT 0,
  `reason` text NOT NULL,
  `opuid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `opusername` varchar(50) NOT NULL DEFAULT '',
  `displayorder` mediumint(8) unsigned NOT NULL DEFAULT 0,
  KEY `uid` (`uid`,`status`),
  KEY `displayorder` (`status`,`displayorder`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_home_specialuser` WRITE;
/*!40000 ALTER TABLE `pre_home_specialuser` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_home_specialuser` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_home_visitor`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_home_visitor` (
  `uid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `vuid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `vusername` char(50) NOT NULL DEFAULT '',
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`uid`,`vuid`),
  KEY `vuid` (`vuid`),
  KEY `dateline` (`dateline`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_home_visitor` WRITE;
/*!40000 ALTER TABLE `pre_home_visitor` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_home_visitor` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_mobile_setting`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_mobile_setting` (
  `skey` varchar(190) NOT NULL DEFAULT '',
  `svalue` text NOT NULL,
  PRIMARY KEY (`skey`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_mobile_setting` WRITE;
/*!40000 ALTER TABLE `pre_mobile_setting` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_mobile_setting` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_portal_article_content`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_portal_article_content` (
  `cid` int(10) unsigned NOT NULL AUTO_INCREMENT,
  `aid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `id` int(10) unsigned NOT NULL DEFAULT 0,
  `idtype` varchar(255) NOT NULL DEFAULT '',
  `title` varchar(255) NOT NULL DEFAULT '',
  `content` mediumtext NOT NULL,
  `pageorder` smallint(6) unsigned NOT NULL DEFAULT 0,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`cid`),
  KEY `aid` (`aid`,`pageorder`),
  KEY `pageorder` (`pageorder`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_portal_article_content` WRITE;
/*!40000 ALTER TABLE `pre_portal_article_content` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_portal_article_content` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_portal_article_count`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_portal_article_count` (
  `aid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `catid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `viewnum` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `commentnum` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `favtimes` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `sharetimes` mediumint(8) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`aid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_portal_article_count` WRITE;
/*!40000 ALTER TABLE `pre_portal_article_count` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_portal_article_count` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_portal_article_moderate`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_portal_article_moderate` (
  `id` int(10) unsigned NOT NULL DEFAULT 0,
  `status` tinyint(3) NOT NULL DEFAULT 0,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`id`),
  KEY `status` (`status`,`dateline`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_portal_article_moderate` WRITE;
/*!40000 ALTER TABLE `pre_portal_article_moderate` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_portal_article_moderate` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_portal_article_related`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_portal_article_related` (
  `aid` mediumint(8) unsigned NOT NULL AUTO_INCREMENT,
  `raid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `displayorder` mediumint(8) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`aid`,`raid`),
  KEY `aid` (`aid`,`displayorder`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_portal_article_related` WRITE;
/*!40000 ALTER TABLE `pre_portal_article_related` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_portal_article_related` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_portal_article_title`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_portal_article_title` (
  `aid` mediumint(8) unsigned NOT NULL AUTO_INCREMENT,
  `catid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `bid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `uid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `username` varchar(255) NOT NULL DEFAULT '',
  `title` varchar(255) NOT NULL DEFAULT '',
  `highlight` varchar(255) NOT NULL DEFAULT '',
  `author` varchar(255) NOT NULL DEFAULT '',
  `from` varchar(255) NOT NULL DEFAULT '',
  `fromurl` varchar(255) NOT NULL DEFAULT '',
  `url` varchar(255) NOT NULL DEFAULT '',
  `summary` varchar(255) NOT NULL DEFAULT '',
  `pic` varchar(255) NOT NULL DEFAULT '',
  `thumb` tinyint(1) NOT NULL DEFAULT 0,
  `remote` tinyint(1) NOT NULL DEFAULT 0,
  `id` int(10) unsigned NOT NULL DEFAULT 0,
  `idtype` varchar(255) NOT NULL DEFAULT '',
  `contents` smallint(6) NOT NULL DEFAULT 0,
  `allowcomment` tinyint(1) NOT NULL DEFAULT 0,
  `owncomment` tinyint(1) NOT NULL DEFAULT 0,
  `click1` smallint(6) unsigned NOT NULL DEFAULT 0,
  `click2` smallint(6) unsigned NOT NULL DEFAULT 0,
  `click3` smallint(6) unsigned NOT NULL DEFAULT 0,
  `click4` smallint(6) unsigned NOT NULL DEFAULT 0,
  `click5` smallint(6) unsigned NOT NULL DEFAULT 0,
  `click6` smallint(6) unsigned NOT NULL DEFAULT 0,
  `click7` smallint(6) unsigned NOT NULL DEFAULT 0,
  `click8` smallint(6) unsigned NOT NULL DEFAULT 0,
  `tags` varchar(255) NOT NULL DEFAULT '',
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  `status` tinyint(1) NOT NULL DEFAULT 0,
  `showinnernav` tinyint(1) NOT NULL DEFAULT 0,
  `preaid` mediumint(8) unsigned NOT NULL,
  `nextaid` mediumint(8) unsigned NOT NULL,
  `htmlmade` tinyint(1) NOT NULL DEFAULT 0,
  `htmlname` varchar(255) NOT NULL DEFAULT '',
  `htmldir` varchar(255) NOT NULL DEFAULT '',
  PRIMARY KEY (`aid`),
  KEY `catid` (`catid`,`dateline`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_portal_article_title` WRITE;
/*!40000 ALTER TABLE `pre_portal_article_title` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_portal_article_title` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_portal_article_trash`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_portal_article_trash` (
  `aid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `content` text NOT NULL,
  PRIMARY KEY (`aid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_portal_article_trash` WRITE;
/*!40000 ALTER TABLE `pre_portal_article_trash` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_portal_article_trash` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_portal_attachment`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_portal_attachment` (
  `attachid` mediumint(8) unsigned NOT NULL AUTO_INCREMENT,
  `uid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  `filename` varchar(255) NOT NULL DEFAULT '',
  `filetype` varchar(255) NOT NULL DEFAULT '',
  `filesize` int(10) unsigned NOT NULL DEFAULT 0,
  `attachment` varchar(255) NOT NULL DEFAULT '',
  `isimage` tinyint(1) NOT NULL DEFAULT 0,
  `thumb` tinyint(1) NOT NULL DEFAULT 0,
  `remote` tinyint(1) NOT NULL DEFAULT 0,
  `aid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`attachid`),
  KEY `aid` (`aid`,`attachid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_portal_attachment` WRITE;
/*!40000 ALTER TABLE `pre_portal_attachment` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_portal_attachment` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_portal_category`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_portal_category` (
  `catid` mediumint(8) unsigned NOT NULL AUTO_INCREMENT,
  `upid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `catname` varchar(255) NOT NULL DEFAULT '',
  `articles` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `allowcomment` tinyint(1) NOT NULL DEFAULT 1,
  `displayorder` smallint(6) NOT NULL DEFAULT 0,
  `notinheritedarticle` tinyint(1) NOT NULL DEFAULT 0,
  `notinheritedblock` tinyint(1) NOT NULL DEFAULT 0,
  `domain` varchar(255) NOT NULL DEFAULT '',
  `url` varchar(255) NOT NULL DEFAULT '',
  `uid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `username` varchar(255) NOT NULL DEFAULT '',
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  `closed` tinyint(1) NOT NULL DEFAULT 0,
  `shownav` tinyint(1) NOT NULL DEFAULT 0,
  `description` text NOT NULL,
  `seotitle` text NOT NULL,
  `keyword` text NOT NULL,
  `primaltplname` varchar(255) NOT NULL DEFAULT '',
  `articleprimaltplname` varchar(255) NOT NULL DEFAULT '',
  `disallowpublish` tinyint(1) NOT NULL DEFAULT 0,
  `foldername` varchar(255) NOT NULL DEFAULT '',
  `notshowarticlesummay` varchar(255) NOT NULL DEFAULT '',
  `perpage` smallint(6) NOT NULL DEFAULT 0,
  `maxpages` smallint(6) NOT NULL DEFAULT 0,
  `noantitheft` tinyint(1) NOT NULL DEFAULT 0,
  `lastpublish` int(10) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`catid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_portal_category` WRITE;
/*!40000 ALTER TABLE `pre_portal_category` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_portal_category` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_portal_category_permission`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_portal_category_permission` (
  `catid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `uid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `allowpublish` tinyint(1) NOT NULL DEFAULT 0,
  `allowmanage` tinyint(1) NOT NULL DEFAULT 0,
  `inheritedcatid` mediumint(8) NOT NULL DEFAULT 0,
  PRIMARY KEY (`catid`,`uid`),
  KEY `uid` (`uid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_portal_category_permission` WRITE;
/*!40000 ALTER TABLE `pre_portal_category_permission` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_portal_category_permission` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_portal_comment`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_portal_comment` (
  `cid` mediumint(8) unsigned NOT NULL AUTO_INCREMENT,
  `uid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `username` varchar(255) NOT NULL DEFAULT '',
  `id` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `idtype` varchar(20) NOT NULL DEFAULT '',
  `postip` varchar(255) NOT NULL DEFAULT '',
  `port` smallint(6) unsigned NOT NULL DEFAULT 0,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  `status` tinyint(1) NOT NULL DEFAULT 0,
  `message` text NOT NULL,
  PRIMARY KEY (`cid`),
  KEY `idtype` (`id`,`idtype`,`dateline`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_portal_comment` WRITE;
/*!40000 ALTER TABLE `pre_portal_comment` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_portal_comment` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_portal_comment_moderate`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_portal_comment_moderate` (
  `id` int(10) unsigned NOT NULL DEFAULT 0,
  `idtype` varchar(15) NOT NULL DEFAULT '',
  `status` tinyint(3) NOT NULL DEFAULT 0,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`id`),
  KEY `idtype` (`idtype`,`status`,`dateline`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_portal_comment_moderate` WRITE;
/*!40000 ALTER TABLE `pre_portal_comment_moderate` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_portal_comment_moderate` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_portal_rsscache`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_portal_rsscache` (
  `lastupdate` int(10) unsigned NOT NULL DEFAULT 0,
  `catid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `aid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  `catname` char(50) NOT NULL DEFAULT '',
  `author` char(50) NOT NULL DEFAULT '',
  `subject` varchar(255) NOT NULL DEFAULT '',
  `description` char(255) NOT NULL DEFAULT '',
  UNIQUE KEY `aid` (`aid`),
  KEY `catid` (`catid`,`dateline`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_portal_rsscache` WRITE;
/*!40000 ALTER TABLE `pre_portal_rsscache` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_portal_rsscache` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_portal_topic`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_portal_topic` (
  `topicid` mediumint(8) unsigned NOT NULL AUTO_INCREMENT,
  `title` varchar(255) NOT NULL DEFAULT '',
  `name` varchar(255) NOT NULL DEFAULT '',
  `domain` varchar(255) NOT NULL DEFAULT '',
  `summary` text NOT NULL,
  `keyword` text NOT NULL,
  `cover` varchar(255) NOT NULL DEFAULT '',
  `picflag` tinyint(1) NOT NULL DEFAULT 0,
  `primaltplname` varchar(255) NOT NULL DEFAULT '',
  `useheader` tinyint(1) NOT NULL DEFAULT 0,
  `usefooter` tinyint(1) NOT NULL DEFAULT 0,
  `uid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `username` varchar(255) NOT NULL DEFAULT '',
  `viewnum` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  `closed` tinyint(1) NOT NULL DEFAULT 0,
  `allowcomment` tinyint(1) NOT NULL DEFAULT 0,
  `commentnum` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `htmlmade` tinyint(1) NOT NULL DEFAULT 0,
  `htmldir` varchar(255) NOT NULL DEFAULT '',
  PRIMARY KEY (`topicid`),
  KEY `name` (`name`(40))
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_portal_topic` WRITE;
/*!40000 ALTER TABLE `pre_portal_topic` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_portal_topic` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_portal_topic_pic`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_portal_topic_pic` (
  `picid` mediumint(8) NOT NULL AUTO_INCREMENT,
  `topicid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `uid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `username` varchar(50) NOT NULL DEFAULT '',
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  `filename` varchar(255) NOT NULL DEFAULT '',
  `title` varchar(255) NOT NULL DEFAULT '',
  `size` int(10) unsigned NOT NULL DEFAULT 0,
  `filepath` varchar(255) NOT NULL DEFAULT '',
  `thumb` tinyint(1) NOT NULL DEFAULT 0,
  `remote` tinyint(1) NOT NULL DEFAULT 0,
  PRIMARY KEY (`picid`),
  KEY `topicid` (`topicid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_portal_topic_pic` WRITE;
/*!40000 ALTER TABLE `pre_portal_topic_pic` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_portal_topic_pic` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_restful_api`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_restful_api` (
  `baseuri` varchar(255) NOT NULL,
  `ver` smallint(6) unsigned NOT NULL,
  `name` varchar(255) NOT NULL,
  `copyright` varchar(255) NOT NULL,
  `data` text NOT NULL,
  `status` tinyint(1) NOT NULL DEFAULT 1,
  `dateline` int(10) unsigned NOT NULL,
  PRIMARY KEY (`baseuri`,`ver`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_restful_api` WRITE;
/*!40000 ALTER TABLE `pre_restful_api` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_restful_api` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_restful_app`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_restful_app` (
  `appid` int(10) unsigned NOT NULL,
  `secret` varchar(255) NOT NULL,
  `name` varchar(255) NOT NULL,
  `data` text NOT NULL,
  `status` tinyint(1) NOT NULL DEFAULT 1,
  `dateline` int(10) unsigned NOT NULL,
  PRIMARY KEY (`appid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_restful_app` WRITE;
/*!40000 ALTER TABLE `pre_restful_app` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_restful_app` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_restful_permission`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_restful_permission` (
  `appid` int(10) unsigned NOT NULL,
  `uri` varchar(255) NOT NULL,
  `ver` smallint(6) unsigned NOT NULL,
  `isbase` tinyint(1) NOT NULL,
  `freq` int(10) unsigned NOT NULL,
  `dateline` int(10) unsigned NOT NULL,
  PRIMARY KEY (`appid`,`uri`,`ver`),
  KEY `isbase` (`isbase`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_restful_permission` WRITE;
/*!40000 ALTER TABLE `pre_restful_permission` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_restful_permission` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_restful_source`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_restful_source` (
  `sourceid` int(10) unsigned NOT NULL AUTO_INCREMENT,
  `name` varchar(255) NOT NULL,
  `url` varchar(255) NOT NULL,
  PRIMARY KEY (`sourceid`)
) ENGINE=InnoDB AUTO_INCREMENT=2 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_restful_source` WRITE;
/*!40000 ALTER TABLE `pre_restful_source` DISABLE KEYS */;
INSERT INTO `pre_restful_source` VALUES
(1,'Discuz! Team','https://api.witframe.com/discuzrestful');
/*!40000 ALTER TABLE `pre_restful_source` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_restful_stat`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_restful_stat` (
  `appid` int(10) unsigned NOT NULL,
  `uri` varchar(255) NOT NULL,
  `daytime` int(10) unsigned NOT NULL DEFAULT 0,
  `request` int(10) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`appid`,`uri`,`daytime`),
  KEY `daytime` (`daytime`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_restful_stat` WRITE;
/*!40000 ALTER TABLE `pre_restful_stat` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_restful_stat` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_security_evilpost`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_security_evilpost` (
  `pid` int(10) unsigned NOT NULL,
  `tid` int(10) unsigned NOT NULL DEFAULT 0,
  `type` tinyint(1) NOT NULL DEFAULT 0,
  `evilcount` int(10) NOT NULL DEFAULT 0,
  `eviltype` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `createtime` int(10) unsigned NOT NULL DEFAULT 0,
  `operateresult` tinyint(1) NOT NULL DEFAULT 0,
  `isreported` tinyint(1) NOT NULL DEFAULT 0,
  `censorword` char(50) NOT NULL,
  PRIMARY KEY (`pid`),
  KEY `type` (`tid`,`type`),
  KEY `operateresult` (`operateresult`,`createtime`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_security_evilpost` WRITE;
/*!40000 ALTER TABLE `pre_security_evilpost` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_security_evilpost` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_security_eviluser`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_security_eviluser` (
  `uid` int(10) unsigned NOT NULL,
  `evilcount` int(10) NOT NULL DEFAULT 0,
  `eviltype` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `createtime` int(10) unsigned NOT NULL DEFAULT 0,
  `operateresult` tinyint(1) NOT NULL DEFAULT 0,
  `isreported` tinyint(1) NOT NULL DEFAULT 0,
  PRIMARY KEY (`uid`),
  KEY `operateresult` (`operateresult`,`createtime`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_security_eviluser` WRITE;
/*!40000 ALTER TABLE `pre_security_eviluser` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_security_eviluser` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_security_failedlog`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_security_failedlog` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `reporttype` char(20) NOT NULL,
  `tid` int(10) unsigned NOT NULL DEFAULT 0,
  `pid` int(10) unsigned NOT NULL DEFAULT 0,
  `uid` int(10) unsigned NOT NULL DEFAULT 0,
  `failcount` int(10) unsigned NOT NULL DEFAULT 0,
  `createtime` int(10) unsigned NOT NULL DEFAULT 0,
  `posttime` int(10) unsigned NOT NULL DEFAULT 0,
  `delreason` char(255) NOT NULL,
  `scheduletime` int(10) unsigned NOT NULL DEFAULT 0,
  `lastfailtime` int(10) unsigned NOT NULL DEFAULT 0,
  `extra1` int(10) unsigned NOT NULL,
  `extra2` char(255) NOT NULL DEFAULT '0',
  PRIMARY KEY (`id`),
  KEY `pid` (`pid`),
  KEY `uid` (`uid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_security_failedlog` WRITE;
/*!40000 ALTER TABLE `pre_security_failedlog` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_security_failedlog` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_ucenter_admins`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_ucenter_admins` (
  `uid` mediumint(8) unsigned NOT NULL AUTO_INCREMENT,
  `username` char(50) NOT NULL DEFAULT '',
  `allowadminsetting` tinyint(1) NOT NULL DEFAULT 0,
  `allowadminapp` tinyint(1) NOT NULL DEFAULT 0,
  `allowadminuser` tinyint(1) NOT NULL DEFAULT 0,
  `allowadminbadword` tinyint(1) NOT NULL DEFAULT 0,
  `allowadmintag` tinyint(1) NOT NULL DEFAULT 0,
  `allowadminpm` tinyint(1) NOT NULL DEFAULT 0,
  `allowadmincredits` tinyint(1) NOT NULL DEFAULT 0,
  `allowadmindomain` tinyint(1) NOT NULL DEFAULT 0,
  `allowadmindb` tinyint(1) NOT NULL DEFAULT 0,
  `allowadminnote` tinyint(1) NOT NULL DEFAULT 0,
  `allowadmincache` tinyint(1) NOT NULL DEFAULT 0,
  `allowadminlog` tinyint(1) NOT NULL DEFAULT 0,
  PRIMARY KEY (`uid`),
  UNIQUE KEY `username` (`username`)
) ENGINE=InnoDB AUTO_INCREMENT=2 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_ucenter_admins` WRITE;
/*!40000 ALTER TABLE `pre_ucenter_admins` DISABLE KEYS */;
INSERT INTO `pre_ucenter_admins` VALUES
(1,'admin',1,1,1,1,1,1,1,1,1,1,1,1);
/*!40000 ALTER TABLE `pre_ucenter_admins` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_ucenter_applications`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_ucenter_applications` (
  `appid` smallint(6) unsigned NOT NULL AUTO_INCREMENT,
  `type` varchar(16) NOT NULL DEFAULT '',
  `name` varchar(20) NOT NULL DEFAULT '',
  `url` varchar(255) NOT NULL DEFAULT '',
  `authkey` varchar(255) NOT NULL DEFAULT '',
  `ip` varchar(45) NOT NULL DEFAULT '',
  `viewprourl` varchar(255) NOT NULL,
  `apifilename` varchar(30) NOT NULL DEFAULT 'uc.php',
  `charset` varchar(8) NOT NULL DEFAULT '',
  `dbcharset` varchar(8) NOT NULL DEFAULT '',
  `synlogin` tinyint(1) NOT NULL DEFAULT 0,
  `recvnote` tinyint(1) DEFAULT 0,
  `extra` text NOT NULL,
  `tagtemplates` text NOT NULL,
  `allowips` text NOT NULL,
  PRIMARY KEY (`appid`)
) ENGINE=InnoDB AUTO_INCREMENT=2 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_ucenter_applications` WRITE;
/*!40000 ALTER TABLE `pre_ucenter_applications` DISABLE KEYS */;
INSERT INTO `pre_ucenter_applications` VALUES
(1,'DISCUZX','Discuz! Board','https://localhost:8080','G3Ue9eo1I321m7r34aa174B3NfEcYch8K5U1Z0c2T2o1GdAbBes3W3Y8y7n7df1c','','','uc.php','utf-8','utf8mb4',1,1,'','','');
/*!40000 ALTER TABLE `pre_ucenter_applications` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_ucenter_badwords`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_ucenter_badwords` (
  `id` smallint(6) unsigned NOT NULL AUTO_INCREMENT,
  `admin` varchar(50) NOT NULL DEFAULT '',
  `find` varchar(255) NOT NULL DEFAULT '',
  `replacement` varchar(255) NOT NULL DEFAULT '',
  `findpattern` varchar(255) NOT NULL DEFAULT '',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_ucenter_badwords` WRITE;
/*!40000 ALTER TABLE `pre_ucenter_badwords` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_ucenter_badwords` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_ucenter_domains`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_ucenter_domains` (
  `id` int(10) unsigned NOT NULL AUTO_INCREMENT,
  `domain` char(40) NOT NULL DEFAULT '',
  `ip` varchar(45) NOT NULL DEFAULT '',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_ucenter_domains` WRITE;
/*!40000 ALTER TABLE `pre_ucenter_domains` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_ucenter_domains` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_ucenter_failedlogins`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_ucenter_failedlogins` (
  `ip` varchar(45) NOT NULL DEFAULT '',
  `count` tinyint(3) unsigned NOT NULL DEFAULT 0,
  `lastupdate` int(10) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`ip`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_ucenter_failedlogins` WRITE;
/*!40000 ALTER TABLE `pre_ucenter_failedlogins` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_ucenter_failedlogins` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_ucenter_feeds`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_ucenter_feeds` (
  `feedid` mediumint(8) unsigned NOT NULL AUTO_INCREMENT,
  `appid` varchar(30) NOT NULL DEFAULT '',
  `icon` varchar(30) NOT NULL DEFAULT '',
  `uid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `username` varchar(50) NOT NULL DEFAULT '',
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  `hash_template` varchar(32) NOT NULL DEFAULT '',
  `hash_data` varchar(32) NOT NULL DEFAULT '',
  `title_template` text NOT NULL DEFAULT '',
  `title_data` text NOT NULL DEFAULT '',
  `body_template` text NOT NULL,
  `body_data` text NOT NULL,
  `body_general` text NOT NULL,
  `image_1` varchar(255) NOT NULL DEFAULT '',
  `image_1_link` varchar(255) NOT NULL DEFAULT '',
  `image_2` varchar(255) NOT NULL DEFAULT '',
  `image_2_link` varchar(255) NOT NULL DEFAULT '',
  `image_3` varchar(255) NOT NULL DEFAULT '',
  `image_3_link` varchar(255) NOT NULL DEFAULT '',
  `image_4` varchar(255) NOT NULL DEFAULT '',
  `image_4_link` varchar(255) NOT NULL DEFAULT '',
  `target_ids` varchar(255) NOT NULL DEFAULT '',
  PRIMARY KEY (`feedid`),
  KEY `uid` (`uid`,`dateline`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_ucenter_feeds` WRITE;
/*!40000 ALTER TABLE `pre_ucenter_feeds` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_ucenter_feeds` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_ucenter_friends`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_ucenter_friends` (
  `uid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `friendid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `direction` tinyint(1) NOT NULL DEFAULT 0,
  `version` int(10) unsigned NOT NULL AUTO_INCREMENT,
  `delstatus` tinyint(1) NOT NULL DEFAULT 0,
  `comment` char(255) NOT NULL DEFAULT '',
  PRIMARY KEY (`version`),
  KEY `uid` (`uid`),
  KEY `friendid` (`friendid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_ucenter_friends` WRITE;
/*!40000 ALTER TABLE `pre_ucenter_friends` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_ucenter_friends` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_ucenter_mailqueue`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_ucenter_mailqueue` (
  `mailid` int(10) unsigned NOT NULL AUTO_INCREMENT,
  `touid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `tomail` varchar(32) NOT NULL,
  `frommail` varchar(100) NOT NULL,
  `subject` varchar(255) NOT NULL,
  `message` text NOT NULL,
  `charset` varchar(15) NOT NULL,
  `htmlon` tinyint(1) NOT NULL DEFAULT 0,
  `level` tinyint(1) NOT NULL DEFAULT 1,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  `failures` tinyint(3) unsigned NOT NULL DEFAULT 0,
  `appid` smallint(6) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`mailid`),
  KEY `appid` (`appid`),
  KEY `level` (`level`,`failures`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_ucenter_mailqueue` WRITE;
/*!40000 ALTER TABLE `pre_ucenter_mailqueue` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_ucenter_mailqueue` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_ucenter_memberfields`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_ucenter_memberfields` (
  `uid` mediumint(8) unsigned NOT NULL,
  `blacklist` text NOT NULL,
  PRIMARY KEY (`uid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_ucenter_memberfields` WRITE;
/*!40000 ALTER TABLE `pre_ucenter_memberfields` DISABLE KEYS */;
INSERT INTO `pre_ucenter_memberfields` VALUES
(1,'');
/*!40000 ALTER TABLE `pre_ucenter_memberfields` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_ucenter_memberlogs`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_ucenter_memberlogs` (
  `lid` int(10) unsigned NOT NULL AUTO_INCREMENT,
  `uid` mediumint(8) unsigned NOT NULL,
  `action` varchar(32) NOT NULL DEFAULT '',
  `extra` varchar(255) NOT NULL DEFAULT '',
  PRIMARY KEY (`lid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_ucenter_memberlogs` WRITE;
/*!40000 ALTER TABLE `pre_ucenter_memberlogs` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_ucenter_memberlogs` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_ucenter_members`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_ucenter_members` (
  `uid` mediumint(8) unsigned NOT NULL AUTO_INCREMENT,
  `username` char(50) NOT NULL DEFAULT '',
  `password` varchar(255) NOT NULL DEFAULT '',
  `secmobicc` varchar(3) NOT NULL DEFAULT '',
  `secmobile` varchar(12) NOT NULL DEFAULT '',
  `email` varchar(255) NOT NULL DEFAULT '',
  `myid` char(30) NOT NULL DEFAULT '',
  `myidkey` char(16) NOT NULL DEFAULT '',
  `regip` varchar(45) NOT NULL DEFAULT '',
  `regdate` int(10) unsigned NOT NULL DEFAULT 0,
  `lastloginip` int(10) NOT NULL DEFAULT 0,
  `lastlogintime` int(10) unsigned NOT NULL DEFAULT 0,
  `salt` varchar(20) NOT NULL DEFAULT '',
  `secques` char(8) NOT NULL DEFAULT '',
  PRIMARY KEY (`uid`),
  UNIQUE KEY `username` (`username`),
  KEY `email` (`email`(40)),
  KEY `secmobile` (`secmobile`,`secmobicc`)
) ENGINE=InnoDB AUTO_INCREMENT=2 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_ucenter_members` WRITE;
/*!40000 ALTER TABLE `pre_ucenter_members` DISABLE KEYS */;
INSERT INTO `pre_ucenter_members` VALUES
(1,'admin','$2y$12$qaDaM4AdQHxLuAVi766kre4DlCpCCl4aAUQAlUa4Bm8.M87rCxixG','','','admin@admin.com','','','hidden',1784453695,0,0,'','');
/*!40000 ALTER TABLE `pre_ucenter_members` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_ucenter_mergemembers`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_ucenter_mergemembers` (
  `appid` smallint(6) unsigned NOT NULL,
  `username` char(50) NOT NULL,
  PRIMARY KEY (`appid`,`username`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_ucenter_mergemembers` WRITE;
/*!40000 ALTER TABLE `pre_ucenter_mergemembers` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_ucenter_mergemembers` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_ucenter_newpm`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_ucenter_newpm` (
  `uid` mediumint(8) unsigned NOT NULL,
  PRIMARY KEY (`uid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_ucenter_newpm` WRITE;
/*!40000 ALTER TABLE `pre_ucenter_newpm` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_ucenter_newpm` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_ucenter_notelist`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_ucenter_notelist` (
  `noteid` int(10) unsigned NOT NULL AUTO_INCREMENT,
  `operation` char(32) NOT NULL,
  `closed` tinyint(4) NOT NULL DEFAULT 0,
  `totalnum` smallint(6) unsigned NOT NULL DEFAULT 0,
  `succeednum` smallint(6) unsigned NOT NULL DEFAULT 0,
  `getdata` mediumtext NOT NULL,
  `postdata` mediumtext NOT NULL,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  `pri` tinyint(3) NOT NULL DEFAULT 0,
  `app1` tinyint(4) NOT NULL,
  PRIMARY KEY (`noteid`),
  KEY `closed` (`closed`,`pri`,`noteid`),
  KEY `dateline` (`dateline`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_ucenter_notelist` WRITE;
/*!40000 ALTER TABLE `pre_ucenter_notelist` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_ucenter_notelist` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_ucenter_pm_indexes`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_ucenter_pm_indexes` (
  `pmid` mediumint(8) unsigned NOT NULL AUTO_INCREMENT,
  `plid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`pmid`),
  KEY `plid` (`plid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_ucenter_pm_indexes` WRITE;
/*!40000 ALTER TABLE `pre_ucenter_pm_indexes` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_ucenter_pm_indexes` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_ucenter_pm_lists`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_ucenter_pm_lists` (
  `plid` mediumint(8) unsigned NOT NULL AUTO_INCREMENT,
  `authorid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `pmtype` tinyint(3) unsigned NOT NULL DEFAULT 0,
  `subject` varchar(80) NOT NULL,
  `members` smallint(5) unsigned NOT NULL DEFAULT 0,
  `min_max` varchar(17) NOT NULL,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  `lastmessage` text NOT NULL,
  PRIMARY KEY (`plid`),
  KEY `pmtype` (`pmtype`),
  KEY `min_max` (`min_max`),
  KEY `authorid` (`authorid`,`dateline`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_ucenter_pm_lists` WRITE;
/*!40000 ALTER TABLE `pre_ucenter_pm_lists` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_ucenter_pm_lists` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_ucenter_pm_members`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_ucenter_pm_members` (
  `plid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `uid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `isnew` tinyint(1) NOT NULL DEFAULT 0,
  `pmnum` int(10) unsigned NOT NULL DEFAULT 0,
  `lastupdate` int(10) unsigned NOT NULL DEFAULT 0,
  `lastdateline` int(10) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`plid`,`uid`),
  KEY `isnew` (`isnew`),
  KEY `lastdateline` (`uid`,`lastdateline`),
  KEY `lastupdate` (`uid`,`lastupdate`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_ucenter_pm_members` WRITE;
/*!40000 ALTER TABLE `pre_ucenter_pm_members` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_ucenter_pm_members` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_ucenter_pm_messages_0`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_ucenter_pm_messages_0` (
  `pmid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `plid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `authorid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `message` text NOT NULL,
  `delstatus` tinyint(3) unsigned NOT NULL DEFAULT 0,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`pmid`),
  KEY `plid` (`plid`,`delstatus`,`dateline`),
  KEY `dateline` (`plid`,`dateline`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_ucenter_pm_messages_0` WRITE;
/*!40000 ALTER TABLE `pre_ucenter_pm_messages_0` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_ucenter_pm_messages_0` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_ucenter_pm_messages_1`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_ucenter_pm_messages_1` (
  `pmid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `plid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `authorid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `message` text NOT NULL,
  `delstatus` tinyint(3) unsigned NOT NULL DEFAULT 0,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`pmid`),
  KEY `plid` (`plid`,`delstatus`,`dateline`),
  KEY `dateline` (`plid`,`dateline`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_ucenter_pm_messages_1` WRITE;
/*!40000 ALTER TABLE `pre_ucenter_pm_messages_1` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_ucenter_pm_messages_1` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_ucenter_pm_messages_2`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_ucenter_pm_messages_2` (
  `pmid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `plid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `authorid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `message` text NOT NULL,
  `delstatus` tinyint(3) unsigned NOT NULL DEFAULT 0,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`pmid`),
  KEY `plid` (`plid`,`delstatus`,`dateline`),
  KEY `dateline` (`plid`,`dateline`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_ucenter_pm_messages_2` WRITE;
/*!40000 ALTER TABLE `pre_ucenter_pm_messages_2` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_ucenter_pm_messages_2` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_ucenter_pm_messages_3`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_ucenter_pm_messages_3` (
  `pmid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `plid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `authorid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `message` text NOT NULL,
  `delstatus` tinyint(3) unsigned NOT NULL DEFAULT 0,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`pmid`),
  KEY `plid` (`plid`,`delstatus`,`dateline`),
  KEY `dateline` (`plid`,`dateline`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_ucenter_pm_messages_3` WRITE;
/*!40000 ALTER TABLE `pre_ucenter_pm_messages_3` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_ucenter_pm_messages_3` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_ucenter_pm_messages_4`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_ucenter_pm_messages_4` (
  `pmid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `plid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `authorid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `message` text NOT NULL,
  `delstatus` tinyint(3) unsigned NOT NULL DEFAULT 0,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`pmid`),
  KEY `plid` (`plid`,`delstatus`,`dateline`),
  KEY `dateline` (`plid`,`dateline`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_ucenter_pm_messages_4` WRITE;
/*!40000 ALTER TABLE `pre_ucenter_pm_messages_4` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_ucenter_pm_messages_4` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_ucenter_pm_messages_5`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_ucenter_pm_messages_5` (
  `pmid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `plid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `authorid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `message` text NOT NULL,
  `delstatus` tinyint(3) unsigned NOT NULL DEFAULT 0,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`pmid`),
  KEY `plid` (`plid`,`delstatus`,`dateline`),
  KEY `dateline` (`plid`,`dateline`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_ucenter_pm_messages_5` WRITE;
/*!40000 ALTER TABLE `pre_ucenter_pm_messages_5` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_ucenter_pm_messages_5` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_ucenter_pm_messages_6`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_ucenter_pm_messages_6` (
  `pmid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `plid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `authorid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `message` text NOT NULL,
  `delstatus` tinyint(3) unsigned NOT NULL DEFAULT 0,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`pmid`),
  KEY `plid` (`plid`,`delstatus`,`dateline`),
  KEY `dateline` (`plid`,`dateline`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_ucenter_pm_messages_6` WRITE;
/*!40000 ALTER TABLE `pre_ucenter_pm_messages_6` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_ucenter_pm_messages_6` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_ucenter_pm_messages_7`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_ucenter_pm_messages_7` (
  `pmid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `plid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `authorid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `message` text NOT NULL,
  `delstatus` tinyint(3) unsigned NOT NULL DEFAULT 0,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`pmid`),
  KEY `plid` (`plid`,`delstatus`,`dateline`),
  KEY `dateline` (`plid`,`dateline`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_ucenter_pm_messages_7` WRITE;
/*!40000 ALTER TABLE `pre_ucenter_pm_messages_7` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_ucenter_pm_messages_7` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_ucenter_pm_messages_8`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_ucenter_pm_messages_8` (
  `pmid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `plid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `authorid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `message` text NOT NULL,
  `delstatus` tinyint(3) unsigned NOT NULL DEFAULT 0,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`pmid`),
  KEY `plid` (`plid`,`delstatus`,`dateline`),
  KEY `dateline` (`plid`,`dateline`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_ucenter_pm_messages_8` WRITE;
/*!40000 ALTER TABLE `pre_ucenter_pm_messages_8` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_ucenter_pm_messages_8` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_ucenter_pm_messages_9`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_ucenter_pm_messages_9` (
  `pmid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `plid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `authorid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `message` text NOT NULL,
  `delstatus` tinyint(3) unsigned NOT NULL DEFAULT 0,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`pmid`),
  KEY `plid` (`plid`,`delstatus`,`dateline`),
  KEY `dateline` (`plid`,`dateline`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_ucenter_pm_messages_9` WRITE;
/*!40000 ALTER TABLE `pre_ucenter_pm_messages_9` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_ucenter_pm_messages_9` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_ucenter_protectedmembers`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_ucenter_protectedmembers` (
  `uid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `username` char(50) NOT NULL DEFAULT '',
  `appid` tinyint(3) unsigned NOT NULL DEFAULT 0,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  `admin` char(50) NOT NULL DEFAULT '0',
  UNIQUE KEY `username` (`username`,`appid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_ucenter_protectedmembers` WRITE;
/*!40000 ALTER TABLE `pre_ucenter_protectedmembers` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_ucenter_protectedmembers` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_ucenter_settings`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_ucenter_settings` (
  `k` varchar(32) NOT NULL DEFAULT '',
  `v` text NOT NULL,
  PRIMARY KEY (`k`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_ucenter_settings` WRITE;
/*!40000 ALTER TABLE `pre_ucenter_settings` DISABLE KEYS */;
INSERT INTO `pre_ucenter_settings` VALUES
('accessemail',''),
('addappbyurl','0'),
('censoremail',''),
('censorusername',''),
('chatpmmemberlimit','35'),
('chatpmthreadlimit','30'),
('dateformat','y-n-j'),
('doublee','0'),
('insecureoperation','0'),
('login_failedtime','5'),
('mailauth','1'),
('mailauth_password','password'),
('mailauth_username','username@21cn.com'),
('maildefault','username@21cn.com'),
('maildelimiter','0'),
('mailfrom','UCenter <username@21cn.com>'),
('mailport','25'),
('mailsend','1'),
('mailserver','smtp.21cn.com'),
('mailsilent','1'),
('mailtimeout','30'),
('mailusername','1'),
('nextnotetime','0'),
('pmcenter','1'),
('pmfloodctrl','15'),
('pmsendregdays','0'),
('privatepmthreadlimit','25'),
('sendpmseccode','1'),
('timeoffset','28800'),
('version','1.7.0');
/*!40000 ALTER TABLE `pre_ucenter_settings` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_ucenter_sqlcache`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_ucenter_sqlcache` (
  `sqlid` char(6) NOT NULL DEFAULT '',
  `data` char(100) NOT NULL,
  `expiry` int(10) unsigned NOT NULL,
  PRIMARY KEY (`sqlid`),
  KEY `expiry` (`expiry`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_ucenter_sqlcache` WRITE;
/*!40000 ALTER TABLE `pre_ucenter_sqlcache` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_ucenter_sqlcache` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_ucenter_tags`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_ucenter_tags` (
  `tagname` char(20) NOT NULL,
  `appid` smallint(6) unsigned NOT NULL DEFAULT 0,
  `data` mediumtext DEFAULT NULL,
  `expiration` int(10) unsigned NOT NULL,
  KEY `tagname` (`tagname`,`appid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_ucenter_tags` WRITE;
/*!40000 ALTER TABLE `pre_ucenter_tags` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_ucenter_tags` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `pre_ucenter_vars`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `pre_ucenter_vars` (
  `name` char(32) NOT NULL DEFAULT '',
  `value` char(255) NOT NULL DEFAULT '',
  PRIMARY KEY (`name`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `pre_ucenter_vars` WRITE;
/*!40000 ALTER TABLE `pre_ucenter_vars` DISABLE KEYS */;
/*!40000 ALTER TABLE `pre_ucenter_vars` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `uc_admins`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `uc_admins` (
  `uid` mediumint(8) unsigned NOT NULL AUTO_INCREMENT,
  `username` char(50) NOT NULL DEFAULT '',
  `allowadminsetting` tinyint(1) NOT NULL DEFAULT 0,
  `allowadminapp` tinyint(1) NOT NULL DEFAULT 0,
  `allowadminuser` tinyint(1) NOT NULL DEFAULT 0,
  `allowadminbadword` tinyint(1) NOT NULL DEFAULT 0,
  `allowadmintag` tinyint(1) NOT NULL DEFAULT 0,
  `allowadminpm` tinyint(1) NOT NULL DEFAULT 0,
  `allowadmincredits` tinyint(1) NOT NULL DEFAULT 0,
  `allowadmindomain` tinyint(1) NOT NULL DEFAULT 0,
  `allowadmindb` tinyint(1) NOT NULL DEFAULT 0,
  `allowadminnote` tinyint(1) NOT NULL DEFAULT 0,
  `allowadmincache` tinyint(1) NOT NULL DEFAULT 0,
  `allowadminlog` tinyint(1) NOT NULL DEFAULT 0,
  PRIMARY KEY (`uid`),
  UNIQUE KEY `username` (`username`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `uc_admins` WRITE;
/*!40000 ALTER TABLE `uc_admins` DISABLE KEYS */;
/*!40000 ALTER TABLE `uc_admins` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `uc_applications`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `uc_applications` (
  `appid` smallint(6) unsigned NOT NULL AUTO_INCREMENT,
  `type` varchar(16) NOT NULL DEFAULT '',
  `name` varchar(20) NOT NULL DEFAULT '',
  `url` varchar(255) NOT NULL DEFAULT '',
  `authkey` varchar(255) NOT NULL DEFAULT '',
  `ip` varchar(45) NOT NULL DEFAULT '',
  `viewprourl` varchar(255) NOT NULL,
  `apifilename` varchar(30) NOT NULL DEFAULT 'uc.php',
  `charset` varchar(8) NOT NULL DEFAULT '',
  `dbcharset` varchar(8) NOT NULL DEFAULT '',
  `synlogin` tinyint(1) NOT NULL DEFAULT 0,
  `recvnote` tinyint(1) DEFAULT 0,
  `extra` text NOT NULL,
  `tagtemplates` text NOT NULL,
  `allowips` text NOT NULL,
  PRIMARY KEY (`appid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `uc_applications` WRITE;
/*!40000 ALTER TABLE `uc_applications` DISABLE KEYS */;
/*!40000 ALTER TABLE `uc_applications` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `uc_badwords`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `uc_badwords` (
  `id` smallint(6) unsigned NOT NULL AUTO_INCREMENT,
  `admin` varchar(50) NOT NULL DEFAULT '',
  `find` varchar(255) NOT NULL DEFAULT '',
  `replacement` varchar(255) NOT NULL DEFAULT '',
  `findpattern` varchar(255) NOT NULL DEFAULT '',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `uc_badwords` WRITE;
/*!40000 ALTER TABLE `uc_badwords` DISABLE KEYS */;
/*!40000 ALTER TABLE `uc_badwords` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `uc_domains`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `uc_domains` (
  `id` int(10) unsigned NOT NULL AUTO_INCREMENT,
  `domain` char(40) NOT NULL DEFAULT '',
  `ip` varchar(45) NOT NULL DEFAULT '',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `uc_domains` WRITE;
/*!40000 ALTER TABLE `uc_domains` DISABLE KEYS */;
/*!40000 ALTER TABLE `uc_domains` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `uc_failedlogins`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `uc_failedlogins` (
  `ip` varchar(45) NOT NULL DEFAULT '',
  `count` tinyint(3) unsigned NOT NULL DEFAULT 0,
  `lastupdate` int(10) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`ip`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `uc_failedlogins` WRITE;
/*!40000 ALTER TABLE `uc_failedlogins` DISABLE KEYS */;
/*!40000 ALTER TABLE `uc_failedlogins` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `uc_feeds`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `uc_feeds` (
  `feedid` mediumint(8) unsigned NOT NULL AUTO_INCREMENT,
  `appid` varchar(30) NOT NULL DEFAULT '',
  `icon` varchar(30) NOT NULL DEFAULT '',
  `uid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `username` varchar(50) NOT NULL DEFAULT '',
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  `hash_template` varchar(32) NOT NULL DEFAULT '',
  `hash_data` varchar(32) NOT NULL DEFAULT '',
  `title_template` text NOT NULL DEFAULT '',
  `title_data` text NOT NULL DEFAULT '',
  `body_template` text NOT NULL,
  `body_data` text NOT NULL,
  `body_general` text NOT NULL,
  `image_1` varchar(255) NOT NULL DEFAULT '',
  `image_1_link` varchar(255) NOT NULL DEFAULT '',
  `image_2` varchar(255) NOT NULL DEFAULT '',
  `image_2_link` varchar(255) NOT NULL DEFAULT '',
  `image_3` varchar(255) NOT NULL DEFAULT '',
  `image_3_link` varchar(255) NOT NULL DEFAULT '',
  `image_4` varchar(255) NOT NULL DEFAULT '',
  `image_4_link` varchar(255) NOT NULL DEFAULT '',
  `target_ids` varchar(255) NOT NULL DEFAULT '',
  PRIMARY KEY (`feedid`),
  KEY `uid` (`uid`,`dateline`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `uc_feeds` WRITE;
/*!40000 ALTER TABLE `uc_feeds` DISABLE KEYS */;
/*!40000 ALTER TABLE `uc_feeds` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `uc_friends`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `uc_friends` (
  `uid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `friendid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `direction` tinyint(1) NOT NULL DEFAULT 0,
  `version` int(10) unsigned NOT NULL AUTO_INCREMENT,
  `delstatus` tinyint(1) NOT NULL DEFAULT 0,
  `comment` char(255) NOT NULL DEFAULT '',
  PRIMARY KEY (`version`),
  KEY `uid` (`uid`),
  KEY `friendid` (`friendid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `uc_friends` WRITE;
/*!40000 ALTER TABLE `uc_friends` DISABLE KEYS */;
/*!40000 ALTER TABLE `uc_friends` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `uc_mailqueue`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `uc_mailqueue` (
  `mailid` int(10) unsigned NOT NULL AUTO_INCREMENT,
  `touid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `tomail` varchar(32) NOT NULL,
  `frommail` varchar(100) NOT NULL,
  `subject` varchar(255) NOT NULL,
  `message` text NOT NULL,
  `charset` varchar(15) NOT NULL,
  `htmlon` tinyint(1) NOT NULL DEFAULT 0,
  `level` tinyint(1) NOT NULL DEFAULT 1,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  `failures` tinyint(3) unsigned NOT NULL DEFAULT 0,
  `appid` smallint(6) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`mailid`),
  KEY `appid` (`appid`),
  KEY `level` (`level`,`failures`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `uc_mailqueue` WRITE;
/*!40000 ALTER TABLE `uc_mailqueue` DISABLE KEYS */;
/*!40000 ALTER TABLE `uc_mailqueue` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `uc_memberfields`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `uc_memberfields` (
  `uid` mediumint(8) unsigned NOT NULL,
  `blacklist` text NOT NULL,
  PRIMARY KEY (`uid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `uc_memberfields` WRITE;
/*!40000 ALTER TABLE `uc_memberfields` DISABLE KEYS */;
/*!40000 ALTER TABLE `uc_memberfields` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `uc_memberlogs`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `uc_memberlogs` (
  `lid` int(10) unsigned NOT NULL AUTO_INCREMENT,
  `uid` mediumint(8) unsigned NOT NULL,
  `action` varchar(32) NOT NULL DEFAULT '',
  `extra` varchar(255) NOT NULL DEFAULT '',
  PRIMARY KEY (`lid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `uc_memberlogs` WRITE;
/*!40000 ALTER TABLE `uc_memberlogs` DISABLE KEYS */;
/*!40000 ALTER TABLE `uc_memberlogs` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `uc_members`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `uc_members` (
  `uid` mediumint(8) unsigned NOT NULL AUTO_INCREMENT,
  `username` char(50) NOT NULL DEFAULT '',
  `password` varchar(255) NOT NULL DEFAULT '',
  `secmobicc` varchar(3) NOT NULL DEFAULT '',
  `secmobile` varchar(12) NOT NULL DEFAULT '',
  `email` varchar(255) NOT NULL DEFAULT '',
  `myid` char(30) NOT NULL DEFAULT '',
  `myidkey` char(16) NOT NULL DEFAULT '',
  `regip` varchar(45) NOT NULL DEFAULT '',
  `regdate` int(10) unsigned NOT NULL DEFAULT 0,
  `lastloginip` int(10) NOT NULL DEFAULT 0,
  `lastlogintime` int(10) unsigned NOT NULL DEFAULT 0,
  `salt` varchar(20) NOT NULL DEFAULT '',
  `secques` char(8) NOT NULL DEFAULT '',
  PRIMARY KEY (`uid`),
  UNIQUE KEY `username` (`username`),
  KEY `email` (`email`(40)),
  KEY `secmobile` (`secmobile`,`secmobicc`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `uc_members` WRITE;
/*!40000 ALTER TABLE `uc_members` DISABLE KEYS */;
/*!40000 ALTER TABLE `uc_members` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `uc_mergemembers`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `uc_mergemembers` (
  `appid` smallint(6) unsigned NOT NULL,
  `username` char(50) NOT NULL,
  PRIMARY KEY (`appid`,`username`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `uc_mergemembers` WRITE;
/*!40000 ALTER TABLE `uc_mergemembers` DISABLE KEYS */;
/*!40000 ALTER TABLE `uc_mergemembers` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `uc_newpm`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `uc_newpm` (
  `uid` mediumint(8) unsigned NOT NULL,
  PRIMARY KEY (`uid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `uc_newpm` WRITE;
/*!40000 ALTER TABLE `uc_newpm` DISABLE KEYS */;
/*!40000 ALTER TABLE `uc_newpm` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `uc_notelist`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `uc_notelist` (
  `noteid` int(10) unsigned NOT NULL AUTO_INCREMENT,
  `operation` char(32) NOT NULL,
  `closed` tinyint(4) NOT NULL DEFAULT 0,
  `totalnum` smallint(6) unsigned NOT NULL DEFAULT 0,
  `succeednum` smallint(6) unsigned NOT NULL DEFAULT 0,
  `getdata` mediumtext NOT NULL,
  `postdata` mediumtext NOT NULL,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  `pri` tinyint(3) NOT NULL DEFAULT 0,
  PRIMARY KEY (`noteid`),
  KEY `closed` (`closed`,`pri`,`noteid`),
  KEY `dateline` (`dateline`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `uc_notelist` WRITE;
/*!40000 ALTER TABLE `uc_notelist` DISABLE KEYS */;
/*!40000 ALTER TABLE `uc_notelist` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `uc_pm_indexes`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `uc_pm_indexes` (
  `pmid` mediumint(8) unsigned NOT NULL AUTO_INCREMENT,
  `plid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`pmid`),
  KEY `plid` (`plid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `uc_pm_indexes` WRITE;
/*!40000 ALTER TABLE `uc_pm_indexes` DISABLE KEYS */;
/*!40000 ALTER TABLE `uc_pm_indexes` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `uc_pm_lists`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `uc_pm_lists` (
  `plid` mediumint(8) unsigned NOT NULL AUTO_INCREMENT,
  `authorid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `pmtype` tinyint(3) unsigned NOT NULL DEFAULT 0,
  `subject` varchar(80) NOT NULL,
  `members` smallint(5) unsigned NOT NULL DEFAULT 0,
  `min_max` varchar(17) NOT NULL,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  `lastmessage` text NOT NULL,
  PRIMARY KEY (`plid`),
  KEY `pmtype` (`pmtype`),
  KEY `min_max` (`min_max`),
  KEY `authorid` (`authorid`,`dateline`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `uc_pm_lists` WRITE;
/*!40000 ALTER TABLE `uc_pm_lists` DISABLE KEYS */;
/*!40000 ALTER TABLE `uc_pm_lists` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `uc_pm_members`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `uc_pm_members` (
  `plid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `uid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `isnew` tinyint(1) NOT NULL DEFAULT 0,
  `pmnum` int(10) unsigned NOT NULL DEFAULT 0,
  `lastupdate` int(10) unsigned NOT NULL DEFAULT 0,
  `lastdateline` int(10) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`plid`,`uid`),
  KEY `isnew` (`isnew`),
  KEY `lastdateline` (`uid`,`lastdateline`),
  KEY `lastupdate` (`uid`,`lastupdate`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `uc_pm_members` WRITE;
/*!40000 ALTER TABLE `uc_pm_members` DISABLE KEYS */;
/*!40000 ALTER TABLE `uc_pm_members` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `uc_pm_messages_0`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `uc_pm_messages_0` (
  `pmid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `plid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `authorid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `message` text NOT NULL,
  `delstatus` tinyint(3) unsigned NOT NULL DEFAULT 0,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`pmid`),
  KEY `plid` (`plid`,`delstatus`,`dateline`),
  KEY `dateline` (`plid`,`dateline`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `uc_pm_messages_0` WRITE;
/*!40000 ALTER TABLE `uc_pm_messages_0` DISABLE KEYS */;
/*!40000 ALTER TABLE `uc_pm_messages_0` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `uc_pm_messages_1`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `uc_pm_messages_1` (
  `pmid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `plid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `authorid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `message` text NOT NULL,
  `delstatus` tinyint(3) unsigned NOT NULL DEFAULT 0,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`pmid`),
  KEY `plid` (`plid`,`delstatus`,`dateline`),
  KEY `dateline` (`plid`,`dateline`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `uc_pm_messages_1` WRITE;
/*!40000 ALTER TABLE `uc_pm_messages_1` DISABLE KEYS */;
/*!40000 ALTER TABLE `uc_pm_messages_1` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `uc_pm_messages_2`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `uc_pm_messages_2` (
  `pmid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `plid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `authorid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `message` text NOT NULL,
  `delstatus` tinyint(3) unsigned NOT NULL DEFAULT 0,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`pmid`),
  KEY `plid` (`plid`,`delstatus`,`dateline`),
  KEY `dateline` (`plid`,`dateline`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `uc_pm_messages_2` WRITE;
/*!40000 ALTER TABLE `uc_pm_messages_2` DISABLE KEYS */;
/*!40000 ALTER TABLE `uc_pm_messages_2` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `uc_pm_messages_3`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `uc_pm_messages_3` (
  `pmid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `plid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `authorid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `message` text NOT NULL,
  `delstatus` tinyint(3) unsigned NOT NULL DEFAULT 0,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`pmid`),
  KEY `plid` (`plid`,`delstatus`,`dateline`),
  KEY `dateline` (`plid`,`dateline`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `uc_pm_messages_3` WRITE;
/*!40000 ALTER TABLE `uc_pm_messages_3` DISABLE KEYS */;
/*!40000 ALTER TABLE `uc_pm_messages_3` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `uc_pm_messages_4`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `uc_pm_messages_4` (
  `pmid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `plid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `authorid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `message` text NOT NULL,
  `delstatus` tinyint(3) unsigned NOT NULL DEFAULT 0,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`pmid`),
  KEY `plid` (`plid`,`delstatus`,`dateline`),
  KEY `dateline` (`plid`,`dateline`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `uc_pm_messages_4` WRITE;
/*!40000 ALTER TABLE `uc_pm_messages_4` DISABLE KEYS */;
/*!40000 ALTER TABLE `uc_pm_messages_4` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `uc_pm_messages_5`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `uc_pm_messages_5` (
  `pmid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `plid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `authorid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `message` text NOT NULL,
  `delstatus` tinyint(3) unsigned NOT NULL DEFAULT 0,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`pmid`),
  KEY `plid` (`plid`,`delstatus`,`dateline`),
  KEY `dateline` (`plid`,`dateline`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `uc_pm_messages_5` WRITE;
/*!40000 ALTER TABLE `uc_pm_messages_5` DISABLE KEYS */;
/*!40000 ALTER TABLE `uc_pm_messages_5` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `uc_pm_messages_6`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `uc_pm_messages_6` (
  `pmid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `plid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `authorid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `message` text NOT NULL,
  `delstatus` tinyint(3) unsigned NOT NULL DEFAULT 0,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`pmid`),
  KEY `plid` (`plid`,`delstatus`,`dateline`),
  KEY `dateline` (`plid`,`dateline`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `uc_pm_messages_6` WRITE;
/*!40000 ALTER TABLE `uc_pm_messages_6` DISABLE KEYS */;
/*!40000 ALTER TABLE `uc_pm_messages_6` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `uc_pm_messages_7`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `uc_pm_messages_7` (
  `pmid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `plid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `authorid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `message` text NOT NULL,
  `delstatus` tinyint(3) unsigned NOT NULL DEFAULT 0,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`pmid`),
  KEY `plid` (`plid`,`delstatus`,`dateline`),
  KEY `dateline` (`plid`,`dateline`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `uc_pm_messages_7` WRITE;
/*!40000 ALTER TABLE `uc_pm_messages_7` DISABLE KEYS */;
/*!40000 ALTER TABLE `uc_pm_messages_7` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `uc_pm_messages_8`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `uc_pm_messages_8` (
  `pmid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `plid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `authorid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `message` text NOT NULL,
  `delstatus` tinyint(3) unsigned NOT NULL DEFAULT 0,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`pmid`),
  KEY `plid` (`plid`,`delstatus`,`dateline`),
  KEY `dateline` (`plid`,`dateline`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `uc_pm_messages_8` WRITE;
/*!40000 ALTER TABLE `uc_pm_messages_8` DISABLE KEYS */;
/*!40000 ALTER TABLE `uc_pm_messages_8` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `uc_pm_messages_9`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `uc_pm_messages_9` (
  `pmid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `plid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `authorid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `message` text NOT NULL,
  `delstatus` tinyint(3) unsigned NOT NULL DEFAULT 0,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  PRIMARY KEY (`pmid`),
  KEY `plid` (`plid`,`delstatus`,`dateline`),
  KEY `dateline` (`plid`,`dateline`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `uc_pm_messages_9` WRITE;
/*!40000 ALTER TABLE `uc_pm_messages_9` DISABLE KEYS */;
/*!40000 ALTER TABLE `uc_pm_messages_9` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `uc_protectedmembers`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `uc_protectedmembers` (
  `uid` mediumint(8) unsigned NOT NULL DEFAULT 0,
  `username` char(50) NOT NULL DEFAULT '',
  `appid` tinyint(3) unsigned NOT NULL DEFAULT 0,
  `dateline` int(10) unsigned NOT NULL DEFAULT 0,
  `admin` char(50) NOT NULL DEFAULT '0',
  UNIQUE KEY `username` (`username`,`appid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `uc_protectedmembers` WRITE;
/*!40000 ALTER TABLE `uc_protectedmembers` DISABLE KEYS */;
/*!40000 ALTER TABLE `uc_protectedmembers` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `uc_settings`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `uc_settings` (
  `k` varchar(32) NOT NULL DEFAULT '',
  `v` text NOT NULL,
  PRIMARY KEY (`k`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `uc_settings` WRITE;
/*!40000 ALTER TABLE `uc_settings` DISABLE KEYS */;
INSERT INTO `uc_settings` VALUES
('accessemail',''),
('addappbyurl','0'),
('censoremail',''),
('censorusername',''),
('chatpmmemberlimit','35'),
('chatpmthreadlimit','30'),
('dateformat','y-n-j'),
('doublee','0'),
('insecureoperation','0'),
('login_failedtime','5'),
('mailauth','1'),
('mailauth_password','password'),
('mailauth_username','username@21cn.com'),
('maildefault','username@21cn.com'),
('maildelimiter','0'),
('mailfrom','UCenter <username@21cn.com>'),
('mailport','25'),
('mailsend','1'),
('mailserver','smtp.21cn.com'),
('mailsilent','1'),
('mailtimeout','30'),
('mailusername','1'),
('nextnotetime','0'),
('pmcenter','1'),
('pmfloodctrl','15'),
('pmsendregdays','0'),
('privatepmthreadlimit','25'),
('sendpmseccode','1'),
('timeoffset','28800'),
('version','1.7.0');
/*!40000 ALTER TABLE `uc_settings` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `uc_sqlcache`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `uc_sqlcache` (
  `sqlid` char(6) NOT NULL DEFAULT '',
  `data` char(100) NOT NULL,
  `expiry` int(10) unsigned NOT NULL,
  PRIMARY KEY (`sqlid`),
  KEY `expiry` (`expiry`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `uc_sqlcache` WRITE;
/*!40000 ALTER TABLE `uc_sqlcache` DISABLE KEYS */;
/*!40000 ALTER TABLE `uc_sqlcache` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `uc_tags`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `uc_tags` (
  `tagname` char(20) NOT NULL,
  `appid` smallint(6) unsigned NOT NULL DEFAULT 0,
  `data` mediumtext DEFAULT NULL,
  `expiration` int(10) unsigned NOT NULL,
  KEY `tagname` (`tagname`,`appid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `uc_tags` WRITE;
/*!40000 ALTER TABLE `uc_tags` DISABLE KEYS */;
/*!40000 ALTER TABLE `uc_tags` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `uc_vars`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `uc_vars` (
  `name` char(32) NOT NULL DEFAULT '',
  `value` char(255) NOT NULL DEFAULT '',
  PRIMARY KEY (`name`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `uc_vars` WRITE;
/*!40000 ALTER TABLE `uc_vars` DISABLE KEYS */;
/*!40000 ALTER TABLE `uc_vars` ENABLE KEYS */;
UNLOCK TABLES;
/*!40103 SET TIME_ZONE=@OLD_TIME_ZONE */;

/*!40101 SET SQL_MODE=@OLD_SQL_MODE */;
/*!40014 SET FOREIGN_KEY_CHECKS=@OLD_FOREIGN_KEY_CHECKS */;
/*!40014 SET UNIQUE_CHECKS=@OLD_UNIQUE_CHECKS */;
/*!40101 SET CHARACTER_SET_CLIENT=@OLD_CHARACTER_SET_CLIENT */;
/*!40101 SET CHARACTER_SET_RESULTS=@OLD_CHARACTER_SET_RESULTS */;
/*!40101 SET COLLATION_CONNECTION=@OLD_COLLATION_CONNECTION */;
/*!40111 SET SQL_NOTES=@OLD_SQL_NOTES */;

