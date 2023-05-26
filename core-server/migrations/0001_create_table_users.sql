CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE users (
    id UUID DEFAULT uuid_generate_v4() PRIMARY KEY,
    username VARCHAR(16) UNIQUE NOT NULL,
    password VARCHAR(64) NOT NULL,
    email VARCHAR(320) UNIQUE NOT NULL,
    verified_email BOOLEAN DEFAULT False NOT NULL
);
