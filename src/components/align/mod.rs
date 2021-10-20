#[cfg(test)]
mod tests;

use std::cmp::max;

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
pub enum Align {
    Left,
    Center,
    Right,
}

impl Align {
    pub fn pad_left(&self, real_w: usize, max_w: u16) -> String {
        let amount = match self {
            Align::Left => 0,
            Align::Center => {
                let ret = (max_w as i32 / 2) - (real_w as i32 / 2);

                if max_w % 2 == 0 {
                    ret
                } else {
                    ret - 1
                }
            },
            Align::Right => (max_w as i32 - real_w as i32),
        };

        " ".repeat(amount as usize)
    }

    pub fn pad_right(&self, real_w: usize, max_w: u16) -> String {
        let amount = match self {
            Align::Left => max(0, max_w as i32 - real_w as i32),
            Align::Center => {
                let ret = (max_w as i32 / 2) - (real_w as i32 / 2);

                if max_w % 2 == 0 {
                    ret
                } else {
                    ret + 1
                }
            },
            Align::Right => 0,
        };

        " ".repeat(amount as usize)
    }

    pub fn crop(&self, s: &str, max_w: u16) -> String {
        let max_w = max_w as usize;

        match self {
            Align::Left => {
                let mut s = s.to_string();
                s.truncate(max_w);
                s
            },
            Align::Center => {
                if s.len() > max_w {
                    let mut s = s.to_string();
                    let excess = max_w - s.len();

                    if excess % 2 == 0 {
                        s.truncate(excess / 2);
                        front_truncate(&s, excess/ 2).to_string()
                    } else {
                        s.truncate((excess - 1) / 2);
                        front_truncate(&s, (excess + 1) / 2).to_string()
                    }
                } else {
                    s.to_string()
                }
            },
            Align::Right => {
                let s = s.to_string();
                front_truncate(&s, max_w - s.len()).to_string()
            },
        }
    }
}

fn front_truncate(s: &str, pos: usize) -> &str {
    match s.char_indices().skip(pos).next() {
        Some((pos, _)) => &s[pos..],
        None => "",
    }
}
