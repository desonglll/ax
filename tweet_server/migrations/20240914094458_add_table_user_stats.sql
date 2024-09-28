-- Add migration script here
DROP TABLE IF EXISTS "user_stats";
CREATE TABLE "user_stats" (
    "user_id" INT NOT NULL,
    "liked_posts_count" INT NOT NULL DEFAULT 0,
    "followers_count" INT NOT NULL DEFAULT 0,
    "following_count" INT NOT NULL DEFAULT 0,
    "average_like_count" FLOAT NOT NULL DEFAULT 0.0,
    "average_comment_count" FLOAT NOT NULL DEFAULT 0.0,
    "recent_activity_score" FLOAT NOT NULL DEFAULT 0.0,
    "engagement_rate" FLOAT NOT NULL DEFAULT 0.0,
    FOREIGN KEY("user_id") REFERENCES "users"("id")
);
