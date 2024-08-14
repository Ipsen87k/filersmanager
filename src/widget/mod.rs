use std::{fmt::Display, fs::Permissions, path::PathBuf, time::SystemTime};

use chrono::{DateTime, Local};
use iced::{
    widget::{button, container, pick_list, row, scrollable, text, Space,},
    Element, Length, Renderer, Theme,
};
use iced_table::table;

use crate::{file::EntryType, icon::{file_icon, folder_icon}, Message};

pub struct TableState {
    pub columns: Vec<TableColumn>,
    pub rows: Vec<FileTableRow>,
    pub header: scrollable::Id,
    pub body: scrollable::Id,
    pub footer: scrollable::Id,
}

impl TableState {
    pub fn new(item: Option<Vec<FileTableRow>>) -> Self {
        if let Some(item) = item {
            Self {
                columns: vec![
                    TableColumn::new(ColumnKind::Index),
                    TableColumn::new(ColumnKind::FileName),
                    TableColumn::new(ColumnKind::Size),
                    TableColumn::new(ColumnKind::ModifiedTime),
                    TableColumn::new(ColumnKind::Delete),
                ],
                rows: item,
                header: scrollable::Id::unique(),
                body: scrollable::Id::unique(),
                footer: scrollable::Id::unique(),
            }
        } else {
            Self {
                columns: vec![
                    TableColumn::new(ColumnKind::Index),
                    TableColumn::new(ColumnKind::FileName),
                    TableColumn::new(ColumnKind::Size),
                    TableColumn::new(ColumnKind::ModifiedTime),
                    TableColumn::new(ColumnKind::Delete),
                ],
                rows: vec![],
                header: scrollable::Id::unique(),
                body: scrollable::Id::unique(),
                footer: scrollable::Id::unique(),
            }
        }
    }

    pub fn set_rows(&mut self, rows: Vec<FileTableRow>) {
        self.rows = rows;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Category {
    FileName,
    Size,
    DateModified,
    Perm,
}

impl Category {
    const ALL: &'static [Self] = &[Self::FileName, Self::Size, Self::DateModified, Self::Perm];
}

impl Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Category::FileName => "Name",
            Category::Size => "Size",
            Category::DateModified => "Date Modified",
            Category::Perm => "Perm",
        }
        .fmt(f)
    }
}

pub enum ColumnKind {
    Index,
    FileName,
    Size,
    ModifiedTime,
    Delete,
}

#[derive(Debug,Clone)]
pub struct FileTableRow {
    filename: PathBuf,
    size: String,
    time:String,
    entry_type:EntryType,
}

impl FileTableRow {
    pub fn generate(filename: PathBuf, size: String,time:Option<SystemTime>) -> Self {
        let time = match time {
            Some(time) => {
                let datetime :DateTime<Local>=  DateTime::from(time);
                datetime.format("%Y/%m/%d %H:%M").to_string()
            },
            None => "".into(),
        };
        let entry_type = EntryType::generate(&filename);

        Self {
            filename: filename,
            size: size,
            time:time,
            entry_type:entry_type,
        }
    }

    pub fn get_filepath(&mut self)->PathBuf{
        self.filename.clone()
    }
}

impl ToString for FileTableRow {
    fn to_string(&self) -> String {
        let filename=self.filename.file_name().unwrap().to_str().unwrap();
        format!("{}\t{}\t{}",filename,&self.size,&self.time)
    }
}

pub struct TableColumn {
    kind: ColumnKind,
    pub width: f32,
    pub resize_offset: Option<f32>,
}

impl TableColumn {
    fn new(kind: ColumnKind) -> Self {
        let width = match kind {
            ColumnKind::Index => 60.0,
            ColumnKind::FileName => 600.0,
            ColumnKind::Size => 90.0,
            ColumnKind::ModifiedTime => 140.0,
            ColumnKind::Delete => 100.0,
        };

        Self {
            kind,
            width,
            resize_offset: None,
        }
    }
}

impl<'a> table::Column<'a, Message, Theme, Renderer> for TableColumn {
    type Row = FileTableRow;

    fn header(&'a self, _col_index: usize) -> Element<'a, Message, Theme, Renderer> {
        let content = match self.kind {
            ColumnKind::Index => "Index",
            ColumnKind::FileName => "Name",
            ColumnKind::Size => "Size",
            ColumnKind::ModifiedTime => "Access Time",
            ColumnKind::Delete => "Delete",
        };

        container(text(content)).height(24).center_y().into()
    }

    fn cell(
        &'a self,
        _col_index: usize,
        row_index: usize,
        row: &'a Self::Row,
    ) -> Element<'a, Message, Theme, Renderer> {
        let content: Element<_> = match self.kind {
            ColumnKind::Index => text(row_index).into(),
            ColumnKind::FileName => {
                let icon = match row.entry_type {
                    EntryType::File => file_icon(),
                    EntryType::Dir => folder_icon(),
                };

                row!(icon,Space::with_width(Length::Fixed(10.)),text(&row.filename.file_name().unwrap().to_str().unwrap()),).into()
            },
            ColumnKind::Size => text(&row.size).into(),
            ColumnKind::ModifiedTime => text(&row.time).into(),
            ColumnKind::Delete => button("delete").on_press(Message::Delete(row_index)).into(),
        };

        container(content)
            .width(Length::Fill)
            .height(32)
            .center_y()
            .into()
    }

    fn width(&self) -> f32 {
        self.width
    }

    fn resize_offset(&self) -> Option<f32> {
        self.resize_offset
    }
}
