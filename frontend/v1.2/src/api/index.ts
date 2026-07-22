import axios from "axios";
import { api } from "./client";
import type { ApiResponse, Comment, FileRecord, FollowStats, Notification, Post, Reaction, ReactionTable, SystemStats, User } from "../types";

export const authApi = {
  login: (userName: string, password: string) => api.post<ApiResponse<User>>("/auth/login", { userName, password }).then(r => r.data),
  check: () => api.get<ApiResponse<User>>("/auth/login-check").then(r => r.data),
  logout: () => api.post<ApiResponse<string>>("/auth/logout").then(r => r.data),
};

export const userApi = {
  register: (payload: { userName: string; email: string; password: string }) => api.post<ApiResponse<User>>("/users/post", payload).then(r => r.data),
  list: (params?: { limit?: number; offset?: number }) => api.get<ApiResponse<User[]>>("/users/get", { params }).then(r => r.data),
  get: (id: number) => api.get<ApiResponse<User>>(`/users/get/${id}`).then(r => r.data),
  profile: () => api.get<ApiResponse<User>>("/users/profile").then(r => r.data),
  update: (id: number, payload: Partial<Pick<User, "userName" | "email" | "fullName" | "phone" | "isActive" | "isAdmin">> & { password?: string }) => api.put<ApiResponse<User>>(`/users/put/${id}`, payload).then(r => r.data),
  delete: (id: number) => api.delete<ApiResponse<User>>(`/users/delete/${id}`).then(r => r.data),
};

export const followApi = {
  follow: (id: number) => api.post<ApiResponse<FollowStats>>(`/users/${id}/follow`).then(r => r.data),
  unfollow: (id: number) => api.delete<ApiResponse<FollowStats>>(`/users/${id}/follow`).then(r => r.data),
  stats: (id: number) => api.get<ApiResponse<FollowStats>>(`/users/${id}/follow-stats`).then(r => r.data),
  followers: (id: number, params?: { limit?: number; offset?: number }) => api.get<ApiResponse<User[]>>(`/users/${id}/followers`, { params }).then(r => r.data),
  following: (id: number, params?: { limit?: number; offset?: number }) => api.get<ApiResponse<User[]>>(`/users/${id}/following`, { params }).then(r => r.data),
  feed: (params?: { limit?: number; offset?: number }) => api.get<ApiResponse<Post[]>>("/posts/feed", { params }).then(r => r.data),
};

export const postApi = {
  create: (payload: { content: string; title: string; attachments?: string[] }) => api.post<ApiResponse<Post>>("/posts/post", payload).then(r => r.data),
  list: (params?: { limit?: number; offset?: number; order_by?: string; sort?: string; search?: string; user_id?: number }) => api.get<ApiResponse<Post[]>>("/posts/get", { params }).then(r => r.data),
  get: (id: string) => api.get<ApiResponse<Post>>(`/posts/get/${id}`).then(r => r.data),
  trending: () => api.get<ApiResponse<Post[]>>("/posts/trending").then(r => r.data),
  update: (id: string, payload: { content: string; title?: string; attachments?: string[] }) => api.put<ApiResponse<Post>>(`/posts/put/${id}`, payload).then(r => r.data),
  delete: (id: string) => api.delete<ApiResponse<Post>>(`/posts/delete/${id}`).then(r => r.data),
};

export const commentApi = {
  create: (payload: { content: string; replyTo: string; attachments?: string[] }) => api.post<ApiResponse<Comment>>("/comments/post", payload).then(r => r.data),
  list: (params: { commentId?: string; replyTo?: string; limit?: number; offset?: number }) => api.get<ApiResponse<Comment[]>>("/comments/get", { params }).then(r => r.data),
  delete: (id: string) => api.delete<ApiResponse<Comment>>(`/comments/delete/${id}`).then(r => r.data),
};

export const reactionApi = {
  like: (toId: string, toType: "post" | "comment") => api.post<ApiResponse<Reaction>>("/reactions/post/like", null, { params: { toId, toType } }).then(r => r.data),
  dislike: (toId: string, toType: "post" | "comment") => api.post<ApiResponse<Reaction>>("/reactions/post/dislike", null, { params: { toId, toType } }).then(r => r.data),
  table: (toId: string, toType: "post" | "comment") => api.get<ApiResponse<ReactionTable>>("/reactions/get-table", { params: { toId, toType } }).then(r => r.data),
  list: (params?: { toId?: string; toType?: "post" | "comment"; reactionName?: "Like" | "Dislike"; userId?: number }) => api.get<ApiResponse<Reaction[]>>("/reactions/get", { params }).then(r => r.data),
  delete: (reactionId: number) => api.delete<ApiResponse<Reaction>>("/reactions/delete", { params: { reactionId } }).then(r => r.data),
};

export const fileApi = {
  all: () => api.get<ApiResponse<FileRecord[]>>("/files/all").then(r => r.data),
  user: (userId: number) => api.get<ApiResponse<FileRecord[]>>("/files/user", { params: { userId } }).then(r => r.data),
  public: () => api.get<ApiResponse<FileRecord[]>>("/files/pub").then(r => r.data),
  upload: (formData: FormData, isPublic: boolean) => api.post<ApiResponse<FileRecord[]>>(isPublic ? "/files/upload-public" : "/files/upload-private", formData).then(r => r.data),
  downloadUrl: (id: string) => `${api.defaults.baseURL}/files/download/${id}`,
  streamUrl: (id: string) => `${api.defaults.baseURL}/files/stream/${id}`,
};

export const notificationApi = {
  list: (params?: { limit?: number; offset?: number }) => api.get<ApiResponse<Notification[]>>("/notifications/get", { params }).then(r => r.data),
  unread: () => api.get<ApiResponse<number>>("/notifications/unread-count").then(r => r.data),
  read: (id: number) => api.post<ApiResponse<void>>(`/notifications/read/${id}`).then(r => r.data),
  readAll: () => api.post<ApiResponse<number>>("/notifications/read-all").then(r => r.data),
};

export const statsApi = {
  get: () => axios.get<Record<string, unknown>>("/stats").then(({ data }) => ({
    requestCount: Number(data.request_count || 0),
    responseTimes: (data.response_times || {}) as SystemStats["responseTimes"],
  })),
};
