-- Add migration script here
-- root: root
-- mike: 070011
-- joe: joe

INSERT INTO users ("user_name", "email", "password_hash", "full_name", "phone", "created_at", "updated_at", "last_login", "is_active", "is_admin", "profile_picture") VALUES
('root', 'root@example.com', '$2b$12$2Fn1PES7J8hwxs479IKRd.N2840/5Lh8oc53Lly4/I4hlR7l5dblq', 'root', '345-678-9012', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, true, true, NULL),
('mike', 'mike@example.com', '$2b$12$o/MDTP/oBct4J/yT2tGbUek5DZdBOrFWCvxy5UERnrda6.MniuHBu', 'Mike Shinoda', '123-456-7890', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, true, true, NULL),
('joe', 'joe@example.com', '$2b$12$d3.FbkzB0ID82Ziz0BnlGuVqL7J3iiZds0eczTdnWmkY6.9VqxQ/.', 'Joe Hanson', '234-567-8901', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, true, false, NULL),
('otis', 'otis@example.com', '$2b$12$SFv/qfIDkVhgdOnsTrUJTugAQGHH7qYsVn4jJiv3aUGdoMLNckOde', 'Otis', '123-456-7890', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, true, true, NULL);
