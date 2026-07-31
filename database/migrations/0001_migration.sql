CREATE TABLE `products` (
    `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
    `name` VARCHAR(255) NOT NULL,
    `slug` VARCHAR(255) NOT NULL,
    `description` TEXT,
    `featured_image_url` VARCHAR(255) NOT NULL,
    `is_active` BOOLEAN NOT NULL,
    `created_at` DATETIME(6) NOT NULL,
    `updated_at` DATETIME(6) NOT NULL,
    PRIMARY KEY (`id`)
);
-- #[toasty::breakpoint]
CREATE UNIQUE INDEX `index_products_by_slug` ON `products` (`slug`);
-- #[toasty::breakpoint]
CREATE TABLE `product_variants` (
    `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
    `product_id` BIGINT UNSIGNED NOT NULL,
    `image_url` VARCHAR(255),
    `sku` VARCHAR(191) NOT NULL,
    `title` VARCHAR(255) NOT NULL,
    `price_cents` BIGINT UNSIGNED NOT NULL,
    `currency_symbol` VARCHAR(3) NOT NULL,
    `stock_quantity` INT UNSIGNED NOT NULL,
    `created_at` DATETIME(6) NOT NULL,
    `updated_at` DATETIME(6) NOT NULL,
    PRIMARY KEY (`id`)
);
-- #[toasty::breakpoint]
CREATE INDEX `index_product_variants_by_product_id` ON `product_variants` (`product_id`);
-- #[toasty::breakpoint]
CREATE UNIQUE INDEX `index_product_variants_by_sku` ON `product_variants` (`sku`);
