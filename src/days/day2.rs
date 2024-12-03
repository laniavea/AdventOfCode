use std::fs;

/// Main idea for part 1
/// Checking abs diff and cummulating it, if abs diff differs from first and last elements abs substitution
/// then at least one of the elements inside it is cursed
///
/// Main idea for part2
/// Trying to find item iterating twice (with fast break) based on two possible situations: row inc, row dec.
/// When error found then taking such vars (a, pr_el, now_el, b), than we'll check, is pr_el
/// and now_el doesn't trigger error by removing one of them using a and b as possible neighbors.
/// If one of the elements can be removed safely, then setting pr_val according to 'safe' el
/// and setting var is_skipped to true state, because by program we can only remove 1 number from row.
pub fn solve() {
    let file_data = fs::read("./days_data/day_2_input_data.txt").expect("Impossible to read_file");
    let file_content: String = String::from_utf8(file_data).expect("Error in file");

    let mut source_data: Vec<Vec<i32>> = vec![];

    for now_st in file_content.split("\n") {
        if now_st.is_empty() { continue }
        source_data.push(now_st.split_whitespace().map(|val| val.parse().expect("Failed parse")).collect())
    }

    let mut safe_count = 0;

    'a: for now_row in &source_data {
        let mut pr_val = now_row[0];
        let first_val = pr_val;
        let mut all_diff = 0;

        for now_val in now_row.iter().skip(1) {
            let now_diff = (pr_val - now_val).abs();

            if now_diff > 3 || now_diff == 0 {
                continue 'a
            }

            all_diff += now_diff;
            pr_val = *now_val;
        }

        if all_diff == (pr_val - first_val).abs() {
            safe_count += 1;
        }
    }

    println!("Part 1: safe reports count: {safe_count}");

    let mut safe_count = 0;

    for now_row in &source_data {
        let first_val = now_row[0];
        let mut pr_val = first_val;

        let mut is_skipped = false;
        let mut sol_found = true;

        // Inc
        for (now_id, now_val) in now_row.iter().enumerate().skip(1) {
            let now_diff = now_val - pr_val;

            if !(1..=3).contains(&now_diff) {
                if is_skipped { 
                    sol_found = false;
                    break;
                }

                let mut allow_rem_pr = true;
                let mut allow_rem_now = true;

                if let Some(val) = now_row.get(now_id.wrapping_sub(2)) {
                    allow_rem_pr = (1..=3).contains(&(now_val - *val));
                };

                if let Some(val) = now_row.get(now_id + 1) {
                    if (1..=3).contains(&(*val - now_val)) {
                        allow_rem_pr &= true;
                    } else {
                        allow_rem_pr = false;
                    }

                    allow_rem_now = (1..=3).contains(&(*val - pr_val));
                }

                match (allow_rem_pr, allow_rem_now) {
                    (true, true) | (true, false) => {
                        pr_val = *now_val;
                        is_skipped = true;
                    },
                    (false, true) => {
                        is_skipped = true;
                    },
                    (false, false) => {
                        sol_found = false;
                        break;
                    }
                }
            } else {
                pr_val = *now_val;
            }
        }

        if sol_found {
            safe_count += 1;
            continue;
        }

        sol_found = true;
        is_skipped = false;

        let mut pr_val = first_val;

        // Dec
        for (now_id, now_val) in now_row.iter().enumerate().skip(1) {
            let now_diff = pr_val - now_val;

            if !(1..=3).contains(&now_diff) {
                if is_skipped { 
                    sol_found = false;
                    break;
                } let mut allow_rem_pr = true;
                let mut allow_rem_now = true;

                if let Some(val) = now_row.get(now_id.wrapping_sub(2)) {
                    allow_rem_pr = (1..=3).contains(&(*val - now_val));
                };

                if let Some(val) = now_row.get(now_id + 1) {
                    if (1..=3).contains(&(now_val - *val)) {
                        allow_rem_pr &= true;
                    } else {
                        allow_rem_pr = false;
                    }

                    allow_rem_now = (1..=3).contains(&(pr_val - *val));
                }

                match (allow_rem_pr, allow_rem_now) {
                    (true, true) | (true, false)=> {
                        pr_val = *now_val;
                        is_skipped = true;
                    },
                    (false, true) => {
                        is_skipped = true;
                    },
                    (false, false) => {
                        sol_found = false;
                        break;
                    }
                }
            } else {
                pr_val = *now_val;
            }
        }

        if sol_found {
            safe_count += 1;
            continue;
        }
    }

    println!("Part 2: safe reports count: {safe_count}");
}
