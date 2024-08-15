export interface Post {
  id: number;
  content: string;
  createdAt: string;
  updatedAt: string;
  userId: number;
  userName: string;
  replyTo?: number;
  reactions?: {
    like: number;
    dislike: number;
  };
}

export interface Comment {
  id: number;
  content: string;
  reply_to: number;
  user_id: number;
  user_name: string;
  created_at: string;
  updated_at: string;
  reactions?: {
    like: number;
    dislike: number;
  };
}
