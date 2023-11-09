pub fn with_array_brackets(s: String, brackets: bool) -> String {
    s + if brackets { "[]" } else { "" }
}
