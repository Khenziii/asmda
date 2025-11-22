use crate::{archivers::Archiver, utils::time::get_current_path_friendly_formatted_date};

pub fn get_backup_path_for_archiver<T: Archiver>(archiver: T) -> String {
    format!("{}/{}", &archiver.get_identificator().as_str(), get_current_path_friendly_formatted_date())
}
