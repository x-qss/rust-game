//pub mod Player {


pub struct Player {
    pub uuid: i32,
    pub name: String,
    pub x: f64,
    pub y: f64,
    pub vx: f64,
    pub vy: f64,
    pub lx: f64,
    pub ly: f64,
    pub scale: i32
}

impl Player {
    pub fn new(uuid: i32, scale: i32) -> Self {
        Self {
            uuid,
            name: String::from("unknown"),
            x: 0.0,
            y: 0.0,
            vx: 0.0,
            vy: 0.0,
            lx: 0.0,
            ly: 0.0,
            scale
        }
    }

    pub fn update_pos(&mut self, x: f64, y: f64) -> &mut Self {
        self.x = x;
        self.y = y;
        self
    }

    pub fn update_last_pos(&mut self, x: f64, y: f64) -> &mut Self {
        self.lx = x;
        self.ly = y;
        self
    }

    pub fn set_data(&mut self, name: String) -> &mut Self {
        self.name = name;
        self
    }
}
//}
