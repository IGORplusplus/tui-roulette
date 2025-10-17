//widget-data.rs
use std::cell::{ Ref, RefCell };

use ratatui::layout::Rect;

use crate::ui::{SHOTGUN_ART, BANG, CLICK};

use ratatui::style::{Color, Style, Stylize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WidgetKind {
    Log,
    Data,
    Inventory,
    Player,
    Shotgun,
}

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

    fn change_state_content(&self, content: &str) -> WidgetState {
        let content: String = content.to_string();
        WidgetState {
            display: self.display,
            focus: self.focus,
            area: self.area,
            content: Some(content),
            color: self.color,
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

    pub fn change_focus(&mut self) {
        self.focus = !self.focus;
    }
}

#[derive(Debug, Clone)]
pub struct WidgetData{
    //these are a little redundant
    log: RefCell<WidgetState>,
    data: RefCell<WidgetState>,
    inventory: RefCell<WidgetState>,
    player: RefCell<WidgetState>,
    shotgun: RefCell<WidgetState>,

    current_focus: Option<WidgetKind>,
    //render last in list first
    pub render_stack: Vec<WidgetKind>,
}

impl WidgetData {
    pub fn new() -> WidgetData {
        WidgetData {
            log: RefCell::new(WidgetState::new_blank()),
            data: RefCell::new(WidgetState::new_color(Some(Color::Green))),
            inventory: RefCell::new(WidgetState::new_blank()),
            player: RefCell::new(WidgetState::new_blank()),
            shotgun: RefCell::new(WidgetState::new_content(SHOTGUN_ART)),
            current_focus: None,

            render_stack: Vec::new(),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (WidgetKind, std::cell::Ref<WidgetState>)> {
        [
            (WidgetKind::Log, self.log.borrow()),
            (WidgetKind::Data, self.data.borrow()),
            (WidgetKind::Inventory, self.inventory.borrow()),
            (WidgetKind::Player, self.player.borrow()),
            (WidgetKind::Shotgun, self.shotgun.borrow()),
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
            WidgetKind::Shotgun,
        ]
    }

    fn get(&self, kind: WidgetKind) -> &RefCell<WidgetState> {
        match kind {
            WidgetKind::Log => &self.log,
            WidgetKind::Data => &self.data,
            WidgetKind::Inventory => &self.inventory,
            WidgetKind::Player => &self.player,
            WidgetKind::Shotgun => &self.shotgun,
        }
    }

    ///TODO: I want to understand this code
    pub fn focus_next(&mut self) {
        let order = Self::order();

        let log_displayed = self.get(WidgetKind::Log).borrow().display;

        // Find current focus index
        let current_idx = order.iter().position(|&kind| self.get(kind).borrow_mut().focus);

        // Clear all focus
        for kind in order.iter() {
            self.get(*kind).borrow_mut().focus = false;
        }

        // Start searching from the next index
        let mut next_idx = match current_idx {
            Some(i) => (i + 1) % order.len(),
            None => 0,
        };

        // Loop until we find a displayed (and allowed) widget
        for _ in 0..order.len() {
            let kind = order[next_idx];

            // skip Shotgun if Log is displayed
            if log_displayed && kind == WidgetKind::Shotgun {
                next_idx = (next_idx + 1) % order.len();
                continue;
            }

            if self.get(kind).borrow().display {
                self.get(kind).borrow_mut().focus = true;
                return;
            }

            next_idx = (next_idx + 1) % order.len();
        }
    }

    pub fn focus_prev(&mut self) {
        let order = Self::order();

        // Find current focus index
        let current_idx = order.iter().position(|&kind| self.get(kind).focus);

        // Clear all focus
        for kind in order {
            self.get_mut(kind).focus = false;
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
                self.get_mut(order[prev_idx]).focus = true;
                return;
            }
            prev_idx = if prev_idx == 0 { order.len() - 1 } else { prev_idx - 1 };
        }
    }

    pub fn is_displayed(&self, kind: WidgetKind) -> bool{
        let widget_state = match kind {
            WidgetKind::Log => &self.log,
            WidgetKind::Data => &self.data,
            WidgetKind::Inventory => &self.inventory,
            WidgetKind::Player => &self.player,
            WidgetKind::Shotgun => &self.shotgun,
        };
        widget_state.display
    }

    pub fn is_focused(&self, kind: WidgetKind) -> bool{
        let widget_state = match kind {
            WidgetKind::Log => &self.log,
            WidgetKind::Data => &self.data,
            WidgetKind::Inventory => &self.inventory,
            WidgetKind::Player => &self.player,
            WidgetKind::Shotgun => &self.shotgun,
        };
        widget_state.focus
    }

    pub fn toggle_focus(&mut self, kind: WidgetKind) {
        match kind {
            WidgetKind::Log => self.log.focus = !self.log.focus,
            WidgetKind::Data => self.data.focus = !self.data.focus,
            WidgetKind::Inventory => self.inventory.focus = !self.inventory.focus,
            WidgetKind::Player => self.player.focus = !self.player.focus,
            WidgetKind::Shotgun => self.shotgun.focus = !self.shotgun.focus,
        }

        if self.current_focus == Some(kind) {
            self.current_focus = None;
        }
        else {
            self.current_focus = Some(kind);
        }
    }

    pub fn get_state(&self, kind: WidgetKind) -> Ref<WidgetState> {
        match kind {
            WidgetKind::Log => self.log.borrow(),
            WidgetKind::Data => &self.data.borrow(),
            WidgetKind::Inventory => &self.inventory.borrow(),
            WidgetKind::Player => &self.player.borrow(),
            WidgetKind::Shotgun => &self.shotgun.borrow(),
        }
    }

    pub fn set_widget(&mut self, kind: WidgetKind, display_b: bool, focus_b: bool) {
        if focus_b {
            self.log.borrow_mut().focus = false;
            self.data.borrow_mut().focus = false;
            self.inventory.borrow_mut().focus = false;
            self.player.borrow_mut().focus = false;
            self.shotgun.borrow_mut().focus = false;
        }

        let widget_to_modify = match kind {
            WidgetKind::Log => &mut self.log,
            WidgetKind::Data => &mut self.data,
            WidgetKind::Inventory => &mut self.inventory,
            WidgetKind::Player => &mut self.player,
            WidgetKind::Shotgun => &mut self.shotgun,
        };
        //_b means boolean
        let mut widget = widget_to_modify.borrow_mut();
        widget.display = display_b;
        widget.focus = focus_b;
        if focus_b {
            self.current_focus = Some(kind);
        }
    }

    pub fn change_content(&mut self, kind: WidgetKind, content: Option<String>) {
        match kind {
            WidgetKind::Log => self.log.borrow_mut().content = content,
            WidgetKind::Data => self.data.borrow_mut().content = content,
            WidgetKind::Inventory => self.inventory.borrow_mut().content = content,
            WidgetKind::Shotgun => self.shotgun.borrow_mut().content = content,
            WidgetKind::Player => self.player.borrow_mut().content = content,
            _=> panic!("can't change content of invalid widget kind"),
        }
    }

    pub fn kind_focus(&mut self, kind: &WidgetKind){
        self.log.borrow_mut().focus = false;
        self.data.borrow_mut().focus = false;
        self.inventory.borrow_mut().focus = false;
        self.player.borrow_mut().focus = false;
        self.shotgun.borrow_mut().focus = false;

        match kind {
            WidgetKind::Log => self.log.borrow_mut().focus = true,
            WidgetKind::Data => self.data.borrow_mut().focus = true,
            WidgetKind::Inventory => self.inventory.borrow_mut().focus = true,
            WidgetKind::Player => self.player.borrow_mut().focus = true,
            WidgetKind::Shotgun => self.shotgun.borrow_mut().focus = true,
        }
    }

    pub fn get_color(&self, kind: &WidgetKind) -> Option<Color> {
        match kind {
            WidgetKind::Log => self.log.borrow().color,
            WidgetKind::Data => self.data.borrow().color,
            WidgetKind::Inventory => self.inventory.borrow().color,
            WidgetKind::Shotgun => self.shotgun.borrow().color,
            WidgetKind::Player => self.player.borrow().color,
        }
    }
}
