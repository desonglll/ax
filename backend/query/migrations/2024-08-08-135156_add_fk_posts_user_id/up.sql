-- Your SQL goes here
ALTER TABLE public.posts
ADD FOREIGN KEY("user_id") REFERENCES public.users("id")
ON UPDATE NO ACTION ON DELETE NO ACTION;