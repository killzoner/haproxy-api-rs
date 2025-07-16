use std::ops::Deref;

use mlua::{FromLua, IntoLua, Lua, ObjectLike, Result, Table, Value};

use crate::{Converters, Fetches, Http, HttpMessage, LogLevel};

/// The txn class contain all the functions relative to the http or tcp transaction.
#[derive(Clone)]
pub struct Txn {
    class: Table,
    pub c: Converters,
    pub f: Fetches,
    pub(crate) r#priv: Value,
}

impl Txn {
    /// Returns an HTTP class object.
    #[inline]
    pub fn http(&self) -> Result<Http> {
        self.class.get("http")
    }

    /// Returns the request HTTPMessage object.
    pub fn http_req(&self) -> Result<HttpMessage> {
        self.class.get("http_req")
    }

    /// Returns the response HTTPMessage object.
    pub fn http_res(&self) -> Result<HttpMessage> {
        self.class.get("http_res")
    }

    /// Sends a log on the default syslog server if it is configured and on the stderr if it is allowed.
    #[inline]
    pub fn log(&self, level: LogLevel, msg: impl AsRef<str>) -> Result<()> {
        let msg = msg.as_ref();
        self.class.call_method("log", (level, msg))
    }

    /// Sends a log line with the default loglevel for the proxy associated with the transaction.
    #[inline]
    pub fn deflog(&self, msg: impl AsRef<str>) -> Result<()> {
        self.class.call_method("deflog", msg.as_ref())
    }

    /// Returns data stored in the current transaction (with the `set_priv()`) function.
    #[inline]
    pub fn get_priv<R: FromLua>(&self) -> Result<R> {
        self.class.call_method("get_priv", ())
    }

    /// Stores any data in the current HAProxy transaction.
    /// This action replaces the old stored data.
    #[inline]
    pub fn set_priv(&self, val: impl IntoLua) -> Result<()> {
        self.class.call_method("set_priv", val)
    }

    /// Returns data stored in the variable `name`.
    #[inline]
    pub fn get_var<R: FromLua>(&self, name: &str) -> Result<R> {
        self.class.call_method("get_var", name)
    }

    /// Store variable `name` in an HAProxy converting the type.
    #[inline]
    pub fn set_var(&self, name: &str, val: impl IntoLua) -> Result<()> {
        self.class.call_method("set_var", (name, val))
    }

    /// Store variable `name` in an HAProxy if the variable already exists.
    #[inline]
    pub fn set_var_if_exists(&self, name: &str, val: impl IntoLua) -> Result<()> {
        self.class.call_method("set_var", (name, val, true))
    }

    /// Unsets the variable `name`.
    #[inline]
    pub fn unset_var(&self, name: &str) -> Result<()> {
        self.class.call_method("unset_var", name)
    }

    /// Changes the log level of the current request.
    /// The `level` must be an integer between 0 and 7.
    #[inline]
    pub fn set_loglevel(&self, level: LogLevel) -> Result<()> {
        self.class.call_method("set_loglevel", level)
    }
}

impl FromLua for Txn {
    #[inline]
    fn from_lua(value: Value, lua: &Lua) -> Result<Self> {
        let class = Table::from_lua(value, lua)?;
        Ok(Txn {
            c: class.get("c")?,
            f: class.get("f")?,
            class,
            r#priv: Value::Nil,
        })
    }
}

impl Deref for Txn {
    type Target = Table;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.class
    }
}
