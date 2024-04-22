use crate::config::Config;

pub struct RobotoProgram {
    config: Config,
}

impl RobotoProgram {
    pub fn new(config: Config) -> Self {
        RobotoProgram { config }
    }

    pub fn generate(&self) -> String {
        let mut program = String::new();
        program += &format!("label 0:\n");
        program += &self.generate_search_logic();
        program += &format!("label 100:\n");
        program += &self.generate_food_collection_logic();
        program += &format!("label 200:\n");
        program += &self.generate_base_navigation_logic();
        program
    }

    fn generate_search_logic(&self) -> String {
        match self.config.search_pattern.as_ref() {
            "random" => "    Flip 2 1 10\n    Turn Left 0\n    Turn Right 0\n".to_string(),
            "pattern" => "    Move 1 10\n    Turn Left 0\n    Turn Right 0\n".to_string(),
            _ => "    Move 0 10 \n".to_string(),
        }
    }

    fn generate_food_collection_logic(&self) -> String {
        "    Sense Ahead 101 0 Food\n    PickUp 200 0\n".to_string()
    }

    fn generate_base_navigation_logic(&self) -> String {
        format!("    Sense Ahead 201 0 HomeBase\n    Move 200 0\n    Drop 0\n")
    }
}
