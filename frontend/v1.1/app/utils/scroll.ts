import { useEffect } from "react";

/**
 * A custom hook to save the window scroll position in sessionStorage and
 * restore it once asynchronous data fetching is complete.
 *
 * @param key Unique key to identify the scroll position in sessionStorage.
 * @param loading Boolean representing whether the page is currently loading data.
 * @param hasData Boolean representing whether the layout has data rendered (page height expanded).
 */
export function useScrollPreservation(key: string, loading: boolean, hasData: boolean) {
  // Save scroll position
  useEffect(() => {
    const handleScroll = () => {
      sessionStorage.setItem(`scroll_position_${key}`, window.scrollY.toString());
    };
    let timeoutId: number;
    const debouncedHandleScroll = () => {
      window.clearTimeout(timeoutId);
      timeoutId = window.setTimeout(handleScroll, 100);
    };

    window.addEventListener("scroll", debouncedHandleScroll);
    return () => {
      window.removeEventListener("scroll", debouncedHandleScroll);
      window.clearTimeout(timeoutId);
    };
  }, [key]);

  // Restore scroll position
  useEffect(() => {
    if (!loading && hasData) {
      const savedScroll = sessionStorage.getItem(`scroll_position_${key}`);
      if (savedScroll) {
        const timer = setTimeout(() => {
          window.scrollTo({
            top: parseInt(savedScroll, 10),
            behavior: "instant" as ScrollBehavior,
          });
        }, 50);
        return () => clearTimeout(timer);
      }
    }
  }, [loading, hasData, key]);
}
