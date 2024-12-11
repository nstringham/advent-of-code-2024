use advent_of_code_2024::parsed_input_lines;

fn valid_decreasing_step(step: i8) -> bool {
    (-3..=-1).contains(&step)
}

fn valid_increasing_step(step: i8) -> bool {
    (1..=3).contains(&step)
}

fn main() {
    let reports = parsed_input_lines::<i8>();

    let safe_reports = reports.filter(|report| {
        let mut decreasing_skip = None;
        let mut increasing_skip = None;
        let mut could_be_decreasing = true;
        let mut could_be_increasing = true;

        let step_without = |i: usize| report[i + 1] - report[i - 1];

        for i in 1..report.len() {
            let step = report[i] - report[i - 1];

            if could_be_decreasing && !valid_decreasing_step(step) && decreasing_skip != Some(i - 1)
            {
                if decreasing_skip.is_none()
                    && (i == report.len() - 1 || valid_decreasing_step(step_without(i)))
                {
                    decreasing_skip = Some(i);
                } else if decreasing_skip.is_none()
                    && (i == 1 || valid_decreasing_step(step_without(i - 1)))
                {
                    decreasing_skip = Some(i - 1);
                } else {
                    if could_be_increasing {
                        could_be_decreasing = false;
                    } else {
                        return false;
                    }
                }
            }

            if could_be_increasing && !valid_increasing_step(step) && increasing_skip != Some(i - 1)
            {
                if increasing_skip.is_none()
                    && (i == report.len() - 1 || valid_increasing_step(step_without(i)))
                {
                    increasing_skip = Some(i);
                } else if increasing_skip.is_none()
                    && (i == 1 || valid_increasing_step(step_without(i - 1)))
                {
                    increasing_skip = Some(i - 1);
                } else {
                    if could_be_decreasing {
                        could_be_increasing = false;
                    } else {
                        return false;
                    }
                }
            }
        }

        true
    });

    let answer = safe_reports.count();

    println!("{answer}");
}
