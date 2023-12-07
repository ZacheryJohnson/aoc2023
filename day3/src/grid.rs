use std::fmt::{Display, Formatter, Write};

pub struct NumberToken {
    // Inclusive
    start_idx: usize,

    // Inclusive
    end_idx: usize,

    pub value: u32
}

impl NumberToken {
    pub fn is_part(&self, grid: &Grid) -> bool {
        let has_room_left = self.start_idx % grid.width != 0;
        let has_room_right = self.end_idx % grid.width != (grid.width - 1);

        let has_part_fn = |maybe_indexes: Vec<Option<usize>>| {
            maybe_indexes
                .iter()
                .filter(|idx| idx.is_some())
                .any(|idx| grid.is_symbol_at(idx.unwrap()))
        };

        if has_room_left {
            let indexes = vec![
                self.start_idx.checked_sub(grid.width + 1), // NW of start char
                self.start_idx.checked_sub(1), // W of start char
                self.start_idx.checked_add(grid.width - 1), // SW of start char
            ];

            if has_part_fn(indexes) {
                return true;
            }
        }

        for i in self.start_idx..=self.end_idx {
            let indexes = vec![
                i.checked_sub(grid.width), // N of start char
                Some(i), // current char
                i.checked_add(grid.width), // SE of start char
            ];

            if has_part_fn(indexes) {
                return true;
            }
        }

        if has_room_right {
            let indexes = vec![
                self.end_idx.checked_sub(grid.width - 1), // NE of end char
                self.end_idx.checked_add(1), // E of end char
                self.end_idx.checked_add(grid.width + 1), // SE of end char
            ];

            if has_part_fn(indexes) {
                return true;
            }
        }

        false
    }
}

pub struct GearToken {
    num_1: u32,
    num_2: u32,
    pub ratio: u32,
}

pub struct Grid {
    pub width: usize,
    pub height: usize,
    data: Vec<char>,
    pub number_tokens: Vec<NumberToken>,
}

impl Grid {
    pub fn from_input(input: String) -> Grid {
        let lines = input
            .split_whitespace()
            .collect::<Vec<&str>>();

        let width = lines.first().unwrap().len();
        let height = lines.len();
        let mut data: Vec<char> = vec![];

        for line in lines {
            data.extend_from_slice(line.chars().collect::<Vec<char>>().as_slice());
        }

        let mut number_tokens = vec![];
        let mut i = 0usize;
        while i < (width * height) {
            if let Some(token) = Grid::number_at(i, &data, width) {
                i = token.end_idx;
                number_tokens.push(token);
            }

            i += 1;
        }

        i = 0usize;
        while i < (width * height) {

        }

        Grid {
            width,
            height,
            data,
            number_tokens
        }
    }

    fn index_of(&self, x_coord: usize, y_coord: usize) -> usize {
        y_coord * self.width + x_coord
    }

    pub fn is_symbol_at(&self, idx: usize) -> bool {
        if idx >= self.data.len() {
            return false;
        }

        let ch = self.data[idx];

        (ch == 0x2f as char) || (ch >= (0x21 as char) && ch <= (0x2d as char)) || (ch >= 0x3a as char && ch <= 0x40 as char)
    }

    fn number_at(idx: usize, data: &[char], grid_width: usize) -> Option<NumberToken> {
        let char = data[idx];
        if !char.is_numeric() {
            return None;
        }

        let row_starting_idx = (idx / grid_width) * grid_width as usize;
        let mut number_start_idx = idx - 1;
        while number_start_idx >= row_starting_idx && data[number_start_idx].is_numeric() {
            number_start_idx -= 1;
        }

        number_start_idx += 1;

        let row_ending_idx = (idx / grid_width + 1) * grid_width;
        let mut number_end_idx_inclusive = number_start_idx + 1;
        while number_end_idx_inclusive < row_ending_idx && data[number_end_idx_inclusive].is_numeric() {
            number_end_idx_inclusive += 1;
        }

        number_end_idx_inclusive -= 1;

        let number_chars = String::from_iter(&data[number_start_idx..=number_end_idx_inclusive]);
        if let Ok(number) = number_chars.parse::<u32>() {
            Some(NumberToken {
                start_idx: idx,
                end_idx: idx + number.ilog10() as usize,
                value: number,
            })
        } else {
            panic!("somehow received data that can not be parsed as integer")
        }
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let idx = self.index_of(x, y);
                f.write_char(self.data[idx])?;
            }
            f.write_char('\n')?;
        }

        Ok(())
    }
}