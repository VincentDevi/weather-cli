CREATE TABLE cities (
    id          INTEGER PRIMARY KEY,
    name        TEXT NOT NULL,
    name_key    TEXT NOT NULL UNIQUE,
    latitude    REAL NOT NULL,
    longitude   REAL NOT NULL,
    is_favorite INTEGER NOT NULL DEFAULT 0 CHECK (is_favorite IN (0, 1))
);

CREATE INDEX idx_cities_favorite ON cities (is_favorite, id);

CREATE TABLE forecasts (
    id                        INTEGER PRIMARY KEY,
    city_id                   INTEGER NOT NULL,
    forecast_at               INTEGER NOT NULL,
    temperature_celsius       REAL NOT NULL,
    humidity_percent          INTEGER NOT NULL
                              CHECK (humidity_percent BETWEEN 0 AND 100),
    precipitation_probability REAL NOT NULL
                              CHECK (precipitation_probability BETWEEN 0 AND 1),
    timezone_offset_seconds   INTEGER NOT NULL,
    FOREIGN KEY (city_id) REFERENCES cities (id) ON DELETE CASCADE,
    UNIQUE (city_id, forecast_at)
);
