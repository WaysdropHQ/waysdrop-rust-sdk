use serde_json::Value;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayMoneyLocal {
    pub currency: String,
    pub amount: f64,
    #[serde(default)]
    pub symbol: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayMoney {
    pub usd: f64,
    #[serde(default)]
    pub local: Option<DisplayMoneyLocal>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistanceInfo {
    #[serde(rename = "distanceKm")]
    pub distance_km: f64,
    #[serde(rename = "etaSeconds")]
    pub eta_seconds: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GeoLocation {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default, rename = "addressLine1")]
    pub address_line1: Option<String>,
    #[serde(default, rename = "lgaOrCity")]
    pub lga_or_city: Option<String>,
    #[serde(default)]
    pub state: Option<String>,
    #[serde(default)]
    pub country: Option<String>,
    #[serde(default, rename = "countryCode")]
    pub country_code: Option<String>,
    #[serde(default)]
    pub lat: Option<f64>,
    #[serde(default)]
    pub lon: Option<f64>,
    #[serde(default, rename = "googlePlaceId")]
    pub google_place_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CountryLocation {
    pub value: String,
    pub name: String,
    #[serde(rename = "type")]
    pub location_type: String,
    pub country: String,
    #[serde(rename = "countryCode")]
    pub country_code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateLocation {
    pub value: String,
    pub name: String,
    #[serde(rename = "type")]
    pub location_type: String,
    pub state: String,
    pub country: String,
    #[serde(rename = "countryCode")]
    pub country_code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CityLocation {
    #[serde(default, rename = "locationId")]
    pub location_id: Option<String>,
    pub value: String,
    pub country: String,
    #[serde(rename = "countryCode")]
    pub country_code: String,
    #[serde(default)]
    pub lat: Option<f64>,
    #[serde(default)]
    pub lon: Option<f64>,
    #[serde(default, rename = "lgaOrCity")]
    pub lga_or_city: Option<String>,
    #[serde(default)]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FleetType {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteDataResponse {
    pub distance: DistanceInfo,
    #[serde(rename = "routeType")]
    pub route_type: String,
    pub origin: GeoLocation,
    pub destination: GeoLocation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricingResponse {
    pub distance: DistanceInfo,
    #[serde(rename = "routeType")]
    pub route_type: String,
    pub costs: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryPackage {
    pub id: String,
    pub name: String,
    pub quantity: i64,
    pub weight: Value,
    pub value: Value,
    pub size: String,
    #[serde(default, rename = "valueDisplay")]
    pub value_display: Option<DisplayMoney>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliverySummary {
    pub id: String,
    #[serde(rename = "trackingId")]
    pub tracking_id: String,
    pub status: String,
    #[serde(default)]
    pub r#type: Option<String>,
    #[serde(default, rename = "routeType")]
    pub route_type: Option<String>,
    #[serde(default, rename = "deliveryFee")]
    pub delivery_fee: Option<Value>,
    #[serde(default)]
    pub origin: Option<GeoLocation>,
    #[serde(default)]
    pub destination: Option<GeoLocation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryDetail {
    #[serde(flatten)]
    pub summary: DeliverySummary,
    #[serde(default, rename = "deliverySteps")]
    pub delivery_steps: Vec<HashMap<String, Value>>,
    #[serde(default)]
    pub proofs: Vec<HashMap<String, Value>>,
    #[serde(default, rename = "fleetType")]
    pub fleet_type: Option<FleetType>,
    #[serde(default, rename = "p2pDelivery")]
    pub p2p_delivery: Option<HashMap<String, Value>>,
    #[serde(default)]
    pub courier: Option<HashMap<String, Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDeliveryResponse {
    pub id: String,
    pub status: String,
    #[serde(default, rename = "totalWeight")]
    pub total_weight: Option<Value>,
    #[serde(default, rename = "totalValue")]
    pub total_value: Option<Value>,
    #[serde(default, rename = "deliveryId")]
    pub delivery_id: Option<String>,
    #[serde(default)]
    pub delivery: Option<DeliverySummary>,
    #[serde(default)]
    pub processor: Option<String>,
    #[serde(default)]
    pub reference: Option<String>,
    #[serde(default, rename = "charge_currency")]
    pub charge_currency: Option<String>,
    #[serde(default, rename = "charge_amount")]
    pub charge_amount: Option<f64>,
    #[serde(default, rename = "authorization_url")]
    pub authorization_url: Option<String>,
    #[serde(default, rename = "checkout_url")]
    pub checkout_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelDeliveryResponse {
    pub delivery: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MerchantWallet {
    pub id: String,
    #[serde(rename = "currencyCode")]
    pub currency_code: String,
    pub balance: String,
    #[serde(default, rename = "balanceDisplay")]
    pub balance_display: Option<DisplayMoney>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentCheckoutResponse {
    pub processor: String,
    pub reference: String,
    #[serde(rename = "charge_currency")]
    pub charge_currency: String,
    #[serde(rename = "charge_amount")]
    pub charge_amount: f64,
    #[serde(default, rename = "authorization_url")]
    pub authorization_url: Option<String>,
    #[serde(default, rename = "checkout_url")]
    pub checkout_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoreProfile {
    pub id: String,
    pub name: String,
    pub tag: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountSummary {
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "countryCode")]
    pub country_code: String,
    #[serde(rename = "displayCurrency")]
    pub display_currency: String,
    #[serde(rename = "merchantWalletCurrencyCode")]
    pub merchant_wallet_currency_code: String,
    #[serde(default, rename = "storeProfile")]
    pub store_profile: Option<StoreProfile>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExchangeRateResponse {
    pub from: String,
    pub to: String,
    pub rate: f64,
    #[serde(default, rename = "isStale")]
    pub is_stale: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvertCurrencyResponse {
    pub from: String,
    pub to: String,
    pub amount: f64,
    #[serde(rename = "convertedAmount")]
    pub converted_amount: f64,
    pub rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedMeta {
    pub total: i64,
    pub page: i64,
    pub limit: i64,
    #[serde(rename = "totalPages")]
    pub total_pages: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListDeliveriesResponse {
    pub data: Vec<DeliveryDetail>,
    pub meta: PaginatedMeta,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookEnvelope {
    pub event: String,
    pub data: Value,
}

pub fn decode_value<T: serde::de::DeserializeOwned>(value: Value) -> Result<T, crate::error::WaysdropError> {
    serde_json::from_value(value).map_err(Into::into)
}
