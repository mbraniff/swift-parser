#[derive(Debug)]
pub enum Type {
    /// (Value)
    SymbolType(String),
    /// (Underlying)
    ListType(Box<Type>),
    /// (Key, Value)
    DictType(Box<Type>, Box<Type>),
}