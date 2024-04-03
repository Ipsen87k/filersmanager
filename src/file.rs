use std::{error::Error, path::PathBuf};



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