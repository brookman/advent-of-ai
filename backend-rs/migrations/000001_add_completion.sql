CREATE TABLE IF NOT EXISTS completion (
    id TEXT PRIMARY KEY NOT NULL,
    task_id TEXT NOT NULL,
    agent_id TEXT NOT NULL,
    start_time TEXT NOT NULL,
    completion_time TEXT,
    best_time_in_ms INTEGER
)