use crate::client::YookassaClient;
use crate::error::YookassaError;
use crate::model::{DealId, ListResponse, Metadata, MonetaryAmount, TimeFilter};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DealType {
    SafeDeal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FeeMoment {
    PaymentSucceeded,
    DealClosed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DealStatus {
    Opened,
    Closed,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateDealRequest {
    pub r#type: DealType,
    pub fee_moment: FeeMoment,
    pub metadata: Option<Metadata>,
    pub description: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DealListQuery {
    #[serde(flatten)]
    pub time: TimeFilter,
    #[serde(rename = "expires_at.gte")]
    pub expires_at_gte: Option<DateTime<Utc>>,
    #[serde(rename = "expires_at.gt")]
    pub expires_at_gt: Option<DateTime<Utc>>,
    #[serde(rename = "expires_at.lte")]
    pub expires_at_lte: Option<DateTime<Utc>>,
    #[serde(rename = "expires_at.lt")]
    pub expires_at_lt: Option<DateTime<Utc>>,
    pub status: Option<DealStatus>,
    pub full_text_search: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Deal {
    pub r#type: DealType,
    pub id: DealId,
    pub fee_moment: FeeMoment,
    pub description: Option<String>,
    pub balance: MonetaryAmount,
    pub payout_balance: MonetaryAmount,
    pub status: DealStatus,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub metadata: Option<Metadata>,
    pub test: bool,
}

impl YookassaClient {
    pub async fn create_deal(
        &self,
        request: &CreateDealRequest,
        idempotence_key: Option<&str>,
    ) -> Result<Deal, YookassaError> {
        self.post("deals", request, idempotence_key).await
    }

    pub async fn list_deals(
        &self,
        query: &DealListQuery,
    ) -> Result<ListResponse<Deal>, YookassaError> {
        self.get("deals", Some(query)).await
    }

    pub async fn get_deal(&self, deal_id: &DealId) -> Result<Deal, YookassaError> {
        self.get(&format!("deals/{}", deal_id.0), Option::<&()>::None)
            .await
    }
}
