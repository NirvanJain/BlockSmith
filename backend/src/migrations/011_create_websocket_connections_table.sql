CREATE TABLE websocket_connections (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT REFERENCES users(id),
    connection_id TEXT NOT NULL,
    connected_at TIMESTAMP DEFAULT NOW(),
    disconnected_at TIMESTAMP
);