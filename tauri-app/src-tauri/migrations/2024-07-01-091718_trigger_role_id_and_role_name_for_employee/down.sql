-- This file should undo anything in `up.sql`
-- 删除触发器
DROP TRIGGER IF EXISTS set_role_name ON employee_table;

-- 删除函数
DROP FUNCTION IF EXISTS update_role_name();
