use std::{cmp::Ordering, fs::{self,}, path::{Path, PathBuf}, vec};

use filersmanager::{file::{self, open_folder, output_folder_infos}, icon,error::Error};
use iced::{executor, mouse, widget::{button, column, container, row, text, text_editor, text_input, tooltip, Column}, window, Application, Command, Element, Font, Length, Renderer, Settings, Theme};

const ONE_KELO_BYTE:f32 = 1024.0;
const SIX_DIGITS:u64 = 999999;
const NINE_DIGITS:u64 = 999999999;

fn main() -> iced::Result{
    AppState::run(Settings{
        fonts:vec![include_bytes!("../font/iced-image.ttf").as_slice().into()],
        default_font:Font::MONOSPACE,
        ..Default::default()
    })
}

// pub enum Error {
//     IoError(io::Error)
// }

struct AppState{
    path:Option<PathBuf>,
    path_input_value:String,
    file_info_vec:Vec<(PathBuf,u64)>,
    content:text_editor::Content,
}


#[derive(Debug,Clone)]
pub enum Message {
    TextEditorOnAction(text_editor::Action),
    OnInput(String),
    OpenFolder,
    FolderOpened(Option<PathBuf>),
    OutputFileInfos,
    FileSearch,
    FileSerachedConvert(Vec<(PathBuf,u64)>),
    FileSeached(Vec<String>),
    EventOccured(iced::event::Event),
    ErrorDialogShow(Result<(),Error>),
    None(filersmanager::Null),
}


impl Application for AppState{
    type Executor = executor::Default;

    type Message = Message;

    type Flags = ();

    type Theme = Theme;

    fn title(&self) -> String {
        String::from("FileRsManager")
    }

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Message>) {
        (
            Self{
                path:Some(PathBuf::from(r"C:\Users\aagao\OneDrive\デスクトップ")),
                path_input_value:String::new(),
                file_info_vec:vec![],
                content:text_editor::Content::new(),
            },
            Command::none(),
        )
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::OnInput(value)=>{
                //"c:/temp"=>c:/temp として扱う
                if value.starts_with('"') && value.ends_with('"'){
                    let value = &value[1..value.len()-1];
                    self.path_input_value = String::from(value);
                }else{
                    self.path_input_value = value;
                }
            }
            Message::FileSearch=>{
                //二回目初期化
                self.file_info_vec.clear();
                if let Some(path) = self.path.as_ref() {
                    if path.exists(){
                        if self.path_input_value.is_empty(){
                            let path = PathBuf::from(path);
                            return Command::perform(filesize_collect(path), Message::FileSerachedConvert);
                        }else{
                            let path = PathBuf::from(&self.path_input_value);
                            if path.exists(){
                                self.path = Some(path.clone());
                                return Command::perform(filesize_collect(path), Message::FileSerachedConvert);
                            }
                        }
                    }
                }
            }
            Message::FileSerachedConvert(mut value)=>{
                value.sort_by(|a,b| a.1.ancestor_cmp(&b.1));
                self.file_info_vec = value.clone();
                return Command::perform(conv_fileinfovec_to_strvec(value), Message::FileSeached);
            }
            Message::FileSeached(value)=>{
                self.content = text_editor::Content::with_text(value.join("\n").as_str());
            }
            Message::TextEditorOnAction(action)=>{
                if let text_editor::Action::Edit(edit) = action.clone(){
                    //fallthrough Pasteのみaction起こす
                    match edit {
                        text_editor::Edit::Paste(_v)=>{
                        }
                        _=>{
                            return Command::none();
                        }
                    }
                }
                self.content.perform(action);
            }
            Message::OpenFolder=>{
                return Command::perform(open_folder(), Message::FolderOpened);
            }
            Message::FolderOpened(path)=>{
                if let Some(path) = path{
                    self.path = Some(path);
                }
            }
            Message::OutputFileInfos=>{
                let text = self.content.text();
                if text.is_empty(){
                    return Command::none();
                }else{
                    return Command::perform(output_folder_infos(text),Message::ErrorDialogShow)
                }
            }
            Message::EventOccured(event)=>{
                match event {
                    iced::Event::Keyboard(_) => {

                    },
                    iced::Event::Mouse(mouse_event) => {
                        match mouse_event {
                            mouse::Event::CursorEntered => {},
                            mouse::Event::CursorLeft => {},
                            mouse::Event::CursorMoved { position } => {
                            },
                            mouse::Event::ButtonPressed(btn) => {
                                if btn == iced::mouse::Button::Right{
                                    self.content.perform(text_editor::Action::SelectLine);
                                    if let Some(selected_file) = self.content.selection(){
                                        let filename = selected_file.split('\t').collect::<Vec<&str>>();
                                        let filename = String::from(filename[0]);
                                        let path = self.path.as_ref().unwrap().join(filename);
                                        return Command::perform(file::remove_file_dialog(path), Message::ErrorDialogShow);
                                    }
                                }
                            },
                            mouse::Event::ButtonReleased(_) => {},
                            mouse::Event::WheelScrolled { delta } => {
                            },
                        }
                    },
                    iced::Event::Window(_, _) => {
                    },
                    iced::Event::Touch(_) => {

                    },
                }
            }
            Message::ErrorDialogShow(result)=>{
                if let Err(e) = result  {
                    return Command::perform(file::error_dialog_show(e), Message::None);
                }
            }
            Message::None(_null)=>{
                
            }
        }
        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Message> {
        let path_input = text_input("please input pass", &self.path_input_value).on_input(Message::OnInput);
        let run_button = button("run").on_press(Message::FileSearch);

        let top_control = row!(path_input,run_button);
        let sub_func = row!(
            create_tooltrip(icon::open_folder_icon(), "開きたいフォルダを選択", Some(Message::OpenFolder)),
            create_tooltrip(icon::output_icon(), "出力", Some(Message::OutputFileInfos)),
        );

        let control = column!(
            top_control,
            sub_func
        );
        if self.file_info_vec.is_empty(){
            container(control).into()
        }else{
            container(
                column!(
                    control,
                    text_editor(&self.content)
                        .height(Length::Fill)
                        .on_action(Message::TextEditorOnAction)
                        ,
                    text(self.path.as_ref().unwrap().display()),
                )
            ).into()
        }
    }

    fn subscription(&self) -> iced::Subscription<Message> {
        iced::event::listen().map(Message::EventOccured)
    }

    fn theme(&self) -> Self::Theme {
        Theme::Dark
    }
}

impl AppState {
    fn conv_map_to_vec(&self)->Vec<String>{
        let mut total_size = 0;
        let mut ret = self.file_info_vec
            .iter()
            .map(|(k,v)| {
                let fsize = *v as f32;
                total_size+=*v;
                if *v< SIX_DIGITS{
                    format!("{}\t{}bytes",k.file_name().unwrap().to_str().unwrap(),v)
                }else if SIX_DIGITS < *v && *v < NINE_DIGITS{
                    let mb_size = fsize / (ONE_KELO_BYTE*ONE_KELO_BYTE);
                    format!("{}\t{:.2}MB",k.file_name().unwrap().to_str().unwrap(),mb_size)
                }else{
                    let gb_size = fsize/(ONE_KELO_BYTE*ONE_KELO_BYTE*ONE_KELO_BYTE);
                    
                    format!("{}\t{:.2}GB",k.file_name().unwrap().to_str().unwrap(),gb_size)
                }
            })
            .collect::<Vec<String>>();
        
        let gb_size = total_size as f32 / (ONE_KELO_BYTE*ONE_KELO_BYTE*ONE_KELO_BYTE);
        ret.insert(0,format!("totalsize\t\t{:.2}GB",gb_size));
        ret
    }
}

#[allow(dead_code)]
fn widget_list<'a>(targets:Vec<String>)->Column<'a,Message>{
    let mut vec:Vec<Element<'_, Message, Theme, Renderer>> = vec![];
    
    for (_idx,str) in targets.iter().enumerate(){
        
        let tx = text(str);
        
        vec.push(tx.into());
    }
    Column::from_vec(vec)
}

async fn conv_fileinfovec_to_strvec(vec:Vec<(PathBuf,u64)>)->Vec<String>{
    let mut total_size = 0;
    let mut fileinfo_str_vec= vec
        .iter()
        .map(|(k,v)| {
            let fsize = *v as f32;
            total_size+=*v;
            if *v< SIX_DIGITS{
                format!("{}\t{}bytes",k.file_name().unwrap().to_str().unwrap(),v)
            }else if SIX_DIGITS < *v && *v < NINE_DIGITS{
                let mb_size = fsize / (ONE_KELO_BYTE*ONE_KELO_BYTE);
                format!("{}\t{:.2}MB",k.file_name().unwrap().to_str().unwrap(),mb_size)
            }else{
                let gb_size = fsize/(ONE_KELO_BYTE*ONE_KELO_BYTE*ONE_KELO_BYTE);
                
                format!("{}\t{:.2}GB",k.file_name().unwrap().to_str().unwrap(),gb_size)
            }
        })
        .collect::<Vec<String>>();
    
    let gb_size = total_size as f32 / (ONE_KELO_BYTE*ONE_KELO_BYTE*ONE_KELO_BYTE);
    fileinfo_str_vec.insert(0,format!("totalsize\t\t{:.2}GB",gb_size));
    fileinfo_str_vec
}

fn serach_file<P>(path:P)->u64
where
    P:AsRef<Path>{
        
        let mut fsize = 0;
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries{
                if let Ok(entry) = entry{
                    if let Ok(meta) = entry.metadata(){
                        if meta.is_file(){
                            fsize+=meta.len();
                        }else if meta.is_dir() {
                            fsize+=serach_file(entry.path());
                        }
                    }
                }
            }
        }
        return fsize;
    }
async fn filesize_collect<P>(path:P)->Vec<(PathBuf,u64)>
where
    P:AsRef<Path>{
        let mut file_info_vec = vec![];
        if let Ok(entries) = fs::read_dir(path){
            for entry in entries{
                if let  Ok(entry) = entry{
                    let path = entry.path();
                    if let Ok(meta) = entry.metadata(){
                        if meta.is_file(){
                            file_info_vec.push((path,meta.len()));
                            //self.file_info_map.insert(path, meta.len());
                        }else if meta.is_dir(){
                            let total_size = serach_file(&path);
                            file_info_vec.push((path,total_size));
                            //self.file_info_map.insert(path, total_size);
                        }
                    }
                }
            }
        }
        file_info_vec
    }


fn create_tooltrip<'a>(content:impl Into<Element<'a,Message>>,label:&'a str,on_press:Option<Message>)->Element<'a,Message>{
    let btn = button(container(content));

    if let Some(on_press) = on_press{
        tooltip(
            btn.on_press(on_press),
            label,
            tooltip::Position::FollowCursor,
        )
        .into()
    }else{
        btn.into()
    }

}

trait CmpExtension {
    fn ancestor_cmp(&self,other:&u64)->Ordering;
}

impl CmpExtension for u64 {
    fn ancestor_cmp(&self,other:&u64)->Ordering {
        if *self < *other {Ordering::Greater}
        else if *self == *other {Ordering::Equal}
        else {Ordering::Less}
    }
}
