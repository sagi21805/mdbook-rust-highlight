use strum_macros::AsRefStr;

/// Token mapping with
#[derive(AsRefStr)]
pub enum TokenTag {
    Keyword,
    Ident,
    String,
    Literal,
    EndOfToken,
    Function,
    SelfToken,
    Type,
    LifeTime,
}

impl ToString for TokenTag {
    fn to_string(&self) -> String {
        match self {
            Self::EndOfToken => String::from("</span>"),
            _ => format!("<span class=\"hlrs-{}\">", self.as_ref()),
        }
    }
}
