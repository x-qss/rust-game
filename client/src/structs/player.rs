//pub mod Player {


pub struct Player {
    pub uuid: i32,
    pub name: String,
    pub x: i32,
    pub y: i32,
    pub vx: i32,
    pub vy: i32,
    pub lx: i32,
    pub ly: i32,
    pub scale: i32
}

impl Player {
    pub fn new(uuid: i32, scale: i32) -> Self {
        Self {
            uuid,
            name: String::from("unknown"),
            x: 0,
            y: 0,
            vx: 0,
            vy: 0,
            lx: 0,
            ly: 0,
            scale
        }
    }

    pub fn update_pos(&mut self, x: i32, y: i32) -> &mut Self {
        self.x = x;
        self.y = y;
        self
    }

    pub fn update_last_pos(&mut self, x: i32, y: i32) -> &mut Self {
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
