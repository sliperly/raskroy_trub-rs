#![allow(dead_code)]
use regex::Regex;
const L_TOTAL: u32 = 6000;
const L_CALIBRATION: u32 = 30;
const L_LOSS: u32 = 255;
const L_WORK: u32 = L_TOTAL-L_LOSS;
const WASTE_BAND_1_MAX: u32 = 325;
const WASTE_BAND_2_MAX: u32 = 1250;
const TIME_BUDGET_SECONDS: f64 = 30.0;

fn sanitize_text(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn sanitize_filename(name: &str) -> String {
    let cleaned: String = name
        .chars()
        .map(|c| if "\\/:*?\"<>|".contains(c) { '-' } else { c })
        .collect();
    let cleaned = sanitize_text(&cleaned);
    if cleaned.is_empty() {
        "Без_названия".to_string()
    } else {
        cleaned
    }
}
                                
fn extract_length(raw_name: &str) -> Result<f64, String> {
    let text = raw_name.trim();

    let re_prefix = Regex::new(r"(?i)^(тр|труба)").unwrap();
    if !re_prefix.is_match(text) {
        return Err("не распознан префикс трубы".to_string());
    }

    let re_diam = Regex::new(r"(?i)(диаметр|диам|d)\s*[\d,.]+\s*мм").unwrap();
    let re_hole = Regex::new(r"(?i)(отверстие|отверстия|отв)\s*[\d,.]+\s*мм").unwrap();
    let masked = re_diam.replace_all(text, " MASKED ");
    let masked = re_hole.replace_all(&masked, " MASKED ");

    let re_len = Regex::new(r"(?i)(\d+(?:[,.]\d+)?)\s*мм").unwrap();
    match re_len.captures(&masked) {
        Some(caps) => {
            let val_str = caps[1].replace(',', ".");
            val_str
                .parse::<f64>()
                .map_err(|_| "некорректный формат длины".to_string())
        }
        None => Err("не найдена длина".to_string()),
    }
}

fn main() {
    println!("{:?}", extract_length("Тр 40х40 1500 мм"));
    println!("{:?}", extract_length("Труба d20 мм 2500 мм"));
    println!("{:?}", extract_length("Тр 1250,5 мм"));
    println!("{:?}", extract_length("Тр без длины"));
    println!("{:?}", extract_length("Лист 500 мм"));

}
