-- Eliminar triggers
DROP TRIGGER IF EXISTS update_producers_timestamp ON producers;
DROP TRIGGER IF EXISTS update_lots_timestamp ON lots;

-- Eliminar funciones
DROP FUNCTION IF EXISTS update_timestamp() CASCADE;
DROP FUNCTION IF EXISTS generate_lot_code() CASCADE;

-- Eliminar índices
DROP INDEX IF EXISTS idx_producers_email;
DROP INDEX IF EXISTS idx_producers_active;
DROP INDEX IF EXISTS idx_lots_producer_id;
DROP INDEX IF EXISTS idx_lots_code;
DROP INDEX IF EXISTS idx_lots_status;
DROP INDEX IF EXISTS idx_lots_harvest_date;
DROP INDEX IF EXISTS idx_events_lot_id;
DROP INDEX IF EXISTS idx_events_type;
DROP INDEX IF EXISTS idx_events_created_at;
DROP INDEX IF EXISTS idx_lots_product_name_gin;

-- Eliminar tablas
DROP TABLE IF EXISTS events CASCADE;
DROP TABLE IF EXISTS lots CASCADE;
DROP TABLE IF EXISTS producers CASCADE;

-- Eliminar tipos ENUM
DROP TYPE IF EXISTS event_type_enum;
DROP TYPE IF EXISTS lot_status_enum;
DROP TYPE IF EXISTS crop_type_enum;
DROP TYPE IF EXISTS language_enum;

-- Eliminar extensiones
DROP EXTENSION IF EXISTS "uuid-ossp";
DROP EXTENSION IF EXISTS "pg_trgm"; 