use crate::app::App;

use hyper::client::HttpConnector;
use hyper::http::{Error, Request};
use hyper::{body::HttpBody as _, Body, Client, Method};
use hyper_tls::HttpsConnector;

use serde::Deserialize;

// JIRA
#[derive(Deserialize, Debug, Clone)]
struct SprintResponse {
    values: Vec<Sprint>,
}
#[derive(Deserialize, Debug, Clone)]
struct Sprint {
    id: u16,
    state: String,
    name: String,
}
#[derive(Deserialize, Debug, Clone)]
struct IssuesResponse {
    issues: Vec<Issue>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct Issue {
    pub key: String,
    pub fields: Fields,
}
#[derive(Deserialize, Debug, Clone)]
pub struct Fields {
    pub status: Status,
    pub summary: String,
}
#[derive(Deserialize, Debug, Clone)]
pub struct Status {
    pub name: String,
}

// GITLAB
#[derive(Deserialize, Debug, Clone)]
pub struct Merge {
    pub title: String,
    pub state: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Pipeline {
    pub status: String,
    pub r#ref: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GitlabTag {
    pub name: String,
    pub commit: Commit,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Commit {
    pub created_at: String,
}

#[derive(Debug, Clone)]
pub struct Tag {
    pub name: String,
    pub created_at: String,
}

async fn get_active_sprint(
    app: &mut App,
    client: &Client<HttpsConnector<HttpConnector>>,
) -> Result<Sprint, Error> {
    // let req = Request::builder()
    //     .method(Method::GET)
    //     .uri("**your_jira_url**/rest/agile/**your_api_version**/board/**your_board_number**/sprint")
    //     .header("Authorization", format!("Basic {}", app.jira_access_token))
    //     .body(Body::empty())?;
    // let mut resp = client.request(req).await.unwrap();
    // let mut data = Vec::new();
    //
    // while let Some(chunk) = resp.body_mut().data().await {
    //     data.push(chunk.unwrap());
    // }
    // let flattened = data.iter().flatten().cloned().collect::<Vec<_>>();
    //
    // let sprints: SprintResponse = serde_json::from_slice(&flattened).unwrap();
    // let active_sprint = sprints
    //     .values
    //     .iter()
    //     .find(|sprint| sprint.state == "active")
    //     .unwrap();
    // app.active_sprint_name = active_sprint.name.to_owned();
    // Ok(active_sprint.to_owned())

    Ok(Sprint {
        id: 0,
        state: String::from("random state"),
        name: String::from("random name"),
    })
}

pub async fn get_issues_for_active_sprint(
    app: &mut App,
    client: &Client<HttpsConnector<HttpConnector>>,
) -> Result<Vec<Issue>, Error> {
    // let active_sprint = get_active_sprint(app, client).await?;
    //
    // let req = Request::builder()
    //     .method(Method::GET)
    //     .uri(format!(
    //         "**your_jira_url**/rest/agile/**your_api_version**/board/**your_board_number**/sprint/{}/issue",
    //         active_sprint.id
    //     ))
    //     .header("Authorization", format!("Basic {}", app.jira_access_token))
    //     .body(Body::empty())?;
    // let mut resp = client.request(req).await.unwrap();
    // let mut data = Vec::new();
    //
    // while let Some(chunk) = resp.body_mut().data().await {
    //     data.push(chunk.unwrap());
    // }
    // let flattened = data.iter().flatten().cloned().collect::<Vec<_>>();
    // let issues: IssuesResponse = serde_json::from_slice(&flattened).unwrap();
    //
    // Ok(issues.issues)

    Ok(vec![])
}

pub async fn get_merges_for_selected_application(
    gitlab_project_id: &u16,
    gitlab_token: &String,
    client: &Client<HttpsConnector<HttpConnector>>,
) -> Result<Vec<Merge>, Error> {
    // let uri = format!(
    //     "**your_gitlab_url**/api/**your_api_version**/projects/{}/merge_requests",
    //     gitlab_project_id
    // );
    // let req = Request::builder()
    //     .method(Method::GET)
    //     .uri(uri)
    //     .header("PRIVATE-TOKEN", gitlab_token)
    //     .body(Body::empty())?;
    //
    // let mut resp = client.request(req).await.unwrap();
    // let mut data = Vec::new();
    //
    // while let Some(chunk) = resp.body_mut().data().await {
    //     data.push(chunk.unwrap());
    // }
    // let flattened = data.iter().flatten().cloned().collect::<Vec<_>>();
    // let merges: Vec<Merge> = serde_json::from_slice(&flattened).unwrap();
    // Ok(merges)

    Ok(vec![])
}

pub async fn get_recent_pipelines(
    gitlab_project_id: &u16,
    gitlab_token: &String,
    client: &Client<HttpsConnector<HttpConnector>>,
) -> Result<Vec<Pipeline>, Error> {
    // let uri = format!(
    //     "**your_gitlab_url**/api/**your_api_version**/projects/{}/pipelines",
    //     gitlab_project_id
    // );
    // let req = Request::builder()
    //     .method(Method::GET)
    //     .uri(uri)
    //     .header("PRIVATE-TOKEN", gitlab_token)
    //     .body(Body::empty())?;
    // let mut resp = client.request(req).await.unwrap();
    // let data = resp.data().await.unwrap().unwrap().to_vec();
    // let pipelines: Vec<Pipeline> = serde_json::from_slice(&data).unwrap();
    //
    // Ok(pipelines)

    Ok(vec![])
}

pub async fn get_tag_names(
    gitlab_token: &String,
    client: &Client<HttpsConnector<HttpConnector>>,
) -> Result<Vec<Tag>, Error> {
    // let req = Request::builder()
    //     .method(Method::GET)
    //     .uri("**your_gitlab_url**/api/**your_api_version**/projects/**your_project_number**/repository/tags?search=v&&per_page=50")
    //     .header("PRIVATE-TOKEN", gitlab_token)
    //     .body(Body::empty())?;
    // let mut resp = client.request(req).await.unwrap();
    // let mut data = Vec::new();
    //
    // while let Some(chunk) = resp.body_mut().data().await {
    //     data.push(chunk.unwrap());
    // }
    // let flattened = data.iter().flatten().cloned().collect::<Vec<_>>();
    // let tags: Vec<GitlabTag> = serde_json::from_slice(&flattened).unwrap();
    // let tag_names: Vec<Tag> = tags
    //     .iter()
    //     .map(|tag| Tag {
    //         name: tag.to_owned().name,
    //         created_at: tag.to_owned().commit.created_at,
    //     })
    //     .collect();
    //
    // Ok(tag_names)

    Ok(vec![])
}
