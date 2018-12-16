use std::fs;
use std::io;

fn wrap_index(idx: usize, diff: i64, len: usize) -> usize {
    (((idx + (len * diff.abs() as usize)) as i64) + diff) as usize % len
}

fn main() -> io::Result<()> {
    let NUM_PLAYERS = 468;
    let LAST_VALUE = 7101000;

    let mut circle = vec![0];
    let mut current_position: usize = 0;
    let mut next_value: i64 = 1;
    let mut player_scores: Vec<i64> = vec![0; NUM_PLAYERS];

    loop {
        for (player, score) in player_scores.iter_mut().enumerate() {
            if next_value % 23 == 0 {
                *score += next_value;
                current_position = wrap_index(current_position, -7, circle.len());
                *score += circle.remove(current_position);
            } else {
                let insert_position = wrap_index(current_position, 1, circle.len()) + 1;
                circle.insert(insert_position, next_value);
                current_position = insert_position;
            }
            //println!("[{}] {:?}", player + 1, circle);
            if next_value == LAST_VALUE {
                break;
            }
            next_value += 1;
        }

        if next_value == LAST_VALUE {
            break;
        }
    }

    println!("{}", player_scores.iter().max().unwrap());
    Ok(())
}
