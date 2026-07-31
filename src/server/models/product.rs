use crate::server::models::ProductVariant;

#[derive(toasty::Model)]
pub struct Product {
    #[key]
    #[auto]
    pub id: u64,

    #[column(type = varchar(255))]
    pub name: String,

    #[unique]
    #[column(type = varchar(255))]
    pub slug: String,

    #[column(type = text)]
    pub description: Option<String>,

    /// Main showcase image for store grids and search results
    #[column(type = varchar(255))]
    pub featured_image_url: String,

    pub is_active: bool,

    /// Merch variants (e.g., "M / Black", "Poster 18x24", "Vinyl LP")
    #[has_many]
    pub variants: toasty::Deferred<Vec<ProductVariant>>,

    #[auto]
    pub created_at: jiff::Timestamp,

    #[auto]
    pub updated_at: jiff::Timestamp,
}
