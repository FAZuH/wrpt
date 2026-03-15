use crate::commands::consts;
use crate::commands::stacks::handlers::list::fetch_stacks;
use crate::commands::stacks::models::deploy::EnvVar;
use crate::commands::wrpt::GlobalArgs;
use log_err::LogErrResult;
use prettytable::format::{FormatBuilder, LinePosition, LineSeparator};
use prettytable::{cell, Cell, Row, Table};
use reqwest::blocking::Response;
use reqwest::header::{HeaderName, HeaderValue};
use reqwest::Url;
use serde::de::DeserializeOwned;
use serde_json::Value::Null;
use simplelog::{debug, error, warn};
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::PathBuf;

pub(crate) fn create_client(api_key: &str, insecure: bool) -> reqwest::blocking::Client {
    reqwest::blocking::Client::builder()
        .danger_accept_invalid_certs(insecure)
        .default_headers({
            let mut headers = reqwest::header::HeaderMap::new();
            headers.insert(
                HeaderName::from_static("x-api-key"),
                HeaderValue::from_str(api_key).unwrap(),
            );
            headers
        })
        .build()
        .unwrap()
}

pub(crate) fn get_stack_id_from_name(
    name: &str,
    base_url: &str,
    access_token: &str,
    insecure: bool,
) -> Result<Option<u32>, ()> {
    let stacks = fetch_stacks(base_url, access_token, insecure)?;

    for stack in stacks {
        if stack.name.eq(name) {
            return Ok(Some(stack.id));
        }
    }

    Ok(None)
}

pub(crate) fn get_swarm_id_from_endpoint_id(
    endpoint_id: u32,
    url: &str,
    access_token: &str,
    insecure: bool,
) -> Option<String> {
    let mut url = url.to_string();
    url.push_str(
        consts::ENDPOINT_ENDPOINTS_DOCKER_INFO
            .replace("{id}", endpoint_id.to_string().as_str())
            .as_str(),
    );

    let response = create_client(access_token, insecure).get(url).send();

    let body = response
        .log_expect("invalid response from API")
        .text()
        .unwrap_or_default();

    let json = serde_json::from_str::<serde_json::Value>(body.as_str()).unwrap_or_default();

    let id = json
        .get("Swarm")
        .unwrap_or(&Null)
        .get("Cluster")
        .unwrap_or(&Null)
        .get("ID")
        .unwrap_or(&Null)
        .as_str();

    if id.is_some() {
        return Some(id?.to_string());
    }

    None
}

pub(crate) fn get_base_url(global_args: &GlobalArgs) -> Result<String, ()> {
    match global_args
        .url
        .clone()
        .or_else(|| env::var("PORTAINER_URL").ok())
    {
        None => {
            error!("param `url` or environment variable `PORTAINER_URL` should be set");
            Err(())
        }
        Some(base_url) => Ok(base_url),
    }
}

pub(crate) fn get_access_token(global_args: &GlobalArgs) -> Result<String, ()> {
    match global_args
        .access_token
        .clone()
        .or_else(|| env::var("PORTAINER_ACCESS_TOKEN").ok())
    {
        None => {
            error!("param `access-token` or environment variable `PORTAINER_ACCESS_TOKEN` should be set");
            Err(())
        }
        Some(access_token) => Ok(access_token),
    }
}

pub(crate) fn construct_url(base_url: &str, endpoint: &str) -> Result<Url, String> {
    let url = Url::parse(base_url).map_err(|_| "invalid base URL".to_string())?;
    url.join(endpoint)
        .map_err(|_| "invalid endpoint path".to_string())
}

pub(crate) fn build_table<T>(items: &[T], columns: Option<&[&str]>) -> Table
where
    T: serde::Serialize,
{
    let mut table = Table::new();
    table.set_format(
        FormatBuilder::new()
            .padding(1, 1)
            .separator(LinePosition::Title, LineSeparator::default())
            .build(),
    );

    if let Some(first_item) = items.first() {
        let headers = extract_headers(first_item, columns);
        table.set_titles(Row::new(headers.iter().map(|h| cell!(h)).collect()));
    }

    for item in items {
        let row = extract_row(item, columns);
        table.add_row(Row::new(row));
    }

    table
}

fn extract_headers<T>(item: &T, columns: Option<&[&str]>) -> Vec<String>
where
    T: serde::Serialize,
{
    let serialized = serde_json::to_value(item).log_expect("failed to serialize item");
    if let serde_json::Value::Object(map) = serialized {
        match columns {
            Some(cols) => cols.iter().map(|&col| col.to_string()).collect(),
            None => map.keys().map(|key| key.to_string()).collect(),
        }
    } else {
        vec![]
    }
}

fn extract_row<T>(item: &T, columns: Option<&[&str]>) -> Vec<Cell>
where
    T: serde::Serialize,
{
    let serialized = serde_json::to_value(item).log_expect("failed to serialize item");
    if let serde_json::Value::Object(map) = serialized {
        match columns {
            Some(cols) => cols
                .iter()
                .map(|&col| {
                    map.get(col).map_or_else(
                        || cell!(""), // If the column is missing, an empty cell is inserted.
                        process_table_value,
                    )
                })
                .collect(),
            None => map.values().map(process_table_value).collect(),
        }
    } else {
        vec![]
    }
}

fn process_table_value(value: &serde_json::Value) -> Cell {
    match value {
        serde_json::Value::Object(obj) => {
            // If the Id property exists, it is displayed
            if let Some(serde_json::Value::Number(id)) = obj.get("Id") {
                cell!(id.to_string())
            } else if let Some(serde_json::Value::String(id)) = obj.get("Id") {
                cell!(id)
            } else {
                // Otherwise, we encode in JSON
                cell!(value.to_string())
            }
        }
        serde_json::Value::String(s) => cell!(s),
        _ => cell!(value.to_string()),
    }
}

pub(crate) fn handle_api_response(response: Response) -> Result<Response, ()> {
    debug!("response = {:?}", response);

    if !response.status().is_success() {
        let status = response.status();
        let body = response
            .text()
            .unwrap_or_else(|_| "<unable to read response body>".to_string());

        // Try to parse the error response as JSON
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&body) {
            let message = json
                .get("message")
                .and_then(|v| v.as_str())
                .unwrap_or("<no message>");
            let details = json
                .get("details")
                .and_then(|v| v.as_str())
                .unwrap_or("<no details>");

            error!(
                "<b>Api error</>: <i>{}</>\n<b>message</>: <i>{}</>\n<b>details</>: <i>{}</>",
                status, message, details
            );
        } else {
            // If not JSON, log the raw body
            error!(
                "<b>Api error</>: <i>{}</>\n<b>body</>: <i>{}</>",
                status, body
            );
        }

        return Err(());
    }

    Ok(response)
}

pub(crate) fn parse_api_response<T>(response: Response) -> Result<Vec<T>, ()>
where
    T: DeserializeOwned,
{
    let response = handle_api_response(response)?.text().unwrap_or_else(|_| {
        warn!("unable to read API response");

        String::new()
    });

    debug!("response_body = {:?}", response);

    // Try to parse as a collection first
    Ok(
        serde_json::from_str::<Vec<T>>(&response).unwrap_or_else(|_| {
            // If parsing as a collection fails, try parsing as a single item
            serde_json::from_str::<T>(&response)
                .map(|item| vec![item]) // Wrap the single item in a Vec
                .unwrap_or_else(|_| {
                    warn!("error when deserializing JSON response as collection or item.");
                    vec![]
                })
        }),
    )
}

pub(crate) fn parse_env_file(file_path: Option<PathBuf>) -> Result<Vec<EnvVar>, io::Error> {
    let file = File::open(file_path.unwrap_or_default())?;
    let reader = io::BufReader::new(file);

    let mut vars = Vec::new();

    for line in reader.lines() {
        let line = line?;
        // Ignore empty lines or comments
        if line.trim().is_empty() || line.trim_start().starts_with('#') {
            continue;
        }

        if let Some((name, value)) = line.split_once('=') {
            // Strip surrounding quotes from value
            let value = value.trim();
            let value = if (value.starts_with('"') && value.ends_with('"'))
                || (value.starts_with('\'') && value.ends_with('\''))
            {
                &value[1..value.len() - 1]
            } else {
                value
            };
            vars.push(EnvVar {
                name: name.trim().to_string(),
                value: value.to_string(),
            });
        }
    }

    Ok(vars)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    // --- parse_env_file tests ---

    fn write_temp_env(content: &str) -> NamedTempFile {
        let mut file = NamedTempFile::new().unwrap();
        file.write_all(content.as_bytes()).unwrap();
        file
    }

    #[test]
    fn parse_env_file_basic() {
        let file = write_temp_env("KEY=value\nFOO=bar\n");
        let result = parse_env_file(Some(file.path().to_path_buf())).unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].name, "KEY");
        assert_eq!(result[0].value, "value");
        assert_eq!(result[1].name, "FOO");
        assert_eq!(result[1].value, "bar");
    }

    #[test]
    fn parse_env_file_comments_and_empty_lines() {
        let file = write_temp_env("# comment\n\nKEY=value\n  # indented comment\n");
        let result = parse_env_file(Some(file.path().to_path_buf())).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].name, "KEY");
    }

    #[test]
    fn parse_env_file_hash_in_value_preserved() {
        let file = write_temp_env("PASSWORD=abc#123\n");
        let result = parse_env_file(Some(file.path().to_path_buf())).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].name, "PASSWORD");
        assert_eq!(result[0].value, "abc#123");
    }

    #[test]
    fn parse_env_file_quoted_value() {
        let file = write_temp_env("KEY=\"hello world\"\nKEY2='single'\n");
        let result = parse_env_file(Some(file.path().to_path_buf())).unwrap();
        assert_eq!(result[0].value, "hello world");
        assert_eq!(result[1].value, "single");
    }

    #[test]
    fn parse_env_file_nonexistent_file() {
        let result = parse_env_file(Some(PathBuf::from("/tmp/nonexistent_env_file_12345")));
        assert!(result.is_err());
    }

    #[test]
    fn parse_env_file_line_without_equals() {
        let file = write_temp_env("NOEQUALS\nKEY=value\n");
        let result = parse_env_file(Some(file.path().to_path_buf())).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].name, "KEY");
    }

    #[test]
    fn parse_env_file_value_with_equals() {
        let file = write_temp_env("KEY=val=ue\n");
        let result = parse_env_file(Some(file.path().to_path_buf())).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].name, "KEY");
        // split_once('=') keeps the rest, but then inline # splitting may affect it
        assert_eq!(result[0].value, "val=ue");
    }

    #[test]
    fn parse_env_file_whitespace_trimming() {
        let file = write_temp_env("  KEY  =  value  \n");
        let result = parse_env_file(Some(file.path().to_path_buf())).unwrap();
        assert_eq!(result[0].name, "KEY");
        assert_eq!(result[0].value, "value");
    }

    // --- construct_url tests ---

    #[test]
    fn construct_url_valid() {
        let url = construct_url("https://portainer.example.com", "/api/stacks").unwrap();
        assert_eq!(url.as_str(), "https://portainer.example.com/api/stacks");
    }

    #[test]
    fn construct_url_invalid_base() {
        let result = construct_url("not-a-url", "/api/stacks");
        assert!(result.is_err());
    }

    #[test]
    fn construct_url_with_trailing_slash() {
        let url = construct_url("https://portainer.example.com/", "/api/stacks").unwrap();
        assert_eq!(url.as_str(), "https://portainer.example.com/api/stacks");
    }

    // --- get_base_url tests ---

    #[test]
    fn get_base_url_from_arg() {
        let args = GlobalArgs {
            url: Some("https://example.com".to_string()),
            access_token: None,
            insecure: false,
            verbose: 1,
            quiet: false,
            color: clap::ColorChoice::Auto,
        };
        assert_eq!(get_base_url(&args).unwrap(), "https://example.com");
    }

    #[test]
    fn get_base_url_missing() {
        // Clear the env var to ensure test isolation
        env::remove_var("PORTAINER_URL");
        let args = GlobalArgs {
            url: None,
            access_token: None,
            insecure: false,
            verbose: 1,
            quiet: false,
            color: clap::ColorChoice::Auto,
        };
        assert!(get_base_url(&args).is_err());
    }

    // --- get_access_token tests ---

    #[test]
    fn get_access_token_from_arg() {
        let args = GlobalArgs {
            url: None,
            access_token: Some("my-token".to_string()),
            insecure: false,
            verbose: 1,
            quiet: false,
            color: clap::ColorChoice::Auto,
        };
        assert_eq!(get_access_token(&args).unwrap(), "my-token");
    }

    #[test]
    fn get_access_token_missing() {
        env::remove_var("PORTAINER_ACCESS_TOKEN");
        let args = GlobalArgs {
            url: None,
            access_token: None,
            insecure: false,
            verbose: 1,
            quiet: false,
            color: clap::ColorChoice::Auto,
        };
        assert!(get_access_token(&args).is_err());
    }

    // --- build_table tests ---

    #[derive(Debug, serde::Serialize)]
    struct TestItem {
        id: u32,
        name: String,
    }

    #[test]
    fn build_table_empty() {
        let items: Vec<TestItem> = vec![];
        let table = build_table(&items, None);
        // Empty table should have no rows
        assert_eq!(table.len(), 0);
    }

    #[test]
    fn build_table_with_items() {
        let items = vec![
            TestItem {
                id: 1,
                name: "foo".to_string(),
            },
            TestItem {
                id: 2,
                name: "bar".to_string(),
            },
        ];
        let table = build_table(&items, None);
        assert_eq!(table.len(), 2); // 2 data rows (title row not counted by len())
    }

    #[test]
    fn build_table_with_column_filter() {
        let items = vec![TestItem {
            id: 1,
            name: "foo".to_string(),
        }];
        let table = build_table(&items, Some(&["name"]));
        // Should still have 1 row, but only the "name" column
        assert_eq!(table.len(), 1);
    }
}
