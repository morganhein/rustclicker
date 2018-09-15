pub struct Player {
    pub clicks: u64,
    modifier: u64,
    pub points: u64,
}

impl Player {
    pub fn new() ->Player {
        Player{
            clicks: 0,
            modifier: 60,
            points: 0,
        }
    }
    pub fn get_click_points(&mut self) ->u64 {
        return self.modifier
    }
    pub fn add_points(&mut self, points: u64) {
        self.points += points;
    }
}