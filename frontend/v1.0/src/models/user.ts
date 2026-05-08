export interface User {
    id: number;
    userName: string;
    email: string;
    fullName?: string;
    phone?: string;
    createdAt?: string;
    updatedAt?: string;
    lastLogin?: string;
    isActive: boolean;
    isAdmin: boolean;
    profilePicture?: string;
}

export interface UpdateUserRequest {
    id: number;
    userName: string;
    email: string;
    password?: string;
    fullName: string;
    phone: string;
    isActive: boolean;
    isAdmin: boolean;
    profilePicture: string;
}
