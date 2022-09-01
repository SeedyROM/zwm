use std::collections::HashMap;

use color_eyre::Result;
use xcb::x;

static SUPPORTED_ATOMS: &[(Atom, &str)] = &[
    (Atom::Utf8String, "UTF8_STRING"),
    (Atom::WM(WMAtom::WMProtocols), "WM_PROTOCOLS"),
    (Atom::WM(WMAtom::WMDelete), "WM_DELETE"),
    (Atom::WM(WMAtom::WMState), "WM_STATE"),
    (Atom::WM(WMAtom::WMTakeFocus), "WM_TAKE_FOCUS"),
    (Atom::Net(NetAtom::NetActiveWindow), "_NET_ACTIVE_WINDOW"),
    (Atom::Net(NetAtom::NetSupported), "_NET_SUPPORTED"),
    (Atom::Net(NetAtom::NetWMName), "_NET_WM_NAME"),
    (Atom::Net(NetAtom::NetWMState), "_NET_WM_STATE"),
    (Atom::Net(NetAtom::NewWMCheck), "_NET_SUPPORTING_WM_CHECK"),
    (
        Atom::Net(NetAtom::NetWMFullscreen),
        "_NET_WM_STATE_FULLSCREEN",
    ),
    (Atom::Net(NetAtom::NetWMWindowType), "_NET_WM_WINDOW_TYPE"),
    (
        Atom::Net(NetAtom::NetWMWindowTypeDialog),
        "_NET_WM_WINDOW_TYPE_DIALOG",
    ),
    (Atom::Net(NetAtom::NetClientList), "_NET_CLIENT_LIST"),
];

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub enum WMAtom {
    WMProtocols,
    WMDelete,
    WMState,
    WMTakeFocus,
}

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub enum NetAtom {
    NetActiveWindow,
    NetSupported,
    NetWMName,
    NetWMState,
    NewWMCheck,
    NetWMFullscreen,
    NetWMWindowType,
    NetWMWindowTypeDialog,
    NetClientList,
}

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub enum Atom {
    Utf8String,
    WM(WMAtom),
    Net(NetAtom),
}

pub struct AtomManager {
    atoms: HashMap<Atom, x::Atom>,
}

impl AtomManager {
    pub fn new() -> Self {
        Self {
            atoms: HashMap::new(),
        }
    }

    pub fn setup(&mut self, connection: &xcb::Connection) -> Result<()> {
        for (atom, name) in SUPPORTED_ATOMS.iter() {
            let request = connection.send_request(&x::InternAtom {
                only_if_exists: false,
                name: name.as_bytes(),
            });

            let reply = connection.wait_for_reply(request)?;

            self.atoms.insert(*atom, reply.atom());
        }

        Ok(())
    }
}
