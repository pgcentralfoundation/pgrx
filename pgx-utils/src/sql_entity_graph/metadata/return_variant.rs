#[derive(Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum ReturnVariant {
    Plain,
    SetOf,
    Table,
}
