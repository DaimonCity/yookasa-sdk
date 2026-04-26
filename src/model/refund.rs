use crate::client::YookassaClient;
use crate::error::YookassaError;
use crate::model::{
    ListResponse, Metadata, MonetaryAmount, PaymentId, ReceiptData, ReceiptRegistrationStatus,
    RefundId, TimeFilter,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RefundSourceData {
    pub account_id: crate::model::AccountId,
    pub amount: MonetaryAmount,
    pub platform_fee_amount: Option<MonetaryAmount>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RefundDealData {
    pub refund_settlements: Vec<crate::model::Settlement>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateRefundRequest {
    pub payment_id: PaymentId,
    pub amount: MonetaryAmount,
    pub description: Option<String>,
    pub receipt: Option<ReceiptData>,
    pub sources: Option<Vec<RefundSourceData>>,
    pub deal: Option<RefundDealData>,
    pub refund_method_data: Option<serde_json::Value>,
    pub metadata: Option<Metadata>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RefundStatus {
    Pending,
    Succeeded,
    Canceled,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RefundListQuery {
    #[serde(flatten)]
    pub time: TimeFilter,
    pub payment_id: Option<PaymentId>,
    pub status: Option<RefundStatus>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Refund {
    pub id: RefundId,
    pub payment_id: PaymentId,
    pub status: RefundStatus,
    pub cancellation_details: Option<serde_json::Value>,
    pub receipt_registration: Option<ReceiptRegistrationStatus>,
    pub created_at: DateTime<Utc>,
    pub amount: MonetaryAmount,
    pub description: Option<String>,
    pub sources: Option<Vec<RefundSourceData>>,
    pub deal: Option<serde_json::Value>,
    pub refund_method: Option<serde_json::Value>,
    pub refund_authorization_details: Option<serde_json::Value>,
    pub metadata: Option<Metadata>,
}

impl YookassaClient {
    pub async fn create_refund(
        &self,
        request: &CreateRefundRequest,
        idempotence_key: Option<&str>,
    ) -> Result<Refund, YookassaError> {
        self.post("refunds", request, idempotence_key).await
    }

    pub async fn list_refunds(
        &self,
        query: &RefundListQuery,
    ) -> Result<ListResponse<Refund>, YookassaError> {
        self.get("refunds", Some(query)).await
    }

    pub async fn get_refund(&self, refund_id: &RefundId) -> Result<Refund, YookassaError> {
        self.get(&format!("refunds/{}", refund_id.0), Option::<&()>::None)
            .await
    }
}
