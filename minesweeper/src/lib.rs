enum CellPosition {
    UpperLeft,
    UpperRight,
    TopRow,
    LowerLeft,
    LowerRight,
    BottomRow,
    LeftColumn,
    RightColumn,
    Middle,
}

enum Possible<T> {
    Some(T),
    None(T),
}

impl<T: Clone> Possible<T> {
    fn unwrap(&self) -> T {
        match self {
            Self::Some(t) => t.clone(),
            Self::None(t) => t.clone(),
        }
    }
}

impl Default for Possible<usize> {
    fn default() -> Self {
        Self::None(usize::MAX)
    }
}

impl Default for Possible<(usize, usize)> {
    fn default() -> Self {
        Self::None((usize::MAX, usize::MAX))
    }
}

mod direction {
    use crate::Possible;
    use crate::Possible::Some;

    #[derive(Default)]
    pub struct Direction {
        pub right: Possible<usize>,
        pub left: Possible<usize>,
        pub above: Possible<(usize, usize)>,
        pub below: Possible<(usize, usize)>,
        pub above_right: Possible<(usize, usize)>,
        pub above_left: Possible<(usize, usize)>,
        pub below_right: Possible<(usize, usize)>,
        pub below_left: Possible<(usize, usize)>,
    }

    impl Direction {
        pub fn new() -> Self {
            Self {
                ..Default::default()
            }
        }

        pub fn right(mut self, c: usize) -> Self {
            self.right = Some(c + 1);
            self
        }

        pub fn left(mut self, c: usize) -> Self {
            self.left = Some(c - 1);
            self
        }

        pub fn above(mut self, sq: usize, c: usize) -> Self {
            self.above = Some((sq - 1, c));
            self
        }

        pub fn below(mut self, sq: usize, c: usize) -> Self {
            self.below = Some((sq + 1, c));
            self
        }

        pub fn above_right(mut self, sq: usize, c: usize) -> Self {
            self.above_right = Some((sq - 1, c + 1));
            self
        }

        pub fn above_left(mut self, sq: usize, c: usize) -> Self {
            self.above_left = Some((sq - 1, c - 1));
            self
        }

        pub fn below_right(mut self, sq: usize, c: usize) -> Self {
            self.below_right = Some((sq + 1, c + 1));
            self
        }

        pub fn below_left(mut self, sq: usize, c: usize) -> Self {
            self.below_left = Some((sq + 1, c - 1));
            self
        }
    }
}

fn check_mines(
    pos: CellPosition,
    sq: usize,
    c: usize,
    field: &[&str],
    row: &str,
    output_row: &mut String,
) {
    use crate::direction::Direction;

    // it's not a needless init as clippy doesn't detect its use outside the match's scope
    #[allow(clippy::needless_late_init)]
    let direction: Direction;
    let mut counter = 0;

    match pos {
        CellPosition::UpperLeft => {
            direction = Direction::new().right(c).below(sq, c).below_right(sq, c)
        }
        CellPosition::UpperRight => {
            direction = Direction::new().left(c).below(sq, c).below_left(sq, c)
        }
        CellPosition::TopRow => {
            direction = Direction::new()
                .left(c)
                .right(c)
                .below(sq, c)
                .below_left(sq, c)
                .below_right(sq, c)
        }
        CellPosition::LowerLeft => {
            direction = Direction::new().right(c).above(sq, c).above_right(sq, c)
        }
        CellPosition::LowerRight => {
            direction = Direction::new().left(c).above(sq, c).above_left(sq, c)
        }
        CellPosition::BottomRow => {
            direction = Direction::new()
                .left(c)
                .right(c)
                .above(sq, c)
                .above_left(sq, c)
                .above_right(sq, c)
        }
        CellPosition::LeftColumn => {
            direction = Direction::new()
                .right(c)
                .above(sq, c)
                .below(sq, c)
                .above_right(sq, c)
                .below_right(sq, c)
        }
        CellPosition::RightColumn => {
            direction = Direction::new()
                .left(c)
                .above(sq, c)
                .below(sq, c)
                .above_left(sq, c)
                .below_left(sq, c)
        }
        CellPosition::Middle => {
            direction = Direction::new()
                .right(c)
                .left(c)
                .above(sq, c)
                .below(sq, c)
                .above_right(sq, c)
                .above_left(sq, c)
                .below_right(sq, c)
                .below_left(sq, c)
        }
    }

    // check to the right
    if let Some(v) = row.as_bytes().get(direction.right.unwrap()) {
        if *v == b'*' {
            counter += 1;
        }
    }

    // check to the left
    if let Some(v) = row.as_bytes().get(direction.left.unwrap()) {
        if *v == b'*' {
            counter += 1;
        }
    }

    // check above
    if let Some(v) = field.get(direction.above.unwrap().0) {
        if let Some(v) = v.as_bytes().get(direction.above.unwrap().1) {
            if *v == b'*' {
                counter += 1;
            }
        }
    }

    // check below
    if let Some(v) = field.get(direction.below.unwrap().0) {
        if let Some(v) = v.as_bytes().get(direction.below.unwrap().1) {
            if *v == b'*' {
                counter += 1;
            }
        }
    }

    // check above to the right
    if let Some(v) = field.get(direction.above_right.unwrap().0) {
        if let Some(v) = v.as_bytes().get(direction.above_right.unwrap().1) {
            if *v == b'*' {
                counter += 1;
            }
        }
    }

    // check above to the left
    if let Some(v) = field.get(direction.above_left.unwrap().0) {
        if let Some(v) = v.as_bytes().get(direction.above_left.unwrap().1) {
            if *v == b'*' {
                counter += 1;
            }
        }
    }

    // check below to the right
    if let Some(v) = field.get(direction.below_right.unwrap().0) {
        if let Some(v) = v.as_bytes().get(direction.below_right.unwrap().1) {
            if *v == b'*' {
                counter += 1;
            }
        }
    }

    // check below to the left
    if let Some(v) = field.get(direction.below_left.unwrap().0) {
        if let Some(v) = v.as_bytes().get(direction.below_left.unwrap().1) {
            if *v == b'*' {
                counter += 1;
            }
        }
    }

    if counter != 0 {
        output_row.push_str(&counter.to_string());
    } else {
        output_row.push(' ');
    }
}

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut output: Vec<String> = vec![];

    for sq in minefield.iter().enumerate() {
        let mut row = String::new();

        for c in sq.1.as_bytes().iter().enumerate() {
            if *c.1 != b'*' {
                match sq.0 {
                    0 => {
                        match c.0 {
                            // upper left corner
                            0 => check_mines(
                                CellPosition::UpperLeft,
                                sq.0,
                                c.0,
                                minefield,
                                sq.1,
                                &mut row,
                            ),
                            // upper right corner
                            _ if c.0 == sq.1.len() - 1 => check_mines(
                                CellPosition::UpperRight,
                                sq.0,
                                c.0,
                                minefield,
                                sq.1,
                                &mut row,
                            ),
                            // top row
                            _ => check_mines(
                                CellPosition::TopRow,
                                sq.0,
                                c.0,
                                minefield,
                                sq.1,
                                &mut row,
                            ),
                        }
                    }
                    _ if sq.0 == minefield.len() - 1 => {
                        match c.0 {
                            // lower left corner
                            0 => check_mines(
                                CellPosition::LowerLeft,
                                sq.0,
                                c.0,
                                minefield,
                                sq.1,
                                &mut row,
                            ),
                            // lower right corner
                            _ if c.0 == sq.1.len() - 1 => check_mines(
                                CellPosition::LowerRight,
                                sq.0,
                                c.0,
                                minefield,
                                sq.1,
                                &mut row,
                            ),
                            // bottom row
                            _ => check_mines(
                                CellPosition::BottomRow,
                                sq.0,
                                c.0,
                                minefield,
                                sq.1,
                                &mut row,
                            ),
                        }
                    }
                    _ => {
                        match c.0 {
                            // left column
                            0 => check_mines(
                                CellPosition::LeftColumn,
                                sq.0,
                                c.0,
                                minefield,
                                sq.1,
                                &mut row,
                            ),
                            // right column
                            _ if c.0 == sq.1.len() - 1 => check_mines(
                                CellPosition::RightColumn,
                                sq.0,
                                c.0,
                                minefield,
                                sq.1,
                                &mut row,
                            ),
                            // middle
                            _ => check_mines(
                                CellPosition::Middle,
                                sq.0,
                                c.0,
                                minefield,
                                sq.1,
                                &mut row,
                            ),
                        }
                    }
                }
            } else {
                row.push('*');
            }
        }

        output.push(row);
    }

    output
}
