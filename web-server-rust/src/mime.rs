use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum MimeType {
    TextPlain,
    #[default]
    TextHtml,
    ImageJpg,
    ImagePng,
    VideoMp4,
    ApplicationJson,
    ApplicationPdf,
}

impl fmt::Display for MimeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TextPlain => write!(f, "text/plain"),
            Self::TextHtml => write!(f, "text/html"),
            Self::ImageJpg => write!(f, "image/jpeg"),
            Self::ImagePng => write!(f, "image/png"),
            Self::VideoMp4 => write!(f, "video/mp4"),
            Self::ApplicationJson => write!(f, "application/json"),
            Self::ApplicationPdf => write!(f, "application/pdf"),
        }
    }
}
