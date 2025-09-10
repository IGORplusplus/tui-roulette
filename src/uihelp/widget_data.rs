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

#[derive(Debug)]
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
}

impl WidgetData {
    pub fn new() -> WidgetData {
        WidgetData {
            log: WidgetState::new(),
            popup: WidgetState::new(),
            inventory: WidgetState::new(),
            player: WidgetState::new(),
            shotgun: WidgetState::new(),
        }
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
    }

    pub fn change_focus(&mut self, next: bool) {
    }
}
