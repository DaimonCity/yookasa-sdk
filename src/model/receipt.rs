use crate::client::YookassaClient;
use crate::error::YookassaError;
use crate::model::{
    IndustryDetails, ListResponse, MonetaryAmount, OperationalDetails, PaymentId, ReceiptId,
    ReceiptRegistrationStatus, RefundId, Settlement, TimeFilter,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReceiptType {
    Payment,
    Refund,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostReceiptItem {
    pub description: String,
    pub amount: MonetaryAmount,
    pub vat_code: u8,
    pub quantity: f64,
    pub measure: Option<String>,
    pub mark_quantity: Option<crate::model::MarkQuantity>,
    pub payment_subject: Option<String>,
    pub payment_mode: Option<String>,
    pub country_of_origin_code: Option<String>,
    pub customs_declaration_number: Option<String>,
    pub excise: Option<String>,
    pub product_code: Option<String>,
    pub planned_status: Option<u8>,
    pub mark_code_info: Option<serde_json::Value>,
    pub mark_mode: Option<String>,
    pub payment_subject_industry_details: Option<Vec<IndustryDetails>>,
    pub additional_payment_subject_props: Option<String>,
    pub supplier: Option<serde_json::Value>,
    pub agent_type: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateReceiptRequest {
    pub r#type: ReceiptType,
    pub payment_id: Option<PaymentId>,
    pub refund_id: Option<RefundId>,
    pub customer: crate::model::ReceiptCustomer,
    pub items: Vec<PostReceiptItem>,
    pub internet: Option<bool>,
    pub send: bool,
    pub tax_system_code: Option<u8>,
    pub timezone: Option<u8>,
    pub additional_user_props: Option<serde_json::Value>,
    pub receipt_industry_details: Option<Vec<IndustryDetails>>,
    pub receipt_operational_details: Option<OperationalDetails>,
    pub settlements: Vec<Settlement>,
    pub on_behalf_of: Option<crate::model::AccountId>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReceiptListQuery {
    #[serde(flatten)]
    pub time: TimeFilter,
    pub status: Option<ReceiptRegistrationStatus>,
    pub payment_id: Option<PaymentId>,
    pub refund_id: Option<RefundId>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Receipt {
    pub id: ReceiptId,
    pub r#type: ReceiptType,
    pub payment_id: Option<PaymentId>,
    pub refund_id: Option<RefundId>,
    pub status: ReceiptRegistrationStatus,
    pub fiscal_document_number: Option<String>,
    pub fiscal_storage_number: Option<String>,
    pub fiscal_attribute: Option<String>,
    pub registered_at: Option<DateTime<Utc>>,
    pub fiscal_provider_id: Option<String>,
    pub items: Vec<crate::model::ReceiptItem>,
    pub internet: Option<bool>,
    pub settlements: Option<Vec<Settlement>>,
    pub on_behalf_of: Option<crate::model::AccountId>,
    pub tax_system_code: Option<u8>,
    pub timezone: Option<u8>,
    pub receipt_industry_details: Option<Vec<IndustryDetails>>,
    pub receipt_operational_details: Option<OperationalDetails>,
}

impl YookassaClient {
    pub async fn create_receipt(
        &self,
        request: &CreateReceiptRequest,
        idempotence_key: Option<&str>,
    ) -> Result<Receipt, YookassaError> {
        self.post("receipts", request, idempotence_key).await
    }

    pub async fn list_receipts(
        &self,
        query: &ReceiptListQuery,
    ) -> Result<ListResponse<Receipt>, YookassaError> {
        self.get("receipts", Some(query)).await
    }

    pub async fn get_receipt(&self, receipt_id: &ReceiptId) -> Result<Receipt, YookassaError> {
        self.get(&format!("receipts/{}", receipt_id.0), Option::<&()>::None)
            .await
    }
}
