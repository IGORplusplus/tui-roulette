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
    Imposter,
}

impl Shotgun {

    pub fn new() -> Shotgun {
        Shotgun {
            shells: RefCell::new(Vec::new()),
            state: ShotgunState::Default,
        }
    }

    pub fn load(&self,all_shells: Vec<Shell>, weights: Vec<usize>, num_shells: usize) {
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
            Shell::Electric,
            Shell::Imposter,
        ];
        
        let weights = vec![
            10, //Live
            14, //Blank
            2, //Incendiary
            2, //BeanBag
            2, //Electric
            1, //Imposter
        ];


        self.load(all_shells, weights, num_shells);
    }

    pub fn load_default_shells(&self, num_shells: usize) {
        let all_shells = vec![
            Shell::Live,
            Shell::Blank,
        ];

        self.load(all_shells, num_shells);
    }

    pub fn shoot(&self) -> Option<String>{
        let mut shell_borrow = self.shells.borrow_mut();
        if let Some(popped_shell) = shell_borrow.pop() {
            Some(format!("Popped shell: {:?}", popped_shell))
        } else {
            Some("No shell in shotgun.".to_string())
        }
    }
}
