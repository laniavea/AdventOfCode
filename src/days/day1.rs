use std::fs;
use std::collections::HashMap;

/// Main idea for part 1
/// Sorting and iterating at the same time
///
/// Main idea for part 2
/// Creating hashmap based on second column (number, number of entries in second column)
/// Iterating first and do math with needed item from hashmap
pub fn solve() {
    let file_data = fs::read("./days_data/day_1_input_data.txt").expect("Impossible to read_file");

    let mut first_num = 0;
    let mut second_num = 0;
    let mut is_first_filled = false;

    let mut first_column: Vec<i32> = vec![];
    let mut second_column: Vec<i32> = vec![];

    for now_sym in file_data { match now_sym { 0x20 => { is_first_filled = true; }, 0x0A => {
                first_column.push(first_num);
                second_column.push(second_num);

                is_first_filled = false;
                first_num = 0;
                second_num = 0;
            },
            b'0'..=b'9' => {
                if !is_first_filled {
                    first_num *= 10;
                    first_num += (now_sym - b'0') as i32
                } else {
                    second_num *= 10;
                    second_num += (now_sym - b'0') as i32
                }
            },
            _ => unreachable!(),
        }
    }

    first_column.sort();
    second_column.sort();

    let mut part_1_abs_diff: u64 = 0;
    for (now_first, now_sec) in first_column.iter().zip(&second_column) {
        part_1_abs_diff += now_first.abs_diff(*now_sec) as u64
    }

    println!("First part result: {part_1_abs_diff}");

    let mut sec_list_hash_map: HashMap<i32, u64> = HashMap::new();

    let mut now_num = *second_column.first().unwrap_or(&0);
    let mut now_count = 1;

    for now_sec in second_column.iter().skip(1) {
        if *now_sec != now_num {
            sec_list_hash_map.insert(now_num, now_count);
            now_num = *now_sec;
            now_count = 1;
        } else {
            now_count += 1;
        }
    }

    let mut part_2_abs_diff: u64 = 0;
    for now_first in first_column {
        let now_count = *sec_list_hash_map.get(&now_first).unwrap_or(&0);
        part_2_abs_diff += (now_first as u64) * now_count;
    }

    println!("Second part result: {part_2_abs_diff}");

}

