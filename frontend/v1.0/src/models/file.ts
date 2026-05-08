export interface File {
    id: string;
    name: string;
    path: string;
    size: number;
    contentType: string;
    createdAt: string | null;
    updatedAt: string | null;
    userId: number;
    description: string | null;
    checksum: string;
    isDeleted: boolean;
    isPub: boolean;
}
