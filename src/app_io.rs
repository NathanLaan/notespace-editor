//!
//! Notespace-Editor
//!
//! IO components and functions for the application.
//!

use rfd::*;
use std::io::ErrorKind;
use std::path::{PathBuf};
use std::sync::Arc;
use std::result::Result;

///
/// Load the specified `path` into a `String`.
///
/// Uses Tokio async IO functions.
///
pub async fn async_open_file_from_path(file_path: PathBuf) -> Result<(PathBuf,Arc<String>), AppIOError> {
    let file_contents = tokio::fs::read_to_string(&file_path)
        .await
        .map(Arc::new)
        .map_err(|error| error.kind())
        .map_err(AppIOError::IOFailedError)?;

    Ok((file_path,file_contents))
}

///
/// Let the user choose a file to open.
///
/// Uses Rusty File Dialogs (RFD).
///
pub async fn async_open_file_from_dialog() -> Result<(PathBuf, Arc<String>), AppIOError> {
    let file_handle = AsyncFileDialog::new()
        .set_title("Select file to open...")
        .pick_file()
        .await
        .ok_or(AppIOError::FileDialogClosedError)?;
    let file_path = PathBuf::from(file_handle);
    async_open_file_from_path(file_path.to_owned()).await
}

pub async fn async_save_file_to_path(file_path: Option<PathBuf>, file_contents: String) -> Result<(PathBuf), AppIOError> {
    let path = if let Some(file_path) = file_path {
        file_path
    } else {
        rfd::AsyncFileDialog::new()
            .set_title("Choose a file name...")
            .save_file()
            .await
            .ok_or(AppIOError::FileDialogClosedError)
            .map(|handle| handle.path().to_owned())?
    };
    tokio::fs::write(&path, &file_contents)
        .await
        .map_err(|error| AppIOError::IOFailedError(error.kind()))?;

    Ok(path)
}

///
/// IO error definitions.
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum AppIOError {
    FileDialogClosedError,
    IOFailedError(ErrorKind),
}

impl std::fmt::Display for AppIOError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppIOError::FileDialogClosedError =>
                write!(f, "AppIOError {}", "FileDialogClosedError"),
            AppIOError::IOFailedError(err) =>
                write!(f, "AppIOError {}", err),
        }
    }
}
