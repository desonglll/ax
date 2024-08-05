-- Your SQL goes here
CREATE TABLE IF NOT EXISTS shipment_table (
    shipment_id SERIAL PRIMARY KEY,
    order_id INTEGER REFERENCES order_table(order_id),
    shipment_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    delivery_address TEXT,
    status VARCHAR(50) DEFAULT 'PENDING'
);
INSERT INTO shipment_table (order_id, shipment_date, delivery_address, status) VALUES
(1, CURRENT_TIMESTAMP, '123 Main St, Springfield', 'SHIPPED'),
(2, CURRENT_TIMESTAMP, '456 Elm St, Springfield', 'PENDING');
