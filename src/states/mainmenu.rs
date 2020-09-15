use crate::{ARENA_WIDTH, ARENA_HEIGHT};
use crate::cache::{FontCache};
use crate::states::slight::{Slight};

use amethyst::{
    prelude::*,
    core::{Transform, Parent, Hidden},
    ecs::{Entity},
    assets::{PrefabLoader, RonFormat, ProgressCounter},
    renderer::{
        sprite::{prefab::SpriteScenePrefab},
    },
    ui::{UiTransform, UiText, UiImage, Interactable, Anchor, LineMode, UiEventType},
};

#[allow(unused_imports)]
use log::*;
use std::collections::HashMap;

pub struct MainMenu {
    my_entities: HashMap<String, Entity>,
    progress: ProgressCounter,
}

impl SimpleState for MainMenu {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        debug!("Loading main screen");
        let StateData { world, .. } = data;

        { // BG
            let bg_prefab = world.exec(|loader: PrefabLoader<'_, SpriteScenePrefab>| {
                loader.load(
                    "mainmenu/bg.ron",
                    RonFormat,
                    &mut self.progress,
                )
            });

            let mut transform = Transform::default();
            transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 0.0);

            self.my_entities.insert("bg".to_string(), world.create_entity()
                .with(bg_prefab)
                .with(transform)
                .build()
            );
        }
        const SCALE: f32 = 4.;
        const WIDTH: f32 = 100.*SCALE;
        const HEIGHT: f32 = 30.*SCALE;
        const SPACING: f32 = -HEIGHT*1.5;
        const FONT_SIZE: f32 = 25.*SCALE;

        { // Slight Button
            let ui_transform = UiTransform::new(
                String::from("mmslight"), // id
                Anchor::Middle,                // anchor
                Anchor::Middle,                // pivot
                0f32,                          // x
                0f32,                          // y
                0f32,                          // z
                WIDTH,                         // width
                HEIGHT,                        // height
            );

            let ui_text;
            {
                let mut cache = world.fetch_mut::<FontCache>();
                let font_handle = cache.get_or_insert("font/square", world);
                ui_text = UiText::new(
                    font_handle,                   // font
                    String::from("Slight"),          // text
                    [0.5, 0.5, 5.0, 1.0],          // color
                    FONT_SIZE,                         // font_size
                    LineMode::Single,              // line mode
                    Anchor::Middle,                // alignment
                );
            }

            let ui_image = UiImage::SolidColor([0.0, 0.4, 0.0, 0.8]);

            let btn = world.create_entity()
                .with(ui_transform)
                .with(ui_text)
                .with(ui_image)
                .with(Interactable)
                .build();
            self.my_entities.insert("slight".to_string(), btn);
        }
        { // Quit Button
            let ui_transform = UiTransform::new(
                String::from("mmquit"), // id
                Anchor::Middle,                // anchor
                Anchor::Middle,                // pivot
                0f32,                          // x
                SPACING,                       // y
                0f32,                          // z
                WIDTH,                         // width
                HEIGHT,                        // height
            );

            let ui_text;
            {
                let mut cache = world.fetch_mut::<FontCache>();
                let font_handle = cache.get_or_insert("font/square", world);
                ui_text = UiText::new(
                    font_handle,                   // font
                    String::from("Quit"),          // text
                    [0.5, 0.5, 5.0, 1.0],          // color
                    FONT_SIZE,                         // font_size
                    LineMode::Single,              // line mode
                    Anchor::Middle,                // alignment
                );
            }

            let ui_image = UiImage::SolidColor([0.0, 0.4, 0.0, 0.8]);

            let btn = world.create_entity()
                .with(ui_transform)
                .with(ui_text)
                .with(ui_image)
                .with(Interactable)
                .with(Parent{
                    entity: self.my_entities.get("slight").copied().unwrap()
                })
                .build();
            self.my_entities.insert("quit".to_string(), btn);
        }

        debug!("Menu screen loaded");
    }

    fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        for ent in self.my_entities.values() {
            let _ = world.delete_entity(*ent);
        }
        self.my_entities = HashMap::new();
    }

    fn on_pause(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        let mut hiddens = world.write_storage::<Hidden>();

        for ent in self.my_entities.values() {
            let _ = hiddens.insert(*ent, Hidden);
        }
    }

    fn on_resume(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        let mut hiddens = world.write_storage::<Hidden>();

        for ent in self.my_entities.values() {
            let _ = hiddens.remove(*ent);
        }
    }

    fn handle_event(&mut self, _data:StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {
        if let StateEvent::Ui(ui_event) = event {
            let target = ui_event.target;
            let quit_button_pressed = Some(target) == self.my_entities.get("quit").copied();
            let slight_button_pressed = Some(target) == self.my_entities.get("slight").copied();

    		match ui_event.event_type {
    			UiEventType::Click if quit_button_pressed => {
                    return Trans::Quit;
    			},
                UiEventType::Click if slight_button_pressed => {
                    return Trans::Push(Box::new(Slight::new()));
    			},
    			_ => {
    				return Trans::None;
    			},
    		};
    	}

        Trans::None
    }
}

impl Default for MainMenu {
    fn default() -> Self {
        Self::new()
    }
}

impl MainMenu {
    pub fn new() -> Self {
        Self{
            my_entities: HashMap::new(),
            progress: Default::default(),
        }
    }
}
