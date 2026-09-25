use std::fs::read_to_string;

use merch_store::server::{models::Product, types::app_config::AppConfig};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct ProductSeed {
    name: String,
    slug: String,
    description: String,
    featured_image_url: String,
    is_active: bool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let AppConfig {
        daraja: _,
        database,
    } = AppConfig::load();

    let mut db = toasty::Db::builder()
        .models(toasty::models!(merch_store::*))
        .connect(&database.connection)
        .await?;

    let products_json = read_to_string("./src/bin/seed/products.json")?;
    let products: Vec<ProductSeed> = serde_json::from_str(&products_json)?;

    let mut insertions = vec![];
    for product_seed in products {
        insertions.push(toasty::create!(Product {
            name: product_seed.name,
            slug: product_seed.slug,
            description: product_seed.description,
            featured_image_url: product_seed.featured_image_url,
            is_active: product_seed.is_active
        }));
    }

    toasty::batch(insertions).exec(&mut db).await?;

    Ok(())
}
