use super::{ActionType, Config, ExecutionMode, LogType, ReportType};
use colored::*;

fn section(title: &str) {
    println!("\n{}", title.bright_blue().bold());
    println!("{}", "─".repeat(title.len()).bright_black());
}

fn key(k: &str) -> ColoredString {
    // pad first, then color (better alignment)
    format!("{:<28}", k).bright_yellow().bold()
}

fn bool_val(v: bool) -> ColoredString {
    if v {
        "true".green().bold()
    } else {
        "false".red().bold()
    }
}

fn opt_str(v: Option<&str>) -> ColoredString {
    match v {
        Some(s) if !s.is_empty() => s.white(),
        _ => "<none>".bright_black().italic(),
    }
}

fn mode_val(m: &ExecutionMode) -> ColoredString {
    match m {
        ExecutionMode::dry_run => "dry_run".yellow().bold(),
        ExecutionMode::run => "run".green().bold(),
    }
}

fn action_val(a: &ActionType) -> ColoredString {
    match a {
        ActionType::Copy => "copy".cyan().bold(),
        ActionType::Move => "move".bright_blue().bold(),
        ActionType::Delete => "delete".red().bold(),
    }
}

fn log_val(l: &LogType) -> ColoredString {
    match l {
        LogType::Info => "info".bright_cyan().bold(),
        LogType::Success => "success".green().bold(),
        LogType::Error => "error".red().bold(),
    }
}

fn report_val(r: &ReportType) -> ColoredString {
    match r {
        ReportType::Text => "text".white().bold(),
        ReportType::SpreadSheet => "spreadsheet".magenta().bold(),
        ReportType::Analytics => "analytics".bright_cyan().bold(),
    }
}

pub fn display_config(config: &Config) {
    println!("\n{}", "YOUR CONFIG".bright_yellow().bold());
    println!("{}", "═══════════".bright_black());

    section("META");
    println!(
        "{} {}",
        key("version"),
        config.version.to_string().cyan().bold()
    );
    println!("{} {}", key("created_by"), config.created_by.white());
    println!("{} {}", key("created_at"), config.created_at.white());

    section("GENERAL");
    println!("{} {}", key("mode"), mode_val(&config.general.mode));
    println!(
        "{} {}",
        key("default_action"),
        action_val(&config.general.default_action)
    );
    println!(
        "{} {}",
        key("recursive"),
        bool_val(config.general.recursive)
    );

    section("PATHS");
    println!("{} {}", key("quarantine"), config.paths.quarantine.green());
    println!("{} {}", key("state_dir"), config.paths.state_dir.green());
    println!("{}", key("roots"));
    if config.paths.roots.is_empty() {
        println!("  {}", "<none>".bright_black().italic());
    } else {
        for r in &config.paths.roots {
            println!("  {} {}", "•".bright_black(), r.green());
        }
    }

    section("IGNORE");
    println!(
        "{} {}",
        key("ignore_hidden"),
        bool_val(config.ignore.ignore_hidden)
    );

    println!("{}", key("extensions"));
    if config.ignore.extensions.is_empty() {
        println!("  {}", "<none>".bright_black().italic());
    } else {
        for ext in &config.ignore.extensions {
            println!("  {} {}", "•".bright_black(), ext.magenta());
        }
    }

    println!("{}", key("globs"));
    if config.ignore.globs.is_empty() {
        println!("  {}", "<none>".bright_black().italic());
    } else {
        for g in &config.ignore.globs {
            println!("  {} {}", "•".bright_black(), g.magenta());
        }
    }

    section("NAMING");
    println!(
        "{} {}",
        key("normalize_names"),
        bool_val(config.naming.normalize_names)
    );

    section("LAYOUT");
    println!(
        "{} {}",
        key("date_source"),
        config.layout.date_source.white()
    );
    println!(
        "{} {}",
        key("date_format"),
        config.layout.date_format.white()
    );

    section("LOG");
    println!("{} {}", key("level"), log_val(&config.log.level));

    section("REPORT");
    println!("{} {}", key("format"), report_val(&config.report.format));

    section("SAFETY");
    println!(
        "{} {}",
        key("require_within_roots"),
        bool_val(config.safety.require_within_roots)
    );

    // allow_delete is high-signal; color it more aggressively
    let allow_delete = if config.safety.allow_delete {
        "true (DELETES ENABLED)".red().bold()
    } else {
        "false".green().bold()
    };
    println!("{} {}", key("allow_delete"), allow_delete);

    section("RULES");
    if config.rules.is_empty() {
        println!("  {}", "<none>".bright_black().italic());
        return;
    }

    for (i, rule) in config.rules.iter().enumerate() {
        let idx = format!("[{}]", i + 1).bright_black();
        let enabled = if rule.enabled {
            "enabled".green().bold()
        } else {
            "disabled".red().bold()
        };

        println!(
            "\n  {} {} {}",
            idx,
            rule.name.bright_white().bold(),
            enabled
        );
        println!(
            "  {} {}",
            key("priority"),
            rule.priority.to_string().cyan().bold()
        );

        // MATCH
        println!("  {}", "match".bright_blue().bold());
        match rule.r#match.as_ref() {
            None => {
                println!("    {}", "<none>".bright_black().italic());
            }
            Some(m) => {
                let any = m.any.unwrap_or(false);
                println!("    {} {}", key("any"), bool_val(any));

                match m.extensions.as_ref() {
                    None => println!(
                        "    {} {}",
                        key("extensions"),
                        "<none>".bright_black().italic()
                    ),
                    Some(exts) if exts.is_empty() => {
                        println!(
                            "    {} {}",
                            key("extensions"),
                            "<none>".bright_black().italic()
                        )
                    }
                    Some(exts) => {
                        println!("    {}", key("extensions"));
                        for e in exts {
                            println!("      {} {}", "•".bright_black(), e.magenta());
                        }
                    }
                }
            }
        }

        // ACTION
        println!("  {}", "action".bright_blue().bold());
        match rule.action.as_ref() {
            None => {
                println!("    {}", "<none>".bright_black().italic());
            }
            Some(a) => {
                println!("    {} {}", key("type"), action_val(&a.r#type));
                println!("    {} {}", key("to"), opt_str(Some(a.to.as_str())).green());
                println!("    {} {}", key("use_layout"), bool_val(a.use_layout));
            }
        }
    }
}
