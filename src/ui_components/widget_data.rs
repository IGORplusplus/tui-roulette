use crossterm::terminal::disable_raw_mode;
//widget-data.rs
use ratatui::layout::Rect;
use crate::ui::{SHOTGUN_ART, BANG, CLICK};

use ratatui::style::{Color, Style, Stylize};

#[derive(Debug, Clone)]
pub struct WidgetState {
    pub display: bool,
    pub focus: bool,
    area: Option<Rect>,
    content: Option<String>,
    color: Option<Color>,
}

impl WidgetState {
    pub fn new_blank() -> WidgetState {
        WidgetState {
            display: false,
            focus: false,
            area: None,
            content: None,
            color: Some(Color::White),
        }
    }

    pub fn new_shotgun_blank() -> WidgetState {
        WidgetState {
            display: false,
            focus: false,
            area: None,
            content: None,
            color: Some(Color::Red),
        }
    }

    pub fn new_content(content: &str) -> WidgetState{
        let content: String = content.to_string();
        WidgetState {
            display: true,
            focus: true,
            area: None,
            content: Some(content),
            color: Some(Color::White),
        }
    }

    pub fn new_color(color: Option<Color>) -> WidgetState{
        WidgetState {
            display: true,
            focus: true,
            area: None,
            content: None,
            color,
        }
    }

    pub fn toggle_focus(&mut self) {
        self.focus = !self.focus;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WidgetKind {
    Log,
    Data,
    Inventory,
    Player,
    Shotgun,
    Confirmation,
}

#[derive(Debug)]
pub struct WidgetData{
    //these are a little redundant
    log: WidgetState,
    data: WidgetState,
    inventory: WidgetState,
    player: WidgetState,
    shotgun: WidgetState,

    confirmation: WidgetState,

    current_focus: Option<WidgetKind>,

    //render last in list first
    pub render_stack: Vec<WidgetKind>,
}

impl WidgetData {
    pub fn new() -> WidgetData {
        WidgetData {
            log: WidgetState::new_blank(),
            data: WidgetState::new_blank(),
            inventory: WidgetState::new_blank(),
            player: WidgetState::new_blank(),
            shotgun: WidgetState::new_content(SHOTGUN_ART),

            confirmation: WidgetState::new_blank(),
            current_focus: None,

            render_stack: Vec::new(),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (WidgetKind, &WidgetState)> {
        [
            (WidgetKind::Log, &self.log),
            (WidgetKind::Data, &self.data),
            (WidgetKind::Inventory, &self.inventory),
            (WidgetKind::Player, &self.player),
            (WidgetKind::Shotgun, &self.shotgun),
            (WidgetKind::Confirmation, &self.confirmation),
        ]
            .into_iter()
    }

    pub fn shown_widgets(&self) -> Option<WidgetKind> {
        self.iter()
            .find(|(_, state)| state.focus)
            .map(|(kind, _)| kind)
    }

    fn order() -> [WidgetKind; 5] {
        [
            WidgetKind::Log,
            WidgetKind::Data,
            WidgetKind::Inventory,
            WidgetKind::Player,
            WidgetKind::Confirmation,
        ]
    }

    fn get(&self, kind: WidgetKind) -> &WidgetState {
        match kind {
            WidgetKind::Log => &self.log,
            WidgetKind::Data => &self.data,
            WidgetKind::Inventory => &self.inventory,
            WidgetKind::Player => &self.player,
            WidgetKind::Shotgun => &self.shotgun,
            WidgetKind::Confirmation => &self.confirmation,
        }
    }

    fn get_mut_widget(&mut self, kind: WidgetKind) -> &mut WidgetState {
        match kind {
            WidgetKind::Log => &mut self.log,
            WidgetKind::Data => &mut self.data,
            WidgetKind::Inventory => &mut self.inventory,
            WidgetKind::Player => &mut self.player,
            WidgetKind::Shotgun => &mut self.shotgun,
            WidgetKind::Confirmation => &mut self.confirmation,
        }
    }

    ///TODO: I want to understand this code
    pub fn focus_next(&mut self) {
        let order = Self::order();

        let confirmation_displayed = self.get(WidgetKind::Confirmation).display;
        if confirmation_displayed == true {
            return;
        }

        // Find current focus index
        let current_idx = order.iter().position(|&kind| self.get(kind).focus);

        // Clear all focus
        for kind in order.iter() {
            self.get_mut_widget(*kind).focus = false;
        }

        // Start searching from the next index
        let mut next_idx = match current_idx {
            Some(i) => (i + 1) % order.len(),
            _ => 0,
        };

        // Loop until we find a displayed (and allowed) widget
        for _ in 0..order.len() {
            let kind = order[next_idx];

            if self.get(kind).display {
                self.get_mut_widget(kind).focus = true;
                self.current_focus = Some(kind);
                return;
            }

            next_idx = (next_idx + 1) % order.len();
        }
    }

    pub fn focus_prev(&mut self) {
        let order = Self::order();

        let confirmation_displayed = self.get(WidgetKind::Confirmation).display;
        if confirmation_displayed == true {
            return;
        }

        // Find current focus index
        let current_idx = order.iter().position(|&kind| self.get(kind).focus);

        // Clear all focus
        for kind in order {
            self.get_mut_widget(kind).focus = false;
        }

        // Start searching from the previous index
        let mut prev_idx = match current_idx {
            Some(0) => order.len() - 1,
            Some(i) => i - 1,
            None => order.len() - 1,
        };

        // Loop until we find a displayed widget
        for _ in 0..order.len() {
            if self.get(order[prev_idx]).display {
                self.get_mut_widget(order[prev_idx]).focus = true;
                return;
            }
            prev_idx = if prev_idx == 0 { order.len() - 1 } else { prev_idx - 1 };
        }
    }

    pub fn get_state(&self, kind: WidgetKind) -> &WidgetState {
        match kind {
            WidgetKind::Log => &self.log,
            WidgetKind::Data => &self.data,
            WidgetKind::Inventory => &self.inventory,
            WidgetKind::Player => &self.player,
            WidgetKind::Shotgun => &self.shotgun,
            WidgetKind::Confirmation => &self.confirmation,
        }
    }

    pub fn is_focused(&self, kind: WidgetKind) -> bool{
        self.get(kind).focus
    }

    pub fn get_focus(&self) -> Option<WidgetKind> {
        self.current_focus
    }

    pub fn toggle_focus(&mut self, kind: WidgetKind) {
        match kind {
            WidgetKind::Log => self.log.focus = !self.log.focus,
            WidgetKind::Data => self.data.focus = !self.data.focus,
            WidgetKind::Inventory => self.inventory.focus = !self.inventory.focus,
            WidgetKind::Player => self.player.focus = !self.player.focus,
            WidgetKind::Shotgun => self.shotgun.focus = !self.shotgun.focus,
            WidgetKind::Confirmation => self.confirmation.focus = !self.confirmation.focus,
        }

        if self.current_focus == Some(kind) {
            self.current_focus = None;
        }
        else {
            self.current_focus = Some(kind);
        }
    }

    pub fn is_displayed(&self, kind: WidgetKind) -> bool{
        let widget_state = match kind {
            WidgetKind::Log => &self.log,
            WidgetKind::Data => &self.data,
            WidgetKind::Inventory => &self.inventory,
            WidgetKind::Player => &self.player,
            WidgetKind::Shotgun => &self.shotgun,
            WidgetKind::Confirmation => &self.confirmation,
        };
        widget_state.display
    }

    pub fn display_widget(&mut self, kind: WidgetKind, focus_new: bool) {

        if focus_new {
            if let Some(prev_kind) = self.get_focus() {
                self.get_mut_widget(prev_kind).focus = false;
            }

            self.get_mut_widget(kind).focus = true;
            self.kind_focus(kind);
            self.get_mut_widget(kind).display = true;
        } else {
            self.get_mut_widget(kind).display = true;
        }
    }

    pub fn hide_widget(&mut self, kind: WidgetKind) {

        //this is why you pass by reference I guess
        let kind_copy = kind;
        let widget_kind_ref = self.get_mut_widget(kind);
        widget_kind_ref.display = false;
        widget_kind_ref.focus = false;
        if self.current_focus == Some(kind_copy) {
            self.current_focus = None;
        }

    }

    //TODO: edit this function so that it works correctly with edge cases
    pub fn remove_focus(&mut self) {
        self.log.focus = false;
        self.data.focus = false;
        self.inventory.focus = false;
        self.player.focus = false;
        self.shotgun.focus = false;
        self.confirmation.focus = false;

        self.current_focus = None;
    }

    pub fn kind_focus(&mut self, kind: WidgetKind){
        self.remove_focus();

        match kind {
            WidgetKind::Log => {
                self.log.focus = true;
                self.current_focus = Some(WidgetKind::Log);
            },
            WidgetKind::Data => {
                self.data.focus = true;
                self.current_focus = Some(WidgetKind::Data)
            },
            WidgetKind::Inventory => {
                self.inventory.focus = true;
                self.current_focus = Some(WidgetKind::Inventory);
            },
            WidgetKind::Player => {
                self.player.focus = true;
                self.current_focus = Some(WidgetKind::Player)
            },
            WidgetKind::Shotgun => {
                self.shotgun.focus = true;
                self.current_focus = Some(WidgetKind::Shotgun)
            },
            WidgetKind::Confirmation => {
                self.confirmation.focus = true;
                self.current_focus = Some(WidgetKind::Confirmation);
            },
        }
    }

    pub fn get_color(&self, kind: &WidgetKind) -> Option<Color> {
        match kind {
            WidgetKind::Log => self.log.color,
            WidgetKind::Data => self.data.color,
            WidgetKind::Inventory => self.inventory.color,
            WidgetKind::Shotgun => self.shotgun.color,
            WidgetKind::Player => self.player.color,
            WidgetKind::Confirmation => self.confirmation.color,
        }
    }
}
