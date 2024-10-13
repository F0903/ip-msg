use crate::{
    db::{error::Result, local_db::LocalDb},
    models::Contact,
};

pub struct ContactsSurface<'a> {
    db: &'a LocalDb,
}

impl<'a> ContactsSurface<'a> {
    pub async fn on(db: &'a LocalDb) -> Result<Self> {
        let me = Self { db };
        Ok(me)
    }

    pub async fn write(&self, contact: Contact) -> Result<()> {
        let name = contact.get_name();
        let ip = contact.get_ip();
        sqlx::query!("INSERT INTO contacts (name, ip) VALUES (?, ?)", name, ip)
            .execute(self.db.get_pool())
            .await?;
        Ok(())
    }

    pub async fn get_all(&self) -> Result<Vec<Contact>> {
        let val = sqlx::query_as("SELECT * FROM contacts")
            .fetch_all(self.db.get_pool())
            .await?;
        Ok(val)
    }
}
