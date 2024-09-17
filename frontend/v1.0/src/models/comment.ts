export interface Comment {
    id: number;
    content: string;
    replyTo: number;
    userId: number;
    userName: string;
    createdAt: string;
    updatedAt: string;
    replyToType: string
}
