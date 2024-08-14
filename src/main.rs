#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
use std::{
    cmp::Ordering, env, fs::{self, Permissions}, path::{Path, PathBuf}, time::SystemTime, vec
};

use filersmanager::{
    error::Error,
    file::{self, open_folder, output_folder_infos},
    icon,
    widget::{FileTableRow, TableState},
    Message,
};
use iced::{
    executor, mouse, widget::{
        button, column, container, responsive, row, scrollable, space, text, text_input, tooltip
    }, Application, Command, Element, Font, Length, Settings, Theme
};
use iced_table::table;

const ONE_KELO_BYTE: f32 = 1024.0;
const FOUR_DIGITS:u64 = 9999;
const SIX_DIGITS: u64 = 999999;
const NINE_DIGITS: u64 = 999999999;

fn main() -> iced::Result {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("error"));

    let icon =
        iced::window::icon::from_file_data(include_bytes!("../assets/icon/frsm_icon.jpg"), None);
    if let Ok(icon) = icon {
        log::info!("iconあり");
        AppState::run(Settings {
            fonts: vec![include_bytes!("../assets/font/iced-image.ttf")
                .as_slice()
                .into()],
            default_font: Font::MONOSPACE,
            window: iced::window::Settings {
                icon: Some(icon),
                ..Default::default()
            },
            ..Default::default()
        })
    } else {
        log::info!("iconなし");
        AppState::run(Settings {
            fonts: vec![include_bytes!("../assets/font/iced-image.ttf")
                .as_slice()
                .into()],
            default_font: Font::MONOSPACE,
            ..Default::default()
        })
    }
}

struct AppState {
    path: Option<PathBuf>,
    path_input_value: String,
    total_size:String,
    file_info_vec: Vec<(PathBuf, u64,Permissions,Option<SystemTime>)>,
    table_state: TableState,
}

impl Application for AppState {
    type Executor = executor::Default;

    type Message = Message;

    type Flags = ();

    type Theme = Theme;

    fn title(&self) -> String {
        String::from("FileRsManager")
    }

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Message>) {
        (
            Self {
                path: Some(PathBuf::from("")),
                path_input_value: String::new(),
                total_size:String::new(),
                file_info_vec: vec![],
                table_state: TableState::new(None),
            },
            Command::none(),
        )
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::OnInput(value) => {
                //"c:/temp"=>c:/temp として扱う
                if value.starts_with('"') && value.ends_with('"') {
                    let value = &value[1..value.len() - 1];
                    self.path_input_value = String::from(value);
                } else {
                    self.path_input_value = value;
                }
            }
            Message::FileSearch => {
                //二回目初期化
                self.file_info_vec.clear();
                if let Some(path) = self.path.as_ref() {
                    if path.exists() {
                        if self.path_input_value.is_empty() {
                            let path = PathBuf::from(path);
                            return Command::perform(
                                filesize_collect(path),
                                Message::FileSerachedConvert,
                            );
                        } else {
                            let path = PathBuf::from(&self.path_input_value);
                            if path.exists() {
                                self.path = Some(path.clone());
                                return Command::perform(
                                    filesize_collect(path),
                                    Message::FileSerachedConvert,
                                );
                            }
                        }
                    }
                }
            }
            Message::FileSerachedConvert(mut value) => {
                value.sort_by(|a, b| a.1.ancestor_cmp(&b.1));
                self.file_info_vec = value.clone();
                return Command::perform(conv_fileinfovec_to_strvec(value), Message::FileSeached);
            }
            Message::FileSeached((file_table_rows,total_size)) => {
                self.table_state.set_rows(file_table_rows);
                self.total_size = total_size;
            }
            Message::OpenFolder => {
                return Command::perform(open_folder(), Message::FolderOpened);
            }
            Message::FolderOpened(path) => {
                if let Some(path) = path {
                    self.path = Some(path.clone());
                    return Command::perform(filesize_collect(path), Message::FileSerachedConvert);
                }
            }
            Message::OutputFileInfos => {
                let text = "self.content.text()";
                if text.is_empty() {
                    return Command::none();
                } else {
                    return Command::perform(output_folder_infos(text), Message::ErrorDialogShow);
                }
            }
            Message::EventOccured(event) => match event {
                iced::Event::Keyboard(_) => {}
                iced::Event::Mouse(mouse_event) => match mouse_event {
                    mouse::Event::CursorEntered => {}
                    mouse::Event::CursorLeft => {}
                    mouse::Event::CursorMoved { position: _ } => {}
                    mouse::Event::ButtonPressed(btn) => {
                        if btn == iced::mouse::Button::Right {
                        }
                    }
                    mouse::Event::ButtonReleased(_) => {}
                    mouse::Event::WheelScrolled { delta: _ } => {}
                },
                iced::Event::Window(_, _) => {}
                iced::Event::Touch(_) => {}
            },
            Message::ErrorDialogShow(result) => {
                if let Err(e) = result {
                    log::error!("{}", e);
                    return Command::perform(file::error_dialog_show(e), Message::None);
                }
            }
            Message::None(_null) => {}
            Message::SyncHeader(offset) => {
                return Command::batch(vec![scrollable::scroll_to(
                    self.table_state.header.clone(),
                    offset,
                )])
            }
            Message::Resizing(index,offset)=>{
                if let Some(column) = self.table_state.columns.get_mut(index){
                    column.resize_offset = Some(offset);
                }
            }
            Message::Resized=>{
                self.table_state.columns.iter_mut().for_each(|column|{
                    if let Some(offset) = column.resize_offset.take(){
                        column.width+=offset;
                    }
            })}
            Message::Delete(index)=>{
                let path = self.table_state.rows[index].get_filepath();
                self.table_state.rows.remove(index);
                return Command::perform(file::remove_file_dialog(path), Message::ErrorDialogShow);
            }
        }
        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Message> {
        let path_input =
            text_input("please input pass", &self.path_input_value).on_input(Message::OnInput);
        let run_button = button(container(icon::search_icon())).on_press(Message::FileSearch);

        let top_control = row!(path_input, run_button);
        let sub_func = row!(
            create_tooltrip(
                icon::open_folder_icon(),
                "開きたいフォルダを選択",
                Some(Message::OpenFolder)
            ),
            create_tooltrip(icon::output_icon(), "出力", Some(Message::OutputFileInfos)),
        );

        let table = responsive(|size| {
            table(
                self.table_state.header.clone(),
                self.table_state.body.clone(),
                &self.table_state.columns,
                &self.table_state.rows,
                Message::SyncHeader,
            ).on_column_resize(Message::Resizing,Message::Resized)
            .min_width(size.width).into()
        });
        let control = column!(top_control, sub_func, table,);
        if self.file_info_vec.is_empty() {
            container(control).into()
        } else {
            container(column!(
                control,
                row!(
                    text(self.path.as_ref().unwrap().display()),
                    space::Space::with_width(Length::Fill),
                    text(format!("total:{}",&self.total_size)),
                )
            ))
            .into()
        }
    }

    fn subscription(&self) -> iced::Subscription<Message> {
        iced::event::listen().map(Message::EventOccured)
    }

    fn theme(&self) -> Self::Theme {
        Theme::Dark
    }
}


async fn conv_fileinfovec_to_strvec(vec: Vec<(PathBuf, u64,Permissions,Option<SystemTime>)>) ->(Vec<FileTableRow>,String) {
    let mut total_size = 0;
    let fileinfo_str_vec = vec
        .into_iter()
        .map(|(filename,size,perm,time)| {
            let size_str = calc_unit(size);
            total_size += size;

            FileTableRow::generate(filename, size_str,time)
        })
        .collect();

    (fileinfo_str_vec,calc_unit(total_size))
}

fn calc_unit(size:u64)->String{
    let fsize= size as f32;
    if size <= SIX_DIGITS{
        let kb_size = fsize / ONE_KELO_BYTE;
        format!("{:.2}kb",kb_size)
    }else if size <= NINE_DIGITS{
        let mb_size = fsize / (ONE_KELO_BYTE * ONE_KELO_BYTE);
        format!("{:.2}MB",mb_size)
    }else{
        let gb_size = fsize / (ONE_KELO_BYTE * ONE_KELO_BYTE * ONE_KELO_BYTE);
        format!("{:.2}GB",gb_size)
    }
}

fn serach_file<P>(path: P) -> u64
where
    P: AsRef<Path>,
{
    let mut fsize = 0;
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Ok(meta) = entry.metadata() {
                    if meta.is_file() {
                        fsize += meta.len();
                    } else if meta.is_dir() {
                        fsize += serach_file(entry.path());
                    }
                }
            }
        }
    }
    return fsize;
}
async fn filesize_collect<P>(path: P) -> Vec<(PathBuf, u64,Permissions,Option<SystemTime>)>
where
    P: AsRef<Path>,
{
    let mut file_info_vec = vec![];
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if let Ok(meta) = entry.metadata() {
                    let accessed_time = match meta.accessed() {
                        Ok(time) => Some(time),
                        Err(_) => None,
                    };
                    if meta.is_file() {
                        file_info_vec.push((path, meta.len(),meta.permissions(),accessed_time));
                    } else if meta.is_dir() {
                        let total_size = serach_file(&path);
                        file_info_vec.push((path, total_size,meta.permissions(),accessed_time));
                    }
                }
            }
        }
    }
    file_info_vec
}

fn create_tooltrip<'a>(
    content: impl Into<Element<'a, Message>>,
    label: &'a str,
    on_press: Option<Message>,
) -> Element<'a, Message> {
    let btn = button(container(content));

    if let Some(on_press) = on_press {
        tooltip(
            btn.on_press(on_press),
            label,
            tooltip::Position::FollowCursor,
        )
        .into()
    } else {
        btn.into()
    }
}

trait CmpExtension {
    fn ancestor_cmp(&self, other: &u64) -> Ordering;
}

impl CmpExtension for u64 {
    fn ancestor_cmp(&self, other: &u64) -> Ordering {
        if *self < *other {
            Ordering::Greater
        } else if *self == *other {
            Ordering::Equal
        } else {
            Ordering::Less
        }
    }
}
