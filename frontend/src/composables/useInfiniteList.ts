import { ref, type Ref } from "vue";
import { getApiError } from "../api/client";

export interface Page<T> {
  items: T[];
  count?: number;
}

/**
 * Append-only paginated list for infinite scrolling.
 *
 * `fetchPage(offset, limit)` returns one page; `reset()` reloads from the
 * start and `loadMore()` appends the next page. Responses from a superseded
 * `reset()` are discarded so fast tab switches never interleave pages.
 */
export function useInfiniteList<T extends { id: string | number }>(
  fetchPage: (offset: number, limit: number) => Promise<Page<T>>,
  limit = 12,
) {
  const items = ref<T[]>([]) as Ref<T[]>;
  const loading = ref(false);
  const loadingMore = ref(false);
  const done = ref(false);
  const error = ref("");
  const total = ref<number>();
  let offset = 0;
  let generation = 0;

  const load = async (reset: boolean) => {
    if (!reset && (done.value || loading.value || loadingMore.value)) return;
    const gen = ++generation;
    if (reset) {
      offset = 0;
      done.value = false;
      loading.value = true;
    } else {
      loadingMore.value = true;
    }
    try {
      const page = await fetchPage(offset, limit);
      if (gen !== generation) return;
      // Skip rows already shown (e.g. added locally before this page arrived).
      const known = new Set(reset ? [] : items.value.map(item => item.id));
      const fresh = page.items.filter(item => !known.has(item.id));
      items.value = reset ? page.items : [...items.value, ...fresh];
      offset += page.items.length;
      total.value = page.count;
      done.value = page.items.length < limit || (page.count !== undefined && offset >= page.count);
      error.value = "";
    } catch (e) {
      if (gen === generation) error.value = getApiError(e);
    } finally {
      if (gen === generation) {
        loading.value = false;
        loadingMore.value = false;
      }
    }
  };

  const prepend = (item: T) => {
    items.value = [item, ...items.value];
    offset += 1;
    if (total.value !== undefined) total.value += 1;
  };
  const remove = (id: T["id"]) => {
    const before = items.value.length;
    items.value = items.value.filter(item => item.id !== id);
    if (items.value.length < before) {
      offset -= 1;
      if (total.value !== undefined) total.value -= 1;
    }
  };
  const replace = (item: T) => {
    items.value = items.value.map(existing => (existing.id === item.id ? item : existing));
  };

  return { items, loading, loadingMore, done, error, total, reset: () => load(true), loadMore: () => load(false), prepend, remove, replace };
}
