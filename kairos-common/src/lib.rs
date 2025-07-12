use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash, Default)]
pub enum Language {
    English,
    #[default]
    Spanish,
    Portuguese,
    French,
    German,
    Italian,
    Russian,
    Chinese,
    Japanese,
    Korean,
}

// Implementación para convertir el enum a un string
impl Language {
    pub fn to_str(&self) -> &'static str {
        match self {
            Language::English => "en",
            Language::Spanish => "es",
            Language::Portuguese => "pt",
            Language::French => "fr",
            Language::German => "de",
            Language::Italian => "it",
            Language::Russian => "ru",
            Language::Chinese => "zh",
            Language::Japanese => "ja",
            Language::Korean => "ko",
        }
    }
}

// Implementación para poder mostrar el enum en la UI
impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

// Implementación para poder parsear un string al enum
impl FromStr for Language {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "en" => Ok(Language::English),
            "es" => Ok(Language::Spanish),
            "pt" => Ok(Language::Portuguese),
            "fr" => Ok(Language::French),
            "de" => Ok(Language::German),
            "it" => Ok(Language::Italian),
            "ru" => Ok(Language::Russian),
            "zh" => Ok(Language::Chinese),
            "ja" => Ok(Language::Japanese),
            "ko" => Ok(Language::Korean),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EventType {
    Planting,
    Fertilization,
    Irrigation,
    PestControl,
    Harvest,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CropType {
    Grain,
    Vegetable,
    Fruit,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LotStatus {
    Registered,
    InProgress,
    Harvested,
    Completed,
    Cancelled,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterProducerRequest {
    pub full_name: String,
    pub email: String,
    pub password: String,
    pub farm_name: Option<String>,
    pub phone: Option<String>,
    pub language_preference: Option<Language>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateLotRequest {
    pub product_name: String,
    pub crop_type: CropType,
    pub estimated_quantity: f64,
    pub unit_of_measure: String,
    pub estimated_harvest_date: DateTime<Utc>,
    pub additional_description: Option<String>,
    pub location_coordinates: Option<Point>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateEventRequest {
    pub event_type: EventType,
    pub description: Option<String>,
    pub event_location: Option<String>,
    pub coordinates: Option<Point>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub data: T,
    pub meta: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiError {
    pub code: String,
    pub message: String,
    pub details: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i64,
    pub user: Producer,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Producer {
    pub id: Uuid,
    pub full_name: String,
    pub email: String,
    pub farm_name: Option<String>,
    pub phone: Option<String>,
    pub language_preference: Language,
    pub is_active: bool,
    pub email_verified: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Lot {
    pub id: Uuid,
    pub producer_id: Uuid,
    pub lot_code: String,
    pub product_name: String,
    pub crop_type: CropType,
    pub estimated_quantity: f64,
    pub unit_of_measure: String,
    pub estimated_harvest_date: DateTime<Utc>,
    pub actual_harvest_date: Option<DateTime<Utc>>,
    pub current_status: LotStatus,
    pub additional_description: Option<String>,
    pub location_coordinates: Option<Point>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    pub id: Uuid,
    pub lot_id: Uuid,
    pub event_type: EventType,
    pub description: Option<String>,
    pub event_location: Option<String>,
    pub coordinates: Option<Point>,
    pub metadata: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
}

// ===============================================
// Nuevas estructuras de DTOs añadidas
// ===============================================

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateEventRequest {
    pub event_type: Option<EventType>,
    pub description: Option<String>,
    pub event_location: Option<String>,
    pub coordinates: Option<Point>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct EventQuery {
    pub event_type: Option<EventType>,
    pub lot_id: Option<Uuid>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateLotRequest {
    pub lot_code: Option<String>,
    pub product_name: Option<String>,
    pub crop_type: Option<CropType>,
    pub estimated_quantity: Option<rust_decimal::Decimal>,
    pub unit_of_measure: Option<String>,
    pub estimated_harvest_date: Option<chrono::NaiveDate>,
    pub actual_harvest_date: Option<chrono::NaiveDate>,
    pub current_status: Option<LotStatus>,
    pub additional_description: Option<String>,
    pub location_coordinates: Option<Point>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct LotQuery {
    pub product_name: Option<String>,
    pub status: Option<LotStatus>,
    pub crop_type: Option<CropType>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateProducerRequest {
    pub full_name: Option<String>,
    pub farm_name: Option<String>,
    pub phone: Option<String>,
    pub language_preference: Option<Language>,
    pub is_active: Option<bool>,
    pub status: Option<ProducerStatus>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ProducerQuery {
    pub name: Option<String>,
    pub email: Option<String>,
    pub is_active: Option<bool>,
    pub language: Option<Language>,
    pub status: Option<ProducerStatus>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateUserRequest {
    pub name: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct PaginationParams {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SearchParams {
    pub search: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProducerStatus {
    Pending,
    Approved,
    Rejected,
}
