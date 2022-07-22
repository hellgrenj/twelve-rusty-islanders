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

fn main() {
    let islanders = init_islanders();
    let islanders_slice = &islanders[..];
    for islander in islanders_slice {
        println!("{}, {}", islander.name, islander.weight);
    }
    let simulation_result = find_odd_islander(&islanders);
    println!(
        "found {} islander {} with weight {}",
        simulation_result.diff, simulation_result.islander.name, simulation_result.islander.weight
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
    // make random islanders heavier
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
    for islander in islanders {
        if islander.weight % 2 != 0 {
            print!("{}, ", islander.name);
        }
    }
    let left = &islanders[0..4];
    let right = &islanders[4..8];
    let sideline = &islanders[8..12];
    let result = seesaw(left, right);
    match result {
        SeesawResult::Balanced => {
            return handle_balanced(left, right, sideline);
        }
        SeesawResult::Left => {
            panic!("not yet implemented!")
        }
        SeesawResult::Right => {
            panic!("not yet implemented!")
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
    // TODO how handle the measurement counter in a safe and good way..?
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
fn handle_balanced(left: &[Islander], right: &[Islander], sideline: &[Islander]) -> Simres {
    println!("reached balanced state");
    // seesaw: L1 L2 L3 L4   R1 R2 R3 R4 (balanced)
    // sideline: S1 S2 S3 S4
    // now compare S1 S2 S3 with R1 R2 R3
    let left_2 = &sideline[0..3];
    let right_2 = &right[0..3];
    let result_2 = seesaw(left_2, right_2);
    match result_2 {
        SeesawResult::Balanced => {
            println!("reached balanced state again");
            // seesaw: S1 S2 S3  R1 R2 R3  (balanced)
            // sideline: L1 L2 L3 S4 L4 R4
            println!("sideline: {:?}", sideline);
            let left_2_1 = &sideline[3..=3]; // S4
            let right_2_1 = &left[0..=0];
            let result_2_1 = seesaw(left_2_1, right_2_1);
            println!("left_2_1: {:?}", left_2_1);
            println!("right_2_1: {:?}", right_2_1);
            println!("result_2_1: {:?}", result_2_1);
            match result_2_1 {
                SeesawResult::Left => {
                    return Simres {
                        // heavy S4
                        islander: left_2_1[0].clone(),
                        diff: "heavy".to_string(),
                        number_of_seesaw_measurements: 3,
                    };
                }
                SeesawResult::Right | SeesawResult::Balanced => {
                    return Simres {
                        // light S4
                        islander: left_2_1[0].clone(),
                        diff: "light".to_string(),
                        number_of_seesaw_measurements: 3,
                    };
                }
            }
            // left21 := []islander{sideline[3]}
            // right21 := []islander{left2[0]}
            // // compare S4 with neutral L1
            // result21 := seesaw(left21, right21)
            // if result21 == "left" {
            // 	return simres{sideline[3], "heavy", numberOfSeesawMeasurements} // heavy S4
            // } else {
            // 	return simres{sideline[3], "light", numberOfSeesawMeasurements} // light S4
            // }
        }
        SeesawResult::Left => {
            panic!("not yet implemented!")
        }
        SeesawResult::Right => {
            panic!("not yet implemented!")
        }
    }
}
