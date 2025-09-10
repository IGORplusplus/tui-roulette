//widget-data.rs
#[derive(Debug)]
pub struct WidgetState {
    display: bool,
    focus: bool,
}

impl WidgetState {
    pub fn new() -> WidgetState {
        WidgetState {
            display: false,
            focus: false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WidgetKind {
    Log,
    Popup,
    Inventory,
    Player,
    Shotgun,
}

#[derive(Debug)]
pub struct WidgetData{
    log: WidgetState,
    popup: WidgetState,
    inventory: WidgetState,
    player: WidgetState,
    shotgun: WidgetState,
    current_focus: Option<WidgetKind>,
}

impl WidgetData {
    pub fn new() -> WidgetData {
        WidgetData {
            log: WidgetState::new(),
            popup: WidgetState::new(),
            inventory: WidgetState::new(),
            player: WidgetState::new(),
            shotgun: WidgetState::new(),
            current_focus: None,
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (WidgetKind, &WidgetState)> {
        [
            (WidgetKind::Log, &self.log),
            (WidgetKind::Popup, &self.popup),
            (WidgetKind::Inventory, &self.inventory),
            (WidgetKind::Player, &self.player),
            (WidgetKind::Shotgun, &self.shotgun),
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
            WidgetKind::Popup,
            WidgetKind::Inventory,
            WidgetKind::Player,
            WidgetKind::Shotgun,
        ]
    }

    fn get_mut(&mut self, kind: WidgetKind) -> &mut WidgetState {
        match kind {
            WidgetKind::Log => &mut self.log,
            WidgetKind::Popup => &mut self.popup,
            WidgetKind::Inventory => &mut self.inventory,
            WidgetKind::Player => &mut self.player,
            WidgetKind::Shotgun => &mut self.shotgun,
        }
    }

    ///TODO: I want to understand this code
    pub fn focus_next(&mut self) {
        let order = Self::order();

        // Find current focus index
        let current_idx = order.iter().position(|&kind| self.get(kind).focus );

        // Clear all focus
        for kind in order {
            self.get_mut(kind).focus = false;
        }

        // Start searching from the next index
        let mut next_idx = match current_idx {
            Some(i) => (i + 1) % order.len(),
            None => 0,
        };

        // Loop until we find a displayed widget
        for _ in 0..order.len() {
            if self.get(order[next_idx]).display {
                self.get_mut(order[next_idx]).focus = true;
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

    fn get(&self, kind: WidgetKind) -> &WidgetState {
        match kind {
            WidgetKind::Log => &self.log,
            WidgetKind::Popup => &self.popup,
            WidgetKind::Inventory => &self.inventory,
            WidgetKind::Player => &self.player,
            WidgetKind::Shotgun => &self.shotgun,
        }
    }

    pub fn is_displayed(&self, kind: WidgetKind) -> bool{
        let widget_state = match kind {
            WidgetKind::Log => &self.log,
            WidgetKind::Popup => &self.popup,
            WidgetKind::Inventory => &self.inventory,
            WidgetKind::Player => &self.player,
            WidgetKind::Shotgun => &self.shotgun,
        };
        widget_state.display
    }

    pub fn is_focused(&self, kind: WidgetKind) -> bool{
        let widget_state = match kind {
            WidgetKind::Log => &self.log,
            WidgetKind::Popup => &self.popup,
            WidgetKind::Inventory => &self.inventory,
            WidgetKind::Player => &self.player,
            WidgetKind::Shotgun => &self.shotgun,
        };
        widget_state.focus
    }

    pub fn set_widget(&mut self, kind: WidgetKind, display_b: bool, focus_b: bool) {
        if focus_b {
            self.log.focus = false;
            self.popup.focus = false;
            self.inventory.focus = false;
            self.player.focus = false;
            self.shotgun.focus = false;
        }

        let widget_to_modify = match kind {
            WidgetKind::Log => &mut self.log,
            WidgetKind::Popup => &mut self.popup,
            WidgetKind::Inventory => &mut self.inventory,
            WidgetKind::Player => &mut self.player,
            WidgetKind::Shotgun => &mut self.shotgun,
        };
        widget_to_modify.display = display_b;
        widget_to_modify.focus = focus_b;
        if focus_b {
            self.current_focus = Some(kind);
        }
    }

    pub fn change_focus(&mut self, next: bool) {
        self.log.focus = false;
        self.popup.focus = false;
        self.inventory.focus = false;
        self.player.focus = false;
        self.shotgun.focus = false;
    }
}
