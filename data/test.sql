CREATE TABLE IF NOT EXISTS equipment (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    kind TEXT NOT NULL,
    purchase_date DATETIME NOT NULL,
    decommission_date DATETIME,
    price_ct INTEGER NOT NULL,
    timestamp DATETIME DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO equipment (
    name,
    kind,
    purchase_date,
    decommission_date,
    price_ct,
    timestamp
) VALUES (
    "Niche Zero",
    "grinder",
    "2024-01-01T00:00:00.000Z",
    "",
    50000,
    "2024-01-01T00:00:00.000Z"
)