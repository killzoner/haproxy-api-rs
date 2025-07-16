use mlua::{FromLua, Lua, ObjectLike, Result, Table, Value};

/// A "Listener" class which indicates the manipulated listener.
#[derive(Clone)]
pub struct Listener(Table);

impl Listener {
    /// Returns server statistics.
    #[inline]
    pub fn get_stats(&self) -> Result<Table> {
        self.0.call_method("get_stats", ())
    }
}

impl FromLua for Listener {
    #[inline]
    fn from_lua(value: Value, lua: &Lua) -> Result<Self> {
        Ok(Listener(Table::from_lua(value, lua)?))
    }
}
