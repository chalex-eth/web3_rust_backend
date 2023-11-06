use crate::counter::Counter;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Json, Response},
    Extension,
};
use ethers::{prelude::ContractError, providers::Middleware};
use ethers::{
    providers::{Http, Provider, ProviderError},
    types::U256,
};
use thiserror::Error;
use tracing::info;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error(": ContractError {0}")]
    ContractError(#[from] ContractError<Provider<Http>>),
    #[error(": ProviderError {0}")]
    ProviderError(#[from] ProviderError),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let body = match self {
            ApiError::ContractError(err) => format!("Contract Error: {}", err),
            ApiError::ProviderError(err) => format!("Provider Error: {}", err),
        };

        (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}

pub async fn handle_number(
    Extension(counter): Extension<Counter>,
) -> Result<Json<String>, ApiError> {
    let number: U256 = counter.get_number().await?;
    info!("API:: Number served");

    Ok(Json(number.to_string()))
}

pub async fn handle_block_number(
    Extension(counter): Extension<Counter>,
) -> Result<Json<String>, ApiError> {
    let block_number = counter.client.get_block_number().await?;
    info!("API:: Block number served");

    Ok(Json(block_number.to_string()))
}
