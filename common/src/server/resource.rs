#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, from_str::FromStr)]
pub enum Resource {
    #[default]
    Local,
    Mongodb,
    PostgreSql,
}