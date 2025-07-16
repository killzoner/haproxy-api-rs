use std::ops::Deref;

use mlua::{AsChunk, Chunk, FromLua, Lua, ObjectLike, Result, Table, Value};

use crate::{EventSub, Proxy};

/// The "Server" class provides a way for manipulating servers and retrieving information.
#[derive(Clone)]
pub struct Server(Table);

impl Server {
    /// Returns the name of the server.
    #[inline]
    pub fn get_name(&self) -> Result<String> {
        self.0.call_method("get_name", ())
    }

    /// Returns the proxy unique identifier of the server.
    #[inline]
    pub fn get_puid(&self) -> Result<String> {
        self.0.call_method("get_puid", ())
    }

    /// Returns the rid (revision ID) of the server.
    #[inline]
    pub fn get_rid(&self) -> Result<u64> {
        self.0.call_method("get_rid", ())
    }

    /// Returns true if the server is currently draining sticky connections.
    #[inline]
    pub fn is_draining(&self) -> Result<bool> {
        self.0.call_method("is_draining", ())
    }

    /// Return true if the server is a backup server.
    #[inline]
    pub fn is_backup(&self) -> Result<bool> {
        self.0.call_method("is_backup", ())
    }

    /// Return true if the server was instantiated at runtime (e.g.: from the cli).
    #[inline]
    pub fn is_dynamic(&self) -> Result<bool> {
        self.0.call_method("is_dynamic", ())
    }

    /// Return the number of currently active sessions on the server.
    pub fn get_cur_sess(&self) -> Result<u64> {
        self.0.call_method("get_cur_sess", ())
    }

    /// Return the number of pending connections to the server.
    #[inline]
    pub fn get_pend_conn(&self) -> Result<u64> {
        self.0.call_method("get_pend_conn", ())
    }

    /// Dynamically changes the maximum connections of the server.
    #[inline]
    pub fn set_maxconn(&self, maxconn: u64) -> Result<()> {
        self.0.call_method("set_maxconn", maxconn)
    }

    /// Returns an integer representing the server maximum connections.
    #[inline]
    pub fn get_maxconn(&self) -> Result<u64> {
        self.0.call_method("get_maxconn", ())
    }

    /// Dynamically changes the weight of the server.
    /// See the management socket documentation for more information about the format of the string.
    #[inline]
    pub fn set_weight(&self, weight: &str) -> Result<()> {
        self.0.call_method("set_weight", weight)
    }

    /// Returns an integer representing the server weight.
    #[inline]
    pub fn get_weight(&self) -> Result<u32> {
        self.0.call_method("get_weight", ())
    }

    /// Dynamically changes the address of the server.
    #[inline]
    pub fn set_addr(&self, addr: String, port: Option<u16>) -> Result<()> {
        self.0.call_method("set_addr", (addr, port))
    }

    /// Returns a string describing the address of the server.
    #[inline]
    pub fn get_addr(&self) -> Result<String> {
        self.0.call_method("get_addr", ())
    }

    /// Returns a table containing the server statistics.
    #[inline]
    pub fn get_stats(&self) -> Result<Table> {
        self.0.call_method("get_stats", ())
    }

    /// Returns the parent proxy to which the server belongs.
    pub fn get_proxy(&self) -> Result<Proxy> {
        self.0.call_method("get_proxy", ())
    }

    /// Shutdowns all the sessions attached to the server.
    #[inline]
    pub fn shut_sess(&self) -> Result<()> {
        self.0.call_method("shut_sess", ())
    }

    /// Drains sticky sessions.
    #[inline]
    pub fn set_drain(&self) -> Result<()> {
        self.0.call_method("set_drain", ())
    }

    /// Sets maintenance mode.
    #[inline]
    pub fn set_maint(&self) -> Result<()> {
        self.0.call_method("set_maint", ())
    }

    /// Sets normal mode.
    #[inline]
    pub fn set_ready(&self) -> Result<()> {
        self.0.call_method("set_ready", ())
    }

    /// Enables health checks.
    #[inline]
    pub fn check_enable(&self) -> Result<()> {
        self.0.call_method("check_enable", ())
    }

    /// Disables health checks.
    #[inline]
    pub fn check_disable(&self) -> Result<()> {
        self.0.call_method("check_disable", ())
    }

    /// Forces health-check up.
    #[inline]
    pub fn check_force_up(&self) -> Result<()> {
        self.0.call_method("check_force_up", ())
    }

    /// Forces health-check nolb mode.
    #[inline]
    pub fn check_force_nolb(&self) -> Result<()> {
        self.0.call_method("check_force_nolb", ())
    }

    /// Forces health-check down.
    #[inline]
    pub fn check_force_down(&self) -> Result<()> {
        self.0.call_method("check_force_down", ())
    }

    /// Enables agent check.
    #[inline]
    pub fn agent_enable(&self) -> Result<()> {
        self.0.call_method("agent_enable", ())
    }

    /// Disables agent check.
    #[inline]
    pub fn agent_disable(&self) -> Result<()> {
        self.0.call_method("agent_disable", ())
    }

    /// Forces agent check up.
    #[inline]
    pub fn agent_force_up(&self) -> Result<()> {
        self.0.call_method("agent_force_up", ())
    }

    /// Forces agent check down.
    #[inline]
    pub fn agent_force_down(&self) -> Result<()> {
        self.0.call_method("agent_force_down", ())
    }

    /// Check if the current server is tracking another server.
    #[inline]
    pub fn tracking(&self) -> Result<Option<Server>> {
        self.0.call_method("tracking(", ())
    }

    /// Check if the current server is being tracked by other servers.
    #[inline]
    pub fn get_trackers(&self) -> Result<Vec<Server>> {
        self.0.call_method("get_trackers", ())
    }

    /// Register a function that will be called on specific server events.
    ///
    /// It works exactly like `core.event_sub()` except that the subscription
    /// will be performed within the server dedicated subscription list instead of the global one.
    pub fn event_sub(&self, event_types: &[&str], code: impl AsChunk) -> Result<EventSub> {
        self.0
            .call_function("event_sub", (event_types, Chunk::wrap(code)))
    }
}

impl FromLua for Server {
    #[inline]
    fn from_lua(value: Value, lua: &Lua) -> Result<Self> {
        let class = Table::from_lua(value, lua)?;
        Ok(Server(class))
    }
}

impl Deref for Server {
    type Target = Table;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
