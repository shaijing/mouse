mod config;

use crate::config::Config;
use core_foundation::runloop::CFRunLoop;
use core_graphics::event::*;
use std::env;

fn main() {
    match env::current_exe() {
        Ok(exe_path) => {
            let config = exe_path
                .parent()
                .and_then(|p| Config::load_from_file(p.join("mouse.toml")).ok())
                .unwrap_or_default();

            create_scroll_event_tap(&config);
        }
        Err(e) => eprintln!("Failed to access the program directory: {e}"),
    }
}

fn create_scroll_event_tap(config: &Config) {
    CGEventTap::with_enabled(
        CGEventTapLocation::HID,
        CGEventTapPlacement::HeadInsertEventTap,
        CGEventTapOptions::Default,
        vec![CGEventType::ScrollWheel],
        |_proxy, _type, event| {
            let new_event = event.to_owned();
            let delta_y =
                event.get_integer_value_field(EventField::SCROLL_WHEEL_EVENT_DELTA_AXIS_1);
            let delta_x =
                event.get_integer_value_field(EventField::SCROLL_WHEEL_EVENT_DELTA_AXIS_2);

            let is_continuous =
                event.get_integer_value_field(EventField::SCROLL_WHEEL_EVENT_IS_CONTINUOUS);

            let should_reverse = if is_continuous != 0 {
                config.trackpad_reverse
            } else {
                config.mouse_reverse
            };

            if should_reverse {
                if config.vertical_reverse {
                    new_event.set_integer_value_field(
                        EventField::SCROLL_WHEEL_EVENT_DELTA_AXIS_1,
                        -delta_y * config.scroll_sensitivity,
                    );
                }
                if config.horizontal_reverse {
                    new_event.set_integer_value_field(
                        EventField::SCROLL_WHEEL_EVENT_DELTA_AXIS_2,
                        -delta_x * config.scroll_sensitivity,
                    );
                }
                CallbackResult::Replace(new_event)
            } else {
                CallbackResult::Keep
            }
        },
        || CFRunLoop::run_current(),
    )
    .expect("Failed to install event tap");
}
