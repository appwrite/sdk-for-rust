//! Videos service for Appwrite SDK

use crate::client::Client;

use reqwest::Method;
use serde_json::json;
use std::collections::HashMap;

/// The Videos service allows you to transcode video files into adaptive HLS
/// and DASH streams.
#[derive(Debug, Clone)]
pub struct Videos {
    client: Client,
}

impl Videos {
    pub fn new(client: &Client) -> Self {
        Self {
            client: client.clone(),
        }
    }

    pub fn client(&self) -> &Client {
        &self.client
    }

    /// Get a list of videos in the current project. Sessions receive only videos
    /// they can read according to each video's `$permissions`. API keys and admins
    /// receive the full project catalog. Playback uses the same document read ACL
    /// plus the `videos.play` (or `videos.read`) scope: a guest can stream a video
    /// that grants `read("any")` or `read("guests")` even though guests lack
    /// `videos.read` and will not see it in this list.
    pub async fn list(
        &self,
        queries: Option<Vec<String>>,
        total: Option<bool>,
    ) -> crate::error::Result<crate::models::VideoList> {
        let mut params = HashMap::new();
        if let Some(value) = queries {
            params.insert(
                "queries".to_string(),
                json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()),
            );
        }
        if let Some(value) = total {
            params.insert("total".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/videos".to_string();

        self.client
            .call(Method::GET, &path, Some(api_headers), Some(params))
            .await
    }

    /// Create a video resource from an existing file in a storage bucket. The
    /// source file must be a video or audio file, and the caller must be able to
    /// read that file. Creating a video stores the document and queues timeline
    /// generation. The worker downloads the source, probes metadata, extracts
    /// embedded captions once, and deletes its working copy when finished. Sources
    /// with a video track also get a thumbnail timeline. Call create-rendition to
    /// package a streaming output.
    ///
    /// An optional `name` defaults to the source file name. An optional
    /// `permissions` array sets the video document ACL (list, get, update, delete,
    /// manage child resources, and playback). By default, only the current user is
    /// granted all permissions; API keys and admins create with empty permissions
    /// when omitted. Guests with `videos.play` can stream when the video grants
    /// them read (`read("any")` or `read("guests")`).
    ///
    /// Uploaded caption files stay listed alongside auto-extracted tracks for the
    /// same language; when the current default is an auto-extracted track with
    /// that code, the upload takes the default flag.
    pub async fn create(
        &self,
        video_id: impl Into<String>,
        bucket_id: impl Into<String>,
        file_id: impl Into<String>,
        name: Option<&str>,
        permissions: Option<Vec<String>>,
    ) -> crate::error::Result<crate::models::Video> {
        let mut params = HashMap::new();
        params.insert("videoId".to_string(), json!(video_id.into()));
        params.insert("bucketId".to_string(), json!(bucket_id.into()));
        params.insert("fileId".to_string(), json!(file_id.into()));
        if let Some(value) = name {
            params.insert("name".to_string(), json!(value));
        }
        if let Some(value) = permissions {
            params.insert(
                "permissions".to_string(),
                json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()),
            );
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/videos".to_string();

        self.client
            .call(Method::POST, &path, Some(api_headers), Some(params))
            .await
    }

    /// Get a list of video encode codecs enabled on this instance. Each codec
    /// includes the output formats it may be packaged into.
    pub async fn list_codecs(&self) -> crate::error::Result<crate::models::VideoCodecList> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/videos/codecs".to_string();

        self.client
            .call(Method::GET, &path, Some(api_headers), Some(params))
            .await
    }

    /// Get a video by its unique ID. This endpoint response returns a JSON object
    /// with the video metadata.
    pub async fn get(
        &self,
        video_id: impl Into<String>,
    ) -> crate::error::Result<crate::models::Video> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let video_id = video_id.into();
        if video_id.is_empty() {
            return Err(crate::error::AppwriteError::new(
                0,
                "Missing required parameter: \"videoId\"",
                None,
                String::new(),
            ));
        }

        let path = "/videos/{videoId}"
            .to_string()
            .replace("{videoId}", &Client::encode_path(&video_id));

        self.client
            .call(Method::GET, &path, Some(api_headers), Some(params))
            .await
    }

    /// Update a video's name, permissions, or both. Only the fields you pass are
    /// changed: omit `name` to keep the current name and `permissions` to keep the
    /// current ACL. The caller must hold update permission on the video document.
    /// The source file cannot be replaced; delete the video and create a new one
    /// to point at a different file.
    pub async fn update(
        &self,
        video_id: impl Into<String>,
        name: Option<&str>,
        permissions: Option<Vec<String>>,
    ) -> crate::error::Result<crate::models::Video> {
        let mut params = HashMap::new();
        if let Some(value) = name {
            params.insert("name".to_string(), json!(value));
        }
        if let Some(value) = permissions {
            params.insert(
                "permissions".to_string(),
                json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()),
            );
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let video_id = video_id.into();
        if video_id.is_empty() {
            return Err(crate::error::AppwriteError::new(
                0,
                "Missing required parameter: \"videoId\"",
                None,
                String::new(),
            ));
        }

        let path = "/videos/{videoId}"
            .to_string()
            .replace("{videoId}", &Client::encode_path(&video_id));

        self.client
            .call(Method::PATCH, &path, Some(api_headers), Some(params))
            .await
    }

    /// Delete a video by its unique ID. This also removes every rendition,
    /// caption, preview and transcoded artifact derived from it. Fails with
    /// `video_rendition_in_progress` while a rendition is started or ended, unless
    /// that rendition has not been updated within the rendition retention window.
    /// Fails with `video_timeline_in_progress` while the video's timeline is being
    /// generated, unless the video has not been updated within the same window.
    pub async fn delete(&self, video_id: impl Into<String>) -> crate::error::Result<()> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let video_id = video_id.into();
        if video_id.is_empty() {
            return Err(crate::error::AppwriteError::new(
                0,
                "Missing required parameter: \"videoId\"",
                None,
                String::new(),
            ));
        }

        let path = "/videos/{videoId}"
            .to_string()
            .replace("{videoId}", &Client::encode_path(&video_id));

        self.client
            .call(Method::DELETE, &path, Some(api_headers), Some(params))
            .await
    }

    /// Get a list of all the caption tracks attached to a video. You can use the
    /// query params to filter your results.
    pub async fn list_captions(
        &self,
        video_id: impl Into<String>,
        queries: Option<Vec<String>>,
        total: Option<bool>,
    ) -> crate::error::Result<crate::models::VideoCaptionList> {
        let mut params = HashMap::new();
        if let Some(value) = queries {
            params.insert(
                "queries".to_string(),
                json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()),
            );
        }
        if let Some(value) = total {
            params.insert("total".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let video_id = video_id.into();
        if video_id.is_empty() {
            return Err(crate::error::AppwriteError::new(
                0,
                "Missing required parameter: \"videoId\"",
                None,
                String::new(),
            ));
        }

        let path = "/videos/{videoId}/captions"
            .to_string()
            .replace("{videoId}", &Client::encode_path(&video_id));

        self.client
            .call(Method::GET, &path, Some(api_headers), Some(params))
            .await
    }

    /// Add a caption track to a video from a WebVTT or SubRip file in a storage
    /// bucket. The file is normalised to WebVTT and segmented in the background.
    /// Subscribe to `videos.[videoId].captions.[captionId].update` for `started`,
    /// `ready`, and `error` status changes instead of only polling the caption
    /// document.
    ///
    /// Uploaded tracks are listed alongside auto-extracted embedded tracks
    /// (`embedded: true`). When the current default is an auto-extracted track for
    /// the same language code, the upload takes the default flag so players pick
    /// the authored file first. Auto-extracted tracks are never removed
    /// automatically — delete a track explicitly to remove it. Extraction runs
    /// once per video, so a deleted extracted track is not re-created.
    pub async fn create_caption(
        &self,
        video_id: impl Into<String>,
        bucket_id: impl Into<String>,
        file_id: impl Into<String>,
        name: impl Into<String>,
        language: impl Into<String>,
        default: Option<bool>,
    ) -> crate::error::Result<crate::models::VideoCaption> {
        let mut params = HashMap::new();
        params.insert("bucketId".to_string(), json!(bucket_id.into()));
        params.insert("fileId".to_string(), json!(file_id.into()));
        params.insert("name".to_string(), json!(name.into()));
        params.insert("language".to_string(), json!(language.into()));
        if let Some(value) = default {
            params.insert("default".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let video_id = video_id.into();
        if video_id.is_empty() {
            return Err(crate::error::AppwriteError::new(
                0,
                "Missing required parameter: \"videoId\"",
                None,
                String::new(),
            ));
        }

        let path = "/videos/{videoId}/captions"
            .to_string()
            .replace("{videoId}", &Client::encode_path(&video_id));

        self.client
            .call(Method::POST, &path, Some(api_headers), Some(params))
            .await
    }

    /// Update a caption track by its unique ID. Pass only `name`, `language`,
    /// and/or `default` to retag an extracted track (for example `und` → `heb`
    /// for Hebrew) without replacing the file. Changing `bucketId`/`fileId`
    /// re-packages the track in the background.
    #[allow(clippy::too_many_arguments)]
    pub async fn update_caption(
        &self,
        video_id: impl Into<String>,
        caption_id: impl Into<String>,
        bucket_id: Option<&str>,
        file_id: Option<&str>,
        name: Option<&str>,
        language: Option<&str>,
        default: Option<bool>,
    ) -> crate::error::Result<crate::models::VideoCaption> {
        let mut params = HashMap::new();
        if let Some(value) = bucket_id {
            params.insert("bucketId".to_string(), json!(value));
        }
        if let Some(value) = file_id {
            params.insert("fileId".to_string(), json!(value));
        }
        if let Some(value) = name {
            params.insert("name".to_string(), json!(value));
        }
        if let Some(value) = language {
            params.insert("language".to_string(), json!(value));
        }
        if let Some(value) = default {
            params.insert("default".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let video_id = video_id.into();
        if video_id.is_empty() {
            return Err(crate::error::AppwriteError::new(
                0,
                "Missing required parameter: \"videoId\"",
                None,
                String::new(),
            ));
        }

        let caption_id = caption_id.into();
        if caption_id.is_empty() {
            return Err(crate::error::AppwriteError::new(
                0,
                "Missing required parameter: \"captionId\"",
                None,
                String::new(),
            ));
        }

        let path = "/videos/{videoId}/captions/{captionId}"
            .to_string()
            .replace("{videoId}", &Client::encode_path(&video_id))
            .replace("{captionId}", &Client::encode_path(&caption_id));

        self.client
            .call(Method::PATCH, &path, Some(api_headers), Some(params))
            .await
    }

    /// Delete a caption track by its unique ID, along with its packaged segments.
    pub async fn delete_caption(
        &self,
        video_id: impl Into<String>,
        caption_id: impl Into<String>,
    ) -> crate::error::Result<()> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let video_id = video_id.into();
        if video_id.is_empty() {
            return Err(crate::error::AppwriteError::new(
                0,
                "Missing required parameter: \"videoId\"",
                None,
                String::new(),
            ));
        }

        let caption_id = caption_id.into();
        if caption_id.is_empty() {
            return Err(crate::error::AppwriteError::new(
                0,
                "Missing required parameter: \"captionId\"",
                None,
                String::new(),
            ));
        }

        let path = "/videos/{videoId}/captions/{captionId}"
            .to_string()
            .replace("{videoId}", &Client::encode_path(&video_id))
            .replace("{captionId}", &Client::encode_path(&caption_id));

        self.client
            .call(Method::DELETE, &path, Some(api_headers), Some(params))
            .await
    }

    /// Get the top-level streaming manifest for a video output: an HLS master
    /// playlist (`master.m3u8`) or a DASH MPD (`master.mpd`). Hand this URL to a
    /// player to begin adaptive playback.
    ///
    /// Guests can request this URL when they hold `videos.play` (or `videos.read`)
    /// **and** the video document grants them read. A video with `read("any")` or
    /// `read("guests")` is playable without a session.
    ///
    /// For CMAF renditions, use `getCmafHlsManifest` or `getCmafDashManifest`.
    /// Both masters point at the same fMP4 segment set.
    pub async fn get_cmaf_hls_manifest(
        &self,
        video_id: impl Into<String>,
    ) -> crate::error::Result<Vec<u8>> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("accept".to_string(), "*/*".to_string());

        let video_id = video_id.into();
        if video_id.is_empty() {
            return Err(crate::error::AppwriteError::new(
                0,
                "Missing required parameter: \"videoId\"",
                None,
                String::new(),
            ));
        }

        let path = "/videos/{videoId}/outputs/cmaf/master.m3u8"
            .to_string()
            .replace("{videoId}", &Client::encode_path(&video_id));

        self.client
            .call_bytes(Method::GET, &path, Some(api_headers), Some(params))
            .await
    }

    /// Get the top-level streaming manifest for a video output: an HLS master
    /// playlist (`master.m3u8`) or a DASH MPD (`master.mpd`). Hand this URL to a
    /// player to begin adaptive playback.
    ///
    /// Guests can request this URL when they hold `videos.play` (or `videos.read`)
    /// **and** the video document grants them read. A video with `read("any")` or
    /// `read("guests")` is playable without a session.
    ///
    /// For CMAF renditions, use `getCmafHlsManifest` or `getCmafDashManifest`.
    /// Both masters point at the same fMP4 segment set.
    pub async fn get_cmaf_dash_manifest(
        &self,
        video_id: impl Into<String>,
    ) -> crate::error::Result<Vec<u8>> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("accept".to_string(), "*/*".to_string());

        let video_id = video_id.into();
        if video_id.is_empty() {
            return Err(crate::error::AppwriteError::new(
                0,
                "Missing required parameter: \"videoId\"",
                None,
                String::new(),
            ));
        }

        let path = "/videos/{videoId}/outputs/cmaf/master.mpd"
            .to_string()
            .replace("{videoId}", &Client::encode_path(&video_id));

        self.client
            .call_bytes(Method::GET, &path, Some(api_headers), Some(params))
            .await
    }

    /// Get the top-level streaming manifest for a video output: an HLS master
    /// playlist (`master.m3u8`) or a DASH MPD (`master.mpd`). Hand this URL to a
    /// player to begin adaptive playback.
    ///
    /// Guests can request this URL when they hold `videos.play` (or `videos.read`)
    /// **and** the video document grants them read. A video with `read("any")` or
    /// `read("guests")` is playable without a session.
    ///
    /// For CMAF renditions, use `getCmafHlsManifest` or `getCmafDashManifest`.
    /// Both masters point at the same fMP4 segment set.
    pub async fn get_dash_manifest(
        &self,
        video_id: impl Into<String>,
    ) -> crate::error::Result<Vec<u8>> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("accept".to_string(), "*/*".to_string());

        let video_id = video_id.into();
        if video_id.is_empty() {
            return Err(crate::error::AppwriteError::new(
                0,
                "Missing required parameter: \"videoId\"",
                None,
                String::new(),
            ));
        }

        let path = "/videos/{videoId}/outputs/dash/master.mpd"
            .to_string()
            .replace("{videoId}", &Client::encode_path(&video_id));

        self.client
            .call_bytes(Method::GET, &path, Some(api_headers), Some(params))
            .await
    }

    /// Get the top-level streaming manifest for a video output: an HLS master
    /// playlist (`master.m3u8`) or a DASH MPD (`master.mpd`). Hand this URL to a
    /// player to begin adaptive playback.
    ///
    /// Guests can request this URL when they hold `videos.play` (or `videos.read`)
    /// **and** the video document grants them read. A video with `read("any")` or
    /// `read("guests")` is playable without a session.
    ///
    /// For CMAF renditions, use `getCmafHlsManifest` or `getCmafDashManifest`.
    /// Both masters point at the same fMP4 segment set.
    pub async fn get_hls_manifest(
        &self,
        video_id: impl Into<String>,
    ) -> crate::error::Result<Vec<u8>> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("accept".to_string(), "*/*".to_string());

        let video_id = video_id.into();
        if video_id.is_empty() {
            return Err(crate::error::AppwriteError::new(
                0,
                "Missing required parameter: \"videoId\"",
                None,
                String::new(),
            ));
        }

        let path = "/videos/{videoId}/outputs/hls/master.m3u8"
            .to_string()
            .replace("{videoId}", &Client::encode_path(&video_id));

        self.client
            .call_bytes(Method::GET, &path, Some(api_headers), Some(params))
            .await
    }

    /// Get the playable caption resource for a video output: an HLS captions
    /// playlist, or the WebVTT file itself for DASH.
    ///
    /// Guests can request this URL when they hold `videos.play` (or `videos.read`)
    /// **and** the video document grants them read.
    pub async fn get_caption_manifest(
        &self,
        video_id: impl Into<String>,
        output: crate::enums::VideoOutput,
        caption_id: impl Into<String>,
    ) -> crate::error::Result<Vec<u8>> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("accept".to_string(), "*/*".to_string());

        let video_id = video_id.into();
        if video_id.is_empty() {
            return Err(crate::error::AppwriteError::new(
                0,
                "Missing required parameter: \"videoId\"",
                None,
                String::new(),
            ));
        }

        let caption_id = caption_id.into();
        if caption_id.is_empty() {
            return Err(crate::error::AppwriteError::new(
                0,
                "Missing required parameter: \"captionId\"",
                None,
                String::new(),
            ));
        }

        let path = "/videos/{videoId}/outputs/{output}/captions/{captionId}/manifest"
            .to_string()
            .replace("{videoId}", &Client::encode_path(&video_id))
            .replace("{output}", &Client::encode_path(&output.to_string()))
            .replace("{captionId}", &Client::encode_path(&caption_id));

        self.client
            .call_bytes(Method::GET, &path, Some(api_headers), Some(params))
            .await
    }

    /// Get a list of all the renditions of a video. You can use the query params
    /// to filter your results.
    pub async fn list_renditions(
        &self,
        video_id: impl Into<String>,
        queries: Option<Vec<String>>,
        total: Option<bool>,
    ) -> crate::error::Result<crate::models::VideoRenditionList> {
        let mut params = HashMap::new();
        if let Some(value) = queries {
            params.insert(
                "queries".to_string(),
                json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()),
            );
        }
        if let Some(value) = total {
            params.insert("total".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let video_id = video_id.into();
        if video_id.is_empty() {
            return Err(crate::error::AppwriteError::new(
                0,
                "Missing required parameter: \"videoId\"",
                None,
                String::new(),
            ));
        }

        let path = "/videos/{videoId}/renditions"
            .to_string()
            .replace("{videoId}", &Client::encode_path(&video_id));

        self.client
            .call(Method::GET, &path, Some(api_headers), Some(params))
            .await
    }

    /// Request a new rendition of a video, encoded against a video profile and
    /// packaged for HLS, DASH, or CMAF. The rendition is created immediately with
    /// a `pending` status and transcoded in the background; poll it or subscribe
    /// to realtime events to follow its progress. The worker downloads the source
    /// file for this job, probes metadata and extracts embedded captions when
    /// needed, then deletes the working copy when the job ends.
    ///
    /// The profile codec must be enabled on this instance. A disabled codec fails
    /// with `video_codec_disabled`.
    ///
    /// Each video may have only one rendition per profile and output combination.
    /// Creating a duplicate fails with `video_rendition_already_exists`. After an
    /// encode fails or is aborted, delete the rendition and create it again. The
    /// same profile may still be encoded for different outputs (for example HLS
    /// and DASH).
    ///
    /// A video accepts 10 rendition creates per hour. The next create returns
    /// `general_rate_limit_exceeded`.
    ///
    /// CMAF packs shared fMP4 segments once and exposes both HLS
    /// (`/outputs/cmaf/master.m3u8`) and DASH (`/outputs/cmaf/master.mpd`) masters
    /// for the same encode.
    ///
    /// Renditions can only be created from the Console or a server SDK with an API
    /// key that has the `videos.write` scope. Each encode spends the project's
    /// compute, so client sessions receive `user_unauthorized`.
    pub async fn create_rendition(
        &self,
        video_id: impl Into<String>,
        profile_id: impl Into<String>,
        output: crate::enums::VideoOutput,
    ) -> crate::error::Result<crate::models::VideoRendition> {
        let mut params = HashMap::new();
        params.insert("profileId".to_string(), json!(profile_id.into()));
        params.insert("output".to_string(), json!(output));
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let video_id = video_id.into();
        if video_id.is_empty() {
            return Err(crate::error::AppwriteError::new(
                0,
                "Missing required parameter: \"videoId\"",
                None,
                String::new(),
            ));
        }

        let path = "/videos/{videoId}/renditions"
            .to_string()
            .replace("{videoId}", &Client::encode_path(&video_id));

        self.client
            .call(Method::POST, &path, Some(api_headers), Some(params))
            .await
    }

    /// Get a rendition by its unique ID, including its current transcoding status
    /// and progress.
    pub async fn get_rendition(
        &self,
        video_id: impl Into<String>,
        rendition_id: impl Into<String>,
    ) -> crate::error::Result<crate::models::VideoRendition> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let video_id = video_id.into();
        if video_id.is_empty() {
            return Err(crate::error::AppwriteError::new(
                0,
                "Missing required parameter: \"videoId\"",
                None,
                String::new(),
            ));
        }

        let rendition_id = rendition_id.into();
        if rendition_id.is_empty() {
            return Err(crate::error::AppwriteError::new(
                0,
                "Missing required parameter: \"renditionId\"",
                None,
                String::new(),
            ));
        }

        let path = "/videos/{videoId}/renditions/{renditionId}"
            .to_string()
            .replace("{videoId}", &Client::encode_path(&video_id))
            .replace("{renditionId}", &Client::encode_path(&rendition_id));

        self.client
            .call(Method::GET, &path, Some(api_headers), Some(params))
            .await
    }

    /// Delete a rendition by its unique ID, along with its packaged segments and
    /// transcoded output. Fails with `video_rendition_in_progress` while the
    /// rendition is started or ended, unless it has not been updated within the
    /// rendition retention window.
    pub async fn delete_rendition(
        &self,
        video_id: impl Into<String>,
        rendition_id: impl Into<String>,
    ) -> crate::error::Result<()> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let video_id = video_id.into();
        if video_id.is_empty() {
            return Err(crate::error::AppwriteError::new(
                0,
                "Missing required parameter: \"videoId\"",
                None,
                String::new(),
            ));
        }

        let rendition_id = rendition_id.into();
        if rendition_id.is_empty() {
            return Err(crate::error::AppwriteError::new(
                0,
                "Missing required parameter: \"renditionId\"",
                None,
                String::new(),
            ));
        }

        let path = "/videos/{videoId}/renditions/{renditionId}"
            .to_string()
            .replace("{videoId}", &Client::encode_path(&video_id))
            .replace("{renditionId}", &Client::encode_path(&rendition_id));

        self.client
            .call(Method::DELETE, &path, Some(api_headers), Some(params))
            .await
    }

    /// Get a single thumbnail sheet from a video's timeline as an image. You can
    /// resize the returned image and choose its output format.
    ///
    /// Guests can request this URL when they hold `videos.play` (or `videos.read`)
    /// **and** the video document grants them read.
    pub async fn get_thumbnail(
        &self,
        video_id: impl Into<String>,
        thumbnail_id: impl Into<String>,
        width: Option<i64>,
        height: Option<i64>,
        output: Option<crate::enums::ImageFormat>,
    ) -> crate::error::Result<Vec<u8>> {
        let mut params = HashMap::new();
        if let Some(value) = width {
            params.insert("width".to_string(), json!(value));
        }
        if let Some(value) = height {
            params.insert("height".to_string(), json!(value));
        }
        if let Some(value) = output {
            params.insert("output".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("accept".to_string(), "image/*".to_string());

        let video_id = video_id.into();
        if video_id.is_empty() {
            return Err(crate::error::AppwriteError::new(
                0,
                "Missing required parameter: \"videoId\"",
                None,
                String::new(),
            ));
        }

        let thumbnail_id = thumbnail_id.into();
        if thumbnail_id.is_empty() {
            return Err(crate::error::AppwriteError::new(
                0,
                "Missing required parameter: \"thumbnailId\"",
                None,
                String::new(),
            ));
        }

        let path = "/videos/{videoId}/thumbnails/{thumbnailId}"
            .to_string()
            .replace("{videoId}", &Client::encode_path(&video_id))
            .replace("{thumbnailId}", &Client::encode_path(&thumbnail_id));

        self.client
            .call_bytes(Method::GET, &path, Some(api_headers), Some(params))
            .await
    }

    /// Get the WebVTT thumbnail timeline for a video, used to render scrubbing
    /// thumbnails in a player. Timeline generation is queued when the video is
    /// created. Subscribe to `videos.[videoId].timelines.update` for probe and
    /// terminal updates, or poll this endpoint until it returns a WebVTT document.
    /// Audio-only sources produce no timeline and this endpoint returns
    /// `video_timelines_not_found`.
    ///
    /// Guests can request this URL when they hold `videos.play` (or `videos.read`)
    /// **and** the video document grants them read. A video with `read("any")` or
    /// `read("guests")` is playable without a session.
    pub async fn get_timeline(&self, video_id: impl Into<String>) -> crate::error::Result<Vec<u8>> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("accept".to_string(), "*/*".to_string());

        let video_id = video_id.into();
        if video_id.is_empty() {
            return Err(crate::error::AppwriteError::new(
                0,
                "Missing required parameter: \"videoId\"",
                None,
                String::new(),
            ));
        }

        let path = "/videos/{videoId}/timeline"
            .to_string()
            .replace("{videoId}", &Client::encode_path(&video_id));

        self.client
            .call_bytes(Method::GET, &path, Some(api_headers), Some(params))
            .await
    }
}

impl crate::services::Service for Videos {
    fn client(&self) -> &Client {
        &self.client
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_videos_creation() {
        let client = Client::new();
        let service = Videos::new(&client);
        assert!(service.client().endpoint().contains("cloud.appwrite.io/v1"));
    }
}
