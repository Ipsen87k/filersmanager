use std::{cmp::Ordering, collections::HashMap, error::Error, fs, path::{Path, PathBuf}};


const DESKTOP_PATH:&str=r"C:\Users\aagao\OneDrive\デスクトップ";
type O=Result<(),Box<dyn Error>>;

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

#[test]
fn test_sort()->O{
    let mut hash = HashMap::new();
    if let Ok(entries) = fs::read_dir(DESKTOP_PATH){
        for entry in entries{
            if let Ok(entry) = entry{
                if let Ok(meta) = entry.metadata() {
                    if meta.is_file(){
                        hash.insert(entry.path(),meta.len());
                    }else if meta.is_dir(){
                        let total_size = serach_file(&entry.path());
                        hash.insert(entry.path(), total_size);
                    }
                }
            }
        }
    }
    let mut temp_vec:Vec<(&PathBuf,&u64)> = hash.iter().collect();
    temp_vec.sort_by(|a,b| a.1.ances_cmp(b.1));
    println!("{:?}",temp_vec);
    Ok(())
}

trait MExtension {
    fn ances_cmp(&self,other:&u64)->Ordering;
}

impl MExtension for u64 {
    fn ances_cmp(&self,other:&u64)->Ordering {
        if *self < *other {Ordering::Greater}
        else if *self == *other {Ordering::Equal}
        else {Ordering::Less}
    }
}
