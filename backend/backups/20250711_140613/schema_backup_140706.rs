// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "crop_type_enum"))]
    pub struct CropTypeEnum;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "event_type_enum"))]
    pub struct EventTypeEnum;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "language_enum"))]
    pub struct LanguageEnum;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "lot_status_enum"))]
    pub struct LotStatusEnum;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "point", schema = "pg_catalog"))]
    pub struct Point;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "producer_status_enum"))]
    pub struct ProducerStatusEnum;
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::schema::sql_types::*;
    use super::sql_types::EventTypeEnum;
    use super::sql_types::Point;

    events (id) {
        id -> Uuid,
        lot_id -> Uuid,
        event_type -> EventTypeEnum,
        description -> Nullable<Text>,
        event_location -> Nullable<Text>,
        coordinates -> Nullable<Point>,
        metadata -> Nullable<Jsonb>,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::schema::sql_types::*;
    use super::sql_types::CropTypeEnum;
    use super::sql_types::LotStatusEnum;
    use super::sql_types::Point;

    lots (id) {
        id -> Uuid,
        producer_id -> Uuid,
        lot_code -> Text,
        product_name -> Text,
        crop_type -> CropTypeEnum,
        estimated_quantity -> Numeric,
        unit_of_measure -> Text,
        estimated_harvest_date -> Date,
        actual_harvest_date -> Nullable<Date>,
        current_status -> LotStatusEnum,
        additional_description -> Nullable<Text>,
        location_coordinates -> Nullable<Point>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::schema::sql_types::*;
    use super::sql_types::LanguageEnum;
    use super::sql_types::ProducerStatusEnum;

    producers (id) {
        id -> Uuid,
        full_name -> Text,
        email -> Text,
        password_hash -> Text,
        farm_name -> Nullable<Text>,
        phone -> Nullable<Text>,
        language_preference -> LanguageEnum,
        is_active -> Bool,
        email_verified -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        status -> ProducerStatusEnum,
    }
}

diesel::joinable!(events -> lots (lot_id));
diesel::joinable!(lots -> producers (producer_id));

diesel::allow_tables_to_appear_in_same_query!(
    events,
    lots,
    producers,
);
