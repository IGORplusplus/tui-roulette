//items.rs
use rand::thread_rng;

//local and online multiplayer, and singleplayer
//multiplayer items easier to pull off, ten second timer is when a shot could be first fired
#[derive(Clone, Copy, Debug)]
pub enum Items {
    Saw, //doubles damage
    Beer, //ejects the next shell
    Cigarette, //restores one health
    Mirror, //deflects bullet only shown after a player makes a decision to 
    Inverter,
    MagnifyingGlass, //shows current shell
    Handcuffs, //skips next player's turn
    Meth, //restores two health, but keeps you from being able to see the next shell
    AED, //secret and default, if shot next round it keeps the health, if not lose a health
    Adrenaline,
    LSD, //shows a shell in the future, specifies which one
}
