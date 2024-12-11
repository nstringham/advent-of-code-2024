use advent_of_code_2024::parsed_input_lines;

fn main() {
    let reports = parsed_input_lines::<u8>();

    let safe_reports = reports.filter(|report| {
        let rising = report.first() < report.last();
        report
            .iter()
            .zip(report.iter().skip(1))
            .all(|(&a, &b)| (a < b) == rising && (1..=3).contains(&a.abs_diff(b)))
    });

    let answer = safe_reports.count();

    println!("{answer}");
}
