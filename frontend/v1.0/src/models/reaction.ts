export interface Reaction {
    id: number;
    userId: number;
    toId: number;
    createdAt: string;
    reactionName: "like" | "dislike";
    toType: "post" | "comment";
}
