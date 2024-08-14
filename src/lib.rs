use std::{fs::Permissions, path::PathBuf, time::SystemTime};

use iced::widget::{scrollable, text_editor};
use widget::{Category, FileTableRow};

pub mod icon;
pub mod file;
pub mod error;
pub mod widget;


#[derive(Debug,Clone)]
pub struct Null;

#[derive(Debug,Clone)]
pub enum Message {
    OnInput(String),
    OpenFolder,
    FolderOpened(Option<PathBuf>),
    OutputFileInfos,
    FileSearch,
    FileSerachedConvert(Vec<(PathBuf,u64,Permissions,Option<SystemTime>)>),
    FileSeached((Vec<FileTableRow>,String)),
    EventOccured(iced::event::Event),
    ErrorDialogShow(Result<(),error::Error>),
    None(Null),
    SyncHeader(scrollable::AbsoluteOffset),
    Resizing(usize,f32),
    Resized,
    Delete(usize),
}