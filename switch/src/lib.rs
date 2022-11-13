use std::thread::{self};
use include_dir::{include_dir, Dir};
use skyline_web::*;
use serde::*;
use serde_json::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use include_dir::File;
use skyline::info::get_program_id;
use std::fs;
use std::path::Path;
use nx_request_handler::*;

static WEB_DIR: Dir = include_dir!("../web/build/");

pub fn is_emulator() -> bool {
    return unsafe { skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as u64 } == 0x8004000;
}

#[skyline::main(name = "switch")]
pub fn main() {
    println!("beginning skyline react template");
    let browser_thread = thread::spawn(move || {
        // create the webpage
        let mut page = Webpage::new();
        
        println!("files:");
        WEB_DIR.files().for_each(|file| println!("file: {}", file.path().display()));

        println!("dirs:");
        WEB_DIR.dirs().for_each(|file| println!("file: {}", file.path().display()));

        // parse the react-built manifest
        let manifest_file = match WEB_DIR.get_file("asset-manifest.json") {
            Some(value) => value,
            None => {
                println!("manifest not found!");
                return;
            }
        };
        let manifest_str = match manifest_file.contents_utf8() {
            Some(value) => value,
            None => {
                println!("manifest content reading failed!");
                return;
            }
        };
        let manifest = match serde_json::from_str::<serde_json::Value>(manifest_str) {
            Ok(value) => value,
            Err(e) => {
                println!("parsing as json failed! Error: {}", e);
                return;
            }
        };
        let assets_list = match manifest.get("files") {
            Some(value) => value,
            None => {
                println!("getting files field failed!");
                return;
            }
        };
        let assets = match assets_list.as_object()  {
            Some(value) => value,
            None => {
                println!("parsing as object failed!");
                return;
            }
        };
        

        let mut files: HashMap<String, &File> = HashMap::new();

        // for each asset, add it to the webpage
        assets.iter().for_each(|entry| {
            let file_path = format!("{}", entry.1.to_string().trim_matches('\"').trim_start_matches('/'));
            let file = match WEB_DIR.get_file(&file_path) {
                Some(f) => f,
                None => {
                    println!("Could not find file: {}", file_path);
                    return;
                }
            };
            files.insert(file_path, file);
        });

        let program_id = get_program_id();
        let htdocs_dir = "skyline-react-template";
        let folder_path = Path::new("sd:/atmosphere/contents")
                .join(&format!("{:016X}", program_id))
                .join(&format!("manual_html/html-document/{}.htdocs/", htdocs_dir));

        for key in files.keys() {
            let contents = match files.get(key)  {
                Some(value) => value.contents(),
                None => {
                    println!("key not found: {}", key);
                    return;
                }
            };
            page.file(key, contents);
            let fullpath = Path::join(&folder_path, key);
            println!("adding file: {}", fullpath.display());
            match fs::create_dir_all(fullpath.parent().unwrap()) {
                Ok(_) => {},
                Err(e) => {println!("Error while making file path: {:?}", e); return;}
            }
        }
        
        let session = match page
            .htdocs_dir(htdocs_dir)
            .background(skyline_web::Background::Default)
            .boot_display(skyline_web::BootDisplay::Black)
            .open_session(skyline_web::Visibility::InitiallyHidden) {
                Ok(s) => s,
                Err(e) => {println!("Error opening page: {:?}", e);return;}
            };
        session.show();
        
        println!("starting engine.");
        RequestEngine::new(session)
            .register_defaults()
            .start();
    });

    // End thread so match can actually start
    browser_thread.join();
}
