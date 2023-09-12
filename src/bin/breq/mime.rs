use std::collections::HashMap;
use strum_macros::{Display, EnumString};

// this is not a complete list, but I don't expect
// to encounter the other types.  If I do, I'll add them.
// `MType` to indicate the "main" type.
#[derive(Debug, Display, PartialEq, EnumString)]
pub(crate) enum MType {
    // discrete types
    #[strum(serialize = "application")]
    Application,
    #[strum(serialize = "text")]
    Text,
    #[strum(serialize = "image")]
    Image,
    #[strum(serialize = "audio")]
    Audio,
    #[strum(serialize = "video")]
    Video,
    // multipart types
    #[strum(serialize = "multipart")]
    Multipart,
}

#[derive(Debug)]
pub(crate) struct MimeType {
    pub(crate) type_: MType,
    pub(crate) subtype: String,
}

#[derive(Debug)]
pub(crate) struct ContentType {
    pub(crate) media_type: MimeType,
    pub(crate) parameters: HashMap<String, String>,
}

impl ContentType {
    pub(crate) fn from_header_value(content_type: &str) -> Result<Self, String> {
        let mut parameters = std::collections::HashMap::new();

        let mut parts = content_type.split(';').map(|s| s.trim());

        let mut media_type = parts
            .next()
            .expect("media type should be the first directive before a ';'")
            .split('/')
            .map(|s| s.trim());

        let type_ = match media_type
            .next()
            .expect("parsed media type to be a '/'-separated directive")
        {
            "application" => MType::Application,
            "text" => MType::Text,
            "image" => MType::Image,
            "audio" => MType::Audio,
            "video" => MType::Video,
            "multipart" => MType::Multipart,
            t => return Err(format!("Unknown MIME type: {}", t)),
        };

        for param in parts {
            let mut param_parts = param.splitn(2, '=').map(|s| s.trim());
            let (key, val) = (param_parts.next().unwrap(), param_parts.next().unwrap());
            parameters.insert(key.to_string(), val.to_string());
        }

        Ok(ContentType {
            media_type: MimeType {
                type_,
                subtype: String::from(media_type.next().unwrap()),
            },
            parameters,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::ContentType;

    #[test]
    fn test_basic_parsing() {
        let mime = ContentType::from_header_value("text/html; charset=utf-8").unwrap();
        assert_eq!(mime.media_type.type_.to_string(), "text");
        assert_eq!(mime.media_type.subtype, "html");
        assert_eq!(
            mime.parameters
                .get("charset")
                .expect("charset should be in parameters"),
            "utf-8"
        );
    }

    #[test]
    fn test_multiple_parameters() {
        let mime = ContentType::from_header_value(
            "multipart/form-data; boundary=something; charset=utf-8",
        )
        .unwrap();
        assert_eq!(mime.media_type.type_.to_string(), "multipart");
        assert_eq!(mime.media_type.subtype, "form-data");
        assert_eq!(mime.parameters.get("boundary").unwrap(), "something");
        assert_eq!(mime.parameters.get("charset").unwrap(), "utf-8");
    }

    #[test]
    #[should_panic]
    fn test_missing_subtype() {
        ContentType::from_header_value("text").unwrap();
    }

    #[test]
    fn test_no_parameters() {
        let mime = ContentType::from_header_value("text/html").unwrap();
        assert_eq!(mime.media_type.type_.to_string(), "text");
        assert_eq!(mime.media_type.subtype, "html");
        assert!(mime.parameters.is_empty());
    }

    #[test]
    fn test_extra_whitespace() {
        let mime = ContentType::from_header_value("  text  /  html  ;  charset = utf-8  ").unwrap();
        assert_eq!(mime.media_type.type_.to_string(), "text");
        assert_eq!(mime.media_type.subtype, "html");
        assert_eq!(mime.parameters.get("charset").unwrap(), "utf-8");
    }
}
