use serde_json::Value;
use serde_json::Value::*;
use std::fs;

fn main() {
    let input = fs::read_to_string("../input.txt").expect("Failed to read file");

    let json = serde_json::from_str(&input).unwrap();

    println!(
        "Part 1: the sum of all numbers is {}",
        sum_numbers(&json)
    );
    println!(
        "Part 2: removing \"red\", the sum of all numbers is {}",
        sum_numbers_filter(&json, "red")
    );
}

fn sum_numbers(json: &Value) -> i64 {
    let mut total = 0;

    match json {
        Number(_) => {
            total += json.as_i64().unwrap();
        }
        Array(arr) => {
            for v in arr {
                total += sum_numbers(&v);
            }
        }
        Object(obj) => {
            for v in obj.values() {
                total += sum_numbers(&v);
            }
        }
        _ => {}
    }

    total
}

fn sum_numbers_filter(json: &Value, filter: &str) -> i64 {
    let mut total = 0;

    match json {
        Number(_) => {
            total += json.as_i64().unwrap();
        }
        Array(arr) => {
            for v in arr {
                total += sum_numbers_filter(&v, filter);
            }
        }
        Object(obj) => {
            let mut tmp = 0;
            for v in obj.values() {
                match v {
                    String(str) => {
                        if str.eq(filter) {
                            tmp = 0;
                            break;
                        }
                    }
                    _ => tmp += sum_numbers_filter(&v, filter),
                }
            }
            total += tmp;
        }
        _ => {}
    }

    total
}
