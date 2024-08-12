export interface Post {
    id: number,
    content: string,
    createdAt: string,
    updatedAt: string,
    userId: number,
    userName: string,
    replyTo: number | null,
    reactions: {
        like: number,
        dislike: number
    } | null
}