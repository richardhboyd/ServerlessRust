use lambda_runtime::{handler_fn, Context};
use serde::{Deserialize, Serialize};

pub type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

#[derive(Deserialize)]
struct Request {
    path: String,
}

#[derive(Serialize)]
struct Response {
    body: ResponseBody,
    // This should be a u32, but API Gateway actually expects a String that looking like an int for some reason.
    statusCode: String
}

#[derive(Serialize)]
struct ResponseBody {
    hello: String
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = handler_fn(my_handler);
    lambda_runtime::run(func).await?;
    Ok(())
}

pub(crate) async fn my_handler(event: Request, ctx: Context) -> Result<Response, Error> {

    let resp = Response {
        body: ResponseBody {hello: String::from("world")},
        statusCode: String::from("200")
    };

    Ok(resp)
}