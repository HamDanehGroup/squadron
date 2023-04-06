use super::Folder;

pub struct Squadron {
    folders: Vec<Folder>,
}

impl Squadron {
    pub fn build(folders: Vec<Folder>) -> Self {
        Self { folders }
    }
}
