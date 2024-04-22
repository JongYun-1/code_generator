#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

impl ToString for Instruction {
    fn to_string(&self) -> String {
        match *self {
            Self::Sense { sensedir, l1, l2, cond } => format!("Sense {} {} {} {}", sensedir.to_string(), l1, l2, cond.to_string()),
            Self::Mark { m, l } => format!("Mark {} {}", m, l),
            Self::Unmark { m, l } => format!("Unmark {} {}", m, l),
            Self::PickUp { l1, l2 } => format!("PickUp {} {}", l1, l2),
            Self::Drop { l } => format!("Drop {}", l),
            Self::Turn { lr, l } => format!("Turn {} {}", lr.to_string(), l),
            Self::Move { l1, l2 } => format!("Move {} {}", l1, l2),
            Self::Flip { p, l1, l2 } => format!("Flip {} {} {}", p, l1, l2),
            //_ => format!("")
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum CardDir {
    NorthEast,
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
}

impl ToString for CardDir {
    fn to_string(&self) -> String {
        match *self {
            Self::NorthEast => "NorthEast".to_string(),
            Self::East => "East".to_string(),
            Self::SouthEast => "SouthEast".to_string(),
            Self::SouthWest => "SouthWest".to_string(),
            Self::West => "West".to_string(),
            Self::NorthWest => "NorthWest".to_string()
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SenseDir {
    Here,
    Ahead,
    LeftAhead,
    RightAhead
}

impl ToString for SenseDir {
    fn to_string(&self) -> String {
        match *self {
            Self::Here => "Here".to_string(),
            Self::Ahead => "Ahead".to_string(),
            Self::LeftAhead => "Left".to_string(),
            Self::RightAhead => "Right".to_string()

        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RelDir {
    Left,
    Right,
}

impl ToString for RelDir {
    fn to_string(&self) -> String {
        match *self {
            Self::Left => "Left".to_string(),
            Self::Right => "Right".to_string()
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
}

impl ToString for Direction {
    fn to_string(&self) -> String {
        match *self {
            Self::Left => "Left".to_string(),
            Self::Right => "Right".to_string()
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Condition {
    Friend,
    Foe,
    FriendWithFood,
    FoeWithFood,
    Food,
    Building,
    Transponder(u32),
    FoeTransponder,
    HomeBase,
    FoeBase
}

impl ToString for Condition {
    fn to_string(&self) -> String {
        match *self {
            Self::Friend => "Friend".to_string(),
            Self::Foe => "Foe".to_string(),
            Self::FriendWithFood => "FriendWithFood".to_string(),
            Self::FoeWithFood => "FoeWithFood".to_string(),
            Self::Food => "Food".to_string(),
            Self::Building => "Building".to_string(),
            Self::Transponder(num) => format!("Transponder {}", num),
            Self::FoeTransponder => "FoeTransponder".to_string(),
            Self::HomeBase => "HomeBase".to_string(),
            Self::FoeBase => "FoeBase".to_string()

        }
    }
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
