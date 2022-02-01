use std::sync::Arc;
use std::sync::RwLock;
use tide::prelude::*;
use tide::Request;

fn resources_list(json_string: &str) -> Vec<String> {
    vec![]
}

fn get_db(resource: &str) -> RwLock<Vec<Option<String>>> {
    return RwLock::new(vec![]);
}

fn create_resource_endpoints(app: &mut tide::Server<()>) -> () {
    // locker with Vec<Option<String>> as the database for the server
    // Map of map to hold all the databases?

    app.at("/:resource")
        .post(|mut req: Request<()>| async move {
            let resource = req.param("resource")?.to_owned();
            let json: String = req.body_string().await?;

            let id = match get_db(&resource).write() {
                Ok(mut writer) => {
                    writer.push(Some(json));
                    writer.len()
                }
                //TODO FIX
                Err(_) => return Ok(format!("Could not get the write lock on db")),
            };

            Ok(format!("{{id: {} }}", id))
        })
        .get(|req: Request<()>| async move {
            // format vector as json removing None
            Ok("")
        });

    app.at("/:resource/:id")
        .put(|mut req: Request<()>| async move {
            // check if exists
            let key = req.param("id")?;
            let json = req.body_string().await?;
            //save it
            Ok(format!("{{id: {} }}", 0))
        })
        .get(|req: Request<()>| async move {
            // format vector as json removing None

            let key = req.param("id")?;
            Ok("")
        });
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();

    for resource in resources_list("custom.json") {
        //set databases
    }

    create_resource_endpoints(&mut app);
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
