use std::{collections::BTreeMap, env, fmt, fs, io};

pub struct Creds {
    values: BTreeMap<String, String>,
}

impl Creds {
    pub fn get(&self, key: &str) -> Option<&str> {
        self.values.get(key).map(String::as_str)
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// All pairs in key order, with secret values replaced by `***`.
    pub fn iter_redacted(&self) -> impl Iterator<Item = (&str, &str)> {
        self.values.iter().map(|(k, v)| {
            let v = if is_secret(k) { "***" } else { v.as_str() };
            (k.as_str(), v)
        })
    }
}

impl fmt::Debug for Creds {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_map().entries(self.iter_redacted()).finish()
    }
}

fn is_secret(key: &str) -> bool {
    let k = key.to_ascii_uppercase();
    ["PASS", "SECRET", "TOKEN", "KEY"]
        .iter()
        .any(|s| k.contains(s))
}

pub fn load_creds() -> io::Result<Creds> {
    let file = match fs::read_to_string(".env") {
        Ok(s) => s,
        Err(e) if e.kind() == io::ErrorKind::NotFound => String::new(),
        Err(e) => return Err(e),
    };

    let mut values = parse(&file);
    for (k, v) in values.iter_mut() {
        if let Ok(env_v) = env::var(k) {
            *v = env_v;
        }
    }
    Ok(Creds { values })
}

fn parse(src: &str) -> BTreeMap<String, String> {
    src.lines()
        .map(str::trim)
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .filter_map(|l| l.strip_prefix("export ").unwrap_or(l).split_once('='))
        .map(|(k, v)| (k.trim(), unquote(v.trim())))
        .filter(|(k, _)| !k.is_empty())
        .map(|(k, v)| (k.to_owned(), v.to_owned()))
        .collect() // later duplicates overwrite earlier ones: last wins
}

fn unquote(v: &str) -> &str {
    for q in ['"', '\''] {
        if let Some(inner) = v.strip_prefix(q).and_then(|s| s.strip_suffix(q)) {
            return inner;
        }
    }
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_all_pairs() {
        let m = parse(
            "# comment\n\nAPP_USER=bob\nexport APP_PASS=\"change-me\"\nAPP_GATEWAY='vpn.example.com/target'\nAPP_USER=alice\nX=a=b\n",
        );
        assert_eq!(m["APP_USER"], "alice");
        assert_eq!(m["APP_PASS"], "change-me");
        assert_eq!(m["APP_GATEWAY"], "vpn.example.com/target");
        assert_eq!(m["X"], "a=b");
        assert_eq!(m.len(), 4);
    }

    #[test]
    fn debug_redacts_secrets() {
        let c = Creds {
            values: parse("APP_USER=bob\nAPP_PASS=hunter2\n"),
        };
        let s = format!("{c:?}");
        assert!(s.contains("bob") && !s.contains("hunter2"));
    }

    #[test]
    fn get_returns_value_or_none() {
        let c = Creds {
            values: parse("APP_USER=bob\nAPP_PASS=hunter2\n"),
        };
        assert_eq!(c.get("APP_USER"), Some("bob"));
        assert_eq!(c.get("APP_PASS"), Some("hunter2"));
        assert_eq!(c.get("APP_MISSING"), None);
    }
}
