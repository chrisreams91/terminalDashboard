use crate::requests;

use std::env;
use std::fs::File;
use std::io::BufReader;
use std::net::TcpListener;

use hyper::{client::HttpConnector, http::Error, Client};
use hyper_tls::HttpsConnector;

use crate::requests::Tag;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Application {
    pub name: String,
    pub local_port: u16,
    pub gitlab_id: u16,
}

pub fn get_local_port_status(port: u16) -> bool {
    match TcpListener::bind(("0.0.0.0", port)) {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub fn generate_local_application_info() -> Result<Vec<Application>, std::io::Error> {
    let applications_json = if cfg!(debug_assertions) {
        let mut current_executable_path = env::current_exe().unwrap();
        // backing out of target dir
        current_executable_path.pop();
        current_executable_path.pop();
        current_executable_path.pop();

        format!(
            "{}/src/Applications.json",
            current_executable_path.to_str().unwrap()
        )
    } else {
        String::from("./Applications.json")
    };

    let file = File::open(applications_json)?;
    let reader = BufReader::new(file);
    let applications: Vec<Application> = serde_json::from_reader(reader)?;

    Ok(applications)
}

pub async fn get_app_version_info(
    gitlab_token: &String,
    client: &Client<HttpsConnector<HttpConnector>>,
) -> Result<String, Error> {
    // let tag_names_and_dates = requests::get_tag_names(gitlab_token, client).await?;
    // custom logic to parse git tags for recent version information

    Ok(String::from(
        "Production: version - date    Beta: version - date",
    ))
}

// used to parse timestamps for tags above
fn format_date(timestamp: String) -> String {
    // 2020-09-04T14:02:01.000-05:00
    let month = &timestamp[5..7];
    let formatted_month = if month.chars().next().unwrap() == '0' {
        month.replace("0", "")
    } else {
        month.to_owned()
    };

    let day = &timestamp[8..10];
    let formatted_day = if day.chars().next().unwrap() == '0' {
        day.replace("0", "")
    } else {
        day.to_owned()
    };

    String::from(format!("{}/{}", formatted_month, formatted_day))
}
