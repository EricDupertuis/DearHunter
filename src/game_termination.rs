pub struct GameTermination {
    pub eaten: bool,
    pub reached_home: bool,
    pub timeout: bool,
}

impl Default for GameTermination {
    fn default() -> Self {
        GameTermination {
            eaten: false,
            reached_home: false,
            timeout: false,
        }
    }
}
