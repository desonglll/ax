export interface ApiResponse<T> {
    code: number;
    message: string;
    body: Data<T> | null;
}

export interface Data<T> {
    data: T | T[] | null;
    pagination?: Pagination;
}

export interface Pagination {
    page: number;
    per_page: number;
    total_pages: number;
    count: number;
}
