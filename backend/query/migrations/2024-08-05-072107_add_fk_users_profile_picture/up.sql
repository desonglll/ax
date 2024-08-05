-- Your SQL goes here
ALTER TABLE "users"
ADD FOREIGN KEY("profile_picture") REFERENCES "files"("id")
ON UPDATE NO ACTION ON DELETE NO ACTION;