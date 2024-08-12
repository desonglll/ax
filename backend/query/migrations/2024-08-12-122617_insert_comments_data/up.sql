-- Your SQL goes here
INSERT INTO comments (content, reply_to, user_id)
VALUES
('This is a comment on the first post', 1, 1),
('Another comment on the first post', 1, 2),
('A comment on the second post', 2, 3);

INSERT INTO comment_reactions (user_id, comment_id, reaction_id, reaction_name)
VALUES
(1, 1, 1, 'like'),  -- Alice likes the first comment
(2, 1, 1, 'like'),  -- Bob likes the first comment
(3, 2, 2, 'dislike');  -- Charlie dislikes the second comment
