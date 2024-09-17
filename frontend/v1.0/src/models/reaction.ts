export interface Reaction {
  id: number;
  userId: number;
  toId: number;
  createdAt: string;
  reactionName: string;
  toType: string;
}

export interface ReactionTable {
  like: number;
  dislike: number;
}
