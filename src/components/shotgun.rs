//shotgun.rs
use rand::{ seq::SliceRandom, thread_rng };
use std::cell::RefCell;

#[derive(Debug, Default, Clone)]
pub struct Shotgun {
    pub shells: RefCell<Vec<Shell>>,
    pub state: ShotgunState,
}

#[derive(Debug, Default, Clone)]
pub enum ShotgunState {
    #[default]
    Default,
    SawedOff,
    HotPotato,
}

#[derive(Debug, Default, Clone)]
pub enum Shell {
    Live,
    #[default]
    Blank,
    Incendiary,
    BeanBag,
    Electric,
}

impl Shotgun {

    pub fn load(&self,all_shells: Vec<Shell>, num_shells: usize) {
        let mut rng = thread_rng();
        let mut shells = self.shells.borrow_mut();
        shells.clear();

        for _ in 0..num_shells {
            if let Some(random_shell) = all_shells.choose(&mut rng) {
                shells.push(random_shell.clone());
            }
        }
    }

    pub fn load_random_shells(&self, num_shells: usize) {
        let all_shells = vec![
            Shell::Live,
            Shell::Blank,
            Shell::Incendiary,
            Shell::BeanBag,
            Shell::Electric
        ];

        self.load(all_shells, num_shells);
    }

    pub fn load_default_shells(& self, num_shells: usize) {
        let all_shells = vec![
            Shell::Live,
            Shell::Blank,
        ];

        self.load(all_shells, num_shells);
    }

    pub fn new() -> Shotgun {
        Shotgun {
            shells: RefCell::new(Vec::new()),
            state: ShotgunState::Default,
        }
    }
}
