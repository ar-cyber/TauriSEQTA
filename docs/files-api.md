# BetterSEQTA+ Files API Documentation

This documentation provides comprehensive information for third-party applications to integrate with the BetterSEQTA+ Files API for file synchronization, sending, and receiving.

## Table of Contents

1. [Authentication](#authentication)
2. [Base URL](#base-url)
3. [File Management API](#file-management-api)
4. [File Upload](#file-upload)
5. [File Download](#file-download)
6. [File Listing](#file-listing)
7. [File Updates](#file-updates)
8. [File Deletion](#file-deletion)
9. [Public File Access](#public-file-access)
10. [TypeScript SDK](#typescript-sdk)
11. [Error Handling](#error-handling)
12. [Rate Limiting](#rate-limiting)
13. [Best Practices](#best-practices)

## Authentication

All API endpoints require JWT authentication. Include the token in the `Authorization` header:

```typescript
const headers = {
  'Authorization': `Bearer ${token}`,
  'Content-Type': 'application/json'
};
```

### Getting an Authentication Token

```typescript
interface LoginResponse {
  token: string;
  user: {
    id: number;
    email: string;
    username: string;
    displayName: string;
  };
}

const login = async (email: string, password: string): Promise<LoginResponse> => {
  const response = await fetch(`${BASE_URL}/api/auth/login`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ email, password })
  });
  
  if (!response.ok) {
    throw new Error('Login failed');
  }
  
  return response.json();
};
```

## Base URL

```
https://your-domain.com/api
```

## File Management API

### File Object Structure

```typescript
interface File {
  id: number;
  userId: number;
  filename: string;
  storedName: string;
  mimeType: string;
  size: number;
  path: string;
  isPublic: boolean;
  createdAt: string;
  updatedAt: string;
}

interface FileListResponse {
  files: File[];
  pagination: {
    page: number;
    limit: number;
    total: number;
    pages: number;
  };
}
```

## File Upload

Upload files to the server with metadata tracking.

**Endpoint:** `POST /api/files/upload`

**Content-Type:** `multipart/form-data`

**Parameters:**
- `file` (required): The file to upload

**Response:**
```typescript
interface UploadResponse extends File {}
```

**Example:**

```typescript
const uploadFile = async (file: File, token: string): Promise<File> => {
  const formData = new FormData();
  formData.append('file', file);

  const response = await fetch(`${BASE_URL}/api/files/upload`, {
    method: 'POST',
    headers: {
      'Authorization': `Bearer ${token}`
    },
    body: formData
  });

  if (!response.ok) {
    throw new Error('Upload failed');
  }

  return response.json();
};

// Usage
const fileInput = document.querySelector('input[type="file"]');
const file = fileInput.files[0];
const uploadedFile = await uploadFile(file, token);
console.log('File uploaded:', uploadedFile);
```

## File Download

Download files using their stored name. There are two download methods:

### 1. Private File Download (Requires Authentication)

**Endpoint:** `GET /api/files/{storedName}`

**Headers:**
- `Authorization: Bearer {token}` (required)

**Response:** File stream with proper headers

**Example:**

```typescript
const downloadFile = async (storedName: string, token: string): Promise<Blob> => {
  const response = await fetch(`${BASE_URL}/api/files/${storedName}`, {
    headers: {
      'Authorization': `Bearer ${token}`
    }
  });

  if (!response.ok) {
    throw new Error('Download failed');
  }

  return response.blob();
};

// Usage
const blob = await downloadFile('a1b2c3d4e5f6.pdf', token);
const url = URL.createObjectURL(blob);
const a = document.createElement('a');
a.href = url;
a.download = 'filename.pdf';
a.click();
```

### 2. Public File Download (No Authentication Required)

**Endpoint:** `GET /api/files/public/{storedName}`

**Headers:** None required

**Response:** File stream with proper headers

**Example:**

```typescript
const downloadPublicFile = async (storedName: string): Promise<Blob> => {
  const response = await fetch(`${BASE_URL}/api/files/public/${storedName}`);

  if (!response.ok) {
    throw new Error('File not found or not public');
  }

  return response.blob();
};

// Usage
const blob = await downloadPublicFile('a1b2c3d4e5f6.pdf');
```

### Download URL Examples

```typescript
// Private file (requires auth)
const privateUrl = `${BASE_URL}/api/files/a1b2c3d4e5f6.pdf`;

// Public file (no auth required)
const publicUrl = `${BASE_URL}/api/files/public/a1b2c3d4e5f6.pdf`;

// Direct file access (if file is in public/uploads)
const directUrl = `${BASE_URL}/uploads/a1b2c3d4e5f6.pdf`;
```

## File Listing

List user files with pagination and filtering.

**Endpoint:** `GET /api/files/list`

**Query Parameters:**
- `page` (optional): Page number (default: 1)
- `limit` (optional): Items per page (default: 20, max: 100)
- `search` (optional): Search by filename
- `mimeType` (optional): Filter by MIME type
- `isPublic` (optional): Filter by public status (true/false)

**Response:**
```typescript
interface FileListResponse {
  files: File[];
  pagination: {
    page: number;
    limit: number;
    total: number;
    pages: number;
  };
}
```

**Example:**

```typescript
const listFiles = async (
  token: string,
  options: {
    page?: number;
    limit?: number;
    search?: string;
    mimeType?: string;
    isPublic?: boolean;
  } = {}
): Promise<FileListResponse> => {
  const params = new URLSearchParams();
  if (options.page) params.append('page', options.page.toString());
  if (options.limit) params.append('limit', options.limit.toString());
  if (options.search) params.append('search', options.search);
  if (options.mimeType) params.append('mimeType', options.mimeType);
  if (options.isPublic !== undefined) params.append('isPublic', options.isPublic.toString());

  const response = await fetch(`${BASE_URL}/api/files/list?${params}`, {
    headers: {
      'Authorization': `Bearer ${token}`
    }
  });

  if (!response.ok) {
    throw new Error('Failed to list files');
  }

  return response.json();
};

// Usage
const files = await listFiles(token, {
  page: 1,
  limit: 10,
  search: 'document',
  mimeType: 'application/pdf'
});
```

## File Updates

Update file metadata (filename and public status).

**Endpoint:** `PATCH /api/files/{id}`

**Body:**
```typescript
interface UpdateFileRequest {
  filename?: string;
  isPublic?: boolean;
}
```

**Response:**
```typescript
interface UpdateFileResponse extends File {}
```

**Example:**

```typescript
const updateFile = async (
  fileId: number,
  updates: UpdateFileRequest,
  token: string
): Promise<File> => {
  const response = await fetch(`${BASE_URL}/api/files/${fileId}`, {
    method: 'PATCH',
    headers: {
      'Authorization': `Bearer ${token}`,
      'Content-Type': 'application/json'
    },
    body: JSON.stringify(updates)
  });

  if (!response.ok) {
    throw new Error('Update failed');
  }

  return response.json();
};

// Usage
const updatedFile = await updateFile(123, {
  filename: 'new-filename.pdf',
  isPublic: true
}, token);
```

## File Deletion

Delete a file permanently.

**Endpoint:** `DELETE /api/files/{id}`

**Response:**
```typescript
interface DeleteFileResponse {
  message: string;
}
```

**Example:**

```typescript
const deleteFile = async (fileId: number, token: string): Promise<void> => {
  const response = await fetch(`${BASE_URL}/api/files/${fileId}`, {
    method: 'DELETE',
    headers: {
      'Authorization': `Bearer ${token}`
    }
  });

  if (!response.ok) {
    throw new Error('Delete failed');
  }
};

// Usage
await deleteFile(123, token);
```

## Public File Access

Access public files without authentication.

**Endpoint:** `GET /api/files/public/{storedName}`

**Response:** File stream

**Example:**

```typescript
const getPublicFile = async (storedName: string): Promise<Blob> => {
  const response = await fetch(`${BASE_URL}/api/files/public/${storedName}`);

  if (!response.ok) {
    throw new Error('File not found or not public');
  }

  return response.blob();
};

// Usage
const blob = await getPublicFile('a1b2c3d4e5f6.jpg');
```

## TypeScript SDK

Here's a complete TypeScript SDK for the Files API:

```typescript
class BetterSEQTAFilesAPI {
  private baseUrl: string;
  private token: string;

  constructor(baseUrl: string, token: string) {
    this.baseUrl = baseUrl;
    this.token = token;
  }

  private async request<T>(
    endpoint: string,
    options: RequestInit = {}
  ): Promise<T> {
    const response = await fetch(`${this.baseUrl}${endpoint}`, {
      ...options,
      headers: {
        'Authorization': `Bearer ${this.token}`,
        ...options.headers,
      },
    });

    if (!response.ok) {
      const error = await response.json().catch(() => ({}));
      throw new Error(error.statusMessage || `HTTP ${response.status}`);
    }

    return response.json();
  }

  async uploadFile(file: File): Promise<File> {
    const formData = new FormData();
    formData.append('file', file);

    return this.request<File>('/files/upload', {
      method: 'POST',
      body: formData,
    });
  }

  async listFiles(options: {
    page?: number;
    limit?: number;
    search?: string;
    mimeType?: string;
    isPublic?: boolean;
  } = {}): Promise<FileListResponse> {
    const params = new URLSearchParams();
    Object.entries(options).forEach(([key, value]) => {
      if (value !== undefined) {
        params.append(key, value.toString());
      }
    });

    return this.request<FileListResponse>(`/files/list?${params}`);
  }

  async downloadFile(storedName: string): Promise<Blob> {
    const response = await fetch(`${this.baseUrl}/files/${storedName}`, {
      headers: {
        'Authorization': `Bearer ${this.token}`,
      },
    });

    if (!response.ok) {
      throw new Error(`Download failed: ${response.status}`);
    }

    return response.blob();
  }

  async downloadPublicFile(storedName: string): Promise<Blob> {
    const response = await fetch(`${this.baseUrl}/files/public/${storedName}`);

    if (!response.ok) {
      throw new Error('File not found or not public');
    }

    return response.blob();
  }

  async updateFile(
    fileId: number,
    updates: { filename?: string; isPublic?: boolean }
  ): Promise<File> {
    return this.request<File>(`/files/${fileId}`, {
      method: 'PATCH',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(updates),
    });
  }

  async deleteFile(fileId: number): Promise<void> {
    await this.request(`/files/${fileId}`, {
      method: 'DELETE',
    });
  }

  // Helper method to get download URL
  getDownloadUrl(file: File): string {
    if (file.isPublic) {
      return `${this.baseUrl}/files/public/${file.storedName}`;
    } else {
      return `${this.baseUrl}/files/${file.storedName}`;
    }
  }
}

// Usage
const api = new BetterSEQTAFilesAPI('https://your-domain.com/api', token);

// Upload a file
const file = await api.uploadFile(fileInput.files[0]);

// List files
const files = await api.listFiles({ page: 1, limit: 10 });

// Download a file
const blob = await api.downloadFile(file.storedName);

// Get download URL
const downloadUrl = api.getDownloadUrl(file);

// Update file
const updatedFile = await api.updateFile(file.id, { isPublic: true });

// Delete file
await api.deleteFile(file.id);
```

## File Synchronization Example

Here's an example of how to implement file synchronization:

```typescript
class FileSyncManager {
  private api: BetterSEQTAFilesAPI;
  private localFiles: Map<string, File> = new Map();

  constructor(api: BetterSEQTAFilesAPI) {
    this.api = api;
  }

  async syncFiles(): Promise<void> {
    try {
      // Get all files from server
      const serverFiles = await this.getAllServerFiles();
      
      // Compare with local files
      const toUpload = this.getFilesToUpload(serverFiles);
      const toDownload = this.getFilesToDownload(serverFiles);
      const toDelete = this.getFilesToDelete(serverFiles);

      // Perform sync operations
      await Promise.all([
        ...toUpload.map(file => this.uploadFile(file)),
        ...toDownload.map(file => this.downloadFile(file)),
        ...toDelete.map(file => this.deleteFile(file))
      ]);

      console.log('File sync completed');
    } catch (error) {
      console.error('File sync failed:', error);
    }
  }

  private async getAllServerFiles(): Promise<File[]> {
    const allFiles: File[] = [];
    let page = 1;
    let hasMore = true;

    while (hasMore) {
      const response = await this.api.listFiles({ page, limit: 100 });
      allFiles.push(...response.files);
      hasMore = page < response.pagination.pages;
      page++;
    }

    return allFiles;
  }

  private getFilesToUpload(serverFiles: File[]): File[] {
    // Implementation depends on your local file tracking
    return [];
  }

  private getFilesToDownload(serverFiles: File[]): File[] {
    // Implementation depends on your local file tracking
    return [];
  }

  private getFilesToDelete(serverFiles: File[]): File[] {
    // Implementation depends on your local file tracking
    return [];
  }

  private async uploadFile(file: File): Promise<void> {
    // Implementation for uploading local file
  }

  private async downloadFile(file: File): Promise<void> {
    const blob = await this.api.downloadFile(file.storedName);
    // Save blob to local storage
  }

  private async deleteFile(file: File): Promise<void> {
    await this.api.deleteFile(file.id);
    // Remove from local storage
  }
}
```

## Error Handling

The API returns standard HTTP status codes and error messages:

```typescript
interface APIError {
  statusCode: number;
  statusMessage: string;
}

// Common error codes:
// 400 - Bad Request (invalid parameters)
// 401 - Unauthorized (invalid/missing token)
// 404 - Not Found (file doesn't exist)
// 413 - Payload Too Large (file too big)
// 500 - Internal Server Error
```

**Error handling example:**

```typescript
const handleAPIError = (error: any) => {
  if (error.statusCode === 401) {
    // Token expired or invalid - redirect to login
    window.location.href = '/login';
  } else if (error.statusCode === 413) {
    // File too large
    alert('File size exceeds the maximum allowed limit');
  } else {
    // Generic error handling
    console.error('API Error:', error);
    alert('An error occurred. Please try again.');
  }
};

try {
  await api.uploadFile(file);
} catch (error) {
  handleAPIError(error);
}
```

## Rate Limiting

The API implements rate limiting to prevent abuse:

- **Upload:** 10 requests per minute per user
- **Download:** 100 requests per minute per user
- **List/Update/Delete:** 60 requests per minute per user

When rate limited, the API returns `429 Too Many Requests` with a `Retry-After` header.

## Best Practices

### 1. File Upload
- Compress large files before upload
- Show upload progress to users
- Validate file types and sizes on the client
- Handle network interruptions gracefully

```typescript
const uploadWithProgress = async (
  file: File,
  onProgress: (progress: number) => void
): Promise<File> => {
  const formData = new FormData();
  formData.append('file', file);

  const xhr = new XMLHttpRequest();
  
  return new Promise((resolve, reject) => {
    xhr.upload.addEventListener('progress', (e) => {
      if (e.lengthComputable) {
        const progress = (e.loaded / e.total) * 100;
        onProgress(progress);
      }
    });

    xhr.addEventListener('load', () => {
      if (xhr.status === 200) {
        resolve(JSON.parse(xhr.responseText));
      } else {
        reject(new Error('Upload failed'));
      }
    });

    xhr.addEventListener('error', () => reject(new Error('Network error')));

    xhr.open('POST', `${BASE_URL}/api/files/upload`);
    xhr.setRequestHeader('Authorization', `Bearer ${token}`);
    xhr.send(formData);
  });
};
```

### 2. File Synchronization
- Implement incremental sync to reduce bandwidth
- Use file hashes to detect changes
- Handle conflicts gracefully
- Implement retry logic for failed operations

### 3. Security
- Never store tokens in localStorage (use httpOnly cookies)
- Validate file types on both client and server
- Implement proper CORS policies
- Use HTTPS for all API calls

### 4. Performance
- Implement caching for frequently accessed files
- Use pagination for large file lists
- Compress images before upload
- Implement lazy loading for file previews

## Complete Integration Example

```typescript
class BetterSEQTAIntegration {
  private api: BetterSEQTAFilesAPI;
  private syncManager: FileSyncManager;

  constructor(baseUrl: string, token: string) {
    this.api = new BetterSEQTAFilesAPI(baseUrl, token);
    this.syncManager = new FileSyncManager(this.api);
  }

  async initialize(): Promise<void> {
    // Perform initial sync
    await this.syncManager.syncFiles();
    
    // Set up periodic sync
    setInterval(() => {
      this.syncManager.syncFiles();
    }, 5 * 60 * 1000); // Sync every 5 minutes
  }

  async uploadFile(file: File, onProgress?: (progress: number) => void): Promise<File> {
    if (onProgress) {
      return uploadWithProgress(file, onProgress);
    }
    return this.api.uploadFile(file);
  }

  async getFileList(options?: any): Promise<FileListResponse> {
    return this.api.listFiles(options);
  }

  async downloadFile(storedName: string): Promise<Blob> {
    return this.api.downloadFile(storedName);
  }

  async shareFile(fileId: number): Promise<File> {
    return this.api.updateFile(fileId, { isPublic: true });
  }

  async unshareFile(fileId: number): Promise<File> {
    return this.api.updateFile(fileId, { isPublic: false });
  }

  getDownloadUrl(file: File): string {
    return this.api.getDownloadUrl(file);
  }
}

// Usage
const integration = new BetterSEQTAIntegration(
  'https://your-domain.com/api',
  token
);

await integration.initialize();

// Upload with progress
const file = await integration.uploadFile(fileInput.files[0], (progress) => {
  console.log(`Upload progress: ${progress}%`);
});

// Share file
const sharedFile = await integration.shareFile(file.id);
console.log('File shared:', sharedFile);

// Get download URL
const downloadUrl = integration.getDownloadUrl(file);
console.log('Download URL:', downloadUrl);
```

This documentation provides everything needed to integrate with the BetterSEQTA+ Files API. The TypeScript SDK makes it easy to implement file synchronization, upload, download, and management features in your third-party application. 