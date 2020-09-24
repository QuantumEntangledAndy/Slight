use amethyst::{
    ecs::Entity,
    ecs::{Component, DenseVecStorage},
};

#[allow(dead_code)]
pub(crate) enum ClickAction {
    None,
    DragTo(Vec<Entity>),
    DrawFrom(Entity),
}

pub(crate) struct Clickable {
    click_action: ClickAction,
}

impl Default for Clickable {
    fn default() -> Self {
        Self::new()
    }
}

impl Clickable {
    pub fn new() -> Self {
        Self {
            click_action: ClickAction::None,
        }
    }
    pub fn click(&self) -> &ClickAction {
        &self.click_action
    }

    pub fn set_click(&mut self, new_click: ClickAction) {
        self.click_action = match new_click {
            ClickAction::DragTo(dragpoints) => {
                if !dragpoints.is_empty() {
                    ClickAction::DragTo(dragpoints)
                } else {
                    ClickAction::None
                }
            },
            n => n,
        };
    }
}

impl Component for Clickable {
    type Storage = DenseVecStorage<Self>;
}
