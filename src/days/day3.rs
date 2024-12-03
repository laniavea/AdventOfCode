use std::fs;

pub fn solve() {
    let file_data = fs::read("./days_data/day_3_input_data.txt").expect("Impossible to read_file");
    let file_content: String = String::from_utf8(file_data).expect("Error in file");

    let mut m_id = 0;
    let mut summed_res = 0;

    for (now_id, now_char) in file_content.bytes().enumerate() {
        match now_char {
            b'm' => m_id = now_id,
            b')' => {
                if now_id - m_id < 7 || now_id - m_id > 11 { continue; }
                let now_bytes = String::from_utf8(file_content.as_bytes().get(m_id..now_id).unwrap().to_vec());
                if let Some(mul_res) = analyze_string(&now_bytes.expect("Failed to create string")) {
                    summed_res += mul_res;
                }
            }
            _ => ()
        };
    }

    println!("Part 1: result {summed_res}");

    let mut m_id = 0;
    let mut d_id = 0;
    let mut summed_res = 0;
    let mut ins_enabled = true;

    for (now_id, now_char) in file_content.bytes().enumerate() {
        match now_char {
            b'm' => m_id = now_id,
            b'd' => d_id = now_id,
            b')' => {
                if m_id > d_id {
                    if now_id - m_id < 7 || now_id - m_id > 11 || !ins_enabled { continue; }
                    let now_st = String::from_utf8(file_content.as_bytes().get(m_id..now_id).unwrap().to_vec());
                    if let Some(mul_res) = analyze_string(&now_st.expect("Failed to create string")) {
                        summed_res += mul_res;
                    }
                } else {
                    match now_id - d_id {
                        3 => {
                            let now_st = String::from_utf8(file_content.as_bytes().get(d_id..now_id).unwrap().to_vec());
                            if now_st.expect("Failed to create string") == "do(" {
                                ins_enabled = true;
                            }
                        },
                        6 => {
                            let now_st = String::from_utf8(file_content.as_bytes().get(d_id..now_id).unwrap().to_vec());
                            if now_st.expect("Failed to create string") == "don't(" {
                                ins_enabled = false;
                            }
                        },
                        _ => ()
                    }
                }
            }
            _ => ()
        };
    }

    println!("Part 2: result {summed_res}")
}

fn analyze_string(st: &str) -> Option<i64> {
    if let Some(post_fix) = st.strip_prefix("mul(") {
        let mut sp_iter = post_fix.split(',');

        let first_num_st = sp_iter.next();
        let second_num_st = sp_iter.next();

        if first_num_st.is_some() && second_num_st.is_some() && sp_iter.next().is_none() {
            let first_num = first_num_st.unwrap().parse::<i64>();
            let second_num = second_num_st.unwrap().parse::<i64>();

            if let (Ok(first), Ok(second)) = (first_num, second_num) {
                return Some(first * second)
            }
        }
    }

    None
}
