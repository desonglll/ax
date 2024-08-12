ALTER TABLE public.reactions
ADD CONSTRAINT reactions_user_id_fkey
FOREIGN KEY (user_id) REFERENCES public.users (id)
ON UPDATE CASCADE
ON DELETE SET NULL;

ALTER TABLE public.reactions
ADD CONSTRAINT reactions_post_id_fkey
FOREIGN KEY (post_id) REFERENCES public.posts (id)
ON UPDATE CASCADE
ON DELETE SET NULL;

ALTER TABLE public.reactions
ADD CONSTRAINT reactions_reaction_id_fkey
FOREIGN KEY (reaction_id) REFERENCES public.post_actions (id)
ON UPDATE CASCADE
ON DELETE SET NULL;

ALTER TABLE public.reactions
ADD CONSTRAINT reactions_reaction_name_fkey
FOREIGN KEY (reaction_name) REFERENCES public.post_actions (name)
ON UPDATE CASCADE
ON DELETE SET NULL;