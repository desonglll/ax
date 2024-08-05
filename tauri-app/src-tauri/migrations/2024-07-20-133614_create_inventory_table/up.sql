-- inventory_table（库存表）：用于存储产品库存的详细信息。
-- 虽然 product_table 包括库存数量，但 inventory_table 可以更细化地记录不同位置或仓库的库存情况。
-- Your SQL goes here
CREATE TABLE IF NOT EXISTS inventory_table (
    inventory_id SERIAL PRIMARY KEY,
    product_id INTEGER REFERENCES product_table(product_id) ON DELETE CASCADE,
    location VARCHAR(255),
    quantity INTEGER DEFAULT 0,
    last_updated TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- 插入示例数据
INSERT INTO inventory_table (product_id, location, quantity) VALUES
(1, 'Warehouse A', 100),
(2, 'Warehouse B', 200),
(3, 'Warehouse C', 150);
