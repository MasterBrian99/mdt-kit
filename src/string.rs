use regex::Regex;

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
  //split on '.', '-', '_', or whitespace characters
  let re = match Regex::new(r"[.\-\s_]+") {
    Ok(re) => re,
    Err(_) => return String::new(),
  };
  re.split(&str)
    .map(|s| s.trim())
    .filter(|s| !s.is_empty())
    .map(|s| capitalize(s.to_string()))
    .collect::<Vec<String>>()
    .join(" ")
}

#[napi]
fn camel(input: String) -> String {
  let uppercase_re = Regex::new(r"[A-Z]+").unwrap();
  let separator_re = Regex::new(r"[.\-\s_]+").unwrap();

  let replaced = uppercase_re.replace_all(&input, |caps: &regex::Captures| {
    capitalize(caps[0].to_string())
  });

  let parts: Vec<String> = separator_re
    .split(&replaced)
    .map(|x| x.to_lowercase())
    .filter(|x| !x.is_empty())
    .collect();

  if parts.is_empty() {
    return String::new();
  }
  if parts.len() == 1 {
    return parts[0].clone();
  }

  parts
    .iter()
    .enumerate()
    .fold(String::new(), |mut acc, (i, part)| {
      if i == 0 {
        acc.push_str(part);
      } else {
        acc.push_str(&capitalize(part.to_string()));
      }
      acc
    })
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
    assert_eq!(
      title_case("hello-world".to_string()),
      "Hello World".to_string()
    );
    assert_eq!(
      title_case("hello_world".to_string()),
      "Hello World".to_string()
    );
    assert_eq!(
      title_case("hello.world".to_string()),
      "Hello World".to_string()
    );
    assert_eq!(
      title_case("hello world".to_string()),
      "Hello World".to_string()
    );
    assert_eq!(title_case("".to_string()), "".to_string());
    assert_eq!(title_case("123".to_string()), "123".to_string());
    assert_eq!(
      title_case("HeLlO WoRlD".to_string()),
      "Hello World".to_string()
    );
  }
  #[test]
  fn camel_test() {
    assert_eq!(camel("hello".to_string()), "hello".to_string());
    assert_eq!(camel("hello-world".to_string()), "helloWorld".to_string());
    assert_eq!(camel("hello_world".to_string()), "helloWorld".to_string());
    assert_eq!(camel("hello.world".to_string()), "helloWorld".to_string());
    assert_eq!(camel("hello world".to_string()), "helloWorld".to_string());
    assert_eq!(camel("".to_string()), "".to_string());
    assert_eq!(camel("123".to_string()), "123".to_string());
    assert_eq!(camel("HeLlO WoRlD".to_string()), "helloWorld".to_string());
  }
}
