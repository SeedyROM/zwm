use color_eyre::{eyre::eyre, Result};
use nix::sys::signal::{signal, SigHandler, Signal};
use tracing::{info, trace};
use xcb::{x, Connection};

use crate::events::EventHandler;

/// The main window manager struct.
pub struct Wm {
    connection: Connection,
    #[allow(dead_code)]
    screen_number: i32,
    screen: x::ScreenBuf,
}

impl Wm {
    /// Create a new window manager and setup the connect to the X server.
    pub fn new(display_name: Option<&str>) -> Result<Self> {
        info!(
            "Connecting to screen at display {}",
            display_name.unwrap_or("DEFAULT")
        );

        let (connection, screen_number) = Connection::connect(display_name).map_err(|err| {
            eyre!(
                "Connection to display {} failed: {}",
                display_name.unwrap_or("DEFAULT"),
                err
            )
        })?;
        let setup = connection.get_setup();
        let screen = setup
            .roots()
            .nth(screen_number as usize)
            .ok_or_else(|| eyre!("Failed to get screen {}", screen_number))?
            .to_owned();

        info!(
            "Connected to screen at display {}",
            display_name.unwrap_or("DEFAULT")
        );

        Ok(Self {
            connection,
            screen_number,
            screen,
        })
    }

    /// Setup the window manager.
    pub fn setup(&self) -> Result<()> {
        self.check_sole_wm()?;
        self.setup_sigchld_handler()?;
        self.setup_root_window()?;

        Ok(())
    }

    /// Start the event loop.
    pub fn run(&mut self) -> Result<()> {
        loop {
            let event = self.connection.wait_for_event()?;
            self.dispatch(event)?;
        }
    }

    /// Ensure that we're the only window manager running.
    fn check_sole_wm(&self) -> Result<()> {
        trace!("Checking that zwm is the only window manager on root window");
        self.connection
            .send_and_check_request(&x::ChangeWindowAttributes {
                window: self.screen.root(),
                value_list: &[x::Cw::EventMask(x::EventMask::SUBSTRUCTURE_REDIRECT)],
            })
            .map_err(|_| eyre!("Another window manager already exists"))
    }

    /// Ignore SIGCHLD signals to stop zombie processes.
    /// We need unsafe to properly setup the handler.
    fn setup_sigchld_handler(&self) -> Result<()> {
        trace!("Setting up signal handler for reaping child processes");
        unsafe { signal(Signal::SIGCHLD, SigHandler::SigIgn) }?;
        trace!("Sucessfully setup signal handler for reaping child processes");

        Ok(())
    }

    /// Modify the root window to redirect events to us.
    fn setup_root_window(&self) -> Result<()> {
        trace!("Setting up the root window");
        // Change root window attributes.
        self.connection
            .send_and_check_request(&x::ChangeWindowAttributes {
                window: self.screen.root(),
                value_list: &[x::Cw::EventMask(
                    x::EventMask::SUBSTRUCTURE_REDIRECT
                        | x::EventMask::SUBSTRUCTURE_NOTIFY
                        | x::EventMask::BUTTON_PRESS
                        | x::EventMask::POINTER_MOTION
                        | x::EventMask::ENTER_WINDOW
                        | x::EventMask::LEAVE_WINDOW
                        | x::EventMask::STRUCTURE_NOTIFY
                        | x::EventMask::PROPERTY_CHANGE,
                )],
            })
            .map_err(|_| eyre!("Failed to root window redirection."))?;

        trace!("Successfully set up the root window");

        Ok(())
    }
}

impl EventHandler for Wm {
    fn handle_map_request(&mut self, event: &x::MapRequestEvent) -> Result<()> {
        info!("Received MapRequestEvent: {:?}", event);
        Ok(())
    }
}
