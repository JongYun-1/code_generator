use std::{collections::HashMap, hash::Hash};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum Instruction {
    Sense { sensedir: SenseDir, l1: usize, l2: usize, cond: String },
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
        match self {
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

pub fn parse_roboto_program(input: Vec<String>) -> Result<Vec<String>, String> {
    let mut label_map : HashMap<String, usize> = HashMap::new();
    let mut code : Vec<String> = Vec::new();
    let mut label_ctr = 0;

    for line in &input {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let first = parts.get(0).map(|s| *s);
        match first {
            Some("label") => {
                let text_label = parts.get(1).map(|s| *s);
                match text_label {
                    Some(str) => {
                        if label_map.contains_key(str) {
                            return Err("Duplicate labels".to_string());
                        } else {
                            label_map.insert(str.to_string(), label_ctr);
                        }
                    },
                    None => ()
                }
            },
            Some(line) => {
                label_ctr+=1;
            },
            None => ()
        }
    }

    let mut line_ctr = 0;
    for line in input {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let first = parts.get(0).map(|s| *s);
        match first {
            Some("label") => {
                //Do nothing
                ()
            },
            Some(l) => {
                let is_instr = parse_instruction(&line, &label_map, line_ctr);
                match is_instr {
                    Ok(instr) => code.push(instr.to_string()),
                    Err(e) => return Err(e),
                }
                line_ctr+=1;
            },
            None => ()
        }
    }

    

    Ok(code)

}

pub fn parse_cond(cond: &str) -> Option<Condition> {
    match cond {
        "Friend" => Some(Condition::Friend),
        "Foe" => Some(Condition::Foe),
        "FriendWithFood" => Some(Condition::FriendWithFood),
        "FoeWithFood" => Some(Condition::FoeWithFood),
        "Food" => Some(Condition::Food),
        "Building" => Some(Condition::Building),
        "FoeTransponder" => Some(Condition::FoeTransponder),
        "HomeBase" => Some(Condition::HomeBase),
        "FoeBase" => Some(Condition::FoeBase),
        _ => None
    }
}

pub fn parse_card_dir(dir: &str) -> Option<SenseDir> {
    match dir {
        "Here" => Some(SenseDir::Here),
        "Ahead" => Some(SenseDir::Ahead),
        "LeftAhead" => Some(SenseDir::LeftAhead),
        "RightAhead" => Some(SenseDir::RightAhead),
        _ => None
    }
}

pub fn parse_rel_dir(dir: &str) -> Option<RelDir> {
    match dir {
        "Right" => Some(RelDir::Right),
        "Left" => Some(RelDir::Left),
        _ => None
    }
}

pub fn resolve_label(label: &str, label_map: &HashMap<String, usize>, cur_line: usize) -> Option<usize> {
    match label.parse::<i64>() {
        Ok(num) => {
            let abs_lab = (cur_line as i64 + num) as usize;
            Some(abs_lab)
        },
        Err(_) => {
            let res = label_map.get(label);
            match res {
                Some(num) => Some(*num),
                None => None
            }
        }
    }
}


pub fn parse_instruction(input: &str, label_map: &HashMap<String, usize>, cur_line: usize) -> Result<Instruction, String> {
    let parts: Vec<&str> = input.split_whitespace().collect();
    match parts.get(0).map(|s| *s) {
        Some("Flip") => {
            Ok(Instruction::Flip {
                p: parts.get(1).and_then(|&s| s.parse().ok()).ok_or("Invalid 'p' value for Flip")?,
                l1: parts.get(2).and_then(|&s| resolve_label(s, label_map, cur_line)).ok_or("Invalid 'l1' value for Flip")?,
                l2: parts.get(3).and_then(|&s| resolve_label(s, label_map, cur_line)).ok_or("Invalid 'l2' value for Flip")?,
            })
        },
        Some("Turn") => {
            let dir_str = parts.get(1).ok_or("Missing direction for Turn")?.to_string();
            let dir = parse_rel_dir(&dir_str);
            match dir {
                Some(realDir) => {
                    Ok(Instruction::Turn {
                        lr: realDir,
                        l: parts.get(2).and_then(|&s| resolve_label(s, label_map, cur_line)).ok_or("Invalid label for Turn")?,
                    })
                },
                None => Err("Invalid reldir".to_string())
            }
        },
        Some("Sense") => {
            let sDirStr = parts.get(1).ok_or("Missing sense direction for Sense")?.to_string();
            let sDir = parse_card_dir(&sDirStr);
            match sDir {
                None => Err("Invalid CardDir".to_string()),
                Some(realDir) => {
                    Ok(Instruction::Sense {
                        sensedir: realDir,
                        l1: parts.get(2).and_then(|&s| resolve_label(s, label_map, cur_line)).ok_or("Invalid 'l1' value for Sense")?,
                        l2: parts.get(3).and_then(|&s| resolve_label(s, label_map, cur_line)).ok_or("Invalid 'l2' value for Sense")?,
                        cond: parts.get(4).ok_or("Missing condition for Sense")?.to_string(),
                    })
                }
            }
        },
        Some("PickUp") => {
            Ok(Instruction::PickUp {
                l1: parts.get(1).and_then(|&s| resolve_label(s, label_map, cur_line)).ok_or("Invalid 'l1' value for PickUp")?,
                l2: parts.get(2).and_then(|&s| resolve_label(s, label_map, cur_line)).ok_or("Invalid 'l2' value for PickUp")?,
            })
        },
        Some("Move") => {
            Ok(Instruction::Move {
                l1: parts.get(1).and_then(|&s| resolve_label(s, label_map, cur_line)).ok_or("Invalid 'l1' value for Move")?,
                l2: parts.get(2).and_then(|&s| resolve_label(s, label_map, cur_line)).ok_or("Invalid 'l2' value for Move")?,
            })
        },
        Some("Drop") => {
            Ok(Instruction::Drop {
                l: parts.get(1).and_then(|&s| resolve_label(s, label_map, cur_line)).ok_or("Invalid label for Drop")?,
            })
        },
        _ => Err("Unknown instruction".to_string()),
    }
}
