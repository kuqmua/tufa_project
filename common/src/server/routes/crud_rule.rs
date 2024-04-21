#[derive(Debug, Clone, Copy)]
pub enum CrudRule {
    Read,
    ReadCreate,
    ReadUpdate,
    ReadDelete,
    ReadCreateUpdate,
    ReadUpdateDelete,
    ReadCreateDelete,
    ReadCreateUpdateDelete,
}
