use mlua::{FromLua, IntoLuaMulti, Lua, ObjectLike, Result, Table, Value};

/// The "Converters" class allows to call a lot of internal HAProxy sample converters.
#[derive(Clone)]
pub struct Converters(Table);

impl Converters {
    /// Executes an internal haproxy sample converter.
    #[inline]
    pub fn get<R>(&self, name: &str, args: impl IntoLuaMulti) -> Result<R>
    where
        R: FromLua,
    {
        self.0.call_method(name, args)
    }

    /// The same as `get` but always returns string.
    #[inline]
    pub fn get_str(&self, name: &str, args: impl IntoLuaMulti) -> Result<String> {
        Ok((self.0.call_method::<Option<_>>(name, args)?).unwrap_or_default())
    }
}

impl FromLua for Converters {
    #[inline]
    fn from_lua(value: Value, lua: &Lua) -> Result<Self> {
        Ok(Converters(Table::from_lua(value, lua)?))
    }
}
