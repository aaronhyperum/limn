use glutin;

use widget::{self, EventHandler, ChangePropEvent, PropsChangeEventHandler, DrawableEventHandler, EventArgs, WidgetProperty};
use event::{self, EventId, EventAddress, Signal, InputEvent};
use widgets::primitives::{self, RectDrawable};
use widgets::text::{self, TextDrawable};
use widget::builder::WidgetBuilder;
use util::Dimensions;
use resources::Id;
use color::*;

pub const BUTTON_ENABLED: EventId = EventId("piston/limn/button_enabled");
pub const BUTTON_DISABLED: EventId = EventId("piston/limn/button_disabled");

// show whether button is held down or not
pub struct ButtonDownHandler {}
impl EventHandler for ButtonDownHandler {
    fn event_id(&self) -> EventId {
        event::WIDGET_PRESS
    }
    fn handle_event(&mut self, args: EventArgs) {
        let event = args.event.data::<glutin::Event>();
        match *event {
            glutin::Event::MouseInput(state, button) => {
                let pressed = match state {
                    glutin::ElementState::Pressed => true,
                    glutin::ElementState::Released => false,
                };
                let event = ChangePropEvent::new(WidgetProperty::Pressed, pressed);
                args.event_queue.push(EventAddress::SubTree(args.widget_id), Box::new(event));
            }, _ => ()
        }
    }
}

// show whether toggle button is activated
pub struct ToggleEventHandler {}
impl EventHandler for ToggleEventHandler {
    fn event_id(&self) -> EventId {
        event::WIDGET_PRESS
    }
    fn handle_event(&mut self, args: EventArgs) {
        let EventArgs { event, props, widget_id, event_queue, .. } = args;
        let event = event.data::<glutin::Event>();
        match *event {
            glutin::Event::MouseInput(state, button) => {
                match state {
                    glutin::ElementState::Released => {
                        let activated = props.contains(&WidgetProperty::Activated);
                        let event = ChangePropEvent::new(WidgetProperty::Activated, !activated);
                        event_queue.push(EventAddress::SubTree(widget_id), Box::new(event));
                    }, _ => ()
                }
            }, _ => ()
        }
    }
}

pub struct ToggleButtonBuilder {
    pub widget: WidgetBuilder,
}
impl ToggleButtonBuilder {
    pub fn new() -> Self {

        let rect = RectDrawable { background: RED };

        struct ButtonRectPropsHandler {}
        impl EventHandler for ButtonRectPropsHandler {
            fn event_id(&self) -> EventId {
                event::WIDGET_PROPS_CHANGED
            }
            fn handle_event(&mut self, args: EventArgs) {
                let EventArgs { state, props, .. } = args;
                let pressed = props.contains(&WidgetProperty::Pressed);
                let activated = props.contains(&WidgetProperty::Activated);

                let color_activated = [1.0, 1.0, 1.0, 1.0];
                let color_activated_pressed = [0.9, 0.9, 0.9, 1.0];
                let color_unactivated = [1.0, 0.0, 0.0, 1.0];
                let color_unactivated_pressed = [0.8, 0.0, 0.0, 1.0];
                let color =
                    if pressed && activated { color_activated_pressed } else
                    if activated { color_activated } else
                    if pressed { color_unactivated_pressed }
                    else { color_unactivated };

                state.update(|state: &mut RectDrawable| state.background = color);
            }
        }
        let mut widget = WidgetBuilder::new()
            .set_drawable(primitives::draw_rect, Box::new(rect))
            .add_handler(Box::new(ButtonDownHandler{}))
            .add_handler(Box::new(ToggleEventHandler{}))
            .add_handler(Box::new(PropsChangeEventHandler{}))
            .add_handler(Box::new(ButtonRectPropsHandler{}));
        widget.layout.dimensions(Dimensions {
            width: 100.0,
            height: 50.0,
        });

        ToggleButtonBuilder { widget: widget }
    }
    pub fn set_text(mut self,
                    on_text: &'static str,
                    off_text: &'static str,
                    font_id: Id) -> Self {

        struct ButtonTextPropsHandler {
            on_text: String,
            off_text: String,
        }
        impl EventHandler for ButtonTextPropsHandler {
            fn event_id(&self) -> EventId {
                event::WIDGET_PROPS_CHANGED
            }
            fn handle_event(&mut self, args: EventArgs) {
                let EventArgs { state, props, .. } = args;
                let activated = props.contains(&WidgetProperty::Activated);
                let text = if activated { self.on_text.to_owned() } else { self.off_text.to_owned() };
                state.update(|state: &mut TextDrawable| state.text = text);
            }
        }
        let button_text_drawable = TextDrawable::new(off_text.to_owned(), font_id, 20.0, BLACK, TRANSPARENT);
        let button_text_dims = button_text_drawable.measure_dims_no_wrap();
        let mut button_text_widget = WidgetBuilder::new()
            .set_drawable(text::draw_text, Box::new(button_text_drawable))
            .add_handler(Box::new(PropsChangeEventHandler{}))
            .add_handler(Box::new(ButtonTextPropsHandler{ on_text: on_text.to_owned(), off_text: off_text.to_owned() }));
        button_text_widget.layout.dimensions(button_text_dims);
        button_text_widget.layout.center(&self.widget.layout);

        self.widget.add_child(Box::new(button_text_widget));
        self
    }
}

pub struct PushButtonBuilder {
    pub widget: WidgetBuilder,
}
impl PushButtonBuilder {
    pub fn new() -> Self {
        let rect = RectDrawable { background: RED };
        let mut widget = WidgetBuilder::new()
            .set_drawable(primitives::draw_rect, Box::new(rect));

        widget.layout.dimensions(Dimensions {
            width: 100.0,
            height: 50.0,
        });

        PushButtonBuilder { widget: widget }
    }
    pub fn set_text(mut self, text: &'static str, font_id: Id) -> Self {
        let button_text_drawable = TextDrawable::new(text.to_owned(), font_id, 20.0, BLACK, TRANSPARENT);
        let button_text_dims = button_text_drawable.measure_dims_no_wrap();
        let mut button_text_widget = WidgetBuilder::new()
            .set_drawable(text::draw_text, Box::new(button_text_drawable));
        button_text_widget.layout.dimensions(button_text_dims);
        button_text_widget.layout.center(&self.widget.layout);

        self.widget.add_child(Box::new(button_text_widget));
        self
    }
}