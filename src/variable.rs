use regex::Regex;
use crate::Error;

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Variable {
    pub full: String,
    pub name: String,
    pub arguments: Option<String>,
}

pub fn search(content: &str) -> Result<Vec<Variable>, Error> {
    let mut matches = Vec::new();

    let re = Regex::new(r"@@(.+)@@")?;

    for caps in re.captures_iter(content) {
        let full = caps.get(0).ok_or(Error::UnprocessedRegexMatch)?.as_str().to_owned();
        let raw = caps.get(1).ok_or(Error::UnprocessedRegexMatch)?.as_str().to_owned();
        let mut splited = raw.splitn(2, ':');
        let name = splited.next().expect("will return at least one item; qed").to_string();
        let arguments = splited.next().map(|v| v.to_string());
        matches.push(Variable { full, name, arguments });
    }

    Ok(matches)
}
