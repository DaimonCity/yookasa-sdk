use crate::model::{
    B2bVatData, CardData, CardRequestData, GatewayId, Locale, PaymentMethodId, PaymentMethodStatus,
    PaymentMethodTypeCode,
};
use crate::{client::YookassaClient, error::YookassaError};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaymentMethodCommon {
    pub id: PaymentMethodId,
    pub saved: bool,
    pub status: PaymentMethodStatus,
    pub title: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum PaymentMethod {
    BankCard {
        #[serde(flatten)]
        common: PaymentMethodCommon,
        card: Option<CardData>,
    },
    Cash {
        #[serde(flatten)]
        common: PaymentMethodCommon,
    },
    Qiwi {
        #[serde(flatten)]
        common: PaymentMethodCommon,
    },
    Alfabank {
        #[serde(flatten)]
        common: PaymentMethodCommon,
        login: Option<String>,
    },
    Webmoney {
        #[serde(flatten)]
        common: PaymentMethodCommon,
    },
    ApplePay {
        #[serde(flatten)]
        common: PaymentMethodCommon,
    },
    GooglePay {
        #[serde(flatten)]
        common: PaymentMethodCommon,
    },
    YooMoney {
        #[serde(flatten)]
        common: PaymentMethodCommon,
        account_number: Option<String>,
    },
    Sberbank {
        #[serde(flatten)]
        common: PaymentMethodCommon,
        phone: Option<String>,
        card: Option<CardData>,
    },
    MobileBalance {
        #[serde(flatten)]
        common: PaymentMethodCommon,
    },
    Installments {
        #[serde(flatten)]
        common: PaymentMethodCommon,
    },
    B2bSberbank {
        #[serde(flatten)]
        common: PaymentMethodCommon,
        payment_purpose: String,
        vat_data: B2bVatData,
        payer_bank_details: Option<serde_json::Value>,
    },
    TinkoffBank {
        #[serde(flatten)]
        common: PaymentMethodCommon,
        card: Option<CardData>,
    },
    Wechat {
        #[serde(flatten)]
        common: PaymentMethodCommon,
    },
    Sbp {
        #[serde(flatten)]
        common: PaymentMethodCommon,
        sbp_operation_id: Option<String>,
        payer_bank_details: Option<serde_json::Value>,
    },
    SberLoan {
        #[serde(flatten)]
        common: PaymentMethodCommon,
        loan_option: Option<String>,
        discount_amount: Option<crate::model::MonetaryAmount>,
        suspended_until: Option<DateTime<Utc>>,
    },
    ElectronicCertificate {
        #[serde(flatten)]
        common: PaymentMethodCommon,
        card: Option<CardData>,
        electronic_certificate: Option<serde_json::Value>,
        articles: Option<Vec<serde_json::Value>>,
    },
    SberBnpl {
        #[serde(flatten)]
        common: PaymentMethodCommon,
    },
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SavePaymentMethodCommon {
    pub id: PaymentMethodId,
    pub saved: bool,
    pub status: PaymentMethodStatus,
    pub holder: SavePaymentMethodHolder,
    pub title: Option<String>,
    pub confirmation: Option<PaymentMethodsConfirmation>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SavePaymentMethodHolder {
    pub account_id: crate::model::AccountId,
    pub gateway_id: Option<GatewayId>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum SavePaymentMethod {
    BankCard {
        #[serde(flatten)]
        common: SavePaymentMethodCommon,
        card: Option<CardData>,
    },
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SavePaymentMethodRequest {
    #[serde(flatten)]
    pub data: SavePaymentMethodData,
    pub holder: Option<crate::model::Recipient>,
    pub client_ip: Option<String>,
    pub confirmation: Option<PaymentMethodsConfirmationData>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum SavePaymentMethodData {
    BankCard { card: Option<CardRequestData> },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum PaymentMethodsConfirmationData {
    Redirect {
        return_url: String,
        enforce: Option<bool>,
        locale: Option<Locale>,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum PaymentMethodsConfirmation {
    Redirect {
        confirmation_url: String,
        enforce: Option<bool>,
        return_url: Option<String>,
    },
}

impl PaymentMethod {
    pub fn kind(&self) -> PaymentMethodTypeCode {
        match self {
            Self::BankCard { .. } => PaymentMethodTypeCode::BankCard,
            Self::Cash { .. } => PaymentMethodTypeCode::Cash,
            Self::Qiwi { .. } => PaymentMethodTypeCode::Qiwi,
            Self::Alfabank { .. } => PaymentMethodTypeCode::Alfabank,
            Self::Webmoney { .. } => PaymentMethodTypeCode::Webmoney,
            Self::ApplePay { .. } => PaymentMethodTypeCode::ApplePay,
            Self::GooglePay { .. } => PaymentMethodTypeCode::GooglePay,
            Self::YooMoney { .. } => PaymentMethodTypeCode::YooMoney,
            Self::Sberbank { .. } => PaymentMethodTypeCode::Sberbank,
            Self::MobileBalance { .. } => PaymentMethodTypeCode::MobileBalance,
            Self::Installments { .. } => PaymentMethodTypeCode::Installments,
            Self::B2bSberbank { .. } => PaymentMethodTypeCode::B2bSberbank,
            Self::TinkoffBank { .. } => PaymentMethodTypeCode::TinkoffBank,
            Self::Wechat { .. } => PaymentMethodTypeCode::Wechat,
            Self::Sbp { .. } => PaymentMethodTypeCode::Sbp,
            Self::SberLoan { .. } => PaymentMethodTypeCode::SberLoan,
            Self::ElectronicCertificate { .. } => PaymentMethodTypeCode::ElectronicCertificate,
            Self::SberBnpl { .. } => PaymentMethodTypeCode::SberBnpl,
        }
    }
}

impl YookassaClient {
    pub async fn create_payment_method(
        &self,
        request: &SavePaymentMethodRequest,
        idempotence_key: Option<&str>,
    ) -> Result<SavePaymentMethod, YookassaError> {
        self.post("payment_methods", request, idempotence_key).await
    }

    pub async fn get_payment_method(
        &self,
        payment_method_id: &PaymentMethodId,
    ) -> Result<SavePaymentMethod, YookassaError> {
        self.get(
            &format!("payment_methods/{}", payment_method_id.0),
            Option::<&()>::None,
        )
        .await
    }
}
