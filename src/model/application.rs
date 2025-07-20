use sqlx::FromRow;

#[derive(Debug)]
pub struct ApplicationAdd {
    pub name: i64,
    pub action: String,
}

#[derive(Debug, FromRow)]
pub struct Application {
    pub id: i64,
    pub name: String,
    pub action: String,
}

impl Application {
    pub fn new(id: i64, name: String, action: String) -> Self {
        Application { id, name, action }
    } 

    pub fn get_name(&self) -> String {
        return self.name.to_owned();
    }
}