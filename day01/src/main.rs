#![allow(unused)]
use std::env;
use std::fs;

fn get_lists_from_file(file_path: String) -> Vec<(char, i32)> {
  let contents = fs::read_to_string(file_path)
      .expect(r#"Should have been able to read the file"#);

  contents.split("\n")
    .map(|s| {
        let mut all_chars = s.trim().chars();
        let direction = all_chars.nth(0).unwrap_or_default();
        let distance = all_chars.collect::<String>().parse::<i32>().unwrap_or_default();
        (direction, distance)
    }).collect::<Vec<(char, i32)>>()
}

fn rotate(sequence: &mut [i32; 100], steps: i32) {
    if (steps > 0) {
        sequence.rotate_left(steps as usize);
        println!("- The dial is rotated by `R{}` to point at `{}`", steps, sequence.first().unwrap());
    } else {
        sequence.rotate_right(-steps as usize);
        println!("- The dial is rotated by `L{}` to point at `{}`", -steps, sequence.first().unwrap());
    }
}

fn main() {
    env::args().enumerate().for_each(|(i, arg)| {
        println!("Arg {}: {}", i, arg);
    });

    let file_path = String::from(r#"c:\repos\aoc2025\day01\big.txt"#);
    
    let instructions = get_lists_from_file(file_path);

    let mut mutable_instructions = instructions
        .iter().map(|instruction| {
            let (direction, step) = *instruction;
            match direction {
                'L' => -step,
                'R' => step,
                _ => 0,
            }
        })
        .collect::<Vec<i32>>();

    const STARTING_POSITION: usize = 50;
    const NUM_POSITIONS: usize = 100;
    let mut sequence:[i32; NUM_POSITIONS] = std::array::from_fn(|i| i as i32);

    sequence.rotate_right(STARTING_POSITION);
    println!("- The dial starts by pointing at `{}`", sequence.first().unwrap());
    let mut cursor = STARTING_POSITION as i32;
    let mut cursor_points_at_zero = false;
    mutable_instructions.iter_mut().for_each(|step| {
        let mut step_mod = *step % NUM_POSITIONS as i32;
        let multiplier = (*step).abs() / NUM_POSITIONS as i32;

        cursor += step_mod;
        let zero_click = !cursor_points_at_zero && cursor <= 0 || cursor >= NUM_POSITIONS as i32;
        if (cursor < 0) {
            cursor = NUM_POSITIONS as i32 + cursor;
        } else {
            cursor = cursor % NUM_POSITIONS as i32;
        }

        *step = zero_click as i32 + multiplier;

        cursor_points_at_zero = cursor == 0;

        rotate(&mut sequence, step_mod);
    });
    let password:i32 = mutable_instructions.iter().sum();

    println!("The password is `{}`", password);

}
