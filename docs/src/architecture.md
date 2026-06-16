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
