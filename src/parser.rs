pub trait BaseParser {
    fn get_pos(&self) -> usize;
    fn get_input(&self) -> &String;
    fn add_pos(&mut self, pos: usize) -> ();

    fn next_char(&self) -> char {
        self.get_input()[self.get_pos()..].chars().next().unwrap()
    }

    fn starts_with(&self, s: &str) -> bool {
        self.get_input()[self.get_pos()..].starts_with(s)
    }

    fn eof(&self) -> bool {
        self.get_pos() >= self.get_input().len()
    }

    fn consume_char(&mut self) -> char {
        let mut iter = self.get_input()[self.get_pos()..].char_indices();
        let (_, cur_char) = iter.next().unwrap();
        let (next_pos, _) = iter.next().unwrap_or((1, ' '));
        self.add_pos(next_pos);
        return cur_char;
    }

    fn consume_while<F>(&mut self, test: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let mut result = String::new();
        while !self.eof() && test(self.next_char()) {
            result.push(self.consume_char());
        }
        return result;
    }

    fn consume_whitespace(&mut self) {
        self.consume_while(char::is_whitespace);
    }
}
