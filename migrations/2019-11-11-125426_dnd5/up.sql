-- Your SQL goes here
CREATE TABLE `character`
(
    `id`              INT(11) PRIMARY KEY NOT NULL AUTO_INCREMENT,
    `character_name`  VARCHAR(60)         NOT NULL,
    `character_class` VARCHAR(30)         NOT NULL,
    `race`            VARCHAR(30)         NOT NULL,
    `image`           VARCHAR(255),
    `level`           INT(2)              NOT NULL,
    `max_hp`          INT(4)              NOT NULL,
    `current_hp`      INT(4)              NOT NULL,
    `hit_dice`        INT(2)              NOT NULL
);

CREATE TABLE `spell`
(
    `id`           INT(11) PRIMARY KEY NOT NULL AUTO_INCREMENT,
    `character_id` INT(11)             NOT NULL,
    `prepared`     BOOL,
    `data_url`     VARCHAR(255)        NOT NULL,
    FOREIGN KEY (`character_id`) REFERENCES `character` (`id`)
);

CREATE TABLE `skill`
(
    `id`           INT(11) PRIMARY KEY NOT NULL AUTO_INCREMENT,
    `character_id` INT(11)             NOT NULL,
    `name`         VARCHAR(30)         NOT NULL,
    `description`  TEXT                NOT NULL,
    `max_uses`     INT(2)              NOT NULL,
    `current_uses` INT(2)              NOT NULL,
    `recover`      VARCHAR(30)         NOT NULL,
    FOREIGN KEY (`character_id`) REFERENCES `character` (`id`)
);

CREATE TABLE `spell_slot`
(
    `id`               INT(11) PRIMARY KEY NOT NULL AUTO_INCREMENT,
    `character_id`     INT(11)             NOT NULL,
    `level`            INT(1)              NOT NULL,
    `current_quantity` INT(1)              NOT NULL,
    `max_quantity`     INT(1)              NOT NULL,
    FOREIGN KEY (`character_id`) REFERENCES `character` (`id`)
);
