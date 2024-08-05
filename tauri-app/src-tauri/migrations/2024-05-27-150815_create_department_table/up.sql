-- Your SQL goes here
CREATE TABLE "department_table" (
  "department_id" serial,
  "department_name" varchar(255) NOT NULL,
  "manager_id" int4 DEFAULT NULL,
  "location" varchar(255) DEFAULT '',
  "creation_date" timestamp DEFAULT CURRENT_TIMESTAMP,
  "last_update" timestamp DEFAULT CURRENT_TIMESTAMP,
  "description" varchar(255) DEFAULT '',
  "budget" int4 DEFAULT 0,
  "number_of_employees" int4 DEFAULT 0,
  "parent_department_id" int4 DEFAULT NULL,
  "email" varchar(255) DEFAULT '',
  "phone_number" varchar(255) DEFAULT '',
  "address" varchar(255) DEFAULT '',
  "city" varchar(255) DEFAULT '',
  "state" varchar(255) DEFAULT '',
  "postal_code" varchar(255) DEFAULT '',
  PRIMARY KEY ("department_id"),
  CONSTRAINT "department_name" UNIQUE ("department_name")
);

-- 向 department_table 表插入示例数据
INSERT INTO department_table (department_name, manager_id, location, description, budget, number_of_employees, email, phone_number, address, city, state, postal_code)
VALUES
('IT Department', 1, 'New York', 'Information Technology Department', 100000, 10, 'it@example.com', '123-456-7890', '123 Main St', 'New York', 'NY', '10001'),
('HR Department', 2, 'Los Angeles', 'Human Resources Department', 80000, 8, 'hr@example.com', '987-654-3210', '456 Elm St', 'Los Angeles', 'CA', '90001'),
('Finance Department', 3, 'Chicago', 'Finance Department', 120000, 12, 'finance@example.com', '111-222-3333', '789 Oak St', 'Chicago', 'IL', '60601'),
('Marketing Department', 4, 'San Francisco', 'Marketing and Advertising', 90000, 9, 'marketing@example.com', '444-555-6666', '101 Pine St', 'San Francisco', 'CA', '94101'),
('Sales Department', 5, 'Seattle', 'Sales and Customer Relations', 110000, 11, 'sales@example.com', '777-888-9999', '202 Birch St', 'Seattle', 'WA', '98101'),
('Product Department', 6, 'Austin', 'Product Development and Management', 95000, 10, 'product@example.com', '123-321-1234', '303 Cedar St', 'Austin', 'TX', '73301'),
('Operations Department', 7, 'Denver', 'Operations and Logistics', 105000, 12, 'operations@example.com', '456-654-4567', '404 Spruce St', 'Denver', 'CO', '80201'),
('Support Department', 8, 'Phoenix', 'Customer Support and Services', 85000, 8, 'support@example.com', '789-987-7890', '505 Maple St', 'Phoenix', 'AZ', '85001'),
('Engineering Department', 9, 'Boston', 'Engineering and Development', 115000, 14, 'engineering@example.com', '321-432-3210', '606 Walnut St', 'Boston', 'MA', '02101'),
('Design Department', 10, 'Miami', 'Design and User Experience', 90000, 7, 'design@example.com', '654-765-6543', '707 Chestnut St', 'Miami', 'FL', '33101');
