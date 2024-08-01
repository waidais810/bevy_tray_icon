//! Convert system tray events to bevy events.


use bevy::app::{App, Plugin, Update};
use bevy::prelude::{Commands, Event, World};
use tray_icon::menu::MenuId;

/// The event fired when a menu is clicked.
#[derive(Event, Debug, Eq, PartialEq)]
pub struct MenuEvent {
    /// The id of the clicked menu.
    pub id: MenuId,
}


pub(crate) struct MenuEventPlugin;

impl Plugin for MenuEventPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<MenuEvent>()
            .add_systems(Update, menu_event);
    }
}

fn menu_event(
    mut commands: Commands
) {
    while let Ok(event) = tray_icon::menu::MenuEvent::receiver().try_recv() {
        commands.add(|w:&mut World|{
            w.send_event(MenuEvent {
                id: event.id,
            });
        });
    }
}
