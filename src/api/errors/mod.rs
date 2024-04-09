mod sentry_error;

use std::fmt;

pub(super) use self::sentry_error::SentryError;

#[derive(Clone, Debug, thiserror::Error)]
#[error("project was renamed to '{0}'\nPlease use this slug in your .sentryclirc file, sentry.properties file or in the CLI --project parameter")]
pub(super) struct ProjectRenamedError(pub(super) String);

/// Represents API errors.
#[derive(Copy, Clone, Eq, PartialEq, Debug, thiserror::Error)]
pub enum ApiErrorKind {
    #[error("could not serialize value as JSON")]
    CannotSerializeAsJson,
    #[error("could not parse JSON response")]
    BadJson,
    #[error("not a JSON response")]
    NotJson,
    #[error("request failed because API URL was incorrectly formatted")]
    BadApiUrl,
    #[error("organization not found")]
    OrganizationNotFound,
    #[error("resource not found")]
    ResourceNotFound,
    #[error("Project not found. Please check that you entered the project and organization slugs correctly.")]
    ProjectNotFound,
    #[error("release not found")]
    ReleaseNotFound,
    #[error("chunk upload endpoint not supported by sentry server")]
    ChunkUploadNotSupported,
    #[error("API request failed")]
    RequestFailed,
    #[error("could not compress data")]
    CompressionFailed,
    #[error("region overrides cannot be applied to absolute urls")]
    InvalidRegionRequest,
    #[error(
        "Auth token is required for this request. Please run `sentry-cli login` and try again!"
    )]
    AuthMissing,
}

#[derive(Debug, thiserror::Error)]
pub struct ApiError {
    inner: ApiErrorKind,
    #[source]
    source: Option<anyhow::Error>,
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.inner, f)
    }
}

impl ApiError {
    pub fn with_source<E: Into<anyhow::Error>>(kind: ApiErrorKind, source: E) -> ApiError {
        ApiError {
            inner: kind,
            source: Some(source.into()),
        }
    }

    pub fn kind(&self) -> ApiErrorKind {
        self.inner
    }

    fn set_source<E: Into<anyhow::Error>>(mut self, source: E) -> ApiError {
        self.source = Some(source.into());
        self
    }
}

impl From<ApiErrorKind> for ApiError {
    fn from(kind: ApiErrorKind) -> ApiError {
        ApiError {
            inner: kind,
            source: None,
        }
    }
}

impl From<curl::Error> for ApiError {
    fn from(err: curl::Error) -> ApiError {
        ApiError::from(ApiErrorKind::RequestFailed).set_source(err)
    }
}

impl From<curl::FormError> for ApiError {
    fn from(err: curl::FormError) -> ApiError {
        ApiError::from(ApiErrorKind::RequestFailed).set_source(err)
    }
}

/// Shortcut alias for results of this module.
pub(super) type ApiResult<T> = Result<T, ApiError>;
