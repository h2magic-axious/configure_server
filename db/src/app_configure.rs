use serde::{Deserialize, Serialize};
use sqlx::postgres::{PgPool, Postgres};

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

// macro_rules! build_app_configure {
//     ($r:expr) => {
//         AppConfigure {
//             id: Some(r.id as u32),
//             name: r.name.clone(),
//             data: r.data.clone().unwrap(),
//             data_type: DataType::from_string(r.data_type.as_str()),
//             description: r.description.clone(),
//             effective: r.effective,
//         }
//     };
// }

impl AppConfigure {
    pub fn to_json(&self) -> serde_json::Value {
        let value = match self.data_type {
            DataType::INT => serde_json::json!(self.data.parse::<i32>().unwrap()),
            DataType::FLOAT => serde_json::json!(self.data.parse::<f64>().unwrap()),
            DataType::BOOL => serde_json::json!(self.data.to_lowercase().as_str() == "true"),
            DataType::STRING => serde_json::json!(self.data),
        };

        serde_json::json!({
            "id": self.id,
            "name": self.name,
            "data": value,
            "data_type": self.data_type.to_string(),
            "effective": self.effective,
            "descption": self.description
        })
    }

    pub async fn all(pool: &PgPool) -> Vec<AppConfigure> {
        let rows = sqlx::query!("SELECT * FROM app_configure")
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

    pub async fn insert(pool: &PgPool, app_configure: Self) -> Self {
        let data_type: &str = app_configure.data_type.to_string();

        let r = sqlx::query!(
            r#"
            INSERT INTO app_configure(name, data_type, data, description)
            VALUES($1, $2, $3, $4)
            RETURNING *
            "#,
            app_configure.name,
            data_type,
            app_configure.data,
            app_configure.description
        )
        .fetch_one(pool)
        .await
        .unwrap();

        AppConfigure {
            id: Some(r.id as u32),
            name: r.name.clone(),
            data: r.data.clone().unwrap(),
            data_type: DataType::from_string(r.data_type.as_str()),
            description: r.description.clone(),
            effective: r.effective,
        }
    }

    pub async fn update(pool: &PgPool, app_configure: Self) -> Self {
        match app_configure.id {
            Some(id) => {
                let data_type: &str = app_configure.data_type.to_string();
                let r = sqlx::query!(
                    r#"
                    UPDATE app_configure SET
                    name=$1, data=$2, data_type=$3, description=$4, effective=$5
                    WHERE id=$6
                    RETURNING *
                    "#,
                    app_configure.name,
                    app_configure.data,
                    data_type,
                    app_configure.description,
                    app_configure.effective,
                    id as i32
                )
                .fetch_one(pool)
                .await
                .unwrap();
                AppConfigure {
                    id: Some(r.id as u32),
                    name: r.name.clone(),
                    data: r.data.clone().unwrap(),
                    data_type: DataType::from_string(r.data_type.as_str()),
                    description: r.description.clone(),
                    effective: r.effective,
                }
            }
            None => Self::insert(pool, app_configure).await,
        }
    }

    pub async fn delete(pool: &PgPool, id: i32) {
        let _ = sqlx::query!("DELETE FROM app_configure WHERE id=$1", id)
            .fetch_one(pool)
            .await;
    }

    pub async fn query_by_id(pool: &PgPool, id: i32) -> Self {
        let rows = sqlx::query!("SELECT * FROM app_configure WHERE id=$1", id)
            .fetch_all(pool)
            .await
            .unwrap();

        let self_list: Vec<Self> = rows
            .iter()
            .map(|r| AppConfigure {
                id: Some(r.id as u32),
                name: r.name.clone(),
                data: r.data.clone().unwrap(),
                data_type: DataType::from_string(r.data_type.as_str()),
                description: r.description.clone(),
                effective: r.effective,
            })
            .collect();

        self_list[0].clone()
    }

    pub async fn query_by_name(pool: &PgPool, name: String) -> Self {
        let rows = sqlx::query!("SELECT * FROM app_configure WHERE name=$1", name)
            .fetch_all(pool)
            .await
            .unwrap();

        let self_list: Vec<Self> = rows
            .iter()
            .map(|r| AppConfigure {
                id: Some(r.id as u32),
                name: r.name.clone(),
                data: r.data.clone().unwrap(),
                data_type: DataType::from_string(r.data_type.as_str()),
                description: r.description.clone(),
                effective: r.effective,
            })
            .collect();

        self_list[0].clone()
    }

    pub async fn update_field_value_with_name(
        pool: &PgPool,
        name: &str,
        field: &str,
        value: &str,
    ) -> bool {
        let sql = format!(
            "UPDATE app_configure SET {}='{}' WHERE name='{}'",
            field, value, name
        );
        println!("SQL: {:?}", sql);
        let r = sqlx::query::<Postgres>(sql.as_str()).execute(pool).await;

        match r {
            Ok(_) => true,
            Err(err) => {
                println!("Error: {:?}", err);
                false
            }
        }
    }
}
