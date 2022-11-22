use actix_files::NamedFile;
use actix_web::HttpRequest;

use crate::application::error::HomeworkError;

pub async fn favicon(_request: HttpRequest) -> Result<NamedFile, HomeworkError> {
    Ok(NamedFile::open("./static_dist/icon_main.svg")?)
}
