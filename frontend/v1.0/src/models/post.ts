export interface Post {
    id: number;
    content: string;
    createdAt: string;
    updatedAt: string;
    userId: number;
    userName: string;
    replyTo?: number;
    likeCount: number,
    dislikeCount: number

}

