#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CategoryId(pub u32);

#[derive(Debug, Clone)]
pub struct Category {
    pub id: CategoryId,
    pub name: String,
}
