import os
import datetime
from fastapi import FastAPI, HTTPException
from pydantic import BaseModel
import psycopg2
from psycopg2.extras import RealDictCursor

app = FastAPI(title="AX Recommendation Engine")

# Read database URL from env
DATABASE_URL = os.getenv("DATABASE_URL", "postgresql://postgres:password@localhost:5432/ax")
# Ensure PostgreSQL protocol compatibility
if DATABASE_URL.startswith("postgres://"):
    DATABASE_URL = DATABASE_URL.replace("postgres://", "postgresql://", 1)

class PredictPayload(BaseModel):
    liked_posts_count: int
    average_comment_count: float
    engagement_rate: float

@app.post("/predict")
def predict_endpoint(payload: PredictPayload):
    try:
        conn = psycopg2.connect(DATABASE_URL, cursor_factory=RealDictCursor)
    except Exception as e:
        print(f"Database connection error: {e}")
        # If DB connection fails, raise 500
        raise HTTPException(status_code=500, detail="Database connection failed")

    try:
        with conn.cursor() as cur:
            # Query posts that are top-level and not deleted
            cur.execute("""
                SELECT id, like_count, dislike_count, engagement_rate, created_at,
                       (SELECT COUNT(*) FROM comments WHERE comments.reply_to = posts.id) AS comment_count
                FROM posts
                WHERE reply_to IS NULL
            """)
            rows = cur.fetchall()
    except Exception as e:
        print(f"Error querying posts: {e}")
        raise HTTPException(status_code=500, detail="Failed to query posts")
    finally:
        conn.close()

    now = datetime.datetime.now(datetime.timezone.utc)
    scored_posts = []

    for row in rows:
        post_id = str(row['id'])
        likes = row['like_count'] or 0
        dislikes = row['dislike_count'] or 0
        engagement = float(row['engagement_rate'] or 0.0)
        comments = row['comment_count'] or 0
        created_at = row['created_at']

        # Ensure created_at has timezone
        if created_at.tzinfo is None:
            created_at = created_at.replace(tzinfo=datetime.timezone.utc)

        # Heuristic math
        hours = max(0.1, (now - created_at).total_seconds() / 3600.0)
        popularity = float(likes * 2.0 - dislikes + comments * 3.0)
        
        # User matching similarity factor (closer engagement rate = higher similarity)
        similarity = 1.0 - abs(payload.engagement_rate - engagement)
        
        # HN gravity formula
        score = (popularity * similarity) / ((hours + 2.0) ** 1.5)
        scored_posts.append((post_id, score))

    # Sort descending by score
    scored_posts.sort(key=lambda x: x[1], reverse=True)
    recommended_uuids = [post_id for post_id, _ in scored_posts[:10]]

    return {
        "message": "success",
        "data": recommended_uuids
    }

@app.get("/health")
def health():
    return {"status": "ok"}
