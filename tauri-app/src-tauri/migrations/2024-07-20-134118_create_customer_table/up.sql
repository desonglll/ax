-- Your SQL goes here
CREATE TABLE IF NOT EXISTS customer_table (
    customer_id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    phone VARCHAR(20),
    address TEXT
);
INSERT INTO customer_table (name, email, phone, address) VALUES
('Alice Johnson', 'alice.johnson@example.com', '123-456-7890', '123 Maple St, Springfield'),
('Bob Smith', 'bob.smith@example.com', '987-654-3210', '456 Oak St, Springfield'),
('Charlie Brown', 'charlie.brown@example.com', '555-123-4567', '789 Pine St, Springfield'),
('Daisy Miller', 'daisy.miller@example.com', '555-987-6543', '321 Elm St, Springfield'),
('Eva Green', 'eva.green@example.com', '555-654-3210', '654 Cedar St, Springfield'),
('Frank White', 'frank.white@example.com', '555-321-9876', '987 Birch St, Springfield'),
('Grace Lee', 'grace.lee@example.com', '555-789-1234', '123 Willow St, Springfield'),
('Henry Davis', 'henry.davis@example.com', '555-456-7890', '456 Maple St, Springfield'),
('Ivy Clark', 'ivy.clark@example.com', '555-234-5678', '789 Oak St, Springfield'),
('Jack Adams', 'jack.adams@example.com', '555-678-1234', '321 Pine St, Springfield');
