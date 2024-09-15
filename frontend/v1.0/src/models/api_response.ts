export interface ApiResponse<T> {
  code: number;
  message: string;
  body: Data<T>;
}

interface Data<T> {
  data: T[];
  pagination?: Pagination;
}

interface Pagination {
  /// 当前页码
  page: number;

  /// 每页的条目数
  per_page: number;

  /// 总页数
  total_pages: number;

  /// 当前页的条目数
  count: number;

  /// 下一页的链接
  next: string | null;

  /// 上一页的链接
  previous: string | null;
}
