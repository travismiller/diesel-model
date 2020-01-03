use diesel::associations::HasTable;
use diesel::backend::Backend;
use diesel::QueryDsl;

pub trait Model: HasTable {
    type All: QueryDsl;
    type AllColumns: Copy + Sized;
    type Backend: Backend;

    fn all() -> Self::All;
}
