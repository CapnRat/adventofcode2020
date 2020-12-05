use crate::solutions::input::get_data;

#[derive(Debug, Default, Eq, PartialEq)]
pub struct BoardingPass {
    pub row: u8,
    pub column: u8
}

impl BoardingPass {
    pub fn get_seat_id (&self) -> u16
    {
        (self.row as u16) * 8 + (self.column as u16)
    }
}

pub fn run() -> String {
    let passes: Vec<BoardingPass> = get_data("input/input_05", parse_bsp);

    passes.iter().map(|bp| bp.get_seat_id()).max().unwrap().to_string()
}

pub fn parse_bsp (line: &str) -> BoardingPass {
    let mut row_range = 128;
    let mut row_offset = 1;
    let mut column_range = 8;
    let mut column_offset = 1;
    for char in line.chars().into_iter() {
        match char {
            'F' => row_range = row_range / 2,
            'B' => {
                row_range = row_range / 2;
                row_offset += row_range;
            }
            'L' => column_range = column_range / 2,
            'R' => {
                column_range = column_range / 2;
                column_offset += column_range;
            }
            _ => panic!("unexpected char")
        }
    }

    BoardingPass { row: row_offset - 1, column: column_offset - 1 }
}

#[test]
fn can_parse_input() {
    let expected_passes = [
        BoardingPass { row: 44 , column: 5},
        BoardingPass { row: 70 , column: 7},
        BoardingPass { row: 14 , column: 7},
        BoardingPass { row: 102, column: 4},
    ];
    let expected_ids = [357, 567, 119, 820];

    for (i, result_pass) in get_data("input/input_05_test", parse_bsp).into_iter().enumerate() {
        assert_eq!(expected_passes[i], result_pass);
        assert_eq!(expected_ids[i], result_pass.get_seat_id());
    }
}