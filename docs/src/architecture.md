# Architecture & Design

This chapter describes the modular architecture and structural layout of the Project Ax backend server.

## Layered Design

The backend server is structured into several distinct layers to decouple concerns and facilitate maintenance:

1. **Routing Layer (`src/routes`)**:
   Defines the HTTP route scopes and registers route handlers under scoped endpoints. For example, `src/routes/post.rs` configures routes for `/api/posts/*`.

2. **Handling Layer (`src/handlers`)**:
   Contains controller logic. Handlers parse payload data, validate session authentication status, check authorization boundaries, invoke database access functions, and format standard API responses.

3. **Database Access Layer (`src/dbaccess`)**:
   Provides abstractions for SQL database operations. Queries are parameterized and compile-time verified using SQLx. 

4. **Model Layer (`src/models`)**:
   Declares serialization and deserialization structures (Data Transfer Objects) for request payloads and database entity representations.

5. **Extractors Layer (`src/extractors`)**:
   Implements custom Actix-web request extractors (such as session-related utilities in `session.rs`) and wrappers for unified API JSON formats.

## Data Flow

A typical request progresses through the following stages:

1. The client issues an HTTP request.
2. The Actix-web server matches the request to a route defined in `src/routes`.
3. The routing engine invokes the matched handler in `src/handlers`.
4. The handler extracts session and query parameters safely.
5. Authentication is validated using the session extractor.
6. The handler makes a parameterized database call through the appropriate module in `src/dbaccess`.
7. SQLx executes the query against the PostgreSQL database.
8. The handler returns a formatted JSON response wrapped in `ApiResponse`.

---

## Background Services & Task Queues

Project Ax includes lightweight background services to handle asynchronous, non-blocking operations like AI-powered content completion.

### AI Post Title Completion Queue

1. **Description**:
   An asynchronous in-memory task queue implemented using `tokio::sync::mpsc::unbounded_channel`. 
   - A background thread runs the `QueueWorker` loop, receiving post UUIDs.
   - For each UUID, the worker retrieves the post content, invokes the Local AI library (`ai` crate), and writes the generated title back to PostgreSQL.
   - When a post is created via `/api/posts/post` without a title, the post ID is pushed onto the queue.
   - On server startup, a database scanner (`scan_and_enqueue_empty_titles`) runs once, enqueuing all historical posts without a title (`title = ''` or `title IS NULL`).

2. **Usage**:
   - Create a post without specifying a title. The title will automatically populate in the background within seconds.
   - Restart the server. Any post that was left with a blank title will be automatically processed.

3. **Debugging & Monitoring**:
   - **Console Logs**: The queue worker outputs system logs at each processing step:
     - `Queue Worker: Processing post_id=...`
     - `Queue Worker: Calling AI API to generate title...`
     - `Queue Worker: Successfully updated title for post_id=...`
   - **Database State**: Inspect queue progress by querying for empty titles:
     ```sql
     SELECT id, title, content FROM posts WHERE title = '';
     ```
   - **Isolated Testing**: Run the mock-server integration test suite:
     ```bash
     cargo nextest run services::queue::tests::test_queue_worker_process_post --nocapture
     ```

