#[derive(Debug)]
//* 路灯信息：时间，路灯亮的情况

pub enum COLOR {
    GREEN,
    YELLOW,
    RED,
}
#[derive(Debug)]
pub struct Lamp {
    pub time: i32,
    pub state: COLOR,
}

impl Lamp {
    //路灯初始化
    pub fn new(time: i32, state: COLOR) -> Lamp {
        Lamp { time, state }
    }

    pub fn set_time(&mut self, time: i32) {
        self.time = time;
    }

    pub fn set_signal(&mut self, state: i32) {
        match state {
            1 => {
                if self.time > 5 {
                    self.state = COLOR::GREEN;
                } else {
                    self.state = COLOR::YELLOW;
                }
            }
            0 => {
                self.state = COLOR::RED;
            }
            _ => {}
        }
    }
    pub fn set_message(&mut self, time: i32, state: i32) {
        self.set_time(time);
        self.set_signal(state);
    }
}
