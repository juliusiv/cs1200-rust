use std::env;
mod scores;

// // Track events
// const M_100: &str = "100_METERS";
// const M_400: &str = "400_METERS";
// const M_1500: &str = "1500_METERS";
// const M_110_HURDLES: &str = "110_METERS_HURDLES";
// // Field events
// const LONG_JUMP: &str = "LONG_JUMP";
// const SHOT_PUT: &str = "SHOT_PUT";
// const HIGH_JUMP: &str = "HIGH_JUMP";
// const DISCUS_THROW: &str = "DISCUS_THROW";
// const POLE_VAULT: &str = "POLE_VAULT";
// const JAVELIN_THROW: &str = "JAVELIN_THROW";

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        4 => match handle_command(&args[1], &args[2], &args[3]) {
            Ok(_) => println!(),
            Err(msg) => println!("{}", msg),
        },
        n => println!(
            "Incorrect number of arguments - excepted 4, received {}.",
            n
        ),
    }
}

fn handle_command(
    input_file_path: &String,
    output_file_path: &String,
    mode: &String,
) -> Result<(), &'static str> {
    if mode != "scores" && mode != "points" {
        return Err("Unknown mode");
    }

    scores::parse_file(&input_file_path, &output_file_path);

    Ok(())
}

// fn track_score(score) -> i32 {
//     let A = 25.4347;
//     let B = 18.0;
//     let C = 1.81;

//     int(A * (B - score)^C)
// }

// fn is_track(event) -> bool {
//     event == ""
// }
