use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::time::Instant;

fn process_log(log_filename: &str, out_log_filename: &str) {
    let mut unique: HashMap<String, u32> = HashMap::new();
    let file = File::open(log_filename).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();

        // Быстрый анализ строки без лишних аллокаций
        if let Some(ip_end) = line.find(" - -") {
            let ip = &line[..ip_end];

            let os = if line.contains("Linux") {
                "Linux"
            } else if line.contains("Windows") {
                "Windows"
            } else if line.contains("Mac OS") {
                "Macintosh"
            } else {
                "Other"
            };

            // Формируем ключ за одну аллокацию
            let mut key = String::with_capacity(ip.len() + 1 + os.len());
            key.push_str(ip);
            key.push(':');
            key.push_str(os);

            *unique.entry(key).or_insert(0) += 1;
        }
    }

    // Запись результатов
    let file = File::create(out_log_filename).unwrap();
    let mut writer = BufWriter::new(file);
    for (item, count) in unique {
        writeln!(writer, "{}:{}", item, count).unwrap();
    }
}

fn main() {
    println!("--- анализ лога Rust ---");
    let start_time = Instant::now();

    process_log("nginx.log", "statlog_rust.txt");

    let duration = start_time.elapsed();
    println!("--- {} seconds ---", duration.as_secs_f64());
}
