use std::convert::TryInto;

pub fn annotate(input: &[&str]) -> Vec<String> {
    let minefield: Vec<Vec<bool>> = input
        .iter()
        .map(|row| {
            row.chars()
                .map(|character| match character {
                    '*' => true,
                    ' ' => false,
                    x => panic!("Invalid input character: '{}'", x),
                })
                .collect()
        })
        .collect();

    minefield
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, value)| match value {
                    false => match surrounding_mine_count(&minefield, &(x, y)) {
                        0 => ' ',
                        count => char::from_digit(count as u32, 10)
                            .expect("Surrounding mine count was inconceivably more than one digit (it should be at most 8 on a 2D grid)"),
                    },
                    true => '*',
                })
                .collect()
        })
        .collect()
}

const SURROUNDING_OFFSETS: [(i8, i8); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];
fn surrounding_mine_count(minefield: &Vec<Vec<bool>>, address: &(usize, usize)) -> u8 {
    SURROUNDING_OFFSETS
        .iter()
        .filter(|offset| {
            match (
                address.0.checked_add_signed(offset.0.into()),
                address.1.checked_add_signed(offset.1.into()),
            ) {
                (Some(x), Some(y)) => minefield
                    .get(y)
                    .and_then(|row| row.get(x))
                    .map(|&cell| cell)
                    .unwrap_or(false),
                _ => false,
            }
        })
        .count()
        .try_into()
        .expect("Surrounding mine count could not be converted into a u8 (it should be at most 8 on a 2D grid)")
}
