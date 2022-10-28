use std::fmt::Debug;

use http::HeaderMap;
use http::StatusCode;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

use crate::Error;

pub struct ClientResponse<T> {
    pub status: StatusCode,
    pub headers: HeaderMap,
    pub body: T,
}

impl<T: Debug> Debug for ClientResponse<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ClientResponse")
            .field("status", &self.status)
            .field("body", &self.body)
            .finish()
    }
}

impl<'a, T> ClientResponse<T>
where
    T: for<'de> serde::Deserialize<'de> + Debug,
{
    pub async fn from_response(resp: reqwest::Response) -> Result<Self, Error> {
        Ok(Self {
            status: resp.status(),
            headers: resp.headers().clone(),
            body: resp.json::<T>().await?,
        })
    }
}

#[derive(serde::Deserialize, std::fmt::Debug)]
pub struct Token {
    pub token: String,
    pub expires: u32,
}

#[derive(serde::Deserialize, std::fmt::Debug)]
pub struct TokenResponse {
    pub status: Status,
    pub response: Token,
}

#[derive(Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessTokenResponse {
    pub access_token: String,
    pub refresh_token: String,
}

impl Debug for AccessTokenResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AccessTokenResponse")
            .field("access_token", &"[redacted]")
            .field("refresh_token", &"[redacted]")
            .finish()
    }
}

#[derive(serde::Deserialize, std::fmt::Debug)]
pub struct LoginResponse {
    pub status: Status,
    pub response: AccessTokenResponse,
}

#[derive(serde::Deserialize, std::fmt::Debug)]
pub struct RegistrationResponse {
    pub status: Status,
    pub response: AccessTokenResponse,
}

#[derive(serde::Deserialize, std::fmt::Debug)]
pub struct ActivationResponse {
    pub status: Status,
    pub response: Option<AccessTokenResponse>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OfferResponse {
    pub status: Status,
    pub response: Option<OfferList>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Status {
    pub code: Value,
    pub type_field: Option<String>,
    #[serde(rename = "correlationID")]
    pub correlation_id: Option<String>,
    pub message: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OfferList {
    pub offers: Vec<Offer>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Offer {
    pub offer_id: i64,
    pub offer_proposition_id: i64,
    pub offer_type: i64,
    pub local_valid_from: String,
    pub local_valid_to: String,
    #[serde(rename = "validFromUTC")]
    pub valid_from_utc: String,
    #[serde(rename = "validToUTC")]
    pub valid_to_utc: String,
    pub name: String,
    pub short_description: String,
    pub long_description: String,
    pub image_base_name: String,
    pub image_base_language: Option<String>,
    pub redemption_mode: i64,
    pub is_archived: bool,
    #[serde(rename = "isSLPOffer")]
    pub is_slpoffer: bool,
    pub is_locked: bool,
    pub is_redeemed: bool,
    pub offer_bucket: String,
    pub punch_info: PunchInfo,
    pub recurring_info: Option<RecurringInfo>,
    pub conditions: Conditions,
    pub color_coding_info: i64,
    pub isvalid_total_order: bool,
    #[serde(rename = "CreationDateUtc")]
    pub creation_date_utc: String,
    #[serde(rename = "extendToEOD")]
    pub extend_to_eod: bool,
    pub is_dynamic_expiration: bool,
    pub daypart_filters: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PunchInfo {
    pub total_punch: i64,
    pub current_punch: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecurringInfo {
    pub total_redemption_quantity: Option<i64>,
    pub current_day_redemption_quantity: Option<i64>,
    pub current_week_redemption_quantity: Option<i64>,
    pub current_month_redemption_quantity: Option<i64>,
    pub max_redemption_quantity: Option<i64>,
    pub max_redemption_quantity_per_day: Option<i64>,
    pub max_redemption_quantity_per_week: Option<i64>,
    pub max_redemption_quantity_per_month: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Conditions {
    pub day_of_week_conditions: Vec<String>,
    pub date_conditions: Vec<Value>,
    pub sale_amount_conditions: Vec<SaleAmountCondition>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaleAmountCondition {
    pub include_eligible: bool,
    pub minimum: i64,
    pub pre_tax_validation: bool,
    pub include_non_product: bool,
    pub exclude_codes: Option<String>,
    pub include_gift_coupons: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RestaurantLocationResponse {
    pub status: Status,
    pub response: Option<RestaurantLocationList>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RestaurantLocationList {
    pub restaurants: Vec<Restaurant>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Restaurant {
    pub restaurant_status: String,
    pub facilities: Vec<String>,
    pub address: Address,
    pub mc_deliveries: McDeliveries,
    pub location: Location,
    pub name: String,
    pub national_store_number: i64,
    pub status: i64,
    pub time_zone: String,
    pub week_opening_hours: Vec<WeekOpeningHour>,
    pub phone_number: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    pub address_line1: String,
    pub city_town: String,
    pub country: String,
    pub postal_zip: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct McDeliveries {
    pub mc_delivery: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeekOpeningHour {
    pub services: Vec<Service>,
    pub day_of_week_id: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Service {
    pub end_time: String,
    pub is_open: bool,
    pub service_name: String,
    pub start_time: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OfferDetailsResponse {
    pub status: Status,
    pub response: Option<OfferDetails>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OfferDetails {
    pub order_discount_type: i64,
    pub offer_proposition_id: i64,
    pub offer_type: i64,
    pub offer_bucket: String,
    pub is_locked: bool,
    pub isvalid_total_order: bool,
    #[serde(rename = "isSLPOffer")]
    pub is_slpoffer: bool,
    pub color_coding_info: i64,
    pub local_valid_from: String,
    pub local_valid_to: String,
    #[serde(rename = "validFromUTC")]
    pub valid_from_utc: String,
    #[serde(rename = "validToUTC")]
    pub valid_to_utc: String,
    pub name: String,
    pub short_description: String,
    pub long_description: String,
    pub image_base_name: String,
    pub image_base_language: String,
    pub redemption_mode: i64,
    pub is_expired: bool,
    pub product_sets: Vec<ProductSet>,
    pub restaurants: Vec<Value>,
    pub frequency_offer_info: FrequencyOfferInfo,
    pub recurring_info: RecurringInfo,
    pub conditions: Conditions,
    pub is_dynamic_expiration: bool,
    #[serde(rename = "exclusiveTOD")]
    pub exclusive_tod: bool,
    pub daypart_filters: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductSet {
    pub alias: String,
    pub quantity: i64,
    pub min_quantity: Option<i64>,
    pub products: Vec<String>,
    pub action: Action,
    pub swap_mapping: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Action {
    #[serde(rename = "type")]
    pub type_field: i64,
    pub discount_type: i64,
    pub value: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FrequencyOfferInfo {
    pub total_punch: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OfferDealStackResponse {
    pub status: Status,
    pub response: Option<OfferDealStack>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OfferDealStack {
    pub random_code: String,
    pub bar_code_content: String,
    pub expiration_time: String,
    pub deal_stack: Option<Vec<DealStack>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DealStack {
    pub offer_id: i64,
    pub offer_proposition_id: String,
    pub state: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginRefreshResponse {
    pub response: Option<AccessTokenResponse>,
    pub status: Status,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomerPointResponse {
    pub status: Status,
    pub response: PointInformationResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PointInformationResponse {
    pub total_points: i64,
    pub life_time_points: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CatalogResponse {
    #[serde(rename = "Market")]
    pub market: Market,
    #[serde(rename = "Store")]
    pub store: Vec<Store>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Market {
    #[serde(rename = "StaticDataVersion")]
    pub static_data_version: Value,
    #[serde(rename = "StaticData")]
    pub static_data: Value,
    #[serde(rename = "DisplayCategoryVersion")]
    pub display_category_version: Value,
    #[serde(rename = "DisplayCategory")]
    pub display_category: Value,
    #[serde(rename = "FacilityVersion")]
    pub facility_version: Value,
    #[serde(rename = "Facilities")]
    pub facilities: Value,
    #[serde(rename = "NamesVersion")]
    pub names_version: Value,
    #[serde(rename = "Names")]
    pub names: Value,
    #[serde(rename = "RestaurantsVersion")]
    pub restaurants_version: Value,
    #[serde(rename = "Restaurants")]
    pub restaurants: Value,
    #[serde(rename = "RecipeVersion")]
    pub recipe_version: Value,
    #[serde(rename = "Recipes")]
    pub recipes: Value,
    #[serde(rename = "LanguageVersion")]
    pub language_version: Value,
    #[serde(rename = "Languages")]
    pub languages: Value,
    #[serde(rename = "PaymentMethodsVersion")]
    pub payment_methods_version: Value,
    #[serde(rename = "PaymentMethods")]
    pub payment_methods: Value,
    #[serde(rename = "FeedbackTypeNamesVersion")]
    pub feedback_type_names_version: Value,
    #[serde(rename = "FeedbackTypeNames")]
    pub feedback_type_names: Value,
    #[serde(rename = "TenderTypeVersion")]
    pub tender_type_version: Value,
    #[serde(rename = "TenderTypes")]
    pub tender_types: Value,
    #[serde(rename = "PromotionVersion")]
    pub promotion_version: Value,
    #[serde(rename = "Promotions")]
    pub promotions: Value,
    #[serde(rename = "MenuTypeVersion")]
    pub menu_type_version: Value,
    #[serde(rename = "MenuType")]
    pub menu_type: Value,
    #[serde(rename = "SocialNetworkVersion")]
    pub social_network_version: Value,
    #[serde(rename = "SocialNetwork")]
    pub social_network: Value,
    #[serde(rename = "Opt-InsVersion")]
    pub opt_ins_version: Value,
    #[serde(rename = "Opt-Ins")]
    pub opt_ins: Value,
    #[serde(rename = "CustomerEnumsVersion")]
    pub customer_enums_version: Value,
    #[serde(rename = "CustomerEnums")]
    pub customer_enums: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Store {
    #[serde(rename = "Store")]
    pub store: String,
    #[serde(rename = "RestaurantDataVersion")]
    pub restaurant_data_version: Value,
    #[serde(rename = "RestaurantData")]
    pub restaurant_data: Value,
    #[serde(rename = "PromotionVersion")]
    pub promotion_version: String,
    #[serde(rename = "Promotions")]
    pub promotions: Vec<Value>,
    #[serde(rename = "ProductVersion")]
    pub product_version: String,
    #[serde(rename = "Products")]
    pub products: Vec<Product>,
    #[serde(rename = "ProductPriceVersion")]
    pub product_price_version: String,
    #[serde(rename = "ProductPrice")]
    pub product_price: Vec<ProductPrice>,
    #[serde(rename = "RecipePriceVersion")]
    pub recipe_price_version: Value,
    #[serde(rename = "RecipePrice")]
    pub recipe_price: Value,
    #[serde(rename = "AvailabilityVersion")]
    pub availability_version: String,
    #[serde(rename = "Availability")]
    pub availability: Vec<Availability>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Product {
    #[serde(rename = "Nutrition")]
    pub nutrition: Option<Nutrition>,
    #[serde(rename = "Categories")]
    #[serde(default)]
    pub categories: Vec<Category>,
    #[serde(rename = "Dimensions")]
    pub dimensions: Vec<Dimension>,
    #[serde(rename = "StaticData")]
    pub static_data: Vec<Value>,
    #[serde(rename = "TimeRestriction")]
    #[serde(default)]
    pub time_restriction: Vec<TimeRestriction>,
    #[serde(rename = "IsPromotional")]
    pub is_promotional: bool,
    #[serde(rename = "DisplayImageName")]
    pub display_image_name: String,
    #[serde(rename = "IsPromotionalChoice")]
    pub is_promotional_choice: bool,
    #[serde(rename = "PromotionalLabel")]
    pub promotional_label: String,
    #[serde(rename = "PromotionStartDate")]
    pub promotion_start_date: String,
    #[serde(rename = "PromotionEndDate")]
    pub promotion_end_date: String,
    #[serde(rename = "PromotionRestriction")]
    pub promotion_restriction: Value,
    #[serde(rename = "PromotionsAssociated")]
    pub promotions_associated: Value,
    #[serde(rename = "ProductCode")]
    pub product_code: i64,
    #[serde(rename = "FamilyGroupID")]
    pub family_group_id: i64,
    #[serde(rename = "RecipeID")]
    pub recipe_id: i64,
    #[serde(rename = "MenuTypeID")]
    pub menu_type_id: i64,
    #[serde(rename = "IsMcCafe")]
    pub is_mc_cafe: bool,
    #[serde(rename = "IsSalable")]
    pub is_salable: bool,
    #[serde(rename = "MaxChoiceOptionsMOT")]
    pub max_choice_options_mot: i64,
    #[serde(rename = "AcceptsLight")]
    pub accepts_light: bool,
    #[serde(rename = "AcceptsOnly")]
    pub accepts_only: bool,
    #[serde(rename = "ProductType")]
    pub product_type: i64,
    #[serde(rename = "ProductUnit")]
    pub product_unit: Option<String>,
    #[serde(rename = "MaxQttyAllowedPerOrder")]
    pub max_qtty_allowed_per_order: Option<i64>,
    #[serde(rename = "POD")]
    #[serde(default)]
    pub pod: Vec<Pod>,
    #[serde(rename = "ExtendedMenuTypeID")]
    #[serde(default)]
    pub extended_menu_type_id: Vec<i64>,
    #[serde(rename = "Recipe")]
    pub recipe: Recipe,
    #[serde(rename = "Names")]
    pub names: Names,
    #[serde(rename = "NutritionPrimaryProductCode")]
    pub nutrition_primary_product_code: Value,
    #[serde(rename = "SmartRouting")]
    pub smart_routing: Option<SmartRouting>,
    #[serde(rename = "MaxExtraIngredientsQuantity")]
    pub max_extra_ingredients_quantity: i64,
    #[serde(rename = "VolumePrices")]
    pub volume_prices: Value,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(rename = "DepositCode")]
    pub deposit_code: Value,
    #[serde(rename = "SugarLevyAmount")]
    pub sugar_levy_amount: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Nutrition {
    #[serde(rename = "Energy")]
    pub energy: i64,
    #[serde(rename = "Name")]
    pub name: Value,
    #[serde(rename = "Serving")]
    pub serving: Value,
    #[serde(rename = "Caloriesfromfat")]
    pub caloriesfromfat: Value,
    #[serde(rename = "Totalfat")]
    pub totalfat: Value,
    #[serde(rename = "TotalfatDV")]
    pub totalfat_dv: Value,
    #[serde(rename = "Saturatedfat")]
    pub saturatedfat: Value,
    #[serde(rename = "SaturatedfatDV")]
    pub saturatedfat_dv: Value,
    #[serde(rename = "Transfat")]
    pub transfat: Value,
    #[serde(rename = "Cholesterol")]
    pub cholesterol: Value,
    #[serde(rename = "CholesterolDV")]
    pub cholesterol_dv: Value,
    #[serde(rename = "Sodium")]
    pub sodium: Value,
    #[serde(rename = "SodiumDV")]
    pub sodium_dv: Value,
    #[serde(rename = "Carbohydrates")]
    pub carbohydrates: Value,
    #[serde(rename = "CarbohydratesDV")]
    pub carbohydrates_dv: Value,
    #[serde(rename = "Dietaryfiber")]
    pub dietaryfiber: Value,
    #[serde(rename = "DietaryfiberDV")]
    pub dietaryfiber_dv: Value,
    #[serde(rename = "Sugars")]
    pub sugars: Value,
    #[serde(rename = "Protein")]
    pub protein: Value,
    #[serde(rename = "ProteinDV")]
    pub protein_dv: Value,
    #[serde(rename = "Vitaminc")]
    pub vitaminc: Value,
    #[serde(rename = "Vitamina")]
    pub vitamina: Value,
    #[serde(rename = "Calcium")]
    pub calcium: Value,
    #[serde(rename = "Iron")]
    pub iron: Value,
    #[serde(rename = "Ingredients")]
    pub ingredients: Value,
    #[serde(rename = "Allergenes")]
    pub allergenes: Value,
    #[serde(rename = "SpecialInfo")]
    pub special_info: Value,
    #[serde(rename = "KCal")]
    pub kcal: Value,
    #[serde(rename = "ExcludedInAccount")]
    pub excluded_in_account: Value,
    #[serde(rename = "SelfPour")]
    pub self_pour: Value,
    #[serde(rename = "MinBeverageSelfPour")]
    pub min_beverage_self_pour: Value,
    #[serde(rename = "MaxBeverageSelfPour")]
    pub max_beverage_self_pour: Value,
    #[serde(rename = "MinBeverageSelfPourKCal")]
    pub min_beverage_self_pour_kcal: Value,
    #[serde(rename = "MaxBeverageSelfPourKCal")]
    pub max_beverage_self_pour_kcal: Value,
    #[serde(rename = "SelfPourProducts")]
    pub self_pour_products: Value,
    #[serde(rename = "PortionExtraEnergy")]
    pub portion_extra_energy: Value,
    #[serde(rename = "PortionExtraEnergyKCal")]
    pub portion_extra_energy_kcal: Value,
    #[serde(rename = "PortionLightEnergy")]
    pub portion_light_energy: Value,
    #[serde(rename = "PortionLightEnergyKCal")]
    pub portion_light_energy_kcal: Value,
    #[serde(rename = "MinEnergy")]
    pub min_energy: Value,
    #[serde(rename = "MaxEnergy")]
    pub max_energy: Value,
    #[serde(rename = "Suffix")]
    pub suffix: Value,
    #[serde(rename = "DisclaimerIDs")]
    pub disclaimer_ids: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Category {
    #[serde(rename = "DisplayCategoryID")]
    pub display_category_id: i64,
    #[serde(rename = "DisplayOrder")]
    pub display_order: i64,
    #[serde(rename = "DisplaySizeSelection")]
    pub display_size_selection: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dimension {
    #[serde(rename = "SizeCodeID")]
    pub size_code_id: i64,
    #[serde(rename = "ProductCode")]
    pub product_code: i64,
    #[serde(rename = "ShowSizeToCustomer")]
    pub show_size_to_customer: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeRestriction {
    #[serde(rename = "FromTime")]
    pub from_time: String,
    #[serde(rename = "ToTime")]
    pub to_time: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pod {
    #[serde(rename = "SaleTypeID")]
    pub sale_type_id: i64,
    #[serde(rename = "TypeName")]
    pub type_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Recipe {
    #[serde(rename = "RecipeID")]
    pub recipe_id: i64,
    #[serde(rename = "IsValid")]
    pub is_valid: bool,
    #[serde(rename = "IsCustomerFriendly")]
    pub is_customer_friendly: bool,
    #[serde(rename = "DefaultSolution")]
    pub default_solution: Value,
    #[serde(rename = "Ingredients")]
    pub ingredients: Vec<Ingredient>,
    #[serde(rename = "Extras")]
    pub extras: Vec<Extra>,
    #[serde(rename = "Choices")]
    pub choices: Vec<Choice>,
    #[serde(rename = "Comments")]
    pub comments: Vec<Comment>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ingredient {
    #[serde(rename = "IsCustomerFriendly")]
    pub is_customer_friendly: bool,
    #[serde(rename = "MinQuantity")]
    pub min_quantity: i64,
    #[serde(rename = "DefaultQuantity")]
    pub default_quantity: i64,
    #[serde(rename = "MaxQuantity")]
    pub max_quantity: i64,
    #[serde(rename = "RefundTreshold")]
    pub refund_treshold: i64,
    #[serde(rename = "ChargeTreshold")]
    pub charge_treshold: i64,
    #[serde(rename = "CostInclusive")]
    pub cost_inclusive: bool,
    #[serde(rename = "ProductCode")]
    pub product_code: i64,
    #[serde(rename = "DefaultSolution")]
    pub default_solution: Value,
    #[serde(rename = "ReferencePriceProductCode")]
    pub reference_price_product_code: Value,
    #[serde(rename = "CytIngredientGroup")]
    pub cyt_ingredient_group: Option<String>,
    #[serde(rename = "CytIngredientType")]
    pub cyt_ingredient_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Extra {
    #[serde(rename = "IsCustomerFriendly")]
    pub is_customer_friendly: bool,
    #[serde(rename = "MinQuantity")]
    pub min_quantity: i64,
    #[serde(rename = "DefaultQuantity")]
    pub default_quantity: i64,
    #[serde(rename = "MaxQuantity")]
    pub max_quantity: i64,
    #[serde(rename = "RefundTreshold")]
    pub refund_treshold: i64,
    #[serde(rename = "ChargeTreshold")]
    pub charge_treshold: i64,
    #[serde(rename = "CostInclusive")]
    pub cost_inclusive: bool,
    #[serde(rename = "ProductCode")]
    pub product_code: i64,
    #[serde(rename = "DefaultSolution")]
    pub default_solution: Value,
    #[serde(rename = "ReferencePriceProductCode")]
    pub reference_price_product_code: Value,
    #[serde(rename = "CytIngredientGroup")]
    pub cyt_ingredient_group: Value,
    #[serde(rename = "CytIngredientType")]
    pub cyt_ingredient_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Choice {
    #[serde(rename = "IsCustomerFriendly")]
    pub is_customer_friendly: bool,
    #[serde(rename = "MinQuantity")]
    pub min_quantity: i64,
    #[serde(rename = "DefaultQuantity")]
    pub default_quantity: i64,
    #[serde(rename = "MaxQuantity")]
    pub max_quantity: i64,
    #[serde(rename = "RefundTreshold")]
    pub refund_treshold: i64,
    #[serde(rename = "ChargeTreshold")]
    pub charge_treshold: i64,
    #[serde(rename = "CostInclusive")]
    pub cost_inclusive: bool,
    #[serde(rename = "ProductCode")]
    pub product_code: i64,
    #[serde(rename = "DefaultSolution")]
    pub default_solution: Option<i64>,
    #[serde(rename = "ReferencePriceProductCode")]
    pub reference_price_product_code: Option<i64>,
    #[serde(rename = "CytIngredientGroup")]
    pub cyt_ingredient_group: Value,
    #[serde(rename = "CytIngredientType")]
    pub cyt_ingredient_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Comment {
    #[serde(rename = "IsCustomerFriendly")]
    pub is_customer_friendly: bool,
    #[serde(rename = "MinQuantity")]
    pub min_quantity: i64,
    #[serde(rename = "DefaultQuantity")]
    pub default_quantity: i64,
    #[serde(rename = "MaxQuantity")]
    pub max_quantity: i64,
    #[serde(rename = "RefundTreshold")]
    pub refund_treshold: i64,
    #[serde(rename = "ChargeTreshold")]
    pub charge_treshold: i64,
    #[serde(rename = "CostInclusive")]
    pub cost_inclusive: bool,
    #[serde(rename = "ProductCode")]
    pub product_code: i64,
    #[serde(rename = "DefaultSolution")]
    pub default_solution: Value,
    #[serde(rename = "ReferencePriceProductCode")]
    pub reference_price_product_code: Value,
    #[serde(rename = "CytIngredientGroup")]
    pub cyt_ingredient_group: Value,
    #[serde(rename = "CytIngredientType")]
    pub cyt_ingredient_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Names {
    #[serde(rename = "ProductCode")]
    pub product_code: i64,
    #[serde(rename = "IsValid")]
    pub is_valid: bool,
    #[serde(rename = "Names")]
    pub names: Vec<Name>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Name {
    #[serde(rename = "LanguageID")]
    pub language_id: String,
    #[serde(rename = "ShortName")]
    pub short_name: String,
    #[serde(rename = "LongName")]
    pub long_name: String,
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SmartRouting {
    #[serde(rename = "CytProduct")]
    pub cyt_product: Value,
    #[serde(rename = "CytGroupDisplayOrder")]
    pub cyt_group_display_order: Value,
    #[serde(rename = "CytIngredientGroup")]
    pub cyt_ingredient_group: Option<String>,
    #[serde(rename = "CytIngredientType")]
    pub cyt_ingredient_type: Option<String>,
    #[serde(rename = "DeliverEarlyEnabled")]
    pub deliver_early_enabled: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductPrice {
    #[serde(rename = "ProductCode")]
    pub product_code: i64,
    #[serde(rename = "Prices")]
    pub prices: Vec<Price>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Price {
    #[serde(rename = "PriceTypeID")]
    pub price_type_id: i64,
    #[serde(rename = "Price")]
    pub price: f64,
    #[serde(rename = "IsValid")]
    pub is_valid: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Availability {
    #[serde(rename = "ProductCode")]
    pub product_code: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RestaurantResponse {
    pub status: Status,
    pub response: Option<InnerRestaurantResponse>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InnerRestaurantResponse {
    pub restaurant: FullRestaurantInformation,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FullRestaurantInformation {
    pub address: Address,
    pub catalog: Catalog,
    pub facilities: Vec<String>,
    pub national_store_number: i64,
    pub name: String,
    pub status: i64,
    pub restaurant_status: String,
    pub location: Location,
    pub order: Order,
    pub phone_number: Option<String>,
    pub time_zone: String,
    pub url: Option<String>,
    pub week_opening_hours: Vec<WeekOpeningHour>,
    pub accept_offer: Option<bool>,
    pub areas: Option<Vec<Area>>,
    pub contacts: Option<Vec<Contact>>,
    pub country_code: Option<String>,
    pub distance: Option<i64>,
    pub gbl_number: Option<String>,
    pub id: Option<String>,
    pub is_valid: Option<bool>,
    pub market_code: Option<String>,
    pub now_in_store_local_time_date: Option<String>,
    pub nutrition: Option<RestaurantNutrition>,
    pub offer_configuration: Option<OfferConfiguration>,
    pub special_dayservice: Option<Vec<Value>>,
    #[serde(rename = "statusID")]
    pub status_id: Option<i64>,
    pub tin_threshold_amout: Option<i64>,
    pub store_type: Option<StoreType>,
    pub tod_cutoff_time: Option<String>,
    pub day_part: Option<i64>,
    pub np_version: Option<String>,
    pub store_cutoff_time: Option<String>,
    pub legal_name: Option<String>,
    pub service_payments: Option<Vec<ServicePayment>>,
    pub general_status: Option<GeneralStatus>,
    pub available_menu_products: Option<AvailableMenuProducts>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Catalog {
    pub points_of_distribution: Vec<PointsOfDistribution>,
    pub table_service: TableService,
    pub outage_product_codes: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PointsOfDistribution {
    pub digital_services: Vec<DigitalService>,
    #[serde(rename = "locationID")]
    pub location_id: i64,
    pub pod: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DigitalService {
    pub key: String,
    pub technologies: Vec<Technology>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Technology {
    pub key: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TableService {
    #[serde(rename = "enablePOSTableService")]
    pub enable_postable_service: bool,
    pub enable_table_service_eatin: String,
    pub enable_table_service_takeout: String,
    pub minimum_purchase_amount: f64,
    pub table_service_enable_map: bool,
    pub table_service_locator_enabled: bool,
    pub table_service_locator_max_number_value: i64,
    pub table_service_locator_min_number_value: i64,
    pub digital_table_service_mode: String,
    pub table_service_table_number_min_number_value: i64,
    pub table_service_table_number_max_number_value: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    pub auto_bag_sale_information: AutoBagSaleInformation,
    pub expected_delivery_time: String,
    pub store_menu_type_calendar: Vec<StoreMenuTypeCalendar>,
    pub minimum_order_value: f64,
    pub large_order_allowed: bool,
    pub linked_payment_information: bool,
    pub loyalty_enabled: bool,
    pub maximum_time_minutes: Option<i64>,
    pub minimum_time_minutes: Option<i64>,
    pub daypart_transition_offset: i64,
    pub ready_on_arrival_information: bool,
    pub order_ahead_lane: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AutoBagSaleInformation {
    pub bag_choice_product_code: i64,
    pub bag_dummy_product_code: i64,
    pub bag_product_code: i64,
    pub enabled: bool,
    pub no_bag_product_code: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StoreMenuTypeCalendar {
    pub end_time: String,
    #[serde(rename = "menuTypeID")]
    pub menu_type_id: i64,
    pub start_time: String,
    pub week_day: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Area {
    pub area_type: String,
    pub capacity: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Contact {
    pub title: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RestaurantNutrition {
    pub energy_unit: String,
    pub customer_self_pour: bool,
    pub recalculate_energy_on_grill: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OfferConfiguration {
    pub enable_multiple_offers: bool,
    pub offer_buckets: Vec<OfferBucket>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OfferBucket {
    pub offer_bucket: String,
    pub limit: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StoreType {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServicePayment {
    #[serde(rename = "serviceID")]
    pub service_id: i64,
    pub sale_type_eat_in: bool,
    pub sale_type_other: bool,
    pub sale_type_take_out: bool,
    pub payment_methods: Vec<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeneralStatus {
    pub start_date: String,
    pub status: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AvailableMenuProducts {
    #[serde(rename = "1")]
    pub n1: Vec<i64>,
    #[serde(rename = "2")]
    pub n2: Vec<i64>,
    #[serde(rename = "3")]
    pub n3: Vec<i64>,
}
