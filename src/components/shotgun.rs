//shotgun.rs
use rand::{
    Rng,
    distributions::{Distribution, WeightedIndex},
    seq::SliceRandom,
    thread_rng,
};
use std::{cell::RefCell, fmt::format};

use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{Duration, sleep};

use crate::{
    components::{enums::ShotgunCycleView, shotgun},
    ui_components::widget_data::{self, WidgetData},
};

#[derive(Debug, Default, Clone)]
pub struct Shotgun {
    pub shells: Arc<Mutex<Vec<Shell>>>,
    pub state: Arc<Mutex<ShotgunState>>,
    pub model: ShotgunModel,
    pub cycle: Arc<Mutex<ShotgunCycle>>,
}

#[derive(Debug, Default, Clone)]
enum ShotgunModel {
    #[default]
    Stock,
    Revolver, //does twice the amount of damage
}

#[derive(Debug, Default, Clone)]
enum ShotgunState {
    #[default]
    Stock,
    SawedOff,    //does twice the amount of damage
    Rusty,       //permanent until next round misfire chance increased
    ThickBarrel, //impossible to saw off
    Reinforced,  //Destruct shell becomes offensive but also destroys the shotgun
}

#[derive(Debug, Default, Clone)]
pub enum ShotgunCycle {
    #[default]
    Ready,
    Reloading,
    //default
    Shooting,
    Blanking,
    Tasing,
    Imposter,
    SelfDestruct,
    Poison,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub enum Shell {
    Live,
    #[default]
    Blank,
    Poison,
    BeanBag, //makes player stunned for the next turn, so can only use one item
    Taser,
    Imposter,     //looks like a blank but isn't
    SelfDestruct, //blows up in the person's face if not reinforced
}

//BeanBag round limits the player to only use one item
//Russian Roulette item, play russian roulette for a turn instead of the shotgun

impl Shotgun {
    pub fn new() -> Shotgun {
        Shotgun {
            shells: Arc::new(Mutex::new(Vec::new())),
            state: Arc::new(Mutex::new(ShotgunState::default())),
            model: ShotgunModel::default(),
            cycle: Arc::new(Mutex::new(ShotgunCycle::default())),
        }
    }

    pub async fn is_empty(&self) -> bool {
        self.shells.lock().await.is_empty()
    }

    pub async fn reload_with(
        &self,
        all_shells: Vec<Shell>,
        weights: Vec<usize>,
        num_shells: usize,
    ) -> Option<String> {
        {
            let cycle = self.cycle.lock().await;
            if !matches!(*cycle, ShotgunCycle::Ready) {
                return Some("Shotgun is busy".to_string());
            }
        }

        // 2. Enter Reloading
        {
            let mut cycle = self.cycle.lock().await;
            *cycle = ShotgunCycle::Reloading;
        }

        // 3. Simulate reload time
        sleep(Duration::from_millis(800)).await;

        // 4. Actually load shells
        self.load(all_shells, weights, num_shells).await;

        // 5. Back to Ready
        {
            let mut cycle = self.cycle.lock().await;
            *cycle = ShotgunCycle::Ready;
        }

        Some(format!("Reloaded shotgun with {} shells", num_shells))
    }

    async fn load(&self, all_shells: Vec<Shell>, weights: Vec<usize>, num_shells: usize) {
        let mut rng = thread_rng();
        let mut shells = self.shells.lock().await;
        shells.clear();

        let dist = WeightedIndex::new(&weights).expect("weights can not be zero or negative");

        for _ in 0..num_shells {
            let idx = dist.sample(&mut rng);
            let random_shell = all_shells[idx].clone();
            shells.push(random_shell);
        }

        if !shells.is_empty() {
            if !shells.contains(&Shell::Blank) {
                let num: usize = rng.gen_range(0..shells.len());
                shells[num] = Shell::Blank;
            }
        }
    }

    pub async fn reload_random_shells(&self, num_shells: usize) -> Option<String> {
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
            12, //Blank
            1,  //Poison
            2,  //BeanBag
            1,  //Taser
            1,  //Imposter
        ];
        self.reload_with(all_shells, weights, num_shells).await
    }

    pub fn load_default_shells(&self, num_shells: usize) {
        let all_shells = vec![Shell::Live, Shell::Blank];

        let weights = vec![
            10, //Live
            14, //Blank
        ];
        self.load(all_shells, weights, num_shells);
    }

    pub async fn shoot(&self, widget_data: Arc<Mutex<WidgetData>>) -> Option<String> {
        {
            let cycle = self.cycle.lock().await;
            if !matches!(*cycle, ShotgunCycle::Ready) {
                return Some(String::from("Some shotgun is busy"));
            }
        }

        let shell_art_helper;

        let msg = {
            let mut shells = self.shells.lock().await;
            if let Some(shell) = shells.pop() {
                match shell {
                    Shell::Live => shell_art_helper = ShotgunCycleView::Shooting,
                    Shell::Blank => shell_art_helper = ShotgunCycleView::Blanking,
                    Shell::Taser => shell_art_helper = ShotgunCycleView::Tasing,
                    _ => shell_art_helper = ShotgunCycleView::Ready,
                };
                if shells.is_empty() {
                    Some(format!("Last shell in shotgun: {:?}", shell))
                } else {
                    Some(format!(
                        "Popped Shell {:?}, {} shells left",
                        shell,
                        shells.len()
                    ))
                }
            } else {
                return Some(String::from("No shell in shotgun"));
            }
        };

        {
            let mut cycle = self.cycle.lock().await;
            *cycle = ShotgunCycle::Shooting;
        }

        {
            let mut snapshot = widget_data.lock().await;
            snapshot.shotgun_cycle_view = shell_art_helper;
        }

        let cycle_clone = Arc::clone(&self.cycle);
        let widget_data_clone = Arc::clone(&widget_data);
        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_millis(500)).await;

            let mut cycle = cycle_clone.lock().await;
            *cycle = ShotgunCycle::Ready;

            let mut snapshot = widget_data_clone.lock().await;
            snapshot.shotgun_cycle_view = ShotgunCycleView::Ready;
        });

        msg
    }
}
