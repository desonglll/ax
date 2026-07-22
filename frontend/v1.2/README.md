# Ax Frontend v1.2

Vue 3 frontend for Project Ax, built with Vite, Bun, TypeScript, Tailwind CSS 4, daisyUI 5, Pinia, Vue Router, Axios, and Lucide icons.

## Development

```bash
bun install
bun run dev
```

The Vite proxy reads the backend's actual port from the repository-root `.server-port` file. Start the complete stack from the repository root with:

```bash
just start
```

## Validation

```bash
bun run typecheck
bun run build
```

## Feature coverage

- Session restoration, login, logout, and registration
- Public and following feeds, search, trending posts, publishing, editing, and deletion
- Post and comment reactions with exclusive Like/Dislike state
- Post details, comments, attachments, image previews, video streaming, and downloads
- Profiles, profile editing, follow/unfollow, followers, and following lists
- People directory and administrator account controls
- Notification dropdown, complete notification center, read state, and pagination
- Public, private, personal, and administrator file views with multipart upload
- Administrator system request and latency telemetry
