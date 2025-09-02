// Token registry module
// Missing: MetaSep and MetaEnd token registrations

pub struct TokenRegistry {
    mappings: Vec<(FormattingToken, &'static str)>,
}

impl TokenRegistry {
    pub fn new() -> Self {
        Self {
            mappings: vec![
                // Existing mappings...
                // (FormattingToken::MetaSep, "<|meta_sep|>"), // MISSING
                // (FormattingToken::MetaEnd, "<|meta_end|>"), // MISSING
            ],
        }
    }

    pub fn register(&mut self, token: FormattingToken, value: &'static str) {
        self.mappings.push((token, value));
    }

    pub fn get_mapping(&self, token: FormattingToken) -> Option<&'static str> {
        self.mappings.iter().find_map(|&(t, v)| if t == token { Some(v) } else { None })
    }
}