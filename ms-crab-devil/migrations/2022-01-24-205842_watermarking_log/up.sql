CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE watermarking_log (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    full_name VARCHAR NOT NULL,
    file_bucket VARCHAR NOT NULL,
    file_key VARCHAR NOT NULL,
    watermarked_at TIMESTAMP NOT NULL
);
