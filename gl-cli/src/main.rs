use reqwest::RequestBuilder;

use std::env;

use toml;

use tokio;

use std::fs;
use std::path::Path;

use git2::Repository;

// use reqwest::Error;

use clap::{Parser, Subcommand};

use serde::Deserialize;

use std::io::stdout;

use crossterm::{
    style::{Color, Print, ResetColor, SetForegroundColor},
    ExecutableCommand,
};

#[derive(Deserialize)]
struct Config {
    private_token: String,
}

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(short, long)]
    token: String,

    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Issues,
    MergeRequests,
    ToDos
}

#[derive(Deserialize, Debug)]
struct Issue{
    title: String,
    web_url: String,
    iid: u32,
    project_id: u32
}

#[derive(Deserialize, Debug)]
struct MergeRequest {
    title: String,
    web_url: String,
    iid: u32
}

#[derive(Deserialize, Debug)]
struct Todo {
    id: u32,
    body: String,
    target_url: String
}

use std::env::VarError;
use std::path::PathBuf;

fn config_file_path() -> Result<PathBuf, VarError> {
    let home_var = std::env::var("HOME");

    match home_var {
        Ok(home_content) => return Ok(Path::new(&home_content).join("gl.toml")),
        Err(error) => return Err(error)
    }

}

fn read_configuration_file() -> std::io::Result<String> {
    let home_var = std::env::var("HOME");

    let home_var_content: String = match home_var {
        Ok(home_content) => home_content,
        Err(error) => panic!("{}", error),
    };

    let config_path = Path::new(&home_var_content).join("gl.toml");

    return fs::read_to_string(config_path);
}

fn parse_configuration_file() -> Config {
    let file_content : std::io::Result<String> = read_configuration_file();
    match file_content {
        Ok(content) => toml::from_str(content.as_str()).unwrap(),
        Err(error) => panic!("Unable to read the configuration file: {:?}", error),
    }
}

fn pretty_print(msg: &String) -> crossterm::Result<()> {
    let output: String = format!("- {}", msg);
    stdout()
        .execute(SetForegroundColor(Color::Blue))?
        .execute(Print(output))?
        .execute(ResetColor)?;

    println!("");

    Ok(())
}

fn pretty_print_issues(issue_list: Vec<Issue>) {
    for issue in issue_list.iter() {
        pretty_print(&issue.web_url);
    }
}

fn pretty_print_mrs(mr_list: Vec<MergeRequest>) {
    for mr in mr_list.iter() {
        pretty_print(&mr.web_url);
    }
}

fn pretty_print_todos(todo_list: Vec<Todo>) {
    for todo in todo_list.iter() {
        pretty_print(&todo.target_url);
    }
}

fn get(client: reqwest::Client, url: String, token: String) -> RequestBuilder {
    let mut bearer: String = "Bearer ".to_string();
    bearer.push_str(&token);
    return client.get(url)
        .header(reqwest::header::AUTHORIZATION, bearer)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .header(reqwest::header::ACCEPT, "application/json");
}

#[tokio::main]
async fn main() -> crossterm::Result<()> {
    let cli = Cli::parse();

    let token: String = cli.token;

    let client = reqwest::Client::new();

    let path = env::current_dir()?;
    println!("The current directory is {}", path.display());

    let config: Config = parse_configuration_file();
    println!("{:?}", config.private_token);

    let git_path = path.join(".git");
    let git_path_is_valid = git_path.exists();

    // let repo = match Repository::open("/path/to/a/repo") {
    //     Ok(repo) => repo,
    //     Err(e) => panic!("failed to open: {}", e),
    // };

    match &cli.command {
        Commands::Issues => {
            let user_issues_url: String = String::from("https://gitlab.com/api/v4/issues?scope=assigned_to_me&state=opened");
            let response = get(client, user_issues_url, token).send().await.unwrap();
            let issues: Vec<Issue> = response.json().await.unwrap();
            pretty_print_issues(issues);
        },
        Commands::MergeRequests=> {
            let user_mr_url: String = String::from("https://gitlab.com/api/v4/merge_requests?scope=assigned_to_me&state=opened");
            // let url = "https://gitlab.com/api/v4/merge_requests?scope=assigned_to_me&state=opened";
            // let url = "https://gitlab.com/api/v4/merge_requests?reviewer_id=current_user_id&state=opened";
            let response = get(client, user_mr_url, token).send().await.unwrap();
            println!("{:?}", response);
            let mrs: Vec<MergeRequest> = response.json().await.unwrap();
            pretty_print_mrs(mrs);
        },
        Commands::ToDos => {
            let user_todos_url: String = String::from("https://gitlab.com/api/v4/todos?state=pending");
            let response = get(client, user_todos_url, token).send().await.unwrap();
            let todos: Vec<Todo> = response.json().await.unwrap();
            pretty_print_todos(todos);
        }

    }

    Ok(())
}
