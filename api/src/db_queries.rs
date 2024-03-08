use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct Response {
    pub from_sdk: String,
    pub to_sdk: String,
    pub number: i32,
}

pub fn slug_a_to_slug_b(slugs: Vec<String>) -> Result<Vec<Response>> {
    let st = slugs
        .iter()
        .map(|s| format!("'{}'", s))
        .collect::<Vec<String>>()
        .join(",");
    let conn = Connection::open("./database/data.db")?;
    let query = format!(
        "
    SELECT app_sdk_1.*, app_sdk_2.*, sdk1.*, sdk2.*, COUNT(DISTINCT app_sdk_1.app_id)
    AS count_1 
    FROM app_sdk AS app_sdk_1, app_sdk AS app_sdk_2, sdk AS sdk1, sdk AS sdk2 
    WHERE app_sdk_1.app_id = app_sdk_2.app_id AND app_sdk_1.sdk_id = sdk1.id 
    AND app_sdk_2.sdk_id = sdk2.id AND app_sdk_1.installed = 0 
    AND app_sdk_2.installed = 1 
    AND sdk1.slug IN ({}) 
    AND sdk2.slug IN ({}) 
    GROUP BY sdk1.id, sdk2.id;",
        st, st
    );
    let mut stmt = conn.prepare(&query)?;
    let mut rows = stmt.query(params![])?;
    let mut response_vec: Vec<Response> = Vec::new();
    while let Some(row) = rows.next()? {
        let name2: String = row.get(8)?;
        let name6: String = row.get(13)?;
        let id9: i32 = row.get(16)?;
        response_vec.push(Response {
            from_sdk: name2,
            to_sdk: name6,
            number: id9,
        });
    }
    Ok(response_vec)
}

pub fn from_none_to_slug(slugs: Vec<String>) -> Result<Vec<Response>> {
    let conn = Connection::open("./database/data.db")?;
    let st = slugs
        .iter()
        .map(|s| format!("'{}'", s))
        .collect::<Vec<String>>()
        .join(",");
    let query = format!(
        "
    SELECT app_sdk_1.*, app_sdk_2.*, sdk1.*, sdk2.*, COUNT(DISTINCT app_sdk_1.app_id) AS count_1
    FROM app_sdk AS app_sdk_1, app_sdk AS app_sdk_2, sdk AS sdk1, sdk AS sdk2
    WHERE app_sdk_1.app_id = app_sdk_2.app_id
    AND app_sdk_1.sdk_id = sdk1.id
    AND app_sdk_2.sdk_id = sdk2.id
    AND app_sdk_1.installed = false
    AND app_sdk_2.installed = true
    AND sdk1.slug NOT IN ({})
    AND sdk2.slug IN ({})
    GROUP BY
    sdk2.id;",
        st, st
    );
    let mut stmt = conn.prepare(&query)?;
    let mut rows = stmt.query(params![])?;
    let mut response_vec: Vec<Response> = Vec::new();
    while let Some(row) = rows.next()? {
        let name6: String = row.get(13)?;
        let id9: i32 = row.get(16)?;
        response_vec.push(Response {
            from_sdk: "None".to_string(),
            to_sdk: name6,
            number: id9,
        })
    }
    Ok(response_vec)
}

pub fn from_slug_to_none(slugs: Vec<String>) -> Result<Vec<Response>> {
    let conn = Connection::open("./database/data.db")?;
    let st = slugs
        .iter()
        .map(|s| format!("'{}'", s))
        .collect::<Vec<String>>()
        .join(",");
    let query = format!(
        "
    SELECT app_sdk_1.*, app_sdk_2.*, sdk1.*, sdk2.*, COUNT(DISTINCT app_sdk_1.app_id) AS count_1
    FROM app_sdk AS app_sdk_1, app_sdk AS app_sdk_2, sdk AS sdk1, sdk AS sdk2
    WHERE app_sdk_1.app_id = app_sdk_2.app_id
    AND app_sdk_1.sdk_id = sdk1.id
    AND app_sdk_2.sdk_id = sdk2.id
    AND app_sdk_1.installed = true
    AND app_sdk_2.installed = false
    AND sdk1.slug IN ({})
    AND sdk2.slug NOT IN ({})
    GROUP BY
    sdk1.id;",
        st, st
    );
    let mut stmt = conn.prepare(&query)?;
    let mut rows = stmt.query(params![])?;
    let mut response_vec: Vec<Response> = Vec::new();
    while let Some(row) = rows.next()? {
        let name2: String = row.get(8)?;
        let id9: i32 = row.get(16)?;
        response_vec.push(Response {
            from_sdk: name2,
            to_sdk: "None".to_string(),
            number: id9,
        })
    }
    Ok(response_vec)
}

pub fn from_slug_to_itself(slugs: Vec<String>) -> Result<Vec<Response>> {
    let conn = Connection::open("./database/data.db")?;
    let st = slugs
        .iter()
        .map(|s| format!("'{}'", s))
        .collect::<Vec<String>>()
        .join(",");
    let query = format!(
        "
    SELECT COUNT(DISTINCT app_sdk.app_id) AS count_1, app_sdk.*, sdk.*
    FROM app_sdk, sdk
    WHERE app_sdk.sdk_id = sdk.id
    AND app_sdk.installed = true
    AND sdk.slug IN ({})
    GROUP BY sdk.id;
",
        st
    );
    let mut stmt = conn.prepare(&query)?;
    let mut rows = stmt.query(params![])?;
    let mut response_vec: Vec<Response> = Vec::new();
    while let Some(row) = rows.next()? {
        let name2: String = row.get(6)?;
        let id9: i32 = row.get(0)?;
        response_vec.push(Response {
            from_sdk: name2.clone(),
            to_sdk: name2,
            number: id9,
        })
    }
    Ok(response_vec)
}

pub fn from_none_to_none(slugs: Vec<String>) -> Result<Vec<Response>> {
    let conn = Connection::open("./database/data.db")?;
    let st = slugs
        .iter()
        .map(|s| format!("'{}'", s))
        .collect::<Vec<String>>()
        .join(",");
    let query = format!(
        "
    SELECT COUNT(app.id) AS count_1
    FROM app
    WHERE NOT EXISTS (
    SELECT 1 FROM app_sdk, sdk
    WHERE app_sdk.sdk_id = sdk.id
    AND sdk.slug IN ({})
    AND app_sdk.installed = 1
    AND app_sdk.app_id = app.id
    );",
        st
    );
    let mut stmt = conn.prepare(&query)?;
    let mut rows = stmt.query(params![])?;
    let mut response_vec: Vec<Response> = Vec::new();
    while let Some(row) = rows.next()? {
        let id9: i32 = row.get(0)?;
        response_vec.push(Response {
            from_sdk: "None".to_string(),
            to_sdk: "None".to_string(),
            number: id9,
        })
    }
    Ok(response_vec)
}
