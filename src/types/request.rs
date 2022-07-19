use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegistrationRequest {
    pub address: Address,
    pub audit: Audit,
    pub credentials: Credentials,
    pub device: Device,
    pub email_address: String,
    pub first_name: String,
    pub last_name: String,
    pub opt_in_for_marketing: bool,
    pub policies: Policies,
    pub preferences: Vec<Preference>,
    pub subscriptions: Vec<Subscription>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    pub country: String,
    pub zip_code: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Audit {
    pub registration_channel: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Credentials {
    pub login_username: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Device {
    pub device_id: String,
    pub device_id_type: String,
    pub is_active: String,
    pub os: String,
    pub os_version: String,
    pub timezone: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Policies {
    pub acceptance_policies: AcceptancePolicies,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AcceptancePolicies {
    #[serde(rename = "1")]
    pub n1: bool,
    #[serde(rename = "4")]
    pub n4: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Preference {
    pub details: Details,
    pub preference_id: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Details {
    pub legacy_id: Option<String>,
    #[serde(rename = "MobileApp")]
    pub mobile_app: Option<String>,
    #[serde(rename = "Email")]
    pub email: Option<String>,
    pub enabled: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Subscription {
    pub opt_in_status: String,
    pub subscription_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivationRequest {
    pub activation_code: String,
    pub credentials: Credentials,
}
