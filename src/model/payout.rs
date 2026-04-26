use crate::client::YookassaClient;
use crate::error::YookassaError;
use crate::model::{
    CardData, DealId, ListResponse, Metadata, MonetaryAmount, PaymentMethodId, PayoutId,
    PersonalDataId, SbpBankId, TimeFilter,
};
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PayoutStatus {
    Pending,
    Succeeded,
    Canceled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PayoutDestinationType {
    YooMoney,
    BankCard,
    Sbp,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum PayoutDestinationData {
    YooMoney { account_number: String },
    BankCard { card: PayoutCardRequest },
    Sbp { phone: String, bank_id: SbpBankId },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PayoutCardRequest {
    pub number: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum PayoutDestination {
    YooMoney {
        account_number: String,
    },
    BankCard {
        card: CardData,
    },
    Sbp {
        phone: String,
        bank_id: SbpBankId,
        sbp_operation_id: Option<String>,
        recipient_checked: bool,
    },
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PayoutDealInfo {
    pub id: DealId,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PayoutPersonalDataRef {
    pub id: PersonalDataId,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreatePayoutRequest {
    pub amount: MonetaryAmount,
    pub payout_destination_data: Option<PayoutDestinationData>,
    pub payout_token: Option<String>,
    pub payment_method_id: Option<PaymentMethodId>,
    pub description: Option<String>,
    pub deal: Option<PayoutDealInfo>,
    pub personal_data: Option<Vec<PayoutPersonalDataRef>>,
    pub metadata: Option<Metadata>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PayoutListQuery {
    #[serde(flatten)]
    pub time: TimeFilter,
    #[serde(rename = "succeeded_at.gte")]
    pub succeeded_at_gte: Option<DateTime<Utc>>,
    #[serde(rename = "succeeded_at.gt")]
    pub succeeded_at_gt: Option<DateTime<Utc>>,
    #[serde(rename = "succeeded_at.lte")]
    pub succeeded_at_lte: Option<DateTime<Utc>>,
    #[serde(rename = "succeeded_at.lt")]
    pub succeeded_at_lt: Option<DateTime<Utc>>,
    #[serde(rename = "payout_destination.type")]
    pub payout_destination_type: Option<PayoutDestinationType>,
    pub status: Option<PayoutStatus>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PayoutSearchQuery {
    #[serde(flatten)]
    pub time: TimeFilter,
    pub metadata: Option<BTreeMap<String, String>>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Payout {
    pub id: PayoutId,
    pub amount: MonetaryAmount,
    pub status: PayoutStatus,
    pub payout_destination: PayoutDestination,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub succeeded_at: Option<DateTime<Utc>>,
    pub deal: Option<PayoutDealInfo>,
    pub self_employed: Option<serde_json::Value>,
    pub receipt: Option<serde_json::Value>,
    pub cancellation_details: Option<serde_json::Value>,
    pub metadata: Option<Metadata>,
    pub test: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SbpParticipantBank {
    pub bank_id: SbpBankId,
    pub name: String,
    pub bic: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PersonalDataType {
    PayoutStatementRecipient,
    SbpPayoutRecipient,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PersonalDataStatus {
    WaitingForOperation,
    Active,
    Canceled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PersonalDataCancellationParty {
    YooMoney,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PersonalDataCancellationReason {
    ExpiredByTimeout,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PersonalDataCancellationDetails {
    pub party: PersonalDataCancellationParty,
    pub reason: PersonalDataCancellationReason,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum PersonalDataRequest {
    SbpPayoutRecipient {
        metadata: Option<Metadata>,
        last_name: String,
        first_name: String,
        middle_name: Option<String>,
    },
    PayoutStatementRecipient {
        metadata: Option<Metadata>,
        last_name: String,
        first_name: String,
        middle_name: Option<String>,
        birthdate: NaiveDate,
    },
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PersonalData {
    pub id: PersonalDataId,
    pub r#type: PersonalDataType,
    pub status: PersonalDataStatus,
    pub cancellation_details: Option<PersonalDataCancellationDetails>,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub metadata: Option<Metadata>,
}

impl YookassaClient {
    pub async fn create_payout(
        &self,
        request: &CreatePayoutRequest,
        idempotence_key: Option<&str>,
    ) -> Result<Payout, YookassaError> {
        self.post("payouts", request, idempotence_key).await
    }

    pub async fn list_payouts(
        &self,
        query: &PayoutListQuery,
    ) -> Result<ListResponse<Payout>, YookassaError> {
        self.get("payouts", Some(query)).await
    }

    pub async fn search_payouts(
        &self,
        query: &PayoutSearchQuery,
    ) -> Result<ListResponse<Payout>, YookassaError> {
        self.get("payouts/search", Some(query)).await
    }

    pub async fn get_payout(&self, payout_id: &PayoutId) -> Result<Payout, YookassaError> {
        self.get(&format!("payouts/{}", payout_id.0), Option::<&()>::None)
            .await
    }

    pub async fn list_sbp_banks(&self) -> Result<ListResponse<SbpParticipantBank>, YookassaError> {
        self.get("sbp_banks", Option::<&()>::None).await
    }

    pub async fn create_personal_data(
        &self,
        request: &PersonalDataRequest,
        idempotence_key: Option<&str>,
    ) -> Result<PersonalData, YookassaError> {
        self.post("personal_data", request, idempotence_key).await
    }

    pub async fn get_personal_data(
        &self,
        personal_data_id: &PersonalDataId,
    ) -> Result<PersonalData, YookassaError> {
        self.get(
            &format!("personal_data/{}", personal_data_id.0),
            Option::<&()>::None,
        )
        .await
    }
}
