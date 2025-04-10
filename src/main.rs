mod data;
mod queries;

use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION, USER_AGENT},
    Client, ClientBuilder,
};
use std::{collections::HashMap, env};

type MainErr = Box<dyn std::error::Error>;
type MainRet = Result<(), MainErr>;

#[tokio::main]
async fn main() -> MainRet {
    if !tokio::fs::try_exists("./generated").await? {
        tokio::fs::create_dir("./generated").await?
    }
    let access_token = env::var("ACCESS_TOKEN").expect("ACCESS_TOKEN must be set");
    let mut headers = HeaderMap::new();
    let mut auth_value = HeaderValue::from_str(&format!("bearer {}", access_token)).unwrap();
    auth_value.set_sensitive(true);

    headers.insert(AUTHORIZATION, auth_value);
    headers.insert(USER_AGENT, HeaderValue::from_static("User"));

    let client = ClientBuilder::new()
        .default_headers(headers.clone())
        .build()
        .expect("Could not build http client");

    let stats = get_stats(&client).await?;
    generate_languages(stats).await?;
    Ok(())
}

async fn generate_languages(stats: data::Stats) -> MainRet {
    let mut template = include_str!("../templates/languages.svg").to_owned();
    dbg!(&template);

    let mut progress = String::new();
    let mut lang_list = String::new();
    let mut languages: Box<[(Box<str>, data::LangStats)]> = stats.langs.into_iter().collect();
    languages.sort_by(|a, b| b.1.size.cmp(&a.1.size));

    let delay = 150;
    languages
        .into_iter()
        .enumerate()
        .for_each(|(i, (name, stat))| {
            let color = match &stat.color {
                Some(color) => color,
                None => "#000000",
            };
            progress += &format!(
                r#"<span style="background-color: {color}; width: {:.2}%;" class="progress-item"></span>"#,
                stat.prop
            );

            lang_list += &format!(
                r#"<li style="animation-delay: {}ms;">
                       <svg xmlns="http://www.w3.org/2000/svg" class="octicon" style="fill: {color};"
                        viewBox="0 0 16 16" version="1.1" width="16" height="16"><path fill-rule="evenodd" d="M8 4a4 4 0 100 8 4 4 0 000-8z"></path>
                       </svg>
                       <span class="lang">{name}</span>
                       <span class="percent">{:.2}%</span>
                </li>"#,
                i * delay,
                stat.prop
            );
        });

    template = template
        .replace("{{ progress }}", &progress)
        .replace("{{ lang_list }}", &lang_list);

    if !tokio::fs::try_exists("./generated").await? {
        tokio::fs::create_dir("./generated").await?
    }
    tokio::fs::write("./generated/languages.svg", template).await?;

    Ok(())
}

async fn get_stats(client: &Client) -> Result<data::Stats, MainErr> {
    let (mut owned_cursor, mut contrib_cursor) = (None, None);
    let mut map = HashMap::new();
    let only_owned_repos =
        env::var("ONLY_OWNED_REPOS").map_or(false, |c| c.to_lowercase() == "true" || c == "1");

    let mut stats = data::Stats::new();

    loop {
        map.insert(
            "query",
            queries::repos_overview(owned_cursor, contrib_cursor),
        );

        let result = client
            .post("https://api.github.com/graphql")
            .json(&map)
            .send()
            .await?
            .json::<data::GithubInfo>()
            .await?
            .data
            .viewer;

        let stats_collector = |n: data::Node| {
            stats.forks += n.fork_count;
            stats.stars += n.stargazers.total_count;

            n.languages.edges.into_iter().for_each(|e| {
                let name = e.node.name;
                let color = e.node.color;
                let size = e.size;

                let lang = stats.langs.get_mut(name.as_ref());
                match lang {
                    Some(lang) => {
                        lang.size += size;
                        lang.occurences += 1;
                    }
                    None => {
                        stats.langs.insert(name, data::LangStats::new(size, color));
                    }
                };
            });
        };

        let mut has_next = false;
        if !only_owned_repos {
            result
                .repositories
                .nodes
                .into_iter()
                .chain(result.repositories_contributed_to.nodes.into_iter())
                .for_each(stats_collector);

            has_next = result.repositories.page_info.has_next_page;
        } else {
            result
                .repositories
                .nodes
                .into_iter()
                .for_each(stats_collector);

            has_next = result.repositories.page_info.has_next_page
                || result.repositories.page_info.has_next_page;
        }

        if !has_next {
            break;
        }

        map.clear();
        owned_cursor = result.repositories.page_info.end_cursor;
        contrib_cursor = result.repositories_contributed_to.page_info.end_cursor;
    }

    let total_size = stats.langs.iter().fold(0, |acc, s| acc + s.1.size);
    stats.langs.iter_mut().for_each(|(_, v)| {
        v.prop = 100.0 * v.size as f64 / total_size as f64;
    });

    Ok(stats)
}
