export interface Reaction {
  id: number;
  userId: number;
  toId: number;
  createdAt: string;
  reactionName: string;
  toType: string;
}
