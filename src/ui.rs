use crate::requests::{Issue, Merge, Pipeline};
use crate::{app, util};

use std::io;
use std::io::Stdout;
use termion::raw::RawTerminal;

use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::ListItem;
use tui::{text, widgets, Frame};

pub fn draw_ui(
    f: &mut Frame<TermionBackend<RawTerminal<Stdout>>>,
    mut app: &mut app::App,
) -> io::Result<()> {
    let chunks = Layout::default()
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .direction(Direction::Horizontal)
        .split(f.size());
    {
        let left_half_chunks = Layout::default()
            .constraints(
                [
                    Constraint::Percentage(35),
                    Constraint::Percentage(10),
                    Constraint::Percentage(55),
                ]
                .as_ref(),
            )
            .split(chunks[0]);
        {
            f.render_widget(ascii_art_block(&mut app), left_half_chunks[0]);
            f.render_widget(version_numbers_block(&mut app), left_half_chunks[1]);

            let list_chunks = Layout::default()
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .direction(Direction::Horizontal)
                .split(left_half_chunks[2]);

            // APP LIST
            let local_apps = &app.local_applications;
            let items: Vec<ListItem> = build_local_list_items(local_apps.to_owned());
            let app_list = widgets::List::new(items).block(
                widgets::Block::default()
                    .title("Local")
                    .borders(widgets::Borders::ALL),
            );
            // GITLAB LIST
            let gitlab_apps = &app.gitlab_applications;
            let items: Vec<ListItem> = build_gitlab_list_items(gitlab_apps.to_owned(), &app);
            let gitlab_list = widgets::List::new(items).block(
                widgets::Block::default()
                    .title("Gitlab Projects")
                    .borders(widgets::Borders::ALL),
            );
            f.render_widget(app_list, list_chunks[0]);
            f.render_widget(gitlab_list, list_chunks[1]);
        }
    }
    {
        let right_half_chunks = Layout::default()
            .constraints([Constraint::Percentage(60), Constraint::Percentage(40)].as_ref())
            .split(chunks[1]);

        {
            let jira_issues: Vec<ListItem> =
                build_jira_issues_list_items(app.active_sprint_issues.to_owned());
            let jira_issues_list = widgets::List::new(jira_issues).block(
                widgets::Block::default()
                    .title(format!("{}", app.active_sprint_name))
                    .borders(widgets::Borders::ALL),
            );

            f.render_widget(jira_issues_list, right_half_chunks[0]);
            {
                let bottom_right_half_chunks = Layout::default()
                    .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                    .split(right_half_chunks[1]);

                // Merges
                let merges = app.selected_application_merges.to_owned();
                let merge_items = build_merge_list_items(merges);
                let merge_list = widgets::List::new(merge_items).block(
                    widgets::Block::default()
                        .title(format!("Merges - {} ", app.selected_application.name))
                        .borders(widgets::Borders::ALL),
                );

                // Pipelines
                let pipelines = app.selected_application_pipelines.to_owned();
                let pipeline_items = build_pipeline_list_items(pipelines);
                let pipeline_list = widgets::List::new(pipeline_items).block(
                    widgets::Block::default()
                        .title(format!("Pipelines - {} ", app.selected_application.name))
                        .borders(widgets::Borders::ALL),
                );
                f.render_widget(merge_list, bottom_right_half_chunks[0]);
                f.render_widget(pipeline_list, bottom_right_half_chunks[1]);
            }
        }
    }
    Ok(())
}

fn version_numbers_block(app: &mut app::App) -> widgets::Paragraph {
    let version_block = widgets::Block::default()
        .title("Current Version")
        .borders(widgets::Borders::ALL)
        .style(Style::default().fg(WHITE));

    widgets::Paragraph::new(app.formatted_version_numbers.as_str())
        .block(version_block)
        .style(Style::default().fg(YELLOW))
}

fn ascii_art_block(app: &mut app::App) -> widgets::Paragraph<'static> {
    let ascii_art_block = widgets::Block::default()
        .title("")
        .borders(widgets::Borders::ALL)
        .style(Style::default().fg(WHITE));
    let big_money: Vec<&str> = vec![
        r#"  $$$$$$$$\   $$$$$$$$\    $$$$$$\   $$$$$$$$\ "#,
        r#"  \__$$  __|  $$  _____|  $$  __$$\  \__$$  __|"#,
        r#"     $$ |     $$ |        $$ /  \__|    $$ |   "#,
        r#"     $$ |     $$$$$\      \$$$$$$\      $$ |   "#,
        r#"     $$ |     $$  __|      \____$$\     $$ |   "#,
        r#"     $$ |     $$ |        $$\   $$ |    $$ |   "#,
        r#"     $$ |     $$$$$$$$\   \$$$$$$  |    $$ |   "#,
        r#"     \__|     \________|   \______/     \__|   "#,
    ];
    let spans: Vec<Spans> = big_money
        .iter()
        .enumerate()
        .map(|(index, &line)| {
            if app.party_mode == true {
                if app.logo_color_change_index == index as u16 {
                    Spans::from(Span::styled(line, Style::default().fg(ORANGE)))
                } else if app.logo_color_change_index + 1 == index as u16
                    || (app.logo_color_change_index == 7 && index == 0)
                {
                    Spans::from(Span::styled(line, Style::default().fg(ORANGE)))
                } else {
                    Spans::from(String::from(line))
                }
            } else {
                Spans::from(String::from(line))
            }
        })
        .collect();
    app.logo_color_change_index = if app.logo_color_change_index == 7 {
        0
    } else {
        app.logo_color_change_index + 1
    };
    widgets::Paragraph::new(spans)
        .style(Style::default().fg(YELLOW))
        .block(ascii_art_block)
}

fn build_merge_list_items(merges: Vec<Merge>) -> Vec<ListItem<'static>> {
    let mut in_progress: Vec<ListItem<'static>> = Vec::new();
    let mut completed: Vec<ListItem<'static>> = Vec::new();

    for merge in merges {
        let formatted = format!("{} ", merge.title);
        let style = Style::default().fg(match merge.state.as_str() {
            "opened" => YELLOW,
            "merged" => GREEN,
            _ => WHITE,
        });

        let content = text::Span::styled(formatted, style);
        match style.fg.unwrap() {
            Color::Rgb(254, 251, 103) => in_progress.push(ListItem::new(content)),
            Color::Rgb(91, 248, 84) => completed.push(ListItem::new(content)),
            _ => {}
        }
    }
    in_progress.append(&mut completed);
    in_progress
}

fn build_pipeline_list_items(apps: Vec<Pipeline>) -> Vec<ListItem<'static>> {
    apps.iter()
        .map(|pipeline| {
            let color = match pipeline.status.as_str() {
                "success" => GREEN,
                "pending" | "running" => YELLOW,
                "failed" => ORANGE,
                _ => WHITE,
            };

            let formatted = format!("{} - {}", pipeline.r#ref, pipeline.status);
            let content = text::Span::styled(formatted, Style::default().fg(color));
            ListItem::new(content)
        })
        .collect()
}

fn build_local_list_items(apps: Vec<util::Application>) -> Vec<ListItem<'static>> {
    apps.iter()
        .map(|app| {
            let is_available = util::get_local_port_status(app.local_port);
            let formatted = format!("{} - {}", app.local_port, app.name);

            let style = Style::default().fg(if is_available { ORANGE } else { GREEN });

            let content = text::Span::styled(formatted, style);
            ListItem::new(content)
        })
        .collect()
}

fn build_gitlab_list_items(apps: Vec<util::Application>, app: &app::App) -> Vec<ListItem<'static>> {
    apps.iter()
        .map(|flex_app| {
            let is_highlighted = &app.selected_list_item.name == &flex_app.name;
            let formatted = format!("{}", flex_app.name);

            let unselected_style = Style::default().fg(WHITE);
            let highlighted_style = Style::default().fg(GREEN).add_modifier(Modifier::ITALIC);

            let content = text::Span::styled(
                formatted,
                if is_highlighted {
                    highlighted_style
                } else {
                    unselected_style
                },
            );
            ListItem::new(content)
        })
        .collect()
}

fn build_jira_issues_list_items(issues: Vec<Issue>) -> Vec<ListItem<'static>> {
    let mut todo: Vec<ListItem<'static>> = Vec::new();
    let mut in_progress: Vec<ListItem<'static>> = Vec::new();
    let mut done: Vec<ListItem<'static>> = Vec::new();

    for issue in issues {
        let formatted_issue_key = if issue.key.len() == 8 {
            format!("{} ", issue.key)
        } else {
            issue.key
        };
        let formatted = format!("{} - {}", formatted_issue_key, issue.fields.summary);
        let status = &issue.fields.status.name;
        let style = Style::default().fg(match status.as_str() {
            "your" => ORANGE,
            "boards" => YELLOW,
            "statuses" => GREEN,
            _ => WHITE,
        });

        let content = text::Span::styled(formatted, style);
        match style.fg.unwrap() {
            Color::Rgb(255, 161, 26) => todo.push(ListItem::new(content)),
            Color::Rgb(254, 251, 103) => in_progress.push(ListItem::new(content)),
            Color::Rgb(91, 248, 84) => done.push(ListItem::new(content)),
            _ => {}
        }
    }

    todo.append(&mut in_progress);
    todo.append(&mut done);
    todo
}

// to preserve color scheme of my terminal to any other config
const GREEN: Color = Color::Rgb(91, 248, 84);
const YELLOW: Color = Color::Rgb(254, 251, 103);
const ORANGE: Color = Color::Rgb(255, 161, 26);
const WHITE: Color = Color::Rgb(244, 236, 236);
