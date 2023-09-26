use std::{ffi::CStr, fmt::Debug};

use crate::{error::McdError, mcd_bindings::mcd_server_info_st, system::System, MCD_LIB};

pub struct Connection {
    servers: Vec<mcd_server_info_st>,
}

impl Connection {
    /// Scan for available servers
    pub fn scan() -> anyhow::Result<Self> {
        let servers = MCD_LIB.query_server_infos().add_mcd_error_info(None)?;

        let connection = Connection { servers };

        log::trace!("Scanned for servers, found {connection:?}");

        Ok(connection)
    }

    /// List all servers available in this connection
    pub fn servers(&self) -> impl Iterator<Item = ServerInfo> + '_ {
        self.servers.iter().map(|info| ServerInfo::from(info))
    }

    /// Number of servers available
    pub fn count(&self) -> usize {
        self.servers.len()
    }
}

/// Information a server
#[derive(Clone, Copy)]
pub struct ServerInfo {
    pub(crate) inner: mcd_server_info_st,
}

impl ServerInfo {
    /// Connect to this server exposing the system in it
    pub fn connect(&self) -> anyhow::Result<System> {
        System::connect(self)
    }
}
impl<'a> From<&'a mcd_server_info_st> for ServerInfo {
    fn from(value: &'a mcd_server_info_st) -> Self {
        ServerInfo { inner: *value }
    }
}

impl ServerInfo {
    /// Descriptor of the hardware in use by the server
    pub fn acc_hw(&self) -> &str {
        unsafe { CStr::from_ptr(self.inner.acc_hw.as_ptr()) }
            .to_str()
            .unwrap()
    }

    /// Description of the server itself
    pub fn server(&self) -> &str {
        unsafe { CStr::from_ptr(self.inner.server.as_ptr()) }
            .to_str()
            .unwrap()
    }

    /// TODO what is the semantics of this?
    pub fn system_instance(&self) -> &str {
        unsafe { CStr::from_ptr(self.inner.system_instance.as_ptr()) }
            .to_str()
            .unwrap()
    }
}

impl Debug for ServerInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ServerInfo")
            .field("acc_hw", &self.acc_hw())
            .field("server", &self.server())
            .field("system_instance", &self.system_instance())
            .finish()
    }
}

impl Debug for Connection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result: Vec<_> = self
            .servers
            .iter()
            .map(|server| ServerInfo::from(server))
            .collect();
        f.debug_struct("Connection")
            .field("servers", &result)
            .finish()
    }
}
