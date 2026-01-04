use futures::stream::BoxStream;
use futures_util::stream::StreamExt;
use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue, ACCEPT, CONTENT_TYPE, WWW_AUTHENTICATE},
    Client,
};
use rmcp::{
    model::{CallToolRequestParam, ClientInfo},
    service::{RoleClient, RunningService},
    transport::{
        common::http_header::{
            EVENT_STREAM_MIME_TYPE, HEADER_LAST_EVENT_ID, HEADER_SESSION_ID, JSON_MIME_TYPE,
        },
        streamable_http_client::{
            AuthRequiredError, StreamableHttpClient, StreamableHttpClientTransport,
            StreamableHttpClientTransportConfig, StreamableHttpError, StreamableHttpPostResponse,
        },
    },
    ServiceExt,
};
use serde::Serialize;
use serde_json::{Map, Value};
use sse_stream::{Sse, SseStream};
use std::borrow::Cow;
use std::{fmt, sync::Arc};

#[derive(Debug)]
pub struct McpSession {
    client: Arc<RunningService<RoleClient, ClientInfo>>,
}

impl McpSession {
    pub async fn connect(url: String, headers: Vec<(String, String)>) -> Result<Self, String> {
        let client = build_http_client_with_headers(&headers)?;
        let transport = StreamableHttpClientTransport::with_client(
            RelaxedReqwestClient::new(client),
            StreamableHttpClientTransportConfig::with_uri(url),
        );
        let service = ClientInfo::default()
            .serve(transport)
            .await
            .map_err(|err| err.to_string())?;
        Ok(Self {
            client: Arc::new(service),
        })
    }

    pub async fn list_tools(&self) -> Result<Vec<ToolView>, String> {
        let tools = self
            .client
            .list_all_tools()
            .await
            .map_err(|err| err.to_string())?;
        Ok(tools.into_iter().map(ToolView::from).collect())
    }

    pub async fn list_resources(&self) -> Result<Vec<ResourceView>, String> {
        let resources = self
            .client
            .list_all_resources()
            .await
            .map_err(|err| err.to_string())?;
        Ok(resources.into_iter().map(ResourceView::from).collect())
    }

    pub async fn list_prompts(&self) -> Result<Vec<PromptView>, String> {
        let prompts = self
            .client
            .list_all_prompts()
            .await
            .map_err(|err| err.to_string())?;
        Ok(prompts.into_iter().map(PromptView::from).collect())
    }

    pub async fn call_tool(
        &self,
        name: String,
        arguments: Option<Map<String, Value>>,
    ) -> Result<Value, String> {
        let params = CallToolRequestParam {
            name: name.into(),
            arguments,
        };
        let result = self
            .client
            .call_tool(params)
            .await
            .map_err(|err| err.to_string())?;
        serde_json::to_value(result).map_err(|err| err.to_string())
    }
}

#[derive(Serialize)]
pub struct ToolView {
    pub name: String,
    pub description: Option<String>,
    pub title: Option<String>,
    pub input_schema: Option<Value>,
}

impl From<rmcp::model::Tool> for ToolView {
    fn from(tool: rmcp::model::Tool) -> Self {
        let input_schema = Some(Value::Object(tool.input_schema.as_ref().clone()));
        Self {
            name: tool.name.into_owned(),
            description: tool.description.map(|desc| desc.into_owned()),
            title: tool.title,
            input_schema,
        }
    }
}

#[derive(Serialize)]
pub struct ResourceView {
    pub uri: String,
    pub name: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub mime_type: Option<String>,
}

impl From<rmcp::model::Resource> for ResourceView {
    fn from(resource: rmcp::model::Resource) -> Self {
        Self {
            uri: resource.uri.clone(),
            name: resource.name.clone(),
            title: resource.title.clone(),
            description: resource.description.clone(),
            mime_type: resource.mime_type.clone(),
        }
    }
}

#[derive(Serialize)]
pub struct PromptView {
    pub name: String,
    pub description: Option<String>,
}

impl From<rmcp::model::Prompt> for PromptView {
    fn from(prompt: rmcp::model::Prompt) -> Self {
        Self {
            name: prompt.name,
            description: prompt.description,
        }
    }
}

pub fn build_http_client_with_headers(headers: &[(String, String)]) -> Result<Client, String> {
    let mut builder = Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .user_agent("MCP-Inspector/1.0 (Windows; Tauri)")
        .http1_only() // 强制使用 HTTP/1.1，避免部分服务器 HTTP/2 协商失败导致 Transport closed
        .tcp_nodelay(true)
        .pool_idle_timeout(std::time::Duration::from_secs(30))
        .pool_max_idle_per_host(5);

    let mut header_map = HeaderMap::new();
    for (name, value) in headers {
        let name = name.trim();
        let value = value.trim();
        if name.is_empty() {
            return Err("header name may not be empty".to_string());
        }
        let header_name = HeaderName::from_bytes(name.as_bytes())
            .map_err(|_| format!("invalid header name: {name}"))?;
        let header_value =
            HeaderValue::from_str(value).map_err(|_| format!("invalid header value for {name}"))?;
        header_map.append(header_name, header_value);
    }

    if !header_map.is_empty() {
        builder = builder.default_headers(header_map);
    }

    builder
        .build()
        .map_err(|err| format!("Failed to create HTTP client: {err}"))
}

#[derive(Clone)]
pub struct RelaxedReqwestClient {
    inner: Client,
}

impl RelaxedReqwestClient {
    pub fn new(inner: Client) -> Self {
        Self { inner }
    }
}

impl StreamableHttpClient for RelaxedReqwestClient {
    type Error = reqwest::Error;

    fn post_message(
        &self,
        uri: Arc<str>,
        message: rmcp::model::ClientJsonRpcMessage,
        session_id: Option<Arc<str>>,
        auth_header: Option<String>,
    ) -> impl std::future::Future<
        Output = Result<StreamableHttpPostResponse, StreamableHttpError<Self::Error>>,
    > + Send
           + '_ {
        let client = self.inner.clone();
        async move {
            let mut request = client
                .post(uri.as_ref())
                .header(ACCEPT, [EVENT_STREAM_MIME_TYPE, JSON_MIME_TYPE].join(", "));
            if let Some(auth_header) = auth_header {
                request = request.bearer_auth(auth_header);
            }
            if let Some(session_id) = session_id {
                request = request.header(HEADER_SESSION_ID, session_id.as_ref());
            }
            let response = request.json(&message).send().await?;
            if response.status() == reqwest::StatusCode::UNAUTHORIZED {
                if let Some(header) = response.headers().get(WWW_AUTHENTICATE) {
                    let header = header.to_str().map_err(|_| {
                        StreamableHttpError::UnexpectedServerResponse(Cow::from(
                            "invalid www-authenticate header value",
                        ))
                    })?;
                    let header = header.to_string();
                    return Err(StreamableHttpError::AuthRequired(AuthRequiredError {
                        www_authenticate_header: header,
                    }));
                }
            }
            let status = response.status();
            let response = response.error_for_status()?;
            if matches!(
                status,
                reqwest::StatusCode::ACCEPTED | reqwest::StatusCode::NO_CONTENT
            ) {
                return Ok(StreamableHttpPostResponse::Accepted);
            }
            let content_type = response.headers().get(CONTENT_TYPE);
            let session_id = response
                .headers()
                .get(HEADER_SESSION_ID)
                .and_then(|v| v.to_str().ok())
                .map(|s| s.to_string());
            match content_type {
                Some(ct) if ct.as_bytes().starts_with(EVENT_STREAM_MIME_TYPE.as_bytes()) => {
                    let event_stream = SseStream::from_byte_stream(response.bytes_stream()).boxed();
                    Ok(StreamableHttpPostResponse::Sse(event_stream, session_id))
                }
                Some(ct) if ct.as_bytes().starts_with(JSON_MIME_TYPE.as_bytes()) => {
                    let message: rmcp::model::ServerJsonRpcMessage = response.json().await?;
                    Ok(StreamableHttpPostResponse::Json(message, session_id))
                }
                Some(ct) => Err(StreamableHttpError::UnexpectedContentType(Some(
                    String::from_utf8_lossy(ct.as_bytes()).to_string(),
                ))),
                None => {
                    let event_stream = SseStream::from_byte_stream(response.bytes_stream()).boxed();
                    Ok(StreamableHttpPostResponse::Sse(event_stream, session_id))
                }
            }
        }
    }

    fn delete_session(
        &self,
        uri: Arc<str>,
        session_id: Arc<str>,
        auth_header: Option<String>,
    ) -> impl std::future::Future<Output = Result<(), StreamableHttpError<Self::Error>>> + Send + '_
    {
        let client = self.inner.clone();
        async move {
            let mut request = client.delete(uri.as_ref());
            if let Some(auth_header) = auth_header {
                request = request.bearer_auth(auth_header);
            }
            let response = request
                .header(HEADER_SESSION_ID, session_id.as_ref())
                .send()
                .await?;
            if response.status() == reqwest::StatusCode::METHOD_NOT_ALLOWED {
                return Ok(());
            }
            let _ = response.error_for_status()?;
            Ok(())
        }
    }

    fn get_stream(
        &self,
        uri: Arc<str>,
        session_id: Arc<str>,
        last_event_id: Option<String>,
        auth_header: Option<String>,
    ) -> impl std::future::Future<
        Output = Result<
            BoxStream<'static, Result<Sse, sse_stream::Error>>,
            StreamableHttpError<Self::Error>,
        >,
    > + Send
           + '_ {
        let client = self.inner.clone();
        async move {
            let mut request = client
                .get(uri.as_ref())
                .header(ACCEPT, EVENT_STREAM_MIME_TYPE)
                .header(HEADER_SESSION_ID, session_id.as_ref());
            if let Some(last_event_id) = last_event_id {
                request = request.header(HEADER_LAST_EVENT_ID, last_event_id);
            }
            if let Some(auth_header) = auth_header {
                request = request.bearer_auth(auth_header);
            }
            let response = request.send().await?;
            if response.status() == reqwest::StatusCode::METHOD_NOT_ALLOWED {
                return Err(StreamableHttpError::ServerDoesNotSupportSse);
            }
            let response = response.error_for_status()?;
            if let Some(ct) = response.headers().get(CONTENT_TYPE) {
                if !ct.as_bytes().starts_with(EVENT_STREAM_MIME_TYPE.as_bytes()) {
                    return Err(StreamableHttpError::UnexpectedContentType(Some(
                        String::from_utf8_lossy(ct.as_bytes()).to_string(),
                    )));
                }
            }
            let event_stream = SseStream::from_byte_stream(response.bytes_stream()).boxed();
            Ok(event_stream)
        }
    }
}

impl fmt::Debug for RelaxedReqwestClient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RelaxedReqwestClient").finish()
    }
}
