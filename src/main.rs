pub mod data_analyzers;
pub mod github_data_fetchers;
pub mod reports;
pub mod utils;
use data_analyzers::{get_repo_info, get_repo_overview_by_scraper, search_bing};
use dotenv::dotenv;
use github_data_fetchers::get_user_data_by_login;
use reports::*;
use serde_json::Value;
use std::collections::HashMap;
use std::env;

use clap::{App, Arg};

#[tokio::main]
async fn main() {
    dotenv().ok();
    let OPENAI_API_KEY = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY is not set");

    let matches = App::new("GitHub Data Fetcher")
        .version("1.0")
        .author("Your Name")
        .about("Fetches data from GitHub and performs operations")
        .arg(
            Arg::with_name("login")
                .long("login")
                .help("Specifies the user login for GitHub")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("about_repo")
                .long("about-repo")
                .help("Provides information about a specific repository")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("owner")
                .long("owner")
                .help("Specifies the owner of the repository")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("repo")
                .long("repo")
                .help("Specifies the repository name")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("username")
                .long("username")
                .help("Specifies the GitHub username")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("token")
                .long("token")
                .help("Specifies the GitHub token for authentication")
                .takes_value(true),
        )
        .get_matches();

    if let Some(login) = matches.value_of("login") {
        let bing_key =
            env::var("bing_key").expect("Bing key was not present in environment variables");
        match get_user_data_by_login(login).await {
            Ok(pro) => {
                let query = format!("github user {}", login);
                match search_bing(&bing_key, &query).await {
                    Some(search_data) => {
                        // println!(
                        //     "Found on profile: {}\nFound with search: {}",
                        //     pro, search_data
                        // );
                    }
                    None => {
                        println!("Error searching Bing: ");
                        std::process::exit(1);
                    }
                }
            }
            Err(e) => {
                println!("Error getting user data: {}", e);
                std::process::exit(1);
            }
        }
    }

    if let Some(about_repo) = matches.value_of("about_repo") {
        match get_repo_overview_by_scraper(about_repo).await {
            Some(summary) => {
                println!("About {}: {}", about_repo, summary);
            }
            None => {
                println!("Error getting repository overview: ");
                std::process::exit(1);
            }
        }
    }

    if let (Some(owner), Some(repo)) = (matches.value_of("owner"), matches.value_of("repo")) {
        let username = matches.value_of("username").map(String::from);
        let token = matches.value_of("token").map(String::from);

        let report = weekly_report(owner, repo, username, token).await;
        println!("Weekly report for {}/{}:\n{}", owner, repo, report);
    }
}
