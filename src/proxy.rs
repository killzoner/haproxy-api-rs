use std::collections::HashMap;
use std::ops::Deref;

use mlua::{FromLua, Lua, ObjectLike, Result, String as LuaString, Table, Value};

use crate::{listener::Listener, Server, StickTable};

/// The "Proxy" class provides a way for manipulating proxy
/// and retrieving information like statistics.
#[derive(Clone)]
pub struct Proxy(Table);

#[derive(Debug, PartialEq, Eq)]
pub enum ProxyCapability {
    Frontend,
    Backend,
    Proxy,
    Ruleset,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ProxyMode {
    Tcp,
    Http,
    Health,
    Unknown,
}

impl Proxy {
    /// Returns the name of the proxy.
    #[inline]
    pub fn get_name(&self) -> Result<String> {
        self.0.call_method("get_name", ())
    }

    /// Returns the UUID of the proxy.
    #[inline]
    pub fn get_uuid(&self) -> Result<String> {
        self.0.call_method("get_uuid", ())
    }

    /// Returns a map with the attached servers.
    /// The map is indexed by server name.
    #[inline]
    pub fn get_servers(&self) -> Result<HashMap<String, Server>> {
        self.0.get("servers")
    }

    /// Returns the stick table attached to the proxy.
    #[inline]
    pub fn get_stktable(&self) -> Result<Option<StickTable>> {
        self.0.get("stktable")
    }

    /// Returns a table with the attached listeners.
    /// The table is indexed by listener name.
    #[inline]
    pub fn get_listeners(&self) -> Result<HashMap<String, Listener>> {
        self.0.get("listeners")
    }

    /// Pauses the proxy.
    /// See the management socket documentation for more information.
    #[inline]
    pub fn pause(&self) -> Result<()> {
        self.0.call_method("pause", ())
    }

    /// Resumes the proxy.
    /// See the management socket documentation for more information.
    #[inline]
    pub fn resume(&self) -> Result<()> {
        self.0.call_method("resume", ())
    }

    /// Stops the proxy.
    /// See the management socket documentation for more information.
    #[inline]
    pub fn stop(&self) -> Result<()> {
        self.0.call_method("stop", ())
    }

    /// Kills the session attached to a backup server.
    /// See the management socket documentation for more information.
    #[inline]
    pub fn shut_bcksess(&self) -> Result<()> {
        self.0.call_method("shut_bcksess", ())
    }

    /// Returns a enum describing the capabilities of the proxy.
    #[inline]
    pub fn get_cap(&self) -> Result<ProxyCapability> {
        let cap: LuaString = self.0.call_method("get_cap", ())?;
        match cap.to_str()?.deref() {
            "frontend" => Ok(ProxyCapability::Frontend),
            "backend" => Ok(ProxyCapability::Backend),
            "proxy" => Ok(ProxyCapability::Proxy),
            _ => Ok(ProxyCapability::Ruleset),
        }
    }

    /// Returns a enum describing the mode of the current proxy.
    #[inline]
    pub fn get_mode(&self) -> Result<ProxyMode> {
        let mode: LuaString = self.0.call_method("get_mode", ())?;
        match mode.to_str()?.deref() {
            "tcp" => Ok(ProxyMode::Tcp),
            "http" => Ok(ProxyMode::Http),
            "health" => Ok(ProxyMode::Health),
            _ => Ok(ProxyMode::Unknown),
        }
    }

    /// Returns the number of current active servers for the current proxy
    /// that are eligible for LB.
    #[inline]
    pub fn get_srv_act(&self) -> Result<usize> {
        self.0.call_method("get_srv_act", ())
    }

    /// Returns the number backup servers for the current proxy that are eligible for LB.
    #[inline]
    pub fn get_srv_bck(&self) -> Result<usize> {
        self.0.call_method("get_srv_bck", ())
    }

    /// Returns a table containing the proxy statistics.
    /// The statistics returned are not the same if the proxy is frontend or a backend.
    #[inline]
    pub fn get_stats(&self) -> Result<Table> {
        self.0.call_method("get_stats", ())
    }
}

impl FromLua for Proxy {
    #[inline]
    fn from_lua(value: Value, lua: &Lua) -> Result<Self> {
        let class = Table::from_lua(value, lua)?;
        Ok(Proxy(class))
    }
}

impl Deref for Proxy {
    type Target = Table;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
