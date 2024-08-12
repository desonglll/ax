-- Your SQL goes here
ALTER TABLE public.users
ADD FOREIGN KEY("profile_picture") REFERENCES public.files("id")
ON UPDATE NO ACTION ON DELETE NO ACTION;