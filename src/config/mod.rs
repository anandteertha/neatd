use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    version: i64,
    created_by: String,
    created_at: String,
    general: General,
    paths: Paths,
    ignore: Ignore,
    naming: Naming,
    layout: Layout,
    log: Log,
    report: Report,
    safety: Safety,
    rules: Vec<Rule>,
}

#[derive(Deserialize, Debug)]
pub struct General {
    mode: ExecutionMode,
    default_action: ActionType,
    recursive: bool,
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
pub struct Paths {
    roots: Vec<String>,
    state_dir: String,
    quarantine: String,
}

#[derive(Deserialize, Debug)]
pub struct Ignore {
    globs: Vec<String>,
    ignore_hidden: bool,
    extensions: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct Naming {
    normalize_names: bool,
}

#[derive(Deserialize, Debug)]
pub struct Layout {
    date_source: String,
    date_format: String,
}

#[derive(Deserialize, Debug)]
pub struct Log {
    level: LogType,
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
    format: ReportType,
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
    require_within_roots: bool,
    allow_delete: bool,
}

#[derive(Deserialize, Debug)]
pub struct Rule {
    name: String,
    enabled: bool,
    priority: i64,
    r#match: Option<Match>,
    action: Option<Action>,
}

#[derive(Deserialize, Debug)]
pub struct Match {
    extensions: Option<Vec<String>>,
    any: Option<bool>,
}

#[derive(Deserialize, Debug)]
pub struct Action {
    r#type: ActionType,
    to: String,
    use_layout: bool,
}
