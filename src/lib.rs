#![deny(clippy::all)]
use regex::{ Regex};

#[macro_use]
extern crate napi_derive;

#[napi]
pub fn capitalize(s: String) -> String {
  let str = s.to_lowercase();
  let mut c = str.chars();
  match c.next() {
    None => String::new(),
    Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
  }
}

#[napi]
pub fn title_case(str: String) -> String {
  // let str=Regex::new(r"(?=[A-Z])|[\.\-\s_]");
  // let str=match Regex::new(r"(?=[A-Z])|[.\-\s_]") {
  //   Ok(s) => s,
  //   Err(e) => return e.to_string(),
  // };
  // let words: Vec<String> = str.split(&s.to_owned()).map(|x| capitalize(x.to_string())).collect();
  // words.join(" ")
  // let capitalize = |s: &str| {
  //   let mut chars = s.chars();
  //   chars.next().map(|c| c.to_uppercase().collect::<String>()).unwrap_or_default() + chars.as_str()
  // };
  // let words: Vec<String> = Regex::new(r"\W+")
  //     .unwrap()
  //     .split(&file_raw.to_owned())
  //     .map(|x| x.to_string())
  //     .collect();
  //split on '.', '-', '_', or whitespace characters
  let re = Regex::new(r"[.\-\s_]+").unwrap();
  re.split(&str)
      .map(|s| s.trim())
      .filter(|s| !s.is_empty())
      .map(|s| capitalize(s.to_string()))
      .collect::<Vec<_>>()
      .join(" ")

}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn capitalize_test() {
    assert_eq!(capitalize("hello".to_string()), "Hello".to_string());
    assert_eq!(
      capitalize("hello world".to_string()),
      "Hello world".to_string()
    );
    assert_eq!(capitalize("".to_string()), "".to_string());
    assert_eq!(capitalize("123".to_string()), "123".to_string());
    assert_eq!(
      capitalize("HeLlO WoRlD".to_string()),
      "Hello world".to_string()
    );
  }
  #[test]
  fn title_case_test() {
    assert_eq!(title_case("hello".to_string()), "Hello".to_string());
    assert_eq!(title_case("hello-world".to_string()), "Hello World".to_string());
    assert_eq!(title_case("hello_world".to_string()), "Hello World".to_string());
    assert_eq!(title_case("hello.world".to_string()), "Hello World".to_string());
    assert_eq!(
      title_case("hello world".to_string()),
      "Hello World".to_string()
    );
    assert_eq!(title_case("".to_string()), "".to_string());
    assert_eq!(title_case("123".to_string()), "123".to_string());
    assert_eq!(
      title_case("HeLlO WoRlD".to_string()),
      "Hello World".to_string());
}}
