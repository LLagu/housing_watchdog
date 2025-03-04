use std::path::Path;
use std::{env, fs};

pub(crate) enum PrevSessionFileType {
    ConfigPath,
    ScrapedContent(String),
}

pub(crate) fn get_prev_session_file_path(file_type: PrevSessionFileType) -> String {
    let prev_session_path = Path::new(env::current_dir().unwrap().as_path()).join("prev_session/");
    match fs::create_dir_all(&prev_session_path) {
        Ok(_) => {}
        Err(e) => match e.kind() {
            _ => panic!("Failed retrieving the previous session folder path{}", e),
        },
    };

    let prev_session_path = prev_session_path.to_str().unwrap();
    match file_type {
        PrevSessionFileType::ConfigPath => {
            let prev_config_file_name = String::from("prev_config.txt");
            let mut sanitized_return = format!("{prev_session_path}{prev_config_file_name}");
            sanitized_return.retain(|c| !c.is_ascii_whitespace());
            sanitized_return
        }
        PrevSessionFileType::ScrapedContent(url) => {
            let file_name = str::replace(&*url, "/", "_") + ".txt";
            let mut sanitized_return = format!("{prev_session_path}{file_name}");
            sanitized_return.retain(|c| !c.is_ascii_whitespace());
            sanitized_return
        }
    }
}
