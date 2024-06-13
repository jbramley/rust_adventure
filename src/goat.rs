use rand::prelude::IndexedRandom;
use rand::Rng;

#[derive(Debug)]
pub struct Goat {
    pub room: usize,
}

impl Goat {
    pub fn maybe_move(&mut self, doors: &[Option<usize>]) -> Option<usize> {
        let mut rng = rand::thread_rng();
        if rng.gen_bool(0.05) { return None; }
        let rooms: Vec<usize> = doors.iter().copied().filter(|d| d.is_some()).map(|d| d.unwrap()).collect();
        rooms.choose(&mut rng).map(|room| self.room = *room);
        println!("Goat moved to room {}", self.room);
        Some(self.room)
    }
}