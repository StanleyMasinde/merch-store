use crate::server::{app::AppState, errors::AppError, models::Product};
use axum::{
    Json,
    extract::{Path, State},
};

#[axum::debug_handler]
pub async fn list_products(State(state): State<AppState>) -> Result<Json<Vec<Product>>, AppError> {
    let AppState { mut db, config: _ } = state;

    let products = Product::all()
        .exec(&mut db)
        .await
        .map_err(|err| AppError::DatabaseError(err.to_string()))?;

    Ok(Json(products))
}

pub async fn create_product() {}

pub async fn show_product(
    Path(id): Path<u64>,
    State(state): State<AppState>,
) -> Result<Json<Product>, AppError> {
    let AppState { mut db, config: _ } = state;
    let product = Product::get_by_id(&mut db, id)
        .await
        .map_err(|_err| AppError::ModelNotFound)?;

    Ok(Json(product))
}

pub async fn update_product() {}

pub async fn delete_product() {}
