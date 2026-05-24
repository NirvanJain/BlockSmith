CREATE TABLE search_index (
    id BIGSERIAL PRIMARY KEY,
    entity_type TEXT NOT NULL,
    entity_id BIGINT NOT NULL,
    searchable_content TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
);