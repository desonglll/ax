import { api } from "./client";
import type {
  ApiResponse,
  Comment,
  FileRecord,
  FollowStats,
  Notification,
  Post,
  Reaction,
  ReactionKind,
  ReactionTargetType,
  User,
} from "../types";

type Page = { limit?: number; offset?: number };

const unwrap = <T>(promise: Promise<{ data: ApiResponse<T> }>) => promise.then(r => r.data);

export const authApi = {
  login: (userName: string, password: string) => unwrap(api.post<ApiResponse<User>>("/auth/login", { userName, password })),
  logout: () => unwrap(api.post<ApiResponse<never>>("/auth/logout")),
  me: () => unwrap(api.get<ApiResponse<User>>("/auth/me")),
};

export const userApi = {
  register: (payload: { userName: string; email: string; password: string }) => unwrap(api.post<ApiResponse<User>>("/users", payload)),
  list: (params?: Page) => unwrap(api.get<ApiResponse<User[]>>("/users", { params })),
  get: (id: number) => unwrap(api.get<ApiResponse<User>>(`/users/${id}`)),
  update: (id: number, payload: Partial<Pick<User, "userName" | "email" | "fullName" | "phone" | "isActive" | "isAdmin">> & { password?: string }) =>
    unwrap(api.put<ApiResponse<User>>(`/users/${id}`, payload)),
  delete: (id: number) => unwrap(api.delete<ApiResponse<User>>(`/users/${id}`)),
};

export const followApi = {
  follow: (id: number) => unwrap(api.post<ApiResponse<FollowStats>>(`/users/${id}/follow`)),
  unfollow: (id: number) => unwrap(api.delete<ApiResponse<FollowStats>>(`/users/${id}/follow`)),
  stats: (id: number) => unwrap(api.get<ApiResponse<FollowStats>>(`/users/${id}/follow-stats`)),
  followers: (id: number, params?: Page) => unwrap(api.get<ApiResponse<User[]>>(`/users/${id}/followers`, { params })),
  following: (id: number, params?: Page) => unwrap(api.get<ApiResponse<User[]>>(`/users/${id}/following`, { params })),
};

export const postApi = {
  create: (payload: { content: string; title?: string; attachments?: string[] }) => unwrap(api.post<ApiResponse<Post>>("/posts", payload)),
  list: (params?: Page & { order_by?: string; sort?: "asc" | "desc"; search?: string; user_id?: number }) => unwrap(api.get<ApiResponse<Post[]>>("/posts", { params })),
  feed: (params?: Page) => unwrap(api.get<ApiResponse<Post[]>>("/posts/feed", { params })),
  trending: (params?: Page) => unwrap(api.get<ApiResponse<Post[]>>("/posts/trending", { params })),
  get: (id: string) => unwrap(api.get<ApiResponse<Post>>(`/posts/${id}`)),
  update: (id: string, payload: { content?: string; title?: string; attachments?: string[] }) => unwrap(api.put<ApiResponse<Post>>(`/posts/${id}`, payload)),
  delete: (id: string) => unwrap(api.delete<ApiResponse<Post>>(`/posts/${id}`)),
};

export const commentApi = {
  create: (payload: { content: string; replyTo: string; attachments?: string[] }) => unwrap(api.post<ApiResponse<Comment>>("/comments", payload)),
  list: (params: Page & { replyTo: string }) => unwrap(api.get<ApiResponse<Comment[]>>("/comments", { params })),
  delete: (id: string) => unwrap(api.delete<ApiResponse<Comment>>(`/comments/${id}`)),
};

export const reactionApi = {
  set: (toId: string, toType: ReactionTargetType, reaction: ReactionKind) => unwrap(api.put<ApiResponse<Reaction>>("/reactions", { toId, toType, reaction })),
  remove: (toId: string, toType: ReactionTargetType) => unwrap(api.delete<ApiResponse<Reaction>>("/reactions", { params: { toId, toType } })),
};

export const fileApi = {
  list: (scope: "public" | "mine" | "all") => unwrap(api.get<ApiResponse<FileRecord[]>>("/files", { params: { scope } })),
  upload: (files: File[], options: { isPublic?: boolean; description?: string } = {}) => {
    const form = new FormData();
    files.forEach(file => form.append("files", file));
    if (options.description) form.append("description", options.description);
    return unwrap(api.post<ApiResponse<FileRecord[]>>("/files", form, { params: { public: options.isPublic ?? true } }));
  },
  downloadUrl: (id: string) => `${api.defaults.baseURL}/files/${id}/download`,
  streamUrl: (id: string) => `${api.defaults.baseURL}/files/${id}/stream`,
};

export const notificationApi = {
  list: (params?: Page) => unwrap(api.get<ApiResponse<Notification[]>>("/notifications", { params })),
  unread: () => unwrap(api.get<ApiResponse<number>>("/notifications/unread-count")),
  read: (id: number) => unwrap(api.post<ApiResponse<never>>(`/notifications/${id}/read`)),
  readAll: () => unwrap(api.post<ApiResponse<number>>("/notifications/read-all")),
};
