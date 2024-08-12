-- Your SQL goes here
ALTER TABLE public.posts
ADD FOREIGN KEY("reply_to") REFERENCES public.posts("id")
ON UPDATE NO ACTION ON DELETE NO ACTION;