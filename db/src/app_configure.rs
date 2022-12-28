use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPool;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum DataType {
    INT,
    FLOAT,
    BOOL,
    STRING,
}

impl DataType {
    pub fn from_string(st: &str) -> Self {
        match st {
            "int" => DataType::INT,
            "float" => DataType::FLOAT,
            "bool" => DataType::BOOL,
            "string" => DataType::STRING,
            _ => DataType::STRING,
        }
    }

    pub fn to_string(&self) -> &str {
        match self {
            DataType::INT => "int",
            DataType::FLOAT => "float",
            DataType::BOOL => "bool",
            DataType::STRING => "string",
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppConfigure {
    pub id: Option<u32>,
    pub name: String,
    pub data_type: DataType,
    pub data: String,
    pub description: Option<String>,
    pub effective: Option<bool>,
}

impl AppConfigure {
    pub async fn query_effective(pool: &PgPool) -> Vec<AppConfigure> {
        let rows = sqlx::query!("SELECT * FROM app_configure WHERE effective = TRUE")
            .fetch_all(pool)
            .await
            .unwrap();

        rows.iter()
            .map(|r| AppConfigure {
                id: Some(r.id as u32),
                name: r.name.clone(),
                data: r.data.clone().unwrap(),
                data_type: DataType::from_string(r.data_type.as_str()),
                description: r.description.clone(),
                effective: r.effective,
            })
            .collect()
    }

    // pub fn insert(pool: &PgPool, app_configure: Self) -> Self {
    //     let data_type: &str = app_configure.data_type.to_string();

    //     let row = sqlx::query!(
    //         r#"
            
    //         "#,
            
    //     )

    //     AppConfigure {
    //         id: (),
    //         name: (),
    //         data_type: (),
    //         data: (),
    //         description: (),
    //         effective: (),
    //     }
    // }
}
