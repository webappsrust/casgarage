//! S3 XML response builders

use serde::Serialize;

/// Build S3 error response
pub fn error_response(code: &str, message: &str) -> String {
    format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<Error>
    <Code>{}</Code>
    <Message>{}</Message>
</Error>"#,
        code, message
    )
}
