use rusqlite::{params, Connection, Result, Rows, Statement};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct Response {
    pub from_sdk: String,
    pub to_sdk: String,
    pub number: i32,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct ExampleResponse {
    pub from_sdk: String,
    pub to_sdk: String,
    pub examples: Vec<Example>,
}

impl Default for ExampleResponse {
    fn default() -> Self {
        ExampleResponse {
            from_sdk: "".to_string(),
            to_sdk: "".to_string(),
            examples: Vec::new(),
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Example {
    pub id: i32,
    pub name: String,
}

pub fn from_slug_a_to_slug_b(slugs: Vec<String>) -> Result<Vec<Response>> {
    let st: String = slugs
        .iter()
        .map(|s| format!("'{}'", s))
        .collect::<Vec<String>>()
        .join(",");
    let conn: Connection = Connection::open("./database/data.db")?;
    let query: String = format!(
        "
    SELECT sdk1.slug, sdk2.slug, COUNT(DISTINCT app_sdk_1.app_id)
    FROM app_sdk AS app_sdk_1, app_sdk AS app_sdk_2, sdk AS sdk1, sdk AS sdk2 
    WHERE app_sdk_1.app_id = app_sdk_2.app_id AND app_sdk_1.sdk_id = sdk1.id 
    AND app_sdk_2.sdk_id = sdk2.id AND app_sdk_1.installed = 0 
    AND app_sdk_2.installed = 1 
    AND sdk1.slug IN ({}) 
    AND sdk2.slug IN ({}) 
    GROUP BY sdk1.id, sdk2.id;",
        st, st
    );
    let mut stmt: Statement<'_> = conn.prepare(&query)?;
    let mut rows: Rows<'_> = stmt.query(params![])?;
    let mut response_vec: Vec<Response> = Vec::new();
    while let Some(row) = rows.next()? {
        let name2: String = row.get(0)?;
        let name6: String = row.get(1)?;
        let id9: i32 = row.get(2)?;
        response_vec.push(Response {
            from_sdk: name2,
            to_sdk: name6,
            number: id9,
        });
    }
    Ok(response_vec)
}

pub fn from_none_to_slug(slugs: Vec<String>) -> Result<Vec<Response>> {
    let conn: Connection = Connection::open("./database/data.db")?;
    let st: String = slugs
        .iter()
        .map(|s| format!("'{}'", s))
        .collect::<Vec<String>>()
        .join(",");
    let query: String = format!(
        "
    SELECT sdk2.slug, COUNT(DISTINCT app_sdk_1.app_id)
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
    let mut stmt: Statement<'_> = conn.prepare(&query)?;
    let mut rows: Rows<'_> = stmt.query(params![])?;
    let mut response_vec: Vec<Response> = Vec::new();
    while let Some(row) = rows.next()? {
        let name6: String = row.get(0)?;
        let id9: i32 = row.get(1)?;
        response_vec.push(Response {
            from_sdk: "None".to_string(),
            to_sdk: name6,
            number: id9,
        })
    }
    Ok(response_vec)
}

pub fn from_slug_to_none(slugs: Vec<String>) -> Result<Vec<Response>> {
    let conn: Connection = Connection::open("./database/data.db")?;
    let st: String = slugs
        .iter()
        .map(|s| format!("'{}'", s))
        .collect::<Vec<String>>()
        .join(",");
    let query: String = format!(
        "
    SELECT sdk1.slug, COUNT(DISTINCT app_sdk_1.app_id)
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
    let mut stmt: Statement<'_> = conn.prepare(&query)?;
    let mut rows: Rows<'_> = stmt.query(params![])?;
    let mut response_vec: Vec<Response> = Vec::new();
    while let Some(row) = rows.next()? {
        let name2: String = row.get(0)?;
        let id9: i32 = row.get(1)?;
        response_vec.push(Response {
            from_sdk: name2,
            to_sdk: "None".to_string(),
            number: id9,
        })
    }
    Ok(response_vec)
}

pub fn from_slug_to_itself(slugs: Vec<String>) -> Result<Vec<Response>> {
    let conn: Connection = Connection::open("./database/data.db")?;
    let st: String = slugs
        .iter()
        .map(|s| format!("'{}'", s))
        .collect::<Vec<String>>()
        .join(",");
    let query: String = format!(
        "
    SELECT sdk.slug, COUNT(DISTINCT app_sdk.app_id)
    FROM app_sdk, sdk
    WHERE app_sdk.sdk_id = sdk.id
    AND app_sdk.installed = true
    AND sdk.slug IN ({})
    GROUP BY sdk.id;
",
        st
    );
    let mut stmt: Statement<'_> = conn.prepare(&query)?;
    let mut rows: Rows<'_> = stmt.query(params![])?;
    let mut response_vec: Vec<Response> = Vec::new();
    while let Some(row) = rows.next()? {
        let name2: String = row.get(0)?;
        let id9: i32 = row.get(1)?;
        response_vec.push(Response {
            from_sdk: name2.clone(),
            to_sdk: name2,
            number: id9,
        })
    }
    Ok(response_vec)
}

pub fn from_none_to_none(slugs: Vec<String>) -> Result<Vec<Response>> {
    let conn: Connection = Connection::open("./database/data.db")?;
    let st: String = slugs
        .iter()
        .map(|s| format!("'{}'", s))
        .collect::<Vec<String>>()
        .join(",");
    let query: String = format!(
        "
    SELECT COUNT(app.id)
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
    let mut stmt: Statement<'_> = conn.prepare(&query)?;
    let mut rows: Rows<'_> = stmt.query(params![])?;
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

pub fn examples_from_slug_a_to_slug_b(slug_1: String, slug_2: String) -> Result<ExampleResponse> {
    let conn: Connection = Connection::open("./database/data.db")?;
    let query: String = format!(
        "SELECT App.id, App.name
        FROM app_sdk AS app_sdk_1
        JOIN app_sdk AS app_sdk_2 ON app_sdk_1.app_id = app_sdk_2.app_id
        JOIN sdk AS Sdk1 ON app_sdk_1.sdk_id = Sdk1.id
        JOIN sdk AS Sdk2 ON app_sdk_2.sdk_id = Sdk2.id
        JOIN app AS App ON App.id = app_sdk_1.app_id
        WHERE app_sdk_1.installed = False
          AND app_sdk_2.installed = True
          AND Sdk1.slug = '{}'
          AND Sdk2.slug = '{}'
        LIMIT 10;
        ",
        slug_1, slug_2
    );
    let mut stmt: Statement<'_> = conn.prepare(&query)?;
    let mut rows: Rows<'_> = stmt.query(params![])?;
    let mut example_vec: Vec<Example> = Vec::new();
    while let Some(row) = rows.next()? {
        let example_id: i32 = row.get(0)?;
        let example_name: String = row.get(1)?;
        example_vec.push(Example {
            id: example_id,
            name: example_name,
        });
    }
    Ok(ExampleResponse {
        from_sdk: slug_1,
        to_sdk: slug_2,
        examples: example_vec,
    })
}

pub fn examples_from_none_to_slug(
    slug: String,
    available_slug: Vec<String>,
) -> Result<ExampleResponse> {
    let st: String = available_slug
        .iter()
        .map(|s| format!("'{}'", s))
        .collect::<Vec<String>>()
        .join(",");
    let conn: Connection = Connection::open("./database/data.db")?;
    let query: String = format!(
        "SELECT App.id, App.name
        FROM app_sdk AS app_sdk_1
        JOIN app_sdk AS app_sdk_2 ON app_sdk_1.app_id = app_sdk_2.app_id
        JOIN sdk AS Sdk1 ON app_sdk_1.sdk_id = Sdk1.id
        JOIN sdk AS Sdk2 ON app_sdk_2.sdk_id = Sdk2.id
        JOIN app AS App ON App.id = app_sdk_1.app_id
        WHERE app_sdk_1.installed = False
          AND app_sdk_2.installed = True
          AND Sdk1.slug NOT IN ({})
          AND Sdk2.slug = '{}'
        LIMIT 10;
        ",
        st, slug
    );
    let mut stmt: Statement<'_> = conn.prepare(&query)?;
    let mut rows: Rows<'_> = stmt.query(params![])?;
    let mut example_vec: Vec<Example> = Vec::new();
    while let Some(row) = rows.next()? {
        let example_id: i32 = row.get(0)?;
        let name: String = row.get(1)?;
        example_vec.push(Example {
            id: example_id,
            name,
        });
    }
    Ok(ExampleResponse {
        from_sdk: "None".to_string(),
        to_sdk: slug,
        examples: example_vec,
    })
}

pub fn examples_from_slug_to_none(
    slug: String,
    available_slug: Vec<String>,
) -> Result<ExampleResponse> {
    let st: String = available_slug
        .iter()
        .map(|s| format!("'{}'", s))
        .collect::<Vec<String>>()
        .join(",");
    let conn: Connection = Connection::open("./database/data.db")?;
    let query: String = format!(
        "SELECT App.id, App.name
        FROM app_sdk AS app_sdk_1
        JOIN app_sdk AS app_sdk_2 ON app_sdk_1.app_id = app_sdk_2.app_id
        JOIN sdk AS Sdk1 ON app_sdk_1.sdk_id = Sdk1.id
        JOIN sdk AS Sdk2 ON app_sdk_2.sdk_id = Sdk2.id
        JOIN app AS App ON App.id = app_sdk_1.app_id
        WHERE app_sdk_1.installed = True
          AND app_sdk_2.installed = False
          AND Sdk1.slug = '{}'
          AND Sdk2.slug NOT IN ({})
        LIMIT 10;
        ",
        slug, st
    );
    let mut stmt: Statement<'_> = conn.prepare(&query)?;
    let mut rows: Rows<'_> = stmt.query(params![])?;
    let mut example_vec: Vec<Example> = Vec::new();
    while let Some(row) = rows.next()? {
        let example_id: i32 = row.get(0)?;
        let name: String = row.get(1)?;
        example_vec.push(Example {
            id: example_id,
            name,
        });
    }
    Ok(ExampleResponse {
        from_sdk: slug,
        to_sdk: "None".to_string(),
        examples: example_vec,
    })
}

pub fn examples_from_slug_to_itself(slug: String) -> Result<ExampleResponse> {
    let conn: Connection = Connection::open("./database/data.db")?;
    let query: String = format!(
        "SELECT App.id, App.name
        FROM app_sdk
        JOIN sdk ON app_sdk.sdk_id = sdk.id
        JOIN app ON app_sdk.app_id = app.id
        WHERE app_sdk.installed = 1
          AND sdk.slug = '{}'
        LIMIT 10;              
        ",
        slug
    );
    let mut stmt: Statement<'_> = conn.prepare(&query)?;
    let mut rows: Rows<'_> = stmt.query(params![])?;
    let mut example_vec: Vec<Example> = Vec::new();
    while let Some(row) = rows.next()? {
        let example_id: i32 = row.get(0)?;
        let name: String = row.get(1)?;
        example_vec.push(Example {
            id: example_id,
            name,
        });
    }
    Ok(ExampleResponse {
        from_sdk: slug.clone(),
        to_sdk: slug,
        examples: example_vec,
    })
}

pub fn examples_from_none_to_none(available_slugs: Vec<String>) -> Result<ExampleResponse> {
    let st: String = available_slugs
        .iter()
        .map(|s| format!("'{}'", s))
        .collect::<Vec<String>>()
        .join(",");
    let conn: Connection = Connection::open("./database/data.db")?;
    let query: String = format!(
        "SELECT App.id, App.name
        FROM app
        WHERE NOT EXISTS (
            SELECT 1
            FROM app_sdk
            JOIN sdk ON app_sdk.sdk_id = sdk.id
            WHERE app_sdk.installed = 1
              AND sdk.slug IN ({})
              AND app_sdk.app_id = app.id
        )
        LIMIT 10;
        ",
        st
    );
    let mut stmt: Statement<'_> = conn.prepare(&query)?;
    let mut rows: Rows<'_> = stmt.query(params![])?;
    let mut example_vec: Vec<Example> = Vec::new();
    while let Some(row) = rows.next()? {
        let example_id: i32 = row.get(0)?;
        let name: String = row.get(1)?;
        example_vec.push(Example {
            id: example_id,
            name,
        });
    }
    Ok(ExampleResponse {
        from_sdk: "None".to_string(),
        to_sdk: "None".to_string(),
        examples: example_vec,
    })
}
