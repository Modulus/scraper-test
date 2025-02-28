use chrono::{DateTime, Utc};
use sqlx::{Pool, Sqlite};

pub struct Forvalter {
    pub id: i64,
    pub company: String,
    pub name: String,
    pub added: DateTime<Utc>,
}

pub struct Company {
    pub id: i64,
    pub name: String,
    pub added: Option<DateTime<Utc>>,
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
    // fn delete(&self, id: i32) -> impl std::future::Future<Output = Result<Forvalter, sqlx::Error>> + Send;
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
            INSERT INTO company (org_nr, name, added)
            VALUES (?, ?, ?)
            "#,
            company.id,
            company.name,
            company.added
        ).execute(&self.pool).await?;


        Ok(Company {
            id: result.last_insert_rowid() as i64,
            name: company.name,
            added: company.added
        })

    }

    async fn get(&self, id: i64) -> Result<Company, sqlx::Error> {
        todo!()
    }
}



impl ForvalterManager {
    pub fn new(pool: Pool<Sqlite>) -> ForvalterManager {
        ForvalterManager {
            pool
        }
    }
}


// impl BaseManager for ForvalterManager {

//     async fn create(&self, forvalter: Forvalter) -> Result<Forvalter, sqlx::Error> {
//         let now = Utc::now();
//         // let mut conn = self.pool.acquire().await?;
//         let record = sqlx::query!(
//             r#"
//             INSERT INTO forvalter (company, name, added)
//             VALUES (?, ?, ?)
//             "#,
//             forvalter.company,
//             forvalter.name,
//             now
//         ).execute(&self.pool).await?;

//         Ok(Forvalter {
//             id: record.last_insert_rowid() as i32,
//             company: forvalter.company,
//             name: forvalter.name,
//             added: now
//         })
//     }


//     fn get(&self, id: i32) -> Result<Forvalter, sqlx::Error> {
//         todo!()
//     }


//     fn update(&self, forvalter: Forvalter) -> Result<Forvalter, sqlx::Error> {
//         todo!()
//     }

//     fn delete(&self, id: i32) -> Result<(), sqlx::Error> {
//         todo!()
//     }
// }