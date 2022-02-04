use clap::{App, AppSettings, Arg};
use once_cell::sync::OnceCell;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;
use tide::prelude::*;
use tide::Request;
type ResourceDB = RwLock<Vec<Option<serde_json::Value>>>;
static DATABASE: OnceCell<HashMap<String, ResourceDB>> = OnceCell::new();

fn get_db(resource: &str) -> &ResourceDB {
    println!("Checking database for {:?}", resource);
    match DATABASE.get() {
        Some(db) => db.get(resource).expect("TODO validate"),
        None => panic!("No database setup!!"),
    }
}
fn not_found_error() -> tide::Error {
    tide::Error::new(
        tide::StatusCode::NotFound,
        anyhow::anyhow!("Resource does not exist"),
    )
}

async fn post_resource(mut req: Request<()>) -> Result<String, tide::Error> {
    let resource = req.param("resource")?.to_owned();
    let json: String = req.body_string().await?;
    let json: serde_json::Value = serde_json::from_str(&json)?;
    let mut writer = get_db(&resource).write().unwrap();
    let id = writer.len();
    if let serde_json::Value::Object(mut obj) = json {
        obj.insert("id".to_owned(), json!(id));
        writer.push(Some(serde_json::Value::Object(obj)))
    }

    Ok(format!("{{id: {} }}", id))
}

async fn get_resource(req: Request<()>) -> Result<String, tide::Error> {
    let resource = req.param("resource")?.to_owned();
    let reader = get_db(&resource).read().unwrap();
    let mut results: Vec<serde_json::Value> = vec![];
    for item in reader.iter() {
        if let Some(json) = item {
            results.push(json.clone());
        }
    }

    Ok(serde_json::to_string(&results)?.to_string())
}

async fn get_resource_item(req: Request<()>) -> Result<String, tide::Error> {
    let resource = req.param("resource")?.to_owned();
    let key = req.param("id")?;

    let id = key.parse::<usize>().unwrap();

    let reader = get_db(&resource).read().unwrap();

    if reader.len() < id {
        return Err(not_found_error());
    }

    match &reader[id] {
        Some(result) => Ok(serde_json::to_string(result)?.to_string()),
        None => Err(not_found_error()),
    }
}
fn create_resource_endpoints(app: &mut tide::Server<()>) -> () {
    // locker with Vec<Option<String>> as the database for the server
    // Map of map to hold all the databases?

    app.at("/:resource").post(post_resource).get(get_resource);

    app.at("/:resource/:id")
        .put(|mut req: Request<()>| async move {
            // check if exists
            let key = req.param("id")?;
            let json = req.body_string().await?;
            //save it
            Ok(format!("{{id: {} }}", 0))
        })
        .get(get_resource_item);
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let matches = App::new("kyo")
        .arg(
            Arg::with_name("port")
                .takes_value(true)
                .index(1)
                .required(true),
        )
        .arg(
            Arg::with_name("endpoints")
                .multiple(true)
                .min_values(1)
                .takes_value(true)
                .index(2)
                .help("Define resources to have endpoints"),
        )
        .get_matches();

    tide::log::start();
    let mut app = tide::new();
    let mut map: HashMap<String, ResourceDB> = HashMap::new();

    for resource in matches.values_of("endpoints").unwrap() {
        println!("Setting up {:?}", resource);
        map.insert(resource.to_owned(), RwLock::new(vec![]));
    }

    DATABASE
        .set(map)
        .expect("Could not set HashMap in the database");

    create_resource_endpoints(&mut app);
    app.listen("127.0.0.1:".to_owned() + matches.value_of("port").unwrap())
        .await?;
    Ok(())
}
