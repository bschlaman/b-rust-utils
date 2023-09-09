#[cfg(test)]
use crate::mime::ContentType;

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
    let mime =
        ContentType::from_header_value("multipart/form-data; boundary=something; charset=utf-8")
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
