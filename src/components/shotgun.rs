//shotgun.rs
use rand::{ Rng, seq::SliceRandom, thread_rng, distributions::{WeightedIndex, Distribution} };
use std::cell::RefCell;

#[derive(Debug, Default, Clone)]
pub struct Shotgun {
    pub shells: RefCell<Vec<Shell>>,
    pub state: ShotgunState,
    pub model: ShotgunModel,
}

#[derive(Debug, Default, Clone)]
pub enum ShotgunModel {
    #[default]
    Default,
    Revolver, //does twice the amount of damage
}

#[derive(Debug, Default, Clone)]
pub enum ShotgunState {
    #[default]
    Default,
    SawedOff, //does twice the amount of damage
    Rusty, //permanent until next round misfire chance increased
    ThickBarrel, //impossible to saw off
    Reinforced, //Destruct shell becomes offensive but also destroys the shotgun
}

#[derive(Debug, Default, Clone, PartialEq)]
pub enum Shell {
    Live,
    #[default]
    Blank,
    Poison, //
    BeanBag, //makes player stunned for the next turn, so can only use one item
    Taser, //
    Imposter, //looks like a blank but isn't
    SelfDestruct, //blows up in the person's face if not reinforced
}

//BeanBag round limits the player to only use one item
//Russian Roulette item, play russian roulette for a turn instead of the shotgun



impl Shotgun {

    pub fn new() -> Shotgun {
        Shotgun {
            shells: RefCell::new(Vec::new()),
            state: ShotgunState::Default,
            model: ShotgunModel::Default,
        }
    }

    pub fn load(&self,all_shells: Vec<Shell>, weights: Vec<usize>, num_shells: usize) {
        let mut rng = thread_rng();
        let mut shells = self.shells.borrow_mut();
        shells.clear();

        let dist = WeightedIndex::new(&weights)
            .expect("weights can not be zero or negative");

        for _ in 0..num_shells {
            let idx = dist.sample(&mut rng);
            let random_shell = all_shells[idx].clone();
            shells.push(random_shell);
        }

        if !shells.is_empty(){
            if !shells.contains(&Shell::Blank) {
                let num: usize = rng.gen_range(0..shells.len());
                shells[num] = Shell::Blank;
            }
        }
    }

    pub fn load_random_shells(&self, num_shells: usize) {
        let all_shells = vec![
            Shell::Live,
            Shell::Blank,
            Shell::Poison,
            Shell::BeanBag,
            Shell::Taser,
            Shell::Imposter,
        ];

        let weights = vec![
            10, //Live
            14, //Blank
            1, //Poison
            2, //BeanBag
            1, //Taser
            1, //Imposter
        ];


        self.load(all_shells, weights, num_shells);
    }

    pub fn load_default_shells(&self, num_shells: usize) {
        let all_shells = vec![
            Shell::Live,
            Shell::Blank,
        ];

        let weights = vec![
            10, //Live
            14, //Blank
        ];
        self.load(all_shells, weights, num_shells);
    }

    pub fn shoot(&self) -> Option<String>{
        let mut shell_borrow = self.shells.borrow_mut();
        if let Some(popped_shell) = shell_borrow.pop() {
            Some(format!("Popped shell: {:?}", popped_shell))
        } else {
            Some("No shell in shotgun.".to_string())
        }
    }

    pub fn shell_count(&self) -> usize {
        self.shells.borrow().len()
    }
}
