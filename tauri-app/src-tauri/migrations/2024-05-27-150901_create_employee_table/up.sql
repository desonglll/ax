-- Your SQL goes here
CREATE TABLE "employee_table" (
  "employee_id" serial,
  "first_name" varchar(255) DEFAULT '',
  "last_name" varchar(255) DEFAULT '',
  "employee_name" varchar(255) NOT NULL,
  "gender" varchar(255) DEFAULT '',
  "date_of_birth" timestamp DEFAULT CURRENT_TIMESTAMP,
  "hire_date" timestamp DEFAULT CURRENT_TIMESTAMP,
  "email" varchar(255) DEFAULT '',
  "phone_number" varchar(255) DEFAULT '',
  "department_id" int4 DEFAULT NULL,
  "job_title" varchar(255) DEFAULT '',
  "salary" FLOAT DEFAULT 0.0,
  "manager_id" int4 DEFAULT NULL,
  "address" varchar(255) DEFAULT '',
  "city" varchar(255) DEFAULT '',
  "state" varchar(255) DEFAULT '',
  "postal_code" varchar(255) DEFAULT '',
  "valid" bool DEFAULT true,
  "last_update" timestamp DEFAULT CURRENT_TIMESTAMP,
  "role_name" varchar(255) DEFAULT NULL,
  "role_id" int4 DEFAULT NULL,
  PRIMARY KEY ("employee_id"),
  CONSTRAINT "employee_name" UNIQUE ("employee_name")
);

-- 向 employee_table 表插入示例数据
INSERT INTO employee_table (first_name, last_name, employee_name, gender, date_of_birth, hire_date, email, phone_number, department_id, job_title, salary, manager_id, address, city, state, postal_code, role_name, role_id)
VALUES
('John', 'Doe', 'John Doe', 'Male', '1980-01-01', '2020-01-01', 'john.doe@example.com', '123-456-7890', 1, 'IT Specialist', 80000, NULL, '123 Main St', 'New York', 'NY', '10001', 'Admin', 1),
('Jane', 'Smith', 'Jane Smith', 'Female', '1985-05-15', '2019-02-01', 'jane.smith@example.com', '987-654-3210', 2, 'HR Manager', 90000, 1, '456 Elm St', 'Los Angeles', 'CA', '90001', 'User', 2),
('Mike', 'Johnson', 'Mike Johnson', 'Male', '1990-07-20', '2020-03-15', 'mike.johnson@example.com', '111-222-3333', 3, 'Financial Analyst', 85000, 2, '789 Oak St', 'Chicago', 'IL', '60601', 'Guest', 3),
('Sara', 'Williams', 'Sara Williams', 'Female', '1988-09-10', '2018-11-01', 'sara.williams@example.com', '444-555-6666', 4, 'Marketing Manager', 92000, 3, '101 Pine St', 'San Francisco', 'CA', '94101', 'User', 2),
('Tom', 'Brown', 'Tom Brown', 'Male', '1975-02-25', '2017-05-20', 'tom.brown@example.com', '777-888-9999', 5, 'Sales Director', 100000, 4, '202 Birch St', 'Seattle', 'WA', '98101', 'Admin', 1),
('Emily', 'Davis', 'Emily Davis', 'Female', '1992-12-05', '2021-04-12', 'emily.davis@example.com', '123-321-1234', 6, 'Product Manager', 87000, 5, '303 Cedar St', 'Austin', 'TX', '73301', 'User', 2),
('Robert', 'Martinez', 'Robert Martinez', 'Male', '1983-08-14', '2019-06-25', 'robert.martinez@example.com', '456-654-4567', 7, 'Operations Manager', 95000, 6, '404 Spruce St', 'Denver', 'CO', '80201', 'Admin', 1),
('Linda', 'Garcia', 'Linda Garcia', 'Female', '1979-11-30', '2016-03-05', 'linda.garcia@example.com', '789-987-7890', 8, 'Customer Support Lead', 83000, 7, '505 Maple St', 'Phoenix', 'AZ', '85001', 'User', 2),
('David', 'Wilson', 'David Wilson', 'Male', '1986-04-17', '2022-01-08', 'david.wilson@example.com', '321-432-3210', 9, 'Software Engineer', 88000, 8, '606 Walnut St', 'Boston', 'MA', '02101', 'Guest', 3),
('Jessica', 'Moore', 'Jessica Moore', 'Female', '1991-06-22', '2019-10-14', 'jessica.moore@example.com', '654-765-6543', 10, 'UX Designer', 86000, 9, '707 Chestnut St', 'Miami', 'FL', '33101', 'User', 2);


ALTER TABLE "department_table" ADD CONSTRAINT "manager_id" FOREIGN KEY ("manager_id") REFERENCES "employee_table" ("employee_id") ON DELETE CASCADE ON UPDATE CASCADE;
ALTER TABLE "department_table" ADD CONSTRAINT "parent_department_id" FOREIGN KEY ("parent_department_id") REFERENCES "department_table" ("department_id") ON DELETE CASCADE ON UPDATE CASCADE;
ALTER TABLE "employee_table" ADD CONSTRAINT "department_id" FOREIGN KEY ("department_id") REFERENCES "department_table" ("department_id") ON DELETE CASCADE ON UPDATE CASCADE;
ALTER TABLE "employee_table" ADD CONSTRAINT "manager_id" FOREIGN KEY ("manager_id") REFERENCES "employee_table" ("employee_id") ON DELETE CASCADE ON UPDATE CASCADE;
ALTER TABLE "employee_table" ADD CONSTRAINT "role_name" FOREIGN KEY ("role_name") REFERENCES "role_table" ("role_name") ON DELETE CASCADE ON UPDATE CASCADE;
ALTER TABLE "employee_table" ADD CONSTRAINT "role_id" FOREIGN KEY ("role_id") REFERENCES "role_table" ("role_id") ON DELETE CASCADE ON UPDATE CASCADE;
