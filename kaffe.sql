CREATE TABLE IF NOT EXISTS equipment (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    kind TEXT NOT NULL,
    purchase_date DATETIME NOT NULL,
    decommission_date DATETIME DEFAULT NULL,
    price_ct INTEGER NOT NULL,
    timestamp DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS coffee (
    id INTEGER PRIMARY KEY,
    roaster TEXT NOT NULL,
    name TEXT NOT NULL,
    roast_level TEXT NOT NULL,
    kind TEXT NOT NULL,
    country TEXT,
    region TEXT,
    farm TEXT,
    producer TEXT,
    varietals TEXT,
    altitude_m INTEGER,
    altitude_lower_m INTEGER,
    altitude_upper_m INTEGER,
    process TEXT NOT NULL,
    tasting_notes TEXT NOT NULL,
    decaf INTEGER NOT NULL,
    timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,

    UNIQUE(roaster, name, roast_level)
);

CREATE TABLE IF NOT EXISTS bag (
    id INTEGER PRIMARY KEY,
    coffee_id INTEGER REFERENCES coffee(id), --foreign key
    roast_date DATETIME NOT NULL,
    open_date DATETIME,
    empty_date DATETIME,
    weight_g INTEGER NOT NULL,
    price_ct INTEGER NOT NULL,
    timestamp DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS brew (
    id INTEGER PRIMARY KEY,
    bag_id INTEGER REFERENCES bag(id), --foreign key
    grinder_id INTEGER REFERENCES equipment(id), --foreign key
    brewer_id INTEGER REFERENCES equipment(id), --foreign key
    grind_level INTEGER NOT NULL,
    coffee_g REAL NOT NULL,
    water_g REAL,
    brew_g REAL,
    temp_c REAL,
    time_s INTEGER,
    rating INTEGER NOT NULL,
    notes TEXT,
    timestamp DATETIME DEFAULT CURRENT_TIMESTAMP
);
