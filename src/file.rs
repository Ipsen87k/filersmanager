use std::{ error::Error, path::{Path, PathBuf}};

pub async fn open_folder()->Option<PathBuf>{
    let picked_path = rfd::AsyncFileDialog::new()
        .set_title("開きたいフォルダ")
        .pick_folder()
        .await;

    if let Some(picked_path) = picked_path{
        println!("selected folder path = {}",picked_path.path().display());
        Some(picked_path.path().into())
    }else{
        None
    }
}

pub async fn output_folder_infos(content:String)->PathBuf{
    let path = rfd::AsyncFileDialog::new()
        .save_file()
        .await
        .as_ref()
        .map(rfd::FileHandle::path)
        .map(Path::to_owned);

    if let Some(path) = path{
        let r = tokio::fs::write(&path, content)
            .await;
        if let Ok(_r)=r{

        }
    }
    PathBuf::new()
}