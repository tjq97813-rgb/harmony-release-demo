// Token encoding module
// Current bug: MetaSep mapped to <|channel|> instead of <|meta_sep|>

pub enum FormattingToken {
    MetaSep,
    MetaEnd,
    // Other tokens...
}

pub fn encode_token(token: FormattingToken) -> &'static str {
    match token {
        FormattingToken::MetaSep => "<|channel|>", // BUG: Should be <|meta_sep|>
        FormattingToken::MetaEnd => "<|meta_end|>",
        // Other mappings...
    }
}