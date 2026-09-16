# Ax Documentation

Ax is a small micro-blogging application: posts with attachments, comments,
Like/Dislike reactions, follows, a personalized feed, trending ranking and
in-app notifications.

It consists of exactly two deployable pieces plus a database:

1. **Backend** — a Rust HTTP API built on Actix-web and SQLx (`tweet_server/`).
2. **Frontend** — a Vue 3 single-page application served by nginx (`frontend/`).
3. **PostgreSQL** — the only external service. Sessions live in signed
   cookies, so there is no Redis; trending is a SQL query, so there is no
   separate recommendation service.

Setup, commands and configuration are covered in the repository
[README](https://github.com/desonglll/ax#readme). This book covers the parts
you need when changing the code: the request flow, the HTTP API and the
database schema with its triggers.
