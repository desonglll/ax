export interface ApiResponse {
    status: string;
    message: string;
    // biome-ignore lint/suspicious/noExplicitAny: <explanation>
    body: any;
}
