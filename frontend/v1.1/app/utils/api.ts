import axios from "axios";

// Standard API Response wrapper
export interface ApiResponse<T> {
  code: number;
  message: string;
  body: {
    data: T;
    pagination?: {
      limit: number;
      offset: number;
      count: number;
    };
  };
}

// Data Entities (matching camelCase serialization from Rust)
export interface User {
  id: number;
  userName: string;
  email: string;
  fullName?: string;
  phone?: string;
  createdAt?: string;
  updatedAt?: string;
  lastLogin?: string;
  isActive: boolean;
  isAdmin: boolean;
  profilePicture?: string;
}

export interface Post {
  id: number;
  content: string;
  createdAt: string;
  updatedAt: string;
  userId: number;
  replyTo?: number;
  userName: string;
  likeCount?: number;
  dislikeCount?: number;
  engagementRate?: number;
}

export interface Comment {
  id: number;
  content: string;
  replyTo: number;
  userId: number;
  userName: string;
  createdAt: string;
  updatedAt: string;
  replyToType: "post" | "comment";
}

export interface Reaction {
  id: number;
  userId: number;
  toId: number;
  createdAt: string;
  reactionName: "Like" | "Dislike";
  toType: "post" | "comment";
}

export interface FileRecord {
  id: string; // Uuid
  name: string;
  path: string;
  size: number;
  contentType: string;
  createdAt?: string;
  updatedAt?: string;
  userId: number;
  description?: string;
  checksum: string;
  isDeleted: boolean;
  isPub: boolean;
}

export interface UserStats {
  id: number;
  userId: number;
  postCount: number;
  commentCount: number;
  averageLikes: number;
  averageCommentCount: number;
  engagementRate: number;
  createdAt: string;
  updatedAt: string;
}

// Axios Instance
const api = axios.create({
  baseURL: (import.meta.env.VITE_API_URL as string) || "http://localhost:8000/api",
  withCredentials: true,
});

// ============================================================================
// Auth Endpoints
// ============================================================================
export const authApi = {
  login: async (userName: string, password: string): Promise<ApiResponse<User>> => {
    const response = await api.post("/auth/login", { userName, password });
    return response.data;
  },
  loginCheck: async (): Promise<ApiResponse<User>> => {
    const response = await api.get("/auth/login-check");
    return response.data;
  },
  logout: async (): Promise<ApiResponse<string>> => {
    const response = await api.post("/auth/logout");
    return response.data;
  },
};

// ============================================================================
// User Endpoints
// ============================================================================
export const userApi = {
  register: async (userName: string, email: string, password: string): Promise<ApiResponse<User>> => {
    const response = await api.post("/users/post", { userName, email, password });
    return response.data;
  },
  list: async (): Promise<ApiResponse<User[]>> => {
    const response = await api.get("/users/get");
    return response.data;
  },
  getById: async (userId: number): Promise<ApiResponse<User>> => {
    const response = await api.get(`/users/get/${userId}`);
    return response.data;
  },
  profile: async (): Promise<ApiResponse<User>> => {
    const response = await api.get("/users/profile");
    return response.data;
  },
  update: async (userId: number, data: { userName?: string; email?: string; password?: string }): Promise<ApiResponse<User>> => {
    const response = await api.put(`/users/put/${userId}`, data);
    return response.data;
  },
  delete: async (userId: number): Promise<ApiResponse<User>> => {
    const response = await api.delete(`/users/delete/${userId}`);
    return response.data;
  },
};

// ============================================================================
// Post Endpoints
// ============================================================================
export const postApi = {
  create: async (content: string): Promise<ApiResponse<Post>> => {
    const response = await api.post("/posts/post", { content });
    return response.data;
  },
  list: async (params?: { limit?: number; offset?: number; order_by?: string; sort?: string; search?: string }): Promise<ApiResponse<Post[]>> => {
    const response = await api.get("/posts/get", { params });
    return response.data;
  },
  getById: async (postId: number): Promise<ApiResponse<Post>> => {
    const response = await api.get(`/posts/get/${postId}`);
    return response.data;
  },
  trending: async (): Promise<ApiResponse<Post[]>> => {
    const response = await api.get("/posts/trending");
    return response.data;
  },
  update: async (postId: number, content: string): Promise<ApiResponse<Post>> => {
    const response = await api.put(`/posts/put/${postId}`, { content });
    return response.data;
  },
  delete: async (postId: number): Promise<ApiResponse<Post>> => {
    const response = await api.delete(`/posts/delete/${postId}`);
    return response.data;
  },
};

// ============================================================================
// Comment Endpoints
// ============================================================================
export const commentApi = {
  create: async (content: string, replyTo: number, replyToType: "post" | "comment"): Promise<ApiResponse<Comment>> => {
    const response = await api.post("/comments/post", {
      content,
      replyTo,
      replyToType,
    });
    return response.data;
  },
  list: async (params: { commentId?: number; replyTo?: number; replyToType?: "post" | "comment"; limit?: number; offset?: number }): Promise<ApiResponse<Comment[]>> => {
    const response = await api.get("/comments/get", { params });
    return response.data;
  },
  delete: async (commentId: number): Promise<ApiResponse<Comment>> => {
    const response = await api.delete(`/comments/delete/${commentId}`);
    return response.data;
  },
};

// ============================================================================
// Reaction Endpoints
// ============================================================================
export const reactionApi = {
  like: async (toId: number, toType: "post" | "comment"): Promise<ApiResponse<Reaction>> => {
    const response = await api.post("/reactions/post/like", null, {
      params: { toId, toType },
    });
    return response.data;
  },
  dislike: async (toId: number, toType: "post" | "comment"): Promise<ApiResponse<Reaction>> => {
    const response = await api.post("/reactions/post/dislike", null, {
      params: { toId, toType },
    });
    return response.data;
  },
  getTable: async (toId: number, toType: "post" | "comment"): Promise<ApiResponse<{ like: number; dislike: number; userReactionId?: number; userReactionType?: string }>> => {
    const response = await api.get("/reactions/get-table", {
      params: { toId, toType },
    });
    return response.data;
  },
  getReactions: async (params?: { toId?: number; toType?: "post" | "comment"; reactionName?: "Like" | "Dislike"; userId?: number }): Promise<ApiResponse<Reaction[]>> => {
    const response = await api.get("/reactions/get", { params });
    return response.data;
  },
  delete: async (reactionId: number): Promise<ApiResponse<Reaction>> => {
    const response = await api.delete("/reactions/delete", {
      params: { reactionId },
    });
    return response.data;
  },
};

// ============================================================================
// File Endpoints
// ============================================================================
export const fileApi = {
  listAll: async (): Promise<ApiResponse<FileRecord[]>> => {
    const response = await api.get("/files/all");
    return response.data;
  },
  listByUser: async (userId: number): Promise<ApiResponse<FileRecord[]>> => {
    const response = await api.get("/files/user", {
      params: { userId },
    });
    return response.data;
  },
  listPublic: async (): Promise<ApiResponse<FileRecord[]>> => {
    const response = await api.get("/files/pub");
    return response.data;
  },
  uploadPublic: async (formData: FormData): Promise<ApiResponse<FileRecord>> => {
    const response = await api.post("/files/upload-public", formData, {
      headers: { "Content-Type": "multipart/form-data" },
    });
    return response.data;
  },
  uploadPrivate: async (formData: FormData): Promise<ApiResponse<FileRecord>> => {
    const response = await api.post("/files/upload-private", formData, {
      headers: { "Content-Type": "multipart/form-data" },
    });
    return response.data;
  },
  getDownloadUrl: (fileId: string): string => {
    return `${api.defaults.baseURL}/files/download/${fileId}`;
  },
};

export const getSystemStats = async (): Promise<{ requestCount: number; responseTimes: Record<string, number[]> }> => {
  const statsUrl = (api.defaults.baseURL || "http://localhost:8000/api").replace(/\/api$/, "") + "/stats";
  const response = await axios.get(statsUrl);
  return {
    requestCount: response.data.request_count ?? 0,
    responseTimes: response.data.response_times ?? {},
  };
};

export default api;
