mod day1;
mod day2;

pub fn run_days(start: u32, end: u32) {
    if start > end { return }

    let mut now_day = start.saturating_sub(1);
    let now_day_ref: &mut u32 = &mut now_day;

    if print_day_info(now_day_ref, start, end) { day1::solve(); }

    if print_day_info(now_day_ref, start, end) { day2::solve(); }

    print_day_info(now_day_ref, start, end);
}

fn print_day_info(now_day: &mut u32, start: u32, end: u32) -> bool {
    let all_len = 40;

    if *now_day >= start && *now_day <= end {
        let now_day_str = format!("END OF DAY {}", now_day);

        let half_blank = "-".repeat(all_len + 3 - now_day_str.len());

        println!();
        println!("{half_blank}{now_day_str}{half_blank}");
        println!();

        if *now_day == end { *now_day += 1 }
    }

    if *now_day < end {
        *now_day += 1;

        let now_day_str = format!("DAY {}", now_day);

        let half_blank = "-".repeat(all_len - now_day_str.len());

        println!("{half_blank}{now_day_str}{half_blank}");
        println!();
    }

    *now_day >= start && *now_day <= end
}
