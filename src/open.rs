use {
    crate::errors::ProgramError,
    lazy_regex::*,
    reqwest::Url,
    std::{
        fs,
        path::PathBuf,
        str::FromStr,
    },
};

pub struct MdFile {
    /// the path to display on string
    pub path: String,

    /// The file's markdown
    pub markdown: String,
}

impl MdFile {
    pub fn new<P: Into<String>>(path: P, markdown: String) -> Self {
        Self {
            path: path.into(),
            markdown,
        }
    }
    pub fn from_file_path(path: PathBuf) -> Result<Self, ProgramError> {
        if path.is_dir() {
            return Err(ProgramError::NotRegular {
                path: path.to_string_lossy().to_string()
            });
        }
        Ok(Self::new(path.to_string_lossy(), fs::read_to_string(&path)?))
    }
    pub fn from_url<U: Into<String>>(url: U) -> Result<Self, ProgramError> {
        let url: String = url.into();
        let markdown = reqwest::blocking::get(&url)?.text()?;
        Ok(Self::new(url, markdown))
    }
}

impl FromStr for MdFile {
    type Err = ProgramError;
    fn from_str(target: &str) -> Result<Self, ProgramError> {
        // first try as a local file
        let path = PathBuf::from(target);
        if path.exists() {
            return Self::from_file_path(path);
        }
        // if it's a link to a github repo, we go fetch the raw readme
        if let Some((_, user, repo)) = regex_captures!(
            r#"^(?:https://github.com/)?([a-zA-Z0-9][[a-zA-Z0-9]-]{1,38})/([a-zA-Z0-9][[a-zA-Z0-9]-]{1,50})/?$"#,
            target,
        ) {
            return Self::from_url(
                format!("https://raw.githubusercontent.com/{user}/{repo}/master/README.md")
            );
        }
        // if it's a link to a md file at github, we fetch it raw
        //if let Some((_, path)) = regex_captures!(r#"^https://github.com/(\S+\.md)$"#i, target) {
        if let Some((_, user, repo, path)) = regex_captures!(
            r#"^(?:https://github.com/)?([a-zA-Z0-9][[a-zA-Z0-9]-]{1,38})/([a-zA-Z0-9][[a-zA-Z0-9]-]{1,50})/blob/(\S+\.md)$$"#,
            target
        ) {
            return Self::from_url(
                format!("https://raw.githubusercontent.com/{user}/{repo}/{path}")
            );
        }
        // at the very least, is it an URL ?
        if Url::parse(target).is_ok() {
            return Self::from_url(target);
        }
        Err(ProgramError::FileNotFound { path: target.to_string() })
    }
}
