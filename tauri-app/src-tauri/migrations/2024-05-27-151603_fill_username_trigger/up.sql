-- Your SQL goes here
CREATE OR REPLACE FUNCTION fill_username()
RETURNS TRIGGER AS $$
BEGIN
  -- 通过查询 users 表获取对应的用户名
  SELECT username INTO NEW.username FROM user_table WHERE user_id = NEW.user_id;
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER fill_username_trigger
BEFORE INSERT ON post_table
FOR EACH ROW EXECUTE FUNCTION fill_username();


CREATE OR REPLACE FUNCTION fill()
RETURNS TRIGGER AS $$
BEGIN
  -- 通过查询 users 表获取对应的用户名
  SELECT role_name INTO NEW.role_name FROM role_table WHERE role_id = NEW.role_id;
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER fill_trigger
BEFORE INSERT ON employee_table
FOR EACH ROW EXECUTE FUNCTION fill();
