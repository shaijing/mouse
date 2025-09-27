mod config;

use crate::config::Config;
use core_foundation::runloop::CFRunLoop;
use core_graphics::event::*;
use std::env;

fn main() {
    match env::current_exe() {
        Ok(exe_path) => {
            let config = Config::load_from_file(exe_path.parent().unwrap().join("mouse.toml"))
                .unwrap_or_default();
            match (config.mouse_reverse, config.trackpad_reverse) {
                (true, false) => {
                    mouse_reverse(&config);
                }
                (false, true) => {
                    trackpad_reverse(&config);
                }
                (true, true) => {
                    mouse_trackpad_reverse(&config);
                }
                _ => {
                    mouse_reverse(&config);
                }
            }
        }
        Err(e) => eprintln!("Fail to access the program dir: {e}"),
    }
}

fn mouse_reverse(config: &Config) {
    CGEventTap::with_enabled(
        CGEventTapLocation::HID,
        CGEventTapPlacement::HeadInsertEventTap,
        CGEventTapOptions::Default,
        vec![CGEventType::ScrollWheel],
        |_proxy, _type, event| {
            let new_event = event.to_owned();
            let delta_y =
                event.get_integer_value_field(EventField::SCROLL_WHEEL_EVENT_DELTA_AXIS_1);
            // let delta_x =
            //     event.get_integer_value_field(EventField::SCROLL_WHEEL_EVENT_DELTA_AXIS_2);

            // let fixed_y = event
            //     .get_integer_value_field(EventField::SCROLL_WHEEL_EVENT_FIXED_POINT_DELTA_AXIS_1);
            // let fixed_x = event
            //     .get_integer_value_field(EventField::SCROLL_WHEEL_EVENT_FIXED_POINT_DELTA_AXIS_2);

            let is_continuous =
                event.get_integer_value_field(EventField::SCROLL_WHEEL_EVENT_IS_CONTINUOUS);

            if is_continuous != 0 {
                // println!(
                //     "Trackpad scroll: fixed=({}, {}), delta=({}, {})",
                //     fixed_x, fixed_y, delta_x, delta_y
                // );
                CallbackResult::Keep
            } else {
                // println!("Mouse wheel scroll: delta=({}, {})", delta_x, delta_y);
                new_event.set_integer_value_field(
                    EventField::SCROLL_WHEEL_EVENT_DELTA_AXIS_1,
                    -delta_y * config.scroll_sensitivity,
                );
                CallbackResult::Replace(new_event)
            }
        },
        || CFRunLoop::run_current(),
    )
    .expect("Failed to install event tap");
}

fn trackpad_reverse(config: &Config) {
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

            if is_continuous != 0 {
                new_event.set_integer_value_field(
                    EventField::SCROLL_WHEEL_EVENT_DELTA_AXIS_1,
                    -delta_y * config.scroll_sensitivity,
                );
                CallbackResult::Replace(new_event)
            } else {
                CallbackResult::Keep
            }
        },
        || CFRunLoop::run_current(),
    )
    .expect("Failed to install event tap");
}

fn mouse_trackpad_reverse(config: &Config) {
    CGEventTap::with_enabled(
        CGEventTapLocation::HID,
        CGEventTapPlacement::HeadInsertEventTap,
        CGEventTapOptions::Default,
        vec![CGEventType::ScrollWheel],
        |_proxy, _type, event| {
            let new_event = event.to_owned();
            let delta_y =
                event.get_integer_value_field(EventField::SCROLL_WHEEL_EVENT_DELTA_AXIS_1);
            new_event.set_integer_value_field(
                EventField::SCROLL_WHEEL_EVENT_DELTA_AXIS_1,
                -delta_y * config.scroll_sensitivity,
            );
            CallbackResult::Replace(new_event)
        },
        || CFRunLoop::run_current(),
    )
    .expect("Failed to install event tap");
}
