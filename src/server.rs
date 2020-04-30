use std::env;
use std::path::PathBuf;
use tokio::sync::Mutex;

use sbert_rs::Error;

use tonic::{transport::Server, Request, Response, Status};

use service::embedder_server::{Embedder, EmbedderServer};

pub mod service {
    tonic::include_proto!("services.embedder");
}

pub struct SBert {
    model: sbert_rs::SBert,
}

impl SBert {
    pub fn new() -> Result<Self, Error> {
        let mut home: PathBuf = env::current_dir().unwrap();
        home.push("models");
        home.push("distiluse-base-multilingual-cased");

        println!("Loading sbert_rs ...");
        let model = sbert_rs::SBert::new(home).unwrap();

        Ok(SBert { model })
    }
}

unsafe impl Send for SBert {}

struct SBertSync(Mutex<SBert>);

#[tonic::async_trait]
impl Embedder for SBertSync {
    async fn vectorize(
        &self,
        query: Request<service::Query>,
    ) -> Result<Response<service::Response>, Status> {
        let texts = Vec::from(query.into_inner().texts);

        println!("Encoding {:?}", texts.len());

        let output = self
            .0
            .lock()
            .await
            .model
            .encode(texts.as_slice())
            .unwrap();

        let r = Vec::<Vec<f32>>::from(output);
        let vecs = r
            .iter()
            .map(|v| service::Vector { v: v.clone() })
            .collect::<Vec<_>>();

        let reply = service::Response { vecs: vecs };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50050".parse()?;
    println!("Starting SBert server on {}", addr);
    let embedder = SBert::new()?;
    let embedder = SBertSync(Mutex::new(embedder));

    Server::builder()
        .add_service(EmbedderServer::new(embedder))
        .serve(addr)
        .await?;

    Ok(())
}