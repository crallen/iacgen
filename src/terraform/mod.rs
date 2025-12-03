use once_cell::sync::Lazy;
use regex::Regex;

pub mod s3;

static RESOURCE_NAME_REPLACE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"[-. /]").unwrap());

pub fn normalize_resource_name(name: &str) -> String {
    RESOURCE_NAME_REPLACE_REGEX
        .replace_all(name, "_")
        .to_string()
}
