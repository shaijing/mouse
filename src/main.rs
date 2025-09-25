use core_foundation::runloop::CFRunLoop;
use core_graphics::event::*;

fn main() {
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
                new_event
                    .set_integer_value_field(EventField::SCROLL_WHEEL_EVENT_DELTA_AXIS_1, -delta_y);
                CallbackResult::Replace(new_event)
            }
        },
        || CFRunLoop::run_current(),
    )
    .expect("Failed to install event tap");
}
