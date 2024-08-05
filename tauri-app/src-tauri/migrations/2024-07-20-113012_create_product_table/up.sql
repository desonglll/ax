-- Your SQL goes here
-- 创建 products 表
CREATE TABLE IF NOT EXISTS product_table(
    product_id SERIAL PRIMARY KEY,
    user_id INTEGER REFERENCES user_table(user_id) ON DELETE SET NULL,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    sku VARCHAR(50) UNIQUE NOT NULL,
    image VARCHAR(255),
    price FLOAT DEFAULT 0.0,
    discount_price FLOAT DEFAULT 0.0,
    is_discounted BOOLEAN DEFAULT FALSE,
    is_valid BOOLEAN DEFAULT FALSE,
    inventory INTEGER DEFAULT 0,
    is_in_stock BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    supplier_id INTEGER REFERENCES supplier_table(supplier_id) ON DELETE SET NULL,
    weight FLOAT DEFAULT 0.0,
    dimensions VARCHAR(50),
    status VARCHAR(20) DEFAULT 'DRAFT',
    public BOOLEAN DEFAULT TRUE
);

-- 插入示例数据到 products 表
INSERT INTO product_table(user_id, name, description, sku, image, price, discount_price, is_discounted, is_valid, inventory, is_in_stock, supplier_id, weight, dimensions, status, public) VALUES 
(1, 'Product 1', 'Description for Product 1', 'SKU001', 'http://example.com/image1.jpg', 100.00, 80.00, TRUE, TRUE, 50, TRUE, 1, 1.5, '10x10x10', 'Available', TRUE),
(2, 'Product 2', 'Description for Product 2', 'SKU002', 'http://example.com/image2.jpg', 200.00, 150.00, TRUE, TRUE, 30, TRUE, 2, 2.5, '20x20x20', 'Available', TRUE),
(3, 'Product 3', 'Description for Product 3', 'SKU003', 'http://example.com/image3.jpg', 300.00, 250.00, TRUE, TRUE, 20, TRUE, 3, 3.5, '30x30x30', 'Available', TRUE),
(1, 'Product 4', 'Description for Product 4', 'SKU004', 'http://example.com/image4.jpg', 400.00, 350.00, TRUE, TRUE, 40, TRUE, 1, 4.5, '40x40x40', 'Available', TRUE),
(2, 'Product 5', 'Description for Product 5', 'SKU005', 'http://example.com/image5.jpg', 500.00, 450.00, TRUE, TRUE, 10, TRUE, 2, 5.5, '50x50x50', 'Available', TRUE),
(3, 'Product 6', 'Description for Product 6', 'SKU006', 'http://example.com/image6.jpg', 600.00, 550.00, TRUE, TRUE, 5, TRUE, 3, 6.5, '60x60x60', 'Available', TRUE),
(1, 'Product 7', 'Description for Product 7', 'SKU007', 'http://example.com/image7.jpg', 700.00, 650.00, TRUE, TRUE, 15, TRUE, 1, 7.5, '70x70x70', 'Available', TRUE),
(2, 'Product 8', 'Description for Product 8', 'SKU008', 'http://example.com/image8.jpg', 800.00, 750.00, TRUE, TRUE, 25, TRUE, 2, 8.5, '80x80x80', 'Available', TRUE),
(3, 'Product 9', 'Description for Product 9', 'SKU009', 'http://example.com/image9.jpg', 900.00, 850.00, TRUE, TRUE, 35, TRUE, 3, 9.5, '90x90x90', 'Available', TRUE),
(1, 'Product 10', 'Description for Product 10', 'SKU010', 'http://example.com/image10.jpg', 1000.00, 950.00, TRUE, TRUE, 45, TRUE, 1, 10.5, '100x100x100', 'Available', TRUE);
