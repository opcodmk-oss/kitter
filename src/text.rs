/// Format an English count with the appropriate singular or plural noun.
pub fn counted(count: usize, singular: &str, plural: &str) -> String {
    format!("{count} {}", if count == 1 { singular } else { plural })
}

#[cfg(test)]
mod tests {
    use super::counted;

    #[test]
    fn english_counts_handle_zero_one_and_many() {
        assert_eq!(counted(0, "skill", "skills"), "0 skills");
        assert_eq!(counted(1, "skill", "skills"), "1 skill");
        assert_eq!(counted(33, "skill", "skills"), "33 skills");
        assert_eq!(counted(1, "location", "locations"), "1 location");
    }
}
