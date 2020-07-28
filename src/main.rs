#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate lazy_static;

use std::sync::Arc;
use tokio::net::UnixListener;
use futures::stream::TryStreamExt;

pub mod rpc_yamori {
    tonic::include_proto!("rpc_yamori");
}
use rpc_yamori::yamori_server::YamoriServer;

mod settings;
use settings::Settings;

mod rpc;
use rpc::YamoriRPCServer;
use tonic::transport::Server;

mod util;

lazy_static! {
    static ref SETTINGS: Settings = Settings::new().map_err(|err| panic!("Something went wrong while reading the configuration. Aborting. : {:?}", err)).unwrap();
}


//
use tonic::{Request, Response, Status};
use rpc_yamori::yamori_server::Yamori;
use rpc_yamori::{NotifyInfoRequest, NotifyInfoReply};
use std::path::Path;


/*
#[derive(Debug, Default)]
pub struct YamoriRPCServer {
    hook_url: Arc<&'static str>,
}
*/

#[tonic::async_trait]
impl Yamori for YamoriRPCServer {
    async fn notify_info(&self, req: Request<NotifyInfoRequest>) -> Result<Response<NotifyInfoReply>, Status> {
        match send_info_message(self.hook_url.clone().as_ref(), &req.into_inner().text).await {
            Ok(_) => Ok(Response::new(NotifyInfoReply {})),
            Err(_) => Err(Status::aborted("Generic Error")),
        }
    }
}

async fn send_info_message(hook_url: &str, text: &String) -> Result<(), ()> {
    let response = reqwest::Client::new()
        .post(hook_url)
        .json(&serde_json::json!({
            "text" : text
        }))
        .send().await;

    match response {
        Ok(resp) => match resp.error_for_status() {
            Ok(_) => Ok(()),
            Err(err) => {
                println!("The Slack API server returned error: {:#?}", err.status());
                Err(())
            },
        },
        Err(err) => {
            println!("Something went wrong while sending the Slack API request: {:#?}", err);
            Err(())
        },
    }
}
//


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tokio::fs::create_dir_all(Path::new(&SETTINGS.rpc_socket).parent().unwrap()).await?;

    let mut uds = UnixListener::bind(&SETTINGS.rpc_socket)?;
    let server = YamoriRPCServer { hook_url : Arc::new(&SETTINGS.webhook_url) };

    Server::builder()
        .add_service(YamoriServer::new(server))
        .serve_with_incoming(uds.incoming().map_ok(util::UnixStream))
        .await?;

    Ok(())
}
