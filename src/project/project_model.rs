use super::super::schema::projects;

#[derive(Debug, Serialize, Queryable)]
pub struct Project {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub published: bool
}

#[derive(Debug, Clone, Deserialize, Insertable)]
#[table_name="projects"]
pub struct NewProject {
    pub name: String,
    pub description: String,
}

// #[derive(Debug, Clone, Deserialize, Insertable)]
// #[table_name="projects"]
// pub struct NewProject<'a> {
//     pub name: &'a str,
//     pub description: &'a str,
// }