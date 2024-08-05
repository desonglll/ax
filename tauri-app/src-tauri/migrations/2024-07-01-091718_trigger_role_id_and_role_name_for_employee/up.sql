-- Your SQL goes here
-- 创建函数
CREATE OR REPLACE FUNCTION update_role_name()
RETURNS TRIGGER AS $$
BEGIN
  -- 查找 role_name 对应的 role_id
  SELECT role_name INTO NEW.role_name FROM role_table WHERE role_id= NEW.role_id;
  
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- 创建触发器，在插入或更新时调用函数
CREATE TRIGGER set_role_name
BEFORE INSERT OR UPDATE ON employee_table
FOR EACH ROW EXECUTE FUNCTION update_role_name();
