CREATE TABLE `app_caches` (
    `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
    `key` VARCHAR(255) NOT NULL,
    `value` VARCHAR(255) NOT NULL,
    `expires_at` DATETIME(6) NOT NULL,
    PRIMARY KEY (`id`)
);
-- #[toasty::breakpoint]
CREATE UNIQUE INDEX `index_app_caches_by_key` ON `app_caches` (`key`);
