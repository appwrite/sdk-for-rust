//! Storage service for Appwrite SDK

use crate::client::Client;
use crate::input_file::InputFile;
use reqwest::Method;
use serde_json::json;
use std::collections::HashMap;

/// The Storage service allows you to manage your project files.
#[derive(Debug, Clone)]
pub struct Storage {
    client: Client,
}

impl Storage {
    pub fn new(client: &Client) -> Self {
        Self { client: client.clone() }
    }

    pub fn client(&self) -> &Client {
        &self.client
    }

    /// Get a list of all the storage buckets. You can use the query params to
    /// filter your results.
    pub async fn list_buckets(
        &self,
        queries: Option<Vec<String>>,
        search: Option<&str>,
        total: Option<bool>,
    ) -> crate::error::Result<crate::models::BucketList> {
        let mut params = HashMap::new();
        if let Some(value) = queries {
            params.insert("queries".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        if let Some(value) = search {
            params.insert("search".to_string(), json!(value));
        }
        if let Some(value) = total {
            params.insert("total".to_string(), json!(value));
        }

        let path = "/storage/buckets".to_string();

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Create a new storage bucket.
    #[allow(clippy::too_many_arguments)]
    pub async fn create_bucket(
        &self,
        bucket_id: impl Into<String>,
        name: impl Into<String>,
        permissions: Option<Vec<String>>,
        file_security: Option<bool>,
        enabled: Option<bool>,
        maximum_file_size: Option<i64>,
        allowed_file_extensions: Option<Vec<String>>,
        compression: Option<crate::enums::Compression>,
        encryption: Option<bool>,
        antivirus: Option<bool>,
        transformations: Option<bool>,
    ) -> crate::error::Result<crate::models::Bucket> {
        let mut params = HashMap::new();
        params.insert("bucketId".to_string(), json!(bucket_id.into()));
        params.insert("name".to_string(), json!(name.into()));
        if let Some(value) = permissions {
            params.insert("permissions".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        if let Some(value) = file_security {
            params.insert("fileSecurity".to_string(), json!(value));
        }
        if let Some(value) = enabled {
            params.insert("enabled".to_string(), json!(value));
        }
        if let Some(value) = maximum_file_size {
            params.insert("maximumFileSize".to_string(), json!(value));
        }
        if let Some(value) = allowed_file_extensions {
            params.insert("allowedFileExtensions".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        if let Some(value) = compression {
            params.insert("compression".to_string(), json!(value));
        }
        if let Some(value) = encryption {
            params.insert("encryption".to_string(), json!(value));
        }
        if let Some(value) = antivirus {
            params.insert("antivirus".to_string(), json!(value));
        }
        if let Some(value) = transformations {
            params.insert("transformations".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/storage/buckets".to_string();

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Get a storage bucket by its unique ID. This endpoint response returns a
    /// JSON object with the storage bucket metadata.
    pub async fn get_bucket(
        &self,
        bucket_id: impl Into<String>,
    ) -> crate::error::Result<crate::models::Bucket> {
        let params = HashMap::new();

        let path = "/storage/buckets/{bucketId}".to_string().replace("{bucketId}", &bucket_id.into().to_string());

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Update a storage bucket by its unique ID.
    #[allow(clippy::too_many_arguments)]
    pub async fn update_bucket(
        &self,
        bucket_id: impl Into<String>,
        name: impl Into<String>,
        permissions: Option<Vec<String>>,
        file_security: Option<bool>,
        enabled: Option<bool>,
        maximum_file_size: Option<i64>,
        allowed_file_extensions: Option<Vec<String>>,
        compression: Option<crate::enums::Compression>,
        encryption: Option<bool>,
        antivirus: Option<bool>,
        transformations: Option<bool>,
    ) -> crate::error::Result<crate::models::Bucket> {
        let mut params = HashMap::new();
        params.insert("name".to_string(), json!(name.into()));
        if let Some(value) = permissions {
            params.insert("permissions".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        if let Some(value) = file_security {
            params.insert("fileSecurity".to_string(), json!(value));
        }
        if let Some(value) = enabled {
            params.insert("enabled".to_string(), json!(value));
        }
        if let Some(value) = maximum_file_size {
            params.insert("maximumFileSize".to_string(), json!(value));
        }
        if let Some(value) = allowed_file_extensions {
            params.insert("allowedFileExtensions".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        if let Some(value) = compression {
            params.insert("compression".to_string(), json!(value));
        }
        if let Some(value) = encryption {
            params.insert("encryption".to_string(), json!(value));
        }
        if let Some(value) = antivirus {
            params.insert("antivirus".to_string(), json!(value));
        }
        if let Some(value) = transformations {
            params.insert("transformations".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/storage/buckets/{bucketId}".to_string().replace("{bucketId}", &bucket_id.into().to_string());

        self.client.call(Method::PUT, &path, Some(api_headers), Some(params)).await
    }

    /// Delete a storage bucket by its unique ID.
    pub async fn delete_bucket(
        &self,
        bucket_id: impl Into<String>,
    ) -> crate::error::Result<()> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/storage/buckets/{bucketId}".to_string().replace("{bucketId}", &bucket_id.into().to_string());

        self.client.call(Method::DELETE, &path, Some(api_headers), Some(params)).await
    }

    /// Get a list of all the user files. You can use the query params to filter
    /// your results.
    pub async fn list_files(
        &self,
        bucket_id: impl Into<String>,
        queries: Option<Vec<String>>,
        search: Option<&str>,
        total: Option<bool>,
    ) -> crate::error::Result<crate::models::FileList> {
        let mut params = HashMap::new();
        if let Some(value) = queries {
            params.insert("queries".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        if let Some(value) = search {
            params.insert("search".to_string(), json!(value));
        }
        if let Some(value) = total {
            params.insert("total".to_string(), json!(value));
        }

        let path = "/storage/buckets/{bucketId}/files".to_string().replace("{bucketId}", &bucket_id.into().to_string());

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Create a new file. Before using this route, you should create a new bucket
    /// resource using either a [server
    /// integration](https://appwrite.io/docs/server/storage#storageCreateBucket)
    /// API or directly from your Appwrite console.
    /// 
    /// Larger files should be uploaded using multiple requests with the
    /// [content-range](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Content-Range)
    /// header to send a partial request with a maximum supported chunk of `5MB`.
    /// The `content-range` header values should always be in bytes.
    /// 
    /// When the first request is sent, the server will return the **File** object,
    /// and the subsequent part request must include the file's **id** in
    /// `x-appwrite-id` header to allow the server to know that the partial upload
    /// is for the existing file and not for a new one.
    /// 
    /// If you're creating a new file using one of the Appwrite SDKs, all the
    /// chunking logic will be managed by the SDK internally.
    pub async fn create_file(
        &self,
        bucket_id: impl Into<String>,
        file_id: impl Into<String>,
        file: InputFile,
        permissions: Option<Vec<String>>,
    ) -> crate::error::Result<crate::models::File> {
        let file_id_str = file_id.into();
        let mut params = HashMap::new();
        params.insert("fileId".to_string(), json!(file_id_str));
        if let Some(value) = permissions {
            params.insert("permissions".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "multipart/form-data".to_string());

        let path = "/storage/buckets/{bucketId}/files".to_string().replace("{bucketId}", &bucket_id.into().to_string());

        self.client.file_upload(&path, Some(api_headers), params, "file", file, Some(file_id_str)).await
    }

    /// Get a file by its unique ID. This endpoint response returns a JSON object
    /// with the file metadata.
    pub async fn get_file(
        &self,
        bucket_id: impl Into<String>,
        file_id: impl Into<String>,
    ) -> crate::error::Result<crate::models::File> {
        let params = HashMap::new();

        let path = "/storage/buckets/{bucketId}/files/{fileId}".to_string().replace("{bucketId}", &bucket_id.into().to_string()).replace("{fileId}", &file_id.into().to_string());

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Update a file by its unique ID. Only users with write permissions have
    /// access to update this resource.
    pub async fn update_file(
        &self,
        bucket_id: impl Into<String>,
        file_id: impl Into<String>,
        name: Option<&str>,
        permissions: Option<Vec<String>>,
    ) -> crate::error::Result<crate::models::File> {
        let mut params = HashMap::new();
        if let Some(value) = name {
            params.insert("name".to_string(), json!(value));
        }
        if let Some(value) = permissions {
            params.insert("permissions".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/storage/buckets/{bucketId}/files/{fileId}".to_string().replace("{bucketId}", &bucket_id.into().to_string()).replace("{fileId}", &file_id.into().to_string());

        self.client.call(Method::PUT, &path, Some(api_headers), Some(params)).await
    }

    /// Delete a file by its unique ID. Only users with write permissions have
    /// access to delete this resource.
    pub async fn delete_file(
        &self,
        bucket_id: impl Into<String>,
        file_id: impl Into<String>,
    ) -> crate::error::Result<()> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/storage/buckets/{bucketId}/files/{fileId}".to_string().replace("{bucketId}", &bucket_id.into().to_string()).replace("{fileId}", &file_id.into().to_string());

        self.client.call(Method::DELETE, &path, Some(api_headers), Some(params)).await
    }

    /// Get a file content by its unique ID. The endpoint response return with a
    /// 'Content-Disposition: attachment' header that tells the browser to start
    /// downloading the file to user downloads directory.
    pub async fn get_file_download(
        &self,
        bucket_id: impl Into<String>,
        file_id: impl Into<String>,
        token: Option<&str>,
    ) -> crate::error::Result<Vec<u8>> {
        let mut params = HashMap::new();
        if let Some(value) = token {
            params.insert("token".to_string(), json!(value));
        }

        let path = "/storage/buckets/{bucketId}/files/{fileId}/download".to_string().replace("{bucketId}", &bucket_id.into().to_string()).replace("{fileId}", &file_id.into().to_string());

        self.client.call_bytes(Method::GET, &path, None, Some(params)).await
    }

    /// Get a file preview image. Currently, this method supports preview for image
    /// files (jpg, png, and gif), other supported formats, like pdf, docs, slides,
    /// and spreadsheets, will return the file icon image. You can also pass query
    /// string arguments for cutting and resizing your preview image. Preview is
    /// supported only for image files smaller than 10MB.
    #[allow(clippy::too_many_arguments)]
    pub async fn get_file_preview(
        &self,
        bucket_id: impl Into<String>,
        file_id: impl Into<String>,
        width: Option<i64>,
        height: Option<i64>,
        gravity: Option<crate::enums::ImageGravity>,
        quality: Option<i64>,
        border_width: Option<i64>,
        border_color: Option<&str>,
        border_radius: Option<i64>,
        opacity: Option<f64>,
        rotation: Option<i64>,
        background: Option<&str>,
        output: Option<crate::enums::ImageFormat>,
        token: Option<&str>,
    ) -> crate::error::Result<Vec<u8>> {
        let mut params = HashMap::new();
        if let Some(value) = width {
            params.insert("width".to_string(), json!(value));
        }
        if let Some(value) = height {
            params.insert("height".to_string(), json!(value));
        }
        if let Some(value) = gravity {
            params.insert("gravity".to_string(), json!(value));
        }
        if let Some(value) = quality {
            params.insert("quality".to_string(), json!(value));
        }
        if let Some(value) = border_width {
            params.insert("borderWidth".to_string(), json!(value));
        }
        if let Some(value) = border_color {
            params.insert("borderColor".to_string(), json!(value));
        }
        if let Some(value) = border_radius {
            params.insert("borderRadius".to_string(), json!(value));
        }
        if let Some(value) = opacity {
            params.insert("opacity".to_string(), json!(value));
        }
        if let Some(value) = rotation {
            params.insert("rotation".to_string(), json!(value));
        }
        if let Some(value) = background {
            params.insert("background".to_string(), json!(value));
        }
        if let Some(value) = output {
            params.insert("output".to_string(), json!(value));
        }
        if let Some(value) = token {
            params.insert("token".to_string(), json!(value));
        }

        let path = "/storage/buckets/{bucketId}/files/{fileId}/preview".to_string().replace("{bucketId}", &bucket_id.into().to_string()).replace("{fileId}", &file_id.into().to_string());

        self.client.call_bytes(Method::GET, &path, None, Some(params)).await
    }

    /// Get a file content by its unique ID. This endpoint is similar to the
    /// download method but returns with no  'Content-Disposition: attachment'
    /// header.
    pub async fn get_file_view(
        &self,
        bucket_id: impl Into<String>,
        file_id: impl Into<String>,
        token: Option<&str>,
    ) -> crate::error::Result<Vec<u8>> {
        let mut params = HashMap::new();
        if let Some(value) = token {
            params.insert("token".to_string(), json!(value));
        }

        let path = "/storage/buckets/{bucketId}/files/{fileId}/view".to_string().replace("{bucketId}", &bucket_id.into().to_string()).replace("{fileId}", &file_id.into().to_string());

        self.client.call_bytes(Method::GET, &path, None, Some(params)).await
    }

}

impl crate::services::Service for Storage {
    fn client(&self) -> &Client {
        &self.client
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_storage_creation() {
        let client = Client::new();
        let service = Storage::new(&client);
        assert!(service.client().endpoint().contains("cloud.appwrite.io/v1"));
    }
}
