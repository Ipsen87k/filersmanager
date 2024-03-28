use std::{collections::HashMap, fs::{self, Metadata},  path::{Path, PathBuf}};

use iced::{executor, widget::{button, column, container,row, text, text_input, Column}, Application, Command, Element, Renderer, Settings, Theme};

const ONE_KELO_BYTE:f32 = 1024.0;
const SIX_DIGITS:u64 = 999999;
const NINE_DIGITS:u64 = 999999999;

fn main() -> iced::Result{
    AppState::run(Settings::default())
}

#[derive(Debug,Clone)]
struct AppState{
    path:Option<PathBuf>,
    path_input_value:String,
    file_info_map:HashMap<PathBuf,u64>,
}

#[derive(Debug,Clone)]
pub enum Message {
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
                file_info_map:HashMap::new(),
            },
            Command::none(),
        )
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::OnInput(value)=>{
                self.path = Some(PathBuf::from(&value));
                self.path_input_value = value;
            }
            Message::Run=>{
                if let Ok(entries) = fs::read_dir(self.path.as_ref().unwrap()){
                    for entry in entries{
                        if let  Ok(entry) = entry{
                            let path = entry.path();
                            if let Ok(meta) = entry.metadata(){
                                if meta.is_file(){
                                    self.file_info_map.insert(path, meta.len());
                                }else if meta.is_dir(){
                                    let total_size = serach_file(&path);
                                    self.file_info_map.insert(path, total_size);
                                }
                            }
                        }
                    }
                }
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

        if self.file_info_map.is_empty(){
            container(top_control).into()
        }else{
            let file_infos = widget_list(self.conv_map_to_vec()); 
            container(
                column!(
                    top_control,
                    file_infos
                )
            ).into()
        }
    }
}

impl AppState {
    fn conv_map_to_vec(&self)->Vec<String>{
        let mut total_size = 0;
        let mut ret = self.file_info_map
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
        ret.push(format!("totalsize    {:.2}GB",gb_size));
        ret
    }
}

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