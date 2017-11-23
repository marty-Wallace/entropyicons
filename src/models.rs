
#[derive(Queryable)]
/// Model representing a queryable imageset. Currently has little function but if user-accounts are
/// enabled,imagesets will be able to be linked to imagesets in the database. Also I needed a
/// database interaction to hit all the checkboxes for class project.
pub struct ImageSet {
    pub id: i32,
    pub name: String,
    pub active: bool,
}

