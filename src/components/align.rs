#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
pub enum Align {
    Left,
    Center,
    Right,
}

impl Align {
    pub fn offset(&self, real_w: usize, max_w: u16) -> i32 {
        match self {
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
        }

    }
}
