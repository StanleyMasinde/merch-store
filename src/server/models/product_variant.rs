use crate::server::models::Product;

#[derive(toasty::Model)]
pub struct ProductVariant {
    #[key]
    #[auto]
    pub id: u64,

    #[index]
    pub product_id: u64,

    #[belongs_to(key = product_id, references = id)]
    pub product: toasty::Deferred<Product>,

    /// Variant-specific image (e.g. close-up of the black shirt mockup)
    #[column(type = varchar(255))]
    pub image_url: Option<String>,

    /// e.g. "TSHIRT-BLK-L"
    #[unique]
    pub sku: String,

    /// Human-readable variant option name (e.g. "Large / Black")
    #[column(type = varchar(255))]
    pub title: String,

    /// Prices are per-variant (e.g., 2XL costs $2 more, or vinyl costs more than CD)
    pub price_cents: u64,

    /// The currency
    #[column(type = varchar(3))]
    pub currency_symbol: String,

    /// Available inventory stock count for this specific SKU
    pub stock_quantity: u32,

    #[auto]
    #[default(jiff::Timestamp::now())]
    pub created_at: jiff::Timestamp,

    #[auto]
    #[update(jiff::Timestamp::now())]
    pub updated_at: jiff::Timestamp,
}
