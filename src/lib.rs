pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines().filter(|l| l.contains(query)).collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|l| l.to_lowercase().contains(&query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "house";
        let contents = "\
                Mitochondira is the\n\
                powerhouse of the cell.";
        assert_eq!(vec!["powerhouse of the cell."], search(query, contents));
    }

    #[test]
    fn case_sensitive() {
        let query = "Rust";
        let contents = "\
                Rust:\n\
                safe, fast, productive.\n\
                Pick three.\n\
                Trust me.";
        assert_eq!(vec!["Rust:"], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "Rust";
        let contents = "\
                Rust:\n\
                safe, fast, productive.\n\
                Pick three.\n\
                Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
