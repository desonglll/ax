export interface User {
  id: number;
  userName: string;
  email: string;
  passwordHash: string;
  fullName?: string;
  phone?: string;
  createdAt?: string; // Assuming NaiveDateTime is a string in ISO format
  updatedAt?: string;
  lastLogin?: string;
  isActive: boolean;
  isAdmin: boolean;
  profilePicture?: string; // Assuming Uuid is a string
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
  profilePicture: string; // Assuming Uuid is a string
}
