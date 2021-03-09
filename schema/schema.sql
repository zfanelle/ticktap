CREATE DATABASE `ticktap` /*!40100 DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci */ /*!80016 DEFAULT ENCRYPTION='N' */;

-- ticktap.account definition

CREATE TABLE ticktap.`account` (
  `id` int NOT NULL AUTO_INCREMENT,
  `entity_name` varchar(1000) NOT NULL,
  `entity_type` enum('personal','corporate','host') NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=18 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;


-- ticktap.event definition

CREATE TABLE ticktap.`event` (
  `id` int NOT NULL AUTO_INCREMENT,
  `host` int NOT NULL,
  `event_name` varchar(1000) NOT NULL,
  PRIMARY KEY (`id`),
  KEY `event_FK` (`host`),
  CONSTRAINT `event_FK` FOREIGN KEY (`host`) REFERENCES `account` (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;


-- ticktap.`transaction` definition

CREATE TABLE ticktap.`transaction` (
  `id` int NOT NULL AUTO_INCREMENT,
  `event` int NOT NULL,
  `account` int NOT NULL,
  PRIMARY KEY (`id`),
  KEY `transaction_FK` (`event`),
  KEY `transaction_FK_1` (`account`),
  CONSTRAINT `transaction_FK` FOREIGN KEY (`event`) REFERENCES `event` (`id`),
  CONSTRAINT `transaction_FK_1` FOREIGN KEY (`account`) REFERENCES `account` (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;


-- ticktap.ticket definition

CREATE TABLE ticktap.`ticket` (
  `id` int NOT NULL AUTO_INCREMENT,
  `transaction` int NOT NULL,
  PRIMARY KEY (`id`),
  CONSTRAINT `ticket_FK` FOREIGN KEY (`id`) REFERENCES `transaction` (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;