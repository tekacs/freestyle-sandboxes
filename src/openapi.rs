#[allow(unused_imports)]
use progenitor_client::{encode_path, ClientHooks, OperationInfo, RequestBuilderExt};
#[allow(unused_imports)]
pub use progenitor_client::{ByteStream, ClientInfo, Error, ResponseValue};
/// Types used as operation parameters and responses.
#[allow(clippy::all)]
pub mod types {
    /// Error types.
    pub mod error {
        /// Error from a `TryFrom` or `FromStr` implementation.
        pub struct ConversionError(::std::borrow::Cow<'static, str>);
        impl ::std::error::Error for ConversionError {}
        impl ::std::fmt::Display for ConversionError {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
                ::std::fmt::Display::fmt(&self.0, f)
            }
        }

        impl ::std::fmt::Debug for ConversionError {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
                ::std::fmt::Debug::fmt(&self.0, f)
            }
        }

        impl From<&'static str> for ConversionError {
            fn from(value: &'static str) -> Self {
                Self(value.into())
            }
        }

        impl From<String> for ConversionError {
            fn from(value: String) -> Self {
                Self(value.into())
            }
        }
    }

    ///`AccessLevel`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "read",
    ///    "write"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AccessLevel {
        #[serde(rename = "read")]
        Read,
        #[serde(rename = "write")]
        Write,
    }

    impl ::std::fmt::Display for AccessLevel {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Read => f.write_str("read"),
                Self::Write => f.write_str("write"),
            }
        }
    }

    impl ::std::str::FromStr for AccessLevel {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "read" => Ok(Self::Read),
                "write" => Ok(Self::Write),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AccessLevel {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for AccessLevel {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AccessLevel {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AccessTokenInfo`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "id"
    ///  ],
    ///  "properties": {
    ///    "id": {
    ///      "type": "string",
    ///      "format": "uuid"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct AccessTokenInfo {
        pub id: ::uuid::Uuid,
    }

    ///Identical to [`RepositoryInfo`], but with the permissions field added.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Identical to [`RepositoryInfo`], but with the
    /// permissions field added.",
    ///  "type": "object",
    ///  "required": [
    ///    "accountId",
    ///    "id",
    ///    "permissions",
    ///    "visibility"
    ///  ],
    ///  "properties": {
    ///    "accountId": {
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "id": {
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "name": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "permissions": {
    ///      "$ref": "#/components/schemas/AccessLevel"
    ///    },
    ///    "visibility": {
    ///      "$ref": "#/components/schemas/Visibility"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct AccessibleRepository {
        #[serde(rename = "accountId")]
        pub account_id: ::uuid::Uuid,
        pub id: ::uuid::Uuid,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
        pub permissions: AccessLevel,
        pub visibility: Visibility,
    }

    ///Account identifier — either "system" or a user UUID
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Account identifier — either \"system\" or a user UUID",
    ///  "type": "string"
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    #[serde(transparent)]
    pub struct AccountId(pub ::std::string::String);
    impl ::std::ops::Deref for AccountId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<AccountId> for ::std::string::String {
        fn from(value: AccountId) -> Self {
            value.0
        }
    }

    impl ::std::convert::From<::std::string::String> for AccountId {
        fn from(value: ::std::string::String) -> Self {
            Self(value)
        }
    }

    impl ::std::str::FromStr for AccountId {
        type Err = ::std::convert::Infallible;
        fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::fmt::Display for AccountId {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            self.0.fmt(f)
        }
    }

    ///Specifies a Docker base image via inline Dockerfile content.
    ///Used as an alternative to `docker_image` when you want to build
    ///a custom image on the fly without a pre-published registry image.
    ///No build context is supported yet.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Specifies a Docker base image via inline Dockerfile
    /// content.\nUsed as an alternative to `docker_image` when you want to
    /// build\na custom image on the fly without a pre-published registry
    /// image.\nNo build context is supported yet.",
    ///  "type": "object",
    ///  "required": [
    ///    "dockerfileContent"
    ///  ],
    ///  "properties": {
    ///    "dockerfileContent": {
    ///      "description": "The text content of a Dockerfile (e.g., `\"FROM
    /// debian:trixie-slim\"`).",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct BaseImageSpec {
        ///The text content of a Dockerfile (e.g., `"FROM
        /// debian:trixie-slim"`).
        #[serde(rename = "dockerfileContent")]
        pub dockerfile_content: ::std::string::String,
    }

    ///`BatchRestartServicesResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "error",
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "error": {
    ///      "description": "Error code in SCREAMING_SNAKE_CASE",
    ///      "type": "string"
    ///    },
    ///    "message": {
    ///      "description": "Human-readable error message",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct BatchRestartServicesResponse {
        ///Error code in SCREAMING_SNAKE_CASE
        pub error: ::std::string::String,
        ///Human-readable error message
        pub message: ::std::string::String,
    }

    ///`BatchServiceOperationResult`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "message",
    ///    "serviceId",
    ///    "success"
    ///  ],
    ///  "properties": {
    ///    "message": {
    ///      "type": "string"
    ///    },
    ///    "serviceId": {
    ///      "type": "string"
    ///    },
    ///    "success": {
    ///      "type": "boolean"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct BatchServiceOperationResult {
        pub message: ::std::string::String,
        #[serde(rename = "serviceId")]
        pub service_id: ::std::string::String,
        pub success: bool,
    }

    ///`BatchServiceRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "services"
    ///  ],
    ///  "properties": {
    ///    "services": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/ServiceIdItem"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct BatchServiceRequest {
        pub services: ::std::vec::Vec<ServiceIdItem>,
    }

    ///`BatchServiceResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "results"
    ///  ],
    ///  "properties": {
    ///    "results": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/BatchServiceOperationResult"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct BatchServiceResponse {
        pub results: ::std::vec::Vec<BatchServiceOperationResult>,
    }

    ///`BatchStartServicesResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "error",
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "error": {
    ///      "description": "Error code in SCREAMING_SNAKE_CASE",
    ///      "type": "string"
    ///    },
    ///    "message": {
    ///      "description": "Human-readable error message",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct BatchStartServicesResponse {
        ///Error code in SCREAMING_SNAKE_CASE
        pub error: ::std::string::String,
        ///Human-readable error message
        pub message: ::std::string::String,
    }

    ///`BatchStopServicesResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "error",
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "error": {
    ///      "description": "Error code in SCREAMING_SNAKE_CASE",
    ///      "type": "string"
    ///    },
    ///    "message": {
    ///      "description": "Human-readable error message",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct BatchStopServicesResponse {
        ///Error code in SCREAMING_SNAKE_CASE
        pub error: ::std::string::String,
        ///Human-readable error message
        pub message: ::std::string::String,
    }

    ///`Behavior`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "default": "exact",
    ///  "type": "string",
    ///  "enum": [
    ///    "regex",
    ///    "exact"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum Behavior {
        #[serde(rename = "regex")]
        Regex,
        #[serde(rename = "exact")]
        Exact,
    }

    impl ::std::fmt::Display for Behavior {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Regex => f.write_str("regex"),
                Self::Exact => f.write_str("exact"),
            }
        }
    }

    impl ::std::str::FromStr for Behavior {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "regex" => Ok(Self::Regex),
                "exact" => Ok(Self::Exact),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for Behavior {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for Behavior {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for Behavior {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::default::Default for Behavior {
        fn default() -> Self {
            Behavior::Exact
        }
    }

    ///The encoding of a blob from the API. Always `base64`.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The encoding of a blob from the API. Always `base64`.",
    ///  "type": "string",
    ///  "enum": [
    ///    "base64"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum BlobEncoding {
        #[serde(rename = "base64")]
        Base64,
    }

    impl ::std::fmt::Display for BlobEncoding {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Base64 => f.write_str("base64"),
            }
        }
    }

    impl ::std::str::FromStr for BlobEncoding {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "base64" => Ok(Self::Base64),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for BlobEncoding {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for BlobEncoding {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for BlobEncoding {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///Blob object
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Blob object",
    ///  "type": "object",
    ///  "required": [
    ///    "content",
    ///    "encoding",
    ///    "sha",
    ///    "size"
    ///  ],
    ///  "properties": {
    ///    "content": {
    ///      "description": "The content of the blob, base64 encoded.",
    ///      "type": "string"
    ///    },
    ///    "encoding": {
    ///      "$ref": "#/components/schemas/BlobEncoding"
    ///    },
    ///    "sha": {
    ///      "description": "The object's hash.",
    ///      "type": "string"
    ///    },
    ///    "size": {
    ///      "description": "The blob's size in bytes",
    ///      "type": "integer",
    ///      "format": "int64",
    ///      "minimum": 0.0
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct BlobObject {
        ///The content of the blob, base64 encoded.
        pub content: ::std::string::String,
        pub encoding: BlobEncoding,
        ///The object's hash.
        pub sha: ::std::string::String,
        ///The blob's size in bytes
        pub size: i64,
    }

    ///`BranchDetails`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "default",
    ///    "name"
    ///  ],
    ///  "properties": {
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "target": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct BranchDetails {
        pub default: ::serde_json::Value,
        pub name: ::std::string::String,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub target: ::std::option::Option<::std::string::String>,
    }

    ///`BranchInfo`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "name"
    ///  ],
    ///  "properties": {
    ///    "commit": {
    ///      "description": "The latest commit ID on this branch. Null if the
    /// branch is empty.",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct BranchInfo {
        ///The latest commit ID on this branch. Null if the branch is empty.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub commit: ::std::option::Option<::std::string::String>,
        pub name: ::std::string::String,
    }

    ///`CommitAuthor`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "email",
    ///    "name"
    ///  ],
    ///  "properties": {
    ///    "date": {
    ///      "description": "Optional date (if not provided, uses current
    /// time)",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ],
    ///      "format": "date-time"
    ///    },
    ///    "email": {
    ///      "description": "The email of the author/committer",
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "description": "The name of the author/committer",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct CommitAuthor {
        ///Optional date (if not provided, uses current time)
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub date: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        ///The email of the author/committer
        pub email: ::std::string::String,
        ///The name of the author/committer
        pub name: ::std::string::String,
    }

    ///`CommitAuthorInfo`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "email",
    ///    "name"
    ///  ],
    ///  "properties": {
    ///    "email": {
    ///      "description": "The email of the author/committer",
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "description": "The name of the author/committer",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct CommitAuthorInfo {
        ///The email of the author/committer
        pub email: ::std::string::String,
        ///The name of the author/committer
        pub name: ::std::string::String,
    }

    ///Response containing the comparison between two commits
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Response containing the comparison between two
    /// commits",
    ///  "type": "object",
    ///  "required": [
    ///    "ahead_by",
    ///    "behind_by",
    ///    "files",
    ///    "status",
    ///    "total_commits"
    ///  ],
    ///  "properties": {
    ///    "ahead_by": {
    ///      "description": "Number of commits the head is ahead of base",
    ///      "type": "integer",
    ///      "minimum": 0.0
    ///    },
    ///    "behind_by": {
    ///      "description": "Number of commits the head is behind base",
    ///      "type": "integer",
    ///      "minimum": 0.0
    ///    },
    ///    "files": {
    ///      "description": "List of changed files",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/DiffFile"
    ///      }
    ///    },
    ///    "status": {
    ///      "$ref": "#/components/schemas/ComparisonStatus"
    ///    },
    ///    "total_commits": {
    ///      "description": "Total number of commits in the comparison",
    ///      "type": "integer",
    ///      "minimum": 0.0
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct CommitComparison {
        ///Number of commits the head is ahead of base
        pub ahead_by: u64,
        ///Number of commits the head is behind base
        pub behind_by: u64,
        ///List of changed files
        pub files: ::std::vec::Vec<DiffFile>,
        pub status: ComparisonStatus,
        ///Total number of commits in the comparison
        pub total_commits: u64,
    }

    ///`CommitList`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "commits",
    ///    "count",
    ///    "limit",
    ///    "order",
    ///    "total"
    ///  ],
    ///  "properties": {
    ///    "commits": {
    ///      "description": "List of commits",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/CommitObject"
    ///      }
    ///    },
    ///    "count": {
    ///      "description": "Number of commits returned in this page",
    ///      "type": "integer",
    ///      "minimum": 0.0
    ///    },
    ///    "limit": {
    ///      "description": "Maximum number of commits requested (limit)",
    ///      "type": "integer",
    ///      "minimum": 0.0
    ///    },
    ///    "next_commit": {
    ///      "description": "SHA of the next commit for pagination. None if
    /// there are no more commits.\n- For `order=desc`: use `until=next_commit`
    /// to get the next page\n- For `order=asc`: use `since=next_commit` to get
    /// the next page",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "offset": {
    ///      "description": "Number of commits skipped (offset) - deprecated,
    /// use `since`/`until` selectors instead",
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ],
    ///      "minimum": 0.0
    ///    },
    ///    "order": {
    ///      "$ref": "#/components/schemas/CommitOrder"
    ///    },
    ///    "selector": {
    ///      "$ref": "#/components/schemas/CommitsSelector"
    ///    },
    ///    "total": {
    ///      "description": "Total number of commits available in the
    /// branch/range",
    ///      "type": "integer",
    ///      "minimum": 0.0
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct CommitList {
        ///List of commits
        pub commits: ::std::vec::Vec<CommitObject>,
        ///Number of commits returned in this page
        pub count: u64,
        ///Maximum number of commits requested (limit)
        pub limit: u64,
        ///SHA of the next commit for pagination. None if there are no more
        /// commits.
        /// - For `order=desc`: use `until=next_commit` to get the next page
        /// - For `order=asc`: use `since=next_commit` to get the next page
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub next_commit: ::std::option::Option<::std::string::String>,
        ///Number of commits skipped (offset) - deprecated, use `since`/`until`
        /// selectors instead
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub offset: ::std::option::Option<u64>,
        pub order: CommitOrder,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub selector: ::std::option::Option<CommitsSelector>,
        ///Total number of commits available in the branch/range
        pub total: u64,
    }

    ///Commit object
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Commit object",
    ///  "type": "object",
    ///  "required": [
    ///    "author",
    ///    "committer",
    ///    "message",
    ///    "parents",
    ///    "sha",
    ///    "tree"
    ///  ],
    ///  "properties": {
    ///    "author": {
    ///      "$ref": "#/components/schemas/Signature"
    ///    },
    ///    "committer": {
    ///      "$ref": "#/components/schemas/Signature"
    ///    },
    ///    "message": {
    ///      "description": "The commit message",
    ///      "type": "string"
    ///    },
    ///    "parents": {
    ///      "description": "Parent commit(s) of this commit",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/CommitParent"
    ///      }
    ///    },
    ///    "sha": {
    ///      "description": "The commit's hash ID",
    ///      "type": "string"
    ///    },
    ///    "tree": {
    ///      "$ref": "#/components/schemas/CommitTree"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct CommitObject {
        pub author: Signature,
        pub committer: Signature,
        ///The commit message
        pub message: ::std::string::String,
        ///Parent commit(s) of this commit
        pub parents: ::std::vec::Vec<CommitParent>,
        ///The commit's hash ID
        pub sha: ::std::string::String,
        pub tree: CommitTree,
    }

    ///Sort order for commit listing
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Sort order for commit listing",
    ///  "type": "string",
    ///  "enum": [
    ///    "desc",
    ///    "asc"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum CommitOrder {
        #[serde(rename = "desc")]
        Desc,
        #[serde(rename = "asc")]
        Asc,
    }

    impl ::std::fmt::Display for CommitOrder {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Desc => f.write_str("desc"),
                Self::Asc => f.write_str("asc"),
            }
        }
    }

    impl ::std::str::FromStr for CommitOrder {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "desc" => Ok(Self::Desc),
                "asc" => Ok(Self::Asc),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for CommitOrder {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for CommitOrder {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for CommitOrder {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`CommitParent`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "sha"
    ///  ],
    ///  "properties": {
    ///    "sha": {
    ///      "description": "The commit's hash ID",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct CommitParent {
        ///The commit's hash ID
        pub sha: ::std::string::String,
    }

    ///`CommitTree`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "sha"
    ///  ],
    ///  "properties": {
    ///    "sha": {
    ///      "description": "The tree's hash ID",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct CommitTree {
        ///The tree's hash ID
        pub sha: ::std::string::String,
    }

    ///Selector for commits based on since/until
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "CommitsSelector",
    ///  "description": "Selector for commits based on since/until",
    ///  "oneOf": [
    ///    {
    ///      "title": "Range",
    ///      "description": "Select commits in a range (like git's A..B)",
    ///      "type": "object",
    ///      "required": [
    ///        "Range"
    ///      ],
    ///      "properties": {
    ///        "Range": {
    ///          "description": "Select commits in a range (like git's A..B)",
    ///          "type": "object",
    ///          "required": [
    ///            "since",
    ///            "until"
    ///          ],
    ///          "properties": {
    ///            "since": {
    ///              "description": "Exclude this commit and its ancestors",
    ///              "type": "string"
    ///            },
    ///            "until": {
    ///              "description": "Start from this commit (inclusive)",
    ///              "type": "string"
    ///            }
    ///          }
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "title": "Since",
    ///      "description": "Select commits since this commit SHA (exclusive)",
    ///      "type": "object",
    ///      "required": [
    ///        "Since"
    ///      ],
    ///      "properties": {
    ///        "Since": {
    ///          "description": "Select commits since this commit SHA
    /// (exclusive)",
    ///          "type": "string",
    ///          "format": "Commit SHA"
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "title": "Until",
    ///      "description": "Select commits until this commit SHA (inclusive)",
    ///      "type": "object",
    ///      "required": [
    ///        "Until"
    ///      ],
    ///      "properties": {
    ///        "Until": {
    ///          "description": "Select commits until this commit SHA
    /// (inclusive)",
    ///          "type": "string",
    ///          "format": "Commit SHA"
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub enum CommitsSelector {
        ///Select commits in a range (like git's A..B)
        Range {
            ///Exclude this commit and its ancestors
            since: ::std::string::String,
            ///Start from this commit (inclusive)
            until: ::std::string::String,
        },
        ///Select commits since this commit SHA (exclusive)
        Since(::std::string::String),
        ///Select commits until this commit SHA (inclusive)
        Until(::std::string::String),
    }

    ///The status of a commit comparison
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The status of a commit comparison",
    ///  "type": "string",
    ///  "enum": [
    ///    "identical",
    ///    "ahead",
    ///    "behind",
    ///    "diverged"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum ComparisonStatus {
        #[serde(rename = "identical")]
        Identical,
        #[serde(rename = "ahead")]
        Ahead,
        #[serde(rename = "behind")]
        Behind,
        #[serde(rename = "diverged")]
        Diverged,
    }

    impl ::std::fmt::Display for ComparisonStatus {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Identical => f.write_str("identical"),
                Self::Ahead => f.write_str("ahead"),
                Self::Behind => f.write_str("behind"),
                Self::Diverged => f.write_str("diverged"),
            }
        }
    }

    impl ::std::str::FromStr for ComparisonStatus {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "identical" => Ok(Self::Identical),
                "ahead" => Ok(Self::Ahead),
                "behind" => Ok(Self::Behind),
                "diverged" => Ok(Self::Diverged),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for ComparisonStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for ComparisonStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for ComparisonStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`ConfigureGithubSyncRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "githubRepoName"
    ///  ],
    ///  "properties": {
    ///    "githubRepoName": {
    ///      "description": "The GitHub repository name in \"owner/repo\"
    /// format",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ConfigureGithubSyncRequest {
        ///The GitHub repository name in "owner/repo" format
        #[serde(rename = "githubRepoName")]
        pub github_repo_name: ::std::string::String,
    }

    ///`CreateBranchRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "sha": {
    ///      "description": "The SHA of the commit this branch should point
    /// to.\nIf not provided, the branch will be created from the default
    /// branch.",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct CreateBranchRequest {
        ///The SHA of the commit this branch should point to.
        ///If not provided, the branch will be created from the default branch.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub sha: ::std::option::Option<::std::string::String>,
    }

    impl ::std::default::Default for CreateBranchRequest {
        fn default() -> Self {
            Self {
                sha: Default::default(),
            }
        }
    }

    ///`CreateBranchResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "name",
    ///    "sha"
    ///  ],
    ///  "properties": {
    ///    "name": {
    ///      "description": "The name of the created branch",
    ///      "type": "string"
    ///    },
    ///    "sha": {
    ///      "description": "The SHA of the commit the branch points to",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct CreateBranchResponse {
        ///The name of the created branch
        pub name: ::std::string::String,
        ///The SHA of the commit the branch points to
        pub sha: ::std::string::String,
    }

    ///`CreateCommitRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "files",
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "author": {
    ///      "$ref": "#/components/schemas/CommitAuthorInfo"
    ///    },
    ///    "branch": {
    ///      "description": "The branch to commit to (e.g., \"main\"). If not
    /// provided, commits to the default branch.\nIf the branch does not exist,
    /// it is created as an orphan branch with this commit as its root.",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "expectedSha": {
    ///      "description": "Optional expected parent SHA for optimistic
    /// concurrency control.\nIf provided, the commit will only be created if
    /// the current branch tip (or parent) matches this SHA.\nThis prevents race
    /// conditions when multiple commits are being created
    /// concurrently.\nReturns a conflict error if the check fails.",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "files": {
    ///      "description": "Files to add, modify, or delete in this commit",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/FileChange"
    ///      }
    ///    },
    ///    "message": {
    ///      "description": "The commit message",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct CreateCommitRequest {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub author: ::std::option::Option<CommitAuthorInfo>,
        ///The branch to commit to (e.g., "main"). If not provided, commits to
        /// the default branch. If the branch does not exist, it is
        /// created as an orphan branch with this commit as its root.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub branch: ::std::option::Option<::std::string::String>,
        ///Optional expected parent SHA for optimistic concurrency control.
        ///If provided, the commit will only be created if the current branch
        /// tip (or parent) matches this SHA. This prevents race
        /// conditions when multiple commits are being created concurrently.
        /// Returns a conflict error if the check fails.
        #[serde(
            rename = "expectedSha",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub expected_sha: ::std::option::Option<::std::string::String>,
        ///Files to add, modify, or delete in this commit
        pub files: ::std::vec::Vec<FileChange>,
        ///The commit message
        pub message: ::std::string::String,
    }

    ///`CreateCommitResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "commit"
    ///  ],
    ///  "properties": {
    ///    "commit": {
    ///      "$ref": "#/components/schemas/CommitObject"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct CreateCommitResponse {
        pub commit: CommitObject,
    }

    ///`CreateDomainMappingRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "deploymentId": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ],
    ///      "format": "uuid"
    ///    },
    ///    "vmId": {
    ///      "$ref": "#/components/schemas/VmId"
    ///    },
    ///    "vmPort": {
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ],
    ///      "format": "int32"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct CreateDomainMappingRequest {
        #[serde(
            rename = "deploymentId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub deployment_id: ::std::option::Option<::uuid::Uuid>,
        #[serde(
            rename = "vmId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub vm_id: ::std::option::Option<VmId>,
        #[serde(
            rename = "vmPort",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub vm_port: ::std::option::Option<i32>,
    }

    impl ::std::default::Default for CreateDomainMappingRequest {
        fn default() -> Self {
            Self {
                deployment_id: Default::default(),
                vm_id: Default::default(),
                vm_port: Default::default(),
            }
        }
    }

    ///`CreateDomainMappingSuccess`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "$ref": "#/components/schemas/FreestyleSandboxDomainMapping"
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(transparent)]
    pub struct CreateDomainMappingSuccess(pub FreestyleSandboxDomainMapping);
    impl ::std::ops::Deref for CreateDomainMappingSuccess {
        type Target = FreestyleSandboxDomainMapping;
        fn deref(&self) -> &FreestyleSandboxDomainMapping {
            &self.0
        }
    }

    impl ::std::convert::From<CreateDomainMappingSuccess> for FreestyleSandboxDomainMapping {
        fn from(value: CreateDomainMappingSuccess) -> Self {
            value.0
        }
    }

    impl ::std::convert::From<FreestyleSandboxDomainMapping> for CreateDomainMappingSuccess {
        fn from(value: FreestyleSandboxDomainMapping) -> Self {
            Self(value)
        }
    }

    ///`CreateOdbCommitRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "message",
    ///    "parents",
    ///    "tree"
    ///  ],
    ///  "properties": {
    ///    "author": {
    ///      "$ref": "#/components/schemas/CommitAuthor"
    ///    },
    ///    "committer": {
    ///      "$ref": "#/components/schemas/CommitAuthor"
    ///    },
    ///    "message": {
    ///      "description": "The commit message",
    ///      "type": "string"
    ///    },
    ///    "parents": {
    ///      "description": "The SHAs of the parent commits (empty array for
    /// initial commit)",
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "tree": {
    ///      "description": "The SHA of the tree for this commit",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct CreateOdbCommitRequest {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub author: ::std::option::Option<CommitAuthor>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub committer: ::std::option::Option<CommitAuthor>,
        ///The commit message
        pub message: ::std::string::String,
        ///The SHAs of the parent commits (empty array for initial commit)
        pub parents: ::std::vec::Vec<::std::string::String>,
        ///The SHA of the tree for this commit
        pub tree: ::std::string::String,
    }

    ///`CreateOdbCommitResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "commit"
    ///  ],
    ///  "properties": {
    ///    "commit": {
    ///      "$ref": "#/components/schemas/CommitObject"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct CreateOdbCommitResponse {
        pub commit: CommitObject,
    }

    ///`CreateRecordParams`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "domain",
    ///    "record"
    ///  ],
    ///  "properties": {
    ///    "domain": {
    ///      "type": "string"
    ///    },
    ///    "record": {
    ///      "$ref": "#/components/schemas/DnsRecordData"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct CreateRecordParams {
        pub domain: ::std::string::String,
        pub record: DnsRecordData,
    }

    ///`CreateRecordResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "record"
    ///  ],
    ///  "properties": {
    ///    "record": {
    ///      "$ref": "#/components/schemas/DnsRecord"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct CreateRecordResponse {
        pub record: DnsRecord,
    }

    ///`CreateRepoImport`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "title": "Files",
    ///      "type": "object",
    ///      "required": [
    ///        "commitMessage",
    ///        "files",
    ///        "type"
    ///      ],
    ///      "properties": {
    ///        "authorEmail": {
    ///          "type": [
    ///            "string",
    ///            "null"
    ///          ]
    ///        },
    ///        "authorName": {
    ///          "type": [
    ///            "string",
    ///            "null"
    ///          ]
    ///        },
    ///        "commitMessage": {
    ///          "type": "string"
    ///        },
    ///        "files": {
    ///          "title": "Files",
    ///          "description": "A map of file names to their contents.",
    ///          "type": "object",
    ///          "additionalProperties": {
    ///            "type": "string"
    ///          }
    ///        },
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "files"
    ///          ]
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "title": "Tar",
    ///      "type": "object",
    ///      "required": [
    ///        "commitMessage",
    ///        "type",
    ///        "url"
    ///      ],
    ///      "properties": {
    ///        "authorEmail": {
    ///          "type": [
    ///            "string",
    ///            "null"
    ///          ]
    ///        },
    ///        "authorName": {
    ///          "type": [
    ///            "string",
    ///            "null"
    ///          ]
    ///        },
    ///        "commitMessage": {
    ///          "type": "string"
    ///        },
    ///        "dir": {
    ///          "type": [
    ///            "string",
    ///            "null"
    ///          ]
    ///        },
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "tar"
    ///          ]
    ///        },
    ///        "url": {
    ///          "type": "string",
    ///          "format": "uri"
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "title": "Zip",
    ///      "type": "object",
    ///      "required": [
    ///        "commitMessage",
    ///        "type",
    ///        "url"
    ///      ],
    ///      "properties": {
    ///        "authorEmail": {
    ///          "type": [
    ///            "string",
    ///            "null"
    ///          ]
    ///        },
    ///        "authorName": {
    ///          "type": [
    ///            "string",
    ///            "null"
    ///          ]
    ///        },
    ///        "commitMessage": {
    ///          "type": "string"
    ///        },
    ///        "dir": {
    ///          "type": [
    ///            "string",
    ///            "null"
    ///          ]
    ///        },
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "zip"
    ///          ]
    ///        },
    ///        "url": {
    ///          "type": "string",
    ///          "format": "uri"
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "title": "Git",
    ///      "type": "object",
    ///      "required": [
    ///        "commitMessage",
    ///        "type",
    ///        "url"
    ///      ],
    ///      "properties": {
    ///        "authorEmail": {
    ///          "type": [
    ///            "string",
    ///            "null"
    ///          ]
    ///        },
    ///        "authorName": {
    ///          "type": [
    ///            "string",
    ///            "null"
    ///          ]
    ///        },
    ///        "branch": {
    ///          "deprecated": true,
    ///          "type": [
    ///            "string",
    ///            "null"
    ///          ]
    ///        },
    ///        "commitMessage": {
    ///          "type": "string"
    ///        },
    ///        "dir": {
    ///          "type": [
    ///            "string",
    ///            "null"
    ///          ]
    ///        },
    ///        "rev": {
    ///          "description": "The revision (branch, tag, or commit hash) to
    /// checkout from the source repo.\n\nThis field supersedes 'branch'.",
    ///          "type": [
    ///            "string",
    ///            "null"
    ///          ]
    ///        },
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "git"
    ///          ]
    ///        },
    ///        "url": {
    ///          "type": "string",
    ///          "format": "uri"
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(tag = "type")]
    pub enum CreateRepoImport {
        ///Files
        #[serde(rename = "files")]
        Files {
            #[serde(
                rename = "authorEmail",
                default,
                skip_serializing_if = "::std::option::Option::is_none"
            )]
            author_email: ::std::option::Option<::std::string::String>,
            #[serde(
                rename = "authorName",
                default,
                skip_serializing_if = "::std::option::Option::is_none"
            )]
            author_name: ::std::option::Option<::std::string::String>,
            #[serde(rename = "commitMessage")]
            commit_message: ::std::string::String,
            ///A map of file names to their contents.
            files: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        },
        ///Tar
        #[serde(rename = "tar")]
        Tar {
            #[serde(
                rename = "authorEmail",
                default,
                skip_serializing_if = "::std::option::Option::is_none"
            )]
            author_email: ::std::option::Option<::std::string::String>,
            #[serde(
                rename = "authorName",
                default,
                skip_serializing_if = "::std::option::Option::is_none"
            )]
            author_name: ::std::option::Option<::std::string::String>,
            #[serde(rename = "commitMessage")]
            commit_message: ::std::string::String,
            #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
            dir: ::std::option::Option<::std::string::String>,
            url: ::std::string::String,
        },
        ///Zip
        #[serde(rename = "zip")]
        Zip {
            #[serde(
                rename = "authorEmail",
                default,
                skip_serializing_if = "::std::option::Option::is_none"
            )]
            author_email: ::std::option::Option<::std::string::String>,
            #[serde(
                rename = "authorName",
                default,
                skip_serializing_if = "::std::option::Option::is_none"
            )]
            author_name: ::std::option::Option<::std::string::String>,
            #[serde(rename = "commitMessage")]
            commit_message: ::std::string::String,
            #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
            dir: ::std::option::Option<::std::string::String>,
            url: ::std::string::String,
        },
        ///Git
        #[serde(rename = "git")]
        Git {
            #[serde(
                rename = "authorEmail",
                default,
                skip_serializing_if = "::std::option::Option::is_none"
            )]
            author_email: ::std::option::Option<::std::string::String>,
            #[serde(
                rename = "authorName",
                default,
                skip_serializing_if = "::std::option::Option::is_none"
            )]
            author_name: ::std::option::Option<::std::string::String>,
            #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
            branch: ::std::option::Option<::std::string::String>,
            #[serde(rename = "commitMessage")]
            commit_message: ::std::string::String,
            #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
            dir: ::std::option::Option<::std::string::String>,
            ///The revision (branch, tag, or commit hash) to checkout from the
            /// source repo.
            ///
            ///This field supersedes 'branch'.
            #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
            rev: ::std::option::Option<::std::string::String>,
            url: ::std::string::String,
        },
    }

    ///`CreateRepoRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "defaultBranch": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "import": {
    ///      "$ref": "#/components/schemas/CreateRepoImport"
    ///    },
    ///    "source": {
    ///      "$ref": "#/components/schemas/CreateRepoSource"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct CreateRepoRequest {
        #[serde(
            rename = "defaultBranch",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub default_branch: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub import: ::std::option::Option<CreateRepoImport>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub source: ::std::option::Option<CreateRepoSource>,
    }

    impl ::std::default::Default for CreateRepoRequest {
        fn default() -> Self {
            Self {
                default_branch: Default::default(),
                import: Default::default(),
                source: Default::default(),
            }
        }
    }

    ///`CreateRepoSource`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "url"
    ///  ],
    ///  "properties": {
    ///    "allBranches": {
    ///      "description": "If true, all branches and tags will be fetched from
    /// the source repository.\nDefaults to false, which only fetches the
    /// default branch (unless `rev` is set).",
    ///      "type": [
    ///        "boolean",
    ///        "null"
    ///      ]
    ///    },
    ///    "branch": {
    ///      "deprecated": true,
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "depth": {
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ],
    ///      "format": "int32"
    ///    },
    ///    "rev": {
    ///      "description": "The revision (branch, tag, or commit hash) to checkout from the source repo.\nThe given revision will be used as the new repository's default branch.\n\nThis field supersedes 'branch'.",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "url": {
    ///      "type": "string",
    ///      "format": "uri"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct CreateRepoSource {
        ///If true, all branches and tags will be fetched from the source
        /// repository. Defaults to false, which only fetches the
        /// default branch (unless `rev` is set).
        #[serde(
            rename = "allBranches",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub all_branches: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub branch: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub depth: ::std::option::Option<i32>,
        ///The revision (branch, tag, or commit hash) to checkout from the
        /// source repo. The given revision will be used as the new
        /// repository's default branch.
        ///
        ///This field supersedes 'branch'.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub rev: ::std::option::Option<::std::string::String>,
        pub url: ::std::string::String,
    }

    ///`CreateRepositoryRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "defaultBranch": {
    ///      "description": "The default branch name for the repository.
    /// Defaults to \"main\" if not specified.",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "import": {
    ///      "$ref": "#/components/schemas/CreateRepoImport"
    ///    },
    ///    "name": {
    ///      "description": "This name is not visible to users, and is only
    /// accessible to you via API and in the\ndashboard. Mostly useful for
    /// observability.",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "public": {
    ///      "default": false,
    ///      "type": "boolean"
    ///    },
    ///    "source": {
    ///      "$ref": "#/components/schemas/CreateRepoSource"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct CreateRepositoryRequest {
        ///The default branch name for the repository. Defaults to "main" if
        /// not specified.
        #[serde(
            rename = "defaultBranch",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub default_branch: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub import: ::std::option::Option<CreateRepoImport>,
        ///This name is not visible to users, and is only accessible to you via
        /// API and in the dashboard. Mostly useful for observability.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
        #[serde(default)]
        pub public: bool,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub source: ::std::option::Option<CreateRepoSource>,
    }

    impl ::std::default::Default for CreateRepositoryRequest {
        fn default() -> Self {
            Self {
                default_branch: Default::default(),
                import: Default::default(),
                name: Default::default(),
                public: Default::default(),
                source: Default::default(),
            }
        }
    }

    ///`CreateRepositoryResponseSuccess`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "repoId"
    ///  ],
    ///  "properties": {
    ///    "repoId": {
    ///      "type": "string",
    ///      "format": "uuid"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct CreateRepositoryResponseSuccess {
        #[serde(rename = "repoId")]
        pub repo_id: ::uuid::Uuid,
    }

    ///`CreateScheduleRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "cron",
    ///    "deploymentId"
    ///  ],
    ///  "properties": {
    ///    "cron": {
    ///      "type": "string"
    ///    },
    ///    "deploymentId": {
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "path": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "payload": {},
    ///    "timezone": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct CreateScheduleRequest {
        pub cron: ::std::string::String,
        #[serde(rename = "deploymentId")]
        pub deployment_id: ::uuid::Uuid,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub path: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub payload: ::std::option::Option<::serde_json::Value>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub timezone: ::std::option::Option<::std::string::String>,
    }

    ///`CreateScheduleResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "schedule"
    ///  ],
    ///  "properties": {
    ///    "schedule": {
    ///      "$ref": "#/components/schemas/Schedule"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct CreateScheduleResponse {
        pub schedule: Schedule,
    }

    ///`CreateServiceResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "error",
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "error": {
    ///      "description": "Error code in SCREAMING_SNAKE_CASE",
    ///      "type": "string"
    ///    },
    ///    "message": {
    ///      "description": "Human-readable error message",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct CreateServiceResponse {
        ///Error code in SCREAMING_SNAKE_CASE
        pub error: ::std::string::String,
        ///Human-readable error message
        pub message: ::std::string::String,
    }

    ///`CreateSnapshotRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "template"
    ///  ],
    ///  "properties": {
    ///    "name": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "persistence": {
    ///      "$ref": "#/components/schemas/SnapshotPersistence"
    ///    },
    ///    "template": {
    ///      "$ref": "#/components/schemas/VmTemplate"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct CreateSnapshotRequest {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub persistence: ::std::option::Option<SnapshotPersistence>,
        pub template: VmTemplate,
    }

    ///`CreateSnapshotResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "snapshotId"
    ///  ],
    ///  "properties": {
    ///    "snapshotId": {
    ///      "$ref": "#/components/schemas/SnapshotId"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct CreateSnapshotResponse {
        #[serde(rename = "snapshotId")]
        pub snapshot_id: SnapshotId,
    }

    ///`CreateVmRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "activityThresholdBytes": {
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ],
    ///      "format": "int64",
    ///      "minimum": 0.0
    ///    },
    ///    "additionalFiles": {
    ///      "type": [
    ///        "object",
    ///        "null"
    ///      ],
    ///      "additionalProperties": {
    ///        "$ref": "#/components/schemas/FreestyleFile"
    ///      }
    ///    },
    ///    "aptDeps": {
    ///      "description": "Optional list of apt packages to install when
    /// setting up the VM.\nThese packages will be installed using `apt-get
    /// install` on VM startup.",
    ///      "examples": [
    ///        [
    ///          "git",
    ///          "curl",
    ///          "vim"
    ///        ]
    ///      ],
    ///      "type": [
    ///        "array",
    ///        "null"
    ///      ],
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "domains": {
    ///      "description": "Optional list of custom domains to map to this VM.
    /// Each domain can optionally\nspecify a vm port. If vm_port is not
    /// specified, defaults to 443.\nDomains must be verified and owned by the
    /// account before they can be mapped.",
    ///      "examples": [
    ///        [
    ///          {
    ///            "domain": "myapp.example.com",
    ///            "vmPort": 3000
    ///          }
    ///        ]
    ///      ],
    ///      "type": [
    ///        "array",
    ///        "null"
    ///      ],
    ///      "items": {
    ///        "$ref": "#/components/schemas/VmDomainConfig"
    ///      }
    ///    },
    ///    "git": {
    ///      "$ref": "#/components/schemas/GitOptions"
    ///    },
    ///    "gitRepos": {
    ///      "deprecated": true,
    ///      "type": [
    ///        "array",
    ///        "null"
    ///      ],
    ///      "items": {
    ///        "$ref": "#/components/schemas/GitRepositorySpec"
    ///      }
    ///    },
    ///    "groups": {
    ///      "type": [
    ///        "array",
    ///        "null"
    ///      ],
    ///      "items": {
    ///        "$ref": "#/components/schemas/LinuxGroupSpec"
    ///      }
    ///    },
    ///    "idleTimeoutSeconds": {
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ],
    ///      "format": "int64"
    ///    },
    ///    "persistence": {
    ///      "$ref": "#/components/schemas/VmPersistence"
    ///    },
    ///    "ports": {
    ///      "description": "Optional list of ports to expose externally. If not
    /// provided, port 3000\nwill be exposed on port 443 by default. Pass an
    /// empty array to disable\nexternal ports. Only ports 8081 and 443 can be
    /// configured externally for\nnow. Any target port is allowed.",
    ///      "examples": [
    ///        [
    ///          {
    ///            "port": 443,
    ///            "targetPort": 3000
    ///          }
    ///        ]
    ///      ],
    ///      "type": [
    ///        "array",
    ///        "null"
    ///      ],
    ///      "items": {
    ///        "$ref": "#/components/schemas/PortMapping"
    ///      }
    ///    },
    ///    "readySignalTimeoutSeconds": {
    ///      "description": "How long to wait for the ready signal before timing
    /// out. Defaults to 120\nseconds if not provided.",
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ],
    ///      "format": "int64"
    ///    },
    ///    "recreate": {
    ///      "description": "If true, the VM can be recreated if it is deleted.
    /// The VM will keep the same ID\nand be recreated with the same
    /// configuration when something tries to start it.",
    ///      "default": false,
    ///      "type": [
    ///        "boolean",
    ///        "null"
    ///      ]
    ///    },
    ///    "snapshotId": {
    ///      "$ref": "#/components/schemas/SnapshotId"
    ///    },
    ///    "systemd": {
    ///      "$ref": "#/components/schemas/SystemdConfig"
    ///    },
    ///    "template": {
    ///      "$ref": "#/components/schemas/VmTemplate"
    ///    },
    ///    "users": {
    ///      "type": [
    ///        "array",
    ///        "null"
    ///      ],
    ///      "items": {
    ///        "$ref": "#/components/schemas/LinuxUserSpec"
    ///      }
    ///    },
    ///    "waitForReadySignal": {
    ///      "description": "Whether the api request should wait for the VM to
    /// be ready before\nreturning. By default, the VM is considered ready when
    /// the serial\nconsole is ready for login.",
    ///      "default": true,
    ///      "examples": [
    ///        true
    ///      ],
    ///      "type": [
    ///        "boolean",
    ///        "null"
    ///      ]
    ///    },
    ///    "workdir": {
    ///      "description": "Optional working directory for the VM. File system
    /// and shell commands\nwill be executed in this directory.",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct CreateVmRequest {
        #[serde(
            rename = "activityThresholdBytes",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub activity_threshold_bytes: ::std::option::Option<i64>,
        #[serde(
            rename = "additionalFiles",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub additional_files: ::std::option::Option<
            ::std::collections::HashMap<::std::string::String, FreestyleFile>,
        >,
        ///Optional list of apt packages to install when setting up the VM.
        ///These packages will be installed using `apt-get install` on VM
        /// startup.
        #[serde(
            rename = "aptDeps",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub apt_deps: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        ///Optional list of custom domains to map to this VM. Each domain can
        /// optionally specify a vm port. If vm_port is not specified,
        /// defaults to 443. Domains must be verified and owned by the
        /// account before they can be mapped.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub domains: ::std::option::Option<::std::vec::Vec<VmDomainConfig>>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub git: ::std::option::Option<GitOptions>,
        #[serde(
            rename = "gitRepos",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub git_repos: ::std::option::Option<::std::vec::Vec<GitRepositorySpec>>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub groups: ::std::option::Option<::std::vec::Vec<LinuxGroupSpec>>,
        #[serde(
            rename = "idleTimeoutSeconds",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub idle_timeout_seconds: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub persistence: ::std::option::Option<VmPersistence>,
        ///Optional list of ports to expose externally. If not provided, port
        /// 3000 will be exposed on port 443 by default. Pass an empty
        /// array to disable external ports. Only ports 8081 and 443 can
        /// be configured externally for now. Any target port is
        /// allowed.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub ports: ::std::option::Option<::std::vec::Vec<PortMapping>>,
        ///How long to wait for the ready signal before timing out. Defaults to
        /// 120 seconds if not provided.
        #[serde(
            rename = "readySignalTimeoutSeconds",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub ready_signal_timeout_seconds: ::std::option::Option<i64>,
        ///If true, the VM can be recreated if it is deleted. The VM will keep
        /// the same ID and be recreated with the same configuration
        /// when something tries to start it.
        #[serde(default = "defaults::create_vm_request_recreate")]
        pub recreate: ::std::option::Option<bool>,
        #[serde(
            rename = "snapshotId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub snapshot_id: ::std::option::Option<SnapshotId>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub systemd: ::std::option::Option<SystemdConfig>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub template: ::std::option::Option<VmTemplate>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub users: ::std::option::Option<::std::vec::Vec<LinuxUserSpec>>,
        ///Whether the api request should wait for the VM to be ready before
        ///returning. By default, the VM is considered ready when the serial
        ///console is ready for login.
        #[serde(
            rename = "waitForReadySignal",
            default = "defaults::create_vm_request_wait_for_ready_signal"
        )]
        pub wait_for_ready_signal: ::std::option::Option<bool>,
        ///Optional working directory for the VM. File system and shell
        /// commands will be executed in this directory.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub workdir: ::std::option::Option<::std::string::String>,
    }

    impl ::std::default::Default for CreateVmRequest {
        fn default() -> Self {
            Self {
                activity_threshold_bytes: Default::default(),
                additional_files: Default::default(),
                apt_deps: Default::default(),
                domains: Default::default(),
                git: Default::default(),
                git_repos: Default::default(),
                groups: Default::default(),
                idle_timeout_seconds: Default::default(),
                persistence: Default::default(),
                ports: Default::default(),
                ready_signal_timeout_seconds: Default::default(),
                recreate: defaults::create_vm_request_recreate(),
                snapshot_id: Default::default(),
                systemd: Default::default(),
                template: Default::default(),
                users: Default::default(),
                wait_for_ready_signal: defaults::create_vm_request_wait_for_ready_signal(),
                workdir: Default::default(),
            }
        }
    }

    ///`CreateVmResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "domains",
    ///    "id"
    ///  ],
    ///  "properties": {
    ///    "consoleUrl": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "domains": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "snapshotId": {
    ///      "$ref": "#/components/schemas/SnapshotId"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct CreateVmResponse {
        #[serde(
            rename = "consoleUrl",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub console_url: ::std::option::Option<::std::string::String>,
        pub domains: ::std::vec::Vec<::std::string::String>,
        pub id: ::std::string::String,
        #[serde(
            rename = "snapshotId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub snapshot_id: ::std::option::Option<SnapshotId>,
    }

    ///`CreatedToken`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "id",
    ///    "token"
    ///  ],
    ///  "properties": {
    ///    "id": {
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "token": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct CreatedToken {
        pub id: ::uuid::Uuid,
        pub token: ::std::string::String,
    }

    ///`CustomBuildOptions`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "command"
    ///  ],
    ///  "properties": {
    ///    "command": {
    ///      "type": "string"
    ///    },
    ///    "envVars": {
    ///      "type": [
    ///        "object",
    ///        "null"
    ///      ],
    ///      "additionalProperties": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "outDir": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct CustomBuildOptions {
        pub command: ::std::string::String,
        #[serde(
            rename = "envVars",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub env_vars: ::std::option::Option<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        >,
        #[serde(
            rename = "outDir",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub out_dir: ::std::option::Option<::std::string::String>,
    }

    ///`DeleteRecordResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "message": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct DeleteRecordResponse {
        pub message: ::std::string::String,
    }

    ///`DeleteRepositorySuccess`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object"
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(transparent)]
    pub struct DeleteRepositorySuccess(
        pub ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    );
    impl ::std::ops::Deref for DeleteRepositorySuccess {
        type Target = ::serde_json::Map<::std::string::String, ::serde_json::Value>;
        fn deref(&self) -> &::serde_json::Map<::std::string::String, ::serde_json::Value> {
            &self.0
        }
    }

    impl ::std::convert::From<DeleteRepositorySuccess>
        for ::serde_json::Map<::std::string::String, ::serde_json::Value>
    {
        fn from(value: DeleteRepositorySuccess) -> Self {
            value.0
        }
    }

    impl ::std::convert::From<::serde_json::Map<::std::string::String, ::serde_json::Value>>
        for DeleteRepositorySuccess
    {
        fn from(value: ::serde_json::Map<::std::string::String, ::serde_json::Value>) -> Self {
            Self(value)
        }
    }

    ///`DeleteScheduleResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "success"
    ///  ],
    ///  "properties": {
    ///    "success": {
    ///      "type": "boolean"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct DeleteScheduleResponse {
        pub success: bool,
    }

    ///`DeleteServiceResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "error",
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "error": {
    ///      "description": "Error code in SCREAMING_SNAKE_CASE",
    ///      "type": "string"
    ///    },
    ///    "message": {
    ///      "description": "Human-readable error message",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct DeleteServiceResponse {
        ///Error code in SCREAMING_SNAKE_CASE
        pub error: ::std::string::String,
        ///Human-readable error message
        pub message: ::std::string::String,
    }

    ///`DeleteSnapshotResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "error",
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "error": {
    ///      "description": "Error code in SCREAMING_SNAKE_CASE",
    ///      "type": "string"
    ///    },
    ///    "message": {
    ///      "description": "Human-readable error message",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct DeleteSnapshotResponse {
        ///Error code in SCREAMING_SNAKE_CASE
        pub error: ::std::string::String,
        ///Human-readable error message
        pub message: ::std::string::String,
    }

    ///`DeleteSnapshotResponses`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "snapshot_id"
    ///  ],
    ///  "properties": {
    ///    "snapshot_id": {
    ///      "$ref": "#/components/schemas/SnapshotId"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct DeleteSnapshotResponses {
        pub snapshot_id: SnapshotId,
    }

    ///`DeleteVmResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "error",
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "error": {
    ///      "description": "Error code in SCREAMING_SNAKE_CASE",
    ///      "type": "string"
    ///    },
    ///    "message": {
    ///      "description": "Human-readable error message",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct DeleteVmResponse {
        ///Error code in SCREAMING_SNAKE_CASE
        pub error: ::std::string::String,
        ///Human-readable error message
        pub message: ::std::string::String,
    }

    ///`DeleteVmResponses`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "id"
    ///  ],
    ///  "properties": {
    ///    "id": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct DeleteVmResponses {
        pub id: ::std::string::String,
    }

    ///`DeploymentBuildOptions`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "type": "boolean"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/CustomBuildOptions"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum DeploymentBuildOptions {
        Boolean(bool),
        CustomBuildOptions(CustomBuildOptions),
    }

    impl ::std::convert::From<bool> for DeploymentBuildOptions {
        fn from(value: bool) -> Self {
            Self::Boolean(value)
        }
    }

    impl ::std::convert::From<CustomBuildOptions> for DeploymentBuildOptions {
        fn from(value: CustomBuildOptions) -> Self {
            Self::CustomBuildOptions(value)
        }
    }

    ///`DeploymentLogEntry`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "accountId",
    ///    "deploymentId",
    ///    "domains",
    ///    "envVars",
    ///    "provisionedAt",
    ///    "state",
    ///    "timeout"
    ///  ],
    ///  "properties": {
    ///    "accountId": {
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "deployedAt": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ],
    ///      "format": "date-time"
    ///    },
    ///    "deploymentId": {
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "domains": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "envVars": {
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "provisionedAt": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "state": {
    ///      "$ref": "#/components/schemas/DeploymentState"
    ///    },
    ///    "timeout": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct DeploymentLogEntry {
        #[serde(rename = "accountId")]
        pub account_id: ::uuid::Uuid,
        #[serde(
            rename = "deployedAt",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub deployed_at: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        #[serde(rename = "deploymentId")]
        pub deployment_id: ::uuid::Uuid,
        pub domains: ::std::vec::Vec<::std::string::String>,
        #[serde(rename = "envVars")]
        pub env_vars: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        #[serde(rename = "provisionedAt")]
        pub provisioned_at: ::chrono::DateTime<::chrono::offset::Utc>,
        pub state: DeploymentState,
        pub timeout: ::std::string::String,
    }

    ///`DeploymentSource`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "title": "Files",
    ///      "type": "object",
    ///      "required": [
    ///        "files",
    ///        "kind"
    ///      ],
    ///      "properties": {
    ///        "files": {
    ///          "type": "object",
    ///          "additionalProperties": {
    ///            "$ref": "#/components/schemas/FreestyleFile"
    ///          }
    ///        },
    ///        "kind": {
    ///          "type": "string",
    ///          "enum": [
    ///            "files"
    ///          ]
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "title": "Tar",
    ///      "type": "object",
    ///      "required": [
    ///        "kind",
    ///        "url"
    ///      ],
    ///      "properties": {
    ///        "kind": {
    ///          "type": "string",
    ///          "enum": [
    ///            "tar"
    ///          ]
    ///        },
    ///        "url": {
    ///          "type": "string"
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "title": "Git",
    ///      "description": "`dir` is the Directory to deploy from. If not
    /// provided, the root of the repository will be used.",
    ///      "type": "object",
    ///      "required": [
    ///        "kind",
    ///        "url"
    ///      ],
    ///      "properties": {
    ///        "branch": {
    ///          "examples": [
    ///            "main"
    ///          ],
    ///          "type": [
    ///            "string",
    ///            "null"
    ///          ]
    ///        },
    ///        "dir": {
    ///          "examples": [
    ///            "dist"
    ///          ],
    ///          "type": [
    ///            "string",
    ///            "null"
    ///          ]
    ///        },
    ///        "kind": {
    ///          "type": "string",
    ///          "enum": [
    ///            "git"
    ///          ]
    ///        },
    ///        "url": {
    ///          "type": "string"
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(tag = "kind")]
    pub enum DeploymentSource {
        ///Files
        #[serde(rename = "files")]
        Files {
            files: ::std::collections::HashMap<::std::string::String, FreestyleFile>,
        },
        ///Tar
        #[serde(rename = "tar")]
        Tar { url: ::std::string::String },
        ///Git
        ///
        ///`dir` is the Directory to deploy from. If not provided, the root of
        /// the repository will be used.
        #[serde(rename = "git")]
        Git {
            #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
            branch: ::std::option::Option<::std::string::String>,
            #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
            dir: ::std::option::Option<::std::string::String>,
            url: ::std::string::String,
        },
    }

    ///`DeploymentState`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "provisioning",
    ///    "deployed",
    ///    "failed"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum DeploymentState {
        #[serde(rename = "provisioning")]
        Provisioning,
        #[serde(rename = "deployed")]
        Deployed,
        #[serde(rename = "failed")]
        Failed,
    }

    impl ::std::fmt::Display for DeploymentState {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Provisioning => f.write_str("provisioning"),
                Self::Deployed => f.write_str("deployed"),
                Self::Failed => f.write_str("failed"),
            }
        }
    }

    impl ::std::str::FromStr for DeploymentState {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "provisioning" => Ok(Self::Provisioning),
                "deployed" => Ok(Self::Deployed),
                "failed" => Ok(Self::Failed),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for DeploymentState {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for DeploymentState {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for DeploymentState {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`DescribeGitPermissionSuccess`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "identity",
    ///    "repo"
    ///  ],
    ///  "properties": {
    ///    "accessLevel": {
    ///      "$ref": "#/components/schemas/AccessLevel"
    ///    },
    ///    "identity": {
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "repo": {
    ///      "type": "string",
    ///      "format": "uuid"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct DescribeGitPermissionSuccess {
        #[serde(
            rename = "accessLevel",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub access_level: ::std::option::Option<AccessLevel>,
        pub identity: ::uuid::Uuid,
        pub repo: ::uuid::Uuid,
    }

    ///A file that was changed in the comparison
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A file that was changed in the comparison",
    ///  "type": "object",
    ///  "required": [
    ///    "additions",
    ///    "changes",
    ///    "deletions",
    ///    "filename",
    ///    "status"
    ///  ],
    ///  "properties": {
    ///    "additions": {
    ///      "description": "Number of lines added",
    ///      "type": "integer",
    ///      "format": "int32",
    ///      "minimum": 0.0
    ///    },
    ///    "changes": {
    ///      "description": "Total number of changes (additions + deletions)",
    ///      "type": "integer",
    ///      "format": "int32",
    ///      "minimum": 0.0
    ///    },
    ///    "deletions": {
    ///      "description": "Number of lines deleted",
    ///      "type": "integer",
    ///      "format": "int32",
    ///      "minimum": 0.0
    ///    },
    ///    "filename": {
    ///      "description": "The file path",
    ///      "type": "string"
    ///    },
    ///    "previous_filename": {
    ///      "description": "Previous filename (for renamed/copied files)",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "sha": {
    ///      "description": "The blob SHA of the file",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "status": {
    ///      "$ref": "#/components/schemas/DiffFileStatus"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct DiffFile {
        ///Number of lines added
        pub additions: i32,
        ///Total number of changes (additions + deletions)
        pub changes: i32,
        ///Number of lines deleted
        pub deletions: i32,
        ///The file path
        pub filename: ::std::string::String,
        ///Previous filename (for renamed/copied files)
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub previous_filename: ::std::option::Option<::std::string::String>,
        ///The blob SHA of the file
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub sha: ::std::option::Option<::std::string::String>,
        pub status: DiffFileStatus,
    }

    ///The status of a file in a diff
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The status of a file in a diff",
    ///  "type": "string",
    ///  "enum": [
    ///    "added",
    ///    "removed",
    ///    "modified",
    ///    "renamed",
    ///    "copied",
    ///    "changed",
    ///    "unchanged"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum DiffFileStatus {
        #[serde(rename = "added")]
        Added,
        #[serde(rename = "removed")]
        Removed,
        #[serde(rename = "modified")]
        Modified,
        #[serde(rename = "renamed")]
        Renamed,
        #[serde(rename = "copied")]
        Copied,
        #[serde(rename = "changed")]
        Changed,
        #[serde(rename = "unchanged")]
        Unchanged,
    }

    impl ::std::fmt::Display for DiffFileStatus {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Added => f.write_str("added"),
                Self::Removed => f.write_str("removed"),
                Self::Modified => f.write_str("modified"),
                Self::Renamed => f.write_str("renamed"),
                Self::Copied => f.write_str("copied"),
                Self::Changed => f.write_str("changed"),
                Self::Unchanged => f.write_str("unchanged"),
            }
        }
    }

    impl ::std::str::FromStr for DiffFileStatus {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "added" => Ok(Self::Added),
                "removed" => Ok(Self::Removed),
                "modified" => Ok(Self::Modified),
                "renamed" => Ok(Self::Renamed),
                "copied" => Ok(Self::Copied),
                "changed" => Ok(Self::Changed),
                "unchanged" => Ok(Self::Unchanged),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for DiffFileStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for DiffFileStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for DiffFileStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`DnsRecord`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "kind",
    ///    "managed",
    ///    "name",
    ///    "ttl",
    ///    "value"
    ///  ],
    ///  "properties": {
    ///    "kind": {
    ///      "$ref": "#/components/schemas/DnsRecordKind"
    ///    },
    ///    "managed": {
    ///      "type": "boolean"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "priority": {
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ],
    ///      "format": "int32",
    ///      "minimum": 0.0
    ///    },
    ///    "ttl": {
    ///      "type": "string"
    ///    },
    ///    "value": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct DnsRecord {
        pub kind: DnsRecordKind,
        pub managed: bool,
        pub name: ::std::string::String,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub priority: ::std::option::Option<i32>,
        pub ttl: ::std::string::String,
        pub value: ::std::string::String,
    }

    ///`DnsRecordData`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "kind",
    ///    "name",
    ///    "value"
    ///  ],
    ///  "properties": {
    ///    "kind": {
    ///      "$ref": "#/components/schemas/DnsRecordKind"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "priority": {
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ],
    ///      "format": "int32",
    ///      "minimum": 0.0
    ///    },
    ///    "ttl": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "value": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct DnsRecordData {
        pub kind: DnsRecordKind,
        pub name: ::std::string::String,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub priority: ::std::option::Option<i32>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub ttl: ::std::option::Option<::std::string::String>,
        pub value: ::std::string::String,
    }

    ///`DnsRecordKind`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "A",
    ///    "AAAA",
    ///    "CNAME",
    ///    "TXT",
    ///    "NS"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum DnsRecordKind {
        A,
        #[serde(rename = "AAAA")]
        Aaaa,
        #[serde(rename = "CNAME")]
        Cname,
        #[serde(rename = "TXT")]
        Txt,
        #[serde(rename = "NS")]
        Ns,
    }

    impl ::std::fmt::Display for DnsRecordKind {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::A => f.write_str("A"),
                Self::Aaaa => f.write_str("AAAA"),
                Self::Cname => f.write_str("CNAME"),
                Self::Txt => f.write_str("TXT"),
                Self::Ns => f.write_str("NS"),
            }
        }
    }

    impl ::std::str::FromStr for DnsRecordKind {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "A" => Ok(Self::A),
                "AAAA" => Ok(Self::Aaaa),
                "CNAME" => Ok(Self::Cname),
                "TXT" => Ok(Self::Txt),
                "NS" => Ok(Self::Ns),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for DnsRecordKind {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for DnsRecordKind {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for DnsRecordKind {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`DomainMappingError`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "FailedToCheckDomainMappingPermissions"
    ///      ],
    ///      "properties": {
    ///        "FailedToCheckDomainMappingPermissions": {
    ///          "type": "object",
    ///          "required": [
    ///            "message"
    ///          ],
    ///          "properties": {
    ///            "message": {
    ///              "type": "string"
    ///            }
    ///          }
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "FailedPermissionsCheck"
    ///      ],
    ///      "properties": {
    ///        "FailedPermissionsCheck": {
    ///          "type": "object",
    ///          "required": [
    ///            "domain"
    ///          ],
    ///          "properties": {
    ///            "domain": {
    ///              "type": "string"
    ///            }
    ///          }
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "FailedRemoveDomainMapping"
    ///      ],
    ///      "properties": {
    ///        "FailedRemoveDomainMapping": {
    ///          "type": "object",
    ///          "required": [
    ///            "message"
    ///          ],
    ///          "properties": {
    ///            "message": {
    ///              "type": "string"
    ///            }
    ///          }
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "FailedToInsertOwnership"
    ///      ],
    ///      "properties": {
    ///        "FailedToInsertOwnership": {
    ///          "type": "object",
    ///          "required": [
    ///            "message"
    ///          ],
    ///          "properties": {
    ///            "message": {
    ///              "type": "string"
    ///            }
    ///          }
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "DomainAlreadyExists"
    ///      ],
    ///      "properties": {
    ///        "DomainAlreadyExists": {
    ///          "type": "object",
    ///          "required": [
    ///            "domain"
    ///          ],
    ///          "properties": {
    ///            "domain": {
    ///              "type": "string"
    ///            }
    ///          }
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "InvalidRequest"
    ///      ],
    ///      "properties": {
    ///        "InvalidRequest": {
    ///          "type": "object",
    ///          "required": [
    ///            "message"
    ///          ],
    ///          "properties": {
    ///            "message": {
    ///              "type": "string"
    ///            }
    ///          }
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "FailedInsertDomainMapping"
    ///      ],
    ///      "properties": {
    ///        "FailedInsertDomainMapping": {
    ///          "type": "object",
    ///          "required": [
    ///            "message"
    ///          ],
    ///          "properties": {
    ///            "message": {
    ///              "type": "string"
    ///            }
    ///          }
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "FailedToProvisionCertificateForMapping"
    ///      ],
    ///      "properties": {
    ///        "FailedToProvisionCertificateForMapping": {
    ///          "type": "object",
    ///          "required": [
    ///            "message"
    ///          ],
    ///          "properties": {
    ///            "message": {
    ///              "type": "string"
    ///            }
    ///          }
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "DeploymentAccessDenied"
    ///      ],
    ///      "properties": {
    ///        "DeploymentAccessDenied": {
    ///          "type": "object",
    ///          "required": [
    ///            "deployment_id"
    ///          ],
    ///          "properties": {
    ///            "deployment_id": {
    ///              "type": "string"
    ///            }
    ///          }
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "VmAccessDeniedForMapping"
    ///      ],
    ///      "properties": {
    ///        "VmAccessDeniedForMapping": {
    ///          "type": "object",
    ///          "required": [
    ///            "vm_id"
    ///          ],
    ///          "properties": {
    ///            "vm_id": {
    ///              "type": "string"
    ///            }
    ///          }
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "DomainOwnershipNotVerified"
    ///      ],
    ///      "properties": {
    ///        "DomainOwnershipNotVerified": {
    ///          "type": "object",
    ///          "required": [
    ///            "domain"
    ///          ],
    ///          "properties": {
    ///            "domain": {
    ///              "type": "string"
    ///            }
    ///          }
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub enum DomainMappingError {
        FailedToCheckDomainMappingPermissions {
            message: ::std::string::String,
        },
        FailedPermissionsCheck {
            domain: ::std::string::String,
        },
        FailedRemoveDomainMapping {
            message: ::std::string::String,
        },
        FailedToInsertOwnership {
            message: ::std::string::String,
        },
        DomainAlreadyExists {
            domain: ::std::string::String,
        },
        InvalidRequest {
            message: ::std::string::String,
        },
        FailedInsertDomainMapping {
            message: ::std::string::String,
        },
        FailedToProvisionCertificateForMapping {
            message: ::std::string::String,
        },
        DeploymentAccessDenied {
            deployment_id: ::std::string::String,
        },
        VmAccessDeniedForMapping {
            vm_id: ::std::string::String,
        },
        DomainOwnershipNotVerified {
            domain: ::std::string::String,
        },
    }

    ///`DomainVerificationRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "accountId",
    ///    "createdAt",
    ///    "domain",
    ///    "id",
    ///    "verificationCode"
    ///  ],
    ///  "properties": {
    ///    "accountId": {
    ///      "examples": [
    ///        "1234-5678-9012-3456"
    ///      ],
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "createdAt": {
    ///      "examples": [
    ///        "1234567890"
    ///      ],
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "domain": {
    ///      "examples": [
    ///        "example.com"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "id": {
    ///      "examples": [
    ///        "1234-5678-9012-3456"
    ///      ],
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "verificationCode": {
    ///      "examples": [
    ///        "freestyle-verification-v1-1234-5678-9012-3456"
    ///      ],
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct DomainVerificationRequest {
        #[serde(rename = "accountId")]
        pub account_id: ::uuid::Uuid,
        #[serde(rename = "createdAt")]
        pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        pub domain: ::std::string::String,
        pub id: ::uuid::Uuid,
        #[serde(rename = "verificationCode")]
        pub verification_code: ::std::string::String,
    }

    ///Dynamic route configuration for paths that should be handled by the
    /// worker
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Dynamic route configuration for paths that should be
    /// handled by the worker",
    ///  "type": "object",
    ///  "required": [
    ///    "source"
    ///  ],
    ///  "properties": {
    ///    "methods": {
    ///      "description": "HTTP methods this route applies to (if None,
    /// applies to all methods)",
    ///      "type": [
    ///        "array",
    ///        "null"
    ///      ],
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "source": {
    ///      "description": "Source pattern (regex or glob pattern like
    /// \"/api/.*\" or \"/api/**\")",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct DynamicConfig {
        ///HTTP methods this route applies to (if None, applies to all methods)
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub methods: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        ///Source pattern (regex or glob pattern like "/api/.*" or "/api/**")
        pub source: ::std::string::String,
    }

    ///`EmptyResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object"
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(transparent)]
    pub struct EmptyResponse(pub ::serde_json::Map<::std::string::String, ::serde_json::Value>);
    impl ::std::ops::Deref for EmptyResponse {
        type Target = ::serde_json::Map<::std::string::String, ::serde_json::Value>;
        fn deref(&self) -> &::serde_json::Map<::std::string::String, ::serde_json::Value> {
            &self.0
        }
    }

    impl ::std::convert::From<EmptyResponse>
        for ::serde_json::Map<::std::string::String, ::serde_json::Value>
    {
        fn from(value: EmptyResponse) -> Self {
            value.0
        }
    }

    impl ::std::convert::From<::serde_json::Map<::std::string::String, ::serde_json::Value>>
        for EmptyResponse
    {
        fn from(value: ::serde_json::Map<::std::string::String, ::serde_json::Value>) -> Self {
            Self(value)
        }
    }

    ///The structure of an error - should rarely be interacted with directly.
    ///
    ///Create your own error types, implement [`ServiceError`] for them, and
    /// they will automatically convert to [`Error`] with `?` or `.into()`.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The structure of an error - should rarely be interacted
    /// with directly.\n\nCreate your own error types, implement
    /// [`ServiceError`] for them, and they will automatically\nconvert to
    /// [`Error`] with `?` or `.into()`.",
    ///  "type": "object",
    ///  "required": [
    ///    "error"
    ///  ],
    ///  "properties": {
    ///    "context": {
    ///      "type": [
    ///        "object",
    ///        "null"
    ///      ],
    ///      "additionalProperties": {}
    ///    },
    ///    "error": {
    ///      "oneOf": [
    ///        {
    ///          "type": "object",
    ///          "required": [
    ///            "FailedToCheckDomainMappingPermissions"
    ///          ],
    ///          "properties": {
    ///            "FailedToCheckDomainMappingPermissions": {
    ///              "type": "object",
    ///              "required": [
    ///                "message"
    ///              ],
    ///              "properties": {
    ///                "message": {
    ///                  "type": "string"
    ///                }
    ///              }
    ///            }
    ///          }
    ///        },
    ///        {
    ///          "type": "object",
    ///          "required": [
    ///            "FailedPermissionsCheck"
    ///          ],
    ///          "properties": {
    ///            "FailedPermissionsCheck": {
    ///              "type": "object",
    ///              "required": [
    ///                "domain"
    ///              ],
    ///              "properties": {
    ///                "domain": {
    ///                  "type": "string"
    ///                }
    ///              }
    ///            }
    ///          }
    ///        },
    ///        {
    ///          "type": "object",
    ///          "required": [
    ///            "FailedRemoveDomainMapping"
    ///          ],
    ///          "properties": {
    ///            "FailedRemoveDomainMapping": {
    ///              "type": "object",
    ///              "required": [
    ///                "message"
    ///              ],
    ///              "properties": {
    ///                "message": {
    ///                  "type": "string"
    ///                }
    ///              }
    ///            }
    ///          }
    ///        },
    ///        {
    ///          "type": "object",
    ///          "required": [
    ///            "FailedToInsertOwnership"
    ///          ],
    ///          "properties": {
    ///            "FailedToInsertOwnership": {
    ///              "type": "object",
    ///              "required": [
    ///                "message"
    ///              ],
    ///              "properties": {
    ///                "message": {
    ///                  "type": "string"
    ///                }
    ///              }
    ///            }
    ///          }
    ///        },
    ///        {
    ///          "type": "object",
    ///          "required": [
    ///            "DomainAlreadyExists"
    ///          ],
    ///          "properties": {
    ///            "DomainAlreadyExists": {
    ///              "type": "object",
    ///              "required": [
    ///                "domain"
    ///              ],
    ///              "properties": {
    ///                "domain": {
    ///                  "type": "string"
    ///                }
    ///              }
    ///            }
    ///          }
    ///        },
    ///        {
    ///          "type": "object",
    ///          "required": [
    ///            "InvalidRequest"
    ///          ],
    ///          "properties": {
    ///            "InvalidRequest": {
    ///              "type": "object",
    ///              "required": [
    ///                "message"
    ///              ],
    ///              "properties": {
    ///                "message": {
    ///                  "type": "string"
    ///                }
    ///              }
    ///            }
    ///          }
    ///        },
    ///        {
    ///          "type": "object",
    ///          "required": [
    ///            "FailedInsertDomainMapping"
    ///          ],
    ///          "properties": {
    ///            "FailedInsertDomainMapping": {
    ///              "type": "object",
    ///              "required": [
    ///                "message"
    ///              ],
    ///              "properties": {
    ///                "message": {
    ///                  "type": "string"
    ///                }
    ///              }
    ///            }
    ///          }
    ///        },
    ///        {
    ///          "type": "object",
    ///          "required": [
    ///            "FailedToProvisionCertificateForMapping"
    ///          ],
    ///          "properties": {
    ///            "FailedToProvisionCertificateForMapping": {
    ///              "type": "object",
    ///              "required": [
    ///                "message"
    ///              ],
    ///              "properties": {
    ///                "message": {
    ///                  "type": "string"
    ///                }
    ///              }
    ///            }
    ///          }
    ///        },
    ///        {
    ///          "type": "object",
    ///          "required": [
    ///            "DeploymentAccessDenied"
    ///          ],
    ///          "properties": {
    ///            "DeploymentAccessDenied": {
    ///              "type": "object",
    ///              "required": [
    ///                "deployment_id"
    ///              ],
    ///              "properties": {
    ///                "deployment_id": {
    ///                  "type": "string"
    ///                }
    ///              }
    ///            }
    ///          }
    ///        },
    ///        {
    ///          "type": "object",
    ///          "required": [
    ///            "VmAccessDeniedForMapping"
    ///          ],
    ///          "properties": {
    ///            "VmAccessDeniedForMapping": {
    ///              "type": "object",
    ///              "required": [
    ///                "vm_id"
    ///              ],
    ///              "properties": {
    ///                "vm_id": {
    ///                  "type": "string"
    ///                }
    ///              }
    ///            }
    ///          }
    ///        },
    ///        {
    ///          "type": "object",
    ///          "required": [
    ///            "DomainOwnershipNotVerified"
    ///          ],
    ///          "properties": {
    ///            "DomainOwnershipNotVerified": {
    ///              "type": "object",
    ///              "required": [
    ///                "domain"
    ///              ],
    ///              "properties": {
    ///                "domain": {
    ///                  "type": "string"
    ///                }
    ///              }
    ///            }
    ///          }
    ///        }
    ///      ]
    ///    },
    ///    "headers": {
    ///      "type": [
    ///        "object",
    ///        "null"
    ///      ],
    ///      "additionalProperties": {
    ///        "type": "string"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ErrorDomainMappingError {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub context:
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        pub error: ErrorDomainMappingErrorError,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub headers: ::std::option::Option<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        >,
    }

    ///`ErrorDomainMappingErrorError`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "FailedToCheckDomainMappingPermissions"
    ///      ],
    ///      "properties": {
    ///        "FailedToCheckDomainMappingPermissions": {
    ///          "type": "object",
    ///          "required": [
    ///            "message"
    ///          ],
    ///          "properties": {
    ///            "message": {
    ///              "type": "string"
    ///            }
    ///          }
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "FailedPermissionsCheck"
    ///      ],
    ///      "properties": {
    ///        "FailedPermissionsCheck": {
    ///          "type": "object",
    ///          "required": [
    ///            "domain"
    ///          ],
    ///          "properties": {
    ///            "domain": {
    ///              "type": "string"
    ///            }
    ///          }
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "FailedRemoveDomainMapping"
    ///      ],
    ///      "properties": {
    ///        "FailedRemoveDomainMapping": {
    ///          "type": "object",
    ///          "required": [
    ///            "message"
    ///          ],
    ///          "properties": {
    ///            "message": {
    ///              "type": "string"
    ///            }
    ///          }
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "FailedToInsertOwnership"
    ///      ],
    ///      "properties": {
    ///        "FailedToInsertOwnership": {
    ///          "type": "object",
    ///          "required": [
    ///            "message"
    ///          ],
    ///          "properties": {
    ///            "message": {
    ///              "type": "string"
    ///            }
    ///          }
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "DomainAlreadyExists"
    ///      ],
    ///      "properties": {
    ///        "DomainAlreadyExists": {
    ///          "type": "object",
    ///          "required": [
    ///            "domain"
    ///          ],
    ///          "properties": {
    ///            "domain": {
    ///              "type": "string"
    ///            }
    ///          }
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "InvalidRequest"
    ///      ],
    ///      "properties": {
    ///        "InvalidRequest": {
    ///          "type": "object",
    ///          "required": [
    ///            "message"
    ///          ],
    ///          "properties": {
    ///            "message": {
    ///              "type": "string"
    ///            }
    ///          }
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "FailedInsertDomainMapping"
    ///      ],
    ///      "properties": {
    ///        "FailedInsertDomainMapping": {
    ///          "type": "object",
    ///          "required": [
    ///            "message"
    ///          ],
    ///          "properties": {
    ///            "message": {
    ///              "type": "string"
    ///            }
    ///          }
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "FailedToProvisionCertificateForMapping"
    ///      ],
    ///      "properties": {
    ///        "FailedToProvisionCertificateForMapping": {
    ///          "type": "object",
    ///          "required": [
    ///            "message"
    ///          ],
    ///          "properties": {
    ///            "message": {
    ///              "type": "string"
    ///            }
    ///          }
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "DeploymentAccessDenied"
    ///      ],
    ///      "properties": {
    ///        "DeploymentAccessDenied": {
    ///          "type": "object",
    ///          "required": [
    ///            "deployment_id"
    ///          ],
    ///          "properties": {
    ///            "deployment_id": {
    ///              "type": "string"
    ///            }
    ///          }
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "VmAccessDeniedForMapping"
    ///      ],
    ///      "properties": {
    ///        "VmAccessDeniedForMapping": {
    ///          "type": "object",
    ///          "required": [
    ///            "vm_id"
    ///          ],
    ///          "properties": {
    ///            "vm_id": {
    ///              "type": "string"
    ///            }
    ///          }
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "DomainOwnershipNotVerified"
    ///      ],
    ///      "properties": {
    ///        "DomainOwnershipNotVerified": {
    ///          "type": "object",
    ///          "required": [
    ///            "domain"
    ///          ],
    ///          "properties": {
    ///            "domain": {
    ///              "type": "string"
    ///            }
    ///          }
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub enum ErrorDomainMappingErrorError {
        FailedToCheckDomainMappingPermissions {
            message: ::std::string::String,
        },
        FailedPermissionsCheck {
            domain: ::std::string::String,
        },
        FailedRemoveDomainMapping {
            message: ::std::string::String,
        },
        FailedToInsertOwnership {
            message: ::std::string::String,
        },
        DomainAlreadyExists {
            domain: ::std::string::String,
        },
        InvalidRequest {
            message: ::std::string::String,
        },
        FailedInsertDomainMapping {
            message: ::std::string::String,
        },
        FailedToProvisionCertificateForMapping {
            message: ::std::string::String,
        },
        DeploymentAccessDenied {
            deployment_id: ::std::string::String,
        },
        VmAccessDeniedForMapping {
            vm_id: ::std::string::String,
        },
        DomainOwnershipNotVerified {
            domain: ::std::string::String,
        },
    }

    ///`ErrorResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "message": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ErrorResponse {
        pub message: ::std::string::String,
    }

    ///`ExecAwaitRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "command"
    ///  ],
    ///  "properties": {
    ///    "command": {
    ///      "type": "string"
    ///    },
    ///    "terminal": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "timeoutMs": {
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ],
    ///      "format": "int64",
    ///      "minimum": 0.0
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ExecAwaitRequest {
        pub command: ::std::string::String,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub terminal: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "timeoutMs",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub timeout_ms: ::std::option::Option<i64>,
    }

    ///`ExecAwaitResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "error",
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "error": {
    ///      "description": "Error code in SCREAMING_SNAKE_CASE",
    ///      "type": "string"
    ///    },
    ///    "message": {
    ///      "description": "Human-readable error message",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ExecAwaitResponse {
        ///Error code in SCREAMING_SNAKE_CASE
        pub error: ::std::string::String,
        ///Human-readable error message
        pub message: ::std::string::String,
    }

    ///`ExecAwaitVmResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "statusCode": {
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ],
    ///      "format": "int32"
    ///    },
    ///    "stderr": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "stdout": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ExecAwaitVmResponse {
        #[serde(
            rename = "statusCode",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub status_code: ::std::option::Option<i32>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub stderr: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub stdout: ::std::option::Option<::std::string::String>,
    }

    impl ::std::default::Default for ExecAwaitVmResponse {
        fn default() -> Self {
            Self {
                status_code: Default::default(),
                stderr: Default::default(),
                stdout: Default::default(),
            }
        }
    }

    ///`ExecuteLogEntry`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "accountId",
    ///    "deployment",
    ///    "envVars",
    ///    "provisionedAt",
    ///    "state"
    ///  ],
    ///  "properties": {
    ///    "accountId": {
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "deployment": {
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "duration": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "envVars": {
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "provisionedAt": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "startedAt": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ],
    ///      "format": "date-time"
    ///    },
    ///    "state": {
    ///      "$ref": "#/components/schemas/ExecuteRunState"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ExecuteLogEntry {
        #[serde(rename = "accountId")]
        pub account_id: ::uuid::Uuid,
        pub deployment: ::uuid::Uuid,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub duration: ::std::option::Option<::std::string::String>,
        #[serde(rename = "envVars")]
        pub env_vars: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        #[serde(rename = "provisionedAt")]
        pub provisioned_at: ::chrono::DateTime<::chrono::offset::Utc>,
        #[serde(
            rename = "startedAt",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub started_at: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        pub state: ExecuteRunState,
    }

    ///`ExecuteRunInfo`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "code",
    ///    "nodeModules"
    ///  ],
    ///  "properties": {
    ///    "code": {
    ///      "type": "string"
    ///    },
    ///    "nodeModules": {
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "string"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ExecuteRunInfo {
        pub code: ::std::string::String,
        #[serde(rename = "nodeModules")]
        pub node_modules: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    }

    ///`ExecuteRunState`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "starting",
    ///    "running",
    ///    "complete"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum ExecuteRunState {
        #[serde(rename = "starting")]
        Starting,
        #[serde(rename = "running")]
        Running,
        #[serde(rename = "complete")]
        Complete,
    }

    impl ::std::fmt::Display for ExecuteRunState {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Starting => f.write_str("starting"),
                Self::Running => f.write_str("running"),
                Self::Complete => f.write_str("complete"),
            }
        }
    }

    impl ::std::str::FromStr for ExecuteRunState {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "starting" => Ok(Self::Starting),
                "running" => Ok(Self::Running),
                "complete" => Ok(Self::Complete),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for ExecuteRunState {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for ExecuteRunState {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for ExecuteRunState {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///Success result from script execution
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Success result from script execution",
    ///  "type": "object",
    ///  "required": [
    ///    "logs",
    ///    "result"
    ///  ],
    ///  "properties": {
    ///    "logs": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/JavascriptLog"
    ///      }
    ///    },
    ///    "result": {}
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ExecuteScriptSuccess {
        pub logs: ::std::vec::Vec<JavascriptLog>,
        pub result: ::serde_json::Value,
    }

    ///`Execution`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "attempts",
    ///    "createdAt",
    ///    "deploymentId",
    ///    "id",
    ///    "maxAttempts",
    ///    "runAt",
    ///    "scheduleId",
    ///    "status",
    ///    "updatedAt"
    ///  ],
    ///  "properties": {
    ///    "attempts": {
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "createdAt": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "deploymentId": {
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "id": {
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "instanceId": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "lastError": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "lockOwner": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "lockedAt": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ],
    ///      "format": "date-time"
    ///    },
    ///    "maxAttempts": {
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "runAt": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "scheduleId": {
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "status": {
    ///      "$ref": "#/components/schemas/ExecutionStatus"
    ///    },
    ///    "updatedAt": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Execution {
        pub attempts: i32,
        #[serde(rename = "createdAt")]
        pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        #[serde(rename = "deploymentId")]
        pub deployment_id: ::uuid::Uuid,
        pub id: ::uuid::Uuid,
        #[serde(
            rename = "instanceId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub instance_id: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "lastError",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub last_error: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "lockOwner",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub lock_owner: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "lockedAt",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub locked_at: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        #[serde(rename = "maxAttempts")]
        pub max_attempts: i32,
        #[serde(rename = "runAt")]
        pub run_at: ::chrono::DateTime<::chrono::offset::Utc>,
        #[serde(rename = "scheduleId")]
        pub schedule_id: ::uuid::Uuid,
        pub status: ExecutionStatus,
        #[serde(rename = "updatedAt")]
        pub updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
    }

    ///`ExecutionStatus`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "queued",
    ///    "running",
    ///    "succeeded",
    ///    "failed",
    ///    "retry"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum ExecutionStatus {
        #[serde(rename = "queued")]
        Queued,
        #[serde(rename = "running")]
        Running,
        #[serde(rename = "succeeded")]
        Succeeded,
        #[serde(rename = "failed")]
        Failed,
        #[serde(rename = "retry")]
        Retry,
    }

    impl ::std::fmt::Display for ExecutionStatus {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Queued => f.write_str("queued"),
                Self::Running => f.write_str("running"),
                Self::Succeeded => f.write_str("succeeded"),
                Self::Failed => f.write_str("failed"),
                Self::Retry => f.write_str("retry"),
            }
        }
    }

    impl ::std::str::FromStr for ExecutionStatus {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "queued" => Ok(Self::Queued),
                "running" => Ok(Self::Running),
                "succeeded" => Ok(Self::Succeeded),
                "failed" => Ok(Self::Failed),
                "retry" => Ok(Self::Retry),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for ExecutionStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for ExecutionStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for ExecutionStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`FileChange`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "path"
    ///  ],
    ///  "properties": {
    ///    "content": {
    ///      "description": "The content of the file. Mutually exclusive with
    /// `deleted`.\nIf encoding is base64, this should be a base64-encoded
    /// string.",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "deleted": {
    ///      "description": "Whether to delete this file. Mutually exclusive
    /// with `content`.",
    ///      "type": [
    ///        "boolean",
    ///        "null"
    ///      ]
    ///    },
    ///    "encoding": {
    ///      "$ref": "#/components/schemas/FileEncoding"
    ///    },
    ///    "path": {
    ///      "description": "The path of the file in the repository (e.g.,
    /// \"src/main.rs\")",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct FileChange {
        ///The content of the file. Mutually exclusive with `deleted`.
        ///If encoding is base64, this should be a base64-encoded string.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub content: ::std::option::Option<::std::string::String>,
        ///Whether to delete this file. Mutually exclusive with `content`.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub deleted: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub encoding: ::std::option::Option<FileEncoding>,
        ///The path of the file in the repository (e.g., "src/main.rs")
        pub path: ::std::string::String,
    }

    ///File content encoding
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "File content encoding",
    ///  "type": "string",
    ///  "enum": [
    ///    "utf8",
    ///    "base64"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum FileEncoding {
        #[serde(rename = "utf8")]
        Utf8,
        #[serde(rename = "base64")]
        Base64,
    }

    impl ::std::fmt::Display for FileEncoding {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Utf8 => f.write_str("utf8"),
                Self::Base64 => f.write_str("base64"),
            }
        }
    }

    impl ::std::str::FromStr for FileEncoding {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "utf8" => Ok(Self::Utf8),
                "base64" => Ok(Self::Base64),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for FileEncoding {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for FileEncoding {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for FileEncoding {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`FileInfo`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "kind",
    ///    "name"
    ///  ],
    ///  "properties": {
    ///    "kind": {
    ///      "$ref": "#/components/schemas/FileKind"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct FileInfo {
        pub kind: FileKind,
        pub name: ::std::string::String,
    }

    ///`FileKind`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "file",
    ///    "directory"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum FileKind {
        #[serde(rename = "file")]
        File,
        #[serde(rename = "directory")]
        Directory,
    }

    impl ::std::fmt::Display for FileKind {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::File => f.write_str("file"),
                Self::Directory => f.write_str("directory"),
            }
        }
    }

    impl ::std::str::FromStr for FileKind {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "file" => Ok(Self::File),
                "directory" => Ok(Self::Directory),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for FileKind {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for FileKind {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for FileKind {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`FileSystemResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "content"
    ///      ],
    ///      "properties": {
    ///        "content": {
    ///          "type": "string"
    ///        },
    ///        "encoding": {
    ///          "$ref": "#/components/schemas/FileEncoding"
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "files"
    ///      ],
    ///      "properties": {
    ///        "files": {
    ///          "type": "array",
    ///          "items": {
    ///            "$ref": "#/components/schemas/FileInfo"
    ///          }
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum FileSystemResponse {
        Variant0 {
            content: ::std::string::String,
            #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
            encoding: ::std::option::Option<FileEncoding>,
        },
        Variant1 {
            files: ::std::vec::Vec<FileInfo>,
        },
    }

    ///`ForkVmRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "count": {
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ],
    ///      "format": "int32",
    ///      "minimum": 0.0
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ForkVmRequest {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub count: ::std::option::Option<i32>,
    }

    impl ::std::default::Default for ForkVmRequest {
        fn default() -> Self {
            Self {
                count: Default::default(),
            }
        }
    }

    ///`ForkVmResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "forks",
    ///    "vmId"
    ///  ],
    ///  "properties": {
    ///    "forks": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/ForkedVmResponse"
    ///      }
    ///    },
    ///    "vmId": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ForkVmResponse {
        pub forks: ::std::vec::Vec<ForkedVmResponse>,
        #[serde(rename = "vmId")]
        pub vm_id: ::std::string::String,
    }

    ///`ForkedVmResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "domains",
    ///    "id"
    ///  ],
    ///  "properties": {
    ///    "consoleUrl": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "domains": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "id": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ForkedVmResponse {
        #[serde(
            rename = "consoleUrl",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub console_url: ::std::option::Option<::std::string::String>,
        pub domains: ::std::vec::Vec<::std::string::String>,
        pub id: ::std::string::String,
    }

    ///`FreestyleCloudstateDeployConfiguration`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "cloudstateDatabaseId": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ],
    ///      "format": "uuid"
    ///    },
    ///    "domains": {
    ///      "description": "ID of the project to deploy, if not provided will
    /// create a new project",
    ///      "type": [
    ///        "array",
    ///        "null"
    ///      ],
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "envVars": {
    ///      "description": "The environment variables that the cloudstate
    /// deploy can access",
    ///      "default": {},
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "string"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct FreestyleCloudstateDeployConfiguration {
        #[serde(
            rename = "cloudstateDatabaseId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub cloudstate_database_id: ::std::option::Option<::uuid::Uuid>,
        ///ID of the project to deploy, if not provided will create a new
        /// project
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub domains: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        ///The environment variables that the cloudstate deploy can access
        #[serde(
            rename = "envVars",
            default,
            skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
        )]
        pub env_vars: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    }

    impl ::std::default::Default for FreestyleCloudstateDeployConfiguration {
        fn default() -> Self {
            Self {
                cloudstate_database_id: Default::default(),
                domains: Default::default(),
                env_vars: Default::default(),
            }
        }
    }

    ///`FreestyleCloudstateDeployErrorResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "message": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct FreestyleCloudstateDeployErrorResponse {
        pub message: ::std::string::String,
    }

    ///`FreestyleCloudstateDeployRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "classes"
    ///  ],
    ///  "properties": {
    ///    "classes": {
    ///      "type": "string"
    ///    },
    ///    "config": {
    ///      "$ref":
    /// "#/components/schemas/FreestyleCloudstateDeployConfiguration"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct FreestyleCloudstateDeployRequest {
        pub classes: ::std::string::String,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub config: ::std::option::Option<FreestyleCloudstateDeployConfiguration>,
    }

    ///`FreestyleCloudstateDeploySuccessResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "cloudstateDatabaseId",
    ///    "deploymentId"
    ///  ],
    ///  "properties": {
    ///    "cloudstateDatabaseId": {
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "deploymentId": {
    ///      "type": "string",
    ///      "format": "uuid"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct FreestyleCloudstateDeploySuccessResponse {
        #[serde(rename = "cloudstateDatabaseId")]
        pub cloudstate_database_id: ::uuid::Uuid,
        #[serde(rename = "deploymentId")]
        pub deployment_id: ::uuid::Uuid,
    }

    ///`FreestyleDeleteDomainVerificationRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "domain",
    ///    "verificationCode"
    ///  ],
    ///  "properties": {
    ///    "domain": {
    ///      "description": "The domain to create a verification code for",
    ///      "examples": [
    ///        "example.com"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "verificationCode": {
    ///      "description": "The verification code",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct FreestyleDeleteDomainVerificationRequest {
        ///The domain to create a verification code for
        pub domain: ::std::string::String,
        ///The verification code
        #[serde(rename = "verificationCode")]
        pub verification_code: ::std::string::String,
    }

    ///`FreestyleDeployWebConfiguration`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "await": {
    ///      "type": [
    ///        "boolean",
    ///        "null"
    ///      ]
    ///    },
    ///    "build": {
    ///      "$ref": "#/components/schemas/DeploymentBuildOptions"
    ///    },
    ///    "cleanUrls": {
    ///      "description": "When true, all HTML files will have their extension
    /// removed. Visiting a path with .html will redirect to extensionless path
    /// (308).\nFor example, a static file named about.html will be served when
    /// visiting /about. Visiting /about.html will redirect to /about.",
    ///      "type": "boolean"
    ///    },
    ///    "domains": {
    ///      "type": [
    ///        "array",
    ///        "null"
    ///      ],
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "dynamic": {
    ///      "description": "Dynamic routes that should be handled by the
    /// worker",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/DynamicConfig"
    ///      }
    ///    },
    ///    "entrypoint": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "envVars": {
    ///      "type": [
    ///        "object",
    ///        "null"
    ///      ],
    ///      "additionalProperties": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "experimental": {
    ///      "$ref":
    /// "#/components/schemas/FreestyleDeployWebExperimentalConfiguration"
    ///    },
    ///    "headers": {
    ///      "description": "Custom headers for matching paths",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/HeaderConfig"
    ///      }
    ///    },
    ///    "networkPermissions": {
    ///      "type": [
    ///        "array",
    ///        "null"
    ///      ],
    ///      "items": {
    ///        "$ref": "#/components/schemas/FreestyleNetworkPermission"
    ///      }
    ///    },
    ///    "nodeModules": {
    ///      "type": [
    ///        "object",
    ///        "null"
    ///      ],
    ///      "additionalProperties": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "prerenderDir": {
    ///      "description": "Directory containing prerendered HTML files (e.g.,
    /// \".next/standalone/.next/server/app\" for Next.js).\nFiles are served
    /// as: ${prerender_dir}/${path}.html or ${prerender_dir}/index.html for
    /// root.",
    ///      "examples": [
    ///        ".next/standalone/.next/server/app"
    ///      ],
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "publicDir": {
    ///      "description": "Directory containing public files to be served at
    /// the root of the domain (e.g., \"public\").\nThese files are served
    /// without any path prefix - a file at public/favicon.ico is served at
    /// /favicon.ico.",
    ///      "examples": [
    ///        "public"
    ///      ],
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "redirects": {
    ///      "description": "Redirects (permanent or temporary)",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/RedirectConfig"
    ///      }
    ///    },
    ///    "rewrites": {
    ///      "description": "Rewrites (internal URL rewrites with capture
    /// groups)",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/RewriteConfig"
    ///      }
    ///    },
    ///    "serverStartCheck": {
    ///      "type": [
    ///        "boolean",
    ///        "null"
    ///      ]
    ///    },
    ///    "staticDir": {
    ///      "description": "Directory containing static files to be served
    /// directly (e.g., \".next/static\", \"_next/static\").\nFiles are served
    /// at the URL path specified by static_path_prefix (defaults to root
    /// \"/\").",
    ///      "examples": [
    ///        ".next/static"
    ///      ],
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "staticOnly": {
    ///      "description": "Set to true for static-only deployments (no dynamic
    /// server/entrypoint required)",
    ///      "type": "boolean"
    ///    },
    ///    "staticPathPrefix": {
    ///      "description": "URL path prefix where static files are served
    /// (e.g., \"/_next/static\"). Defaults to \"/\" (root).\nWhen set, only
    /// requests matching this prefix will be served from static_dir.",
    ///      "examples": [
    ///        "/_next/static"
    ///      ],
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "timeout": {
    ///      "description": "The amount of milliseconds after with no traffic we
    /// kill your server. For rate limiting purposes we count any started server
    /// as active for the 5 seconds after the last request.",
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ],
    ///      "format": "int64",
    ///      "minimum": 0.0
    ///    },
    ///    "trailingSlash": {
    ///      "description": "When false, visiting a path with a trailing slash
    /// will redirect (308) to path without trailing slash.\nWhen true, opposite
    /// behavior occurs. When None (default), no automatic redirects based on
    /// trailing slash.",
    ///      "type": [
    ///        "boolean",
    ///        "null"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct FreestyleDeployWebConfiguration {
        #[serde(
            rename = "await",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub await_: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub build: ::std::option::Option<DeploymentBuildOptions>,
        ///When true, all HTML files will have their extension removed.
        /// Visiting a path with .html will redirect to extensionless path
        /// (308). For example, a static file named about.html will be
        /// served when visiting /about. Visiting /about.html will redirect to
        /// /about.
        #[serde(
            rename = "cleanUrls",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub clean_urls: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub domains: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        ///Dynamic routes that should be handled by the worker
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub dynamic: ::std::vec::Vec<DynamicConfig>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub entrypoint: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "envVars",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub env_vars: ::std::option::Option<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        >,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub experimental: ::std::option::Option<FreestyleDeployWebExperimentalConfiguration>,
        ///Custom headers for matching paths
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub headers: ::std::vec::Vec<HeaderConfig>,
        #[serde(
            rename = "networkPermissions",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub network_permissions: ::std::option::Option<::std::vec::Vec<FreestyleNetworkPermission>>,
        #[serde(
            rename = "nodeModules",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub node_modules: ::std::option::Option<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        >,
        ///Directory containing prerendered HTML files (e.g.,
        /// ".next/standalone/.next/server/app" for Next.js).
        /// Files are served as: ${prerender_dir}/${path}.html or
        /// ${prerender_dir}/index.html for root.
        #[serde(
            rename = "prerenderDir",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub prerender_dir: ::std::option::Option<::std::string::String>,
        ///Directory containing public files to be served at the root of the
        /// domain (e.g., "public"). These files are served without any
        /// path prefix - a file at public/favicon.ico is served at
        /// /favicon.ico.
        #[serde(
            rename = "publicDir",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub public_dir: ::std::option::Option<::std::string::String>,
        ///Redirects (permanent or temporary)
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub redirects: ::std::vec::Vec<RedirectConfig>,
        ///Rewrites (internal URL rewrites with capture groups)
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub rewrites: ::std::vec::Vec<RewriteConfig>,
        #[serde(
            rename = "serverStartCheck",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub server_start_check: ::std::option::Option<bool>,
        ///Directory containing static files to be served directly (e.g.,
        /// ".next/static", "_next/static"). Files are served at the URL
        /// path specified by static_path_prefix (defaults to root "/").
        #[serde(
            rename = "staticDir",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub static_dir: ::std::option::Option<::std::string::String>,
        ///Set to true for static-only deployments (no dynamic
        /// server/entrypoint required)
        #[serde(
            rename = "staticOnly",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub static_only: ::std::option::Option<bool>,
        ///URL path prefix where static files are served (e.g.,
        /// "/_next/static"). Defaults to "/" (root). When set, only
        /// requests matching this prefix will be served from static_dir.
        #[serde(
            rename = "staticPathPrefix",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub static_path_prefix: ::std::option::Option<::std::string::String>,
        ///The amount of milliseconds after with no traffic we kill your
        /// server. For rate limiting purposes we count any started server as
        /// active for the 5 seconds after the last request.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub timeout: ::std::option::Option<i64>,
        ///When false, visiting a path with a trailing slash will redirect
        /// (308) to path without trailing slash. When true, opposite
        /// behavior occurs. When None (default), no automatic redirects based
        /// on trailing slash.
        #[serde(
            rename = "trailingSlash",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub trailing_slash: ::std::option::Option<bool>,
    }

    impl ::std::default::Default for FreestyleDeployWebConfiguration {
        fn default() -> Self {
            Self {
                await_: Default::default(),
                build: Default::default(),
                clean_urls: Default::default(),
                domains: Default::default(),
                dynamic: Default::default(),
                entrypoint: Default::default(),
                env_vars: Default::default(),
                experimental: Default::default(),
                headers: Default::default(),
                network_permissions: Default::default(),
                node_modules: Default::default(),
                prerender_dir: Default::default(),
                public_dir: Default::default(),
                redirects: Default::default(),
                rewrites: Default::default(),
                server_start_check: Default::default(),
                static_dir: Default::default(),
                static_only: Default::default(),
                static_path_prefix: Default::default(),
                timeout: Default::default(),
                trailing_slash: Default::default(),
            }
        }
    }

    ///`FreestyleDeployWebErrorResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "message": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct FreestyleDeployWebErrorResponse {
        pub message: ::std::string::String,
    }

    ///`FreestyleDeployWebExperimentalConfiguration`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "nextjsOptimization": {
    ///      "type": "boolean"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct FreestyleDeployWebExperimentalConfiguration {
        #[serde(
            rename = "nextjsOptimization",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub nextjs_optimization: ::std::option::Option<bool>,
    }

    impl ::std::default::Default for FreestyleDeployWebExperimentalConfiguration {
        fn default() -> Self {
            Self {
                nextjs_optimization: Default::default(),
            }
        }
    }

    ///`FreestyleDeployWebPayload`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "files"
    ///  ],
    ///  "properties": {
    ///    "config": {
    ///      "$ref": "#/components/schemas/FreestyleDeployWebConfiguration"
    ///    },
    ///    "files": {
    ///      "description": "The files to deploy, a map of file paths to file
    /// contents, e.g. { \\\"index.js\\\": {\\\"content\\\": \\\"your main\\\",
    /// \\\"encoding\\\": \\\"utf-8\\\"}, \\\"file2.js\\\": {\\\"content\\\":
    /// \\\"your helper\\\" } }\n\n**Do not include node modules in this bundle,
    /// they will not work**. Instead, includes a package-lock.json, bun.lockb,
    /// pnpm-lock.yaml, or yarn.lock, the node modules for the project will be
    /// installed from that lock file, or use the node_modules field in the
    /// configuration to specify the node modules to install.",
    ///      "examples": [
    ///        {
    ///          "index.js": {
    ///            "content": "import http from 'node:http';\\n// import { resolver } from './file2.js';\\n\\nconsole.log('starting server');\\n\\nconst server = http.createServer(async(req, res) => {\\n  // wait 5 seconds before responding\\n  // await new Promise((resolve) => setTimeout(resolve, 5000));\\n  res.writeHead(200, { 'Content-Type': 'text/plain' });\\n  res.end('Welcome to New York its been waiting for you');\\n});\\n\\nserver.listen(3000, () => {\\n  console.log('Server is running at http://localhost:3000');\\n});"
    ///          }
    ///        }
    ///      ],
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "$ref": "#/components/schemas/FreestyleFile"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct FreestyleDeployWebPayload {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub config: ::std::option::Option<FreestyleDeployWebConfiguration>,
        ///The files to deploy, a map of file paths to file contents, e.g. {
        /// \"index.js\": {\"content\": \"your main\", \"encoding\": \"utf-8\"},
        /// \"file2.js\": {\"content\": \"your helper\" } }
        ///
        ///**Do not include node modules in this bundle, they will not work**.
        /// Instead, includes a package-lock.json, bun.lockb, pnpm-lock.yaml, or
        /// yarn.lock, the node modules for the project will be installed from
        /// that lock file, or use the node_modules field in the configuration
        /// to specify the node modules to install.
        pub files: ::std::collections::HashMap<::std::string::String, FreestyleFile>,
    }

    ///`FreestyleDeployWebPayloadV2`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "config",
    ///    "source"
    ///  ],
    ///  "properties": {
    ///    "config": {
    ///      "$ref": "#/components/schemas/FreestyleDeployWebConfiguration"
    ///    },
    ///    "source": {
    ///      "$ref": "#/components/schemas/DeploymentSource"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct FreestyleDeployWebPayloadV2 {
        pub config: FreestyleDeployWebConfiguration,
        pub source: DeploymentSource,
    }

    ///`FreestyleDeployWebSuccessResponseV2`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "deploymentId",
    ///        "projectId"
    ///      ],
    ///      "properties": {
    ///        "deploymentId": {
    ///          "type": "string",
    ///          "format": "uuid"
    ///        },
    ///        "domains": {
    ///          "type": [
    ///            "array",
    ///            "null"
    ///          ],
    ///          "items": {
    ///            "type": "string"
    ///          }
    ///        },
    ///        "entrypoint": {
    ///          "description": "The entrypoint file for the website. If not
    /// specified we try to automatically detect it.\nFor static-only
    /// deployments (static: true), this will be None.",
    ///          "type": [
    ///            "string",
    ///            "null"
    ///          ]
    ///        },
    ///        "projectId": {
    ///          "deprecated": true,
    ///          "type": "string",
    ///          "format": "uuid"
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "deploymentId"
    ///      ],
    ///      "properties": {
    ///        "deploymentId": {
    ///          "type": "string",
    ///          "format": "uuid"
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum FreestyleDeployWebSuccessResponseV2 {
        Variant0 {
            #[serde(rename = "deploymentId")]
            deployment_id: ::uuid::Uuid,
            #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
            domains: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
            ///The entrypoint file for the website. If not specified we try to
            /// automatically detect it. For static-only deployments
            /// (static: true), this will be None.
            #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
            entrypoint: ::std::option::Option<::std::string::String>,
            #[serde(rename = "projectId")]
            project_id: ::uuid::Uuid,
        },
        Variant1 {
            #[serde(rename = "deploymentId")]
            deployment_id: ::uuid::Uuid,
        },
    }

    ///`FreestyleDomainVerificationRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "domain"
    ///  ],
    ///  "properties": {
    ///    "domain": {
    ///      "description": "The domain to create a verification code for",
    ///      "examples": [
    ///        "example.com"
    ///      ],
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct FreestyleDomainVerificationRequest {
        ///The domain to create a verification code for
        pub domain: ::std::string::String,
    }

    ///`FreestyleExecuteScriptParams`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "script"
    ///  ],
    ///  "properties": {
    ///    "config": {
    ///      "$ref":
    /// "#/components/schemas/FreestyleExecuteScriptParamsConfiguration"
    ///    },
    ///    "script": {
    ///      "description": "The JavaScript or TypeScript script to execute",
    ///      "examples": [
    ///        "export default () => {\n  // get the value of the factorials of
    /// the numbers from 1 to 10 combined\n  const a = [1, 2, 3, 4, 5, 6, 7, 8,
    /// 9, 10];\n\n  function factorial(n) {\n    if (n === 0) {\n      return
    /// 1;\n    }\n    return n * factorial(n - 1);\n  }\n\n  const b =
    /// a.map(factorial);\n\n  return b.reduce((a, b) => a + b);\n};\n"
    ///      ],
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct FreestyleExecuteScriptParams {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub config: ::std::option::Option<FreestyleExecuteScriptParamsConfiguration>,
        ///The JavaScript or TypeScript script to execute
        pub script: ::std::string::String,
    }

    ///`FreestyleExecuteScriptParamsConfiguration`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "customHeaders": {
    ///      "description": "These headers will be added to every fetch request
    /// made through the script",
    ///      "default": {},
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "envVars": {
    ///      "description": "The environment variables to set for the script",
    ///      "default": {},
    ///      "examples": [
    ///        {
    ///          "RESEND_API_KEY": "re_123456789"
    ///        }
    ///      ],
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "networkPermissions": {
    ///      "type": [
    ///        "array",
    ///        "null"
    ///      ],
    ///      "items": {
    ///        "$ref": "#/components/schemas/FreestyleNetworkPermission"
    ///      }
    ///    },
    ///    "nodeModules": {
    ///      "description": "The node modules to install for the script",
    ///      "default": {},
    ///      "examples": [
    ///        {
    ///          "resend": "4.0.1"
    ///        }
    ///      ],
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "peerDependencyResolution": {
    ///      "description": "If false, we'll not resolve peer dependencies for
    /// the packages given, this can speed up execute performance, but will
    /// break packages with peers unless the peers are manually specified.",
    ///      "default": true,
    ///      "type": "boolean"
    ///    },
    ///    "proxy": {
    ///      "description": "Proxy all outgoing requests through this URL",
    ///      "examples": [
    ///        "https://aproxyyouown.com"
    ///      ],
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "tags": {
    ///      "description": "Tags for you to organize your scripts, useful for
    /// tracking what you're running",
    ///      "default": [],
    ///      "examples": [
    ///        [
    ///          "email"
    ///        ]
    ///      ],
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "timeout": {
    ///      "description": "The script timeout",
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ],
    ///      "format": "int32",
    ///      "minimum": 0.0
    ///    },
    ///    "zeroRetention": {
    ///      "description": "If true, Freestyle will not retain the code, any
    /// logs, environment variables, or results from this execution.",
    ///      "examples": [
    ///        false
    ///      ],
    ///      "type": [
    ///        "boolean",
    ///        "null"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct FreestyleExecuteScriptParamsConfiguration {
        ///These headers will be added to every fetch request made through the
        /// script
        #[serde(
            rename = "customHeaders",
            default,
            skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
        )]
        pub custom_headers:
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        ///The environment variables to set for the script
        #[serde(
            rename = "envVars",
            default,
            skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
        )]
        pub env_vars: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        #[serde(
            rename = "networkPermissions",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub network_permissions: ::std::option::Option<::std::vec::Vec<FreestyleNetworkPermission>>,
        ///The node modules to install for the script
        #[serde(
            rename = "nodeModules",
            default,
            skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
        )]
        pub node_modules: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        ///If false, we'll not resolve peer dependencies for the packages
        /// given, this can speed up execute performance, but will break
        /// packages with peers unless the peers are manually specified.
        #[serde(
            rename = "peerDependencyResolution",
            default = "defaults::default_bool::<true>"
        )]
        pub peer_dependency_resolution: bool,
        ///Proxy all outgoing requests through this URL
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub proxy: ::std::option::Option<::std::string::String>,
        ///Tags for you to organize your scripts, useful for tracking what
        /// you're running
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub tags: ::std::vec::Vec<::std::string::String>,
        ///The script timeout
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub timeout: ::std::option::Option<i32>,
        ///If true, Freestyle will not retain the code, any logs, environment
        /// variables, or results from this execution.
        #[serde(
            rename = "zeroRetention",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub zero_retention: ::std::option::Option<bool>,
    }

    impl ::std::default::Default for FreestyleExecuteScriptParamsConfiguration {
        fn default() -> Self {
            Self {
                custom_headers: Default::default(),
                env_vars: Default::default(),
                network_permissions: Default::default(),
                node_modules: Default::default(),
                peer_dependency_resolution: defaults::default_bool::<true>(),
                proxy: Default::default(),
                tags: Default::default(),
                timeout: Default::default(),
                zero_retention: Default::default(),
            }
        }
    }

    ///`FreestyleExecuteScriptResultSuccess`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "logs",
    ///    "result"
    ///  ],
    ///  "properties": {
    ///    "logs": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/FreestyleJavaScriptLog"
    ///      }
    ///    },
    ///    "result": {
    ///      "description": "The return value of the default export of the
    /// script"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct FreestyleExecuteScriptResultSuccess {
        pub logs: ::std::vec::Vec<FreestyleJavaScriptLog>,
        ///The return value of the default export of the script
        pub result: ::serde_json::Value,
    }

    ///`FreestyleFile`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "content"
    ///  ],
    ///  "properties": {
    ///    "content": {
    ///      "description": "The content of the file",
    ///      "type": "string"
    ///    },
    ///    "encoding": {
    ///      "description": "The encoding of the file. Either **utf-8** or
    /// **base64**",
    ///      "type": "string"
    ///    },
    ///    "executable": {
    ///      "description": "Whether the file should be marked executable after
    /// being written",
    ///      "type": "boolean"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct FreestyleFile {
        ///The content of the file
        pub content: ::std::string::String,
        ///The encoding of the file. Either **utf-8** or **base64**
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub encoding: ::std::option::Option<::std::string::String>,
        ///Whether the file should be marked executable after being written
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub executable: ::std::option::Option<bool>,
    }

    ///`FreestyleGetLogsResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "logs"
    ///  ],
    ///  "properties": {
    ///    "logs": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/FreestyleLogResponseObject"
    ///      }
    ///    },
    ///    "nextPageToken": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct FreestyleGetLogsResponse {
        pub logs: ::std::vec::Vec<FreestyleLogResponseObject>,
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }

    ///`FreestyleIdentity`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "id",
    ///    "managed"
    ///  ],
    ///  "properties": {
    ///    "id": {
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "managed": {
    ///      "type": "boolean"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct FreestyleIdentity {
        pub id: ::uuid::Uuid,
        pub managed: bool,
    }

    ///`FreestyleJavaScriptLog`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "message",
    ///    "type"
    ///  ],
    ///  "properties": {
    ///    "message": {
    ///      "description": "The log message",
    ///      "examples": [
    ///        "I'm a log!"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "type": {
    ///      "description": "The log level",
    ///      "examples": [
    ///        "log"
    ///      ],
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct FreestyleJavaScriptLog {
        ///The log message
        pub message: ::std::string::String,
        ///The log level
        #[serde(rename = "type")]
        pub type_: ::std::string::String,
    }

    ///`FreestyleLogResponseObject`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "message",
    ///    "timestamp"
    ///  ],
    ///  "properties": {
    ///    "instanceId": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "level": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "message": {
    ///      "type": "string"
    ///    },
    ///    "origin": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "resourceId": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "resourceType": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "source": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "timestamp": {
    ///      "type": "string"
    ///    },
    ///    "vmService": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct FreestyleLogResponseObject {
        #[serde(
            rename = "instanceId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub instance_id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub level: ::std::option::Option<::std::string::String>,
        pub message: ::std::string::String,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub origin: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "resourceId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub resource_id: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "resourceType",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub resource_type: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub source: ::std::option::Option<::std::string::String>,
        pub timestamp: ::std::string::String,
        #[serde(
            rename = "vmService",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub vm_service: ::std::option::Option<::std::string::String>,
    }

    ///`FreestyleNetworkPermission`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/NetworkPermissionData"
    ///        },
    ///        {
    ///          "type": "object",
    ///          "required": [
    ///            "action"
    ///          ],
    ///          "properties": {
    ///            "action": {
    ///              "type": "string",
    ///              "enum": [
    ///                "allow"
    ///              ]
    ///            }
    ///          }
    ///        }
    ///      ]
    ///    },
    ///    {
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/NetworkPermissionData"
    ///        },
    ///        {
    ///          "type": "object",
    ///          "required": [
    ///            "action"
    ///          ],
    ///          "properties": {
    ///            "action": {
    ///              "type": "string",
    ///              "enum": [
    ///                "deny"
    ///              ]
    ///            }
    ///          }
    ///        }
    ///      ]
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum FreestyleNetworkPermission {
        Variant0 {
            action: FreestyleNetworkPermissionVariant0Action,
            #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
            behavior: ::std::option::Option<Behavior>,
            query: ::std::string::String,
        },
        Variant1 {
            action: FreestyleNetworkPermissionVariant1Action,
            #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
            behavior: ::std::option::Option<Behavior>,
            query: ::std::string::String,
        },
    }

    ///`FreestyleNetworkPermissionVariant0Action`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "allow"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum FreestyleNetworkPermissionVariant0Action {
        #[serde(rename = "allow")]
        Allow,
    }

    impl ::std::fmt::Display for FreestyleNetworkPermissionVariant0Action {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Allow => f.write_str("allow"),
            }
        }
    }

    impl ::std::str::FromStr for FreestyleNetworkPermissionVariant0Action {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "allow" => Ok(Self::Allow),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for FreestyleNetworkPermissionVariant0Action {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for FreestyleNetworkPermissionVariant0Action {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for FreestyleNetworkPermissionVariant0Action {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`FreestyleNetworkPermissionVariant1Action`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "deny"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum FreestyleNetworkPermissionVariant1Action {
        #[serde(rename = "deny")]
        Deny,
    }

    impl ::std::fmt::Display for FreestyleNetworkPermissionVariant1Action {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Deny => f.write_str("deny"),
            }
        }
    }

    impl ::std::str::FromStr for FreestyleNetworkPermissionVariant1Action {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "deny" => Ok(Self::Deny),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for FreestyleNetworkPermissionVariant1Action {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for FreestyleNetworkPermissionVariant1Action {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for FreestyleNetworkPermissionVariant1Action {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`FreestyleSandboxDomainMapping`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "createdAt",
    ///    "domain",
    ///    "id",
    ///    "ownershipId"
    ///  ],
    ///  "properties": {
    ///    "createdAt": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "deploymentId": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ],
    ///      "format": "uuid"
    ///    },
    ///    "domain": {
    ///      "type": "string"
    ///    },
    ///    "id": {
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "ownershipId": {
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "vmId": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "vmPort": {
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ],
    ///      "format": "int32"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct FreestyleSandboxDomainMapping {
        #[serde(rename = "createdAt")]
        pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        #[serde(
            rename = "deploymentId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub deployment_id: ::std::option::Option<::uuid::Uuid>,
        pub domain: ::std::string::String,
        pub id: ::uuid::Uuid,
        #[serde(rename = "ownershipId")]
        pub ownership_id: ::uuid::Uuid,
        #[serde(
            rename = "vmId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub vm_id: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "vmPort",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub vm_port: ::std::option::Option<i32>,
    }

    ///Verify a domain verification request, can either be done for a domain,
    /// or for a specific request
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Verify a domain verification request, can either be
    /// done for a domain, or for a specific request",
    ///  "oneOf": [
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "domain"
    ///      ],
    ///      "properties": {
    ///        "domain": {
    ///          "examples": [
    ///            "example.com"
    ///          ],
    ///          "type": "string"
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "id"
    ///      ],
    ///      "properties": {
    ///        "id": {
    ///          "examples": [
    ///            "1234-5678-9012-3456"
    ///          ],
    ///          "type": "string",
    ///          "format": "uuid"
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub enum FreestyleVerifyDomainRequest {
        #[serde(rename = "domain")]
        Domain(::std::string::String),
        #[serde(rename = "id")]
        Id(::uuid::Uuid),
    }

    impl ::std::convert::From<::uuid::Uuid> for FreestyleVerifyDomainRequest {
        fn from(value: ::uuid::Uuid) -> Self {
            Self::Id(value)
        }
    }

    ///`GetDefaultBranchResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "defaultBranch"
    ///  ],
    ///  "properties": {
    ///    "defaultBranch": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct GetDefaultBranchResponse {
        #[serde(rename = "defaultBranch")]
        pub default_branch: ::std::string::String,
    }

    ///`GetFileResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "error",
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "error": {
    ///      "description": "Error code in SCREAMING_SNAKE_CASE",
    ///      "type": "string"
    ///    },
    ///    "message": {
    ///      "description": "Human-readable error message",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct GetFileResponse {
        ///Error code in SCREAMING_SNAKE_CASE
        pub error: ::std::string::String,
        ///Human-readable error message
        pub message: ::std::string::String,
    }

    ///`GetScheduleResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "schedule"
    ///  ],
    ///  "properties": {
    ///    "schedule": {
    ///      "$ref": "#/components/schemas/Schedule"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct GetScheduleResponse {
        pub schedule: Schedule,
    }

    ///`GetServiceLogsResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "error",
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "error": {
    ///      "description": "Error code in SCREAMING_SNAKE_CASE",
    ///      "type": "string"
    ///    },
    ///    "message": {
    ///      "description": "Human-readable error message",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct GetServiceLogsResponse {
        ///Error code in SCREAMING_SNAKE_CASE
        pub error: ::std::string::String,
        ///Human-readable error message
        pub message: ::std::string::String,
    }

    ///`GetServiceStatusResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "error",
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "error": {
    ///      "description": "Error code in SCREAMING_SNAKE_CASE",
    ///      "type": "string"
    ///    },
    ///    "message": {
    ///      "description": "Human-readable error message",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct GetServiceStatusResponse {
        ///Error code in SCREAMING_SNAKE_CASE
        pub error: ::std::string::String,
        ///Human-readable error message
        pub message: ::std::string::String,
    }

    ///`GetTerminalLogsResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "error",
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "error": {
    ///      "description": "Error code in SCREAMING_SNAKE_CASE",
    ///      "type": "string"
    ///    },
    ///    "message": {
    ///      "description": "Human-readable error message",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct GetTerminalLogsResponse {
        ///Error code in SCREAMING_SNAKE_CASE
        pub error: ::std::string::String,
        ///Human-readable error message
        pub message: ::std::string::String,
    }

    ///`GetTerminalXtermResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "error",
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "error": {
    ///      "description": "Error code in SCREAMING_SNAKE_CASE",
    ///      "type": "string"
    ///    },
    ///    "message": {
    ///      "description": "Human-readable error message",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct GetTerminalXtermResponse {
        ///Error code in SCREAMING_SNAKE_CASE
        pub error: ::std::string::String,
        ///Human-readable error message
        pub message: ::std::string::String,
    }

    ///`GetVisibilityResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "visibility"
    ///  ],
    ///  "properties": {
    ///    "visibility": {
    ///      "$ref": "#/components/schemas/Visibility"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct GetVisibilityResponse {
        pub visibility: Visibility,
    }

    ///`GetVmResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "id",
    ///    "state"
    ///  ],
    ///  "properties": {
    ///    "cpuTimeSeconds": {
    ///      "type": [
    ///        "number",
    ///        "null"
    ///      ],
    ///      "format": "double"
    ///    },
    ///    "id": {
    ///      "$ref": "#/components/schemas/VmId"
    ///    },
    ///    "lastNetworkActivity": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ],
    ///      "format": "date-time"
    ///    },
    ///    "state": {
    ///      "$ref": "#/components/schemas/VMState"
    ///    },
    ///    "vmInstanceId": {
    ///      "$ref": "#/components/schemas/VmInstanceId"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct GetVmResponse {
        #[serde(
            rename = "cpuTimeSeconds",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub cpu_time_seconds: ::std::option::Option<f64>,
        pub id: VmId,
        #[serde(
            rename = "lastNetworkActivity",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub last_network_activity: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        pub state: VmState,
        #[serde(
            rename = "vmInstanceId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub vm_instance_id: ::std::option::Option<VmInstanceId>,
    }

    ///`GitConfig`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "user": {
    ///      "$ref": "#/components/schemas/GitUser"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct GitConfig {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub user: ::std::option::Option<GitUser>,
    }

    impl ::std::default::Default for GitConfig {
        fn default() -> Self {
            Self {
                user: Default::default(),
            }
        }
    }

    ///`GitContents`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "title": "File",
    ///      "type": "object",
    ///      "required": [
    ///        "content",
    ///        "name",
    ///        "path",
    ///        "sha",
    ///        "size",
    ///        "type"
    ///      ],
    ///      "properties": {
    ///        "content": {
    ///          "description": "Base64-encoded content.",
    ///          "type": "string"
    ///        },
    ///        "name": {
    ///          "type": "string"
    ///        },
    ///        "path": {
    ///          "type": "string"
    ///        },
    ///        "sha": {
    ///          "description": "The hash / object ID of the file.",
    ///          "type": "string"
    ///        },
    ///        "size": {
    ///          "type": "integer",
    ///          "format": "int64",
    ///          "minimum": 0.0
    ///        },
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "file"
    ///          ]
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "title": "Directory",
    ///      "type": "object",
    ///      "required": [
    ///        "entries",
    ///        "name",
    ///        "path",
    ///        "sha",
    ///        "type"
    ///      ],
    ///      "properties": {
    ///        "entries": {
    ///          "type": "array",
    ///          "items": {
    ///            "$ref": "#/components/schemas/GitContentsDirEntryItem"
    ///          }
    ///        },
    ///        "name": {
    ///          "type": "string"
    ///        },
    ///        "path": {
    ///          "type": "string"
    ///        },
    ///        "sha": {
    ///          "description": "The hash / object ID of the directory.",
    ///          "type": "string"
    ///        },
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "dir"
    ///          ]
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(tag = "type")]
    pub enum GitContents {
        ///File
        #[serde(rename = "file")]
        File {
            ///Base64-encoded content.
            content: ::std::string::String,
            name: ::std::string::String,
            path: ::std::string::String,
            ///The hash / object ID of the file.
            sha: ::std::string::String,
            size: i64,
        },
        ///Directory
        #[serde(rename = "dir")]
        Dir {
            entries: ::std::vec::Vec<GitContentsDirEntryItem>,
            name: ::std::string::String,
            path: ::std::string::String,
            ///The hash / object ID of the directory.
            sha: ::std::string::String,
        },
    }

    ///`GitContentsDirEntryItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "title": "File Entry",
    ///      "type": "object",
    ///      "required": [
    ///        "name",
    ///        "path",
    ///        "sha",
    ///        "size",
    ///        "type"
    ///      ],
    ///      "properties": {
    ///        "name": {
    ///          "type": "string"
    ///        },
    ///        "path": {
    ///          "type": "string"
    ///        },
    ///        "sha": {
    ///          "description": "The hash / object ID of the file.",
    ///          "type": "string"
    ///        },
    ///        "size": {
    ///          "type": "integer",
    ///          "format": "int64",
    ///          "minimum": 0.0
    ///        },
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "file"
    ///          ]
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "title": "Directory Entry",
    ///      "description": "Directory",
    ///      "type": "object",
    ///      "required": [
    ///        "entries",
    ///        "name",
    ///        "path",
    ///        "sha",
    ///        "type"
    ///      ],
    ///      "properties": {
    ///        "entries": {
    ///          "type": "array",
    ///          "items": {
    ///            "oneOf": [
    ///              {
    ///                "title": "File Entry",
    ///                "type": "object"
    ///              },
    ///              {
    ///                "title": "Dir Entry (recursive)",
    ///                "type": "object"
    ///              }
    ///            ]
    ///          }
    ///        },
    ///        "name": {
    ///          "type": "string"
    ///        },
    ///        "path": {
    ///          "type": "string"
    ///        },
    ///        "sha": {
    ///          "description": "The hash / object ID of the directory.",
    ///          "type": "string"
    ///        },
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "dir"
    ///          ]
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(tag = "type")]
    pub enum GitContentsDirEntryItem {
        ///File Entry
        #[serde(rename = "file")]
        File {
            name: ::std::string::String,
            path: ::std::string::String,
            ///The hash / object ID of the file.
            sha: ::std::string::String,
            size: i64,
        },
        ///Directory Entry
        ///
        ///Directory
        #[serde(rename = "dir")]
        Dir {
            entries: ::std::vec::Vec<GitContentsDirEntryItemEntriesItem>,
            name: ::std::string::String,
            path: ::std::string::String,
            ///The hash / object ID of the directory.
            sha: ::std::string::String,
        },
    }

    ///`GitContentsDirEntryItemEntriesItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "title": "File Entry",
    ///      "type": "object"
    ///    },
    ///    {
    ///      "title": "Dir Entry (recursive)",
    ///      "type": "object"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum GitContentsDirEntryItemEntriesItem {
        FileEntry(::serde_json::Map<::std::string::String, ::serde_json::Value>),
        DirEntryRecursive(::serde_json::Map<::std::string::String, ::serde_json::Value>),
    }

    ///`GitOptions`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "config"
    ///  ],
    ///  "properties": {
    ///    "config": {
    ///      "$ref": "#/components/schemas/GitConfig"
    ///    },
    ///    "repos": {
    ///      "type": [
    ///        "array",
    ///        "null"
    ///      ],
    ///      "items": {
    ///        "$ref": "#/components/schemas/GitRepositorySpec"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct GitOptions {
        pub config: GitConfig,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub repos: ::std::option::Option<::std::vec::Vec<GitRepositorySpec>>,
    }

    ///A reference to a Git object
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A reference to a Git object",
    ///  "type": "object",
    ///  "required": [
    ///    "name",
    ///    "sha"
    ///  ],
    ///  "properties": {
    ///    "name": {
    ///      "description": "The name of the ref (e.g., \"refs/heads/main\" or
    /// \"refs/tags/v1.0.0\")",
    ///      "type": "string"
    ///    },
    ///    "sha": {
    ///      "description": "The SHA-1 hash of the Git object this reference
    /// points to",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct GitReference {
        ///The name of the ref (e.g., "refs/heads/main" or "refs/tags/v1.0.0")
        pub name: ::std::string::String,
        ///The SHA-1 hash of the Git object this reference points to
        pub sha: ::std::string::String,
    }

    ///`GitRepositorySpec`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "path",
    ///    "repo"
    ///  ],
    ///  "properties": {
    ///    "path": {
    ///      "description": "path to place the repo on",
    ///      "type": "string"
    ///    },
    ///    "repo": {
    ///      "description": "url or id of the git repository",
    ///      "type": "string"
    ///    },
    ///    "rev": {
    ///      "description": "optional rev (branch, tag, commit)",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct GitRepositorySpec {
        ///path to place the repo on
        pub path: ::std::string::String,
        ///url or id of the git repository
        pub repo: ::std::string::String,
        ///optional rev (branch, tag, commit)
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub rev: ::std::option::Option<::std::string::String>,
    }

    ///`GitRepositoryTrigger`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "action",
    ///    "createdAt",
    ///    "id",
    ///    "managed",
    ///    "repositoryId",
    ///    "trigger"
    ///  ],
    ///  "properties": {
    ///    "action": {
    ///      "type": "object",
    ///      "required": [
    ///        "action",
    ///        "endpoint"
    ///      ],
    ///      "properties": {
    ///        "action": {
    ///          "type": "string",
    ///          "enum": [
    ///            "webhook"
    ///          ]
    ///        },
    ///        "endpoint": {
    ///          "type": "string"
    ///        }
    ///      }
    ///    },
    ///    "createdAt": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "id": {
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "managed": {
    ///      "type": "boolean"
    ///    },
    ///    "repositoryId": {
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "trigger": {
    ///      "type": "object",
    ///      "required": [
    ///        "event"
    ///      ],
    ///      "properties": {
    ///        "branches": {
    ///          "type": [
    ///            "array",
    ///            "null"
    ///          ],
    ///          "items": {
    ///            "type": "string"
    ///          }
    ///        },
    ///        "event": {
    ///          "type": "string",
    ///          "enum": [
    ///            "push"
    ///          ]
    ///        },
    ///        "globs": {
    ///          "type": [
    ///            "array",
    ///            "null"
    ///          ],
    ///          "items": {
    ///            "type": "string"
    ///          }
    ///        }
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct GitRepositoryTrigger {
        pub action: GitRepositoryTriggerAction,
        #[serde(rename = "createdAt")]
        pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        pub id: ::uuid::Uuid,
        pub managed: bool,
        #[serde(rename = "repositoryId")]
        pub repository_id: ::uuid::Uuid,
        pub trigger: GitRepositoryTriggerTrigger,
    }

    ///`GitRepositoryTriggerAction`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "action",
    ///    "endpoint"
    ///  ],
    ///  "properties": {
    ///    "action": {
    ///      "type": "string",
    ///      "enum": [
    ///        "webhook"
    ///      ]
    ///    },
    ///    "endpoint": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct GitRepositoryTriggerAction {
        pub action: GitRepositoryTriggerActionAction,
        pub endpoint: ::std::string::String,
    }

    ///`GitRepositoryTriggerActionAction`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "webhook"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum GitRepositoryTriggerActionAction {
        #[serde(rename = "webhook")]
        Webhook,
    }

    impl ::std::fmt::Display for GitRepositoryTriggerActionAction {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Webhook => f.write_str("webhook"),
            }
        }
    }

    impl ::std::str::FromStr for GitRepositoryTriggerActionAction {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "webhook" => Ok(Self::Webhook),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for GitRepositoryTriggerActionAction {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for GitRepositoryTriggerActionAction {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for GitRepositoryTriggerActionAction {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`GitRepositoryTriggerTrigger`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "event"
    ///  ],
    ///  "properties": {
    ///    "branches": {
    ///      "type": [
    ///        "array",
    ///        "null"
    ///      ],
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "event": {
    ///      "type": "string",
    ///      "enum": [
    ///        "push"
    ///      ]
    ///    },
    ///    "globs": {
    ///      "type": [
    ///        "array",
    ///        "null"
    ///      ],
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct GitRepositoryTriggerTrigger {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub branches: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        pub event: GitRepositoryTriggerTriggerEvent,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub globs: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }

    ///`GitRepositoryTriggerTriggerEvent`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "push"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum GitRepositoryTriggerTriggerEvent {
        #[serde(rename = "push")]
        Push,
    }

    impl ::std::fmt::Display for GitRepositoryTriggerTriggerEvent {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Push => f.write_str("push"),
            }
        }
    }

    impl ::std::str::FromStr for GitRepositoryTriggerTriggerEvent {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "push" => Ok(Self::Push),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for GitRepositoryTriggerTriggerEvent {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for GitRepositoryTriggerTriggerEvent {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for GitRepositoryTriggerTriggerEvent {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`GitTrigger`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "event"
    ///  ],
    ///  "properties": {
    ///    "branches": {
    ///      "type": [
    ///        "array",
    ///        "null"
    ///      ],
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "event": {
    ///      "type": "string",
    ///      "enum": [
    ///        "push"
    ///      ]
    ///    },
    ///    "globs": {
    ///      "type": [
    ///        "array",
    ///        "null"
    ///      ],
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct GitTrigger {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub branches: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        pub event: GitTriggerEvent,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub globs: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }

    ///`GitTriggerAction`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "action",
    ///    "endpoint"
    ///  ],
    ///  "properties": {
    ///    "action": {
    ///      "type": "string",
    ///      "enum": [
    ///        "webhook"
    ///      ]
    ///    },
    ///    "endpoint": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct GitTriggerAction {
        pub action: GitTriggerActionAction,
        pub endpoint: ::std::string::String,
    }

    ///`GitTriggerActionAction`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "webhook"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum GitTriggerActionAction {
        #[serde(rename = "webhook")]
        Webhook,
    }

    impl ::std::fmt::Display for GitTriggerActionAction {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Webhook => f.write_str("webhook"),
            }
        }
    }

    impl ::std::str::FromStr for GitTriggerActionAction {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "webhook" => Ok(Self::Webhook),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for GitTriggerActionAction {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for GitTriggerActionAction {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for GitTriggerActionAction {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`GitTriggerEvent`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "push"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum GitTriggerEvent {
        #[serde(rename = "push")]
        Push,
    }

    impl ::std::fmt::Display for GitTriggerEvent {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Push => f.write_str("push"),
            }
        }
    }

    impl ::std::str::FromStr for GitTriggerEvent {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "push" => Ok(Self::Push),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for GitTriggerEvent {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for GitTriggerEvent {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for GitTriggerEvent {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`GitUser`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "email": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "name": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "signingkey": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct GitUser {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub email: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub signingkey: ::std::option::Option<::std::string::String>,
    }

    impl ::std::default::Default for GitUser {
        fn default() -> Self {
            Self {
                email: Default::default(),
                name: Default::default(),
                signingkey: Default::default(),
            }
        }
    }

    ///`GithubRepoSyncConfig`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "accountId",
    ///    "createdAt",
    ///    "freestyleRepoId",
    ///    "githubRepoId",
    ///    "githubRepoName",
    ///    "installationId"
    ///  ],
    ///  "properties": {
    ///    "accountId": {
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "createdAt": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "freestyleRepoId": {
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "githubRepoId": {
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "githubRepoName": {
    ///      "type": "string"
    ///    },
    ///    "installationId": {
    ///      "type": "integer",
    ///      "format": "int64"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct GithubRepoSyncConfig {
        #[serde(rename = "accountId")]
        pub account_id: ::uuid::Uuid,
        #[serde(rename = "createdAt")]
        pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        #[serde(rename = "freestyleRepoId")]
        pub freestyle_repo_id: ::uuid::Uuid,
        #[serde(rename = "githubRepoId")]
        pub github_repo_id: i64,
        #[serde(rename = "githubRepoName")]
        pub github_repo_name: ::std::string::String,
        #[serde(rename = "installationId")]
        pub installation_id: i64,
    }

    ///`GithubSyncConfigResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "githubRepoName"
    ///  ],
    ///  "properties": {
    ///    "githubRepoName": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct GithubSyncConfigResponse {
        #[serde(rename = "githubRepoName")]
        pub github_repo_name: ::std::string::String,
    }

    ///`GrantGitPermissionRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "permission"
    ///  ],
    ///  "properties": {
    ///    "permission": {
    ///      "$ref": "#/components/schemas/AccessLevel"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct GrantGitPermissionRequest {
        pub permission: AccessLevel,
    }

    ///`GrantVmPermissionRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "allowedUsers": {
    ///      "description": "List of allowed Linux users. If null, identity can
    /// SSH as any user.\nIf specified, identity can only SSH as users in this
    /// list.",
    ///      "type": [
    ///        "array",
    ///        "null"
    ///      ],
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct GrantVmPermissionRequest {
        ///List of allowed Linux users. If null, identity can SSH as any user.
        ///If specified, identity can only SSH as users in this list.
        #[serde(
            rename = "allowedUsers",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub allowed_users: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }

    impl ::std::default::Default for GrantVmPermissionRequest {
        fn default() -> Self {
            Self {
                allowed_users: Default::default(),
            }
        }
    }

    ///`HandleCreateBranchResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "error",
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "error": {
    ///      "description": "Error code in SCREAMING_SNAKE_CASE",
    ///      "type": "string"
    ///    },
    ///    "message": {
    ///      "description": "Human-readable error message",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct HandleCreateBranchResponse {
        ///Error code in SCREAMING_SNAKE_CASE
        pub error: ::std::string::String,
        ///Human-readable error message
        pub message: ::std::string::String,
    }

    ///`HandleCreateCommitResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "error",
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "error": {
    ///      "description": "Error code in SCREAMING_SNAKE_CASE",
    ///      "type": "string"
    ///    },
    ///    "message": {
    ///      "description": "Human-readable error message",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct HandleCreateCommitResponse {
        ///Error code in SCREAMING_SNAKE_CASE
        pub error: ::std::string::String,
        ///Human-readable error message
        pub message: ::std::string::String,
    }

    ///`HandleCreateDomainVerificationResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "message": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct HandleCreateDomainVerificationResponse {
        pub message: ::std::string::String,
    }

    ///`HandleCreateGitTriggerBody`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "action",
    ///    "trigger"
    ///  ],
    ///  "properties": {
    ///    "action": {
    ///      "type": "object",
    ///      "required": [
    ///        "action",
    ///        "endpoint"
    ///      ],
    ///      "properties": {
    ///        "action": {
    ///          "type": "string",
    ///          "enum": [
    ///            "webhook"
    ///          ]
    ///        },
    ///        "endpoint": {
    ///          "type": "string"
    ///        }
    ///      }
    ///    },
    ///    "trigger": {
    ///      "type": "object",
    ///      "required": [
    ///        "event"
    ///      ],
    ///      "properties": {
    ///        "branches": {
    ///          "type": [
    ///            "array",
    ///            "null"
    ///          ],
    ///          "items": {
    ///            "type": "string"
    ///          }
    ///        },
    ///        "event": {
    ///          "type": "string",
    ///          "enum": [
    ///            "push"
    ///          ]
    ///        },
    ///        "globs": {
    ///          "type": [
    ///            "array",
    ///            "null"
    ///          ],
    ///          "items": {
    ///            "type": "string"
    ///          }
    ///        }
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct HandleCreateGitTriggerBody {
        pub action: HandleCreateGitTriggerBodyAction,
        pub trigger: HandleCreateGitTriggerBodyTrigger,
    }

    ///`HandleCreateGitTriggerBodyAction`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "action",
    ///    "endpoint"
    ///  ],
    ///  "properties": {
    ///    "action": {
    ///      "type": "string",
    ///      "enum": [
    ///        "webhook"
    ///      ]
    ///    },
    ///    "endpoint": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct HandleCreateGitTriggerBodyAction {
        pub action: HandleCreateGitTriggerBodyActionAction,
        pub endpoint: ::std::string::String,
    }

    ///`HandleCreateGitTriggerBodyActionAction`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "webhook"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum HandleCreateGitTriggerBodyActionAction {
        #[serde(rename = "webhook")]
        Webhook,
    }

    impl ::std::fmt::Display for HandleCreateGitTriggerBodyActionAction {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Webhook => f.write_str("webhook"),
            }
        }
    }

    impl ::std::str::FromStr for HandleCreateGitTriggerBodyActionAction {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "webhook" => Ok(Self::Webhook),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for HandleCreateGitTriggerBodyActionAction {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for HandleCreateGitTriggerBodyActionAction {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for HandleCreateGitTriggerBodyActionAction {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`HandleCreateGitTriggerBodyTrigger`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "event"
    ///  ],
    ///  "properties": {
    ///    "branches": {
    ///      "type": [
    ///        "array",
    ///        "null"
    ///      ],
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "event": {
    ///      "type": "string",
    ///      "enum": [
    ///        "push"
    ///      ]
    ///    },
    ///    "globs": {
    ///      "type": [
    ///        "array",
    ///        "null"
    ///      ],
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct HandleCreateGitTriggerBodyTrigger {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub branches: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        pub event: HandleCreateGitTriggerBodyTriggerEvent,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub globs: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }

    ///`HandleCreateGitTriggerBodyTriggerEvent`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "push"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum HandleCreateGitTriggerBodyTriggerEvent {
        #[serde(rename = "push")]
        Push,
    }

    impl ::std::fmt::Display for HandleCreateGitTriggerBodyTriggerEvent {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Push => f.write_str("push"),
            }
        }
    }

    impl ::std::str::FromStr for HandleCreateGitTriggerBodyTriggerEvent {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "push" => Ok(Self::Push),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for HandleCreateGitTriggerBodyTriggerEvent {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for HandleCreateGitTriggerBodyTriggerEvent {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for HandleCreateGitTriggerBodyTriggerEvent {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`HandleCreateGitTriggerResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "triggerId"
    ///  ],
    ///  "properties": {
    ///    "triggerId": {
    ///      "type": "string",
    ///      "format": "uuid"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct HandleCreateGitTriggerResponse {
        #[serde(rename = "triggerId")]
        pub trigger_id: ::uuid::Uuid,
    }

    ///`HandleCreateOdbCommitResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "error",
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "error": {
    ///      "description": "Error code in SCREAMING_SNAKE_CASE",
    ///      "type": "string"
    ///    },
    ///    "message": {
    ///      "description": "Human-readable error message",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct HandleCreateOdbCommitResponse {
        ///Error code in SCREAMING_SNAKE_CASE
        pub error: ::std::string::String,
        ///Human-readable error message
        pub message: ::std::string::String,
    }

    ///`HandleCreateRecordResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "error",
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "error": {
    ///      "description": "Error code in SCREAMING_SNAKE_CASE",
    ///      "type": "string"
    ///    },
    ///    "message": {
    ///      "description": "Human-readable error message",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct HandleCreateRecordResponse {
        ///Error code in SCREAMING_SNAKE_CASE
        pub error: ::std::string::String,
        ///Human-readable error message
        pub message: ::std::string::String,
    }

    ///`HandleCreateRepoBody`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "defaultBranch": {
    ///      "description": "The default branch name for the repository.
    /// Defaults to \"main\" if not specified.",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "import": {
    ///      "$ref": "#/components/schemas/CreateRepoImport"
    ///    },
    ///    "name": {
    ///      "description": "This name is not visible to users, and is only
    /// accessible to you via API and in the\ndashboard. Mostly useful for
    /// observability.",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "public": {
    ///      "default": false,
    ///      "type": "boolean"
    ///    },
    ///    "source": {
    ///      "$ref": "#/components/schemas/CreateRepoSource"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct HandleCreateRepoBody {
        ///The default branch name for the repository. Defaults to "main" if
        /// not specified.
        #[serde(
            rename = "defaultBranch",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub default_branch: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub import: ::std::option::Option<CreateRepoImport>,
        ///This name is not visible to users, and is only accessible to you via
        /// API and in the dashboard. Mostly useful for observability.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
        #[serde(default)]
        pub public: bool,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub source: ::std::option::Option<CreateRepoSource>,
    }

    impl ::std::default::Default for HandleCreateRepoBody {
        fn default() -> Self {
            Self {
                default_branch: Default::default(),
                import: Default::default(),
                name: Default::default(),
                public: Default::default(),
                source: Default::default(),
            }
        }
    }

    ///`HandleCreateRepoResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "error",
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "error": {
    ///      "description": "Error code in SCREAMING_SNAKE_CASE",
    ///      "type": "string"
    ///    },
    ///    "message": {
    ///      "description": "Human-readable error message",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct HandleCreateRepoResponse {
        ///Error code in SCREAMING_SNAKE_CASE
        pub error: ::std::string::String,
        ///Human-readable error message
        pub message: ::std::string::String,
    }

    ///`HandleDeleteDomainMappingResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "error",
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "error": {
    ///      "description": "Error code in SCREAMING_SNAKE_CASE",
    ///      "type": "string"
    ///    },
    ///    "message": {
    ///      "description": "Human-readable error message",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct HandleDeleteDomainMappingResponse {
        ///Error code in SCREAMING_SNAKE_CASE
        pub error: ::std::string::String,
        ///Human-readable error message
        pub message: ::std::string::String,
    }

    ///`HandleDeleteDomainVerificationResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "domain",
    ///    "verificationCode"
    ///  ],
    ///  "properties": {
    ///    "domain": {
    ///      "examples": [
    ///        "example.com"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "verificationCode": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct HandleDeleteDomainVerificationResponse {
        pub domain: ::std::string::String,
        #[serde(rename = "verificationCode")]
        pub verification_code: ::std::string::String,
    }

    ///`HandleDeleteGitTriggerResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "message": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct HandleDeleteGitTriggerResponse {
        pub message: ::std::string::String,
    }

    ///`HandleDeleteRecordResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "error",
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "error": {
    ///      "description": "Error code in SCREAMING_SNAKE_CASE",
    ///      "type": "string"
    ///    },
    ///    "message": {
    ///      "description": "Human-readable error message",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct HandleDeleteRecordResponse {
        ///Error code in SCREAMING_SNAKE_CASE
        pub error: ::std::string::String,
        ///Human-readable error message
        pub message: ::std::string::String,
    }

    ///`HandleDeleteRepoResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "error",
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "error": {
    ///      "description": "Error code in SCREAMING_SNAKE_CASE",
    ///      "type": "string"
    ///    },
    ///    "message": {
    ///      "description": "Human-readable error message",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct HandleDeleteRepoResponse {
        ///Error code in SCREAMING_SNAKE_CASE
        pub error: ::std::string::String,
        ///Human-readable error message
        pub message: ::std::string::String,
    }

    ///`HandleDeployWebV2Response`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "error",
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "error": {
    ///      "description": "Error code in SCREAMING_SNAKE_CASE",
    ///      "type": "string"
    ///    },
    ///    "message": {
    ///      "description": "Human-readable error message",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct HandleDeployWebV2Response {
        ///Error code in SCREAMING_SNAKE_CASE
        pub error: ::std::string::String,
        ///Human-readable error message
        pub message: ::std::string::String,
    }

    ///`HandleExecuteScriptResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "logs",
    ///    "result"
    ///  ],
    ///  "properties": {
    ///    "logs": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/FreestyleJavaScriptLog"
    ///      }
    ///    },
    ///    "result": {
    ///      "description": "The return value of the default export of the
    /// script"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct HandleExecuteScriptResponse {
        pub logs: ::std::vec::Vec<FreestyleJavaScriptLog>,
        ///The return value of the default export of the script
        pub result: ::serde_json::Value,
    }

    ///`HandleExecuteScriptV2Response`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "error",
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "error": {
    ///      "description": "Error code in SCREAMING_SNAKE_CASE",
    ///      "type": "string"
    ///    },
    ///    "message": {
    ///      "description": "Human-readable error message",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct HandleExecuteScriptV2Response {
        ///Error code in SCREAMING_SNAKE_CASE
        pub error: ::std::string::String,
        ///Human-readable error message
        pub message: ::std::string::String,
    }

    ///`HandleExecuteScriptV3Response`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "error",
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "error": {
    ///      "description": "Error code in SCREAMING_SNAKE_CASE",
    ///      "type": "string"
    ///    },
    ///    "message": {
    ///      "description": "Human-readable error message",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct HandleExecuteScriptV3Response {
        ///Error code in SCREAMING_SNAKE_CASE
        pub error: ::std::string::String,
        ///Human-readable error message
        pub message: ::std::string::String,
    }

    ///`HandleGetDefaultBranchResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "error",
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "error": {
    ///      "description": "Error code in SCREAMING_SNAKE_CASE",
    ///      "type": "string"
    ///    },
    ///    "message": {
    ///      "description": "Human-readable error message",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct HandleGetDefaultBranchResponse {
        ///Error code in SCREAMING_SNAKE_CASE
        pub error: ::std::string::String,
        ///Human-readable error message
        pub message: ::std::string::String,
    }

    ///`HandleGetExecuteRunResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "metadata"
    ///  ],
    ///  "properties": {
    ///    "code": {
    ///      "$ref": "#/components/schemas/ExecuteRunInfo"
    ///    },
    ///    "metadata": {
    ///      "$ref": "#/components/schemas/ExecuteLogEntry"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct HandleGetExecuteRunResponse {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub code: ::std::option::Option<ExecuteRunInfo>,
        pub metadata: ExecuteLogEntry,
    }

    ///`HandleGetRefBranchResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "error",
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "error": {
    ///      "description": "Error code in SCREAMING_SNAKE_CASE",
    ///      "type": "string"
    ///    },
    ///    "message": {
    ///      "description": "Human-readable error message",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct HandleGetRefBranchResponse {
        ///Error code in SCREAMING_SNAKE_CASE
        pub error: ::std::string::String,
        ///Human-readable error message
        pub message: ::std::string::String,
    }

    ///Tag object
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Tag object",
    ///  "type": "object",
    ///  "required": [
    ///    "name",
    ///    "sha",
    ///    "target"
    ///  ],
    ///  "properties": {
    ///    "message": {
    ///      "description": "The tag message",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "name": {
    ///      "description": "The tag name",
    ///      "type": "string"
    ///    },
    ///    "sha": {
    ///      "description": "The tag's hash ID",
    ///      "type": "string"
    ///    },
    ///    "tagger": {
    ///      "$ref": "#/components/schemas/Signature"
    ///    },
    ///    "target": {
    ///      "$ref": "#/components/schemas/TagTarget"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct HandleGetRefTagResponse {
        ///The tag message
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub message: ::std::option::Option<::std::string::String>,
        ///The tag name
        pub name: ::std::string::String,
        ///The tag's hash ID
        pub sha: ::std::string::String,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub tagger: ::std::option::Option<Signature>,
        pub target: TagTarget,
    }

    ///`HandleGetVisibilityResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "error",
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "error": {
    ///      "description": "Error code in SCREAMING_SNAKE_CASE",
    ///      "type": "string"
    ///    },
    ///    "message": {
    ///      "description": "Human-readable error message",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct HandleGetVisibilityResponse {
        ///Error code in SCREAMING_SNAKE_CASE
        pub error: ::std::string::String,
        ///Human-readable error message
        pub message: ::std::string::String,
    }

    ///`HandleListCommitsResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "error",
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "error": {
    ///      "description": "Error code in SCREAMING_SNAKE_CASE",
    ///      "type": "string"
    ///    },
    ///    "message": {
    ///      "description": "Human-readable error message",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct HandleListCommitsResponse {
        ///Error code in SCREAMING_SNAKE_CASE
        pub error: ::std::string::String,
        ///Human-readable error message
        pub message: ::std::string::String,
    }

    ///`HandleListDomainVerificationRequestsResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "message": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct HandleListDomainVerificationRequestsResponse {
        pub message: ::std::string::String,
    }

    ///`HandleListDomainVerificationRequestsResponseItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "createdAt",
    ///    "domain",
    ///    "verificationCode"
    ///  ],
    ///  "properties": {
    ///    "createdAt": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "domain": {
    ///      "type": "string"
    ///    },
    ///    "verificationCode": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct HandleListDomainVerificationRequestsResponseItem {
        #[serde(rename = "createdAt")]
        pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        pub domain: ::std::string::String,
        #[serde(rename = "verificationCode")]
        pub verification_code: ::std::string::String,
    }

    ///`HandleListDomainsResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "message": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct HandleListDomainsResponse {
        pub message: ::std::string::String,
    }

    ///`HandleListDomainsResponseItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "accountId",
    ///    "createdAt",
    ///    "deployToDomain",
    ///    "deployToSubdomains",
    ///    "domain",
    ///    "id",
    ///    "implicitlyOwned",
    ///    "manageDns",
    ///    "verifiedDns"
    ///  ],
    ///  "properties": {
    ///    "accountId": {
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "createdAt": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "deployToDomain": {
    ///      "type": "boolean"
    ///    },
    ///    "deployToSubdomains": {
    ///      "type": "boolean"
    ///    },
    ///    "domain": {
    ///      "type": "string"
    ///    },
    ///    "id": {
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "implicitlyOwned": {
    ///      "type": "boolean"
    ///    },
    ///    "manageDns": {
    ///      "type": "boolean"
    ///    },
    ///    "verifiedDns": {
    ///      "type": "boolean"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct HandleListDomainsResponseItem {
        #[serde(rename = "accountId")]
        pub account_id: ::uuid::Uuid,
        #[serde(rename = "createdAt")]
        pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        #[serde(rename = "deployToDomain")]
        pub deploy_to_domain: bool,
        #[serde(rename = "deployToSubdomains")]
        pub deploy_to_subdomains: bool,
        pub domain: ::std::string::String,
        pub id: ::uuid::Uuid,
        #[serde(rename = "implicitlyOwned")]
        pub implicitly_owned: bool,
        #[serde(rename = "manageDns")]
        pub manage_dns: bool,
        #[serde(rename = "verifiedDns")]
        pub verified_dns: bool,
    }

    ///`HandleListExecuteRunsResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "entries",
    ///    "offset",
    ///    "total"
    ///  ],
    ///  "properties": {
    ///    "entries": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/ExecuteLogEntry"
    ///      }
    ///    },
    ///    "offset": {
    ///      "type": "integer",
    ///      "format": "int64",
    ///      "minimum": 0.0
    ///    },
    ///    "total": {
    ///      "type": "integer",
    ///      "format": "int64",
    ///      "minimum": 0.0
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct HandleListExecuteRunsResponse {
        pub entries: ::std::vec::Vec<ExecuteLogEntry>,
        pub offset: i64,
        pub total: i64,
    }

    ///`HandleListGitTriggersResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "triggers"
    ///  ],
    ///  "properties": {
    ///    "triggers": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/GitRepositoryTrigger"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct HandleListGitTriggersResponse {
        pub triggers: ::std::vec::Vec<GitRepositoryTrigger>,
    }

    ///`HandleListRecordsResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "message": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct HandleListRecordsResponse {
        pub message: ::std::string::String,
    }

    ///`HandleListRepositoriesResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "error",
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "error": {
    ///      "description": "Error code in SCREAMING_SNAKE_CASE",
    ///      "type": "string"
    ///    },
    ///    "message": {
    ///      "description": "Human-readable error message",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct HandleListRepositoriesResponse {
        ///Error code in SCREAMING_SNAKE_CASE
        pub error: ::std::string::String,
        ///Human-readable error message
        pub message: ::std::string::String,
    }

    ///`HandleListWebDeploysResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "error",
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "error": {
    ///      "description": "Error code in SCREAMING_SNAKE_CASE",
    ///      "type": "string"
    ///    },
    ///    "message": {
    ///      "description": "Human-readable error message",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct HandleListWebDeploysResponse {
        ///Error code in SCREAMING_SNAKE_CASE
        pub error: ::std::string::String,
        ///Human-readable error message
        pub message: ::std::string::String,
    }

    ///`HandleSetDefaultBranchResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "error",
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "error": {
    ///      "description": "Error code in SCREAMING_SNAKE_CASE",
    ///      "type": "string"
    ///    },
    ///    "message": {
    ///      "description": "Human-readable error message",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct HandleSetDefaultBranchResponse {
        ///Error code in SCREAMING_SNAKE_CASE
        pub error: ::std::string::String,
        ///Human-readable error message
        pub message: ::std::string::String,
    }

    ///`HandleSetVisibilityResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "error",
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "error": {
    ///      "description": "Error code in SCREAMING_SNAKE_CASE",
    ///      "type": "string"
    ///    },
    ///    "message": {
    ///      "description": "Human-readable error message",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct HandleSetVisibilityResponse {
        ///Error code in SCREAMING_SNAKE_CASE
        pub error: ::std::string::String,
        ///Human-readable error message
        pub message: ::std::string::String,
    }

    ///`HandleVerifyDomainResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "error",
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "error": {
    ///      "description": "Error code in SCREAMING_SNAKE_CASE",
    ///      "type": "string"
    ///    },
    ///    "message": {
    ///      "description": "Human-readable error message",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct HandleVerifyDomainResponse {
        ///Error code in SCREAMING_SNAKE_CASE
        pub error: ::std::string::String,
        ///Human-readable error message
        pub message: ::std::string::String,
    }

    ///`HandleVerifyWildcardResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "domain"
    ///  ],
    ///  "properties": {
    ///    "domain": {
    ///      "examples": [
    ///        "example.com"
    ///      ],
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct HandleVerifyWildcardResponse {
        pub domain: ::std::string::String,
    }

    ///Header configuration for setting custom headers on matching paths
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Header configuration for setting custom headers on
    /// matching paths",
    ///  "type": "object",
    ///  "required": [
    ///    "headers",
    ///    "source"
    ///  ],
    ///  "properties": {
    ///    "headers": {
    ///      "description": "Headers to set (array of key-value pairs)",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/HeaderKeyValue"
    ///      }
    ///    },
    ///    "source": {
    ///      "description": "Source pattern (regex pattern)",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct HeaderConfig {
        ///Headers to set (array of key-value pairs)
        pub headers: ::std::vec::Vec<HeaderKeyValue>,
        ///Source pattern (regex pattern)
        pub source: ::std::string::String,
    }

    ///Key-value pair for headers
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Key-value pair for headers",
    ///  "type": "object",
    ///  "required": [
    ///    "key",
    ///    "value"
    ///  ],
    ///  "properties": {
    ///    "key": {
    ///      "description": "Header name",
    ///      "type": "string"
    ///    },
    ///    "value": {
    ///      "description": "Header value",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct HeaderKeyValue {
        ///Header name
        pub key: ::std::string::String,
        ///Header value
        pub value: ::std::string::String,
    }

    ///`InternalServerError`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string"
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    #[serde(transparent)]
    pub struct InternalServerError(pub ::std::string::String);
    impl ::std::ops::Deref for InternalServerError {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<InternalServerError> for ::std::string::String {
        fn from(value: InternalServerError) -> Self {
            value.0
        }
    }

    impl ::std::convert::From<::std::string::String> for InternalServerError {
        fn from(value: ::std::string::String) -> Self {
            Self(value)
        }
    }

    impl ::std::str::FromStr for InternalServerError {
        type Err = ::std::convert::Infallible;
        fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::fmt::Display for InternalServerError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            self.0.fmt(f)
        }
    }

    ///`JavascriptLog`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "message",
    ///        "type"
    ///      ],
    ///      "properties": {
    ///        "callstack": {
    ///          "type": [
    ///            "string",
    ///            "null"
    ///          ]
    ///        },
    ///        "message": {
    ///          "type": "string"
    ///        },
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "log"
    ///          ]
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "message",
    ///        "type"
    ///      ],
    ///      "properties": {
    ///        "callstack": {
    ///          "type": [
    ///            "string",
    ///            "null"
    ///          ]
    ///        },
    ///        "message": {
    ///          "type": "string"
    ///        },
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "error"
    ///          ]
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(tag = "type")]
    pub enum JavascriptLog {
        #[serde(rename = "log")]
        Log {
            #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
            callstack: ::std::option::Option<::std::string::String>,
            message: ::std::string::String,
        },
        #[serde(rename = "error")]
        Error {
            #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
            callstack: ::std::option::Option<::std::string::String>,
            message: ::std::string::String,
        },
    }

    ///`JournaldLogItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "message",
    ///    "timestamp"
    ///  ],
    ///  "properties": {
    ///    "message": {
    ///      "type": "string"
    ///    },
    ///    "priority": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "timestamp": {
    ///      "type": "string"
    ///    },
    ///    "unit": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct JournaldLogItem {
        pub message: ::std::string::String,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub priority: ::std::option::Option<::std::string::String>,
        pub timestamp: ::std::string::String,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub unit: ::std::option::Option<::std::string::String>,
    }

    ///`JournaldLogsResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "logs"
    ///  ],
    ///  "properties": {
    ///    "logs": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/JournaldLogItem"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct JournaldLogsResponse {
        pub logs: ::std::vec::Vec<JournaldLogItem>,
    }

    ///`KillVmResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "id"
    ///  ],
    ///  "properties": {
    ///    "id": {
    ///      "$ref": "#/components/schemas/VmId"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct KillVmResponse {
        pub id: VmId,
    }

    ///`LinuxGroupSpec`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "name"
    ///  ],
    ///  "properties": {
    ///    "gid": {
    ///      "description": "Optional fixed GID; if None, allocate",
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ],
    ///      "format": "int32",
    ///      "minimum": 0.0
    ///    },
    ///    "name": {
    ///      "description": "Unique group name",
    ///      "type": "string"
    ///    },
    ///    "system": {
    ///      "description": "System group (allocator uses system range)",
    ///      "default": false,
    ///      "type": [
    ///        "boolean",
    ///        "null"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct LinuxGroupSpec {
        ///Optional fixed GID; if None, allocate
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub gid: ::std::option::Option<i32>,
        ///Unique group name
        pub name: ::std::string::String,
        ///System group (allocator uses system range)
        #[serde(default = "defaults::linux_group_spec_system")]
        pub system: ::std::option::Option<bool>,
    }

    ///`LinuxUserSpec`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "name"
    ///  ],
    ///  "properties": {
    ///    "gecos": {
    ///      "description": "GECOS field (descriptive string, e.g., full name)",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "groups": {
    ///      "description": "Groups to add user to (all groups use 'm'
    /// membership lines)",
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "home": {
    ///      "description": "Home directory path (optional, defaults to
    /// /home/{username} for regular users, / for system users)",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "name": {
    ///      "description": "Unique username",
    ///      "type": "string"
    ///    },
    ///    "shell": {
    ///      "description": "Login shell (optional, defaults to /bin/bash for
    /// regular users, /usr/sbin/nologin for system users)",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "system": {
    ///      "description": "System user (allocator uses system range)",
    ///      "default": false,
    ///      "type": [
    ///        "boolean",
    ///        "null"
    ///      ]
    ///    },
    ///    "uid": {
    ///      "description": "Optional fixed UID; if None, allocate",
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ],
    ///      "format": "int32",
    ///      "minimum": 0.0
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct LinuxUserSpec {
        ///GECOS field (descriptive string, e.g., full name)
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub gecos: ::std::option::Option<::std::string::String>,
        ///Groups to add user to (all groups use 'm' membership lines)
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub groups: ::std::vec::Vec<::std::string::String>,
        ///Home directory path (optional, defaults to /home/{username} for
        /// regular users, / for system users)
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub home: ::std::option::Option<::std::string::String>,
        ///Unique username
        pub name: ::std::string::String,
        ///Login shell (optional, defaults to /bin/bash for regular users,
        /// /usr/sbin/nologin for system users)
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub shell: ::std::option::Option<::std::string::String>,
        ///System user (allocator uses system range)
        #[serde(default = "defaults::linux_user_spec_system")]
        pub system: ::std::option::Option<bool>,
        ///Optional fixed UID; if None, allocate
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub uid: ::std::option::Option<i32>,
    }

    ///`ListBranchesResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "branches"
    ///  ],
    ///  "properties": {
    ///    "branches": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/BranchInfo"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ListBranchesResponse {
        pub branches: ::std::vec::Vec<BranchInfo>,
    }

    ///`ListDeploymentsResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "entries",
    ///    "offset",
    ///    "total"
    ///  ],
    ///  "properties": {
    ///    "entries": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/DeploymentLogEntry"
    ///      }
    ///    },
    ///    "offset": {
    ///      "type": "integer",
    ///      "format": "int64",
    ///      "minimum": 0.0
    ///    },
    ///    "total": {
    ///      "type": "integer",
    ///      "format": "int64",
    ///      "minimum": 0.0
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ListDeploymentsResponse {
        pub entries: ::std::vec::Vec<DeploymentLogEntry>,
        pub offset: i64,
        pub total: i64,
    }

    ///`ListGitPermissionSuccess`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "repositories"
    ///  ],
    ///  "properties": {
    ///    "repositories": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/AccessibleRepository"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ListGitPermissionSuccess {
        pub repositories: ::std::vec::Vec<AccessibleRepository>,
    }

    ///`ListGitTokensSuccess`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "tokens"
    ///  ],
    ///  "properties": {
    ///    "tokens": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/AccessTokenInfo"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ListGitTokensSuccess {
        pub tokens: ::std::vec::Vec<AccessTokenInfo>,
    }

    ///`ListIdentitiesSuccess`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "identities",
    ///    "offset",
    ///    "total"
    ///  ],
    ///  "properties": {
    ///    "identities": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/FreestyleIdentity"
    ///      }
    ///    },
    ///    "offset": {
    ///      "type": "integer",
    ///      "format": "int64",
    ///      "minimum": 0.0
    ///    },
    ///    "total": {
    ///      "type": "integer",
    ///      "format": "int64",
    ///      "minimum": 0.0
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ListIdentitiesSuccess {
        pub identities: ::std::vec::Vec<FreestyleIdentity>,
        pub offset: i64,
        pub total: i64,
    }

    ///`ListRecordsResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "records"
    ///  ],
    ///  "properties": {
    ///    "records": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/DnsRecord"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ListRecordsResponse {
        pub records: ::std::vec::Vec<DnsRecord>,
    }

    ///`ListRepositoriesSuccess`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "offset",
    ///    "repositories",
    ///    "total"
    ///  ],
    ///  "properties": {
    ///    "offset": {
    ///      "type": "integer",
    ///      "format": "int64",
    ///      "minimum": 0.0
    ///    },
    ///    "repositories": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/RepositoryMetadata"
    ///      }
    ///    },
    ///    "total": {
    ///      "type": "integer",
    ///      "format": "int64",
    ///      "minimum": 0.0
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ListRepositoriesSuccess {
        pub offset: i64,
        pub repositories: ::std::vec::Vec<RepositoryMetadata>,
        pub total: i64,
    }

    ///`ListScheduleExecutionsResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "executions",
    ///    "limit",
    ///    "offset",
    ///    "total"
    ///  ],
    ///  "properties": {
    ///    "executions": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/Execution"
    ///      }
    ///    },
    ///    "limit": {
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "offset": {
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "total": {
    ///      "type": "integer",
    ///      "format": "int64"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ListScheduleExecutionsResponse {
        pub executions: ::std::vec::Vec<Execution>,
        pub limit: i64,
        pub offset: i64,
        pub total: i64,
    }

    ///`ListSchedulesResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "schedules"
    ///  ],
    ///  "properties": {
    ///    "schedules": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/ScheduleWithStats"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ListSchedulesResponse {
        pub schedules: ::std::vec::Vec<ScheduleWithStats>,
    }

    ///`ListServicesResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "error",
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "error": {
    ///      "description": "Error code in SCREAMING_SNAKE_CASE",
    ///      "type": "string"
    ///    },
    ///    "message": {
    ///      "description": "Human-readable error message",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ListServicesResponse {
        ///Error code in SCREAMING_SNAKE_CASE
        pub error: ::std::string::String,
        ///Human-readable error message
        pub message: ::std::string::String,
    }

    ///`ListSnapshotsResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "deletedCount",
    ///    "failedCount",
    ///    "snapshots",
    ///    "totalCount"
    ///  ],
    ///  "properties": {
    ///    "deletedCount": {
    ///      "type": "integer",
    ///      "minimum": 0.0
    ///    },
    ///    "failedCount": {
    ///      "type": "integer",
    ///      "minimum": 0.0
    ///    },
    ///    "snapshots": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/SnapshotInfo"
    ///      }
    ///    },
    ///    "totalCount": {
    ///      "type": "integer",
    ///      "minimum": 0.0
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ListSnapshotsResponse {
        #[serde(rename = "deletedCount")]
        pub deleted_count: u64,
        #[serde(rename = "failedCount")]
        pub failed_count: u64,
        pub snapshots: ::std::vec::Vec<SnapshotInfo>,
        #[serde(rename = "totalCount")]
        pub total_count: u64,
    }

    ///`ListTagsResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "tags"
    ///  ],
    ///  "properties": {
    ///    "tags": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/TagObject"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ListTagsResponse {
        pub tags: ::std::vec::Vec<TagObject>,
    }

    ///`ListTerminalsResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "error",
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "error": {
    ///      "description": "Error code in SCREAMING_SNAKE_CASE",
    ///      "type": "string"
    ///    },
    ///    "message": {
    ///      "description": "Human-readable error message",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ListTerminalsResponse {
        ///Error code in SCREAMING_SNAKE_CASE
        pub error: ::std::string::String,
        ///Human-readable error message
        pub message: ::std::string::String,
    }

    ///`ListVmPermissionsSuccess`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "offset",
    ///    "permissions",
    ///    "total"
    ///  ],
    ///  "properties": {
    ///    "offset": {
    ///      "type": "integer",
    ///      "format": "int64",
    ///      "minimum": 0.0
    ///    },
    ///    "permissions": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/VmPermission"
    ///      }
    ///    },
    ///    "total": {
    ///      "type": "integer",
    ///      "format": "int64",
    ///      "minimum": 0.0
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ListVmPermissionsSuccess {
        pub offset: i64,
        pub permissions: ::std::vec::Vec<VmPermission>,
        pub total: i64,
    }

    ///`ListVmsResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "runningCount",
    ///    "startingCount",
    ///    "stoppedCount",
    ///    "suspendedCount",
    ///    "totalCount",
    ///    "vms"
    ///  ],
    ///  "properties": {
    ///    "runningCount": {
    ///      "type": "integer",
    ///      "minimum": 0.0
    ///    },
    ///    "startingCount": {
    ///      "type": "integer",
    ///      "minimum": 0.0
    ///    },
    ///    "stoppedCount": {
    ///      "type": "integer",
    ///      "minimum": 0.0
    ///    },
    ///    "suspendedCount": {
    ///      "type": "integer",
    ///      "minimum": 0.0
    ///    },
    ///    "totalCount": {
    ///      "type": "integer",
    ///      "minimum": 0.0
    ///    },
    ///    "userId": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "vms": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/VmInfo"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ListVmsResponse {
        #[serde(rename = "runningCount")]
        pub running_count: u64,
        #[serde(rename = "startingCount")]
        pub starting_count: u64,
        #[serde(rename = "stoppedCount")]
        pub stopped_count: u64,
        #[serde(rename = "suspendedCount")]
        pub suspended_count: u64,
        #[serde(rename = "totalCount")]
        pub total_count: u64,
        #[serde(
            rename = "userId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub user_id: ::std::option::Option<::std::string::String>,
        pub vms: ::std::vec::Vec<VmInfo>,
    }

    ///`LogEntry`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "line"
    ///  ],
    ///  "properties": {
    ///    "line": {
    ///      "type": "string"
    ///    },
    ///    "timestamp": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct LogEntry {
        pub line: ::std::string::String,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub timestamp: ::std::option::Option<::std::string::String>,
    }

    ///`MetricsDataPoint`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "executionCount",
    ///    "failedCount",
    ///    "succeededCount",
    ///    "timestamp"
    ///  ],
    ///  "properties": {
    ///    "averageLatencyMs": {
    ///      "type": [
    ///        "number",
    ///        "null"
    ///      ],
    ///      "format": "double"
    ///    },
    ///    "executionCount": {
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "failedCount": {
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "succeededCount": {
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "timestamp": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct MetricsDataPoint {
        #[serde(
            rename = "averageLatencyMs",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub average_latency_ms: ::std::option::Option<f64>,
        #[serde(rename = "executionCount")]
        pub execution_count: i64,
        #[serde(rename = "failedCount")]
        pub failed_count: i64,
        #[serde(rename = "succeededCount")]
        pub succeeded_count: i64,
        pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
    }

    ///`MetricsTimelineResponseBody`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "dataPoints",
    ///    "end",
    ///    "scheduleId",
    ///    "start"
    ///  ],
    ///  "properties": {
    ///    "dataPoints": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/MetricsDataPoint"
    ///      }
    ///    },
    ///    "end": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "scheduleId": {
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "start": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct MetricsTimelineResponseBody {
        #[serde(rename = "dataPoints")]
        pub data_points: ::std::vec::Vec<MetricsDataPoint>,
        pub end: ::chrono::DateTime<::chrono::offset::Utc>,
        #[serde(rename = "scheduleId")]
        pub schedule_id: ::uuid::Uuid,
        pub start: ::chrono::DateTime<::chrono::offset::Utc>,
    }

    ///`NetworkPermissionData`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "query"
    ///  ],
    ///  "properties": {
    ///    "behavior": {
    ///      "$ref": "#/components/schemas/Behavior"
    ///    },
    ///    "query": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct NetworkPermissionData {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub behavior: ::std::option::Option<Behavior>,
        pub query: ::std::string::String,
    }

    ///`OptimizeVmResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "id",
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "message": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct OptimizeVmResponse {
        pub id: ::std::string::String,
        pub message: ::std::string::String,
    }

    ///`PortMapping`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "port",
    ///    "targetPort"
    ///  ],
    ///  "properties": {
    ///    "port": {
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "targetPort": {
    ///      "type": "integer",
    ///      "format": "int32"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct PortMapping {
        pub port: i32,
        #[serde(rename = "targetPort")]
        pub target_port: i32,
    }

    ///`PublicDomainMappingError`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "$ref": "#/components/schemas/Error_DomainMappingError"
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(transparent)]
    pub struct PublicDomainMappingError(pub ErrorDomainMappingError);
    impl ::std::ops::Deref for PublicDomainMappingError {
        type Target = ErrorDomainMappingError;
        fn deref(&self) -> &ErrorDomainMappingError {
            &self.0
        }
    }

    impl ::std::convert::From<PublicDomainMappingError> for ErrorDomainMappingError {
        fn from(value: PublicDomainMappingError) -> Self {
            value.0
        }
    }

    impl ::std::convert::From<ErrorDomainMappingError> for PublicDomainMappingError {
        fn from(value: ErrorDomainMappingError) -> Self {
            Self(value)
        }
    }

    ///`PutFileResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "error",
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "error": {
    ///      "description": "Error code in SCREAMING_SNAKE_CASE",
    ///      "type": "string"
    ///    },
    ///    "message": {
    ///      "description": "Human-readable error message",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct PutFileResponse {
        ///Error code in SCREAMING_SNAKE_CASE
        pub error: ::std::string::String,
        ///Human-readable error message
        pub message: ::std::string::String,
    }

    ///Redirect configuration for permanent or temporary redirects
    /// (Vercel-compatible)
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Redirect configuration for permanent or temporary
    /// redirects (Vercel-compatible)",
    ///  "type": "object",
    ///  "required": [
    ///    "destination",
    ///    "source"
    ///  ],
    ///  "properties": {
    ///    "destination": {
    ///      "description": "Destination path or URL",
    ///      "type": "string"
    ///    },
    ///    "permanent": {
    ///      "description": "If true, uses 308 (permanent redirect). If false,
    /// uses 307 (temporary redirect).\nIgnored if `status_code` is explicitly
    /// set. Vercel-compatible alternative to `status_code`.",
    ///      "type": [
    ///        "boolean",
    ///        "null"
    ///      ]
    ///    },
    ///    "source": {
    ///      "description": "Source pattern (can be exact path or regex)",
    ///      "type": "string"
    ///    },
    ///    "statusCode": {
    ///      "description": "HTTP status code for redirect (301, 302, 307, 308).
    /// Takes precedence over `permanent`.\nIf neither `status_code` nor
    /// `permanent` is provided, defaults to 308.",
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ],
    ///      "format": "int32",
    ///      "minimum": 0.0
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct RedirectConfig {
        ///Destination path or URL
        pub destination: ::std::string::String,
        ///If true, uses 308 (permanent redirect). If false, uses 307
        /// (temporary redirect). Ignored if `status_code` is explicitly
        /// set. Vercel-compatible alternative to `status_code`.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub permanent: ::std::option::Option<bool>,
        ///Source pattern (can be exact path or regex)
        pub source: ::std::string::String,
        ///HTTP status code for redirect (301, 302, 307, 308). Takes precedence
        /// over `permanent`. If neither `status_code` nor `permanent`
        /// is provided, defaults to 308.
        #[serde(
            rename = "statusCode",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub status_code: ::std::option::Option<i32>,
    }

    ///`RepositoryInfoRaw`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "accountId",
    ///    "createdAt",
    ///    "defaultBranch",
    ///    "id",
    ///    "visibility"
    ///  ],
    ///  "properties": {
    ///    "accountId": {
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "createdAt": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "defaultBranch": {
    ///      "type": "string"
    ///    },
    ///    "id": {
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "name": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "visibility": {
    ///      "$ref": "#/components/schemas/Visibility"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct RepositoryInfoRaw {
        #[serde(rename = "accountId")]
        pub account_id: ::uuid::Uuid,
        #[serde(rename = "createdAt")]
        pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        #[serde(rename = "defaultBranch")]
        pub default_branch: ::std::string::String,
        pub id: ::uuid::Uuid,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
        pub visibility: Visibility,
    }

    ///`RepositoryMetadata`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "branches",
    ///    "defaultBranch",
    ///    "tags"
    ///  ],
    ///  "properties": {
    ///    "branches": {
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "$ref": "#/components/schemas/BranchDetails"
    ///      }
    ///    },
    ///    "defaultBranch": {
    ///      "type": "string"
    ///    },
    ///    "tags": {
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "$ref": "#/components/schemas/TagDetails"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct RepositoryMetadata {
        pub branches: ::std::collections::HashMap<::std::string::String, BranchDetails>,
        #[serde(rename = "defaultBranch")]
        pub default_branch: ::std::string::String,
        pub tags: ::std::collections::HashMap<::std::string::String, TagDetails>,
    }

    ///`ResizeVmRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "memSizeGb": {
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ],
    ///      "format": "int32",
    ///      "minimum": 0.0
    ///    },
    ///    "rootfsSizeGb": {
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ],
    ///      "format": "int64",
    ///      "minimum": 0.0
    ///    },
    ///    "vcpuCount": {
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ],
    ///      "format": "int32",
    ///      "minimum": 0.0
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ResizeVmRequest {
        #[serde(
            rename = "memSizeGb",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub mem_size_gb: ::std::option::Option<i32>,
        #[serde(
            rename = "rootfsSizeGb",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub rootfs_size_gb: ::std::option::Option<i64>,
        #[serde(
            rename = "vcpuCount",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub vcpu_count: ::std::option::Option<i32>,
    }

    impl ::std::default::Default for ResizeVmRequest {
        fn default() -> Self {
            Self {
                mem_size_gb: Default::default(),
                rootfs_size_gb: Default::default(),
                vcpu_count: Default::default(),
            }
        }
    }

    ///`ResizeVmResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object"
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(transparent)]
    pub struct ResizeVmResponse(pub ::serde_json::Map<::std::string::String, ::serde_json::Value>);
    impl ::std::ops::Deref for ResizeVmResponse {
        type Target = ::serde_json::Map<::std::string::String, ::serde_json::Value>;
        fn deref(&self) -> &::serde_json::Map<::std::string::String, ::serde_json::Value> {
            &self.0
        }
    }

    impl ::std::convert::From<ResizeVmResponse>
        for ::serde_json::Map<::std::string::String, ::serde_json::Value>
    {
        fn from(value: ResizeVmResponse) -> Self {
            value.0
        }
    }

    impl ::std::convert::From<::serde_json::Map<::std::string::String, ::serde_json::Value>>
        for ResizeVmResponse
    {
        fn from(value: ::serde_json::Map<::std::string::String, ::serde_json::Value>) -> Self {
            Self(value)
        }
    }

    ///`RevokeGitTokenRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "tokenId"
    ///  ],
    ///  "properties": {
    ///    "tokenId": {
    ///      "type": "string",
    ///      "format": "uuid"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct RevokeGitTokenRequest {
        #[serde(rename = "tokenId")]
        pub token_id: ::uuid::Uuid,
    }

    ///Rewrite configuration for URL rewrites (internal, not visible to client)
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Rewrite configuration for URL rewrites (internal, not
    /// visible to client)",
    ///  "type": "object",
    ///  "required": [
    ///    "destination",
    ///    "source"
    ///  ],
    ///  "properties": {
    ///    "destination": {
    ///      "description": "Destination path with optional capture group
    /// substitution ($1, $2, etc.)",
    ///      "type": "string"
    ///    },
    ///    "source": {
    ///      "description": "Source pattern (regex pattern)",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct RewriteConfig {
        ///Destination path with optional capture group substitution ($1, $2,
        /// etc.)
        pub destination: ::std::string::String,
        ///Source pattern (regex pattern)
        pub source: ::std::string::String,
    }

    ///`Schedule`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "accountId",
    ///    "active",
    ///    "createdAt",
    ///    "cron",
    ///    "deploymentId",
    ///    "id",
    ///    "payload",
    ///    "timezone",
    ///    "updatedAt"
    ///  ],
    ///  "properties": {
    ///    "accountId": {
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "active": {
    ///      "type": "boolean"
    ///    },
    ///    "createdAt": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "cron": {
    ///      "type": "string"
    ///    },
    ///    "deploymentId": {
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "id": {
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "path": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "payload": {},
    ///    "timezone": {
    ///      "type": "string"
    ///    },
    ///    "updatedAt": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Schedule {
        #[serde(rename = "accountId")]
        pub account_id: ::uuid::Uuid,
        pub active: bool,
        #[serde(rename = "createdAt")]
        pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        pub cron: ::std::string::String,
        #[serde(rename = "deploymentId")]
        pub deployment_id: ::uuid::Uuid,
        pub id: ::uuid::Uuid,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub path: ::std::option::Option<::std::string::String>,
        pub payload: ::serde_json::Value,
        pub timezone: ::std::string::String,
        #[serde(rename = "updatedAt")]
        pub updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
    }

    ///`ScheduleSuccessRateResponseBody`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "end",
    ///    "failed",
    ///    "start",
    ///    "succeeded",
    ///    "successRate",
    ///    "total"
    ///  ],
    ///  "properties": {
    ///    "averageLatencyMs": {
    ///      "type": [
    ///        "number",
    ///        "null"
    ///      ],
    ///      "format": "double"
    ///    },
    ///    "end": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "failed": {
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "start": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "succeeded": {
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "successRate": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "total": {
    ///      "type": "integer",
    ///      "format": "int64"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ScheduleSuccessRateResponseBody {
        #[serde(
            rename = "averageLatencyMs",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub average_latency_ms: ::std::option::Option<f64>,
        pub end: ::chrono::DateTime<::chrono::offset::Utc>,
        pub failed: i64,
        pub start: ::chrono::DateTime<::chrono::offset::Utc>,
        pub succeeded: i64,
        #[serde(rename = "successRate")]
        pub success_rate: f64,
        pub total: i64,
    }

    ///`ScheduleWithStats`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/Schedule"
    ///    },
    ///    {
    ///      "type": "object",
    ///      "properties": {
    ///        "averageLatencyMs": {
    ///          "type": [
    ///            "number",
    ///            "null"
    ///          ],
    ///          "format": "double"
    ///        },
    ///        "executionCount": {
    ///          "type": [
    ///            "integer",
    ///            "null"
    ///          ],
    ///          "format": "int64"
    ///        },
    ///        "successRate": {
    ///          "type": [
    ///            "number",
    ///            "null"
    ///          ],
    ///          "format": "double"
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ScheduleWithStats {
        #[serde(rename = "accountId")]
        pub account_id: ::uuid::Uuid,
        pub active: bool,
        #[serde(
            rename = "averageLatencyMs",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub average_latency_ms: ::std::option::Option<f64>,
        #[serde(rename = "createdAt")]
        pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        pub cron: ::std::string::String,
        #[serde(rename = "deploymentId")]
        pub deployment_id: ::uuid::Uuid,
        #[serde(
            rename = "executionCount",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub execution_count: ::std::option::Option<i64>,
        pub id: ::uuid::Uuid,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub path: ::std::option::Option<::std::string::String>,
        pub payload: ::serde_json::Value,
        #[serde(
            rename = "successRate",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub success_rate: ::std::option::Option<f64>,
        pub timezone: ::std::string::String,
        #[serde(rename = "updatedAt")]
        pub updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
    }

    ///`ServiceIdItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "id"
    ///  ],
    ///  "properties": {
    ///    "id": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ServiceIdItem {
        pub id: ::std::string::String,
    }

    ///`SetDefaultBranchRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "defaultBranch"
    ///  ],
    ///  "properties": {
    ///    "defaultBranch": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct SetDefaultBranchRequest {
        #[serde(rename = "defaultBranch")]
        pub default_branch: ::std::string::String,
    }

    ///`SetDefaultBranchResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object"
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(transparent)]
    pub struct SetDefaultBranchResponse(
        pub ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    );
    impl ::std::ops::Deref for SetDefaultBranchResponse {
        type Target = ::serde_json::Map<::std::string::String, ::serde_json::Value>;
        fn deref(&self) -> &::serde_json::Map<::std::string::String, ::serde_json::Value> {
            &self.0
        }
    }

    impl ::std::convert::From<SetDefaultBranchResponse>
        for ::serde_json::Map<::std::string::String, ::serde_json::Value>
    {
        fn from(value: SetDefaultBranchResponse) -> Self {
            value.0
        }
    }

    impl ::std::convert::From<::serde_json::Map<::std::string::String, ::serde_json::Value>>
        for SetDefaultBranchResponse
    {
        fn from(value: ::serde_json::Map<::std::string::String, ::serde_json::Value>) -> Self {
            Self(value)
        }
    }

    ///`SetVisibilityRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "visibility"
    ///  ],
    ///  "properties": {
    ///    "visibility": {
    ///      "$ref": "#/components/schemas/Visibility"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct SetVisibilityRequest {
        pub visibility: Visibility,
    }

    ///`SetVisibilityResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object"
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(transparent)]
    pub struct SetVisibilityResponse(
        pub ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    );
    impl ::std::ops::Deref for SetVisibilityResponse {
        type Target = ::serde_json::Map<::std::string::String, ::serde_json::Value>;
        fn deref(&self) -> &::serde_json::Map<::std::string::String, ::serde_json::Value> {
            &self.0
        }
    }

    impl ::std::convert::From<SetVisibilityResponse>
        for ::serde_json::Map<::std::string::String, ::serde_json::Value>
    {
        fn from(value: SetVisibilityResponse) -> Self {
            value.0
        }
    }

    impl ::std::convert::From<::serde_json::Map<::std::string::String, ::serde_json::Value>>
        for SetVisibilityResponse
    {
        fn from(value: ::serde_json::Map<::std::string::String, ::serde_json::Value>) -> Self {
            Self(value)
        }
    }

    ///`Signature`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "date",
    ///    "email",
    ///    "name"
    ///  ],
    ///  "properties": {
    ///    "date": {
    ///      "description": "The date marker for this signature",
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "email": {
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Signature {
        ///The date marker for this signature
        pub date: ::chrono::DateTime<::chrono::offset::Utc>,
        pub email: ::std::string::String,
        pub name: ::std::string::String,
    }

    ///Branded snapshot ID in the format `sc-<20 lowercase alphanumeric
    /// chars>`. Unlike `VmShortId`, this is a plain `String` wrapper with
    /// no fixed-size array or padding.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Branded snapshot ID in the format `sc-<20 lowercase
    /// alphanumeric chars>`.\nUnlike `VmShortId`, this is a plain `String`
    /// wrapper with no fixed-size array or padding.",
    ///  "type": "string"
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    #[serde(transparent)]
    pub struct SnapshotId(pub ::std::string::String);
    impl ::std::ops::Deref for SnapshotId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<SnapshotId> for ::std::string::String {
        fn from(value: SnapshotId) -> Self {
            value.0
        }
    }

    impl ::std::convert::From<::std::string::String> for SnapshotId {
        fn from(value: ::std::string::String) -> Self {
            Self(value)
        }
    }

    impl ::std::str::FromStr for SnapshotId {
        type Err = ::std::convert::Infallible;
        fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::fmt::Display for SnapshotId {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            self.0.fmt(f)
        }
    }

    ///`SnapshotInfo`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "createdAt",
    ///    "deleted",
    ///    "failed",
    ///    "snapshotId",
    ///    "snapshotPersistence",
    ///    "sourceVmId"
    ///  ],
    ///  "properties": {
    ///    "accountId": {
    ///      "$ref": "#/components/schemas/AccountId"
    ///    },
    ///    "createdAt": {
    ///      "description": "When the snapshot was created",
    ///      "type": "string"
    ///    },
    ///    "deleted": {
    ///      "description": "Whether the snapshot has been soft-deleted",
    ///      "type": "boolean"
    ///    },
    ///    "failed": {
    ///      "description": "Whether the snapshot creation failed",
    ///      "type": "boolean"
    ///    },
    ///    "failureReason": {
    ///      "description": "Reason for failure if applicable",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "memSizeMib": {
    ///      "description": "Memory size in MiB configured at snapshot time",
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ],
    ///      "format": "int32",
    ///      "minimum": 0.0
    ///    },
    ///    "name": {
    ///      "description": "Optional name for the snapshot",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "rootfsSizeMb": {
    ///      "description": "Root filesystem size in MB",
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ],
    ///      "format": "int64",
    ///      "minimum": 0.0
    ///    },
    ///    "snapshotId": {
    ///      "$ref": "#/components/schemas/SnapshotId"
    ///    },
    ///    "snapshotPersistence": {
    ///      "$ref": "#/components/schemas/SnapshotPersistence"
    ///    },
    ///    "sourceVmId": {
    ///      "$ref": "#/components/schemas/VmId"
    ///    },
    ///    "vcpuCount": {
    ///      "description": "Number of vCPUs configured at snapshot time",
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ],
    ///      "format": "int32",
    ///      "minimum": 0.0
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct SnapshotInfo {
        #[serde(
            rename = "accountId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub account_id: ::std::option::Option<AccountId>,
        ///When the snapshot was created
        #[serde(rename = "createdAt")]
        pub created_at: ::std::string::String,
        ///Whether the snapshot has been soft-deleted
        pub deleted: bool,
        ///Whether the snapshot creation failed
        pub failed: bool,
        ///Reason for failure if applicable
        #[serde(
            rename = "failureReason",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub failure_reason: ::std::option::Option<::std::string::String>,
        ///Memory size in MiB configured at snapshot time
        #[serde(
            rename = "memSizeMib",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub mem_size_mib: ::std::option::Option<i32>,
        ///Optional name for the snapshot
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
        ///Root filesystem size in MB
        #[serde(
            rename = "rootfsSizeMb",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub rootfs_size_mb: ::std::option::Option<i64>,
        #[serde(rename = "snapshotId")]
        pub snapshot_id: SnapshotId,
        #[serde(rename = "snapshotPersistence")]
        pub snapshot_persistence: SnapshotPersistence,
        #[serde(rename = "sourceVmId")]
        pub source_vm_id: VmId,
        ///Number of vCPUs configured at snapshot time
        #[serde(
            rename = "vcpuCount",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub vcpu_count: ::std::option::Option<i32>,
    }

    ///Branded snapshot layer ID — always a 16-character alphanumeric lowercase
    /// string.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Branded snapshot layer ID — always a 16-character
    /// alphanumeric lowercase string.",
    ///  "type": "string"
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    #[serde(transparent)]
    pub struct SnapshotLayerId(pub ::std::string::String);
    impl ::std::ops::Deref for SnapshotLayerId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<SnapshotLayerId> for ::std::string::String {
        fn from(value: SnapshotLayerId) -> Self {
            value.0
        }
    }

    impl ::std::convert::From<::std::string::String> for SnapshotLayerId {
        fn from(value: ::std::string::String) -> Self {
            Self(value)
        }
    }

    impl ::std::str::FromStr for SnapshotLayerId {
        type Err = ::std::convert::Infallible;
        fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::fmt::Display for SnapshotLayerId {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            self.0.fmt(f)
        }
    }

    ///`SnapshotPersistence`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "description": "When storage quota is reached, the least recently
    /// used snapshots will be\ndeleted. Higher priority = less likely to be
    /// evicted. Range 0-10, default 5.",
    ///      "type": "object",
    ///      "required": [
    ///        "type"
    ///      ],
    ///      "properties": {
    ///        "priority": {
    ///          "default": 5,
    ///          "type": [
    ///            "integer",
    ///            "null"
    ///          ],
    ///          "format": "int32",
    ///          "maximum": 10.0,
    ///          "minimum": 0.0
    ///        },
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "sticky"
    ///          ]
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "description": "The snapshot will be stored until manually
    /// deleted.",
    ///      "type": "object",
    ///      "required": [
    ///        "type"
    ///      ],
    ///      "properties": {
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "persistent"
    ///          ]
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(tag = "type")]
    pub enum SnapshotPersistence {
        ///When storage quota is reached, the least recently used snapshots
        /// will be deleted. Higher priority = less likely to be
        /// evicted. Range 0-10, default 5.
        #[serde(rename = "sticky")]
        Sticky {
            #[serde(default = "defaults::snapshot_persistence_sticky_priority")]
            priority: ::std::option::Option<i32>,
        },
        #[serde(rename = "persistent")]
        Persistent,
    }

    ///`SnapshotVmRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "name": {
    ///      "description": "Optional name/label for the snapshot",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "persistence": {
    ///      "$ref": "#/components/schemas/SnapshotPersistence"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct SnapshotVmRequest {
        ///Optional name/label for the snapshot
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub persistence: ::std::option::Option<SnapshotPersistence>,
    }

    impl ::std::default::Default for SnapshotVmRequest {
        fn default() -> Self {
            Self {
                name: Default::default(),
                persistence: Default::default(),
            }
        }
    }

    ///`SnapshotVmResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "snapshotId",
    ///    "sourceVmId"
    ///  ],
    ///  "properties": {
    ///    "snapshotId": {
    ///      "$ref": "#/components/schemas/SnapshotId"
    ///    },
    ///    "sourceVmId": {
    ///      "$ref": "#/components/schemas/VmId"
    ///    },
    ///    "sourceVmInstanceId": {
    ///      "$ref": "#/components/schemas/VmInstanceId"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct SnapshotVmResponse {
        #[serde(rename = "snapshotId")]
        pub snapshot_id: SnapshotId,
        #[serde(rename = "sourceVmId")]
        pub source_vm_id: VmId,
        #[serde(
            rename = "sourceVmInstanceId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub source_vm_instance_id: ::std::option::Option<VmInstanceId>,
    }

    ///`StartVmRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "activityThresholdBytes": {
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ],
    ///      "format": "int64",
    ///      "minimum": 0.0
    ///    },
    ///    "idleTimeoutSeconds": {
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ],
    ///      "format": "int64"
    ///    },
    ///    "readySignalTimeoutSeconds": {
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ],
    ///      "format": "int64"
    ///    },
    ///    "waitForReadySignal": {
    ///      "type": [
    ///        "boolean",
    ///        "null"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct StartVmRequest {
        #[serde(
            rename = "activityThresholdBytes",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub activity_threshold_bytes: ::std::option::Option<i64>,
        #[serde(
            rename = "idleTimeoutSeconds",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub idle_timeout_seconds: ::std::option::Option<i64>,
        #[serde(
            rename = "readySignalTimeoutSeconds",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub ready_signal_timeout_seconds: ::std::option::Option<i64>,
        #[serde(
            rename = "waitForReadySignal",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub wait_for_ready_signal: ::std::option::Option<bool>,
    }

    impl ::std::default::Default for StartVmRequest {
        fn default() -> Self {
            Self {
                activity_threshold_bytes: Default::default(),
                idle_timeout_seconds: Default::default(),
                ready_signal_timeout_seconds: Default::default(),
                wait_for_ready_signal: Default::default(),
            }
        }
    }

    ///`StartedVmResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "domains",
    ///    "id"
    ///  ],
    ///  "properties": {
    ///    "console_url": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "domains": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "id": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct StartedVmResponse {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub console_url: ::std::option::Option<::std::string::String>,
        pub domains: ::std::vec::Vec<::std::string::String>,
        pub id: ::std::string::String,
    }

    ///`StopVmResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "vmId",
    ///    "vmInstanceId"
    ///  ],
    ///  "properties": {
    ///    "vmId": {
    ///      "$ref": "#/components/schemas/VmId"
    ///    },
    ///    "vmInstanceId": {
    ///      "$ref": "#/components/schemas/VmInstanceId"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct StopVmResponse {
        #[serde(rename = "vmId")]
        pub vm_id: VmId,
        #[serde(rename = "vmInstanceId")]
        pub vm_instance_id: VmInstanceId,
    }

    ///`SuccessfullyDeletedDomainMapping`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object"
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(transparent)]
    pub struct SuccessfullyDeletedDomainMapping(
        pub ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    );
    impl ::std::ops::Deref for SuccessfullyDeletedDomainMapping {
        type Target = ::serde_json::Map<::std::string::String, ::serde_json::Value>;
        fn deref(&self) -> &::serde_json::Map<::std::string::String, ::serde_json::Value> {
            &self.0
        }
    }

    impl ::std::convert::From<SuccessfullyDeletedDomainMapping>
        for ::serde_json::Map<::std::string::String, ::serde_json::Value>
    {
        fn from(value: SuccessfullyDeletedDomainMapping) -> Self {
            value.0
        }
    }

    impl ::std::convert::From<::serde_json::Map<::std::string::String, ::serde_json::Value>>
        for SuccessfullyDeletedDomainMapping
    {
        fn from(value: ::serde_json::Map<::std::string::String, ::serde_json::Value>) -> Self {
            Self(value)
        }
    }

    ///`SuspendVmResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "id",
    ///    "snapshotLayerId",
    ///    "vmInstanceId"
    ///  ],
    ///  "properties": {
    ///    "id": {
    ///      "description": "The ID of the VM that was suspended",
    ///      "type": "string"
    ///    },
    ///    "snapshotLayerId": {
    ///      "$ref": "#/components/schemas/SnapshotLayerId"
    ///    },
    ///    "vmInstanceId": {
    ///      "$ref": "#/components/schemas/VmInstanceId"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct SuspendVmResponse {
        ///The ID of the VM that was suspended
        pub id: ::std::string::String,
        #[serde(rename = "snapshotLayerId")]
        pub snapshot_layer_id: SnapshotLayerId,
        #[serde(rename = "vmInstanceId")]
        pub vm_instance_id: VmInstanceId,
    }

    ///`SystemdConfig`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "patchedServices": {
    ///      "type": [
    ///        "array",
    ///        "null"
    ///      ],
    ///      "items": {
    ///        "$ref": "#/components/schemas/SystemdUnitSpecPatch"
    ///      }
    ///    },
    ///    "services": {
    ///      "type": [
    ///        "array",
    ///        "null"
    ///      ],
    ///      "items": {
    ///        "$ref": "#/components/schemas/SystemdUnitSpec"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct SystemdConfig {
        #[serde(
            rename = "patchedServices",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub patched_services: ::std::option::Option<::std::vec::Vec<SystemdUnitSpecPatch>>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub services: ::std::option::Option<::std::vec::Vec<SystemdUnitSpec>>,
    }

    impl ::std::default::Default for SystemdConfig {
        fn default() -> Self {
            Self {
                patched_services: Default::default(),
                services: Default::default(),
            }
        }
    }

    ///`SystemdCreateServiceResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "message",
    ///    "serviceName",
    ///    "success"
    ///  ],
    ///  "properties": {
    ///    "message": {
    ///      "type": "string"
    ///    },
    ///    "serviceName": {
    ///      "type": "string"
    ///    },
    ///    "success": {
    ///      "type": "boolean"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct SystemdCreateServiceResponse {
        pub message: ::std::string::String,
        #[serde(rename = "serviceName")]
        pub service_name: ::std::string::String,
        pub success: bool,
    }

    ///`SystemdDeleteServiceResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "message",
    ///    "success"
    ///  ],
    ///  "properties": {
    ///    "message": {
    ///      "type": "string"
    ///    },
    ///    "success": {
    ///      "type": "boolean"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct SystemdDeleteServiceResponse {
        pub message: ::std::string::String,
        pub success: bool,
    }

    ///`SystemdListServicesResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "services"
    ///  ],
    ///  "properties": {
    ///    "services": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/SystemdServiceListItem"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct SystemdListServicesResponse {
        pub services: ::std::vec::Vec<SystemdServiceListItem>,
    }

    ///`SystemdRestartPolicy`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "policy"
    ///  ],
    ///  "properties": {
    ///    "policy": {
    ///      "$ref": "#/components/schemas/SystemdRestartPolicyKind"
    ///    },
    ///    "restartSec": {
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ],
    ///      "format": "int64",
    ///      "minimum": 0.0
    ///    },
    ///    "startLimitBurst": {
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ],
    ///      "format": "int32",
    ///      "minimum": 0.0
    ///    },
    ///    "startLimitIntervalSec": {
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ],
    ///      "format": "int64",
    ///      "minimum": 0.0
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct SystemdRestartPolicy {
        pub policy: SystemdRestartPolicyKind,
        #[serde(
            rename = "restartSec",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub restart_sec: ::std::option::Option<i64>,
        #[serde(
            rename = "startLimitBurst",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub start_limit_burst: ::std::option::Option<i32>,
        #[serde(
            rename = "startLimitIntervalSec",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub start_limit_interval_sec: ::std::option::Option<i64>,
    }

    ///`SystemdRestartPolicyKind`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "no",
    ///    "on-failure",
    ///    "always",
    ///    "on-abnormal"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum SystemdRestartPolicyKind {
        #[serde(rename = "no")]
        No,
        #[serde(rename = "on-failure")]
        OnFailure,
        #[serde(rename = "always")]
        Always,
        #[serde(rename = "on-abnormal")]
        OnAbnormal,
    }

    impl ::std::fmt::Display for SystemdRestartPolicyKind {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::No => f.write_str("no"),
                Self::OnFailure => f.write_str("on-failure"),
                Self::Always => f.write_str("always"),
                Self::OnAbnormal => f.write_str("on-abnormal"),
            }
        }
    }

    impl ::std::str::FromStr for SystemdRestartPolicyKind {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "no" => Ok(Self::No),
                "on-failure" => Ok(Self::OnFailure),
                "always" => Ok(Self::Always),
                "on-abnormal" => Ok(Self::OnAbnormal),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for SystemdRestartPolicyKind {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for SystemdRestartPolicyKind {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for SystemdRestartPolicyKind {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`SystemdServiceListItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "name"
    ///  ],
    ///  "properties": {
    ///    "name": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct SystemdServiceListItem {
        pub name: ::std::string::String,
    }

    ///`SystemdServiceStatus`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "activeState",
    ///    "description",
    ///    "loadState",
    ///    "name",
    ///    "subState"
    ///  ],
    ///  "properties": {
    ///    "activeState": {
    ///      "type": "string"
    ///    },
    ///    "description": {
    ///      "type": "string"
    ///    },
    ///    "loadState": {
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "subState": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct SystemdServiceStatus {
        #[serde(rename = "activeState")]
        pub active_state: ::std::string::String,
        pub description: ::std::string::String,
        #[serde(rename = "loadState")]
        pub load_state: ::std::string::String,
        pub name: ::std::string::String,
        #[serde(rename = "subState")]
        pub sub_state: ::std::string::String,
    }

    ///`SystemdUnitMode`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "oneshot",
    ///    "service"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum SystemdUnitMode {
        #[serde(rename = "oneshot")]
        Oneshot,
        #[serde(rename = "service")]
        Service,
    }

    impl ::std::fmt::Display for SystemdUnitMode {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Oneshot => f.write_str("oneshot"),
                Self::Service => f.write_str("service"),
            }
        }
    }

    impl ::std::str::FromStr for SystemdUnitMode {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "oneshot" => Ok(Self::Oneshot),
                "service" => Ok(Self::Service),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for SystemdUnitMode {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for SystemdUnitMode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for SystemdUnitMode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`SystemdUnitSpec`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "exec",
    ///    "mode",
    ///    "name"
    ///  ],
    ///  "properties": {
    ///    "after": {
    ///      "description": "Establishes an ordering dependency. The current
    /// unit will start only\nafter the units listed in After= have started.
    /// This is useful for\nensuring that certain services are up and running
    /// before the current\nservice begins its operation.",
    ///      "type": [
    ///        "array",
    ///        "null"
    ///      ],
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "deleteAfterSuccess": {
    ///      "description": "For oneshot: remove unit on success.",
    ///      "default": false,
    ///      "type": [
    ///        "boolean",
    ///        "null"
    ///      ]
    ///    },
    ///    "enable": {
    ///      "description": "Whether to enable this service (calls `systemctl
    /// enable <service>`).\nWhen enabled, the service will start automatically
    /// at boot.",
    ///      "default": true,
    ///      "type": [
    ///        "boolean",
    ///        "null"
    ///      ]
    ///    },
    ///    "env": {
    ///      "description": "Environment variables.",
    ///      "type": [
    ///        "object",
    ///        "null"
    ///      ],
    ///      "additionalProperties": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "exec": {
    ///      "description": "Executable to run (can specify multiple commands
    /// that run sequentially).",
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "group": {
    ///      "description": "Linux group to run the service in.",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "mode": {
    ///      "$ref": "#/components/schemas/SystemdUnitMode"
    ///    },
    ///    "name": {
    ///      "description": "Unique slug; becomes unit name
    /// \"<name>.service\".",
    ///      "type": "string"
    ///    },
    ///    "onFailure": {
    ///      "description": "Units to activate when this unit enters a failed
    /// state. This is useful\nfor triggering recovery actions, notifications,
    /// or cleanup services when\nthe current service fails.",
    ///      "type": [
    ///        "array",
    ///        "null"
    ///      ],
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "readySignal": {
    ///      "description": "Use sd_notify; maps to Type=notify.",
    ///      "default": false,
    ///      "type": [
    ///        "boolean",
    ///        "null"
    ///      ]
    ///    },
    ///    "remainAfterExit": {
    ///      "description": "For oneshot: remain active after exit (default:
    /// true).\nWhen false, the service can be started again even if it already
    /// ran.",
    ///      "default": true,
    ///      "type": [
    ///        "boolean",
    ///        "null"
    ///      ]
    ///    },
    ///    "requires": {
    ///      "description": "Establishes a strong dependency. If the required
    /// unit fails to\nstart or stops unexpectedly, the current unit will also
    /// be stopped. This\nensures that a service critical to the functioning of
    /// the current unit\nis running and stable. Units listed in Requires= are
    /// activated along\nwith the current unit. If the required unit is not
    /// active, systemd will\nattempt to start it. This directive signifies a
    /// tight coupling between\nservices, where the current service cannot
    /// function without the required\nservice.",
    ///      "type": [
    ///        "array",
    ///        "null"
    ///      ],
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "restartPolicy": {
    ///      "$ref": "#/components/schemas/SystemdRestartPolicy"
    ///    },
    ///    "timeoutSec": {
    ///      "description": "Overall start/stop timeout.",
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ],
    ///      "format": "int64",
    ///      "minimum": 0.0
    ///    },
    ///    "user": {
    ///      "description": "Linux user to run the service as.",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "wantedBy": {
    ///      "description": "Target used when enabling (default:
    /// multi-user.target).",
    ///      "default": [
    ///        "multi-user.target"
    ///      ],
    ///      "type": [
    ///        "array",
    ///        "null"
    ///      ],
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "watchdogSec": {
    ///      "description": "Enable systemd watchdog (seconds).",
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ],
    ///      "format": "int64",
    ///      "minimum": 0.0
    ///    },
    ///    "workdir": {
    ///      "description": "Working directory.",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct SystemdUnitSpec {
        ///Establishes an ordering dependency. The current unit will start only
        ///after the units listed in After= have started. This is useful for
        ///ensuring that certain services are up and running before the current
        ///service begins its operation.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub after: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        ///For oneshot: remove unit on success.
        #[serde(
            rename = "deleteAfterSuccess",
            default = "defaults::systemd_unit_spec_delete_after_success"
        )]
        pub delete_after_success: ::std::option::Option<bool>,
        ///Whether to enable this service (calls `systemctl enable <service>`).
        ///When enabled, the service will start automatically at boot.
        #[serde(default = "defaults::systemd_unit_spec_enable")]
        pub enable: ::std::option::Option<bool>,
        ///Environment variables.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub env: ::std::option::Option<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        >,
        ///Executable to run (can specify multiple commands that run
        /// sequentially).
        pub exec: ::std::vec::Vec<::std::string::String>,
        ///Linux group to run the service in.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub group: ::std::option::Option<::std::string::String>,
        pub mode: SystemdUnitMode,
        ///Unique slug; becomes unit name "<name>.service".
        pub name: ::std::string::String,
        ///Units to activate when this unit enters a failed state. This is
        /// useful for triggering recovery actions, notifications, or
        /// cleanup services when the current service fails.
        #[serde(
            rename = "onFailure",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub on_failure: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        ///Use sd_notify; maps to Type=notify.
        #[serde(
            rename = "readySignal",
            default = "defaults::systemd_unit_spec_ready_signal"
        )]
        pub ready_signal: ::std::option::Option<bool>,
        ///For oneshot: remain active after exit (default: true).
        ///When false, the service can be started again even if it already ran.
        #[serde(
            rename = "remainAfterExit",
            default = "defaults::systemd_unit_spec_remain_after_exit"
        )]
        pub remain_after_exit: ::std::option::Option<bool>,
        ///Establishes a strong dependency. If the required unit fails to
        ///start or stops unexpectedly, the current unit will also be stopped.
        /// This ensures that a service critical to the functioning of
        /// the current unit is running and stable. Units listed in
        /// Requires= are activated along with the current unit. If the
        /// required unit is not active, systemd will attempt to start
        /// it. This directive signifies a tight coupling between
        /// services, where the current service cannot function without the
        /// required service.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub requires: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[serde(
            rename = "restartPolicy",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub restart_policy: ::std::option::Option<SystemdRestartPolicy>,
        ///Overall start/stop timeout.
        #[serde(
            rename = "timeoutSec",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub timeout_sec: ::std::option::Option<i64>,
        ///Linux user to run the service as.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub user: ::std::option::Option<::std::string::String>,
        ///Target used when enabling (default: multi-user.target).
        #[serde(rename = "wantedBy", default = "defaults::systemd_unit_spec_wanted_by")]
        pub wanted_by: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        ///Enable systemd watchdog (seconds).
        #[serde(
            rename = "watchdogSec",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub watchdog_sec: ::std::option::Option<i64>,
        ///Working directory.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub workdir: ::std::option::Option<::std::string::String>,
    }

    ///`SystemdUnitSpecPatch`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "name"
    ///  ],
    ///  "properties": {
    ///    "after": {
    ///      "description": "Establishes an ordering dependency. The current
    /// unit will start only\nafter the units listed in After= have started.
    /// This is useful for\nensuring that certain services are up and running
    /// before the current\nservice begins its operation.",
    ///      "type": [
    ///        "array",
    ///        "null"
    ///      ],
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "deleteAfterSuccess": {
    ///      "description": "For oneshot: remove unit on success.",
    ///      "type": [
    ///        "boolean",
    ///        "null"
    ///      ]
    ///    },
    ///    "enable": {
    ///      "description": "Whether to enable this service (calls `systemctl
    /// enable <service>`).\nWhen enabled, the service will start automatically
    /// at boot.",
    ///      "type": [
    ///        "boolean",
    ///        "null"
    ///      ]
    ///    },
    ///    "env": {
    ///      "description": "Environment variables.",
    ///      "type": [
    ///        "object",
    ///        "null"
    ///      ],
    ///      "additionalProperties": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "exec": {
    ///      "description": "Executable to run (can specify multiple commands
    /// that run sequentially).",
    ///      "type": [
    ///        "array",
    ///        "null"
    ///      ],
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "group": {
    ///      "description": "Linux group to run the service in.",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "mode": {
    ///      "$ref": "#/components/schemas/SystemdUnitMode"
    ///    },
    ///    "name": {
    ///      "description": "Unique slug; becomes unit name
    /// \"<name>.service\".",
    ///      "type": "string"
    ///    },
    ///    "onFailure": {
    ///      "description": "Units to activate when this unit enters a failed
    /// state. This is useful\nfor triggering recovery actions, notifications,
    /// or cleanup services when\nthe current service fails.",
    ///      "type": [
    ///        "array",
    ///        "null"
    ///      ],
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "readySignal": {
    ///      "description": "Use sd_notify; maps to Type=notify.",
    ///      "type": [
    ///        "boolean",
    ///        "null"
    ///      ]
    ///    },
    ///    "requires": {
    ///      "description": "Establishes a strong dependency. If the required
    /// unit fails to\nstart or stops unexpectedly, the current unit will also
    /// be stopped. This\nensures that a service critical to the functioning of
    /// the current unit\nis running and stable. Units listed in Requires= are
    /// activated along\nwith the current unit. If the required unit is not
    /// active, systemd will\nattempt to start it. This directive signifies a
    /// tight coupling between\nservices, where the current service cannot
    /// function without the required\nservice.",
    ///      "type": [
    ///        "array",
    ///        "null"
    ///      ],
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "restartPolicy": {
    ///      "$ref": "#/components/schemas/SystemdRestartPolicy"
    ///    },
    ///    "timeoutSec": {
    ///      "description": "Overall start/stop timeout.",
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ],
    ///      "format": "int64",
    ///      "minimum": 0.0
    ///    },
    ///    "user": {
    ///      "description": "Linux user to run the service as.",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "wantedBy": {
    ///      "description": "Target used when enabling (default:
    /// multi-user.target).",
    ///      "type": [
    ///        "array",
    ///        "null"
    ///      ],
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "watchdogSec": {
    ///      "description": "Enable systemd watchdog (seconds).",
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ],
    ///      "format": "int64",
    ///      "minimum": 0.0
    ///    },
    ///    "workdir": {
    ///      "description": "Working directory.",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct SystemdUnitSpecPatch {
        ///Establishes an ordering dependency. The current unit will start only
        ///after the units listed in After= have started. This is useful for
        ///ensuring that certain services are up and running before the current
        ///service begins its operation.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub after: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        ///For oneshot: remove unit on success.
        #[serde(
            rename = "deleteAfterSuccess",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub delete_after_success: ::std::option::Option<bool>,
        ///Whether to enable this service (calls `systemctl enable <service>`).
        ///When enabled, the service will start automatically at boot.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub enable: ::std::option::Option<bool>,
        ///Environment variables.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub env: ::std::option::Option<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        >,
        ///Executable to run (can specify multiple commands that run
        /// sequentially).
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub exec: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        ///Linux group to run the service in.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub group: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub mode: ::std::option::Option<SystemdUnitMode>,
        ///Unique slug; becomes unit name "<name>.service".
        pub name: ::std::string::String,
        ///Units to activate when this unit enters a failed state. This is
        /// useful for triggering recovery actions, notifications, or
        /// cleanup services when the current service fails.
        #[serde(
            rename = "onFailure",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub on_failure: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        ///Use sd_notify; maps to Type=notify.
        #[serde(
            rename = "readySignal",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub ready_signal: ::std::option::Option<bool>,
        ///Establishes a strong dependency. If the required unit fails to
        ///start or stops unexpectedly, the current unit will also be stopped.
        /// This ensures that a service critical to the functioning of
        /// the current unit is running and stable. Units listed in
        /// Requires= are activated along with the current unit. If the
        /// required unit is not active, systemd will attempt to start
        /// it. This directive signifies a tight coupling between
        /// services, where the current service cannot function without the
        /// required service.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub requires: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[serde(
            rename = "restartPolicy",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub restart_policy: ::std::option::Option<SystemdRestartPolicy>,
        ///Overall start/stop timeout.
        #[serde(
            rename = "timeoutSec",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub timeout_sec: ::std::option::Option<i64>,
        ///Linux user to run the service as.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub user: ::std::option::Option<::std::string::String>,
        ///Target used when enabling (default: multi-user.target).
        #[serde(
            rename = "wantedBy",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub wanted_by: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        ///Enable systemd watchdog (seconds).
        #[serde(
            rename = "watchdogSec",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub watchdog_sec: ::std::option::Option<i64>,
        ///Working directory.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub workdir: ::std::option::Option<::std::string::String>,
    }

    ///`TagDetails`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "name",
    ///    "target"
    ///  ],
    ///  "properties": {
    ///    "message": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "target": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct TagDetails {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub message: ::std::option::Option<::std::string::String>,
        pub name: ::std::string::String,
        pub target: ::std::string::String,
    }

    ///Tag object
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Tag object",
    ///  "type": "object",
    ///  "required": [
    ///    "name",
    ///    "sha",
    ///    "target"
    ///  ],
    ///  "properties": {
    ///    "message": {
    ///      "description": "The tag message",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "name": {
    ///      "description": "The tag name",
    ///      "type": "string"
    ///    },
    ///    "sha": {
    ///      "description": "The tag's hash ID",
    ///      "type": "string"
    ///    },
    ///    "tagger": {
    ///      "$ref": "#/components/schemas/Signature"
    ///    },
    ///    "target": {
    ///      "$ref": "#/components/schemas/TagTarget"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct TagObject {
        ///The tag message
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub message: ::std::option::Option<::std::string::String>,
        ///The tag name
        pub name: ::std::string::String,
        ///The tag's hash ID
        pub sha: ::std::string::String,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub tagger: ::std::option::Option<Signature>,
        pub target: TagTarget,
    }

    ///`TagTarget`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "sha"
    ///  ],
    ///  "properties": {
    ///    "sha": {
    ///      "description": "The target object's hash ID",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct TagTarget {
        ///The target object's hash ID
        pub sha: ::std::string::String,
    }

    ///`TerminalListResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "terminals"
    ///  ],
    ///  "properties": {
    ///    "terminals": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/TerminalSession"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct TerminalListResponse {
        pub terminals: ::std::vec::Vec<TerminalSession>,
    }

    ///`TerminalLogsArrayResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "logs"
    ///  ],
    ///  "properties": {
    ///    "logs": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/LogEntry"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct TerminalLogsArrayResponse {
        pub logs: ::std::vec::Vec<LogEntry>,
    }

    ///`TerminalLogsResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "output"
    ///  ],
    ///  "properties": {
    ///    "output": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct TerminalLogsResponse {
        pub output: ::std::string::String,
    }

    ///`TerminalSession`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "name",
    ///    "status"
    ///  ],
    ///  "properties": {
    ///    "created": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "status": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct TerminalSession {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub created: ::std::option::Option<::std::string::String>,
        pub name: ::std::string::String,
        pub status: ::std::string::String,
    }

    ///`TreeEntry`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "title": "Blob",
    ///      "type": "object",
    ///      "required": [
    ///        "path",
    ///        "sha",
    ///        "size",
    ///        "type"
    ///      ],
    ///      "properties": {
    ///        "path": {
    ///          "type": "string"
    ///        },
    ///        "sha": {
    ///          "type": "string"
    ///        },
    ///        "size": {
    ///          "type": "integer",
    ///          "format": "int64",
    ///          "minimum": 0.0
    ///        },
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "blob"
    ///          ]
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "title": "Tree",
    ///      "type": "object",
    ///      "required": [
    ///        "path",
    ///        "sha",
    ///        "type"
    ///      ],
    ///      "properties": {
    ///        "path": {
    ///          "type": "string"
    ///        },
    ///        "sha": {
    ///          "type": "string"
    ///        },
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "tree"
    ///          ]
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(tag = "type")]
    pub enum TreeEntry {
        ///Blob
        #[serde(rename = "blob")]
        Blob {
            path: ::std::string::String,
            sha: ::std::string::String,
            size: i64,
        },
        ///Tree
        #[serde(rename = "tree")]
        Tree {
            path: ::std::string::String,
            sha: ::std::string::String,
        },
    }

    ///Tree object
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Tree object",
    ///  "type": "object",
    ///  "required": [
    ///    "sha",
    ///    "tree"
    ///  ],
    ///  "properties": {
    ///    "sha": {
    ///      "description": "The tree's hash ID",
    ///      "type": "string"
    ///    },
    ///    "tree": {
    ///      "description": "The tree's entries",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/TreeEntry"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct TreeObject {
        ///The tree's hash ID
        pub sha: ::std::string::String,
        ///The tree's entries
        pub tree: ::std::vec::Vec<TreeEntry>,
    }

    ///`UpdateAllowedUsersRequestBody`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "allowedUsers": {
    ///      "description": "List of allowed Linux users. If null, identity can
    /// SSH as any user.\nIf specified, identity can only SSH as users in this
    /// list.",
    ///      "type": [
    ///        "array",
    ///        "null"
    ///      ],
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct UpdateAllowedUsersRequestBody {
        ///List of allowed Linux users. If null, identity can SSH as any user.
        ///If specified, identity can only SSH as users in this list.
        #[serde(
            rename = "allowedUsers",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub allowed_users: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }

    impl ::std::default::Default for UpdateAllowedUsersRequestBody {
        fn default() -> Self {
            Self {
                allowed_users: Default::default(),
            }
        }
    }

    ///`UpdateGitPermissionRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "permission"
    ///  ],
    ///  "properties": {
    ///    "permission": {
    ///      "$ref": "#/components/schemas/AccessLevel"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct UpdateGitPermissionRequest {
        pub permission: AccessLevel,
    }

    ///`UpdateScheduleRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "active": {
    ///      "type": [
    ///        "boolean",
    ///        "null"
    ///      ]
    ///    },
    ///    "cron": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "deploymentId": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ],
    ///      "format": "uuid"
    ///    },
    ///    "path": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "payload": {},
    ///    "timezone": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct UpdateScheduleRequest {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub active: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub cron: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "deploymentId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub deployment_id: ::std::option::Option<::uuid::Uuid>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub path: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub payload: ::std::option::Option<::serde_json::Value>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub timezone: ::std::option::Option<::std::string::String>,
    }

    impl ::std::default::Default for UpdateScheduleRequest {
        fn default() -> Self {
            Self {
                active: Default::default(),
                cron: Default::default(),
                deployment_id: Default::default(),
                path: Default::default(),
                payload: Default::default(),
                timezone: Default::default(),
            }
        }
    }

    ///`UpdateScheduleResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "success"
    ///  ],
    ///  "properties": {
    ///    "success": {
    ///      "type": "boolean"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct UpdateScheduleResponse {
        pub success: bool,
    }

    ///`UpdateSnapshotRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "name": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct UpdateSnapshotRequest {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
    }

    impl ::std::default::Default for UpdateSnapshotRequest {
        fn default() -> Self {
            Self {
                name: Default::default(),
            }
        }
    }

    ///`UpdateSnapshotResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "error",
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "error": {
    ///      "description": "Error code in SCREAMING_SNAKE_CASE",
    ///      "type": "string"
    ///    },
    ///    "message": {
    ///      "description": "Human-readable error message",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct UpdateSnapshotResponse {
        ///Error code in SCREAMING_SNAKE_CASE
        pub error: ::std::string::String,
        ///Human-readable error message
        pub message: ::std::string::String,
    }

    ///`UpdateSnapshotResponses`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "snapshot_id"
    ///  ],
    ///  "properties": {
    ///    "name": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "snapshot_id": {
    ///      "$ref": "#/components/schemas/SnapshotId"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct UpdateSnapshotResponses {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
        pub snapshot_id: SnapshotId,
    }

    ///`VerifyDomainSuccess`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "domain"
    ///  ],
    ///  "properties": {
    ///    "domain": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct VerifyDomainSuccess {
        pub domain: ::std::string::String,
    }

    ///`Visibility`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "public",
    ///    "private"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum Visibility {
        #[serde(rename = "public")]
        Public,
        #[serde(rename = "private")]
        Private,
    }

    impl ::std::fmt::Display for Visibility {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Public => f.write_str("public"),
                Self::Private => f.write_str("private"),
            }
        }
    }

    impl ::std::str::FromStr for Visibility {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "public" => Ok(Self::Public),
                "private" => Ok(Self::Private),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for Visibility {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for Visibility {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for Visibility {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`VmDeleteEvent`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "OnStop",
    ///    "OnSuspend"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum VmDeleteEvent {
        OnStop,
        OnSuspend,
    }

    impl ::std::fmt::Display for VmDeleteEvent {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::OnStop => f.write_str("OnStop"),
                Self::OnSuspend => f.write_str("OnSuspend"),
            }
        }
    }

    impl ::std::str::FromStr for VmDeleteEvent {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "OnStop" => Ok(Self::OnStop),
                "OnSuspend" => Ok(Self::OnSuspend),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for VmDeleteEvent {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for VmDeleteEvent {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for VmDeleteEvent {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///Domain configuration for a VM, specifying the domain and optional target
    /// port
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Domain configuration for a VM, specifying the domain
    /// and optional target port",
    ///  "type": "object",
    ///  "required": [
    ///    "domain"
    ///  ],
    ///  "properties": {
    ///    "domain": {
    ///      "description": "The domain name to map to the VM",
    ///      "examples": [
    ///        "myapp.example.com"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "vmPort": {
    ///      "description": "The port on the VM to route traffic to. Defaults to
    /// 443 if not specified.",
    ///      "examples": [
    ///        3000
    ///      ],
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ],
    ///      "format": "int32"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct VmDomainConfig {
        ///The domain name to map to the VM
        pub domain: ::std::string::String,
        ///The port on the VM to route traffic to. Defaults to 443 if not
        /// specified.
        #[serde(
            rename = "vmPort",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub vm_port: ::std::option::Option<i32>,
    }

    ///VM ID — always 20 alphanumeric lowercase characters.
    ///New IDs are fully random. Legacy short IDs are right-padded with '0' on
    /// parse.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "VM ID — always 20 alphanumeric lowercase
    /// characters.\nNew IDs are fully random. Legacy short IDs are right-padded
    /// with '0' on parse.",
    ///  "type": "string"
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    #[serde(transparent)]
    pub struct VmId(pub ::std::string::String);
    impl ::std::ops::Deref for VmId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<VmId> for ::std::string::String {
        fn from(value: VmId) -> Self {
            value.0
        }
    }

    impl ::std::convert::From<::std::string::String> for VmId {
        fn from(value: ::std::string::String) -> Self {
            Self(value)
        }
    }

    impl ::std::str::FromStr for VmId {
        type Err = ::std::convert::Infallible;
        fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::fmt::Display for VmId {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            self.0.fmt(f)
        }
    }

    ///`VmInfo`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "id",
    ///    "state"
    ///  ],
    ///  "properties": {
    ///    "cpuTimeSeconds": {
    ///      "type": [
    ///        "number",
    ///        "null"
    ///      ],
    ///      "format": "double"
    ///    },
    ///    "createdAt": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ],
    ///      "format": "date-time"
    ///    },
    ///    "createdDeclaratively": {
    ///      "type": "boolean"
    ///    },
    ///    "deleted": {
    ///      "type": "boolean"
    ///    },
    ///    "id": {
    ///      "$ref": "#/components/schemas/VmId"
    ///    },
    ///    "lastNetworkActivity": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ],
    ///      "format": "date-time"
    ///    },
    ///    "metrics": {
    ///      "$ref": "#/components/schemas/VmMetricsInfo"
    ///    },
    ///    "persistence": {
    ///      "$ref": "#/components/schemas/VmPersistence"
    ///    },
    ///    "snapshotId": {
    ///      "$ref": "#/components/schemas/SnapshotId"
    ///    },
    ///    "state": {
    ///      "$ref": "#/components/schemas/VMState"
    ///    },
    ///    "vmInstanceId": {
    ///      "$ref": "#/components/schemas/VmInstanceId"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct VmInfo {
        #[serde(
            rename = "cpuTimeSeconds",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub cpu_time_seconds: ::std::option::Option<f64>,
        #[serde(
            rename = "createdAt",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub created_at: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        #[serde(
            rename = "createdDeclaratively",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub created_declaratively: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub deleted: ::std::option::Option<bool>,
        pub id: VmId,
        #[serde(
            rename = "lastNetworkActivity",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub last_network_activity: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub metrics: ::std::option::Option<VmMetricsInfo>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub persistence: ::std::option::Option<VmPersistence>,
        #[serde(
            rename = "snapshotId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub snapshot_id: ::std::option::Option<SnapshotId>,
        pub state: VmState,
        #[serde(
            rename = "vmInstanceId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub vm_instance_id: ::std::option::Option<VmInstanceId>,
    }

    ///`VmInstanceId`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string"
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    #[serde(transparent)]
    pub struct VmInstanceId(pub ::std::string::String);
    impl ::std::ops::Deref for VmInstanceId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<VmInstanceId> for ::std::string::String {
        fn from(value: VmInstanceId) -> Self {
            value.0
        }
    }

    impl ::std::convert::From<::std::string::String> for VmInstanceId {
        fn from(value: ::std::string::String) -> Self {
            Self(value)
        }
    }

    impl ::std::str::FromStr for VmInstanceId {
        type Err = ::std::convert::Infallible;
        fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::fmt::Display for VmInstanceId {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            self.0.fmt(f)
        }
    }

    ///`VmMetricsInfo`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "billingDay",
    ///    "isActiveToday",
    ///    "wallTimeSeconds"
    ///  ],
    ///  "properties": {
    ///    "billingDay": {
    ///      "type": "string"
    ///    },
    ///    "isActiveToday": {
    ///      "type": "boolean"
    ///    },
    ///    "wallTimeSeconds": {
    ///      "type": "integer",
    ///      "format": "int64",
    ///      "minimum": 0.0
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct VmMetricsInfo {
        #[serde(rename = "billingDay")]
        pub billing_day: ::std::string::String,
        #[serde(rename = "isActiveToday")]
        pub is_active_today: bool,
        #[serde(rename = "wallTimeSeconds")]
        pub wall_time_seconds: i64,
    }

    ///Full VM permission record
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Full VM permission record",
    ///  "type": "object",
    ///  "required": [
    ///    "grantedAt",
    ///    "grantedBy",
    ///    "id",
    ///    "identityId",
    ///    "vmId"
    ///  ],
    ///  "properties": {
    ///    "allowedUsers": {
    ///      "type": [
    ///        "array",
    ///        "null"
    ///      ],
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "grantedAt": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "grantedBy": {
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "id": {
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "identityId": {
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "vmId": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct VmPermission {
        #[serde(
            rename = "allowedUsers",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub allowed_users: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[serde(rename = "grantedAt")]
        pub granted_at: ::chrono::DateTime<::chrono::offset::Utc>,
        #[serde(rename = "grantedBy")]
        pub granted_by: ::uuid::Uuid,
        pub id: ::uuid::Uuid,
        #[serde(rename = "identityId")]
        pub identity_id: ::uuid::Uuid,
        #[serde(rename = "vmId")]
        pub vm_id: ::std::string::String,
    }

    ///`VmPersistence`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "description": "When your storage quota is reached, the least
    /// recently used VMs will be\ndeleted.",
    ///      "type": "object",
    ///      "required": [
    ///        "type"
    ///      ],
    ///      "properties": {
    ///        "priority": {
    ///          "description": "Priority for eviction when storage quota is
    /// reached. Higher values\nmean the VM is less likely to be evicted. Range
    /// is 0-10, default is\n5.",
    ///          "default": 5,
    ///          "type": [
    ///            "integer",
    ///            "null"
    ///          ],
    ///          "format": "int32",
    ///          "maximum": 10.0,
    ///          "minimum": 0.0
    ///        },
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "sticky"
    ///          ]
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "description": "The VM will be deleted after the idle timeout. It's
    /// not guaranteed that\nthe VM will be deleted immediately.",
    ///      "type": "object",
    ///      "required": [
    ///        "type"
    ///      ],
    ///      "properties": {
    ///        "deleteEvent": {
    ///          "$ref": "#/components/schemas/VmDeleteEvent"
    ///        },
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "ephemeral"
    ///          ]
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "type"
    ///      ],
    ///      "properties": {
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "persistent"
    ///          ]
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(tag = "type")]
    pub enum VmPersistence {
        ///When your storage quota is reached, the least recently used VMs will
        /// be deleted.
        #[serde(rename = "sticky")]
        Sticky {
            ///Priority for eviction when storage quota is reached. Higher
            /// values mean the VM is less likely to be evicted.
            /// Range is 0-10, default is 5.
            #[serde(default = "defaults::vm_persistence_sticky_priority")]
            priority: ::std::option::Option<i32>,
        },
        ///The VM will be deleted after the idle timeout. It's not guaranteed
        /// that the VM will be deleted immediately.
        #[serde(rename = "ephemeral")]
        Ephemeral {
            #[serde(
                rename = "deleteEvent",
                default,
                skip_serializing_if = "::std::option::Option::is_none"
            )]
            delete_event: ::std::option::Option<VmDeleteEvent>,
        },
        #[serde(rename = "persistent")]
        Persistent,
    }

    ///`VmState`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "starting",
    ///    "running",
    ///    "suspending",
    ///    "suspended",
    ///    "stopped"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum VmState {
        #[serde(rename = "starting")]
        Starting,
        #[serde(rename = "running")]
        Running,
        #[serde(rename = "suspending")]
        Suspending,
        #[serde(rename = "suspended")]
        Suspended,
        #[serde(rename = "stopped")]
        Stopped,
    }

    impl ::std::fmt::Display for VmState {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Starting => f.write_str("starting"),
                Self::Running => f.write_str("running"),
                Self::Suspending => f.write_str("suspending"),
                Self::Suspended => f.write_str("suspended"),
                Self::Stopped => f.write_str("stopped"),
            }
        }
    }

    impl ::std::str::FromStr for VmState {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "starting" => Ok(Self::Starting),
                "running" => Ok(Self::Running),
                "suspending" => Ok(Self::Suspending),
                "suspended" => Ok(Self::Suspended),
                "stopped" => Ok(Self::Stopped),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for VmState {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for VmState {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for VmState {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`VmTemplate`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "additionalFiles": {
    ///      "type": [
    ///        "object",
    ///        "null"
    ///      ],
    ///      "additionalProperties": {
    ///        "$ref": "#/components/schemas/FreestyleFile"
    ///      }
    ///    },
    ///    "aptDeps": {
    ///      "description": "Optional list of apt packages to install when
    /// setting up the VM.\nThese packages will be installed using `apt-get
    /// install` on VM startup.",
    ///      "examples": [
    ///        [
    ///          "git",
    ///          "curl",
    ///          "vim"
    ///        ]
    ///      ],
    ///      "type": [
    ///        "array",
    ///        "null"
    ///      ],
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "baseImage": {
    ///      "$ref": "#/components/schemas/BaseImageSpec"
    ///    },
    ///    "discriminator": {
    ///      "description": "Optional discriminator to differentiate snapshots
    /// with otherwise identical configurations",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "git": {
    ///      "$ref": "#/components/schemas/GitOptions"
    ///    },
    ///    "gitRepos": {
    ///      "deprecated": true,
    ///      "type": [
    ///        "array",
    ///        "null"
    ///      ],
    ///      "items": {
    ///        "$ref": "#/components/schemas/GitRepositorySpec"
    ///      }
    ///    },
    ///    "groups": {
    ///      "description": "Linux groups to create on VM startup",
    ///      "type": [
    ///        "array",
    ///        "null"
    ///      ],
    ///      "items": {
    ///        "$ref": "#/components/schemas/LinuxGroupSpec"
    ///      }
    ///    },
    ///    "idleTimeoutSeconds": {
    ///      "description": "Idle timeout in seconds. If set, the VM will be
    /// automatically suspended\nafter this many seconds of no network activity.
    /// Defaults to 300 seconds\n(5 minutes) if not provided or the last used
    /// timeout for the forked VM.",
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ],
    ///      "format": "int64",
    ///      "minimum": 0.0
    ///    },
    ///    "memSizeGb": {
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ],
    ///      "format": "int64",
    ///      "minimum": 0.0
    ///    },
    ///    "persistence": {
    ///      "$ref": "#/components/schemas/VmPersistence"
    ///    },
    ///    "ports": {
    ///      "description": "Optional list of ports to expose externally. If not
    /// provided, port 3000\nwill be exposed on port 443 by default. Pass an
    /// empty array to disable\nexternal ports. Only ports 8081 and 443 can be
    /// configured externally for\nnow. Any target port is allowed.",
    ///      "examples": [
    ///        [
    ///          {
    ///            "port": 443,
    ///            "targetPort": 3000
    ///          }
    ///        ]
    ///      ],
    ///      "type": [
    ///        "array",
    ///        "null"
    ///      ],
    ///      "items": {
    ///        "type": "object",
    ///        "required": [
    ///          "port",
    ///          "targetPort"
    ///        ],
    ///        "properties": {
    ///          "port": {
    ///            "type": "integer",
    ///            "format": "int32",
    ///            "minimum": 0.0
    ///          },
    ///          "targetPort": {
    ///            "type": "integer",
    ///            "format": "int32",
    ///            "minimum": 0.0
    ///          }
    ///        }
    ///      }
    ///    },
    ///    "readySignalTimeoutSeconds": {
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ],
    ///      "format": "int64",
    ///      "minimum": 0.0
    ///    },
    ///    "rootfsSizeGb": {
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ],
    ///      "format": "int64",
    ///      "minimum": 0.0
    ///    },
    ///    "skipCache": {
    ///      "description": "When true, bypasses the snapshot cache and always
    /// creates a new snapshot.\nThe new snapshot still stores the template
    /// hash, so it becomes the updated cache entry\nfor future requests that do
    /// not set skip_cache.",
    ///      "type": [
    ///        "boolean",
    ///        "null"
    ///      ]
    ///    },
    ///    "snapshotId": {
    ///      "$ref": "#/components/schemas/SnapshotId"
    ///    },
    ///    "systemd": {
    ///      "$ref": "#/components/schemas/SystemdConfig"
    ///    },
    ///    "users": {
    ///      "description": "Linux users to create on VM startup",
    ///      "type": [
    ///        "array",
    ///        "null"
    ///      ],
    ///      "items": {
    ///        "$ref": "#/components/schemas/LinuxUserSpec"
    ///      }
    ///    },
    ///    "vcpuCount": {
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ],
    ///      "format": "int32",
    ///      "minimum": 0.0
    ///    },
    ///    "waitForReadySignal": {
    ///      "type": [
    ///        "boolean",
    ///        "null"
    ///      ]
    ///    },
    ///    "workdir": {
    ///      "description": "Optional working directory for the VM. If not
    /// provided, the default to '/'",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct VmTemplate {
        #[serde(
            rename = "additionalFiles",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub additional_files: ::std::option::Option<
            ::std::collections::HashMap<::std::string::String, FreestyleFile>,
        >,
        ///Optional list of apt packages to install when setting up the VM.
        ///These packages will be installed using `apt-get install` on VM
        /// startup.
        #[serde(
            rename = "aptDeps",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub apt_deps: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[serde(
            rename = "baseImage",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub base_image: ::std::option::Option<BaseImageSpec>,
        ///Optional discriminator to differentiate snapshots with otherwise
        /// identical configurations
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub discriminator: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub git: ::std::option::Option<GitOptions>,
        #[serde(
            rename = "gitRepos",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub git_repos: ::std::option::Option<::std::vec::Vec<GitRepositorySpec>>,
        ///Linux groups to create on VM startup
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub groups: ::std::option::Option<::std::vec::Vec<LinuxGroupSpec>>,
        ///Idle timeout in seconds. If set, the VM will be automatically
        /// suspended after this many seconds of no network activity.
        /// Defaults to 300 seconds (5 minutes) if not provided or the
        /// last used timeout for the forked VM.
        #[serde(
            rename = "idleTimeoutSeconds",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub idle_timeout_seconds: ::std::option::Option<i64>,
        #[serde(
            rename = "memSizeGb",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub mem_size_gb: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub persistence: ::std::option::Option<VmPersistence>,
        ///Optional list of ports to expose externally. If not provided, port
        /// 3000 will be exposed on port 443 by default. Pass an empty
        /// array to disable external ports. Only ports 8081 and 443 can
        /// be configured externally for now. Any target port is
        /// allowed.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub ports: ::std::option::Option<::std::vec::Vec<VmTemplatePortsItem>>,
        #[serde(
            rename = "readySignalTimeoutSeconds",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub ready_signal_timeout_seconds: ::std::option::Option<i64>,
        #[serde(
            rename = "rootfsSizeGb",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub rootfs_size_gb: ::std::option::Option<i64>,
        ///When true, bypasses the snapshot cache and always creates a new
        /// snapshot. The new snapshot still stores the template hash,
        /// so it becomes the updated cache entry for future requests
        /// that do not set skip_cache.
        #[serde(
            rename = "skipCache",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub skip_cache: ::std::option::Option<bool>,
        #[serde(
            rename = "snapshotId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub snapshot_id: ::std::option::Option<SnapshotId>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub systemd: ::std::option::Option<SystemdConfig>,
        ///Linux users to create on VM startup
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub users: ::std::option::Option<::std::vec::Vec<LinuxUserSpec>>,
        #[serde(
            rename = "vcpuCount",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub vcpu_count: ::std::option::Option<i32>,
        #[serde(
            rename = "waitForReadySignal",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub wait_for_ready_signal: ::std::option::Option<bool>,
        ///Optional working directory for the VM. If not provided, the default
        /// to '/'
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub workdir: ::std::option::Option<::std::string::String>,
    }

    impl ::std::default::Default for VmTemplate {
        fn default() -> Self {
            Self {
                additional_files: Default::default(),
                apt_deps: Default::default(),
                base_image: Default::default(),
                discriminator: Default::default(),
                git: Default::default(),
                git_repos: Default::default(),
                groups: Default::default(),
                idle_timeout_seconds: Default::default(),
                mem_size_gb: Default::default(),
                persistence: Default::default(),
                ports: Default::default(),
                ready_signal_timeout_seconds: Default::default(),
                rootfs_size_gb: Default::default(),
                skip_cache: Default::default(),
                snapshot_id: Default::default(),
                systemd: Default::default(),
                users: Default::default(),
                vcpu_count: Default::default(),
                wait_for_ready_signal: Default::default(),
                workdir: Default::default(),
            }
        }
    }

    ///`VmTemplatePortsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "port",
    ///    "targetPort"
    ///  ],
    ///  "properties": {
    ///    "port": {
    ///      "type": "integer",
    ///      "format": "int32",
    ///      "minimum": 0.0
    ///    },
    ///    "targetPort": {
    ///      "type": "integer",
    ///      "format": "int32",
    ///      "minimum": 0.0
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct VmTemplatePortsItem {
        pub port: i32,
        #[serde(rename = "targetPort")]
        pub target_port: i32,
    }

    ///`WaitVmResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "exitStatus",
    ///    "id"
    ///  ],
    ///  "properties": {
    ///    "exitStatus": {
    ///      "type": "string"
    ///    },
    ///    "id": {
    ///      "$ref": "#/components/schemas/VmId"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct WaitVmResponse {
        #[serde(rename = "exitStatus")]
        pub exit_status: ::std::string::String,
        pub id: VmId,
    }

    ///`WhoAmIResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "accountId"
    ///  ],
    ///  "properties": {
    ///    "accountId": {
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "identityId": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ],
    ///      "format": "uuid"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct WhoAmIResponse {
        #[serde(rename = "accountId")]
        pub account_id: ::uuid::Uuid,
        #[serde(
            rename = "identityId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub identity_id: ::std::option::Option<::uuid::Uuid>,
    }

    ///`WriteFileRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "content"
    ///  ],
    ///  "properties": {
    ///    "content": {
    ///      "type": "string"
    ///    },
    ///    "encoding": {
    ///      "$ref": "#/components/schemas/FileEncoding"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct WriteFileRequest {
        pub content: ::std::string::String,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub encoding: ::std::option::Option<FileEncoding>,
    }

    ///`WriteFileResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object"
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(transparent)]
    pub struct WriteFileResponse(pub ::serde_json::Map<::std::string::String, ::serde_json::Value>);
    impl ::std::ops::Deref for WriteFileResponse {
        type Target = ::serde_json::Map<::std::string::String, ::serde_json::Value>;
        fn deref(&self) -> &::serde_json::Map<::std::string::String, ::serde_json::Value> {
            &self.0
        }
    }

    impl ::std::convert::From<WriteFileResponse>
        for ::serde_json::Map<::std::string::String, ::serde_json::Value>
    {
        fn from(value: WriteFileResponse) -> Self {
            value.0
        }
    }

    impl ::std::convert::From<::serde_json::Map<::std::string::String, ::serde_json::Value>>
        for WriteFileResponse
    {
        fn from(value: ::serde_json::Map<::std::string::String, ::serde_json::Value>) -> Self {
            Self(value)
        }
    }

    /// Generation of default values for serde.
    pub mod defaults {
        pub(super) fn default_bool<const V: bool>() -> bool {
            V
        }

        pub(super) fn create_vm_request_recreate() -> ::std::option::Option<bool> {
            ::std::option::Option::Some(false)
        }

        pub(super) fn create_vm_request_wait_for_ready_signal() -> ::std::option::Option<bool> {
            ::std::option::Option::Some(true)
        }

        pub(super) fn linux_group_spec_system() -> ::std::option::Option<bool> {
            ::std::option::Option::Some(false)
        }

        pub(super) fn linux_user_spec_system() -> ::std::option::Option<bool> {
            ::std::option::Option::Some(false)
        }

        pub(super) fn snapshot_persistence_sticky_priority() -> ::std::option::Option<i32> {
            ::std::option::Option::Some(5_i32)
        }

        pub(super) fn systemd_unit_spec_delete_after_success() -> ::std::option::Option<bool> {
            ::std::option::Option::Some(false)
        }

        pub(super) fn systemd_unit_spec_enable() -> ::std::option::Option<bool> {
            ::std::option::Option::Some(true)
        }

        pub(super) fn systemd_unit_spec_ready_signal() -> ::std::option::Option<bool> {
            ::std::option::Option::Some(false)
        }

        pub(super) fn systemd_unit_spec_remain_after_exit() -> ::std::option::Option<bool> {
            ::std::option::Option::Some(true)
        }

        pub(super) fn systemd_unit_spec_wanted_by(
        ) -> ::std::option::Option<::std::vec::Vec<::std::string::String>> {
            ::std::option::Option::Some(vec!["multi-user.target".to_string()])
        }

        pub(super) fn vm_persistence_sticky_priority() -> ::std::option::Option<i32> {
            ::std::option::Option::Some(5_i32)
        }
    }
}

#[derive(Clone, Debug)]
///Client for Freestyle Sandboxes
///
///
///Freestyle Sandboxes lets you deploy your users or AIs code. **Get your API Key at [admin.freestyle.sh](https://admin.freestyle.sh)**
///
///
///Version: 0.1.0
pub struct Client {
    pub(crate) baseurl: String,
    pub(crate) client: reqwest::Client,
}

impl Client {
    /// Create a new client.
    ///
    /// `baseurl` is the base URL provided to the internal
    /// `reqwest::Client`, and should include a scheme and hostname,
    /// as well as port and a path stem if applicable.
    pub fn new(baseurl: &str) -> Self {
        #[cfg(not(target_arch = "wasm32"))]
        let client = {
            let dur = ::std::time::Duration::from_secs(15u64);
            reqwest::ClientBuilder::new()
                .connect_timeout(dur)
                .timeout(dur)
        };
        #[cfg(target_arch = "wasm32")]
        let client = reqwest::ClientBuilder::new();
        Self::new_with_client(baseurl, client.build().unwrap())
    }

    /// Construct a new client with an existing `reqwest::Client`,
    /// allowing more control over its configuration.
    ///
    /// `baseurl` is the base URL provided to the internal
    /// `reqwest::Client`, and should include a scheme and hostname,
    /// as well as port and a path stem if applicable.
    pub fn new_with_client(baseurl: &str, client: reqwest::Client) -> Self {
        Self {
            baseurl: baseurl.to_string(),
            client,
        }
    }
}

impl ClientInfo<()> for Client {
    fn api_version() -> &'static str {
        "0.1.0"
    }

    fn baseurl(&self) -> &str {
        self.baseurl.as_str()
    }

    fn client(&self) -> &reqwest::Client {
        &self.client
    }

    fn inner(&self) -> &() {
        &()
    }
}

impl ClientHooks<()> for &Client {}
#[allow(clippy::all)]
impl Client {
    ///Get current user
    ///
    ///Returns information about the currently authenticated user or identity.
    ///
    ///Sends a `GET` request to `/auth/v1/whoami`
    pub async fn handle_whoami<'a>(
        &'a self,
    ) -> Result<ResponseValue<types::WhoAmIResponse>, Error<()>> {
        let url = format!("{}/auth/v1/whoami", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "handle_whoami",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Deploy Cloudstate Project
    ///
    ///Deploy a cloudstate project
    ///
    ///Sends a `POST` request to `/cloudstate/v1/deploy`
    pub async fn handle_deploy_cloudstate<'a>(
        &'a self,
        body: &'a types::FreestyleCloudstateDeployRequest,
    ) -> Result<
        ResponseValue<types::FreestyleCloudstateDeploySuccessResponse>,
        Error<types::FreestyleCloudstateDeployErrorResponse>,
    > {
        let url = format!("{}/cloudstate/v1/deploy", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "handle_deploy_cloudstate",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            500u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get Backup of Cloudstate Project
    ///
    ///Get a backup of a cloudstate project
    ///
    ///Sends a `GET` request to `/cloudstate/v1/projects/{id}/backup`
    pub async fn handle_backup_cloudstate<'a>(
        &'a self,
        id: &'a str,
    ) -> Result<ResponseValue<ByteStream>, Error<()>> {
        let url = format!(
            "{}/cloudstate/v1/projects/{}/backup",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.get(url).headers(header_map).build()?;
        let info = OperationInfo {
            operation_id: "handle_backup_cloudstate",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::stream(response)),
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///List DNS Records
    ///
    ///Sends a `GET` request to `/dns/v1/records`
    pub async fn handle_list_records<'a>(
        &'a self,
        domain: &'a str,
    ) -> Result<ResponseValue<types::ListRecordsResponse>, Error<types::HandleListRecordsResponse>>
    {
        let url = format!("{}/dns/v1/records", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("domain", &domain))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "handle_list_records",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Create DNS Record
    ///
    ///Sends a `POST` request to `/dns/v1/records`
    pub async fn handle_create_record<'a>(
        &'a self,
        body: &'a types::CreateRecordParams,
    ) -> Result<ResponseValue<types::CreateRecordResponse>, Error<types::HandleCreateRecordResponse>>
    {
        let url = format!("{}/dns/v1/records", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "handle_create_record",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            403u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Delete DNS Record
    ///
    ///Sends a `DELETE` request to `/dns/v1/records`
    pub async fn handle_delete_record<'a>(
        &'a self,
        domain: &'a str,
        record: &'a types::DnsRecord,
    ) -> Result<ResponseValue<types::DeleteRecordResponse>, Error<types::HandleDeleteRecordResponse>>
    {
        let url = format!("{}/dns/v1/records", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("domain", &domain))
            .query(&progenitor_client::QueryParam::new("record", &record))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "handle_delete_record",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            403u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Provision a wildcard certificate
    ///
    ///Provisions a wildcard certificate for a verified domain
    ///
    ///
    ///This speeds up deploys on all subdomains of the domain. In order to use
    /// it, you must add the following record to your DNS config:
    ///
    ///`_acme-challenge.yourdomain.com` NS `dns.freestyle.sh`
    ///
    ///Sends a `POST` request to `/domains/v1/certs/{domain}/wildcard`
    pub async fn handle_verify_wildcard<'a>(
        &'a self,
        domain: &'a str,
    ) -> Result<
        ResponseValue<types::HandleVerifyWildcardResponse>,
        Error<types::HandleVerifyWildcardResponse>,
    > {
        let url = format!(
            "{}/domains/v1/certs/{}/wildcard",
            self.baseurl,
            encode_path(&domain.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "handle_verify_wildcard",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///List domains for an account
    ///
    ///This lists the domains that an account has verified ownership of. This
    /// includes the *.style.dev domains the account has claimed.
    ///
    ///Sends a `GET` request to `/domains/v1/domains`
    pub async fn handle_list_domains<'a>(
        &'a self,
        implicitly_owned: Option<bool>,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<
        ResponseValue<::std::vec::Vec<types::HandleListDomainsResponseItem>>,
        Error<types::HandleListDomainsResponse>,
    > {
        let url = format!("{}/domains/v1/domains", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new(
                "implicitlyOwned",
                &implicitly_owned,
            ))
            .query(&progenitor_client::QueryParam::new("limit", &limit))
            .query(&progenitor_client::QueryParam::new("offset", &offset))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "handle_list_domains",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///List Domain Mappings
    ///
    ///List domain mappings for any query based on exact domain or domain
    /// ownership (the domain ownership that gave the right to use the domain)
    ///
    ///Sends a `GET` request to `/domains/v1/mappings`
    pub async fn handle_list_domain_mappings<'a>(
        &'a self,
        domain: Option<&'a str>,
        domain_ownership: Option<&'a ::uuid::Uuid>,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<ResponseValue<::std::vec::Vec<types::FreestyleSandboxDomainMapping>>, Error<()>>
    {
        let url = format!("{}/domains/v1/mappings", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("domain", &domain))
            .query(&progenitor_client::QueryParam::new(
                "domainOwnership",
                &domain_ownership,
            ))
            .query(&progenitor_client::QueryParam::new("limit", &limit))
            .query(&progenitor_client::QueryParam::new("offset", &offset))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "handle_list_domain_mappings",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            401u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Insert Domain Mapping
    ///
    ///This will unmap any other deployment to this domain. Provide either
    /// deployment_id or vm_id (with optional vm_port), but not both.
    ///
    ///Sends a `POST` request to `/domains/v1/mappings/{domain}`
    pub async fn handle_insert_domain_mapping<'a>(
        &'a self,
        domain: &'a str,
        body: &'a types::CreateDomainMappingRequest,
    ) -> Result<
        ResponseValue<types::CreateDomainMappingSuccess>,
        Error<types::PublicDomainMappingError>,
    > {
        let url = format!(
            "{}/domains/v1/mappings/{}",
            self.baseurl,
            encode_path(&domain.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "handle_insert_domain_mapping",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Remove Domain Mapping
    ///
    ///Sends a `DELETE` request to `/domains/v1/mappings/{domain}`
    pub async fn handle_delete_domain_mapping<'a>(
        &'a self,
        domain: &'a str,
    ) -> Result<
        ResponseValue<types::SuccessfullyDeletedDomainMapping>,
        Error<types::HandleDeleteDomainMappingResponse>,
    > {
        let url = format!(
            "{}/domains/v1/mappings/{}",
            self.baseurl,
            encode_path(&domain.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "handle_delete_domain_mapping",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            422u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            502u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///List domain verification requests for an account
    ///
    ///Lists domain verification requests for the current account.
    ///
    ///Sends a `GET` request to `/domains/v1/verifications`
    pub async fn handle_list_domain_verification_requests<'a>(
        &'a self,
    ) -> Result<
        ResponseValue<::std::vec::Vec<types::HandleListDomainVerificationRequestsResponseItem>>,
        Error<types::HandleListDomainVerificationRequestsResponse>,
    > {
        let url = format!("{}/domains/v1/verifications", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "handle_list_domain_verification_requests",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Verify a domain verification request
    ///
    ///This checks a pre-existing verification request for a domain. To create
    /// a verification request, call the [create domain
    /// verification](/#tag/domains/POST/domains/v1/verifications) endpoint.
    /// This endpoint will check if the domain has a TXT record with the
    /// verification code. If it does, the domain will be verified.
    ///
    ///Sends a `PUT` request to `/domains/v1/verifications`
    pub async fn handle_verify_domain<'a>(
        &'a self,
        body: &'a types::FreestyleVerifyDomainRequest,
    ) -> Result<ResponseValue<types::VerifyDomainSuccess>, Error<types::HandleVerifyDomainResponse>>
    {
        let url = format!("{}/domains/v1/verifications", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .put(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "handle_verify_domain",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            403u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            404u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            422u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            502u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Create a domain verification request
    ///
    ///This creates a Freestyle Domain Verification Request. It returns a
    /// `verificationCode` for your domain. You need to place this code in a TXT
    /// record at `_freestyle_custom_hostname.thedomain.com`, then call the
    /// [verify domain](/#tag/domains/PUT/domains/v1/verifications) endpoint
    /// with the domain to verify it.
    ///
    ///Sends a `POST` request to `/domains/v1/verifications`
    pub async fn handle_create_domain_verification<'a>(
        &'a self,
        body: &'a types::FreestyleDomainVerificationRequest,
    ) -> Result<
        ResponseValue<types::DomainVerificationRequest>,
        Error<types::HandleCreateDomainVerificationResponse>,
    > {
        let url = format!("{}/domains/v1/verifications", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "handle_create_domain_verification",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Delete a domain verification request
    ///
    ///This deletes a Freestyle Domain Verification Request. This does not
    /// remove the domain from the account if it has already been verified,
    /// however the verification code will no longer be valid.
    ///
    ///Sends a `DELETE` request to `/domains/v1/verifications`
    pub async fn handle_delete_domain_verification<'a>(
        &'a self,
        body: &'a types::FreestyleDeleteDomainVerificationRequest,
    ) -> Result<
        ResponseValue<types::HandleDeleteDomainVerificationResponse>,
        Error<types::HandleDeleteDomainVerificationResponse>,
    > {
        let url = format!("{}/domains/v1/verifications", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "handle_delete_domain_verification",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///List execute runs
    ///
    ///List execute runs.
    ///
    ///Sends a `GET` request to `/execute/v1/deployments`
    pub async fn handle_list_execute_runs<'a>(
        &'a self,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Result<
        ResponseValue<types::HandleListExecuteRunsResponse>,
        Error<types::HandleListExecuteRunsResponse>,
    > {
        let url = format!("{}/execute/v1/deployments", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("limit", &limit))
            .query(&progenitor_client::QueryParam::new("offset", &offset))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "handle_list_execute_runs",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            500u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get information on execute run
    ///
    ///Get information on execute run
    ///
    ///Sends a `GET` request to `/execute/v1/deployments/{deployment}`
    pub async fn handle_get_execute_run<'a>(
        &'a self,
        deployment: &'a ::uuid::Uuid,
    ) -> Result<
        ResponseValue<types::HandleGetExecuteRunResponse>,
        Error<types::HandleGetExecuteRunResponse>,
    > {
        let url = format!(
            "{}/execute/v1/deployments/{}",
            self.baseurl,
            encode_path(&deployment.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "handle_get_execute_run",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            404u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Execute Code
    ///
    ///Send a TypeScript or JavaScript module, get the result
    ///
    ///Sends a `POST` request to `/execute/v1/script`
    pub async fn handle_execute_script<'a>(
        &'a self,
        body: &'a types::FreestyleExecuteScriptParams,
    ) -> Result<
        ResponseValue<types::HandleExecuteScriptResponse>,
        Error<types::HandleExecuteScriptResponse>,
    > {
        let url = format!("{}/execute/v1/script", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "handle_execute_script",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Execute Code
    ///
    ///Send a TypeScript or JavaScript module, get the result (v2 - proper
    /// error codes)
    ///
    ///Sends a `POST` request to `/execute/v2/script`
    pub async fn handle_execute_script_v2<'a>(
        &'a self,
        body: &'a types::FreestyleExecuteScriptParams,
    ) -> Result<
        ResponseValue<types::ExecuteScriptSuccess>,
        Error<types::HandleExecuteScriptV2Response>,
    > {
        let url = format!("{}/execute/v2/script", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "handle_execute_script_v2",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            403u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Execute Code (Stateless)
    ///
    ///Send a TypeScript or JavaScript module, get the result (v3 - stateless
    /// execution)
    ///
    ///Sends a `POST` request to `/execute/v3/script`
    pub async fn handle_execute_script_v3<'a>(
        &'a self,
        body: &'a types::FreestyleExecuteScriptParams,
    ) -> Result<
        ResponseValue<types::ExecuteScriptSuccess>,
        Error<types::HandleExecuteScriptV3Response>,
    > {
        let url = format!("{}/execute/v3/script", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "handle_execute_script_v3",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            403u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///List repositories
    ///
    ///List repositories with metadata.
    ///
    ///Sends a `GET` request to `/git/v1/repo`
    ///
    ///Arguments:
    /// - `limit`: Maximum number of repositories to return
    /// - `offset`: Offset for the list of repositories
    pub async fn handle_list_repositories<'a>(
        &'a self,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<
        ResponseValue<types::ListRepositoriesSuccess>,
        Error<types::HandleListRepositoriesResponse>,
    > {
        let url = format!("{}/git/v1/repo", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("limit", &limit))
            .query(&progenitor_client::QueryParam::new("offset", &offset))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "handle_list_repositories",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            500u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Create a repository
    ///
    ///Create a repository. Once the repository is created, it will also be
    /// created on the Git server. The repository name must be unique within
    /// your account.
    ///
    ///Once created, you can then push your code to this repository.
    ///
    ///The repo will be available at `git.freestyle.sh/{repo-id}`
    ///
    ///
    ///Sends a `POST` request to `/git/v1/repo`
    pub async fn handle_create_repo<'a>(
        &'a self,
        body: &'a types::HandleCreateRepoBody,
    ) -> Result<
        ResponseValue<types::CreateRepositoryResponseSuccess>,
        Error<types::HandleCreateRepoResponse>,
    > {
        let url = format!("{}/git/v1/repo", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "handle_create_repo",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            403u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get repository default branch
    ///
    ///Get the default branch name for a repository.
    ///
    ///Sends a `GET` request to `/git/v1/repo/{repo_id}/default-branch`
    ///
    ///Arguments:
    /// - `repo_id`: The repository ID
    pub async fn handle_get_default_branch<'a>(
        &'a self,
        repo_id: &'a ::uuid::Uuid,
    ) -> Result<
        ResponseValue<types::GetDefaultBranchResponse>,
        Error<types::HandleGetDefaultBranchResponse>,
    > {
        let url = format!(
            "{}/git/v1/repo/{}/default-branch",
            self.baseurl,
            encode_path(&repo_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "handle_get_default_branch",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            403u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Set repository default branch
    ///
    ///Set the default branch name for a repository. This will update the HEAD
    /// reference to point to the new default branch.
    ///
    ///Sends a `PUT` request to `/git/v1/repo/{repo_id}/default-branch`
    ///
    ///Arguments:
    /// - `repo_id`: The repository ID
    /// - `body`
    pub async fn handle_set_default_branch<'a>(
        &'a self,
        repo_id: &'a ::uuid::Uuid,
        body: &'a types::SetDefaultBranchRequest,
    ) -> Result<
        ResponseValue<types::SetDefaultBranchResponse>,
        Error<types::HandleSetDefaultBranchResponse>,
    > {
        let url = format!(
            "{}/git/v1/repo/{}/default-branch",
            self.baseurl,
            encode_path(&repo_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .put(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "handle_set_default_branch",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            403u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get GitHub sync configuration
    ///
    ///Get the GitHub sync configuration for a repository, if configured.
    ///
    ///Sends a `GET` request to `/git/v1/repo/{repo_id}/github-sync`
    ///
    ///Arguments:
    /// - `repo_id`: Repository ID
    pub async fn get_github_sync<'a>(
        &'a self,
        repo_id: &'a ::uuid::Uuid,
    ) -> Result<ResponseValue<types::GithubSyncConfigResponse>, Error<()>> {
        let url = format!(
            "{}/git/v1/repo/{}/github-sync",
            self.baseurl,
            encode_path(&repo_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_github_sync",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Configure GitHub sync for repository
    ///
    ///Configure GitHub synchronization for an existing Freestyle repository.
    /// This links your Freestyle repository to a GitHub repository for
    /// automatic syncing. Requires the GitHub repository name in 'owner/repo'
    /// format.
    ///
    ///Sends a `POST` request to `/git/v1/repo/{repo_id}/github-sync`
    ///
    ///Arguments:
    /// - `repo_id`: Repository ID
    /// - `body`
    pub async fn configure_github_sync<'a>(
        &'a self,
        repo_id: &'a ::uuid::Uuid,
        body: &'a types::ConfigureGithubSyncRequest,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/git/v1/repo/{}/github-sync",
            self.baseurl,
            encode_path(&repo_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "configure_github_sync",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Remove GitHub sync configuration
    ///
    ///Remove GitHub sync configuration from a repository. This stops automatic
    /// syncing but doesn't affect the repository content.
    ///
    ///Sends a `DELETE` request to `/git/v1/repo/{repo_id}/github-sync`
    ///
    ///Arguments:
    /// - `repo_id`: Repository ID
    pub async fn remove_github_sync<'a>(
        &'a self,
        repo_id: &'a ::uuid::Uuid,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/git/v1/repo/{}/github-sync",
            self.baseurl,
            encode_path(&repo_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.delete(url).headers(header_map).build()?;
        let info = OperationInfo {
            operation_id: "remove_github_sync",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get repository visibility
    ///
    ///Get the visibility (public or private) for a repository.
    ///
    ///Sends a `GET` request to `/git/v1/repo/{repo_id}/visibility`
    ///
    ///Arguments:
    /// - `repo_id`: The repository ID
    pub async fn handle_get_visibility<'a>(
        &'a self,
        repo_id: &'a ::uuid::Uuid,
    ) -> Result<
        ResponseValue<types::GetVisibilityResponse>,
        Error<types::HandleGetVisibilityResponse>,
    > {
        let url = format!(
            "{}/git/v1/repo/{}/visibility",
            self.baseurl,
            encode_path(&repo_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "handle_get_visibility",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            403u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Set repository visibility
    ///
    ///Set the visibility (public or private) for a repository.
    ///
    ///Sends a `PUT` request to `/git/v1/repo/{repo_id}/visibility`
    ///
    ///Arguments:
    /// - `repo_id`: The repository ID
    /// - `body`
    pub async fn handle_set_visibility<'a>(
        &'a self,
        repo_id: &'a ::uuid::Uuid,
        body: &'a types::SetVisibilityRequest,
    ) -> Result<
        ResponseValue<types::SetVisibilityResponse>,
        Error<types::HandleSetVisibilityResponse>,
    > {
        let url = format!(
            "{}/git/v1/repo/{}/visibility",
            self.baseurl,
            encode_path(&repo_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .put(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "handle_set_visibility",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            403u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get repository information
    ///
    ///Retrieve information about a specific repository, including its ID,
    /// name, and default branch.
    ///
    ///Sends a `GET` request to `/git/v1/repo/{repo}`
    ///
    ///Arguments:
    /// - `repo`: The repository id
    pub async fn handle_get_repo_info<'a>(
        &'a self,
        repo: &'a str,
    ) -> Result<ResponseValue<types::RepositoryInfoRaw>, Error<()>> {
        let url = format!(
            "{}/git/v1/repo/{}",
            self.baseurl,
            encode_path(&repo.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "handle_get_repo_info",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Delete a repository
    ///
    ///Delete a repository. This is irreversible, and will also delete the
    /// repository on the Git server.
    ///
    ///Sends a `DELETE` request to `/git/v1/repo/{repo}`
    ///
    ///Arguments:
    /// - `repo`: The repository id
    pub async fn handle_delete_repo<'a>(
        &'a self,
        repo: &'a str,
    ) -> Result<ResponseValue<types::DeleteRepositorySuccess>, Error<types::HandleDeleteRepoResponse>>
    {
        let url = format!(
            "{}/git/v1/repo/{}",
            self.baseurl,
            encode_path(&repo.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "handle_delete_repo",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            403u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Create a commit with files
    ///
    ///Create a commit from files with automatic tree building and branch
    /// updates. Supports text files (UTF-8), binary files (base64), file
    /// deletions, and optimistic concurrency control via expectedSha. If the
    /// target branch does not exist, it is created as an orphan branch with
    /// this commit as its root.
    ///
    ///Sends a `POST` request to `/git/v1/repo/{repo}/commits`
    ///
    ///Arguments:
    /// - `repo`: The repository id
    /// - `body`
    pub async fn handle_create_commit<'a>(
        &'a self,
        repo: &'a ::uuid::Uuid,
        body: &'a types::CreateCommitRequest,
    ) -> Result<ResponseValue<types::CreateCommitResponse>, Error<types::HandleCreateCommitResponse>>
    {
        let url = format!(
            "{}/git/v1/repo/{}/commits",
            self.baseurl,
            encode_path(&repo.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "handle_create_commit",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            403u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Compare two commits
    ///
    ///Get the comparison between two commits in a repository
    ///
    ///Sends a `GET` request to `/git/v1/repo/{repo}/compare`
    ///
    ///Arguments:
    /// - `repo`: The repository id
    /// - `base`: Base revision (commit SHA, branch name, tag, or any valid Git
    ///   revision)
    /// - `head`: Head revision (commit SHA, branch name, tag, or any valid Git
    ///   revision)
    pub async fn handle_compare_commits<'a>(
        &'a self,
        repo: &'a ::uuid::Uuid,
        base: &'a str,
        head: &'a str,
    ) -> Result<ResponseValue<types::CommitComparison>, Error<()>> {
        let url = format!(
            "{}/git/v1/repo/{}/compare",
            self.baseurl,
            encode_path(&repo.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("base", &base))
            .query(&progenitor_client::QueryParam::new("head", &head))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "handle_compare_commits",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get the contents of a file or directory
    ///
    ///Get the contents of a file or directory in a repository
    ///
    ///Sends a `GET` request to `/git/v1/repo/{repo}/contents/{path}`
    ///
    ///Arguments:
    /// - `repo`: The repository ID.
    /// - `path`: The path to the file or directory. Empty for root.
    /// - `rev`: The git revision (branch name, commit SHA, etc.). Defaults to
    ///   HEAD.
    pub async fn handle_get_contents<'a>(
        &'a self,
        repo: &'a ::uuid::Uuid,
        path: &'a str,
        rev: Option<&'a str>,
    ) -> Result<ResponseValue<types::GitContents>, Error<()>> {
        let url = format!(
            "{}/git/v1/repo/{}/contents/{}",
            self.baseurl,
            encode_path(&repo.to_string()),
            encode_path(&path.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("rev", &rev))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "handle_get_contents",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get a blob object
    ///
    ///Get a blob from the Git database. The contents will always be base64
    /// encoded.
    ///
    ///Sends a `GET` request to `/git/v1/repo/{repo}/git/blobs/{hash}`
    ///
    ///Arguments:
    /// - `repo`: The repository id
    /// - `hash`: The object's hash
    pub async fn handle_get_blob<'a>(
        &'a self,
        repo: &'a ::uuid::Uuid,
        hash: &'a str,
    ) -> Result<ResponseValue<types::BlobObject>, Error<()>> {
        let url = format!(
            "{}/git/v1/repo/{}/git/blobs/{}",
            self.baseurl,
            encode_path(&repo.to_string()),
            encode_path(&hash.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "handle_get_blob",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///List commits for a repository
    ///
    ///List commits with flexible filtering and pagination. Supports:
    /// - Cursor-based pagination (recommended): Use `next_commit` from response
    ///   with `until` (desc order) or `since` (asc order)
    /// - Commit ranges: Specify both `since` and `until` to get commits between
    ///   them (like git's `since..until`)
    /// - Order control: `order=desc` (newest first, default) or `order=asc`
    ///   (oldest first)
    /// - Legacy offset pagination: Use `offset` param (deprecated, incompatible
    ///   with since/until)
    ///
    ///Requires repository visibility to be public or user to be the owner.
    ///
    ///Sends a `GET` request to `/git/v1/repo/{repo}/git/commits`
    ///
    ///Arguments:
    /// - `repo`: The repository id
    /// - `branch`: Branch name (defaults to HEAD)
    /// - `limit`: Maximum number of commits to return (default: 50, max: 500)
    /// - `offset`: Number of commits to skip (default: 0)
    /// - `order`: Sort order: "desc" (newest first, default) or "asc" (oldest
    ///   first)
    /// - `since`: Start point (older commit) for filtering/pagination (commit
    ///   SHA)
    /// - When used alone with order=desc: Returns commits newer than this
    ///   (exclusive). Use for "what's new since X?"
    /// - When used alone with order=asc: Starts from this commit (inclusive).
    ///   Use with next_commit for pagination.
    /// - When used with `until`: Defines the base of a range (like git's A..B
    ///   where this is A)
    /// - `until`: End point (newer commit) for filtering/pagination (commit
    ///   SHA)
    /// - When used alone with order=desc: Starts from this commit (inclusive).
    ///   Use with next_commit for pagination.
    /// - When used alone with order=asc: Returns commits up to and including
    ///   this commit.
    /// - When used with `since`: Defines the head of a range (like git's A..B
    ///   where this is B)
    ///
    ///When both `since` and `until` are provided, `until` must be a descendant
    /// of `since`.
    pub async fn handle_list_commits<'a>(
        &'a self,
        repo: &'a ::uuid::Uuid,
        branch: Option<&'a str>,
        limit: Option<u64>,
        offset: Option<u64>,
        order: Option<types::CommitOrder>,
        since: Option<&'a str>,
        until: Option<&'a str>,
    ) -> Result<ResponseValue<types::CommitList>, Error<types::HandleListCommitsResponse>> {
        let url = format!(
            "{}/git/v1/repo/{}/git/commits",
            self.baseurl,
            encode_path(&repo.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("branch", &branch))
            .query(&progenitor_client::QueryParam::new("limit", &limit))
            .query(&progenitor_client::QueryParam::new("offset", &offset))
            .query(&progenitor_client::QueryParam::new("order", &order))
            .query(&progenitor_client::QueryParam::new("since", &since))
            .query(&progenitor_client::QueryParam::new("until", &until))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "handle_list_commits",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            403u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Create a commit from git objects
    ///
    ///Create a commit from tree and parent SHAs. This is a git object database
    /// operation for advanced use cases. For file-based commits, use the
    /// high-level endpoint at /git/v1/repo/{repo}/commits.
    ///
    ///Sends a `POST` request to `/git/v1/repo/{repo}/git/commits`
    ///
    ///Arguments:
    /// - `repo`: The repository id
    /// - `body`
    pub async fn handle_create_odb_commit<'a>(
        &'a self,
        repo: &'a ::uuid::Uuid,
        body: &'a types::CreateOdbCommitRequest,
    ) -> Result<
        ResponseValue<types::CreateOdbCommitResponse>,
        Error<types::HandleCreateOdbCommitResponse>,
    > {
        let url = format!(
            "{}/git/v1/repo/{}/git/commits",
            self.baseurl,
            encode_path(&repo.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "handle_create_odb_commit",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            403u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get a commit object
    ///
    ///Get a commit from the Git database with detailed information.
    ///
    ///Sends a `GET` request to `/git/v1/repo/{repo}/git/commits/{hash}`
    ///
    ///Arguments:
    /// - `repo`: The repository id
    /// - `hash`: The object's hash
    pub async fn handle_get_commit<'a>(
        &'a self,
        repo: &'a ::uuid::Uuid,
        hash: &'a str,
    ) -> Result<ResponseValue<types::CommitObject>, Error<()>> {
        let url = format!(
            "{}/git/v1/repo/{}/git/commits/{}",
            self.baseurl,
            encode_path(&repo.to_string()),
            encode_path(&hash.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "handle_get_commit",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///List branches in a repo
    ///
    ///Get a list of all branches in the Git repository. Returns branch names
    /// and their commit SHAs.
    ///
    ///Sends a `GET` request to `/git/v1/repo/{repo}/git/refs/heads/`
    ///
    ///Arguments:
    /// - `repo`: The repository id
    pub async fn handle_list_branches<'a>(
        &'a self,
        repo: &'a ::uuid::Uuid,
    ) -> Result<ResponseValue<types::ListBranchesResponse>, Error<()>> {
        let url = format!(
            "{}/git/v1/repo/{}/git/refs/heads/",
            self.baseurl,
            encode_path(&repo.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "handle_list_branches",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get a branch reference
    ///
    ///Get a reference to a branch in the Git repository. Returns the ref name
    /// and SHA of the branch.
    ///
    ///Sends a `GET` request to `/git/v1/repo/{repo}/git/refs/heads/{branch}`
    ///
    ///Arguments:
    /// - `repo`: The repository id
    /// - `branch`: The branch's name
    pub async fn handle_get_ref_branch<'a>(
        &'a self,
        repo: &'a ::uuid::Uuid,
        branch: &'a str,
    ) -> Result<ResponseValue<types::GitReference>, Error<types::HandleGetRefBranchResponse>> {
        let url = format!(
            "{}/git/v1/repo/{}/git/refs/heads/{}",
            self.baseurl,
            encode_path(&repo.to_string()),
            encode_path(&branch.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "handle_get_ref_branch",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            403u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///List all tags in a repository
    ///
    ///List all tags in the Git repository with full details. Returns both
    /// annotated tags (with tagger and message)        and lightweight
    /// tags. For each tag, the `sha` field contains the tag object SHA (for
    /// annotated tags) or        commit SHA (for lightweight tags), while
    /// `target.sha` always contains the commit SHA the tag points to.
    ///
    ///Sends a `GET` request to `/git/v1/repo/{repo}/git/refs/tags/`
    ///
    ///Arguments:
    /// - `repo`: The repository id
    pub async fn handle_list_tags<'a>(
        &'a self,
        repo: &'a ::uuid::Uuid,
    ) -> Result<ResponseValue<types::ListTagsResponse>, Error<()>> {
        let url = format!(
            "{}/git/v1/repo/{}/git/refs/tags/",
            self.baseurl,
            encode_path(&repo.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "handle_list_tags",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get a tag by name
    ///
    ///Get a tag by its name. This is the recommended endpoint for retrieving
    /// tag information.        Works with both annotated tags (returns
    /// tagger, message, and target commit) and lightweight tags
    ///        (returns only name and target commit). For annotated tags, the
    /// `sha` field contains the tag object's SHA,        while `target.sha`
    /// contains the commit SHA. For lightweight tags, both fields contain the
    /// same commit SHA.
    ///
    ///        **Note**: If you need to retrieve an annotated tag object by its
    /// SHA (for low-level git database operations),        use
    /// `/git/v1/repo/{repo}/git/tags/{hash}` instead.
    ///
    ///Sends a `GET` request to `/git/v1/repo/{repo}/git/refs/tags/{tag}`
    ///
    ///Arguments:
    /// - `repo`: The repository id
    /// - `tag`: The tag's name
    pub async fn handle_get_ref_tag<'a>(
        &'a self,
        repo: &'a ::uuid::Uuid,
        tag: &'a str,
    ) -> Result<ResponseValue<types::HandleGetRefTagResponse>, Error<types::HandleGetRefTagResponse>>
    {
        let url = format!(
            "{}/git/v1/repo/{}/git/refs/tags/{}",
            self.baseurl,
            encode_path(&repo.to_string()),
            encode_path(&tag.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "handle_get_ref_tag",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            403u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get an annotated tag object by SHA (advanced)
    ///
    ///Low-level endpoint for retrieving annotated tag objects directly from
    /// the Git database by their SHA hash.        This endpoint only works
    /// with annotated tags and requires the tag object's SHA, not the tag name.
    ///
    ///
    ///        **Most users should use `/git/v1/repo/{repo}/git/refs/tags/{tag}`
    /// instead**, which works with both        annotated and lightweight
    /// tags and accepts tag names rather than SHAs. This endpoint is primarily
    ///        for advanced use cases that need direct access to git database
    /// objects.
    ///
    ///Sends a `GET` request to `/git/v1/repo/{repo}/git/tags/{hash}`
    ///
    ///Arguments:
    /// - `repo`: The repository id
    /// - `hash`: The object's hash
    pub async fn handle_get_tag<'a>(
        &'a self,
        repo: &'a ::uuid::Uuid,
        hash: &'a str,
    ) -> Result<ResponseValue<types::TagObject>, Error<()>> {
        let url = format!(
            "{}/git/v1/repo/{}/git/tags/{}",
            self.baseurl,
            encode_path(&repo.to_string()),
            encode_path(&hash.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "handle_get_tag",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get a tree object
    ///
    ///Get a tree from the Git database with its entries.
    ///
    ///Sends a `GET` request to `/git/v1/repo/{repo}/git/trees/{hash}`
    ///
    ///Arguments:
    /// - `repo`: The repository id
    /// - `hash`: The object's hash
    pub async fn handle_get_tree<'a>(
        &'a self,
        repo: &'a ::uuid::Uuid,
        hash: &'a str,
    ) -> Result<ResponseValue<types::TreeObject>, Error<()>> {
        let url = format!(
            "{}/git/v1/repo/{}/git/trees/{}",
            self.baseurl,
            encode_path(&repo.to_string()),
            encode_path(&hash.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "handle_get_tree",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Download a tarball of a repo
    ///
    ///Download the contents of a repo as a tarball.
    ///
    ///Sends a `GET` request to `/git/v1/repo/{repo}/tarball`
    ///
    ///Arguments:
    /// - `repo`: The repository id
    /// - `rev`: The git revision (branch name, commit SHA, etc.). Defaults to
    ///   HEAD.
    pub async fn handle_download_tarball<'a>(
        &'a self,
        repo: &'a ::uuid::Uuid,
        rev: Option<&'a str>,
    ) -> Result<ResponseValue<ByteStream>, Error<()>> {
        let url = format!(
            "{}/git/v1/repo/{}/tarball",
            self.baseurl,
            encode_path(&repo.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .query(&progenitor_client::QueryParam::new("rev", &rev))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "handle_download_tarball",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::stream(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///List git triggers for a repository
    ///
    ///List git triggers for the given repository.
    ///
    ///Sends a `GET` request to `/git/v1/repo/{repo}/trigger`
    ///
    ///Arguments:
    /// - `repo`: The repository id
    pub async fn handle_list_git_triggers<'a>(
        &'a self,
        repo: &'a ::uuid::Uuid,
    ) -> Result<
        ResponseValue<types::HandleListGitTriggersResponse>,
        Error<types::HandleListGitTriggersResponse>,
    > {
        let url = format!(
            "{}/git/v1/repo/{}/trigger",
            self.baseurl,
            encode_path(&repo.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "handle_list_git_triggers",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            403u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            404u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Create a git trigger
    ///
    ///Create a git trigger for the given repository.
    ///
    ///Sends a `POST` request to `/git/v1/repo/{repo}/trigger`
    ///
    ///Arguments:
    /// - `repo`: The repository id
    /// - `body`
    pub async fn handle_create_git_trigger<'a>(
        &'a self,
        repo: &'a ::uuid::Uuid,
        body: &'a types::HandleCreateGitTriggerBody,
    ) -> Result<
        ResponseValue<types::HandleCreateGitTriggerResponse>,
        Error<types::HandleCreateGitTriggerResponse>,
    > {
        let url = format!(
            "{}/git/v1/repo/{}/trigger",
            self.baseurl,
            encode_path(&repo.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "handle_create_git_trigger",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            403u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            404u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Delete a git trigger
    ///
    ///Delete a git trigger. This is irreversible.
    ///
    ///Sends a `DELETE` request to `/git/v1/repo/{repo}/trigger/{trigger}`
    ///
    ///Arguments:
    /// - `repo`: The repository id
    /// - `trigger`: The trigger id
    pub async fn handle_delete_git_trigger<'a>(
        &'a self,
        repo: &'a ::uuid::Uuid,
        trigger: &'a str,
    ) -> Result<
        ResponseValue<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        Error<types::HandleDeleteGitTriggerResponse>,
    > {
        let url = format!(
            "{}/git/v1/repo/{}/trigger/{}",
            self.baseurl,
            encode_path(&repo.to_string()),
            encode_path(&trigger.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "handle_delete_git_trigger",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            403u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Download a zip of a repo
    ///
    ///Download the contents of a repo as a zip.
    ///
    ///Sends a `GET` request to `/git/v1/repo/{repo}/zip`
    ///
    ///Arguments:
    /// - `repo`: The repository id
    /// - `rev`: The git revision (branch name, commit SHA, etc.). Defaults to
    ///   HEAD.
    pub async fn handle_download_zip<'a>(
        &'a self,
        repo: &'a ::uuid::Uuid,
        rev: Option<&'a str>,
    ) -> Result<ResponseValue<ByteStream>, Error<()>> {
        let url = format!(
            "{}/git/v1/repo/{}/zip",
            self.baseurl,
            encode_path(&repo.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .query(&progenitor_client::QueryParam::new("rev", &rev))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "handle_download_zip",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::stream(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///List identities
    ///
    ///List identities created by your account.
    ///
    ///Sends a `GET` request to `/identity/v1/identities`
    pub async fn handle_list_identities<'a>(
        &'a self,
        include_managed: Option<bool>,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<ResponseValue<types::ListIdentitiesSuccess>, Error<()>> {
        let url = format!("{}/identity/v1/identities", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new(
                "includeManaged",
                &include_managed,
            ))
            .query(&progenitor_client::QueryParam::new("limit", &limit))
            .query(&progenitor_client::QueryParam::new("offset", &offset))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "handle_list_identities",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Create an identity
    ///
    ///Create an identity. This identity will be used to authenticate with the
    /// Git server.
    ///
    ///Sends a `POST` request to `/identity/v1/identities`
    pub async fn handle_create_identity<'a>(
        &'a self,
    ) -> Result<ResponseValue<types::FreestyleIdentity>, Error<()>> {
        let url = format!("{}/identity/v1/identities", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "handle_create_identity",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Delete an identity
    ///
    ///Delete an identity. This will revoke all permissions granted to this
    /// identity.
    ///
    ///Sends a `DELETE` request to `/identity/v1/identities/{identity}`
    pub async fn handle_delete_identity<'a>(
        &'a self,
        identity: &'a ::uuid::Uuid,
    ) -> Result<ResponseValue<types::EmptyResponse>, Error<()>> {
        let url = format!(
            "{}/identity/v1/identities/{}",
            self.baseurl,
            encode_path(&identity.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "handle_delete_identity",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///List repository permissions for an identity
    ///
    ///List repository permissions for an identity. This will return a list of
    /// repositories that the identity has access to.
    ///
    ///Sends a `GET` request to
    /// `/identity/v1/identities/{identity}/permissions/git`
    ///
    ///Arguments:
    /// - `identity`
    /// - `limit`: Maximum number of repositories to return
    /// - `offset`: Offset for the list of repositories
    pub async fn handle_list_git_permissions<'a>(
        &'a self,
        identity: &'a ::uuid::Uuid,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<ResponseValue<types::ListGitPermissionSuccess>, Error<()>> {
        let url = format!(
            "{}/identity/v1/identities/{}/permissions/git",
            self.baseurl,
            encode_path(&identity.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("limit", &limit))
            .query(&progenitor_client::QueryParam::new("offset", &offset))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "handle_list_git_permissions",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get the git permission of an identity on a repository
    ///
    ///Get the permission of an identity on a repository. This will return the
    /// access level of the identity on the repository.
    ///
    ///Sends a `GET` request to
    /// `/identity/v1/identities/{identity}/permissions/git/{repo}`
    ///
    ///Arguments:
    /// - `identity`: The git identity ID
    /// - `repo`: The git repository ID
    pub async fn handle_describe_git_permission<'a>(
        &'a self,
        identity: &'a ::uuid::Uuid,
        repo: &'a ::uuid::Uuid,
    ) -> Result<ResponseValue<types::DescribeGitPermissionSuccess>, Error<()>> {
        let url = format!(
            "{}/identity/v1/identities/{}/permissions/git/{}",
            self.baseurl,
            encode_path(&identity.to_string()),
            encode_path(&repo.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "handle_describe_git_permission",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Update a git repository permission for an identity
    ///
    ///Update a permission for an identity on a repository
    ///
    ///Sends a `PUT` request to
    /// `/identity/v1/identities/{identity}/permissions/git/{repo}`
    ///
    ///Arguments:
    /// - `identity`: The git identity ID
    /// - `repo`: The git repository ID
    /// - `body`
    pub async fn handle_update_git_permission<'a>(
        &'a self,
        identity: &'a ::uuid::Uuid,
        repo: &'a ::uuid::Uuid,
        body: &'a types::UpdateGitPermissionRequest,
    ) -> Result<ResponseValue<types::EmptyResponse>, Error<()>> {
        let url = format!(
            "{}/identity/v1/identities/{}/permissions/git/{}",
            self.baseurl,
            encode_path(&identity.to_string()),
            encode_path(&repo.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .put(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "handle_update_git_permission",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Grant a git repository permission to an identity
    ///
    ///Grant a permission to an identity on a repository
    ///
    ///Sends a `POST` request to
    /// `/identity/v1/identities/{identity}/permissions/git/{repo}`
    ///
    ///Arguments:
    /// - `identity`: The git identity ID
    /// - `repo`: The git repository ID
    /// - `body`
    pub async fn handle_grant_git_permission<'a>(
        &'a self,
        identity: &'a ::uuid::Uuid,
        repo: &'a ::uuid::Uuid,
        body: &'a types::GrantGitPermissionRequest,
    ) -> Result<ResponseValue<types::EmptyResponse>, Error<()>> {
        let url = format!(
            "{}/identity/v1/identities/{}/permissions/git/{}",
            self.baseurl,
            encode_path(&identity.to_string()),
            encode_path(&repo.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "handle_grant_git_permission",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Revoke git repository permission from an identity
    ///
    ///Revoke a permission to an identity on a repository
    ///
    ///Sends a `DELETE` request to
    /// `/identity/v1/identities/{identity}/permissions/git/{repo}`
    ///
    ///Arguments:
    /// - `identity`: The git identity ID
    /// - `repo`: The git repository ID
    pub async fn handle_revoke_git_permission<'a>(
        &'a self,
        identity: &'a ::uuid::Uuid,
        repo: &'a ::uuid::Uuid,
    ) -> Result<ResponseValue<types::EmptyResponse>, Error<()>> {
        let url = format!(
            "{}/identity/v1/identities/{}/permissions/git/{}",
            self.baseurl,
            encode_path(&identity.to_string()),
            encode_path(&repo.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "handle_revoke_git_permission",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///List VM permissions for an identity
    ///
    ///List all VM permissions granted to a specific Git identity
    ///
    ///Sends a `GET` request to
    /// `/identity/v1/identities/{identity}/permissions/vm`
    pub async fn handle_list_vm_permissions<'a>(
        &'a self,
        identity: &'a ::uuid::Uuid,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<ResponseValue<types::ListVmPermissionsSuccess>, Error<()>> {
        let url = format!(
            "{}/identity/v1/identities/{}/permissions/vm",
            self.baseurl,
            encode_path(&identity.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("limit", &limit))
            .query(&progenitor_client::QueryParam::new("offset", &offset))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "handle_list_vm_permissions",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get VM permission details
    ///
    ///Get the details of a VM permission for a specific identity and VM
    ///
    ///Sends a `GET` request to
    /// `/identity/v1/identities/{identity}/permissions/vm/{vm_id}`
    ///
    ///Arguments:
    /// - `identity`: The git identity ID
    /// - `vm_id`: The VM ID
    pub async fn handle_describe_vm_permission<'a>(
        &'a self,
        identity: &'a ::uuid::Uuid,
        vm_id: &'a str,
    ) -> Result<ResponseValue<types::VmPermission>, Error<()>> {
        let url = format!(
            "{}/identity/v1/identities/{}/permissions/vm/{}",
            self.baseurl,
            encode_path(&identity.to_string()),
            encode_path(&vm_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "handle_describe_vm_permission",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Update allowed users for VM permission
    ///
    ///Update the list of allowed users for a VM permission. Set to null to
    /// allow all users, or provide a list to restrict access.
    ///
    ///Sends a `PUT` request to
    /// `/identity/v1/identities/{identity}/permissions/vm/{vm_id}`
    ///
    ///Arguments:
    /// - `identity`: The git identity ID
    /// - `vm_id`: The VM ID
    /// - `body`
    pub async fn handle_update_allowed_users<'a>(
        &'a self,
        identity: &'a ::uuid::Uuid,
        vm_id: &'a str,
        body: &'a types::UpdateAllowedUsersRequestBody,
    ) -> Result<ResponseValue<types::VmPermission>, Error<()>> {
        let url = format!(
            "{}/identity/v1/identities/{}/permissions/vm/{}",
            self.baseurl,
            encode_path(&identity.to_string()),
            encode_path(&vm_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .put(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "handle_update_allowed_users",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Grant VM permission to an identity for a VM
    ///
    ///Grant VM access permission to an identity for a specific VM. Optionally
    /// restrict access to specific Linux users.
    ///
    ///Sends a `POST` request to
    /// `/identity/v1/identities/{identity}/permissions/vm/{vm_id}`
    ///
    ///Arguments:
    /// - `identity`: The git identity ID
    /// - `vm_id`: The VM ID
    /// - `body`
    pub async fn handle_grant_vm_permission<'a>(
        &'a self,
        identity: &'a ::uuid::Uuid,
        vm_id: &'a str,
        body: &'a types::GrantVmPermissionRequest,
    ) -> Result<ResponseValue<types::VmPermission>, Error<()>> {
        let url = format!(
            "{}/identity/v1/identities/{}/permissions/vm/{}",
            self.baseurl,
            encode_path(&identity.to_string()),
            encode_path(&vm_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "handle_grant_vm_permission",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Revoke VM permission from an identity for a VM
    ///
    ///Revoke VM permission from an identity for a specific VM
    ///
    ///Sends a `DELETE` request to
    /// `/identity/v1/identities/{identity}/permissions/vm/{vm_id}`
    ///
    ///Arguments:
    /// - `identity`: The git identity ID
    /// - `vm_id`: The VM ID
    pub async fn handle_revoke_vm_permission<'a>(
        &'a self,
        identity: &'a ::uuid::Uuid,
        vm_id: &'a str,
    ) -> Result<ResponseValue<types::EmptyResponse>, Error<()>> {
        let url = format!(
            "{}/identity/v1/identities/{}/permissions/vm/{}",
            self.baseurl,
            encode_path(&identity.to_string()),
            encode_path(&vm_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "handle_revoke_vm_permission",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///List access tokens for an identity
    ///
    ///List access tokens for an identity
    ///
    ///Sends a `GET` request to `/identity/v1/identities/{identity}/tokens`
    pub async fn handle_list_git_tokens<'a>(
        &'a self,
        identity: &'a ::uuid::Uuid,
    ) -> Result<ResponseValue<types::ListGitTokensSuccess>, Error<()>> {
        let url = format!(
            "{}/identity/v1/identities/{}/tokens",
            self.baseurl,
            encode_path(&identity.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "handle_list_git_tokens",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Create an access token for an identity
    ///
    ///Create an access token for an identity
    ///
    ///Sends a `POST` request to `/identity/v1/identities/{identity}/tokens`
    pub async fn handle_create_git_token<'a>(
        &'a self,
        identity: &'a ::uuid::Uuid,
    ) -> Result<ResponseValue<types::CreatedToken>, Error<()>> {
        let url = format!(
            "{}/identity/v1/identities/{}/tokens",
            self.baseurl,
            encode_path(&identity.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "handle_create_git_token",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Revoke an access token for an identity
    ///
    ///Revoke an access token for an identity
    ///
    ///Sends a `DELETE` request to
    /// `/identity/v1/identities/{identity}/tokens/{token}`
    pub async fn handle_revoke_git_token<'a>(
        &'a self,
        identity: &'a ::uuid::Uuid,
        token: &'a ::uuid::Uuid,
    ) -> Result<ResponseValue<types::EmptyResponse>, Error<()>> {
        let url = format!(
            "{}/identity/v1/identities/{}/tokens/{}",
            self.baseurl,
            encode_path(&identity.to_string()),
            encode_path(&token.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "handle_revoke_git_token",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Deployment Logs
    ///
    ///Get the logs for a deployment
    ///
    ///Sends a `GET` request to `/observability/v1/logs`
    ///
    ///Arguments:
    /// - `deployment_id`
    /// - `domain`
    /// - `end_time`: End time in RFC3339 format (e.g., "2024-01-02T00:00:00Z").
    ///   Defaults to now if not specified.
    /// - `instance_id`
    /// - `page_size`
    /// - `page_token`
    /// - `start_time`: Start time in RFC3339 format (e.g.,
    ///   "2024-01-01T00:00:00Z"). Defaults to 24 hours ago if not specified.
    /// - `vm_id`
    /// - `vm_service`
    pub async fn handle_get_logs<'a>(
        &'a self,
        deployment_id: Option<&'a str>,
        domain: Option<&'a str>,
        end_time: Option<&'a str>,
        instance_id: Option<&'a str>,
        page_size: Option<i32>,
        page_token: Option<&'a str>,
        start_time: Option<&'a str>,
        vm_id: Option<&'a str>,
        vm_service: Option<&'a str>,
    ) -> Result<ResponseValue<types::FreestyleGetLogsResponse>, Error<()>> {
        let url = format!("{}/observability/v1/logs", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new(
                "deploymentId",
                &deployment_id,
            ))
            .query(&progenitor_client::QueryParam::new("domain", &domain))
            .query(&progenitor_client::QueryParam::new("endTime", &end_time))
            .query(&progenitor_client::QueryParam::new(
                "instanceId",
                &instance_id,
            ))
            .query(&progenitor_client::QueryParam::new("pageSize", &page_size))
            .query(&progenitor_client::QueryParam::new(
                "pageToken",
                &page_token,
            ))
            .query(&progenitor_client::QueryParam::new(
                "startTime",
                &start_time,
            ))
            .query(&progenitor_client::QueryParam::new("vmId", &vm_id))
            .query(&progenitor_client::QueryParam::new(
                "vmService",
                &vm_service,
            ))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "handle_get_logs",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to `/v1/cron/schedules`
    pub async fn handle_list_schedules<'a>(
        &'a self,
        deployment_id: Option<&'a ::uuid::Uuid>,
        metrics_end: Option<&'a ::chrono::DateTime<::chrono::offset::Utc>>,
        metrics_start: Option<&'a ::chrono::DateTime<::chrono::offset::Utc>>,
    ) -> Result<ResponseValue<types::ListSchedulesResponse>, Error<()>> {
        let url = format!("{}/v1/cron/schedules", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new(
                "deploymentId",
                &deployment_id,
            ))
            .query(&progenitor_client::QueryParam::new(
                "metricsEnd",
                &metrics_end,
            ))
            .query(&progenitor_client::QueryParam::new(
                "metricsStart",
                &metrics_start,
            ))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "handle_list_schedules",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `POST` request to `/v1/cron/schedules`
    pub async fn handle_create_schedule<'a>(
        &'a self,
        body: &'a types::CreateScheduleRequest,
    ) -> Result<ResponseValue<types::CreateScheduleResponse>, Error<()>> {
        let url = format!("{}/v1/cron/schedules", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "handle_create_schedule",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to `/v1/cron/schedules/{id}`
    ///
    ///Arguments:
    /// - `id`: Schedule ID
    pub async fn handle_get_schedule<'a>(
        &'a self,
        id: &'a ::uuid::Uuid,
    ) -> Result<ResponseValue<types::GetScheduleResponse>, Error<()>> {
        let url = format!(
            "{}/v1/cron/schedules/{}",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "handle_get_schedule",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `DELETE` request to `/v1/cron/schedules/{id}`
    ///
    ///Arguments:
    /// - `id`: Schedule ID
    pub async fn handle_delete_schedule<'a>(
        &'a self,
        id: &'a ::uuid::Uuid,
    ) -> Result<ResponseValue<types::DeleteScheduleResponse>, Error<()>> {
        let url = format!(
            "{}/v1/cron/schedules/{}",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "handle_delete_schedule",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `PATCH` request to `/v1/cron/schedules/{id}`
    ///
    ///Arguments:
    /// - `id`: Schedule ID
    /// - `body`
    pub async fn handle_update_schedule<'a>(
        &'a self,
        id: &'a ::uuid::Uuid,
        body: &'a types::UpdateScheduleRequest,
    ) -> Result<ResponseValue<types::UpdateScheduleResponse>, Error<()>> {
        let url = format!(
            "{}/v1/cron/schedules/{}",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .patch(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "handle_update_schedule",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to `/v1/cron/schedules/{id}/executions`
    ///
    ///Arguments:
    /// - `id`: Schedule ID
    /// - `limit`
    /// - `offset`
    pub async fn handle_list_schedule_executions<'a>(
        &'a self,
        id: &'a ::uuid::Uuid,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<ResponseValue<types::ListScheduleExecutionsResponse>, Error<()>> {
        let url = format!(
            "{}/v1/cron/schedules/{}/executions",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("limit", &limit))
            .query(&progenitor_client::QueryParam::new("offset", &offset))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "handle_list_schedule_executions",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to `/v1/cron/schedules/{id}/metrics-timeline`
    ///
    ///Arguments:
    /// - `id`: Schedule ID
    /// - `bucket_minutes`
    /// - `end`
    /// - `start`
    pub async fn handle_get_metrics_timeline<'a>(
        &'a self,
        id: &'a ::uuid::Uuid,
        bucket_minutes: Option<i64>,
        end: &'a ::chrono::DateTime<::chrono::offset::Utc>,
        start: &'a ::chrono::DateTime<::chrono::offset::Utc>,
    ) -> Result<ResponseValue<types::MetricsTimelineResponseBody>, Error<()>> {
        let url = format!(
            "{}/v1/cron/schedules/{}/metrics-timeline",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new(
                "bucketMinutes",
                &bucket_minutes,
            ))
            .query(&progenitor_client::QueryParam::new("end", &end))
            .query(&progenitor_client::QueryParam::new("start", &start))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "handle_get_metrics_timeline",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to `/v1/cron/schedules/{id}/success-rate`
    ///
    ///Arguments:
    /// - `id`: Schedule ID
    /// - `end`
    /// - `start`
    pub async fn handle_get_schedule_success_rate<'a>(
        &'a self,
        id: &'a ::uuid::Uuid,
        end: &'a ::chrono::DateTime<::chrono::offset::Utc>,
        start: &'a ::chrono::DateTime<::chrono::offset::Utc>,
    ) -> Result<ResponseValue<types::ScheduleSuccessRateResponseBody>, Error<()>> {
        let url = format!(
            "{}/v1/cron/schedules/{}/success-rate",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("end", &end))
            .query(&progenitor_client::QueryParam::new("start", &start))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "handle_get_schedule_success_rate",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///List VMs
    ///
    ///Sends a `GET` request to `/v1/vms`
    pub async fn list_vms<'a>(
        &'a self,
        include_deleted: Option<bool>,
    ) -> Result<ResponseValue<types::ListVmsResponse>, Error<types::ListVmsResponse>> {
        let url = format!("{}/v1/vms", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new(
                "includeDeleted",
                &include_deleted,
            ))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "list_vms",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            500u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Create VM
    ///
    ///Sends a `POST` request to `/v1/vms`
    pub async fn create_vm<'a>(
        &'a self,
        body: &'a types::CreateVmRequest,
    ) -> Result<ResponseValue<types::CreateVmResponse>, Error<()>> {
        let url = format!("{}/v1/vms", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "create_vm",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///List all snapshots.
    ///
    ///Sends a `GET` request to `/v1/vms/snapshots`
    pub async fn list_snapshots<'a>(
        &'a self,
        include_deleted: Option<bool>,
        include_failed: Option<bool>,
    ) -> Result<ResponseValue<types::ListSnapshotsResponse>, Error<types::ListSnapshotsResponse>>
    {
        let url = format!("{}/v1/vms/snapshots", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new(
                "includeDeleted",
                &include_deleted,
            ))
            .query(&progenitor_client::QueryParam::new(
                "includeFailed",
                &include_failed,
            ))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "list_snapshots",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Create a snapshot by creating a temporary VM, starting it, snapshotting
    /// it, then deleting the VM.
    ///
    ///Sends a `POST` request to `/v1/vms/snapshots`
    pub async fn create_snapshot<'a>(
        &'a self,
        body: &'a types::CreateSnapshotRequest,
    ) -> Result<ResponseValue<types::CreateSnapshotResponse>, Error<()>> {
        let url = format!("{}/v1/vms/snapshots", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "create_snapshot",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Delete a snapshot
    ///
    ///Sends a `DELETE` request to `/v1/vms/snapshots/{snapshot_id}`
    ///
    ///Arguments:
    /// - `snapshot_id`: The ID of the snapshot to delete
    pub async fn delete_snapshot<'a>(
        &'a self,
        snapshot_id: &'a str,
    ) -> Result<ResponseValue<types::DeleteSnapshotResponses>, Error<types::DeleteSnapshotResponse>>
    {
        let url = format!(
            "{}/v1/vms/snapshots/{}",
            self.baseurl,
            encode_path(&snapshot_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "delete_snapshot",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            404u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            409u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Update snapshot metadata
    ///
    ///Sends a `PATCH` request to `/v1/vms/snapshots/{snapshot_id}`
    ///
    ///Arguments:
    /// - `snapshot_id`: The ID of the snapshot to update
    /// - `body`
    pub async fn update_snapshot<'a>(
        &'a self,
        snapshot_id: &'a str,
        body: &'a types::UpdateSnapshotRequest,
    ) -> Result<ResponseValue<types::UpdateSnapshotResponses>, Error<types::UpdateSnapshotResponse>>
    {
        let url = format!(
            "{}/v1/vms/snapshots/{}",
            self.baseurl,
            encode_path(&snapshot_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .patch(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "update_snapshot",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            404u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Resize VM
    ///
    ///Sends a `POST` request to `/v1/vms/{id}/resize`
    pub async fn resize_vm<'a>(
        &'a self,
        id: &'a str,
        body: &'a types::ResizeVmRequest,
    ) -> Result<ResponseValue<types::ResizeVmResponse>, Error<types::ResizeVmResponse>> {
        let url = format!(
            "{}/v1/vms/{}/resize",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "resize_vm",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            404u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get VM
    ///
    ///Sends a `GET` request to `/v1/vms/{vm_id}`
    pub async fn get_vm<'a>(
        &'a self,
        vm_id: &'a str,
    ) -> Result<ResponseValue<types::GetVmResponse>, Error<types::GetVmResponse>> {
        let url = format!(
            "{}/v1/vms/{}",
            self.baseurl,
            encode_path(&vm_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_vm",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            500u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Delete VM
    ///
    ///Sends a `DELETE` request to `/v1/vms/{vm_id}`
    ///
    ///Arguments:
    /// - `vm_id`: The ID of the VM to delete
    pub async fn delete_vm<'a>(
        &'a self,
        vm_id: &'a str,
    ) -> Result<ResponseValue<types::DeleteVmResponses>, Error<types::DeleteVmResponse>> {
        let url = format!(
            "{}/v1/vms/{}",
            self.baseurl,
            encode_path(&vm_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "delete_vm",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            500u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Wait for VM to stop
    ///
    ///Sends a `POST` request to `/v1/vms/{vm_id}/await`
    ///
    ///Arguments:
    /// - `vm_id`: The ID of the VM to wait for
    pub async fn wait_vm<'a>(
        &'a self,
        vm_id: &'a str,
    ) -> Result<ResponseValue<types::WaitVmResponse>, Error<types::WaitVmResponse>> {
        let url = format!(
            "{}/v1/vms/{}/await",
            self.baseurl,
            encode_path(&vm_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "wait_vm",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            500u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Execute command in VM and await result
    ///
    ///Sends a `POST` request to `/v1/vms/{vm_id}/exec-await`
    ///
    ///Arguments:
    /// - `vm_id`: The ID of the VM to execute the command in
    /// - `body`
    pub async fn exec_await<'a>(
        &'a self,
        vm_id: &'a str,
        body: &'a types::ExecAwaitRequest,
    ) -> Result<ResponseValue<types::ExecAwaitVmResponse>, Error<types::ExecAwaitResponse>> {
        let url = format!(
            "{}/v1/vms/{}/exec-await",
            self.baseurl,
            encode_path(&vm_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "exec_await",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            500u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get file from VM
    ///
    ///Sends a `GET` request to `/v1/vms/{vm_id}/files/{filepath}`
    ///
    ///Arguments:
    /// - `vm_id`: The ID of the VM to get the file from
    /// - `filepath`: The path of the file to get
    pub async fn get_file<'a>(
        &'a self,
        vm_id: &'a str,
        filepath: &'a str,
    ) -> Result<ResponseValue<types::FileSystemResponse>, Error<types::GetFileResponse>> {
        let url = format!(
            "{}/v1/vms/{}/files/{}",
            self.baseurl,
            encode_path(&vm_id.to_string()),
            encode_path(&filepath.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_file",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            404u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Put file to VM
    ///
    ///Sends a `PUT` request to `/v1/vms/{vm_id}/files/{filepath}`
    ///
    ///Arguments:
    /// - `vm_id`: The ID of the VM to put the file to
    /// - `filepath`: The path of the file to put
    /// - `body`
    pub async fn put_file<'a>(
        &'a self,
        vm_id: &'a str,
        filepath: &'a str,
        body: &'a types::WriteFileRequest,
    ) -> Result<ResponseValue<types::WriteFileResponse>, Error<types::PutFileResponse>> {
        let url = format!(
            "{}/v1/vms/{}/files/{}",
            self.baseurl,
            encode_path(&vm_id.to_string()),
            encode_path(&filepath.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .put(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "put_file",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            404u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Fork VM
    ///
    ///Sends a `POST` request to `/v1/vms/{vm_id}/fork`
    pub async fn fork_vm<'a>(
        &'a self,
        vm_id: &'a str,
        body: &'a types::ForkVmRequest,
    ) -> Result<ResponseValue<types::ForkVmResponse>, Error<types::ForkVmResponse>> {
        let url = format!(
            "{}/v1/vms/{}/fork",
            self.baseurl,
            encode_path(&vm_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "fork_vm",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            403u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Kill VM
    ///
    ///Sends a `POST` request to `/v1/vms/{vm_id}/kill`
    ///
    ///Arguments:
    /// - `vm_id`: The ID of the VM to kill
    pub async fn kill_vm<'a>(
        &'a self,
        vm_id: &'a str,
    ) -> Result<ResponseValue<types::KillVmResponse>, Error<types::KillVmResponse>> {
        let url = format!(
            "{}/v1/vms/{}/kill",
            self.baseurl,
            encode_path(&vm_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "kill_vm",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            500u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Suspends a VM and reallocates storage for more efficient forking.
    ///
    ///Sends a `POST` request to `/v1/vms/{vm_id}/optimize`
    ///
    ///Arguments:
    /// - `vm_id`: The ID of the VM to optimize
    pub async fn optimize_vm<'a>(
        &'a self,
        vm_id: &'a str,
    ) -> Result<ResponseValue<types::OptimizeVmResponse>, Error<types::OptimizeVmResponse>> {
        let url = format!(
            "{}/v1/vms/{}/optimize",
            self.baseurl,
            encode_path(&vm_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "optimize_vm",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            500u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Create a snapshot of a VM. The snapshot is stored in a special snapshots
    /// folder and cannot be booted directly, but can be used to create new VMs.
    ///
    ///Sends a `POST` request to `/v1/vms/{vm_id}/snapshot`
    pub async fn snapshot_vm<'a>(
        &'a self,
        vm_id: &'a str,
        body: &'a types::SnapshotVmRequest,
    ) -> Result<ResponseValue<types::SnapshotVmResponse>, Error<()>> {
        let url = format!(
            "{}/v1/vms/{}/snapshot",
            self.baseurl,
            encode_path(&vm_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "snapshot_vm",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Start VM
    ///
    ///Sends a `POST` request to `/v1/vms/{vm_id}/start`
    pub async fn start_vm<'a>(
        &'a self,
        vm_id: &'a str,
        body: &'a types::StartVmRequest,
    ) -> Result<ResponseValue<types::StartedVmResponse>, Error<()>> {
        let url = format!(
            "{}/v1/vms/{}/start",
            self.baseurl,
            encode_path(&vm_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "start_vm",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Stop VM
    ///
    ///Sends a `POST` request to `/v1/vms/{vm_id}/stop`
    ///
    ///Arguments:
    /// - `vm_id`: The ID of the VM to stop
    pub async fn stop_vm<'a>(
        &'a self,
        vm_id: &'a str,
    ) -> Result<ResponseValue<types::StopVmResponse>, Error<types::StopVmResponse>> {
        let url = format!(
            "{}/v1/vms/{}/stop",
            self.baseurl,
            encode_path(&vm_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "stop_vm",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Suspend VM
    ///
    ///Sends a `POST` request to `/v1/vms/{vm_id}/suspend`
    ///
    ///Arguments:
    /// - `vm_id`: The ID of the VM to suspend
    pub async fn suspend_vm<'a>(
        &'a self,
        vm_id: &'a str,
    ) -> Result<ResponseValue<types::SuspendVmResponse>, Error<types::SuspendVmResponse>> {
        let url = format!(
            "{}/v1/vms/{}/suspend",
            self.baseurl,
            encode_path(&vm_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "suspend_vm",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            500u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Restart multiple systemd services
    ///
    ///Sends a `POST` request to `/v1/vms/{vm_id}/systemd/restart`
    ///
    ///Arguments:
    /// - `vm_id`: The ID of the VM
    /// - `body`
    pub async fn batch_restart_services<'a>(
        &'a self,
        vm_id: &'a str,
        body: &'a types::BatchServiceRequest,
    ) -> Result<
        ResponseValue<types::BatchServiceResponse>,
        Error<types::BatchRestartServicesResponse>,
    > {
        let url = format!(
            "{}/v1/vms/{}/systemd/restart",
            self.baseurl,
            encode_path(&vm_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "batch_restart_services",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            500u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///List all systemd services for a VM
    ///
    ///Sends a `GET` request to `/v1/vms/{vm_id}/systemd/services`
    ///
    ///Arguments:
    /// - `vm_id`: The ID of the VM
    pub async fn list_services<'a>(
        &'a self,
        vm_id: &'a str,
    ) -> Result<ResponseValue<types::SystemdListServicesResponse>, Error<types::ListServicesResponse>>
    {
        let url = format!(
            "{}/v1/vms/{}/systemd/services",
            self.baseurl,
            encode_path(&vm_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "list_services",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            500u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Create a new systemd service
    ///
    ///Sends a `POST` request to `/v1/vms/{vm_id}/systemd/services`
    ///
    ///Arguments:
    /// - `vm_id`: The ID of the VM
    /// - `body`
    pub async fn create_service<'a>(
        &'a self,
        vm_id: &'a str,
        body: &'a types::SystemdUnitSpec,
    ) -> Result<
        ResponseValue<types::SystemdCreateServiceResponse>,
        Error<types::CreateServiceResponse>,
    > {
        let url = format!(
            "{}/v1/vms/{}/systemd/services",
            self.baseurl,
            encode_path(&vm_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "create_service",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            409u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Delete a systemd service
    ///
    ///Sends a `DELETE` request to
    /// `/v1/vms/{vm_id}/systemd/services/{service_id}`
    ///
    ///Arguments:
    /// - `vm_id`: The ID of the VM
    /// - `service_id`: The ID of the service
    pub async fn delete_service<'a>(
        &'a self,
        vm_id: &'a str,
        service_id: &'a str,
    ) -> Result<
        ResponseValue<types::SystemdDeleteServiceResponse>,
        Error<types::DeleteServiceResponse>,
    > {
        let url = format!(
            "{}/v1/vms/{}/systemd/services/{}",
            self.baseurl,
            encode_path(&vm_id.to_string()),
            encode_path(&service_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "delete_service",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            404u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get logs for a systemd service
    ///
    ///Sends a `GET` request to
    /// `/v1/vms/{vm_id}/systemd/services/{service_id}/logs`
    ///
    ///Arguments:
    /// - `vm_id`: The ID of the VM
    /// - `service_id`: The ID of the service
    /// - `lines`: Number of log lines to return
    /// - `since`: Only show logs since this timestamp
    pub async fn get_service_logs<'a>(
        &'a self,
        vm_id: &'a str,
        service_id: &'a str,
        lines: Option<i32>,
        since: Option<&'a str>,
    ) -> Result<ResponseValue<types::JournaldLogsResponse>, Error<types::GetServiceLogsResponse>>
    {
        let url = format!(
            "{}/v1/vms/{}/systemd/services/{}/logs",
            self.baseurl,
            encode_path(&vm_id.to_string()),
            encode_path(&service_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("lines", &lines))
            .query(&progenitor_client::QueryParam::new("since", &since))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_service_logs",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            404u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get status of a systemd service
    ///
    ///Sends a `GET` request to
    /// `/v1/vms/{vm_id}/systemd/services/{service_id}/status`
    ///
    ///Arguments:
    /// - `vm_id`: The ID of the VM
    /// - `service_id`: The ID of the service
    pub async fn get_service_status<'a>(
        &'a self,
        vm_id: &'a str,
        service_id: &'a str,
    ) -> Result<ResponseValue<types::SystemdServiceStatus>, Error<types::GetServiceStatusResponse>>
    {
        let url = format!(
            "{}/v1/vms/{}/systemd/services/{}/status",
            self.baseurl,
            encode_path(&vm_id.to_string()),
            encode_path(&service_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_service_status",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            404u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Start multiple systemd services
    ///
    ///Sends a `POST` request to `/v1/vms/{vm_id}/systemd/start`
    ///
    ///Arguments:
    /// - `vm_id`: The ID of the VM
    /// - `body`
    pub async fn batch_start_services<'a>(
        &'a self,
        vm_id: &'a str,
        body: &'a types::BatchServiceRequest,
    ) -> Result<ResponseValue<types::BatchServiceResponse>, Error<types::BatchStartServicesResponse>>
    {
        let url = format!(
            "{}/v1/vms/{}/systemd/start",
            self.baseurl,
            encode_path(&vm_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "batch_start_services",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            500u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Stop multiple systemd services
    ///
    ///Sends a `POST` request to `/v1/vms/{vm_id}/systemd/stop`
    ///
    ///Arguments:
    /// - `vm_id`: The ID of the VM
    /// - `body`
    pub async fn batch_stop_services<'a>(
        &'a self,
        vm_id: &'a str,
        body: &'a types::BatchServiceRequest,
    ) -> Result<ResponseValue<types::BatchServiceResponse>, Error<types::BatchStopServicesResponse>>
    {
        let url = format!(
            "{}/v1/vms/{}/systemd/stop",
            self.baseurl,
            encode_path(&vm_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "batch_stop_services",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            500u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///List all terminal sessions for a VM
    ///
    ///Sends a `GET` request to `/v1/vms/{vm_id}/terminals`
    ///
    ///Arguments:
    /// - `vm_id`: The ID of the VM
    pub async fn list_terminals<'a>(
        &'a self,
        vm_id: &'a str,
    ) -> Result<ResponseValue<types::TerminalListResponse>, Error<types::ListTerminalsResponse>>
    {
        let url = format!(
            "{}/v1/vms/{}/terminals",
            self.baseurl,
            encode_path(&vm_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "list_terminals",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            500u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get terminal logs as plain text array
    ///
    ///Sends a `GET` request to `/v1/vms/{vm_id}/terminals/{terminal_id}/logs`
    ///
    ///Arguments:
    /// - `vm_id`: The ID of the VM
    /// - `terminal_id`: The ID of the terminal session
    pub async fn get_terminal_logs<'a>(
        &'a self,
        vm_id: &'a str,
        terminal_id: &'a str,
    ) -> Result<
        ResponseValue<types::TerminalLogsArrayResponse>,
        Error<types::GetTerminalLogsResponse>,
    > {
        let url = format!(
            "{}/v1/vms/{}/terminals/{}/logs",
            self.baseurl,
            encode_path(&vm_id.to_string()),
            encode_path(&terminal_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_terminal_logs",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            500u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get terminal output with xterm formatting
    ///
    ///Sends a `GET` request to
    /// `/v1/vms/{vm_id}/terminals/{terminal_id}/xterm-256color`
    ///
    ///Arguments:
    /// - `vm_id`: The ID of the VM
    /// - `terminal_id`: The ID of the terminal session
    pub async fn get_terminal_xterm<'a>(
        &'a self,
        vm_id: &'a str,
        terminal_id: &'a str,
    ) -> Result<ResponseValue<types::TerminalLogsResponse>, Error<types::GetTerminalXtermResponse>>
    {
        let url = format!(
            "{}/v1/vms/{}/terminals/{}/xterm-256color",
            self.baseurl,
            encode_path(&vm_id.to_string()),
            encode_path(&terminal_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_terminal_xterm",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            500u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Watch VM Files
    ///
    ///Sends a `POST` request to `/v1/vms/{vm_id}/watch-files`
    ///
    ///Arguments:
    /// - `vm_id`: The ID of the VM to watch files for
    pub async fn watch_files<'a>(
        &'a self,
        vm_id: &'a str,
    ) -> Result<ResponseValue<ByteStream>, Error<()>> {
        let url = format!(
            "{}/v1/vms/{}/watch-files",
            self.baseurl,
            encode_path(&vm_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.post(url).headers(header_map).build()?;
        let info = OperationInfo {
            operation_id: "watch_files",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200..=299 => Ok(ResponseValue::stream(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Deploy a Website (v1)
    ///
    ///Deploy a website. Files is a map of file paths to file contents.
    /// Configuration is optional and contains additional information about the
    /// deployment.
    ///
    ///Sends a `POST` request to `/web/v1/deploy`
    pub async fn handle_deploy_web<'a>(
        &'a self,
        body: &'a types::FreestyleDeployWebPayload,
    ) -> Result<
        ResponseValue<types::FreestyleDeployWebSuccessResponseV2>,
        Error<types::FreestyleDeployWebErrorResponse>,
    > {
        let url = format!("{}/web/v1/deploy", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "handle_deploy_web",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Deploy a Website
    ///
    ///Deploy a website. Files is a map of file paths to file contents.
    /// Configuration is optional and contains additional information about the
    /// deployment.
    ///
    ///Sends a `POST` request to `/web/v1/deployment`
    pub async fn handle_deploy_web_v2<'a>(
        &'a self,
        body: &'a types::FreestyleDeployWebPayloadV2,
    ) -> Result<
        ResponseValue<types::FreestyleDeployWebSuccessResponseV2>,
        Error<types::HandleDeployWebV2Response>,
    > {
        let url = format!("{}/web/v1/deployment", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "handle_deploy_web_v2",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            403u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            404u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            502u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///List web deploys
    ///
    ///List web deploys.
    ///
    ///Sends a `GET` request to `/web/v1/deployments`
    ///
    ///Arguments:
    /// - `limit`: Maximum number of deployments to return
    /// - `offset`: Offset for pagination
    /// - `search`: Search by deployment ID or domain
    pub async fn handle_list_web_deploys<'a>(
        &'a self,
        limit: i64,
        offset: i64,
        search: &'a str,
    ) -> Result<
        ResponseValue<types::ListDeploymentsResponse>,
        Error<types::HandleListWebDeploysResponse>,
    > {
        let url = format!("{}/web/v1/deployments", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("limit", &limit))
            .query(&progenitor_client::QueryParam::new("offset", &offset))
            .query(&progenitor_client::QueryParam::new("search", &search))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "handle_list_web_deploys",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            500u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get information on web deploy
    ///
    ///Get information about a web deploy by its ID.
    ///
    ///Sends a `GET` request to `/web/v1/deployments/{deployment_id}`
    pub async fn handle_get_web_deploy_details<'a>(
        &'a self,
        deployment_id: &'a ::uuid::Uuid,
    ) -> Result<ResponseValue<ByteStream>, Error<()>> {
        let url = format!(
            "{}/web/v1/deployments/{}",
            self.baseurl,
            encode_path(&deployment_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.get(url).headers(header_map).build()?;
        let info = OperationInfo {
            operation_id: "handle_get_web_deploy_details",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200..=299 => Ok(ResponseValue::stream(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Create a new branch
    ///
    ///Create a new branch in the Git repository. If sha is not provided, the
    /// branch is created from the default branch HEAD.
    ///
    ///Sends a `POST` request to
    /// `/git/v1/repo/{repo}/git/refs/heads/create/{branch}`
    ///
    ///Arguments:
    /// - `repo`: The repository id
    /// - `branch`: The branch's name
    /// - `body`
    pub async fn handle_create_branch<'a>(
        &'a self,
        repo: &'a ::uuid::Uuid,
        branch: &'a str,
        body: &'a types::CreateBranchRequest,
    ) -> Result<ResponseValue<types::CreateBranchResponse>, Error<types::HandleCreateBranchResponse>>
    {
        let url = format!(
            "{}/git/v1/repo/{}/git/refs/heads/create/{}",
            self.baseurl,
            encode_path(&repo.to_string()),
            encode_path(&branch.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "handle_create_branch",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            403u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
}

/// Items consumers will typically use such as the Client.
pub mod prelude {
    #[allow(unused_imports)]
    pub use super::Client;
}
