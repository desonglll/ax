import { i18n } from "../i18n";

type Params = Record<string, string | number | boolean | null | undefined>;
interface RequestOptions {
  params?: Params;
}

/** A failed request. `response` is absent for network errors and timeouts. */
export class ApiError extends Error {
  constructor(
    message: string,
    readonly response?: { status: number; data: unknown },
  ) {
    super(message);
    this.name = "ApiError";
  }
}

const baseURL: string = import.meta.env.VITE_API_URL || "/api";
const TIMEOUT_MS = 20_000;

/** `?a=1&b=x`, skipping null/undefined values (as axios did). */
export const queryString = (params?: Params) => {
  if (!params) return "";
  const search = new URLSearchParams();
  for (const [key, value] of Object.entries(params)) {
    if (value !== undefined && value !== null) search.append(key, String(value));
  }
  const text = search.toString();
  return text ? `?${text}` : "";
};

const parse = (text: string): unknown => {
  if (!text) return undefined;
  try {
    return JSON.parse(text);
  } catch {
    return text;
  }
};

async function request<T>(method: string, url: string, data?: unknown, options: RequestOptions = {}): Promise<{ data: T; status: number }> {
  const headers: Record<string, string> = { Accept: "application/json" };
  let body: BodyInit | undefined;
  if (data instanceof FormData) body = data;
  else if (data !== undefined) {
    body = JSON.stringify(data);
    headers["Content-Type"] = "application/json";
  }
  let response: Response;
  try {
    response = await fetch(`${baseURL}${url}${queryString(options.params)}`, {
      method,
      headers,
      body,
      credentials: "include",
      signal: AbortSignal.timeout(TIMEOUT_MS),
    });
  } catch (error) {
    throw new ApiError(error instanceof Error ? error.message : "Network error");
  }
  const payload = parse(await response.text().catch(() => ""));
  if (!response.ok) {
    throw new ApiError(`Request failed with status ${response.status}`, { status: response.status, data: payload });
  }
  return { data: payload as T, status: response.status };
}

/** Minimal fetch-based client; same call shape as the axios instance it replaced. */
export const api = {
  defaults: { baseURL },
  get: <T>(url: string, options?: RequestOptions) => request<T>("GET", url, undefined, options),
  delete: <T>(url: string, options?: RequestOptions) => request<T>("DELETE", url, undefined, options),
  post: <T>(url: string, data?: unknown, options?: RequestOptions) => request<T>("POST", url, data, options),
  put: <T>(url: string, data?: unknown, options?: RequestOptions) => request<T>("PUT", url, data, options),
};

/** HTTP status of a failed request, if the server answered. */
export const apiStatus = (error: unknown) => (error instanceof ApiError ? error.response?.status : undefined);

/** Human-readable message for a failed request; the API returns `{ code, message }`. */
export const getApiError = (error: unknown, fallback = i18n.global.t("errors.request")) => {
  if (error instanceof ApiError) {
    if (error.response?.status === 429) return i18n.global.t("errors.tooMany");
    const data = error.response?.data as { message?: string } | string | undefined;
    if (typeof data === "object" && data?.message) return data.message;
    return error.response ? `${fallback} (${error.response.status})` : i18n.global.t("errors.offline");
  }
  return error instanceof Error ? error.message : fallback;
};
