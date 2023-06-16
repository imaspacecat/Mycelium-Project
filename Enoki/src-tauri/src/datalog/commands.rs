use wpilog::log::DatalogEntryResponse;

use crate::error::{log_result, EnokiError};

use super::handler::open_datalog;


#[tauri::command]
pub fn read_datalog(path: String) -> Result<Vec<DatalogEntryResponse>, EnokiError> {
    let datalog = log_result(open_datalog(path.into()))?;
    let entries = datalog.get_all_entries();
    Ok(entries)
}