use std::env;

use rand::Rng;

const APPLICATION_VERSION: &str = "0.2.0";

const ALERT_STATE_NO_ALERT: usize = 0;
const ALERT_STATE_PASSIVE_ALERT: usize = 1;
const ALERT_STATE_ACTIVE_ALERT: usize = 2;
const ALERT_STATE_SHUTDOWN: usize = 3;

const STATE_TRANSITION_TO_PASSIVE: &str = "=> Passive";
const STATE_TRANSITION_TO_ACTIVE: &str = "=> Active";
const STATE_TRANSITION_TO_SHUTDOWN: &str = "=> Shutdown";


fn main() {
    println!("Security sheaf generator - {}", APPLICATION_VERSION);

    let args: Vec<String> = env::args().collect();

    let mut args_iterator = args.iter();

    let mut system_security_code: String = "blue".to_string();
    let mut system_security_value: usize = 8;

    // Skip the application name thing ($0)
    args_iterator.next();

    loop {
        match args_iterator.next() {
            Some(parameter) => {
                match parameter.as_str() {
                    "--security_code" => {
                        system_security_code = match args_iterator.next() {
                            Some(sec_code_as_parm) => { sec_code_as_parm.clone()}
                            None => { println!("EEE the parameter was not there for --security_code"); "blue".to_string() }
                        }
                        // TODO V fail in not a valid security code
                    }
                    "--security_value" => {
                        system_security_value = match args_iterator.next() {
                            Some(sec_code_as_parm) => { match sec_code_as_parm.parse::<usize>() {
                                Ok(sev_value) => sev_value,
                                Err(err) => {println!("EEE the parameter was not a number for --security_value. {err}"); 999},
                              }}
                            None => { println!("EEE the parameter was not there for --security_code"); 888 }
                        }
                        // TODO V fail in not a valid security code
                    }
                    "--help" => show_help(),
                    _ => println!("EEE unknown option: '{}'", parameter),
                }
            }
            None => {
                break;
            }
        }
    }

    let die_roll_modifier = get_die_roll_modifier_for_security_code(&system_security_code);

    println!("DDD System Security Code: {}, Die roll modifier: {}, Security Value: {}", system_security_code, die_roll_modifier, system_security_value);


    let mut trigger_step: usize = 0;
    let mut current_alert_state: usize = ALERT_STATE_NO_ALERT;
    let mut events_in_state: usize = 0;

    println!("{}\t{}", "Step", "Event");
    while (current_alert_state != ALERT_STATE_SHUTDOWN) && (trigger_step < 100) {
        trigger_step += (d6() / 2) + die_roll_modifier;
        println!("{}\t{}", trigger_step, get_alert_entry(&mut current_alert_state, &mut events_in_state, system_security_value));
    }
}

fn get_alert_entry(current_alert_state: &mut usize, events_in_state: &mut usize, system_security_value: usize) -> String {
    let event: String = match *current_alert_state {
        ALERT_STATE_NO_ALERT => get_no_alert_entry(d6()+*events_in_state, system_security_value),
        ALERT_STATE_PASSIVE_ALERT => get_passive_alert_entry(d6()+*events_in_state, system_security_value),
        ALERT_STATE_ACTIVE_ALERT => get_active_alert_entry(d6()+*events_in_state, system_security_value),
        _ => STATE_TRANSITION_TO_SHUTDOWN.to_string(),
    };

    match event.as_str() {
        STATE_TRANSITION_TO_PASSIVE => {*current_alert_state = ALERT_STATE_PASSIVE_ALERT; *events_in_state = 0},
        STATE_TRANSITION_TO_ACTIVE => {*current_alert_state = ALERT_STATE_ACTIVE_ALERT; *events_in_state = 0},
        STATE_TRANSITION_TO_SHUTDOWN => {*current_alert_state = ALERT_STATE_SHUTDOWN; *events_in_state = 0},
        _ => { *events_in_state += 1 }
    }
    event
}

fn get_no_alert_entry(trigger_step: usize, system_security_value: usize) -> String {
    match trigger_step {
        1..=3 => reactive_white_ic(system_security_value),
        4..=5 => proactive_white_ic(system_security_value),
        6..=7 => reactive_gray_ic(system_security_value),
        8.. => STATE_TRANSITION_TO_PASSIVE.to_string(),
        _ => { println!("EEE Unexpected trigger level: {}", trigger_step); "EEE".to_string() }
        
    }
}

fn get_passive_alert_entry(trigger_step: usize, system_security_value: usize) -> String {
    match trigger_step {
        1..=3 => proactive_white_ic(system_security_value),
        4 | 5 => reactive_gray_ic(system_security_value),
        6 | 7 => proactive_gray_ic(system_security_value),
        8.. => STATE_TRANSITION_TO_ACTIVE.to_string(),
        _ => { println!("EEE Unexpected trigger level: {}", trigger_step); "EEE".to_string() }
        
    }
}

fn get_active_alert_entry(trigger_step: usize, system_security_value: usize) -> String {
    match trigger_step {
        1..=3 => proactive_white_ic(system_security_value),
        4 | 5 => proactive_gray_ic(system_security_value),
        6 | 7 => black_ic(system_security_value),
        8.. => STATE_TRANSITION_TO_SHUTDOWN.to_string(),
        _ => { println!("EEE Unexpected trigger level: {}", trigger_step); "EEE".to_string() }
        
    }
}

fn reactive_white_ic(system_security_value: usize) -> String {
    match d6() {
        1 | 2 => format!("Probe{} \t\t", ic_rating_list(system_security_value)),
        3..=5 => format!("Trace{} \t\t", ic_rating_list(system_security_value)),
        6 => format!("Tar Baby{} \t\t", ic_rating_list(system_security_value)),
        _ => { println!("EEE Unexpected d6 result in reactive_white_ic()"); "EEE".to_string() }
    }
}

fn reactive_gray_ic(system_security_value: usize) -> String {
    match d6() {
        1 | 2 => format!("Tar Pit{} \t\t", ic_rating_list(system_security_value)),
        3 => ic_with_trap("Trace", system_security_value),
        4 => ic_with_trap("Probe", system_security_value),
        5 => ic_with_trap("Scout", system_security_value),
        6 => "Construct**\t\t".to_string(),
        _ => { println!("EEE Unexpected d6 result in reactive_gray_ic()"); "EEE".to_string() }
    }
}

fn proactive_white_ic(system_security_value: usize) -> String {
    match d6() + d6() {
        2..=5 => crippler_ripper(format!("Crippler{}",ic_rating_list(system_security_value)).as_str()),
        6..=8 => crippler_ripper(format!("Killer{}",ic_rating_list(system_security_value)).as_str()),
        9..=11 => format!("Scout{}   \t\t",ic_rating_list(system_security_value)),
        12 => "Construct**\t\t".to_string(),
        _ => { println!("EEE Unexpected d6 result in proactive_white_ic()"); "EEE".to_string() }
    }
}

fn proactive_gray_ic(system_security_value: usize) -> String {
    match d6() + d6() {
        2..=5 => crippler_ripper(format!("Ripper{}",ic_rating_list(system_security_value)).as_str()),
        6..=8 => format!("Blaster{}\t\t",ic_rating_list(system_security_value)),
        9..=11 => format!("Sparky{}\t\t",ic_rating_list(system_security_value)),
        12 => "Construct**\t\t".to_string(),
        _ => { println!("EEE Unexpected d6 result in proactive_gray_ic()"); "EEE".to_string() }
    }
}

fn black_ic(system_security_value: usize) -> String {
    match d6() + d6() {
        2..=4 => "Psychotropic*".to_string(),
        5..=7 => lethal_black_ic(system_security_value),
        8..=10 => non_lethal_black_ic(system_security_value),
        11 => "Cerebropathic".to_string(),
        12 => "Construct**".to_string(),
        _ => { println!("EEE Unexpected d6 result in black_ic()"); "EEE".to_string() }
    }
}

// SR3-223
fn lethal_black_ic(system_security_value: usize) -> String {
    format!("Black Hammer{}\t\t",ic_rating_list(system_security_value))
} 

// SR3-223
fn non_lethal_black_ic(system_security_value: usize) -> String {
    format!("Killjoy{}  \t\t",ic_rating_list(system_security_value))
} 


/* 
Remember that the total combined ratings of the
IC within an IC construct cannot exceed the construct’s Frame
Core Rating x 2 (see IC Constructs, p. 91). Mat113

- First, roll on the IC Rating Table (p. 116) to determine the construct’s Frame Core Rating.
- Then roll twice on the Alert Table (p. 115) to determine two types of IC that the construct will hold;
   follow the standard procedure for determining the ratings of these programs.
   Roll 2D6 and consult the Proactive IC Options Table (p. 116) to determine the options and defenses for the entire construct.

If the sum of the two IC programs’ combined ratings is less than the Frame Core Rating x 2, generate a third piece of IC.
If the sum of the three IC programs’ combined ratings is still less than the Frame Core Rating x 2, generate a fourth IC program.
Continue this process until the sum of the combined IC ratings equals or exceeds the Frame Core Rating x 2.

If the IC ratings exceed the frame core’s IC Payload, reduce the rating of a random IC program until the two figures
are equal.
 */

fn crippler_ripper(ic_type_name: &str) -> String {
    match d6() {
        1 | 2 => format!("{ic_type_name}; Acid(bod)\t"),
        3 => format!("{ic_type_name}; Binder(evasion)"),
        4 | 5 => format!("{ic_type_name}; marker(masking)"),
        6 => format!("{ic_type_name}; Jammer(sensor)"),
        _ => { println!("EEE Unexpected d6 result in crippler_ripper()"); "EEE".to_string() }
    }
}

fn ic_with_trap(ic_type_name: &str, system_security_value: usize) -> String {
    // TODO V add rating to the trap.
    // TODO V add databomb or pavlov data bomb for '2' see Mat116
    match d6()+d6() {
        2 => format!("{ic_type_name}{}; with data bomb{}\t", ic_rating_list(system_security_value), ic_rating_list(system_security_value)),
        3..=5 => format!("{ic_type_name}{}; with blaster{}\t", ic_rating_list(system_security_value), ic_rating_list(system_security_value)),
        6..=8 => format!("{ic_type_name}{}; with killer{}\t", ic_rating_list(system_security_value), ic_rating_list(system_security_value)),
        9..=11 => format!("{ic_type_name}{}; with sparky{}\t", ic_rating_list(system_security_value), ic_rating_list(system_security_value)),
        12 => format!("{ic_type_name}{}; with {}", ic_rating_list(system_security_value), black_ic(system_security_value)),
        _ => { println!("EEE Unexpected d6 result in ic_with_trap()"); "EEE".to_string() }
    }
}

fn ic_rating_list(system_security_value:usize ) -> String {
    match d6()+d6() {
        2..=5 => ic_rating_value_2_to_5(system_security_value),
        6..=8 => ic_rating_value_6_to_8(system_security_value),
        9..=11 => ic_rating_value_9_to_11(system_security_value),
        12 => ic_rating_value_12(system_security_value),
        _ => { println!("EEE Unexpected d6 result in ic_rating_list()"); "EEE".to_string() }
    }
}

fn ic_rating_value_2_to_5(system_security_value: usize) -> String {
    match system_security_value {
        4 => "-4".to_string(),
        5..=7 => "-5".to_string(),
        8..=10 => "-6".to_string(),
        11.. => "-8".to_string(),
        _ => { println!("EEE Unexpected d6 result in ic_rating_value_2_to_5()"); "EEE".to_string() }
    }
}

fn ic_rating_value_9_to_11(system_security_value: usize) -> String {
    match system_security_value {
        4 => "-6".to_string(),
        5..=7 => "-7".to_string(),
        8..=10 => "-10".to_string(),
        11.. => "-12".to_string(),
        _ => { println!("EEE Unexpected d6 result in ic_rating_value_6_to_8()"); "EEE".to_string() }
    }
}

fn ic_rating_value_12(system_security_value: usize) -> String {
    match system_security_value {
        4 => "-8".to_string(),
        5..=7 => "-10".to_string(),
        8..=10 => "-11".to_string(),
        11.. => "-12".to_string(),
        _ => { println!("EEE Unexpected d6 result in ic_rating_value_6_to_8()"); "EEE".to_string() }
    }
}
fn ic_rating_value_6_to_8(system_security_value: usize) -> String {
    match system_security_value {
        4 => "-5".to_string(),
        5..=7 => "-7".to_string(),
        8..=10 => "-8".to_string(),
        11.. => "-10".to_string(),
        _ => { println!("EEE Unexpected d6 result in ic_rating_value_6_to_8()"); "EEE".to_string() }
    }
}

fn get_die_roll_modifier_for_security_code(system_security_code: &String) -> usize {
    match system_security_code.as_str() {
        "blue" => 4,
        "green" => 3,
        "orange" => 2,
        "red" => 1,
        _ => 1000
    }
}

// inspired by https://rust-lang-nursery.github.io/rust-cookbook/algorithms/randomness.html
fn d6() -> usize {
    let mut rng = rand::thread_rng();
    // The end number '7' is not included in the range.
    rng.gen_range(1..7)
}

fn show_help() {
    println!("Render svg map from xml definition file - help text");
    println!("  --door-mode                   : only generate the code for a door");
    println!("  --door-sections <units>       : door units 1 or 2, default 1");
    println!("  --door-width <units>          : door width in units");
    println!("  --gm-map-file <file_name>     : set the output file name, for the gm map");
    println!("  --help                        : this text");
    println!("  --input-file  <file_name>     : set the input file name");
    println!("  --player-map-file <file_name> : set the output file name, for the gm map");
    println!("  --start-x <x-coord>           : ");
    println!("  --start-y <y-coord>           : ");
    println!("  --version                     : show application version number({})", APPLICATION_VERSION);
}
