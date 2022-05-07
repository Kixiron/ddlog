pub fn replace_in_place(string: &mut String, pattern: &str, replacement: &str) {
    // If the string doesn't contain the pattern, do nothing
    if string.len() < pattern.len() || !string.contains(pattern) {
        return;
    } else if string == pattern {
        string.clear();
        string.push_str(replacement);
        return;
    }

    // TODO: We can do an optimized in-place replacement when the `replacement`
    //       string is smaller than the pattern string
    replace_in_place_larger(string, pattern, replacement)
}

fn replace_in_place_larger(string: &mut String, pattern: &str, replacement: &str) {
    debug_assert!(string.len() >= pattern.len() && string.contains(pattern));

    let (mut buffer, mut current) = (String::with_capacity(string.len()), 0);
    for match_idx in string.match_indices(pattern).map(|(idx, _)| idx) {
        // Push the previous chunk of string
        buffer.push_str(&string[current..match_idx]);

        // Push the replacement string
        buffer.push_str(replacement);

        // Increment the current index
        current = match_idx + pattern.len();
    }

    *string = buffer;
}
