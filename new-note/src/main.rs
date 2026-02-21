mod args;
mod frontmatter;

use core::time;
use std::{fs::File, io::Write, path::{Path, PathBuf}};

use args::Args;
use caretta_id::CarettaId;
use chrono::{DateTime, Local, NaiveDateTime, Timelike, Utc};
use clap::Parser as _;

use crate::frontmatter::Frontmatter;

fn get_file_name(id: &CarettaId) -> String {
    id.to_string().to_ascii_uppercase() + "_.md"
}

fn get_file_content(id: &CarettaId, timestamp: &NaiveDateTime, tags: &[String]) -> String {
    let timestamp_without_sec= timestamp.with_second(0).unwrap().with_nanosecond(0).unwrap();
    let frontmatter = Frontmatter {
        id: id.clone(),
        updated_at: timestamp_without_sec.clone(),
        created_at: timestamp_without_sec.clone(),
        tags: Vec::from(tags)
    };
    [
        "---\n",
        &serde_yaml::to_string(&frontmatter).unwrap(),
        "---\n"
    ].join("")
}


fn main() {
    let args = Args::parse();
    let timestamp = Utc::now().naive_local();
    let id= CarettaId::now_unix();
    let mut path = args.dir;
    path.push(get_file_name(&id));
    let content = get_file_content(&id, &timestamp, &args.tags);
    if !args.dry_run {
        let mut file = File::create(&path).unwrap();
        file.write_all(content.as_bytes()).unwrap();
    }
    println!("Created {}", &path.as_os_str().to_string_lossy());
    println!("content: \n{}", content );
}

#[cfg(test)]
mod tests {

    use std::{str::FromStr, sync::LazyLock};

    use chrono::{FixedOffset, TimeZone, Utc};

    use super::*;

    const TIMESTAMP: LazyLock<NaiveDateTime> = LazyLock::new(|| {
        DateTime::parse_from_rfc3339("2026-02-19T12:34:56.789+09:00").unwrap().naive_local()
    });
    const ID: LazyLock<CarettaId> = LazyLock::new(|| {
        CarettaId::from_timestamp_unix((*TIMESTAMP).and_local_timezone(FixedOffset::east_opt(9*3600).unwrap()).unwrap())
    });
    #[test]
    fn assert_id() {
        assert_eq!(
            &(*ID).to_string(),
            "gfy2c67"
        );
    }
    #[test]
    fn assert_get_file_name() {
        assert_eq!(
            &get_file_name(&*ID),
            "GFY2C67_.md"
        )
    }

    #[test]
    fn assert_get_file_content() {
        assert_eq!(
            &get_file_content(&*ID, &*TIMESTAMP, &["Task/ToDo".to_string(), "Coding".to_string(), "Journal".to_string() ]),
r#"---
id: gfy2c67
created_at: 2026-02-19T12:34
updated_at: 2026-02-19T12:34
tags:
- Task/ToDo
- Coding
- Journal
---
"#
        )
    }
}