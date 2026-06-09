use std::path::Path;

pub fn file_to_data_url(path: impl AsRef<Path>) -> anyhow::Result<String> {
    file_icon::file_to_data_url(path).map_err(|error| anyhow::anyhow!(error.to_string()))
}

pub fn get_app_name(path: impl AsRef<Path>) -> anyhow::Result<String> {
    file_icon::get_app_name_with_options(path, file_icon::FileIconOptions::default())
        .map_err(|error| anyhow::anyhow!(error.to_string()))
}
