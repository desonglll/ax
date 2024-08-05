-- Your SQL goes here
-- 创建 suppliers 表
CREATE TABLE IF NOT EXISTS supplier_table (
    supplier_id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    address VARCHAR(255),
    phone_number VARCHAR(20),
    email VARCHAR(255),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- 插入示例数据
INSERT INTO supplier_table (name, address, phone_number, email) VALUES 
('Supplier 1', '123 Supplier St, Supplier City', '123-456-7890', 'supplier1@example.com'),
('Supplier 2', '456 Supplier St, Supplier City', '234-567-8901', 'supplier2@example.com'),
('Supplier 3', '789 Supplier St, Supplier City', '345-678-9012', 'supplier3@example.com'),
('Supplier 4', '101 Supplier St, Supplier City', '456-789-0123', 'supplier4@example.com'),
('Supplier 5', '202 Supplier St, Supplier City', '567-890-1234', 'supplier5@example.com'),
('Supplier 6', '303 Supplier St, Supplier City', '678-901-2345', 'supplier6@example.com'),
('Supplier 7', '404 Supplier St, Supplier City', '789-012-3456', 'supplier7@example.com'),
('Supplier 8', '505 Supplier St, Supplier City', '890-123-4567', 'supplier8@example.com'),
('Supplier 9', '606 Supplier St, Supplier City', '901-234-5678', 'supplier9@example.com'),
('Supplier 10', '707 Supplier St, Supplier City', '012-345-6789', 'supplier10@example.com');
