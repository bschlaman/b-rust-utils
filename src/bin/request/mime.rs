use std::collections::HashMap;

// this is not a complete list, but I don't expect
// to encounter the other types.  If I do, I'll add them.
// `MType` to indicate the "main" type.
#[derive(Debug)]
enum MType {
    // discrete types
    Application,
    Text,
    Image,
    Audio,
    Video,
    // multipart types
    Multipart,
}

#[derive(Debug)]
struct MimeType {
    type_: MType,
    subtype: String,
}

#[derive(Debug)]
pub(crate) struct ContentType {
    media_type: MimeType,
    parameters: HashMap<String, String>,
}

impl ContentType {
    pub(crate) fn from_header_value(content_type: &str) -> Result<Self, String> {
        let parts = content_type.split(';').collect::<Vec<&str>>();
        let media_type = parts[0].split('/').collect::<Vec<&str>>();
        let type_ = match media_type[0] {
            "application" => MType::Application,
            "text" => MType::Text,
            "image" => MType::Image,
            "audio" => MType::Audio,
            "video" => MType::Video,
            "multipart" => MType::Multipart,
            t => return Err(format!("Unknown MIME type: {}", t)),
        };

        Ok(ContentType {
            media_type: MimeType {
                type_,
                subtype: String::from(""),
            },
            parameters: HashMap::new(),
        })
    }
}
