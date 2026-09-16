import axios, { AxiosError } from "axios";

export const api = axios.create({
  baseURL: import.meta.env.VITE_API_URL || "/api",
  withCredentials: true,
  timeout: 20_000,
});

/** Human-readable message for a failed request; the API returns `{ code, message }`. */
export const getApiError = (error: unknown, fallback = "Request failed") => {
  if (error instanceof AxiosError) {
    if (error.response?.status === 429) return "Too many requests. Please slow down.";
    const data = error.response?.data as { message?: string } | string | undefined;
    if (typeof data === "object" && data?.message) return data.message;
    return error.response ? `${fallback} (${error.response.status})` : "Cannot reach the server";
  }
  return error instanceof Error ? error.message : fallback;
};
