use serde::{Serialize};

#[derive(Clone, Serialize, Debug)]
pub enum MutationKind {
    Create,
    Delete,
    Update,
}
