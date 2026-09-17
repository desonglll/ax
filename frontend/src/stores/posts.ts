import { defineStore } from "pinia";
import { ref } from "vue";
import type { Post } from "../types";

type Field = keyof Post;
type Patch = { [K in Field]?: { at: number; value: Post[K] } };

/**
 * Keeps every mounted copy of a post in agreement. List pages stay alive in
 * the background, so a like, comment, edit or delete made elsewhere is
 * recorded here and each PostCard renders `view(post)`.
 *
 * A logical clock orders things: a patched field only overrides a post
 * object first seen *before* the patch, so a list that refetches after the
 * change shows the server's (newer) numbers rather than a stale local value.
 */
export const usePostStore = defineStore("posts", () => {
  const patches = ref<Record<string, Patch>>({});
  const deleted = ref<Record<string, true>>({});
  const seen = new Map<string, Post>();
  const stamps = new WeakMap<Post, number>();
  let clock = 0;

  const view = (post: Post): Post => {
    let at = stamps.get(post);
    if (at === undefined) {
      at = ++clock;
      stamps.set(post, at);
      seen.set(post.id, post);
    }
    const patch = patches.value[post.id];
    if (!patch) return post;
    const merged: Record<string, unknown> = { ...post };
    for (const [field, change] of Object.entries(patch)) {
      if (change && change.at > at) merged[field] = change.value;
    }
    return merged as unknown as Post;
  };

  /** Last known copy of a post with local changes applied, if it was seen in a list. */
  const peek = (id: string) => {
    const post = seen.get(id);
    return post && { ...view(post) };
  };

  const patch = (id: string, changes: Partial<Post>) => {
    const at = ++clock;
    const next: Record<string, unknown> = { ...patches.value[id] };
    for (const [field, value] of Object.entries(changes)) next[field] = { at, value };
    patches.value[id] = next as Patch;
  };

  const markDeleted = (id: string) => {
    deleted.value[id] = true;
    seen.delete(id);
  };

  return { patches, deleted, view, peek, patch, markDeleted };
});
