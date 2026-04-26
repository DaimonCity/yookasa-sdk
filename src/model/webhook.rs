use crate::client::YookassaClient;
use crate::error::YookassaError;
use crate::model::{AccountId, ListResponse, MonetaryAmount, PaymentMethodTypeCode, WebhookId};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NotificationEventType {
    #[serde(rename = "payment.waiting_for_capture")]
    PaymentWaitingForCapture,
    #[serde(rename = "payment.succeeded")]
    PaymentSucceeded,
    #[serde(rename = "payment.canceled")]
    PaymentCanceled,
    #[serde(rename = "refund.succeeded")]
    RefundSucceeded,
    #[serde(rename = "payment_method.active")]
    PaymentMethodActive,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateWebhookRequest {
    pub event: NotificationEventType,
    pub url: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Webhook {
    pub id: WebhookId,
    pub event: NotificationEventType,
    pub url: String,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SettingsQuery {
    pub on_behalf_of: Option<AccountId>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ShopStatus {
    Enabled,
    Disabled,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FiscalizationData {
    pub enabled: Option<bool>,
    pub provider: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PayoutMethodType {
    BankCard,
    YooMoney,
    Sbp,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Me {
    pub account_id: AccountId,
    pub status: ShopStatus,
    pub test: bool,
    pub fiscalization: Option<FiscalizationData>,
    pub fiscalization_enabled: Option<bool>,
    pub payment_methods: Option<Vec<PaymentMethodTypeCode>>,
    pub itn: Option<String>,
    pub payout_methods: Option<Vec<PayoutMethodType>>,
    pub name: Option<String>,
    pub payout_balance: Option<MonetaryAmount>,
}

impl YookassaClient {
    fn ensure_oauth(&self) -> Result<(), YookassaError> {
        if self.uses_oauth() {
            Ok(())
        } else {
            Err(YookassaError::OAuthRequired)
        }
    }

    pub async fn create_webhook(
        &self,
        request: &CreateWebhookRequest,
        idempotence_key: Option<&str>,
    ) -> Result<Webhook, YookassaError> {
        self.ensure_oauth()?;
        self.post("webhooks", request, idempotence_key).await
    }

    pub async fn list_webhooks(&self) -> Result<ListResponse<Webhook>, YookassaError> {
        self.ensure_oauth()?;
        self.get("webhooks", Option::<&()>::None).await
    }

    pub async fn delete_webhook(
        &self,
        webhook_id: &WebhookId,
        idempotence_key: Option<&str>,
    ) -> Result<(), YookassaError> {
        self.ensure_oauth()?;
        self.delete_empty(&format!("webhooks/{}", webhook_id.0), idempotence_key)
            .await
    }

    pub async fn get_me(&self, query: Option<&SettingsQuery>) -> Result<Me, YookassaError> {
        self.get("me", query).await
    }
}
