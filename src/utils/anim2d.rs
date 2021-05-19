pub struct Anim2DDesc {
    durations: Vec<u32>,
    keys: Vec<u32>,
}

pub struct Anim2D {
    time: u32,
    started: u32,
    durations: Vec<u32>,
    keys: Vec<u32>,
    index: u32,
}

#[allow(dead_code)]
impl Anim2D {
    pub fn new(time: u32, started: u32, anim: &Anim2DDesc) -> Anim2D {
        assert!(anim.duration != 0);
        Anim2D {
            time: time,
            started: started,
            durations: anim.durations,
            keys: anim.keys,
            index: 0,
        }
    }

    pub fn update(&mut self, time: u32) {
        self.time = time;
        self.index = self.index()
    }

    pub fn index(&self) -> u32 {
        let keyframe = (self.time - self.started) / self.durations[self.index];
        self.keys[keyframe % self.keys.len()]
    }
}
