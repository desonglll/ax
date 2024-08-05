-- Your SQL goes here
CREATE TABLE reload_counts (
    reload_date DATE PRIMARY KEY,
    count INTEGER NOT NULL
);
INSERT INTO reload_counts (reload_date, count) VALUES
('2023-06-01', 5),
('2023-06-02', 3),
('2023-06-03', 8);
