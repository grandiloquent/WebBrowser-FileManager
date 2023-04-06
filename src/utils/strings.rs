pub trait StringExt {
    /// Returns the string before the search string.
    fn substring_before(&self, search: &str) -> String;
    /// Returns the string after the search string.
    fn substring_after(&self, search: &str) -> String;

    /// Returns the string before the last match of the search string.
    fn substring_before_last(&self, search: &str) -> String;

    /// Returns the string after the last match of the search string.
    fn substring_after_last(&self, search: &str) -> String;

    /// Returns the string between the start and end bookend strings.
    fn substring_between(&self, start: &str, end: &str) -> String;
}

impl StringExt for String {
    fn substring_before(&self, search: &str) -> String {
        let i_pos = self.find(search);
        let answer = match i_pos {
            None => String::from(self),
            Some(val) => self[..val].to_string()
        };
        answer
    }

    fn substring_before_last(&self, search: &str) -> String {
        let i_pos = self.rfind(search);
        let answer = match i_pos {
            None => String::from(self),
            Some(val) => self[..val].to_string()
        };
        answer
    }

    fn substring_after(&self, search: &str) -> String {
        let i_pos = self.find(search);
        let answer = match i_pos {
            None => String::new(),
            Some(val) => self[(val + search.len())..].to_string()
        };
        answer
    }

    fn substring_after_last(&self, search: &str) -> String {
        let i_pos = self.rfind(search);
        let answer = match i_pos {
            None => String::new(),
            Some(val) => self[(val + search.len())..].to_string()
        };
        answer
    }

    fn substring_between(&self, start: &str, end: &str) -> String {
        let i_start_pos = self.find(start);
        let answer = match i_start_pos {
            None => String::new(),
            Some(val) => {
                let rest = self[(val + start.len())..].to_string();
                let i_end_pos = rest.find(end);
                match i_end_pos {
                    None => String::new(),
                    Some(val2) => rest[0..val2].to_string()
                }
            }
        };
        answer
    }
}