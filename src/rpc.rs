use std::sync::Arc;
use tonic::{Request, Response, Status};
use reqwest;
use serde_json;

pub mod rpc_yamori {
    tonic::include_proto!("rpc_yamori");
}
//use tonic::{Request, Response, Status};
use rpc_yamori::yamori_server::Yamori;
use rpc_yamori::{NotifyInfoRequest, NotifyInfoReply};
#[derive(Debug, Default)]
pub struct YamoriRPCServer {
    pub(crate) hook_url: Arc<&'static str>,
}

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
