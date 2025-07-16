use std::ops::Deref;

use mlua::{FromLua, Lua, ObjectLike, Result, Table, Value};

/// The "EventSub" class that can be used to manipulate HAProxy subscription.
#[derive(Clone)]
pub struct EventSub(Table);

impl EventSub {
    /// Returns stick table attributes as a Lua table.
    #[inline]
    pub fn unsub(&self) -> Result<Table> {
        self.0.call_method("unsub", ())
    }
}

impl FromLua for EventSub {
    #[inline]
    fn from_lua(value: Value, lua: &Lua) -> Result<Self> {
        let class = Table::from_lua(value, lua)?;
        Ok(EventSub(class))
    }
}

impl Deref for EventSub {
    type Target = Table;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
