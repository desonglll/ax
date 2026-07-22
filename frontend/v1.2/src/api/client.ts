import axios, { AxiosError } from "axios";

export const api = axios.create({
  baseURL: import.meta.env.VITE_API_URL || "/api",
  withCredentials: true,
  timeout: 20_000,
});

export const getApiError = (error: unknown, fallback = "Request failed") => {
  if (error instanceof AxiosError) {
    const data = error.response?.data as { message?: string; error_message?: string } | undefined;
    return data?.message || data?.error_message || error.message || fallback;
  }
  return error instanceof Error ? error.message : fallback;
};
