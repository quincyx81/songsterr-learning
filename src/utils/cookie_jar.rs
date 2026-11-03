use serde::{Serialize, Deserialize};
use reqwest::cookie::Jar;
use reqwest::Url;
use serde_json;

use crate::utils::python_execute::run_py_command;

#[derive(Serialize, Deserialize, Debug)]
struct Cookie {
   name: String,
   value: String
}

fn serialize_cookies(domain: &str) -> Vec<Cookie> {
    let domain_arg = format!("--domain {}", domain);
    let args = ["./python_scripts/get_cookies.py", &domain_arg];
    let stdout = run_py_command(&args).unwrap();
    serde_json::from_str::<Vec<Cookie>>(&stdout).unwrap()
}

pub fn fill_from_domain(url: &Url, domain: &str) -> Jar {
    let jar = Jar::default();
    let cookies = serialize_cookies(&domain);
    for cookie in &cookies {
        let cookie_str = format!("{}={}", cookie.name, cookie.value);
        jar.add_cookie_str(&cookie_str, &url);
    }
    return jar;
} 
