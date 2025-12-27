use std::path::PathBuf;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub version: i64,
    pub created_by: String,
    pub created_at: String,
    pub general: General,
    pub paths: ConfigPaths,
    pub ignore: Ignore,
    pub naming: Naming,
    pub layout: Layout,
    pub log: Log,
    pub report: Report,
    pub safety: Safety,
    pub rules: Vec<Rule>,
}

#[derive(Deserialize, Debug)]
pub struct General {
    pub mode: ExecutionMode,
    pub default_action: ActionType,
    pub recursive: bool,
}

#[derive(Debug, Deserialize)]
pub enum ExecutionMode {
    dry_run,
    run,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ActionType {
    Copy,
    Move,
    Delete,
}

#[derive(Deserialize, Debug)]
pub struct ConfigPaths {
    pub roots: Vec<PathBuf>,
    pub state_dir: PathBuf,
    pub quarantine: PathBuf,
}

#[derive(Deserialize, Debug)]
pub struct Ignore {
    pub globs: Vec<PathBuf>,
    pub ignore_hidden: bool,
    pub extensions: Vec<PathBuf>,
}

#[derive(Deserialize, Debug)]
pub struct Naming {
    pub normalize_names: bool,
}

#[derive(Deserialize, Debug)]
pub struct Layout {
    pub date_source: String,
    pub date_format: String,
}

#[derive(Deserialize, Debug)]
pub struct Log {
    pub level: LogType,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogType {
    Info,
    Error,
    Success,
}

#[derive(Deserialize, Debug)]
pub struct Report {
    pub format: ReportType,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ReportType {
    Text,
    SpreadSheet,
    Analytics,
}

#[derive(Deserialize, Debug)]
pub struct Safety {
    pub require_within_roots: bool,
    pub allow_delete: bool,
}

#[derive(Deserialize, Debug)]
pub struct Rule {
    pub name: String,
    pub enabled: bool,
    pub priority: i64,
    pub r#match: Option<Match>,
    pub action: Option<Action>,
}

#[derive(Deserialize, Debug)]
pub struct Match {
    pub extensions: Option<Vec<String>>,
    pub any: Option<bool>,
}

#[derive(Deserialize, Debug)]
pub struct Action {
    pub r#type: ActionType,
    pub to: PathBuf,
    pub use_layout: bool,
}
