pub trait StrExt {
    fn escape(&self) -> String;
}

impl StrExt for str {
    fn escape(&self) -> String {
        let mut out = String::with_capacity(self.len());
        for c in self.chars() {
            match c {
                '\\' => {
                    out.push('\\');
                    out.push('\\');
                }
                '"' => {
                    out.push('\\');
                    out.push('"');
                }
                _ => out.push(c),
            }
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn escapes_quotes_and_backslashes() {
        assert_eq!(r#"/odd"path\here"#.escape(), r#"/odd\"path\\here"#);
    }

    #[test]
    fn leaves_normal_strings_untouched() {
        assert_eq!("/usr/local/bin".escape(), "/usr/local/bin");
    }
}
