// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn expected_minutes_in_oven() -> i32 {
    40
}

pub fn remaining_minutes_in_oven(actual_minutes_in_oven: i32) -> i32 {
    if actual_minutes_in_oven > 40 {
        0
    } else {
        40 - actual_minutes_in_oven
    }
}

const MINUTE_PER_LAYER: i32 = 2;

pub fn preparation_time_in_minutes(number_of_layers: i32) -> i32 {
    number_of_layers * MINUTE_PER_LAYER
}

pub fn elapsed_time_in_minutes(number_of_layers: i32, actual_minutes_in_oven: i32) -> i32 {
    preparation_time_in_minutes(number_of_layers) + actual_minutes_in_oven
}
