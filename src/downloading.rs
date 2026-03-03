use directories::ProjectDirs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use tokio::fs::create_dir_all;

//this function returns Path where dada shude be stored
pub fn getting_path() -> Option<PathBuf> {
    ProjectDirs::from("com", "Detalexpl", "TuiWeather")
        .map(|proj_dirs| proj_dirs.data_local_dir().to_path_buf())
}
//this function download file worldcities.csv needed to find out location
pub async fn downloading_data(path: PathBuf) -> Result<(), String> {
    //dbg!(&path);
    let reqs = reqwest::Client::new().get("https://raw.githubusercontent.com/Detalexpl/TuiWeather/refs/heads/master/worldcities.csv")
        .send().await.map_err(|e| e.to_string())?.text().await.map_err(|e| e.to_string())?;

    create_dir_all(&path).await.map_err(|e| e.to_string())?;
    let mut out = File::create(path.join("cities.csv")).map_err(|e| e.to_string())?;
    out.write_all(reqs.as_bytes()).map_err(|e| e.to_string())?;

    Ok(())
}
