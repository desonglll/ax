# API Reference

This chapter specifies the HTTP endpoints available in Project Ax. The base URL is `http://localhost:8000`. All routes are scoped under the `/api` prefix.

## Response Format

Every API response follows a unified JSON format:

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

Standard error codes are 400 (bad request), 401 (unauthorized), 404 (not found), and 500 (internal server error).

---

## Auth Endpoints (`/api/auth`)

### POST /api/auth/login
Authenticates a user and starts a session.
- **Request Body**:
  ```json
  {
    "userName": "root",
    "password": "password"
  }
  ```
- **Response (200 OK)**:
  Returns user details on success.
- **Response (401 Unauthorized)**:
  Returned when password validation fails.

### GET /api/auth/login-check
Checks the login status of the current session.
- **Response (200 OK)**:
  Returns welcome message if authenticated.
- **Response (401 Unauthorized)**:
  Returns authorization prompt if not authenticated.

### POST /api/auth/logout
Clears the session of the authenticated user.
- **Response (200 OK)**:
  Returns confirmation message.

---

## User Endpoints (`/api/users`)

### POST /api/users/post
Creates a new user account.
- **Request Body**:
  ```json
  {
    "userName": "JohnDoe",
    "email": "johndoe@example.com",
    "password": "password123"
  }
  ```
- **Response (200 OK)**:
  Returns the created user record.

### GET /api/users/get
Retrieves the list of all users.
- **Response (200 OK)**:
  Returns user array.

### GET /api/users/get/{user_id}
Retrieves user details by ID.
- **Response (200 OK)**:
  Returns user record.

### GET /api/users/profile
Retrieves the profile of the current logged-in user.
- **Response (200 OK)**:
  Returns user record if authenticated.

### PUT /api/users/put/{user_id}
Updates user details. The request must originate from the matching user or an administrator.
- **Response (200 OK)**:
  Returns the updated user record.

### DELETE /api/users/delete/{user_id}
Deletes the specified user account.
- **Response (200 OK)**:
  Returns the deleted user record.

---

## Post Endpoints (`/api/posts`)

### POST /api/posts/post
Creates a new post. Requires authentication.
- **Request Body**:
  ```json
  {
    "title": "Catchy Title (Optional)",
    "content": "Hello world",
    "attachments": [
      "d78f2379-cb4a-4467-bc18-97c7e5cb2fb6"
    ]
  }
  ```
- **Response (200 OK)**:
  Returns the created post record with its attachments list.
  ```json
  {
    "id": "76495db6-0cb2-4a00-9844-4638706d87e0",
    "title": "Catchy Title (Optional)",
    "content": "Hello world",
    "createdAt": "2026-06-18T18:00:00Z",
    "updatedAt": "2026-06-18T18:00:00Z",
    "userId": 1,
    "replyTo": null,
    "userName": "root",
    "likeCount": 0,
    "dislikeCount": 0,
    "engagementRate": 0.0,
    "attachments": [
      {
        "id": "d78f2379-cb4a-4467-bc18-97c7e5cb2fb6",
        "name": "image.png",
        "path": "/uploads/image.png",
        "size": 1024,
        "mime": "image/png",
        "userId": 1,
        "userName": "root",
        "createdAt": "2026-06-18T18:00:00Z",
        "updatedAt": "2026-06-18T18:00:00Z",
        "isDeleted": false,
        "isPublic": true,
        "md5": "d41d8cd98f00b204e9800998ecf8427e",
        "postId": "76495db6-0cb2-4a00-9844-4638706d87e0"
      }
    ]
  }
  ```

### GET /api/posts/get
Retrieves the paginated list of posts.
- **Query Parameters**:
  - `limit` (default: 10)
  - `offset` (default: 0)
  - `order_by` (default: "id")
  - `sort` (default: "desc")
- **Response (200 OK)**:
  Returns a list of post details (each including an `attachments` array) and a pagination metadata object.

### GET /api/posts/get/{post_id}
Retrieves a post by its ID.
- **Response (200 OK)**:
  Returns the post detail record including its `attachments` array.

### GET /api/posts/trending
Retrieves trending posts recommended for the current user.
- **Response (200 OK)**:
  Returns recommended post detail array including nested `attachments`.

### PUT /api/posts/put/{post_id}
Updates the title or content of a post. The request must originate from the post owner or an administrator.
- **Request Body**:
  ```json
  {
    "title": "New Title",
    "content": "Updated content"
  }
  ```
- **Response (200 OK)**:
  Returns the updated post detail record with its `attachments`.

### DELETE /api/posts/delete/{post_id}
Deletes the post. The request must originate from the post owner or an administrator.
- **Response (200 OK)**:
  Returns the deleted post record.

---

## Comment Endpoints (`/api/comments`)

### POST /api/comments/post
Creates a comment replying to a post or another comment.
- **Request Body**:
  ```json
  {
    "content": "Nice post!",
    "replyTo": 1,
    "replyType": "post"
  }
  ```
- **Response (200 OK)**:
  Returns the created comment record.

### GET /api/comments/get
Retrieves comments matching the query. Supports pagination.
- **Query Parameters**:
  - `commentId`: filter by comment ID
  - `replyTo`: filter by target ID
  - `replyToType` (default: "post"): filter by reply target type
  - `limit` (default: 10)
  - `offset` (default: 0)
- **Response (200 OK)**:
  Returns a list of comments and a pagination metadata object.

### DELETE /api/comments/delete/{id}
Deletes the specified comment. The request must originate from the comment owner or an administrator.
- **Response (200 OK)**:
  Returns the deleted comment record.

---

## Reaction Endpoints (`/api/reactions`)

### POST /api/reactions/post/like
Likes a post or comment.
- **Query Parameters**:
  - `toId`: target ID
  - `toType` (default: "post"): "post" or "comment"
- **Response (200 OK)**:
  Returns the reaction record.

### POST /api/reactions/post/dislike
Dislikes a post or comment.
- **Query Parameters**:
  - `toId`: target ID
  - `toType` (default: "post"): "post" or "comment"
- **Response (200 OK)**:
  Returns the reaction record.

### GET /api/reactions/get-table
Gets reaction counts (likes/dislikes) for a target.
- **Query Parameters**:
  - `toId`: target ID
  - `toType`: "post" or "comment"
- **Response (200 OK)**:
  Returns counts object: `{ "like": X, "dislike": Y }`.

### DELETE /api/reactions/delete
Deletes a reaction.
- **Query Parameters**:
  - `reactionId`: reaction ID
- **Response (200 OK)**:
  Returns the deleted reaction record.

---

## File Endpoints (`/api/files`)

### GET /api/files/all
Retrieves all files. Requires administrator privilege.
- **Response (200 OK)**:
  Returns file record list.

### GET /api/files/user
Retrieves files uploaded by a user.
- **Query Parameters**:
  - `userId`: owner's user ID
- **Response (200 OK)**:
  Returns file record list.

### GET /api/files/pub
Retrieves all public files.
- **Response (200 OK)**:
  Returns file record list.

### GET /api/files/download/{file_id}
Downloads a file by its UUID.
- **Response (200 OK)**:
  Returns file binary stream.

### POST /api/files/upload-public
Uploads a public file. Multipart form data.
- **Response (200 OK)**:
  Returns uploaded file record.

### POST /api/files/upload-private
Uploads a private file. Multipart form data.
- **Response (200 OK)**:
  Returns uploaded file record.
