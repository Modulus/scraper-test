use std::sync::Arc;

use sqlx::{Pool, Sqlite};
use sqlx::types::chrono::{Utc, DateTime};
pub struct Forvalter {
    pub id: i64,
    pub company: String,
    pub name: String,
    // pub added: DateTime<Utc>,
}
#[derive(Debug, PartialEq, Eq)]
pub struct Company {
    pub id: i64,
    pub name: String,
    // pub added: DateTime<Utc>,
}


pub trait BaseForvalterManager {

    fn create(&self, forvalter: Forvalter) ->  impl std::future::Future<Output = Result<Forvalter, sqlx::Error>> + Send;
    fn get(&self, id: i64) -> impl std::future::Future<Output = Result<Forvalter, sqlx::Error>> + Send;
    // fn update(&self, forvalter: Forvalter) -> impl std::future::Future<Output = Result<Forvalter, sqlx::Error>> + Send;
    // fn delete(&self, id: i32) -> impl std::future::Future<Output = Result<Forvalter, sqlx::Error>> + Send;
}

pub trait BaseCompanyManager {

    fn create(&self, forvalter: Company) ->  impl std::future::Future<Output = Result<Company, sqlx::Error>> + Send;
    fn get(&self, id: i64) -> impl std::future::Future<Output = Result<Company, sqlx::Error>> + Send;
    // fn update(&self, forvalter: Forvalter) -> impl std::future::Future<Output = Result<Forvalter, sqlx::Error>> + Send;
    fn delete(&self, id: i64) -> impl std::future::Future<Output = Result<bool, sqlx::Error>> + Send;
}



#[derive(Clone)]
pub struct ForvalterManager {
    pub pool: Pool<Sqlite>
}

#[derive(Clone)]
pub struct CompanyManager {
    pub pool: Pool<Sqlite>
}


impl CompanyManager {
    pub fn new(pool: Pool<Sqlite>) -> CompanyManager {
        CompanyManager {
            pool
        }
    }
}

impl BaseCompanyManager for CompanyManager {
    async fn create(&self, company: Company) ->  Result<Company, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            INSERT INTO company (org_nr, name)
            VALUES (?, ?)
            "#,
            company.id,
            company.name,
            // company.added
        ).execute(&self.pool).await?;


        Ok(Company {
            id: result.last_insert_rowid() as i64,
            name: company.name,
            // added: company.added
        })

    }

    async fn get(&self, id: i64) -> Result<Company, sqlx::Error> { 
        let result = sqlx::query!(
            r#"
            SELECT org_nr, name FROM company WHERE org_nr = ?
            "#,
            id
        ).fetch_one(&self.pool).await?;

        Ok(Company {
            id: result.org_nr,
            name: result.name,
            // added: result.added 
        })

    }
    
    async fn delete(&self, id: i64) -> Result<bool, sqlx::Error> {
        let rows_affected = sqlx::query!(
            r#"
            DELETE FROM company
            WHERE org_nr = ?
            "#,
            id
        )
        .execute(&self.pool)
        .await?
        .rows_affected();
    
        Ok(rows_affected > 0)
    }
}



impl ForvalterManager {
    pub fn new(pool: Pool<Sqlite>) -> ForvalterManager {
        ForvalterManager {
            pool
        }
    }
}


impl BaseForvalterManager for ForvalterManager {
    //TODO: Add fetch for company (org_nr) using name
    async fn create(&self, forvalter: Forvalter) -> Result<Forvalter, sqlx::Error> {
        let now = Utc::now();
        // let mut conn = self.pool.acquire().await?;
        let record = sqlx::query!(
            r#"
            INSERT INTO forvalter (id, name)
            VALUES (?, ?)
            "#,
            forvalter.id,
            forvalter.name,
        ).execute(&self.pool).await?;

        Ok(Forvalter {
            id: record.last_insert_rowid() as i64,
            company: "forvalter.company".into(),
            name: forvalter.name,
        })
    }


    async fn get(&self, id: i64) -> Result<Forvalter, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            SELECT id, name FROM forvalter WHERE id = ?
            "#,
            id
        ).fetch_one(&self.pool).await?;

        Ok(Forvalter {
            id: result.id,
            name: result.name,
            company: "forvalter.company".into(),
            // added: result.added 
        })
    }


    // async fn update(&self, forvalter: Forvalter) -> Result<Forvalter, sqlx::Error> {
    //     todo!()
    // }

    // async fn delete(&self, id: i32) -> Result<(), sqlx::Error> {
    //     todo!()
    // }
}