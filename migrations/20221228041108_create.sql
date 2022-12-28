CREATE TABLE IF NOT EXISTS  app_configure (
    "id" SERIAL PRIMARY KEY,
    "name" VARCHAR(32) NOT NULL,
    "data_type" VARCHAR(7) NOT NULL,
    "data" TEXT,
    "description" VARCHAR(64),
    "effective" BOOLEAN DEFAULT true
);