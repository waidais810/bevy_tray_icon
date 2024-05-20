//! Defines the plugins allows you manipulation of the system tray. 


use bevy::app::{App, Plugin, PostUpdate};
use bevy::asset::Assets;
use bevy::prelude::{Image, IntoSystemConfigs, NonSend, Res, resource_added, resource_changed, World};
use tray_icon::TrayIconBuilder;

use crate::plugin::menu_event::MenuEventPlugin;
use crate::resource::{RealTrayIcon, TrayIcon};

pub mod menu_event;


/// The plugin allows you manipulation of the system tray. 
pub struct TrayIconPlugin;

impl Plugin for TrayIconPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MenuEventPlugin);
        app.add_systems(PostUpdate, (
            create_tray.run_if(resource_added::<TrayIcon>),
            update_tray.run_if(resource_changed::<TrayIcon>)
        ).chain());
    }
}

fn create_tray(
    world: &mut World
) {
    let real = TrayIconBuilder::new().build().unwrap();
    world.insert_non_send_resource(RealTrayIcon(real));
}

fn update_tray(
    images: Res<Assets<Image>>,
    tray: Res<TrayIcon>,
    real: NonSend<RealTrayIcon>,
) {
    real.0.set_icon(tray.as_icon(&images)).unwrap();
    real.0.set_tooltip(tray.tooltip.as_ref()).unwrap();
    real.0.set_menu(tray.menu.as_context());
    #[cfg(target_os = "macos")]
    real.0.set_show_menu_on_left_click(tray.show_menu_on_left_click);
}