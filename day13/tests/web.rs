#![cfg(target_arch = "wasm32")]

use day13::*;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

static INPUT: &str = include_str!("../input.txt");

#[wasm_bindgen_test]
fn day13_part1() {
    assert_eq!(part1(INPUT), Ok(216));
}

#[wasm_bindgen_test]
fn day13_part2() {
    assert_eq!(part2(INPUT), Ok(0));
}