use rand::Rng;

#[derive(Debug, Clone)]
struct Islander {
    name: String,
    weight: i32,
}

struct Simres {
    islander: Islander,
    diff: String,
    number_of_seesaw_measurements: i32,
}
const DEFAULT_WEIGHT: i32 = 100;
static mut SEESAW_COUNTER: i32 = 0;
fn main() {
    let islanders = init_islanders();
    let simulation_result = find_odd_islander(&islanders);
    println!(
        "found {} islander {} with weight {} in {} seesaw measurements",
        simulation_result.diff,
        simulation_result.islander.name,
        simulation_result.islander.weight,
        simulation_result.number_of_seesaw_measurements
    );
}

fn init_islanders() -> Vec<Islander> {
    let mut islanders: Vec<Islander> = Vec::new();
    for i in 0..12 {
        let islander = Islander {
            name: (i + 1).to_string(),
            weight: DEFAULT_WEIGHT,
        };
        islanders.push(islander);
    }
    // make random islander randomly heavier or lighter
    let random_index = get_random_number(0, 11) as usize;
    let random_weigth = get_random_number(80, 120);
    islanders[random_index].weight = random_weigth;
    islanders
}
fn get_random_number(min: i32, max: i32) -> i32 {
    let random_numb = rand::thread_rng().gen_range(min..=max);
    if random_numb == DEFAULT_WEIGHT {
        return get_random_number(min, max);
    } else {
        return random_numb;
    }
}
fn find_odd_islander(islanders: &Vec<Islander>) -> Simres {
    unsafe {
        // reset seesaw counter
        SEESAW_COUNTER = 0;
    }
    let left = &islanders[0..4];
    let right = &islanders[4..8];
    let sideline = &islanders[8..12];
    let result = seesaw(left, right);
    match result {
        SeesawResult::Balanced => {
            return handle_balanced(right, sideline);
        }
        SeesawResult::Left => {
            return handle_left_heavy(left, right, sideline);
        }
        SeesawResult::Right => {
            // make sure you have the heavy side on the left... i.e pass in right as left and left as right
            return handle_left_heavy(right, left, sideline);
        }
    }
}
#[derive(Debug)]
enum SeesawResult {
    Balanced,
    Left,
    Right,
}
fn seesaw(left: &[Islander], right: &[Islander]) -> SeesawResult {
    unsafe {
        // its fine tho...this program only ever use the one thread
        SEESAW_COUNTER += 1;
    }
    let total_left = left.iter().fold(0, |acc, islander| acc + islander.weight);
    let total_right = right.iter().fold(0, |acc, islander| acc + islander.weight);
    if total_left == total_right {
        return SeesawResult::Balanced;
    } else if total_left > total_right {
        return SeesawResult::Left;
    } else {
        return SeesawResult::Right;
    }
}
fn handle_balanced(right: &[Islander], sideline: &[Islander]) -> Simres {
    // seesaw: L1 L2 L3 L4   R1 R2 R3 R4 (balanced)
    // sideline: S1 S2 S3 S4
    // now compare S1 S2 S3 with R1 R2 R3
    let left_2 = &sideline[0..3];
    let right_2 = &right[0..3];
    let result_2 = seesaw(left_2, right_2);
    match result_2 {
        SeesawResult::Balanced => {
            // seesaw 2: S1 S2 S3  R1 R2 R3  (balanced)
            // sideline 2: L1 L2 L3 S4 L4 R4
            let left_2_1 = &sideline[3..=3]; // S4
            let right_2_1 = &left_2[0..=0];
            let result_2_1 = seesaw(left_2_1, right_2_1);
            match result_2_1 {
                SeesawResult::Left => {
                    return Simres {
                        // heavy S4
                        islander: left_2_1[0].clone(),
                        diff: "heavy".to_string(),
                        number_of_seesaw_measurements: unsafe { SEESAW_COUNTER },
                    };
                }
                SeesawResult::Right | SeesawResult::Balanced => {
                    return Simres {
                        // light S4
                        islander: left_2_1[0].clone(),
                        diff: "light".to_string(),
                        number_of_seesaw_measurements: unsafe { SEESAW_COUNTER },
                    };
                }
            }
        }
        SeesawResult::Left => {
            // seesaw  2: S1 S2 S3 (heavy)  R1 R2 R3
            // sideline 2: L1 L2 L3 S4 L4 R4
            let left_2_2 = &left_2[0..=0]; // S1
            let right_2_2 = &left_2[1..=1]; // S2
                                            // compare S1 with S2
            let result_2_2 = seesaw(left_2_2, right_2_2);
            match result_2_2 {
                SeesawResult::Balanced => {
                    return Simres {
                        // heavy S3
                        islander: left_2[2].clone(),
                        diff: "heavy".to_string(),
                        number_of_seesaw_measurements: unsafe { SEESAW_COUNTER },
                    };
                }
                SeesawResult::Left => {
                    return Simres {
                        // heavy S1
                        islander: left_2_2[0].clone(),
                        diff: "heavy".to_string(),
                        number_of_seesaw_measurements: unsafe { SEESAW_COUNTER },
                    };
                }
                SeesawResult::Right => {
                    return Simres {
                        // heavy S2
                        islander: right_2_2[0].clone(),
                        diff: "heavy".to_string(),
                        number_of_seesaw_measurements: unsafe { SEESAW_COUNTER },
                    };
                }
            }
        }
        SeesawResult::Right => {
            // seesaw 2: S1 S2 S3  R1 R2 R3 (heavy)
            // sideline 2: L1 L2 L3 S4 L4 R4

            let left_2_3 = &left_2[0..=0]; // S1
            let right_2_3 = &left_2[1..=1]; // S2

            // // compare S1 and S2
            let result_2_3 = seesaw(left_2_3, right_2_3);
            match result_2_3 {
                SeesawResult::Balanced => {
                    return Simres {
                        // light S3
                        islander: left_2[2].clone(),
                        diff: "light".to_string(),
                        number_of_seesaw_measurements: unsafe { SEESAW_COUNTER },
                    };
                }
                SeesawResult::Left => {
                    return Simres {
                        // light S2
                        islander: right_2_3[0].clone(),
                        diff: "light".to_string(),
                        number_of_seesaw_measurements: unsafe { SEESAW_COUNTER },
                    };
                }
                SeesawResult::Right => {
                    return Simres {
                        // light S1
                        islander: left_2_3[0].clone(),
                        diff: "light".to_string(),
                        number_of_seesaw_measurements: unsafe { SEESAW_COUNTER },
                    };
                }
            }
        }
    }
}
fn handle_left_heavy(left: &[Islander], right: &[Islander], sideline: &[Islander]) -> Simres {
    // seesaw: L1 L2 L3 L4 (heavy)      R1 R2 R3 R4
    // sideline: S1 S2 S3 S4
    // now switch R1 R2 R3 and S2 S3 S4 AND switch R1 AND L1
    let left_2 = [&right[0..=0], &left[1..=1], &left[2..=2], &left[3..=3]].concat();
    let right_2 = [
        &left[0..=0],
        &sideline[1..=1],
        &sideline[2..=2],
        &sideline[3..=3],
    ]
    .concat();
    let sideline_2 = [
        &sideline[0..=0],
        &right[1..=1],
        &right[2..=2],
        &right[3..=3],
    ]
    .concat();

    let result_2 = seesaw(&left_2, &right_2);
    match result_2 {
        SeesawResult::Left => {
            // left still heavy
            // we now know there is a heavy islander amongst L2 L3 and L4
            // seesaw: R1 *L2 L3 L4*    L1 S2 S3 S4
            // sideline: S1 R2 R3 R4
            let left_2_1 = &left_2[1..=1]; // L2
            let right_2_1 = &left_2[2..=2]; // L3
                                            // // now compare L2 and L3
            let result_2_1 = seesaw(left_2_1, right_2_1);
            match result_2_1 {
                SeesawResult::Balanced => {
                    return Simres {
                        // heavy L4
                        islander: left[3].clone(),
                        diff: "heavy".to_string(),
                        number_of_seesaw_measurements: unsafe { SEESAW_COUNTER },
                    };
                }
                SeesawResult::Left => {
                    return Simres {
                        // heavy L2
                        islander: left_2_1[0].clone(),
                        diff: "heavy".to_string(),
                        number_of_seesaw_measurements: unsafe { SEESAW_COUNTER },
                    };
                }
                SeesawResult::Right => {
                    return Simres {
                        // heavy L3
                        islander: right_2_1[0].clone(),
                        diff: "heavy".to_string(),
                        number_of_seesaw_measurements: unsafe { SEESAW_COUNTER },
                    };
                }
            }
        }
        SeesawResult::Balanced => {
            // we now know that the light islander is amongst R2 R3 and R4
            // seesaw: R1 L2 L3 L4    L1 S2 S3 S4
            // sideline: S1 *R2 R3 R4*
            let left_2_2 = &sideline_2[1..=1]; // R2
            let right_2_2 = &sideline_2[2..=2]; // R3
                                                // now compare R2 and R3
            let result_2_2 = seesaw(left_2_2, right_2_2);
            match result_2_2 {
                SeesawResult::Balanced => {
                    return Simres {
                        // light R4
                        islander: sideline_2[3].clone(),
                        diff: "light".to_string(),
                        number_of_seesaw_measurements: unsafe { SEESAW_COUNTER },
                    };
                }
                SeesawResult::Left => {
                    return Simres {
                        // light R3
                        islander: right_2_2[0].clone(),
                        diff: "light".to_string(),
                        number_of_seesaw_measurements: unsafe { SEESAW_COUNTER },
                    };
                }
                SeesawResult::Right => {
                    return Simres {
                        // light R2
                        islander: left_2_2[0].clone(),
                        diff: "light".to_string(),
                        number_of_seesaw_measurements: unsafe { SEESAW_COUNTER },
                    };
                }
            }
        }
        SeesawResult::Right => {
            // right side now heavy
            // we now know it is either R1 or L1
            // seesaw: *R1* L2 L3 L4    *L1* S2 S3 S4 (heavy)
            // sideline: S1 R2 R3 R4
            // left23 := []islander{left2[0]}
            let left_2_3 = &left_2[0..=0]; // R1

            // right23 := []islander{sideline2[0]} // known neutral islander ...
            let right_2_3 = &sideline_2[0..=0]; // S1

            // // now compare R1 against a neutral islander (S1 for example)
            let result_2_3 = seesaw(left_2_3, right_2_3);
            match result_2_3 {
                SeesawResult::Balanced => {
                    return Simres {
                        // heavy L1
                        islander: right_2[0].clone(),
                        diff: "heavy".to_string(),
                        number_of_seesaw_measurements: unsafe { SEESAW_COUNTER },
                    };
                }
                SeesawResult::Left | SeesawResult::Right => {
                    return Simres {
                        // light R1
                        islander: left_2_3[0].clone(),
                        diff: "light".to_string(),
                        number_of_seesaw_measurements: unsafe { SEESAW_COUNTER },
                    };
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_find_odd_islander() {
        // test all variants of lighter islander
        for i in 0..12 {
            let islanders = get_test_islanders(i, 80);
            let result = find_odd_islander(&islanders);
            assert_eq!(result.islander.name, islanders[i].name);
            assert_eq!(result.islander.weight, 80);
            assert_eq!(result.diff, "light");
            assert_eq!(result.number_of_seesaw_measurements, 3);
        }
        // test all variants of heavier islander
        for i in 0..12 {
            let islanders = get_test_islanders(i, 120);
            let result = find_odd_islander(&islanders);
            assert_eq!(result.islander.name, islanders[i].name);
            assert_eq!(result.islander.weight, 120);
            assert_eq!(result.diff, "heavy");
            assert_eq!(result.number_of_seesaw_measurements, 3);
        }
    }

    fn get_test_islanders(index_odd_one: usize, weight: i32) -> Vec<Islander> {
        let mut islanders: Vec<Islander> = Vec::new();
        for i in 0..12 {
            let islander = Islander {
                name: (i + 1).to_string(),
                weight: DEFAULT_WEIGHT,
            };
            islanders.push(islander);
        }
        islanders[index_odd_one].weight = weight;
        islanders
    }
}
