# Project Ax: Frontend Development Guidelines (v1.1)

Welcome! This document defines the architectural structure, coding standards, API conventions, and design guidelines for the frontend of Project Ax.

All frontend development MUST happen inside the `frontend/v1.1/` directory.

---

## 1. Tech Stack Overview

- **Runtime & Package Manager**: Bun
- **Build Tool**: Vite
- **Framework**: React 19 + TypeScript
- **Routing**: React Router v7 (explicit route-config style)
- **Styling**: TailwindCSS v4
- **HTTP Client**: Axios

---

## 2. Directory Structure

All application code resides in the `app/` directory:
- `app/root.tsx`: The root component containing the HTML container, layout framework, navigation bar, and error boundaries.
- `app/routes.ts`: The explicit routing configuration table mapping URLs to page components.
- `app/routes/`: Page views/routes (e.g., `home.tsx`, `post.tsx`, `profile.tsx`, `files.tsx`, `login.tsx`, `register.tsx`).
- `app/contexts/`: React Context providers (e.g., `AuthContext.tsx` for global login state).
- `app/utils/`: Common helpers and API wrappers (e.g., `api.ts`).
- `app/app.css`: Global style definitions including Tailwind imports.

---

## 3. Strict GNU-Style Design Aesthetics

Project Ax adopts a clean, content-focused, minimalist design modeled after the GNU website. Avoid modern flashy visual elements:
- **No Shadows**: Do not use `shadow-md` or custom shadow rules. All borders must be flat lines.
- **No Animations**: Do not use transitions, slide-ins, scale effects, or floating animations.
- **Standard Layouts**: Use basic linear tables, straightforward lists, and block-level forms.
- **Typography**: Utilize standard, clean sans-serif/serif fonts. Use standard headings (`h1`, `h2`, `h3`) with bold text.
- **Borders & Spacing**: Use clean black/grey borders (`border border-gray-300` or `dark:border-gray-700`) and standard margins/paddings.
- **Forms & Inputs**: Render plain `<input>`, `<textarea>`, and `<button>` elements with standard outline focus styles.

---

## 4. API Client & Session Integration

All HTTP requests to the backend server (running at `http://localhost:8000/api`) must be channeled through the centralized Axios client defined in `app/utils/api.ts`.
- **Credential Storage**: You MUST set `withCredentials: true` on the Axios client configuration. This ensures the backend `actix-session` Cookie is preserved and sent back/forth between the frontend and backend.
- **Error Propagation**: Catch and bubble up backend validation messages. If the backend returns an error message, extract the `message` field from the standard JSON body and display it as plain red text in the UI.

---

## 5. Global State & Routing

- **Authentication**: Use the `AuthContext` provider (via `useAuth()`) to inspect the current user object and access utility methods (`login`, `logout`, `register`).
- **Routing**: All paths must be registered in `app/routes.ts` before creating route components under `app/routes/`.
- **Navigation**: Use the standard React Router `<Link>` or `useNavigate()` hook to perform route transitions. Do not use standard `<a>` tags except for downloading files from external endpoints.

---

## 6. How to Extend the Frontend

1. **Route Mapping**: Add your route definition in `app/routes.ts`.
2. **Page View**: Create the corresponding page view file in `app/routes/` (e.g., `app/routes/my_page.tsx`).
3. **API Logic**: If new endpoints are invoked, add typed wrappers in `app/utils/api.ts`.
4. **Layout**: Style components using TailwindCSS classes adhering to the flat, clean GNU design requirements.
5. **Testing**: Run local validations to ensure typecheck compiles and builds cleanly.
