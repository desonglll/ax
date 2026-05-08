export interface Post {
    id: number;
    content: string;
    createdAt: string;
    updatedAt: string;
    userId: number;
    replyTo: number | null;
    userName: string;
    likeCount: number | null;
    dislikeCount: number | null;
    engagementRate: number | null;
}
