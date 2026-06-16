//! Structured Error Types for Verona Agent Toolkit
//!
//! This module provides structured error handling with:
//! - Unique error codes for each error type
//! - Actionable remediation hints
//! - Retry classification for transient failures
//!
//! # Error Code Schema
//!
//! Format: `E{MODULE}{NUMBER}`
//!
//! Modules:
//! - AUTH: Authentication (EAUTH001-EAUTH099)
//! - TREASURY: Treasury operations (ETREASURY001-ETREASURY099)
//! - ASSET: Asset builder (EASSET001-EASSET099)
//! - BATCH: Batch operations (EBATCH001-EBATCH099)
//! - CONFIG: Configuration (ECONFIG001-ECONFIG099)
//! - NETWORK: Network/API (ENETWORK001-ENETWORK099)

use serde::{Deserialize, Serialize};
use std::fmt;
use thiserror::Error;

/// Error code enumeration with structured hints
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VeronaErrorCode {
    // ========================================================================
    // Authentication Errors (EAUTH001-EAUTH099)
    // ========================================================================
    /// Not authenticated
    EAUTH001,
    /// Token expired
    EAUTH002,
    /// Refresh token expired
    EAUTH003,
    /// Invalid credentials
    EAUTH004,
    /// OAuth2 callback failed
    EAUTH005,
    /// PKCE verification failed
    EAUTH006,
    /// Authentication timeout
    EAUTH007,

    // ========================================================================
    // Treasury Errors (ETREASURY001-ETREASURY099)
    // ========================================================================
    /// Treasury not found
    ETREASURY001,
    /// Insufficient balance
    ETREASURY002,
    /// Invalid treasury address
    ETREASURY003,
    /// Treasury creation failed
    ETREASURY004,
    /// Treasury operation failed
    ETREASURY005,
    /// Grant config not found
    ETREASURY006,
    /// Fee config not found
    ETREASURY007,
    /// Not authorized for treasury
    ETREASURY008,
    /// Treasury already exists
    ETREASURY009,
    /// Missing authorization input
    ETREASURY010,

    // ========================================================================
    // Asset Builder Errors (EASSET001-EASSET099)
    // ========================================================================
    /// Invalid metadata
    EASSET001,
    /// Asset creation failed
    EASSET002,
    /// Invalid asset configuration
    EASSET003,
    /// Code ID not found
    EASSET004,
    /// Invalid schema
    EASSET005,

    // ========================================================================
    // Batch Errors (EBATCH001-EBATCH099)
    // ========================================================================
    /// Batch too large
    EBATCH001,
    /// Batch execution failed
    EBATCH002,
    /// Partial batch failure
    EBATCH003,
    /// Invalid batch item
    EBATCH004,

    // ========================================================================
    // Configuration Errors (ECONFIG001-ECONFIG099)
    // ========================================================================
    /// Configuration not found
    ECONFIG001,
    /// Invalid configuration
    ECONFIG002,
    /// Encryption failed
    ECONFIG003,
    /// Decryption failed
    ECONFIG004,
    /// Network not found
    ECONFIG005,

    // ========================================================================
    // Network Errors (ENETWORK001-ENETWORK099)
    // ========================================================================
    /// Connection timeout
    ENETWORK001,
    /// Rate limited
    ENETWORK002,
    /// Service unavailable
    ENETWORK003,
    /// Invalid response
    ENETWORK004,
    /// Request failed
    ENETWORK005,
    /// Connection refused
    ENETWORK006,
    /// DNS resolution failed
    ENETWORK007,
    /// TLS error
    ENETWORK008,

    // ========================================================================
    // Transaction Errors (ETX001-ETX099)
    // ========================================================================
    /// Transaction query failed
    ETX001,
    /// Transaction wait failed
    ETX002,
    /// Transaction timeout
    ETX003,

    // ========================================================================
    // Faucet Errors (EFAUCET001-EFAUCET099)
    // ========================================================================
    /// Faucet claim failed
    EFAUCET001,
    /// Faucet query failed
    EFAUCET002,
    /// Not authenticated for faucet
    EFAUCET003,
    /// Faucet not available
    EFAUCET004,

    // ========================================================================
    // OAuth Client Errors (EOAUTHCLIENT001-EOAUTHCLIENT099)
    // ========================================================================
    /// Bad request
    EOAUTHCLIENT001,
    /// Client ID required
    EOAUTHCLIENT002,
    /// Redirect URIs required
    EOAUTHCLIENT003,
    /// Binded treasury required
    EOAUTHCLIENT004,
    /// Owner required
    EOAUTHCLIENT005,
    /// Invalid grant type
    EOAUTHCLIENT006,
    /// Manager user ID required
    EOAUTHCLIENT007,
    /// Authentication required
    EOAUTHCLIENT008,
    /// User not found
    EOAUTHCLIENT009,
    /// Insufficient scope
    EOAUTHCLIENT010,
    /// Only owner allowed
    EOAUTHCLIENT011,
    /// Client not found
    EOAUTHCLIENT012,
    /// Client extension not found
    EOAUTHCLIENT013,
    /// Treasury not found (MGR API)
    EOAUTHCLIENT014,
    /// Internal server error (MGR API)
    EOAUTHCLIENT015,
    /// Treasury fetch error
    EOAUTHCLIENT016,
    /// Treasury query error
    EOAUTHCLIENT017,
    /// Unknown network
    EOAUTHCLIENT018,
    /// Re-run the command with --force to confirm
    EOAUTHCLIENT019,
}

impl VeronaErrorCode {
    /// Get the error message for this code
    pub fn message(&self) -> &'static str {
        match self {
            // Authentication
            VeronaErrorCode::EAUTH001 => "Not authenticated",
            VeronaErrorCode::EAUTH002 => "Token expired",
            VeronaErrorCode::EAUTH003 => "Refresh token expired",
            VeronaErrorCode::EAUTH004 => "Invalid credentials",
            VeronaErrorCode::EAUTH005 => "OAuth2 callback failed",
            VeronaErrorCode::EAUTH006 => "PKCE verification failed",
            VeronaErrorCode::EAUTH007 => "Authentication timeout",

            // Treasury
            VeronaErrorCode::ETREASURY001 => "Treasury not found",
            VeronaErrorCode::ETREASURY002 => "Insufficient balance",
            VeronaErrorCode::ETREASURY003 => "Invalid treasury address",
            VeronaErrorCode::ETREASURY004 => "Treasury creation failed",
            VeronaErrorCode::ETREASURY005 => "Treasury operation failed",
            VeronaErrorCode::ETREASURY006 => "Grant config not found",
            VeronaErrorCode::ETREASURY007 => "Fee config not found",
            VeronaErrorCode::ETREASURY008 => "Not authorized for treasury operation",
            VeronaErrorCode::ETREASURY009 => "Treasury already exists",
            VeronaErrorCode::ETREASURY010 => "Missing authorization input for grant config",

            // Asset Builder
            VeronaErrorCode::EASSET001 => "Invalid metadata",
            VeronaErrorCode::EASSET002 => "Asset creation failed",
            VeronaErrorCode::EASSET003 => "Invalid asset configuration",
            VeronaErrorCode::EASSET004 => "Code ID not found",
            VeronaErrorCode::EASSET005 => "Invalid schema",

            // Batch
            VeronaErrorCode::EBATCH001 => "Batch too large",
            VeronaErrorCode::EBATCH002 => "Batch execution failed",
            VeronaErrorCode::EBATCH003 => "Partial batch failure",
            VeronaErrorCode::EBATCH004 => "Invalid batch item",

            // Configuration
            VeronaErrorCode::ECONFIG001 => "Configuration not found",
            VeronaErrorCode::ECONFIG002 => "Invalid configuration",
            VeronaErrorCode::ECONFIG003 => "Encryption failed",
            VeronaErrorCode::ECONFIG004 => "Decryption failed",
            VeronaErrorCode::ECONFIG005 => "Network not found in configuration",

            // Network
            VeronaErrorCode::ENETWORK001 => "Connection timeout",
            VeronaErrorCode::ENETWORK002 => "Rate limited",
            VeronaErrorCode::ENETWORK003 => "Service unavailable",
            VeronaErrorCode::ENETWORK004 => "Invalid response from server",
            VeronaErrorCode::ENETWORK005 => "Request failed",
            VeronaErrorCode::ENETWORK006 => "Connection refused",
            VeronaErrorCode::ENETWORK007 => "DNS resolution failed",
            VeronaErrorCode::ENETWORK008 => "TLS error",

            // Transaction
            VeronaErrorCode::ETX001 => "Transaction query failed",
            VeronaErrorCode::ETX002 => "Transaction wait failed",
            VeronaErrorCode::ETX003 => "Transaction timeout",

            // Faucet
            VeronaErrorCode::EFAUCET001 => "Faucet claim failed",
            VeronaErrorCode::EFAUCET002 => "Faucet query failed",
            VeronaErrorCode::EFAUCET003 => "Not authenticated for faucet operation",
            VeronaErrorCode::EFAUCET004 => "Faucet not available on this network",

            // OAuth Client Management
            VeronaErrorCode::EOAUTHCLIENT001 => "Bad request",
            VeronaErrorCode::EOAUTHCLIENT002 => "Client ID is required",
            VeronaErrorCode::EOAUTHCLIENT003 => "Redirect URIs are required",
            VeronaErrorCode::EOAUTHCLIENT004 => "Binded treasury is required",
            VeronaErrorCode::EOAUTHCLIENT005 => "Owner is required",
            VeronaErrorCode::EOAUTHCLIENT006 => "Invalid grant type",
            VeronaErrorCode::EOAUTHCLIENT007 => "Manager user ID is required",
            VeronaErrorCode::EOAUTHCLIENT008 => "Authentication required",
            VeronaErrorCode::EOAUTHCLIENT009 => "User not found",
            VeronaErrorCode::EOAUTHCLIENT010 => "Insufficient scope",
            VeronaErrorCode::EOAUTHCLIENT011 => "Only owner allowed",
            VeronaErrorCode::EOAUTHCLIENT012 => "Client not found",
            VeronaErrorCode::EOAUTHCLIENT013 => "Client extension not found",
            VeronaErrorCode::EOAUTHCLIENT014 => "Treasury not found",
            VeronaErrorCode::EOAUTHCLIENT015 => "Internal server error",
            VeronaErrorCode::EOAUTHCLIENT016 => "Treasury fetch error",
            VeronaErrorCode::EOAUTHCLIENT017 => "Treasury query error",
            VeronaErrorCode::EOAUTHCLIENT018 => "Unknown network",
            VeronaErrorCode::EOAUTHCLIENT019 => "Confirmation required",
        }
    }

    /// Get the remediation hint for this error code
    pub fn hint(&self) -> &'static str {
        match self {
            // Authentication
            VeronaErrorCode::EAUTH001 => "Run 'verona-toolkit auth login' first",
            VeronaErrorCode::EAUTH002 => "Token refreshed automatically, please retry",
            VeronaErrorCode::EAUTH003 => "Re-login required: 'verona-toolkit auth login'",
            VeronaErrorCode::EAUTH004 => "Check your credentials and try again",
            VeronaErrorCode::EAUTH005 => "Ensure callback URL is accessible and try again",
            VeronaErrorCode::EAUTH006 => "PKCE verification mismatch, restart login flow",
            VeronaErrorCode::EAUTH007 => "Authentication took too long, please try again",

            // Treasury
            VeronaErrorCode::ETREASURY001 => {
                "Run 'verona-toolkit treasury list' to see available treasuries"
            }
            VeronaErrorCode::ETREASURY002 => "Fund treasury with 'verona-toolkit treasury fund'",
            VeronaErrorCode::ETREASURY003 => "Verify the treasury address is a valid bech32 address",
            VeronaErrorCode::ETREASURY004 => "Check parameters and try again",
            VeronaErrorCode::ETREASURY005 => "Check treasury state and try again",
            VeronaErrorCode::ETREASURY006 => {
                "Run 'verona-toolkit treasury grant-config list' to see available grants"
            }
            VeronaErrorCode::ETREASURY007 => {
                "Run 'verona-toolkit treasury fee-config query' to check fee config"
            }
            VeronaErrorCode::ETREASURY008 => "Ensure you are the admin of this treasury",
            VeronaErrorCode::ETREASURY009 => "Use a different salt or address for the new treasury",
            VeronaErrorCode::ETREASURY010 => {
                "Ensure grant config has authorization_input when importing"
            }

            // Asset Builder
            VeronaErrorCode::EASSET001 => "Check JSON structure against schema",
            VeronaErrorCode::EASSET002 => "Check asset configuration and try again",
            VeronaErrorCode::EASSET003 => "Verify all required fields are present",
            VeronaErrorCode::EASSET004 => {
                "Check available code IDs with 'verona-toolkit asset code-ids'"
            }
            VeronaErrorCode::EASSET005 => "Validate your schema against the expected format",

            // Batch
            VeronaErrorCode::EBATCH001 => "Maximum 50 messages per batch",
            VeronaErrorCode::EBATCH002 => "Check individual message errors and retry",
            VeronaErrorCode::EBATCH003 => "Some operations succeeded, check results for details",
            VeronaErrorCode::EBATCH004 => "Verify batch item format and content",

            // Configuration
            VeronaErrorCode::ECONFIG001 => "Run 'verona-toolkit config init' to create configuration",
            VeronaErrorCode::ECONFIG002 => "Check configuration file format and values",
            VeronaErrorCode::ECONFIG003 => "Check encryption key availability",
            VeronaErrorCode::ECONFIG004 => "Check encryption key matches the one used for encryption",
            VeronaErrorCode::ECONFIG005 => "Specify network with '--network' flag or update config",

            // Network
            VeronaErrorCode::ENETWORK001 => "Check network connectivity, will retry",
            VeronaErrorCode::ENETWORK002 => "Wait and retry, or reduce request frequency",
            VeronaErrorCode::ENETWORK003 => "Service is temporarily unavailable, retry later",
            VeronaErrorCode::ENETWORK004 => "Server returned unexpected data, check API version",
            VeronaErrorCode::ENETWORK005 => "Check network settings and API endpoint",
            VeronaErrorCode::ENETWORK006 => "Server is not accepting connections, check endpoint",
            VeronaErrorCode::ENETWORK007 => "Check DNS settings and network connectivity",
            VeronaErrorCode::ENETWORK008 => "Check TLS certificates and HTTPS configuration",

            // Transaction
            VeronaErrorCode::ETX001 => "Check network connection and transaction hash",
            VeronaErrorCode::ETX002 => "Check network connection and wait parameters",
            VeronaErrorCode::ETX003 => "Transaction took too long to confirm, check chain status",

            // Faucet
            VeronaErrorCode::EFAUCET001 => "Wait for cooldown or check error details",
            VeronaErrorCode::EFAUCET002 => {
                "Check network connection and faucet contract availability"
            }
            VeronaErrorCode::EFAUCET003 => "Run 'verona-toolkit auth login' first",
            VeronaErrorCode::EFAUCET004 => "Use --network testnet to claim testnet tokens",

            // OAuth Client Management
            VeronaErrorCode::EOAUTHCLIENT001 => "Check request parameters and try again",
            VeronaErrorCode::EOAUTHCLIENT002 => "Provide a client ID",
            VeronaErrorCode::EOAUTHCLIENT003 => "Provide at least one redirect URI",
            VeronaErrorCode::EOAUTHCLIENT004 => "Provide a treasury address with --treasury",
            VeronaErrorCode::EOAUTHCLIENT005 => "Provide an owner user ID",
            VeronaErrorCode::EOAUTHCLIENT006 => "Use a valid grant type (authorization_code, etc.)",
            VeronaErrorCode::EOAUTHCLIENT007 => "Provide a manager user ID",
            VeronaErrorCode::EOAUTHCLIENT008 => {
                "Token was rejected by the server. Try re-authenticating: verona-toolkit auth login --force --dev-mode"
            }
            VeronaErrorCode::EOAUTHCLIENT009 => "Run 'verona-toolkit auth login' first",
            VeronaErrorCode::EOAUTHCLIENT010 => {
                "Re-authorize with --dev-mode: verona-toolkit auth login --dev-mode"
            }
            VeronaErrorCode::EOAUTHCLIENT011 => "Only the client owner can perform this action",
            VeronaErrorCode::EOAUTHCLIENT012 => "Check the client ID and try again",
            VeronaErrorCode::EOAUTHCLIENT013 => "Check the client ID; extension may not exist",
            VeronaErrorCode::EOAUTHCLIENT014 => "Verify the treasury address is correct",
            VeronaErrorCode::EOAUTHCLIENT015 => {
                "The server encountered an error. Please try again later."
            }
            VeronaErrorCode::EOAUTHCLIENT016 => "Failed to fetch treasury data. Try again later.",
            VeronaErrorCode::EOAUTHCLIENT017 => "Failed to query treasury data. Try again later.",
            VeronaErrorCode::EOAUTHCLIENT018 => "Verify network configuration and try again",
            VeronaErrorCode::EOAUTHCLIENT019 => "Re-run the command with --force to confirm",
        }
    }

    /// Check if this error is retryable
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            // Network errors are generally retryable
            VeronaErrorCode::ENETWORK001
            | VeronaErrorCode::ENETWORK002
            | VeronaErrorCode::ENETWORK003
            | VeronaErrorCode::ENETWORK006
            | VeronaErrorCode::ENETWORK007
            // Token expired can be retried after refresh
            | VeronaErrorCode::EAUTH002
            // Faucet errors may be retryable (cooldown, temporary issues)
            | VeronaErrorCode::EFAUCET001
            | VeronaErrorCode::EFAUCET002
        )
    }

    /// Get the module name for this error code
    pub fn module(&self) -> &'static str {
        match self {
            VeronaErrorCode::EAUTH001
            | VeronaErrorCode::EAUTH002
            | VeronaErrorCode::EAUTH003
            | VeronaErrorCode::EAUTH004
            | VeronaErrorCode::EAUTH005
            | VeronaErrorCode::EAUTH006
            | VeronaErrorCode::EAUTH007 => "AUTH",
            VeronaErrorCode::ETREASURY001
            | VeronaErrorCode::ETREASURY002
            | VeronaErrorCode::ETREASURY003
            | VeronaErrorCode::ETREASURY004
            | VeronaErrorCode::ETREASURY005
            | VeronaErrorCode::ETREASURY006
            | VeronaErrorCode::ETREASURY007
            | VeronaErrorCode::ETREASURY008
            | VeronaErrorCode::ETREASURY009
            | VeronaErrorCode::ETREASURY010 => "TREASURY",
            VeronaErrorCode::EASSET001
            | VeronaErrorCode::EASSET002
            | VeronaErrorCode::EASSET003
            | VeronaErrorCode::EASSET004
            | VeronaErrorCode::EASSET005 => "ASSET",
            VeronaErrorCode::EBATCH001
            | VeronaErrorCode::EBATCH002
            | VeronaErrorCode::EBATCH003
            | VeronaErrorCode::EBATCH004 => "BATCH",
            VeronaErrorCode::ECONFIG001
            | VeronaErrorCode::ECONFIG002
            | VeronaErrorCode::ECONFIG003
            | VeronaErrorCode::ECONFIG004
            | VeronaErrorCode::ECONFIG005 => "CONFIG",
            VeronaErrorCode::ENETWORK001
            | VeronaErrorCode::ENETWORK002
            | VeronaErrorCode::ENETWORK003
            | VeronaErrorCode::ENETWORK004
            | VeronaErrorCode::ENETWORK005
            | VeronaErrorCode::ENETWORK006
            | VeronaErrorCode::ENETWORK007
            | VeronaErrorCode::ENETWORK008 => "NETWORK",
            VeronaErrorCode::ETX001 | VeronaErrorCode::ETX002 | VeronaErrorCode::ETX003 => "TX",
            VeronaErrorCode::EFAUCET001
            | VeronaErrorCode::EFAUCET002
            | VeronaErrorCode::EFAUCET003
            | VeronaErrorCode::EFAUCET004 => "FAUCET",
            VeronaErrorCode::EOAUTHCLIENT001
            | VeronaErrorCode::EOAUTHCLIENT002
            | VeronaErrorCode::EOAUTHCLIENT003
            | VeronaErrorCode::EOAUTHCLIENT004
            | VeronaErrorCode::EOAUTHCLIENT005
            | VeronaErrorCode::EOAUTHCLIENT006
            | VeronaErrorCode::EOAUTHCLIENT007
            | VeronaErrorCode::EOAUTHCLIENT008
            | VeronaErrorCode::EOAUTHCLIENT009
            | VeronaErrorCode::EOAUTHCLIENT010
            | VeronaErrorCode::EOAUTHCLIENT011
            | VeronaErrorCode::EOAUTHCLIENT012
            | VeronaErrorCode::EOAUTHCLIENT013
            | VeronaErrorCode::EOAUTHCLIENT014
            | VeronaErrorCode::EOAUTHCLIENT015
            | VeronaErrorCode::EOAUTHCLIENT016
            | VeronaErrorCode::EOAUTHCLIENT017
            | VeronaErrorCode::EOAUTHCLIENT018 => "OAUTH_CLIENT",
            VeronaErrorCode::EOAUTHCLIENT019 => "OAUTH_CLIENT",
        }
    }
}

impl fmt::Display for VeronaErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Detailed error information for JSON output
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorDetail {
    /// Error code (e.g., "ETREASURY001")
    pub code: VeronaErrorCode,
    /// Human-readable error message
    pub message: String,
    /// Actionable remediation hint
    pub hint: String,
    /// Whether this error can be retried
    pub retryable: bool,
    /// Optional source error for debugging
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

impl ErrorDetail {
    /// Create a new error detail with the given code
    pub fn new(code: VeronaErrorCode) -> Self {
        Self {
            code,
            message: code.message().to_string(),
            hint: code.hint().to_string(),
            retryable: code.is_retryable(),
            source: None,
        }
    }

    /// Create a new error detail with additional context
    pub fn with_context(code: VeronaErrorCode, context: impl Into<String>) -> Self {
        Self {
            code,
            message: format!("{}: {}", code.message(), context.into()),
            hint: code.hint().to_string(),
            retryable: code.is_retryable(),
            source: None,
        }
    }

    /// Create a new error detail with source information
    pub fn with_source(code: VeronaErrorCode, source: impl Into<String>) -> Self {
        Self {
            code,
            message: code.message().to_string(),
            hint: code.hint().to_string(),
            retryable: code.is_retryable(),
            source: Some(source.into()),
        }
    }

    /// Add source information to the error
    pub fn source(mut self, source: impl Into<String>) -> Self {
        self.source = Some(source.into());
        self
    }
}

/// Structured error output for JSON responses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    /// Always false for errors
    pub success: bool,
    /// Error details
    pub error: ErrorDetail,
}

impl ErrorResponse {
    /// Create a new error response
    pub fn new(code: VeronaErrorCode) -> Self {
        Self {
            success: false,
            error: ErrorDetail::new(code),
        }
    }

    /// Create a new error response with context
    pub fn with_context(code: VeronaErrorCode, context: impl Into<String>) -> Self {
        Self {
            success: false,
            error: ErrorDetail::with_context(code, context),
        }
    }

    /// Create a new error response with source
    pub fn with_source(code: VeronaErrorCode, source: impl Into<String>) -> Self {
        Self {
            success: false,
            error: ErrorDetail::with_source(code, source),
        }
    }
}

impl fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Error [{}]: {}\n\nHint: {}",
            self.error.code, self.error.message, self.error.hint
        )
    }
}

/// Main error type for Verona Agent Toolkit
#[derive(Debug, Error)]
pub enum VeronaError {
    /// Authentication error
    #[error("{0}")]
    Auth(#[source] AuthError),

    /// Treasury operation error
    #[error("{0}")]
    Treasury(#[source] TreasuryError),

    /// Asset builder error
    #[error("{0}")]
    Asset(#[source] AssetError),

    /// Batch operation error
    #[error("{0}")]
    Batch(#[source] BatchError),

    /// Configuration error
    #[error("{0}")]
    Config(#[source] ConfigError),

    /// Network/API error
    #[error("{0}")]
    Network(#[source] NetworkError),

    /// Transaction error
    #[error("{0}")]
    Tx(#[source] TxError),

    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Serialization error
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// OAuth client management error
    #[error("{0}")]
    OAuthClient(#[source] OAuthClientError),

    /// Generic error with code
    #[error("{message}")]
    Generic {
        code: VeronaErrorCode,
        message: String,
        hint: String,
    },
}

impl VeronaError {
    /// Get the error code for this error
    pub fn code(&self) -> VeronaErrorCode {
        match self {
            VeronaError::Auth(e) => e.code(),
            VeronaError::Treasury(e) => e.code(),
            VeronaError::Asset(e) => e.code(),
            VeronaError::Batch(e) => e.code(),
            VeronaError::Config(e) => e.code(),
            VeronaError::Network(e) => e.code(),
            VeronaError::Tx(e) => e.code(),
            VeronaError::OAuthClient(e) => e.code(),
            VeronaError::Io(_) => VeronaErrorCode::ECONFIG002,
            VeronaError::Serialization(_) => VeronaErrorCode::ECONFIG002,
            VeronaError::Generic { code, .. } => *code,
        }
    }

    /// Get the hint for this error
    pub fn hint(&self) -> String {
        match self {
            VeronaError::Auth(e) => e.hint(),
            VeronaError::Treasury(e) => e.hint(),
            VeronaError::Asset(e) => e.hint(),
            VeronaError::Batch(e) => e.hint(),
            VeronaError::Config(e) => e.hint(),
            VeronaError::Network(e) => e.hint(),
            VeronaError::Tx(e) => e.hint(),
            VeronaError::OAuthClient(e) => e.hint(),
            VeronaError::Io(_) => "Check file permissions and disk space".to_string(),
            VeronaError::Serialization(_) => "Check JSON format and structure".to_string(),
            VeronaError::Generic { hint, .. } => hint.clone(),
        }
    }

    /// Check if this error is retryable
    pub fn is_retryable(&self) -> bool {
        self.code().is_retryable()
    }

    /// Convert to error response for JSON output
    pub fn to_response(&self) -> ErrorResponse {
        ErrorResponse {
            success: false,
            error: ErrorDetail {
                code: self.code(),
                message: self.to_string(),
                hint: self.hint(),
                retryable: self.is_retryable(),
                source: std::error::Error::source(self).map(|s| s.to_string()),
            },
        }
    }
}

/// Authentication errors
#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Not authenticated: {0}")]
    NotAuthenticated(String),

    #[error("Token expired: {0}")]
    TokenExpired(String),

    #[error("Refresh token expired: {0}")]
    RefreshTokenExpired(String),

    #[error("Invalid credentials: {0}")]
    InvalidCredentials(String),

    #[error("OAuth2 callback failed: {0}")]
    CallbackFailed(String),

    #[error("PKCE verification failed: {0}")]
    PkceFailed(String),

    #[error("Authentication timeout: {0}")]
    Timeout(String),
}

impl AuthError {
    pub fn code(&self) -> VeronaErrorCode {
        match self {
            AuthError::NotAuthenticated(_) => VeronaErrorCode::EAUTH001,
            AuthError::TokenExpired(_) => VeronaErrorCode::EAUTH002,
            AuthError::RefreshTokenExpired(_) => VeronaErrorCode::EAUTH003,
            AuthError::InvalidCredentials(_) => VeronaErrorCode::EAUTH004,
            AuthError::CallbackFailed(_) => VeronaErrorCode::EAUTH005,
            AuthError::PkceFailed(_) => VeronaErrorCode::EAUTH006,
            AuthError::Timeout(_) => VeronaErrorCode::EAUTH007,
        }
    }

    pub fn hint(&self) -> String {
        self.code().hint().to_string()
    }
}

/// Treasury operation errors
#[derive(Debug, Error)]
pub enum TreasuryError {
    #[error("Treasury not found: {0}")]
    NotFound(String),

    #[error("Insufficient balance: {0}")]
    InsufficientBalance(String),

    #[error("Invalid treasury address: {0}")]
    InvalidAddress(String),

    #[error("Treasury creation failed: {0}")]
    CreationFailed(String),

    #[error("Treasury operation failed: {0}")]
    OperationFailed(String),

    #[error("Grant config not found: {0}")]
    GrantConfigNotFound(String),

    #[error("Fee config not found: {0}")]
    FeeConfigNotFound(String),

    #[error("Not authorized for treasury: {0}")]
    NotAuthorized(String),

    #[error("Treasury already exists: {0}")]
    AlreadyExists(String),

    #[error("Missing authorization input for grant config: {0}")]
    MissingAuthorizationInput(String),
}

impl TreasuryError {
    pub fn code(&self) -> VeronaErrorCode {
        match self {
            TreasuryError::NotFound(_) => VeronaErrorCode::ETREASURY001,
            TreasuryError::InsufficientBalance(_) => VeronaErrorCode::ETREASURY002,
            TreasuryError::InvalidAddress(_) => VeronaErrorCode::ETREASURY003,
            TreasuryError::CreationFailed(_) => VeronaErrorCode::ETREASURY004,
            TreasuryError::OperationFailed(_) => VeronaErrorCode::ETREASURY005,
            TreasuryError::GrantConfigNotFound(_) => VeronaErrorCode::ETREASURY006,
            TreasuryError::FeeConfigNotFound(_) => VeronaErrorCode::ETREASURY007,
            TreasuryError::NotAuthorized(_) => VeronaErrorCode::ETREASURY008,
            TreasuryError::AlreadyExists(_) => VeronaErrorCode::ETREASURY009,
            TreasuryError::MissingAuthorizationInput(_) => VeronaErrorCode::ETREASURY010,
        }
    }

    pub fn hint(&self) -> String {
        self.code().hint().to_string()
    }
}

/// Asset builder errors
#[derive(Debug, Error)]
pub enum AssetError {
    #[error("Invalid metadata: {0}")]
    InvalidMetadata(String),

    #[error("Asset creation failed: {0}")]
    CreationFailed(String),

    #[error("Invalid asset configuration: {0}")]
    InvalidConfiguration(String),

    #[error("Code ID not found: {0}")]
    CodeIdNotFound(String),

    #[error("Invalid schema: {0}")]
    InvalidSchema(String),
}

impl AssetError {
    pub fn code(&self) -> VeronaErrorCode {
        match self {
            AssetError::InvalidMetadata(_) => VeronaErrorCode::EASSET001,
            AssetError::CreationFailed(_) => VeronaErrorCode::EASSET002,
            AssetError::InvalidConfiguration(_) => VeronaErrorCode::EASSET003,
            AssetError::CodeIdNotFound(_) => VeronaErrorCode::EASSET004,
            AssetError::InvalidSchema(_) => VeronaErrorCode::EASSET005,
        }
    }

    pub fn hint(&self) -> String {
        self.code().hint().to_string()
    }
}

/// Batch operation errors
#[derive(Debug, Error)]
pub enum BatchError {
    #[error("Batch too large: {0} messages, maximum is 50")]
    TooLarge(usize),

    #[error("Batch execution failed: {0}")]
    ExecutionFailed(String),

    #[error("Partial batch failure: {0} succeeded, {1} failed")]
    PartialFailure(usize, usize),

    #[error("Invalid batch item at index {0}: {1}")]
    InvalidItem(usize, String),
}

impl BatchError {
    pub fn code(&self) -> VeronaErrorCode {
        match self {
            BatchError::TooLarge(_) => VeronaErrorCode::EBATCH001,
            BatchError::ExecutionFailed(_) => VeronaErrorCode::EBATCH002,
            BatchError::PartialFailure(_, _) => VeronaErrorCode::EBATCH003,
            BatchError::InvalidItem(_, _) => VeronaErrorCode::EBATCH004,
        }
    }

    pub fn hint(&self) -> String {
        self.code().hint().to_string()
    }
}

/// Configuration errors
#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Configuration not found: {0}")]
    NotFound(String),

    #[error("Invalid configuration: {0}")]
    Invalid(String),

    #[error("Encryption failed: {0}")]
    EncryptionFailed(String),

    #[error("Decryption failed: {0}")]
    DecryptionFailed(String),

    #[error("Network not found: {0}")]
    NetworkNotFound(String),
}

impl ConfigError {
    pub fn code(&self) -> VeronaErrorCode {
        match self {
            ConfigError::NotFound(_) => VeronaErrorCode::ECONFIG001,
            ConfigError::Invalid(_) => VeronaErrorCode::ECONFIG002,
            ConfigError::EncryptionFailed(_) => VeronaErrorCode::ECONFIG003,
            ConfigError::DecryptionFailed(_) => VeronaErrorCode::ECONFIG004,
            ConfigError::NetworkNotFound(_) => VeronaErrorCode::ECONFIG005,
        }
    }

    pub fn hint(&self) -> String {
        self.code().hint().to_string()
    }
}

/// Network/API errors
#[derive(Debug, Error)]
pub enum NetworkError {
    #[error("Connection timeout: {0}")]
    Timeout(String),

    #[error("Rate limited: {0}")]
    RateLimited(String),

    #[error("Service unavailable: {0}")]
    ServiceUnavailable(String),

    #[error("Invalid response: {0}")]
    InvalidResponse(String),

    #[error("Request failed: {0}")]
    RequestFailed(String),

    #[error("Connection refused: {0}")]
    ConnectionRefused(String),

    #[error("DNS resolution failed: {0}")]
    DnsFailed(String),

    #[error("TLS error: {0}")]
    TlsError(String),
}

impl NetworkError {
    pub fn code(&self) -> VeronaErrorCode {
        match self {
            NetworkError::Timeout(_) => VeronaErrorCode::ENETWORK001,
            NetworkError::RateLimited(_) => VeronaErrorCode::ENETWORK002,
            NetworkError::ServiceUnavailable(_) => VeronaErrorCode::ENETWORK003,
            NetworkError::InvalidResponse(_) => VeronaErrorCode::ENETWORK004,
            NetworkError::RequestFailed(_) => VeronaErrorCode::ENETWORK005,
            NetworkError::ConnectionRefused(_) => VeronaErrorCode::ENETWORK006,
            NetworkError::DnsFailed(_) => VeronaErrorCode::ENETWORK007,
            NetworkError::TlsError(_) => VeronaErrorCode::ENETWORK008,
        }
    }

    pub fn hint(&self) -> String {
        self.code().hint().to_string()
    }
}

/// Transaction operation errors
#[derive(Debug, Error)]
pub enum TxError {
    #[error("Transaction query failed: {0}")]
    QueryFailed(String),

    #[error("Transaction wait failed: {0}")]
    WaitFailed(String),

    #[error("Transaction timeout: {0}")]
    Timeout(String),
}

impl TxError {
    pub fn code(&self) -> VeronaErrorCode {
        match self {
            TxError::QueryFailed(_) => VeronaErrorCode::ETX001,
            TxError::WaitFailed(_) => VeronaErrorCode::ETX002,
            TxError::Timeout(_) => VeronaErrorCode::ETX003,
        }
    }

    pub fn hint(&self) -> String {
        self.code().hint().to_string()
    }
}

/// OAuth client management errors
#[derive(Debug, Error)]
pub enum OAuthClientError {
    #[error("Bad request: {code} - {message}")]
    BadRequest { code: String, message: String },

    #[error("Authentication required: {message}")]
    AuthenticationRequired { message: String },

    #[error("Insufficient scope: {message}")]
    InsufficientScope { message: String },

    #[error("Only owner allowed: {message}")]
    OnlyOwnerAllowed { message: String },

    #[error("Client not found: {client_id}")]
    ClientNotFound { client_id: String },

    #[error("Client extension not found: {client_id}")]
    ClientExtensionNotFound { client_id: String },

    #[error("Treasury not found: {address}")]
    TreasuryNotFound { address: String },

    #[error("User not found: {message}")]
    UserNotFound { message: String },

    #[error("Server error: {code} - {message}")]
    ServerError { code: String, message: String },

    #[error("Network error: {message}")]
    NetworkError { message: String },

    #[error("Invalid response: {message}")]
    InvalidResponse { message: String },

    #[error("Confirmation required: {message}")]
    ConfirmationRequired { message: String },
}

impl OAuthClientError {
    pub fn code(&self) -> VeronaErrorCode {
        match self {
            OAuthClientError::BadRequest { .. } => VeronaErrorCode::EOAUTHCLIENT001,
            OAuthClientError::AuthenticationRequired { .. } => VeronaErrorCode::EOAUTHCLIENT008,
            OAuthClientError::InsufficientScope { .. } => VeronaErrorCode::EOAUTHCLIENT010,
            OAuthClientError::OnlyOwnerAllowed { .. } => VeronaErrorCode::EOAUTHCLIENT011,
            OAuthClientError::ClientNotFound { .. } => VeronaErrorCode::EOAUTHCLIENT012,
            OAuthClientError::ClientExtensionNotFound { .. } => VeronaErrorCode::EOAUTHCLIENT013,
            OAuthClientError::TreasuryNotFound { .. } => VeronaErrorCode::EOAUTHCLIENT014,
            OAuthClientError::UserNotFound { .. } => VeronaErrorCode::EOAUTHCLIENT009,
            OAuthClientError::ServerError { .. } => VeronaErrorCode::EOAUTHCLIENT015,
            OAuthClientError::NetworkError { .. } => VeronaErrorCode::ENETWORK005,
            OAuthClientError::InvalidResponse { .. } => VeronaErrorCode::ENETWORK004,
            OAuthClientError::ConfirmationRequired { .. } => VeronaErrorCode::EOAUTHCLIENT019,
        }
    }

    pub fn hint(&self) -> String {
        match self {
            OAuthClientError::BadRequest { message, .. } => {
                format!("Check request parameters: {}", message)
            }
            OAuthClientError::AuthenticationRequired { .. } => {
                "Run 'verona-toolkit auth login' first".to_string()
            }
            OAuthClientError::InsufficientScope { .. } => {
                "Re-authorize with --dev-mode: verona-toolkit auth login --dev-mode".to_string()
            }
            OAuthClientError::OnlyOwnerAllowed { .. } => {
                "Only the client owner can perform this action".to_string()
            }
            OAuthClientError::ClientNotFound { .. } => {
                "Check the client ID and try again".to_string()
            }
            OAuthClientError::ClientExtensionNotFound { .. } => {
                "Check the client ID; extension may not exist".to_string()
            }
            OAuthClientError::TreasuryNotFound { .. } => {
                "Verify the treasury address is correct".to_string()
            }
            OAuthClientError::UserNotFound { .. } => {
                "Run 'verona-toolkit auth login' first".to_string()
            }
            OAuthClientError::ServerError { .. } => {
                "The server encountered an error. Please try again later.".to_string()
            }
            OAuthClientError::NetworkError { .. } => {
                "Check network connectivity and try again".to_string()
            }
            OAuthClientError::InvalidResponse { .. } => {
                "Server returned unexpected data. Check API version.".to_string()
            }
            OAuthClientError::ConfirmationRequired { .. } => {
                "Re-run the command with --force to confirm the destructive operation".to_string()
            }
        }
    }
}

// Implement From traits for easy conversion

impl From<AuthError> for VeronaError {
    fn from(e: AuthError) -> Self {
        VeronaError::Auth(e)
    }
}

impl From<TreasuryError> for VeronaError {
    fn from(e: TreasuryError) -> Self {
        VeronaError::Treasury(e)
    }
}

impl From<AssetError> for VeronaError {
    fn from(e: AssetError) -> Self {
        VeronaError::Asset(e)
    }
}

impl From<BatchError> for VeronaError {
    fn from(e: BatchError) -> Self {
        VeronaError::Batch(e)
    }
}

impl From<ConfigError> for VeronaError {
    fn from(e: ConfigError) -> Self {
        VeronaError::Config(e)
    }
}

impl From<NetworkError> for VeronaError {
    fn from(e: NetworkError) -> Self {
        VeronaError::Network(e)
    }
}

impl From<TxError> for VeronaError {
    fn from(e: TxError) -> Self {
        VeronaError::Tx(e)
    }
}

impl From<OAuthClientError> for VeronaError {
    fn from(e: OAuthClientError) -> Self {
        VeronaError::OAuthClient(e)
    }
}

// Implement From for crate::treasury::encoding::EncodingError
// This is in a separate impl block to handle the cross-module import
impl From<crate::treasury::encoding::EncodingError> for VeronaError {
    fn from(e: crate::treasury::encoding::EncodingError) -> Self {
        VeronaError::Treasury(TreasuryError::OperationFailed(e.to_string()))
    }
}

// Implement From<anyhow::Error> for backward compatibility with modules not yet migrated
impl From<anyhow::Error> for VeronaError {
    fn from(e: anyhow::Error) -> Self {
        // Try to extract a more specific error type from the message
        let err_str = e.to_string();

        // Check for common auth errors
        if err_str.contains("Not authenticated") || err_str.contains("Please login") {
            return VeronaError::Auth(AuthError::NotAuthenticated(err_str));
        }
        if err_str.contains("Token expired") || err_str.contains("refresh") {
            return VeronaError::Auth(AuthError::TokenExpired(err_str));
        }
        if err_str.contains("Refresh token expired") {
            return VeronaError::Auth(AuthError::RefreshTokenExpired(err_str));
        }

        // Default to a generic error
        VeronaError::Generic {
            code: VeronaErrorCode::ECONFIG002,
            message: err_str,
            hint: "Check the error message for details".to_string(),
        }
    }
}

/// Result type alias for VeronaError
pub type VeronaResult<T> = std::result::Result<T, VeronaError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_code_message() {
        assert_eq!(VeronaErrorCode::EAUTH001.message(), "Not authenticated");
        assert_eq!(
            VeronaErrorCode::ETREASURY001.message(),
            "Treasury not found"
        );
        assert_eq!(VeronaErrorCode::ENETWORK001.message(), "Connection timeout");
    }

    #[test]
    fn test_error_code_hint() {
        assert_eq!(
            VeronaErrorCode::EAUTH001.hint(),
            "Run 'verona-toolkit auth login' first"
        );
        assert_eq!(
            VeronaErrorCode::ETREASURY001.hint(),
            "Run 'verona-toolkit treasury list' to see available treasuries"
        );
    }

    #[test]
    fn test_error_code_retryable() {
        // Network errors are retryable
        assert!(VeronaErrorCode::ENETWORK001.is_retryable());
        assert!(VeronaErrorCode::ENETWORK002.is_retryable());
        assert!(VeronaErrorCode::ENETWORK003.is_retryable());

        // Token expired is retryable (after refresh)
        assert!(VeronaErrorCode::EAUTH002.is_retryable());

        // Most other errors are not retryable
        assert!(!VeronaErrorCode::EAUTH001.is_retryable());
        assert!(!VeronaErrorCode::ETREASURY001.is_retryable());
    }

    #[test]
    fn test_error_detail_new() {
        let detail = ErrorDetail::new(VeronaErrorCode::ETREASURY001);
        assert_eq!(detail.code, VeronaErrorCode::ETREASURY001);
        assert_eq!(detail.message, "Treasury not found");
        assert!(!detail.retryable);
        assert!(detail.source.is_none());
    }

    #[test]
    fn test_error_detail_with_context() {
        let detail = ErrorDetail::with_context(VeronaErrorCode::ETREASURY001, "xion1abc123");
        assert_eq!(detail.message, "Treasury not found: xion1abc123");
        assert_eq!(
            detail.hint,
            "Run 'verona-toolkit treasury list' to see available treasuries"
        );
    }

    #[test]
    fn test_error_response() {
        let response =
            ErrorResponse::with_context(VeronaErrorCode::ETREASURY002, "Required: 1000000uxion");
        assert!(!response.success);
        assert_eq!(response.error.code, VeronaErrorCode::ETREASURY002);
        assert!(response.error.message.contains("Insufficient balance"));
    }

    #[test]
    fn test_error_response_display() {
        let response = ErrorResponse::new(VeronaErrorCode::EAUTH001);
        let display = format!("{}", response);
        assert!(display.contains("Error [EAUTH001]"));
        assert!(display.contains("Not authenticated"));
        assert!(display.contains("Hint:"));
    }

    #[test]
    fn test_verona_error_from_auth_error() {
        let auth_err = AuthError::NotAuthenticated("Please login".to_string());
        let xion_err: VeronaError = auth_err.into();
        assert_eq!(xion_err.code(), VeronaErrorCode::EAUTH001);
    }

    #[test]
    fn test_verona_error_from_network_error() {
        let net_err = NetworkError::Timeout("Request timed out".to_string());
        let xion_err: VeronaError = net_err.into();
        assert_eq!(xion_err.code(), VeronaErrorCode::ENETWORK001);
        assert!(xion_err.is_retryable());
    }

    #[test]
    fn test_verona_error_to_response() {
        let err = VeronaError::from(AuthError::TokenExpired("Access token expired".to_string()));
        let response = err.to_response();
        assert!(!response.success);
        assert_eq!(response.error.code, VeronaErrorCode::EAUTH002);
        assert!(response.error.retryable);
    }

    #[test]
    fn test_error_code_serialization() {
        let code = VeronaErrorCode::ETREASURY001;
        let json = serde_json::to_string(&code).unwrap();
        assert_eq!(json, "\"ETREASURY001\"");

        let decoded: VeronaErrorCode = serde_json::from_str(&json).unwrap();
        assert_eq!(decoded, code);
    }

    #[test]
    fn test_error_response_serialization() {
        let response = ErrorResponse::new(VeronaErrorCode::EAUTH001);
        let json = serde_json::to_string(&response).unwrap();

        assert!(json.contains("\"success\":false"));
        assert!(json.contains("\"code\":\"EAUTH001\""));
        assert!(json.contains("\"retryable\":false"));
    }
}
