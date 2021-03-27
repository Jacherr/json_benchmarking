#![feature(test)]
extern crate test;

use std::fs::read_to_string;

const CURRENT_FILE: &str = "./data/canada.json";

fn read_file() -> String {
    let raw = read_to_string(CURRENT_FILE);
    raw.unwrap()
}

fn main() {}

#[bench]
fn sn(b: &mut test::Bencher) {
    let file = test::black_box(read_file());
    b.iter(|| {
        let mut parser = sn::Parser::new(file.as_bytes());
        parser.parse().unwrap()
    })
}

#[bench]
fn serde_json(b: &mut test::Bencher) {
    let file = test::black_box(read_file());
    b.iter(|| serde_json::from_str::<serde_json::Value>(&file).unwrap())
}

#[bench]
fn tinyjson(b: &mut test::Bencher) {
    let file = test::black_box(read_file());
    b.iter(|| file.parse::<tinyjson::JsonValue>().unwrap())
}

#[bench]
fn simple_json(b: &mut test::Bencher) {
    let file = test::black_box(read_file());
    b.iter(|| simple_json::Json::parse(&file).unwrap())
}

#[bench]
fn json_parser(b: &mut test::Bencher) {
    let file = test::black_box(read_file());
    b.iter(|| file.parse::<json_parser::parser::Json>().unwrap())
}

// this one is getting stuck or something ?? idk
// its here in case i fix it but idk, it just never completes
// the benchmark rn
/*
#[bench]
fn lite_json(b: &mut test::Bencher) {
    let file = test::black_box(read_file());
    b.iter(|| {
        lite_json::parse_json(&file).unwrap()
    })
}
*/

#[bench]
fn pjson_parser(b: &mut test::Bencher) {
    let file = test::black_box(read_file());
    b.iter(|| pjson_parser::parse_json(&file).unwrap())
}

#[bench]
fn erjson(b: &mut test::Bencher) {
    let file = test::black_box(read_file());
    b.iter(|| {
        let mut doc = erjson::JSONDocument::new();
        doc.parse_string(file.clone()).unwrap()
    })
}

#[bench]
fn json_peek(b: &mut test::Bencher) {
    let file = test::black_box(read_file());
    b.iter(|| json_peek::Parser::new(&file).parse().unwrap())
}

#[bench]
fn json_pop(b: &mut test::Bencher) {
    let file = test::black_box(read_file());
    b.iter(|| json_pop::parse_str(&file).unwrap())
}

#[bench]
fn jstr(b: &mut test::Bencher) {
    let file = test::black_box(read_file());
    b.iter(|| jstr::deserialize(&file).unwrap())
}

#[bench]
fn strason(b: &mut test::Bencher) {
    let file = test::black_box(read_file());
    b.iter(|| strason::Json::from_str(&file).unwrap())
}

#[bench]
fn smoljson(b: &mut test::Bencher) {
    let file = test::black_box(read_file());
    b.iter(|| smoljson::Value::from_str(&file).unwrap())
}

#[bench]
fn json(b: &mut test::Bencher) {
    let file = test::black_box(read_file());
    b.iter(|| json::parse(&file).unwrap())
}
