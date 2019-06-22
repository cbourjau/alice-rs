/// Return true if the given type name represents a core type.
/// Core types are differently implemented even if they are described
/// in the `TStreamer`. Gotta love ROOT!
pub(crate) fn type_is_core(name: &str) -> bool {
    match name {
        "TObject" | "TString" | "TNamed" | "TObjArray" | "TObjString" | "TList" => true,
        s => s.starts_with("TArray"),
    }
}

/// If necessary, annotate the given type name with a life time or replace with an alias
pub(crate) fn alias_or_lifetime(t: &str) -> String {
    // Most core types do not need a life time specifier
    if type_is_core(t) && t != "TObjArray" {
        return t.to_string();
    }
    // All non-core types get a life time
    // This is over zealous, but currently, I don't have a proper way
    // to check if a type has a member with a lifetime
    return format!("{}<'s>", t);
}

pub(crate) fn sanitize(n: &str) -> String {
    let keywords = vec![
        "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn",
        "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref",
        "return", "Self", "self", "static", "struct", "super", "trait", "true", "type", "unsafe",
        "use", "where", "while", "abstract", "alignof", "become", "box", "do", "final", "macro",
        "offsetof", "override", "priv", "proc", "pure", "sizeof", "typeof", "unsized", "virtual",
        "yield",
    ];
    if keywords.into_iter().any(|w| w == n) {
        format!("{}_", n)
    } else {
        n.to_string()
    }
}
