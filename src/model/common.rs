use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;
use std::collections::BTreeMap;

macro_rules! string_id {
    ($name:ident) => {
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct $name(pub String);

        impl From<String> for $name {
            fn from(value: String) -> Self {
                Self(value)
            }
        }

        impl From<&str> for $name {
            fn from(value: &str) -> Self {
                Self(value.to_string())
            }
        }

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                &self.0
            }
        }
    };
}

string_id!(AccountId);
string_id!(GatewayId);
string_id!(PaymentId);
string_id!(RefundId);
string_id!(ReceiptId);
string_id!(InvoiceId);
string_id!(PaymentMethodId);
string_id!(DealId);
string_id!(PayoutId);
string_id!(PersonalDataId);
string_id!(WebhookId);
string_id!(SbpBankId);

pub type Metadata = BTreeMap<String, Value>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CurrencyCode {
    Rub,
    Eur,
    Usd,
    Kzt,
    Byn,
    Uah,
    Uzs,
    Try,
    Inr,
    Mdl,
    Azn,
    Amd,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MonetaryAmount {
    #[serde(with = "rust_decimal::serde::str")]
    pub value: Decimal,
    pub currency: CurrencyCode,
}

impl MonetaryAmount {
    pub fn new(value: Decimal, currency: CurrencyCode) -> Self {
        Self { value, currency }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentStatus {
    Pending,
    WaitingForCapture,
    Succeeded,
    Canceled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReceiptRegistrationStatus {
    Pending,
    Succeeded,
    Canceled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodStatus {
    Pending,
    Active,
    Inactive,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodTypeCode {
    BankCard,
    Cash,
    Alfabank,
    Webmoney,
    Wechat,
    ApplePay,
    GooglePay,
    Qiwi,
    Installments,
    YooMoney,
    Sberbank,
    MobileBalance,
    B2bSberbank,
    TinkoffBank,
    Sbp,
    SberLoan,
    ElectronicCertificate,
    SberBnpl,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Locale {
    #[serde(rename = "ru_RU")]
    RuRu,
    #[serde(rename = "en_US")]
    EnUs,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BankCardType {
    MasterCard,
    Visa,
    Mir,
    UnionPay,
    Jcb,
    AmericanExpress,
    DinersClub,
    DiscoverCard,
    InstaPayment,
    InstaPaymentTm,
    Laser,
    Dankort,
    Solo,
    Switch,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VatDataType {
    Calculated,
    Untaxed,
    Mixed,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum B2bVatData {
    Calculated {
        amount: MonetaryAmount,
        rate: String,
    },
    Untaxed,
    Mixed {
        amount: MonetaryAmount,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Recipient {
    pub gateway_id: GatewayId,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RecipientInfo {
    pub account_id: AccountId,
    pub gateway_id: GatewayId,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TimeFilter {
    #[serde(rename = "created_at.gte")]
    pub created_at_gte: Option<DateTime<Utc>>,
    #[serde(rename = "created_at.gt")]
    pub created_at_gt: Option<DateTime<Utc>>,
    #[serde(rename = "created_at.lte")]
    pub created_at_lte: Option<DateTime<Utc>>,
    #[serde(rename = "created_at.lt")]
    pub created_at_lt: Option<DateTime<Utc>>,
    pub limit: Option<u8>,
    #[serde(rename = "cursor")]
    pub next_cursor: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListResponse<T> {
    pub r#type: String,
    pub items: Vec<T>,
    pub next_cursor: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CardData {
    pub first6: Option<String>,
    pub last4: String,
    pub expiry_year: String,
    pub expiry_month: String,
    pub card_type: BankCardType,
    pub card_product: Option<CardProduct>,
    pub issuer_country: Option<String>,
    pub issuer_name: Option<String>,
    pub source: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CardProduct {
    pub code: String,
    pub name: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CardRequestData {
    pub number: String,
    pub expiry_year: String,
    pub expiry_month: String,
    pub cardholder: Option<String>,
    pub csc: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MarkQuantity {
    pub numerator: u32,
    pub denominator: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IndustryDetails {
    pub federal_id: String,
    pub document_date: NaiveDate,
    pub document_number: String,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OperationalDetails {
    pub operation_id: u8,
    pub value: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Settlement {
    pub r#type: SettlementType,
    pub amount: MonetaryAmount,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SettlementType {
    Cashless,
    Prepayment,
    Postpayment,
    Consideration,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaymentDealInfo {
    pub id: DealId,
    pub settlements: Vec<Settlement>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransferData {
    pub account_id: AccountId,
    pub amount: MonetaryAmount,
    pub platform_fee_amount: Option<MonetaryAmount>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Transfer {
    pub account_id: AccountId,
    pub amount: MonetaryAmount,
    pub status: PaymentStatus,
    pub platform_fee_amount: Option<MonetaryAmount>,
    pub description: Option<String>,
    pub metadata: Option<Metadata>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthorizationDetails {
    pub rrn: Option<String>,
    pub auth_code: Option<String>,
    pub three_d_secure: ThreeDSecureDetails,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ThreeDSecureDetails {
    pub applied: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaymentCancellationDetails {
    pub party: PaymentCancellationParty,
    pub reason: PaymentCancellationReason,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentCancellationParty {
    YooMoney,
    PaymentNetwork,
    Merchant,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PaymentCancellationReason {
    #[serde(rename = "3d_secure_failed")]
    ThreeDSecureFailed,
    #[serde(rename = "call_issuer")]
    CallIssuer,
    #[serde(rename = "card_expired")]
    CardExpired,
    #[serde(rename = "payment_method_limit_exceeded")]
    PaymentMethodLimitExceeded,
    #[serde(rename = "payment_method_restricted")]
    PaymentMethodRestricted,
    #[serde(rename = "country_forbidden")]
    CountryForbidden,
    #[serde(rename = "general_decline")]
    GeneralDecline,
    #[serde(rename = "fraud_suspected")]
    FraudSuspected,
    #[serde(rename = "identification_required")]
    IdentificationRequired,
    #[serde(rename = "insufficient_funds")]
    InsufficientFunds,
    #[serde(rename = "invalid_card_number")]
    InvalidCardNumber,
    #[serde(rename = "invalid_csc")]
    InvalidCsc,
    #[serde(rename = "issuer_unavailable")]
    IssuerUnavailable,
    #[serde(rename = "canceled_by_merchant")]
    CanceledByMerchant,
    #[serde(rename = "permission_revoked")]
    PermissionRevoked,
    #[serde(rename = "internal_timeout")]
    InternalTimeout,
    #[serde(rename = "expired_on_confirmation")]
    ExpiredOnConfirmation,
    #[serde(rename = "expired_on_capture")]
    ExpiredOnCapture,
    #[serde(rename = "unsupported_mobile_operator")]
    UnsupportedMobileOperator,
    #[serde(rename = "deal_expired")]
    DealExpired,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn monetary_amount_serializes_decimal_as_string() {
        let amount = MonetaryAmount::new(Decimal::new(12345, 2), CurrencyCode::Rub);
        let json = serde_json::to_value(amount).expect("amount json");

        assert_eq!(json["value"], "123.45");
        assert_eq!(json["currency"], "RUB");
    }
}
