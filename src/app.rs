use crate::requests::{
    get_issues_for_active_sprint, get_merges_for_selected_application, get_recent_pipelines, Issue,
    Merge, Pipeline,
};
use crate::util::{generate_local_application_info, get_app_version_info, Application};

use hyper::{client::HttpConnector, http::Error, Client};
use hyper_tls::HttpsConnector;

use std::env;

#[derive(Debug)]
pub struct App {
    pub local_applications: Vec<Application>,
    pub gitlab_applications: Vec<Application>,
    pub selected_list_item: Application,
    pub selected_application: Application,
    pub selected_application_merges: Vec<Merge>,
    pub selected_application_pipelines: Vec<Pipeline>,
    pub gitlab_access_token: String,
    pub formatted_version_numbers: String,
    pub jira_access_token: String,
    pub active_sprint_name: String,
    pub active_sprint_issues: Vec<Issue>,
    pub party_mode: bool,
    pub logo_color_change_index: u16,
}

impl App {
    pub fn new() -> App {
        let apps: Vec<Application> = generate_local_application_info().unwrap();
        let initial_selection = &apps.clone()[0];
        let local_apps = Vec::into_iter(apps.clone())
            .filter(|app| if app.local_port != 0 { true } else { false })
            .collect();
        let gitlab_apps = Vec::into_iter(apps.clone())
            .filter(|app| if app.gitlab_id != 0 { true } else { false })
            .collect();

        let args: Vec<String> = env::args().collect();
        let gitlab_token = args[1].to_owned();
        let jira_token = args[2].to_owned();

        App {
            local_applications: local_apps,
            gitlab_applications: gitlab_apps,
            selected_application: initial_selection.to_owned(),
            selected_list_item: initial_selection.to_owned(),
            selected_application_merges: Vec::new(),
            selected_application_pipelines: Vec::new(),
            gitlab_access_token: gitlab_token,
            jira_access_token: jira_token,
            formatted_version_numbers: String::from("Not currently on VPN"),
            active_sprint_name: String::from("Sprint"),
            active_sprint_issues: Vec::new(),
            party_mode: false,
            logo_color_change_index: 0,
        }
    }

    pub async fn fetch_gitlab_data(&mut self, client: &Client<HttpsConnector<HttpConnector>>) {
        let version_info = get_app_version_info(&self.gitlab_access_token, client)
            .await
            .unwrap();
        let merges = get_merges_for_selected_application(
            &self.selected_application.gitlab_id,
            &self.gitlab_access_token,
            client,
        )
        .await
        .unwrap();
        let pipelines = get_recent_pipelines(
            &self.selected_application.gitlab_id,
            &self.gitlab_access_token,
            client,
        )
        .await
        .unwrap();

        self.formatted_version_numbers = version_info;
        self.selected_application_merges = merges;
        self.selected_application_pipelines = pipelines;
    }

    pub async fn fetch_all_data(
        &mut self,
        client: &Client<HttpsConnector<HttpConnector>>,
    ) -> Result<(), Error> {
        let version_info = get_app_version_info(&self.gitlab_access_token, client).await?;
        let merges = get_merges_for_selected_application(
            &self.selected_application.gitlab_id,
            &self.gitlab_access_token,
            client,
        )
        .await?;
        let pipelines = get_recent_pipelines(
            &self.selected_application.gitlab_id,
            &self.gitlab_access_token,
            client,
        )
        .await?;
        let issues = get_issues_for_active_sprint(self, client).await?;

        self.formatted_version_numbers = version_info;
        self.selected_application_merges = merges;
        self.selected_application_pipelines = pipelines;
        self.active_sprint_issues = issues;

        Ok(())
    }

    pub fn select_next_gitlab_application(&mut self) {
        let apps = self.gitlab_applications.to_owned();
        let current_selection = self.selected_list_item.to_owned();
        let index = apps
            .iter()
            .position(|app| app.name == current_selection.name)
            .unwrap();

        let new_index = if index == apps.len() - 1 {
            0
        } else {
            index + 1
        };
        let next_selection = apps.get(new_index).unwrap();
        self.selected_list_item = next_selection.to_owned();
    }

    pub fn select_previous_gitlab_application(&mut self) {
        let apps = self.gitlab_applications.to_owned();
        let current_selection = self.selected_list_item.to_owned();
        let index = apps
            .iter()
            .position(|app| app.name == current_selection.name)
            .unwrap();

        let new_index = if index == 0 {
            apps.len() - 1
        } else {
            index - 1
        };
        let next_selection = apps.get(new_index).unwrap();
        self.selected_list_item = next_selection.to_owned();
    }
}
