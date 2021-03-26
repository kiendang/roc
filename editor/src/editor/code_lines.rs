use crate::ui::text::lines::Lines;
use crate::ui::ui_error::UIResult;
use crate::ui::util::slice_get;
use bumpalo::collections::String as BumpString;
use bumpalo::Bump;

#[derive(Debug)]
pub struct CodeLines {
    pub lines: Vec<String>,
    pub nr_of_chars: usize,
}

impl CodeLines {
    pub fn from_str(code_str: &str) -> CodeLines {
        CodeLines {
            lines: split_inclusive(code_str),
            nr_of_chars: code_str.len(),
        }
    }
}

//TODO use rust's split_inclusive once it's no longer unstable
fn split_inclusive(code_str: &str) -> Vec<String> {
    let mut split_vec: Vec<String> = Vec::new();
    let mut temp_str = String::new();

    for token in code_str.chars() {
        if token != '\n' {
            temp_str.push(token);
        } else {
            split_vec.push(temp_str);
            temp_str = String::new();
            temp_str.push(token);
        }
    }

    if !temp_str.is_empty() {
        split_vec.push(temp_str);
    }

    split_vec
}

impl Lines for CodeLines {
    fn get_line(&self, line_nr: usize) -> UIResult<&str> {
        let line_string = slice_get(line_nr, &self.lines)?;

        Ok(&line_string)
    }

    fn line_len(&self, line_nr: usize) -> UIResult<usize> {
        self.get_line(line_nr).map(|line| line.len())
    }

    fn nr_of_lines(&self) -> usize {
        self.lines.len()
    }

    fn nr_of_chars(&self) -> usize {
        self.nr_of_chars
    }

    fn all_lines<'a>(&self, arena: &'a Bump) -> BumpString<'a> {
        let mut lines = BumpString::with_capacity_in(self.nr_of_chars(), arena);

        for line in &self.lines {
            lines.push_str(&line);
        }

        lines
    }

    fn last_char(&self, line_nr: usize) -> UIResult<Option<char>> {
        Ok(self.get_line(line_nr)?.chars().last())
    }
}