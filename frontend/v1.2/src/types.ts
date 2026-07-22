export interface Pagination {
  limit: number;
  offset: number;
  count?: number;
}

export interface ApiResponse<T> {
  code: number;
  message: string;
  body?: { data: T; pagination?: Pagination };
}

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

export interface FileRecord {
  id: string;
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
  postId?: string;
  commentId?: string;
}

export interface Post {
  id: string;
  title: string;
  content: string;
  createdAt: string;
  updatedAt: string;
  userId: number;
  replyTo?: string;
  userName: string;
  likeCount: number;
  dislikeCount: number;
  engagementRate: number;
  attachments?: FileRecord[];
}

export interface Comment {
  id: string;
  content: string;
  replyTo: string;
  userId: number;
  userName: string;
  createdAt: string;
  updatedAt: string;
  attachments?: FileRecord[];
}

export interface Reaction {
  id: number;
  userId: number;
  toId: string;
  createdAt: string;
  reactionName: "Like" | "Dislike";
  toType: "post" | "comment";
}

export interface ReactionTable {
  like: number;
  dislike: number;
}

export interface FollowStats {
  userId: number;
  followersCount: number;
  followingCount: number;
  isFollowing: boolean;
}

export interface Notification {
  id: number;
  userId: number;
  actorId: number;
  actorName?: string;
  kind: "follow" | "comment" | "reaction";
  postId?: string;
  commentId?: string;
  isRead: boolean;
  createdAt: string;
}

export interface SystemStats {
  requestCount: number;
  responseTimes: Record<string, number[]>;
}
