use crate::client::YookassaClient;
use crate::error::YookassaError;
use crate::model::{
    InvoiceId, Locale, Metadata, MonetaryAmount, PaymentId, PaymentStatus, ReceiptData, Recipient,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InvoicePaymentData {
    pub amount: MonetaryAmount,
    pub receipt: Option<ReceiptData>,
    pub recipient: Option<Recipient>,
    pub save_payment_method: Option<bool>,
    pub capture: Option<bool>,
    pub client_ip: Option<String>,
    pub description: Option<String>,
    pub metadata: Option<Metadata>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LineItem {
    pub description: String,
    pub price: MonetaryAmount,
    pub discount_price: Option<MonetaryAmount>,
    pub quantity: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum DeliveryMethodData {
    #[serde(rename = "self")]
    SelfDelivery,
    Sms {
        phone: String,
    },
    Email {
        email: String,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum DeliveryMethod {
    #[serde(rename = "self")]
    SelfDelivery {
        url: Option<String>,
    },
    Sms,
    Email,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateInvoiceRequest {
    pub payment_data: InvoicePaymentData,
    pub cart: Vec<LineItem>,
    pub delivery_method_data: Option<DeliveryMethodData>,
    pub expires_at: DateTime<Utc>,
    pub locale: Option<Locale>,
    pub description: Option<String>,
    pub metadata: Option<Metadata>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InvoiceStatus {
    Pending,
    Succeeded,
    Canceled,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaymentDetails {
    pub id: PaymentId,
    pub status: PaymentStatus,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InvoiceCancellationDetails {
    pub party: String,
    pub reason: String,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Invoice {
    pub id: InvoiceId,
    pub status: InvoiceStatus,
    pub cart: Vec<LineItem>,
    pub delivery_method: Option<DeliveryMethod>,
    pub payment_details: Option<PaymentDetails>,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub description: Option<String>,
    pub cancellation_details: Option<InvoiceCancellationDetails>,
    pub metadata: Option<Metadata>,
}

impl YookassaClient {
    pub async fn create_invoice(
        &self,
        request: &CreateInvoiceRequest,
        idempotence_key: Option<&str>,
    ) -> Result<Invoice, YookassaError> {
        self.post("invoices", request, idempotence_key).await
    }

    pub async fn get_invoice(&self, invoice_id: &InvoiceId) -> Result<Invoice, YookassaError> {
        self.get(&format!("invoices/{}", invoice_id.0), Option::<&()>::None)
            .await
    }
}
