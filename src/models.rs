use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Debug, PartialEq)]
pub enum MutationKind {
    Create,
    Delete,
}

#[derive(Clone, Serialize, Debug, PartialEq)]
pub struct TodoUpdate {
    pub mutation_kind: MutationKind,
    pub id: i32,
}

#[derive(sqlx::FromRow, Serialize, Deserialize, PartialEq, Clone)]
pub struct Todo {
    pub id: i32,
    pub description: String,
}

#[derive(sqlx::FromRow, Serialize, Deserialize, PartialEq)]
pub struct TodoNew {
    pub description: String,
}
