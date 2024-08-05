-- Your SQL goes here
CREATE TABLE permission_table (
    permission_id SERIAL PRIMARY KEY,
    permission_name VARCHAR(255) NOT NULL,
    permission_description TEXT,
    resource VARCHAR(255) NOT NULL,
    action VARCHAR(50) NOT NULL,
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    created_by VARCHAR(255),
    updated_by VARCHAR(255),
    notes TEXT
);

-- 插入示例数据
INSERT INTO permission_table (
     permission_name, permission_description, resource, action, is_active, created_by, updated_by, notes
) VALUES
    ('View Users', 'Can view user details', 'User Management', 'View', TRUE, 'admin', 'admin', 'Initial setup'),
    ('Edit Users', 'Can edit user details', 'User Management', 'Edit', TRUE, 'admin', 'admin', 'Initial setup'),
    ('View Reports', 'Can view financial reports', 'Reports', 'View', TRUE, 'admin', 'admin', 'Initial setup');
