use end_of_month::eom;
use chrono::NaiveDate;

fn main() {
    // Example 1: From string in MM/DD/YYYY format
    let date_str = "07/15/2025";
    let eom1 = eom(date_str);
    println!("End of month for {}: {}", date_str, eom1);  // Outputs: 2025-07-31

    // Example 2: From a NaiveDate object
    let input_date = NaiveDate::from_ymd_opt(2025, 2, 3).unwrap();
    let eom2 = eom(input_date);
    println!("End of month for {}: {}", input_date, eom2);  // Outputs: 2025-02-28

    // Example 3: No input = uses today's date
    let eom3 = eom(None);
    println!("End of current month: {}", eom3);
}
