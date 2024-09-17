export interface File {
    id: string;
    name: string;
    path: string;
    size: number;
    content_type: string;
    created_at: string;
    updated_at: string;
    user_id: number;
    description?: string;
    checksum: string;
    is_deleted: boolean;
}
