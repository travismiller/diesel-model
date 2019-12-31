pub trait Model {
    type AllColumns: Copy;
    type All;
    const ALL_COLUMNS: Self::AllColumns;
}
