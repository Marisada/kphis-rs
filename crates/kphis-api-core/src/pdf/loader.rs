use std::path::{Path, PathBuf};
use typst_library::diag::{EcoString, FileError};

use crate::state::{ApiState, UserState};

// root path is crate root (where Cargo.lock located)
// 1. api: we search for left-most 'files' path excepts 'snippets' [jsons, statics, templates, typsts], the rest are apis
// 2. files: root path is "/volume/pwa/"
// 3. thumbnail image: root path is "/volume/thumbs/"
// 4. full image: root path is "/volume/images/"
// ex. "statics/picture/krut.svg" in Typst points to "/volume/pwa/statics/picture/krut.svg"
// *NOTE* this function must be sync (Typst NOT support async)
// please update kphis-api-pdf/tests/create_reports.rs::load_file()
pub fn load_file(path: PathBuf, app: &Option<ApiState>, user: &Option<UserState>) -> Result<Vec<u8>, FileError> {
    let path = strip_slash(&path);
    if ["jsons", "statics", "templates", "typsts"].iter().any(|key| path.starts_with(key)) {
        let mut p = PathBuf::new();
        p.push("volume");
        p.push("pwa");
        p.push(path);
        std::fs::read(&p).map_err(|e| FileError::from_io(e, &p))
    } else if path.starts_with("thumbs") || path.starts_with("images") {
        let mut p = PathBuf::new();
        p.push("volume");
        p.push(path);
        std::fs::read(&p).map_err(|e| FileError::from_io(e, &p))
    } else if let (Some(app_state), Some(user_state)) = (app, user) {
        // /api/.., /customs/.., /img/..
        // read `kphis-api-pdf::json_data::get_json_data()` for more details
        app_state.get_api(&path, user_state)
    } else {
        Err(FileError::Other(Some(EcoString::from("State Not Found"))))
    }
}

/// strip '../', './', '/' from path<br>
pub fn strip_slash(path: &Path) -> PathBuf {
    let s = path.strip_prefix("../").unwrap_or(path);
    let s = path.strip_prefix("./").unwrap_or(s);
    s.strip_prefix("/").unwrap_or(s).to_path_buf()
}
