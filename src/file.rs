use std::{path::{Path, PathBuf}};

use crate::{error::Error, Null};

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

pub async fn output_folder_infos(content:&str)->Result<(),Error>{
    let path = rfd::AsyncFileDialog::new()
        .save_file()
        .await
        .as_ref()
        .map(rfd::FileHandle::path)
        .map(Path::to_owned);

    if let Some(path) = path{
        tokio::fs::write(&path, content)
            .await
            .map_err(|e| Error::AsyncTokioIoError(e.kind()))?;
    }
    Ok(())
}

pub async fn remove_file_dialog(path:PathBuf)->Result<(),Error>{
    let dialog_handle = rfd::AsyncMessageDialog::new()
        .set_buttons(rfd::MessageButtons::YesNoCancel)
        .set_description(format!("{}を削除しますか",path.file_name().unwrap().to_str().unwrap()))
        .set_level(rfd::MessageLevel::Warning)
        .set_title("ファイル削除")
        .show()
        .await;

    if dialog_handle == rfd::MessageDialogResult::Yes{
        if path.exists(){
            if path.is_file(){
                tokio::fs::remove_file(path).await.map_err(|e| {
                    Error::AsyncTokioIoError(e.kind())
                })?;
            }else if path.is_dir() {
                tokio::fs::remove_dir_all(path).await.map_err(|e| {
                    Error::AsyncTokioIoError(e.kind())
                })?;
            }
        }
    }

    Ok(())
}

pub async fn error_dialog_show(e:Error)->Null{
    let _ = rfd::AsyncMessageDialog::new()
        .set_level(rfd::MessageLevel::Error)
        .set_description(format!("{}",e))
        .set_title("Error")
        .show()
        .await;

    Null{}
}