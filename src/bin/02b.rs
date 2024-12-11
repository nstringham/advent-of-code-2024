use advent_of_code_2024::parsed_input_lines;

fn main() {
    let reports = parsed_input_lines::<u8>();

    fn is_safe(report: &[u8]) -> bool {
        let rising = report.first() < report.last();
        report
            .iter()
            .zip(report.iter().skip(1))
            .all(|(&a, &b)| (a < b) == rising && (1..=3).contains(&a.abs_diff(b)))
    }

    let safe_reports = reports.filter(|report| {
        if is_safe(report) {
            return true;
        }

        for i in 0..report.len() {
            let mut report = report.clone();
            report.remove(i);
            if is_safe(&report) {
                return true;
            }
        }

        false
    });

    let answer = safe_reports.count();

    println!("{answer}");
}
