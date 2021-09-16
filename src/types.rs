pub type RelativeFilePath = String;
pub type AbsoluteFilePath = String;
pub type ContentId = i32;
pub type LastModifiedTimestamp = i32;
pub type EventType = String;

#[derive(PartialEq)]
pub enum ContentType {
    File,
    HtmlDocument,
    Folder,
}

impl ContentType {
    pub fn from_str(str_: &str) -> Option<Self> {
        match str_ {
            "file" => Some(Self::File),
            "html-document" => Some(Self::HtmlDocument),
            "folder" => Some(Self::Folder),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            ContentType::File => "file".to_string(),
            ContentType::HtmlDocument => "html-document".to_string(),
            ContentType::Folder => "folder".to_string(),
        }
    }
}

#[derive(PartialEq)]
pub enum RemoteEventType {
    Created,
    Modified,
    Deleted,
}

impl RemoteEventType {
    pub fn from_str(str_: &str) -> Option<Self> {
        match str_ {
            "content.modified.html-document" => Some(Self::Modified),
            "content.modified.file" => Some(Self::Modified),
            "content.modified.folder" => Some(Self::Modified),
            "content.created.html-document" => Some(Self::Created),
            "content.created.file" => Some(Self::Created),
            "content.created.folder" => Some(Self::Created),
            "content.deleted.html-document" => Some(Self::Deleted),
            "content.deleted.file" => Some(Self::Deleted),
            "content.deleted.folder" => Some(Self::Deleted),
            "content.undeleted.html-document" => Some(Self::Created),
            "content.undeleted.file" => Some(Self::Created),
            "content.undeleted.folder" => Some(Self::Created),
            _ => None,
        }
    }
}
