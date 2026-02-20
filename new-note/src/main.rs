mod args;

use std::path::{Path, PathBuf};

use args::Args;
use caretta_id::CarettaId;
use chrono::{DateTime, Local, NaiveDateTime};

fn get_file_name<P>(parend_dir: P, id: &CarettaId) -> PathBuf
where P: AsRef<Path>{
    todo!()
}
fn get_file_content(id: &CarettaId, timestamp: &NaiveDateTime, tags: &[String]) -> String {
    todo!()
}


fn main() {
    println!("Hello, world!");
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
        let test_dir = Path::new("example");
        assert_eq!(
            get_file_name(test_dir,&*ID).as_path(),
            Path::new("example/GFY2C67_.md")
        )
    }

    #[test]
    fn assert_get_file_content() {
        assert_eq!(
            &get_file_content(&*ID, &*TIMESTAMP, &["tag1".to_string(), "tag2".to_string(), "tag3".to_string() ]),
r#"---
id: gfy2c67
created: 2026-02-19T12:34
updated: 2026-02-19T12:34
tags:
  - Task/ToDo
  - Coding
  - journal
---
"#
        )
    }
}