# Frontend Design & Guide

This chapter documents the structure, layout conventions, API integrations, and developer guidelines for the Project Ax frontend client.

---

## Technical Stack

The frontend client in `frontend/v1.1` is built on a modern, robust, and lightweight JavaScript/TypeScript ecosystem:
- **Package Manager & Runtime**: Bun
- **Build Engine**: Vite
- **UI Framework**: React 19
- **Routing Engine**: React Router v7 (configured using explicit config routing)
- **Styling**: TailwindCSS v4
- **HTTP client**: Axios

---

## GNU-Style Layout Philosophy & daisyUI Integration

Project Ax adopts a clean, content-focused, lightweight design layout inspired by classic GNU project pages (such as gnu.org). To make frontend development more robust and maintainable, the layout utilizes **daisyUI** components (like `card`, `stats`, `table`, `btn`, `input`, and `alert`) configured to stay flat, clean, and completely non-flashy:
1. **Flat Design**: Zero card shadows. Containers utilize the `card card-border` modifier and flat borders (`border-base-300`) for structural separation.
2. **Static Viewports**: Zero transition animations, slide effects, or hover-scaling. Clicking actions or navigating routes transitions the view instantly.
3. **Clean Forms & Inputs**: Form fields render daisyUI input and textarea components with clean outlines (`input-bordered`, `textarea-bordered`) and standard focus outlines. Buttons use compact sizes (`btn-sm`, `btn-xs`) in neutral or outline styles (`btn-neutral`, `btn-outline`).
4. **Contrast alerts & stats**: Feedback and errors use standard alert components (`alert alert-error`, `alert alert-success`) and user statistics utilize the stats grid (`stats stats-vertical sm:stats-horizontal`) to align layout data.

---

## Client Directories & Routing

All source files are nested within the `app/` folder:
- **`root.tsx`**: Sets up global HTML shell, attaches `<AuthProvider>` for auth context, and renders the master header/footer navigation layout. The "Files" navbar tab link is restricted to administrator users (`user?.isAdmin` is true) to prevent standard users from seeing the files list. It also features a global floating scroll-to-top button (`[↑ Top]`) that becomes visible once the viewport is scrolled down beyond 300px.
- **`routes.ts`**: The central routing registry table.
- **`app.css`**: Configures TailwindCSS imports and basic color schemes.
- **`contexts/AuthContext.tsx`**: Exposes authentication actions (`login`, `logout`, `register`) and keeps the current user reference synced.
- **`components/`**: Houses reusable sub-components, such as:
  - `PostItem.tsx`: Renders a post, handles deletions, loads reactions (Likes/Dislikes), and displays linked attachments (images and videos are previewed inline; other file types are displayed as download links with corresponding icons).
  - `CommentNode.tsx`: Recursively fetches and displays comments and replies in a threaded view.
- **`routes/`**: Contains the route components:
  - `home.tsx`: Displays the main timeline list (with pagination) and the post creator interface. The post creator supports picking multiple attachments (accumulated across selection actions) with a visual list, individual "[Remove]" buttons, and inline previews restricted strictly to images and videos. Files are uploaded concurrently upon submission.
  - `trending.tsx`: Displays recommended trending posts.
  - `post.tsx`: Displays the post detail and nested comment threads.
  - `profile.tsx`: Displays user details, locally computes user statistics (total posts, average likes/dislikes, engagement rate), and handles profile updates. Admins can view and delete users here.
  - `files.tsx`: Manages public/private file uploads, file lists, and downloads. Direct access to this route is restricted: normal users see an "Access Denied" view, securing the overall files registry list.
  - `login.tsx` & `register.tsx`: Auth forms.

---

## API Client and State Credentials

All HTTP traffic to the backend server (at `http://localhost:8000/api`) uses the Axios client configured in `app/utils/api.ts`.
- **CORS Credentials**: The client must enable `withCredentials: true`. This allows the browser to receive, store, and return the `actix-session` cookie, which is required for authentication state verification.
- **CamelCase Properties**: Because the backend uses `#[serde(rename_all = "camelCase")]`, all JSON payloads are serialized/deserialized in camelCase. TypeScript interfaces in `api.ts` must use camelCase naming conventions (e.g. `userName`, `likeCount`, `createdAt`, `userId`) to match.

---

## Build and Compilation Instructions

For local development or CI check verification:
- **Install Dependencies**: `bun install`
- **Start Development Server**: `bun run dev`
- **Run TypeScript Checks**: `bun run typecheck`
- **Build Production Bundle**: `bun run build`
