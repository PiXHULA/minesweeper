use crate::Board;
use crate::events::TileTriggerEvent;

use bevy::input::ButtonState;
use bevy::input::mouse::MouseButtonInput;

use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub fn input_handling(
    window_primary_query: Query<&Window, With<PrimaryWindow>>,
    board: Res<Board>,
    mut button_event_reader: EventReader<MouseButtonInput>,
    mut tile_trigger_event_writer: EventWriter<TileTriggerEvent>,
) {
   let Ok(window) = window_primary_query.get_single() else { return };

   for event in button_event_reader.read() {
       if let ButtonState::Pressed = event.state {
           if let Some(click_position) = window.cursor_position() {
               if let Some(tile_coordinates) = board.mouse_position(window, click_position) {
                   match event.button  {
                       MouseButton::Left => {
                           #[cfg(feature = "debug")]
                           info!("Trying uncover tile on {}", tile_coordinates);
                           tile_trigger_event_writer.send(TileTriggerEvent {
                               coordinates: tile_coordinates
                           });
                       },
                       MouseButton::Right => {
                           #[cfg(feature = "debug")]
                           info!("Trying mark tile on {}", tile_coordinates);
                       },
                       _ => (),
                   }
               }
           };
       }
   }
}