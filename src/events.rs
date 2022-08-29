use color_eyre::Result;
use tracing::trace;
use xcb::{self, x};

/// Abstraction to handle incoming events from the X server.
#[allow(unused_variables)]
pub trait EventHandler {
    /// Dispatch our event handlers based on the incoming event.
    fn dispatch(&mut self, event: xcb::Event) -> Result<()> {
        match event {
            xcb::Event::X(event) => {
                trace!("Received event: {:?}", event);

                match event {
                    x::Event::ButtonPress(event) => self.handle_button_press(&event),
                    x::Event::ClientMessage(event) => self.handle_client_message(&event),
                    x::Event::ConfigureRequest(event) => self.handle_configure_request(&event),
                    x::Event::ConfigureNotify(event) => self.handle_configure_notify(&event),
                    x::Event::DestroyNotify(event) => self.handle_destroy_notify(&event),
                    x::Event::EnterNotify(event) => self.handle_enter_notify(&event),
                    x::Event::Expose(event) => self.handle_expose(&event),
                    x::Event::FocusIn(event) => self.handle_focus_in(&event),
                    x::Event::KeyPress(event) => self.handle_key_press(&event),
                    x::Event::MappingNotify(event) => self.handle_mapping_notify(&event),
                    x::Event::MapRequest(event) => self.handle_map_request(&event),
                    x::Event::MotionNotify(event) => self.handle_motion_notify(&event),
                    x::Event::PropertyNotify(event) => self.handle_property_notify(&event),
                    x::Event::UnmapNotify(event) => self.handle_upmap_notify(&event),
                    _ => Ok(()),
                }
            }
            xcb::Event::Unknown(event) => {
                trace!("Received unknown event: {:?}", event);
                Ok(())
            }
        }
    }

    fn handle_button_press(&mut self, event: &x::ButtonPressEvent) -> Result<()> {
        Ok(())
    }

    fn handle_client_message(&mut self, event: &x::ClientMessageEvent) -> Result<()> {
        Ok(())
    }

    fn handle_configure_request(&mut self, event: &x::ConfigureRequestEvent) -> Result<()> {
        Ok(())
    }

    fn handle_configure_notify(&mut self, event: &x::ConfigureNotifyEvent) -> Result<()> {
        Ok(())
    }

    fn handle_destroy_notify(&mut self, event: &x::DestroyNotifyEvent) -> Result<()> {
        Ok(())
    }

    fn handle_enter_notify(&mut self, event: &x::EnterNotifyEvent) -> Result<()> {
        Ok(())
    }

    fn handle_expose(&mut self, event: &x::ExposeEvent) -> Result<()> {
        Ok(())
    }

    fn handle_focus_in(&mut self, event: &x::FocusInEvent) -> Result<()> {
        Ok(())
    }

    fn handle_key_press(&mut self, event: &x::KeyPressEvent) -> Result<()> {
        Ok(())
    }

    fn handle_mapping_notify(&mut self, event: &x::MappingNotifyEvent) -> Result<()> {
        Ok(())
    }

    fn handle_map_request(&mut self, event: &x::MapRequestEvent) -> Result<()> {
        Ok(())
    }

    fn handle_motion_notify(&mut self, event: &x::MotionNotifyEvent) -> Result<()> {
        Ok(())
    }

    fn handle_property_notify(&mut self, event: &x::PropertyNotifyEvent) -> Result<()> {
        Ok(())
    }

    fn handle_upmap_notify(&mut self, event: &x::UnmapNotifyEvent) -> Result<()> {
        Ok(())
    }
}
