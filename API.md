# API Documentation

Base URL: `http://localhost:8000`

All routes are under `/api` scope.

## Response Format

```json
{
  "code": 200,
  "message": "description",
  "body": {
    "data": ...,
    "pagination": { "limit": 10, "offset": 0, "count": 100 }
  }
}
```

Error codes: 400 (bad request), 401 (unauthorized), 404 (not found), 500 (server error)

---

## Auth `/api/auth`

### POST /api/auth/login
Login and create session.

Request body:
```json
{
  "userName": "root",
  "password": "070011"
}
```

Response 200:
```json
{
  "code": 200,
  "message": "Logged in root.",
  "body": {
    "data": { User object },
    "pagination": null
  }
}
```

Response 401 (wrong password or no params):
```json
{
  "code": 401,
  "message": "Password validation failed for root.",
  "body": null
}
```

### GET /api/auth/login-check
Check login status.

Response 200 (logged in):
```json
{
  "code": 200,
  "message": "Welcome back! root",
  "body": null
}
```

Response 401 (not logged in):
```json
{
  "code": 401,
  "message": "Please Log in.",
  "body": null
}
```

### POST /api/auth/logout
Logout, clear session.

Response 200:
```json
"Logged out root successfully."
```

---

## User `/api/users`

### POST /api/users/post
Create a new user.

Request body:
```json
{
  "userName": "JohnDoe",
  "email": "johndoe@example.com",
  "password": "password123",
  "fullName": "John Doe",
  "phone": "1234567890",
  "isActive": true,
  "isAdmin": false,
  "profilePicture": null
}
```

Required: userName, email, password. Others optional.

Response 200:
```json
{
  "code": 200,
  "message": "Create User Success",
  "body": {
    "data": { User object },
    "pagination": null
  }
}
```

### GET /api/users/get
Get all users.

Response 200:
```json
{
  "code": 200,
  "message": "Get UserList Success",
  "body": {
    "data": [ User objects ],
    "pagination": null
  }
}
```

### GET /api/users/get/{user_id}
Get user by ID.

Response 200:
```json
{
  "code": 200,
  "message": "Get UserDetail Success",
  "body": {
    "data": { User object },
    "pagination": null
  }
}
```

### GET /api/users/profile
Get current logged-in user profile (from session).

Response 200:
```json
{
  "code": 200,
  "message": "Get `1` profile successfully.",
  "body": {
    "data": { User object },
    "pagination": null
  }
}
```

Response 401 (not logged in):
```json
{
  "code": 401,
  "message": "Please Login to store session into redis.",
  "body": null
}
```

### PUT /api/users/put/{user_id}
Update user. Requires session user_id matches path id.

Request body (all fields optional):
```json
{
  "userName": "NewName",
  "email": "new@email.com",
  "password": "newpass",
  "fullName": "New Full Name",
  "phone": "9876543210",
  "isActive": true,
  "isAdmin": false,
  "profilePicture": "uuid"
}
```

Response 200:
```json
{
  "code": 200,
  "message": "Update User Success",
  "body": {
    "data": { User object },
    "pagination": null
  }
}
```

Response 401 (not the same user):
```json
{
  "error_message": "Invalid user"
}
```

### DELETE /api/users/delete/{user_id}
Delete user. Requires session user_id matches path id.

Response 200: deleted user data.

Response 401 (not the same user):
```json
{
  "error_message": "Invalid user"
}
```

---

## Post `/api/posts`

### POST /api/posts/post
Create a new post. Requires login.

Request body:
```json
{
  "content": "Hello world",
  "replyTo": null,
  "userName": "John Doe"
}
```

userId is set from session automatically.

Response 200:
```json
{
  "code": 200,
  "message": "Insert Post Successful",
  "body": {
    "data": { Post object },
    "pagination": null
  }
}
```

Response 401 (not logged in):
```json
{
  "code": 401,
  "message": "Please Login",
  "body": null
}
```

### GET /api/posts/get
Get post list. Requires login.

Query params (required, at least provide limit/offset):
- limit (default 10)
- offset (default 0)
- order_by (default "id")
- sort (default "desc")

Example: `/api/posts/get?limit=5&offset=0`

Response 200:
```json
{
  "code": 200,
  "message": "Success",
  "body": {
    "data": [ Post objects ],
    "reaction": null,
    "pagination": { "limit": 5, "offset": 0, "count": 100 }
  }
}
```

### GET /api/posts/get/{post_id}
Get post by ID. Requires login.

Response 200:
```json
{
  "code": 200,
  "message": "Get Post Successful",
  "body": {
    "data": { Post object },
    "pagination": null
  }
}
```

### GET /api/posts/trending
Get recommended/trending posts for current user. Requires login.

Response 200:
```json
{
  "code": 200,
  "message": "Success",
  "body": {
    "data": [ Post objects ],
    "pagination": null
  }
}
```

### PUT /api/posts/put/{post_id}
Update post. Requires login.

Request body:
```json
{
  "content": "Updated content"
}
```

Response 200:
```json
{
  "code": 200,
  "message": "Update Successful",
  "body": {
    "data": { Post object },
    "pagination": null
  }
}
```

### DELETE /api/posts/delete/{post_id}
Delete post. Requires login.

Response 200:
```json
{
  "code": 200,
  "message": "Delete Successful",
  "body": {
    "data": { Post object },
    "pagination": null
  }
}
```

---

## Comment `/api/comments`

### POST /api/comments/post
Create a comment. Requires login.

Request body:
```json
{
  "content": "Nice post!",
  "replyTo": 1,
  "replyType": "post"
}
```

userId is set from session automatically.

Response 200:
```json
{
  "code": 200,
  "message": "Create Comment Successful",
  "body": {
    "data": { Comment object },
    "pagination": null
  }
}
```

### GET /api/comments/get?replyTo={id}&replyType=post
Get comments by query. Requires login.

Query params (all optional):
- commentId: filter by comment ID
- replyTo: filter by target post/comment ID
- replyToType: filter by type (default "post")

Response 200:
```json
{
  "code": 200,
  "message": "Get Comment Successful",
  "body": {
    "data": [ Comment objects ],
    "pagination": null
  }
}
```

### DELETE /api/comments/delete/{id}
Delete comment by ID. Requires login.

Response 200:
```json
{
  "code": 200,
  "message": "Delete Comment Successful",
  "body": {
    "data": { Comment object },
    "pagination": null
  }
}
```

---

## Reaction `/api/reactions`

### POST /api/reactions/post/like?toId={id}&toType=post
Like a post or comment. Requires login.

Query params:
- toId (required): target post/comment ID
- toType (optional, default "post"): "post" or "comment"

userId is set from session automatically.

Response 200:
```json
{
  "code": 200,
  "message": "Insert Like Successful",
  "body": {
    "data": { Reaction object },
    "pagination": null
  }
}
```

### POST /api/reactions/post/dislike?toId={id}&toType=post
Dislike a post or comment. Requires login.

Query params: same as like.

Response 200:
```json
{
  "code": 200,
  "message": "Insert Dislike Successful",
  "body": {
    "data": { Reaction object },
    "pagination": null
  }
}
```

### GET /api/reactions/get-table?toId={id}&toType=post
Get reaction counts (like/dislike) for a target.

Query params: any key-value for filtering.

Response 200:
```json
{
  "code": 200,
  "message": "Get Reaction Table Successful",
  "body": {
    "data": { "like": 5, "dislike": 2 },
    "pagination": null
  }
}
```

### GET /api/reactions/get?userId={id}
Get user's reactions. Requires login.

If userId not provided, uses session user_id.

Response 200:
```json
{
  "code": 200,
  "message": "Get Reactions Successful",
  "body": {
    "data": [ Reaction objects ],
    "pagination": null
  }
}
```

### DELETE /api/reactions/delete?reactionId={id}
Delete a reaction by ID.

Query params:
- reactionId (required)

Response 200:
```json
{
  "code": 200,
  "message": "Delete Reaction Successful",
  "body": {
    "data": { Reaction object },
    "pagination": null
  }
}
```

---

## File `/api/files`

### GET /api/files/all
Get all files (admin only). Requires login + admin.

Response 200: list of file objects.

Response 400 (not admin):
```json
{
  "AuthenticationError": "Not admin"
}
```

### GET /api/files/user?userId={id}
Get files for a specific user. Requires login.

Query params:
- userId (required)

Response 200: list of user's files.

### GET /api/files/pub
Get all public files. Requires login.

Response 200: list of public files.

### GET /api/files/download/{file_id}
Download a file by UUID.

If file is private, only the owner can access it.

Response 200: file binary with Content-Disposition header.

Response 401 (no permission):
```json
"User Not Permitted To Access This File"
```

### GET /api/files/stream/{file_id}
Stream a file (supports Range header for video).

If file is private, only the owner can access it.

Response 206: partial content with Content-Range header.

### POST /api/files/upload-public
Upload a public file. Requires login. Multipart form data.

Request: multipart/form-data with file field.

Response 200: uploaded file info.

### POST /api/files/upload-private
Upload a private file. Requires login. Multipart form data.

Request: multipart/form-data with file field.

Response 200: uploaded file info.

---

## Data Models

### User
```
id: i32
userName: string
email: string
passwordHash: string
fullName: string | null
phone: string | null
createdAt: datetime | null
updatedAt: datetime | null
lastLogin: datetime | null
isActive: bool
isAdmin: bool
profilePicture: uuid | null
```

### Post
```
id: i32
content: string
createdAt: datetime
updatedAt: datetime
userId: i32
replyTo: i32 | null
userName: string
likeCount: i32 | null
dislikeCount: i32 | null
engagementRate: f64 | null
```

### Comment
```
id: i32
content: string
replyTo: i32
userId: i32
userName: string
createdAt: datetime
updatedAt: datetime
replyToType: string
```

### Reaction
```
id: i32
userId: i32
toId: i32
createdAt: datetime
reactionName: string ("like" | "dislike")
toType: string ("post" | "comment")
```

### File
```
id: uuid
name: string
path: string
size: i64
contentType: string
createdAt: datetime | null
updatedAt: datetime | null
userId: i32
description: string | null
checksum: string
isDeleted: bool
isPub: bool
```
