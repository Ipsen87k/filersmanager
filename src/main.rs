use std::{cmp::Ordering, fs::{self, Metadata}, path::{Path, PathBuf}};

use iced::{executor,  widget::{button, column, container, row, text, text_editor, text_input, Column},  Application, Command, Element, Font, Length, Renderer, Settings, Theme};

const ONE_KELO_BYTE:f32 = 1024.0;
const SIX_DIGITS:u64 = 999999;
const NINE_DIGITS:u64 = 999999999;

fn main() -> iced::Result{
    AppState::run(Settings{
        fonts:vec![include_bytes!("../font/FreeSerif.ttf").as_slice().into()],
        default_font:Font::MONOSPACE,
        ..Default::default()
    })
}


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
    Run,
    None,
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
                    self.path = Some(PathBuf::from(value));
                    self.path_input_value = String::from(value);
                }else{
                    self.path = Some(PathBuf::from(&value));
                    self.path_input_value = value;
                }
            }
            Message::Run=>{
                if let Ok(entries) = fs::read_dir(self.path.as_ref().unwrap()){
                    for entry in entries{
                        if let  Ok(entry) = entry{
                            let path = entry.path();
                            if let Ok(meta) = entry.metadata(){
                                if meta.is_file(){
                                    self.file_info_vec.push((path,meta.len()));
                                    //self.file_info_map.insert(path, meta.len());
                                }else if meta.is_dir(){
                                    let total_size = serach_file(&path);
                                    self.file_info_vec.push((path,total_size));
                                    //self.file_info_map.insert(path, total_size);
                                }
                            }
                        }
                    }
                }
                self.file_info_vec.sort_by(|a,b| a.1.ancestor_cmp(&b.1));
                let file_info_str = self.conv_map_to_vec();
                self.content = text_editor::Content::with_text(file_info_str.join("\n").as_str());
            }
            Message::TextEditorOnAction(action)=>{
                self.content.perform(action);

            }
            Message::None=>{
                
            }
        }
        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Message> {
        let path_input = text_input("please input pass", &self.path_input_value).on_input(Message::OnInput);
        let run_button = button("run").on_press(Message::Run);

        let top_control = row!(path_input,run_button);

        if self.file_info_vec.is_empty(){
            container(top_control).into()
        }else{
            // let file_infos_str = self.conv_map_to_vec();
            // self.content = text_editor::Content::with_text(file_infos_str.join("\n").as_str());
            container(
                column!(
                    top_control,
                    text_editor(&self.content)
                        .height(Length::Fill)
                        .on_action(Message::TextEditorOnAction)
                        ,
                )
            ).into()
        }
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
                    format!("{}     {}bytes",k.file_name().unwrap().to_str().unwrap(),v)
                }else if SIX_DIGITS < *v && *v < NINE_DIGITS{
                    let mb_size = fsize / (ONE_KELO_BYTE*ONE_KELO_BYTE);
                    format!("{}     {:.2}MB",k.file_name().unwrap().to_str().unwrap(),mb_size)
                }else{
                    let gb_size = fsize/(ONE_KELO_BYTE*ONE_KELO_BYTE*ONE_KELO_BYTE);
                    
                    format!("{}     {:.2}GB",k.file_name().unwrap().to_str().unwrap(),gb_size)
                }
            })
            .collect::<Vec<String>>();
        
        let gb_size = total_size as f32 / (ONE_KELO_BYTE*ONE_KELO_BYTE*ONE_KELO_BYTE);
        ret.insert(0,format!("totalsize    {:.2}GB",gb_size));
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
