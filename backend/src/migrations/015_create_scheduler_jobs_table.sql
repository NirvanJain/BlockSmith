CREATE TABLE scheduler_jobs (
    id BIGSERIAL PRIMARY KEY,
    job_name TEXT NOT NULL,
    job_status TEXT NOT NULL,
    last_run TIMESTAMP,
    next_run TIMESTAMP,
    created_at TIMESTAMP DEFAULT NOW()
);