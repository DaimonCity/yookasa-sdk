use crate::client::YookassaClient;
use crate::error::YookassaError;
use crate::model::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum PaymentMethodData {
    BankCard {
        card: Option<CardRequestData>,
    },
    Cash {
        phone: Option<String>,
    },
    Sberbank {
        phone: Option<String>,
    },
    TinkoffBank,
    YooMoney,
    MobileBalance {
        phone: String,
    },
    B2bSberbank {
        payment_purpose: String,
        vat_data: B2bVatData,
    },
    Sbp,
    SberLoan,
    ElectronicCertificate {
        card: Option<CardRequestData>,
        electronic_certificate: Option<serde_json::Value>,
        articles: Option<Vec<serde_json::Value>>,
    },
    SberBnpl {
        phone: Option<String>,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ConfirmationData {
    Redirect {
        return_url: String,
        enforce: Option<bool>,
        locale: Option<Locale>,
    },
    External {
        locale: Option<Locale>,
    },
    Qr {
        locale: Option<Locale>,
        return_url: Option<String>,
    },
    Embedded {
        locale: Option<Locale>,
    },
    MobileApplication {
        locale: Option<Locale>,
        return_url: String,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Confirmation {
    Redirect {
        confirmation_url: String,
        enforce: Option<bool>,
        return_url: Option<String>,
    },
    External,
    Qr {
        confirmation_data: String,
    },
    Embedded {
        confirmation_token: String,
    },
    MobileApplication {
        confirmation_url: String,
    },
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReceiptCustomer {
    pub full_name: Option<String>,
    pub inn: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReceiptItem {
    pub description: String,
    pub amount: MonetaryAmount,
    pub vat_code: u8,
    pub quantity: f64,
    pub measure: Option<String>,
    pub mark_quantity: Option<MarkQuantity>,
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
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReceiptData {
    pub customer: Option<ReceiptCustomer>,
    pub items: Vec<ReceiptItem>,
    pub internet: Option<bool>,
    pub tax_system_code: Option<u8>,
    pub timezone: Option<u8>,
    pub receipt_industry_details: Option<Vec<IndustryDetails>>,
    pub receipt_operational_details: Option<OperationalDetails>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreatePaymentRequest {
    pub amount: MonetaryAmount,
    pub description: Option<String>,
    pub receipt: Option<ReceiptData>,
    pub recipient: Option<Recipient>,
    pub payment_token: Option<String>,
    pub payment_method_id: Option<PaymentMethodId>,
    pub payment_method_data: Option<PaymentMethodData>,
    pub confirmation: Option<ConfirmationData>,
    pub save_payment_method: Option<bool>,
    pub capture: Option<bool>,
    pub client_ip: Option<String>,
    pub metadata: Option<Metadata>,
    pub airline: Option<serde_json::Value>,
    pub transfers: Option<Vec<TransferData>>,
    pub deal: Option<PaymentDealInfo>,
    pub merchant_customer_id: Option<String>,
    pub payment_order: Option<serde_json::Value>,
    pub receiver: Option<serde_json::Value>,
    pub statements: Option<Vec<serde_json::Value>>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CapturePaymentRequest {
    pub amount: Option<MonetaryAmount>,
    pub receipt: Option<ReceiptData>,
    pub airline: Option<serde_json::Value>,
    pub transfers: Option<Vec<TransferData>>,
    pub deal: Option<CapturePaymentDeal>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CapturePaymentDeal {
    pub settlements: Vec<Settlement>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaymentListQuery {
    #[serde(flatten)]
    pub time: TimeFilter,
    #[serde(rename = "captured_at.gte")]
    pub captured_at_gte: Option<DateTime<Utc>>,
    #[serde(rename = "captured_at.gt")]
    pub captured_at_gt: Option<DateTime<Utc>>,
    #[serde(rename = "captured_at.lte")]
    pub captured_at_lte: Option<DateTime<Utc>>,
    #[serde(rename = "captured_at.lt")]
    pub captured_at_lt: Option<DateTime<Utc>>,
    pub payment_method: Option<PaymentMethodTypeCode>,
    pub status: Option<PaymentStatus>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Payment {
    pub id: PaymentId,
    pub status: PaymentStatus,
    pub amount: MonetaryAmount,
    pub income_amount: Option<MonetaryAmount>,
    pub description: Option<String>,
    pub recipient: RecipientInfo,
    pub payment_method: Option<PaymentMethod>,
    pub captured_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub confirmation: Option<Confirmation>,
    pub test: bool,
    pub refunded_amount: Option<MonetaryAmount>,
    pub paid: bool,
    pub refundable: bool,
    pub receipt_registration: Option<ReceiptRegistrationStatus>,
    pub metadata: Option<Metadata>,
    pub cancellation_details: Option<PaymentCancellationDetails>,
    pub authorization_details: Option<AuthorizationDetails>,
    pub transfers: Option<Vec<Transfer>>,
    pub deal: Option<PaymentDealInfo>,
    pub merchant_customer_id: Option<String>,
    pub invoice_details: Option<PaymentInvoiceDetails>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaymentInvoiceDetails {
    pub id: InvoiceId,
}

impl YookassaClient {
    pub async fn create_payment(
        &self,
        request: &CreatePaymentRequest,
        idempotence_key: Option<&str>,
    ) -> Result<Payment, YookassaError> {
        self.post("payments", request, idempotence_key).await
    }

    pub async fn list_payments(
        &self,
        query: &PaymentListQuery,
    ) -> Result<ListResponse<Payment>, YookassaError> {
        self.get("payments", Some(query)).await
    }

    pub async fn get_payment(&self, payment_id: &PaymentId) -> Result<Payment, YookassaError> {
        self.get(&format!("payments/{}", payment_id.0), Option::<&()>::None)
            .await
    }

    pub async fn capture_payment(
        &self,
        payment_id: &PaymentId,
        request: &CapturePaymentRequest,
        idempotence_key: Option<&str>,
    ) -> Result<Payment, YookassaError> {
        self.post(
            &format!("payments/{}/capture", payment_id.0),
            request,
            idempotence_key,
        )
        .await
    }

    pub async fn cancel_payment(
        &self,
        payment_id: &PaymentId,
        idempotence_key: Option<&str>,
    ) -> Result<Payment, YookassaError> {
        self.post_without_body(
            &format!("payments/{}/cancel", payment_id.0),
            idempotence_key,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::Decimal;

    #[test]
    fn create_payment_uses_snake_case_confirmation_type() {
        let request = CreatePaymentRequest {
            amount: MonetaryAmount::new(Decimal::new(10000, 2), CurrencyCode::Rub),
            description: Some("Order #42".to_string()),
            receipt: None,
            recipient: None,
            payment_token: None,
            payment_method_id: None,
            payment_method_data: None,
            confirmation: Some(ConfirmationData::Redirect {
                return_url: "https://example.com/return".to_string(),
                enforce: Some(true),
                locale: Some(Locale::RuRu),
            }),
            save_payment_method: Some(true),
            capture: Some(true),
            client_ip: None,
            metadata: None,
            airline: None,
            transfers: None,
            deal: None,
            merchant_customer_id: None,
            payment_order: None,
            receiver: None,
            statements: None,
        };

        let json = serde_json::to_value(request).expect("request json");

        assert_eq!(json["confirmation"]["type"], "redirect");
        assert_eq!(json["confirmation"]["locale"], "ru_RU");
        assert_eq!(json["amount"]["value"], "100.00");
    }
}
