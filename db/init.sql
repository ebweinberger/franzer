START TRANSACTION;
CREATE DATABASE IF NOT EXISTS franzerdb;
USE franzerdb;
CREATE TABLE IF NOT EXISTS entries
(
    id INT unsigned NOT NULL AUTO_INCREMENT PRIMARY KEY,
    `namespace` VARCHAR(255) NOT NULL,
    `type` ENUM('text', 'file') NOT NULL,
    `text` LONGTEXT,
    `file` LONGBLOB
);
COMMIT;