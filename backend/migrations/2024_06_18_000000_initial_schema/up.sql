-- Extensiones requeridas
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "pg_trgm";

-- Tipos ENUM
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'crop_type_enum') THEN
        CREATE TYPE crop_type_enum AS ENUM (
            'CONVENTIONAL',
            'AGROECOLOGICAL_UNCERTIFIED', 
            'ORGANIC_CERTIFIED',
            'HYDROPONIC'
        );
    END IF;
END$$;

DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'lot_status_enum') THEN
        CREATE TYPE lot_status_enum AS ENUM (
            'REGISTERED',
            'GROWING', 
            'READY_FOR_HARVEST',
            'HARVESTED',
            'SOLD',
            'CANCELLED'
        );
    END IF;
END$$;

DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'event_type_enum') THEN
        CREATE TYPE event_type_enum AS ENUM (
            'LOT_REGISTERED',
            'FERTILIZER_APPLICATION',
            'IRRIGATION',
            'PEST_CONTROL',
            'HARVEST_STARTED', 
            'HARVEST_COMPLETED',
            'LOT_UPDATED',
            'QUALITY_INSPECTION'
        );
    END IF;
END$$;

DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'language_enum') THEN
        CREATE TYPE language_enum AS ENUM ('es', 'pt', 'en');
    END IF;
END$$;

-- Tabla productores
CREATE TABLE IF NOT EXISTS producers (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    full_name TEXT NOT NULL CHECK (length(full_name) >= 2),
    email TEXT NOT NULL UNIQUE CHECK (email ~* '^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}$'),
    password_hash TEXT NOT NULL CHECK (length(password_hash) >= 32),
    farm_name TEXT CHECK (farm_name IS NULL OR length(farm_name) >= 2),
    phone TEXT CHECK (phone IS NULL OR phone ~ '^\+?[1-9]\d{1,14}$'),
    language_preference language_enum NOT NULL DEFAULT 'es',
    is_active BOOLEAN NOT NULL DEFAULT true,
    email_verified BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Tabla lotes
CREATE TABLE IF NOT EXISTS lots (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    producer_id UUID NOT NULL REFERENCES producers(id) ON DELETE CASCADE,
    lot_code TEXT NOT NULL UNIQUE CHECK (lot_code ~ '^[A-Z0-9-]{8,20}$'),
    product_name TEXT NOT NULL CHECK (length(product_name) >= 2),
    crop_type crop_type_enum NOT NULL,
    estimated_quantity NUMERIC(12,3) NOT NULL CHECK (estimated_quantity > 0),
    unit_of_measure TEXT NOT NULL CHECK (unit_of_measure IN ('kg', 'ton', 'unit', 'box', 'sack')),
    estimated_harvest_date DATE NOT NULL CHECK (estimated_harvest_date >= CURRENT_DATE),
    actual_harvest_date DATE CHECK (actual_harvest_date IS NULL OR actual_harvest_date >= CURRENT_DATE - INTERVAL '1 year'),
    current_status lot_status_enum NOT NULL DEFAULT 'REGISTERED',
    additional_description TEXT,
    location_coordinates POINT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT check_harvest_dates CHECK (
        actual_harvest_date IS NULL OR 
        actual_harvest_date >= estimated_harvest_date
    )
);

-- Tabla eventos
CREATE TABLE IF NOT EXISTS events (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    lot_id UUID NOT NULL REFERENCES lots(id) ON DELETE CASCADE,
    event_type event_type_enum NOT NULL,
    description TEXT CHECK (description IS NULL OR length(description) >= 3),
    event_location TEXT,
    coordinates POINT,
    metadata JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT check_description_required CHECK (
        event_type NOT IN ('FERTILIZER_APPLICATION', 'PEST_CONTROL', 'QUALITY_INSPECTION') OR 
        (description IS NOT NULL AND length(description) >= 3)
    )
);

-- Función para auto-actualizar updated_at
CREATE OR REPLACE FUNCTION update_timestamp()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Triggers
CREATE TRIGGER update_producers_timestamp
    BEFORE UPDATE ON producers
    FOR EACH ROW EXECUTE FUNCTION update_timestamp();

CREATE TRIGGER update_lots_timestamp
    BEFORE UPDATE ON lots
    FOR EACH ROW EXECUTE FUNCTION update_timestamp();

-- Función para generar código de lote único
CREATE OR REPLACE FUNCTION generate_lot_code()
RETURNS TEXT AS $$
DECLARE
    code TEXT;
    exists BOOLEAN;
BEGIN
    LOOP
        code := 'KAI-' || 
                upper(substring(md5(random()::text) from 1 for 3)) || '-' ||
                to_char(CURRENT_DATE, 'YYYY') || '-' ||
                upper(substring(md5(random()::text) from 1 for 4));
        SELECT EXISTS(SELECT 1 FROM lots WHERE lot_code = code) INTO exists;
        IF NOT exists THEN
            RETURN code;
        END IF;
    END LOOP;
END;
$$ LANGUAGE plpgsql;

-- Índices
CREATE INDEX idx_producers_email ON producers(email);
CREATE INDEX idx_producers_active ON producers(is_active) WHERE is_active = true;
CREATE INDEX idx_lots_producer_id ON lots(producer_id);
CREATE INDEX idx_lots_code ON lots(lot_code);
CREATE INDEX idx_lots_status ON lots(current_status);
CREATE INDEX idx_lots_harvest_date ON lots(estimated_harvest_date);
CREATE INDEX idx_events_lot_id ON events(lot_id);
CREATE INDEX idx_events_type ON events(event_type);
CREATE INDEX idx_events_created_at ON events(created_at DESC);
CREATE INDEX idx_lots_product_name_gin ON lots USING gin(product_name gin_trgm_ops); 