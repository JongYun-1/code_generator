pub(crate) enum Instruction {
    Sense { sensedir: CardDir, l1: i32, l2: i32, cond: String },
    Mark { m: usize, l: usize },
    Unmark { m: usize, l: usize },
    PickUp { l1: usize, l2: usize },
    Drop { l: usize },
    Turn { lr: RelDir, l: usize },
    Move { l1: usize, l2: usize },
    Flip { p: usize, l1: usize, l2: usize },
}

pub(crate) enum CardDir {
    NorthEast,
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
}

pub enum SenseDir {
    Here,
    Ahead,
    LeftAhead,
    RightAhead
}

pub enum RelDir {
    Left,
    Right,
}
enum Direction {
    Left,
    Right,
}

enum Condition {
    Food,
    HomeBase,
}

pub(crate) struct Label {
    pub(crate) number: usize,
    pub(crate) instructions: Vec<Instruction>,
}

pub fn parse_roboto_program(input: &str) -> Vec<Label> {
    let mut labels: Vec<Label> = Vec::new();
    let mut current_label: Option<Label> = None;

    for line in input.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("label") {
            if let Some(label) = current_label.take() {
                labels.push(label);
            }
            // Extract the label number with more careful handling
            let label_number_str =  trimmed.trim_start_matches("label ").trim_end_matches(":");
            match label_number_str.parse::<usize>() {
                Ok(label_number) => {
                    current_label = Some(Label { number: label_number, instructions: Vec::new() });
                },
                Err(e) => {
                    eprintln!("Error parsing label number '{}': {}", label_number_str, e);
                    continue; // Skip this label
                }
            }
        } else if let Some(ref mut label) = current_label {
            match parse_instruction(trimmed) {
                Ok(instruction) => label.instructions.push(instruction),
                Err(e) => {
                    eprintln!("Error parsing instruction '{}': {}", trimmed, e);
                    continue; // Skip this instruction
                }
            }
        }
    }

    if let Some(label) = current_label {
        labels.push(label);
    }

    labels
}

pub fn parse_instruction(input: &str) -> Result<Instruction, String> {
    let parts: Vec<&str> = input.split_whitespace().collect();
    match parts.get(0).map(|s| *s) {
        Some("Flip") => {
            Ok(Instruction::Flip {
                p: parts.get(1).and_then(|&s| s.parse().ok()).ok_or("Invalid 'p' value for Flip")?,
                l1: parts.get(2).and_then(|&s| s.parse().ok()).ok_or("Invalid 'l1' value for Flip")?,
                l2: parts.get(3).and_then(|&s| s.parse().ok()).ok_or("Invalid 'l2' value for Flip")?,
            })
        },
        Some("Turn") => {
            Ok(Instruction::Turn {
                lr: parts.get(1).ok_or("Missing direction for Turn")?.to_string(),
                l: parts.get(2).and_then(|&s| s.parse().ok()).ok_or("Invalid label for Turn")?,
            })
        },
        Some("Sense") => {
            Ok(Instruction::Sense {
                sensedir: parts.get(1).ok_or("Missing sense direction for Sense")?.to_string(),
                l1: parts.get(2).and_then(|&s| s.parse().ok()).ok_or("Invalid 'l1' value for Sense")?,
                l2: parts.get(3).and_then(|&s| s.parse().ok()).ok_or("Invalid 'l2' value for Sense")?,
                cond: parts.get(4).ok_or("Missing condition for Sense")?.to_string(),
            })
        },
        Some("PickUp") => {
            Ok(Instruction::PickUp {
                l1: parts.get(1).and_then(|&s| s.parse().ok()).ok_or("Invalid 'l1' value for PickUp")?,
                l2: parts.get(2).and_then(|&s| s.parse().ok()).ok_or("Invalid 'l2' value for PickUp")?,
            })
        },
        Some("Move") => {
            Ok(Instruction::Move {
                l1: parts.get(1).and_then(|&s| s.parse().ok()).ok_or("Invalid 'l1' value for Move")?,
                l2: parts.get(2).and_then(|&s| s.parse().ok()).ok_or("Invalid 'l2' value for Move")?,
            })
        },
        Some("Drop") => {
            Ok(Instruction::Drop {
                l: parts.get(1).and_then(|&s| s.parse().ok()).ok_or("Invalid label for Drop")?,
            })
        },
        _ => Err("Unknown instruction".to_string()),
    }
}
