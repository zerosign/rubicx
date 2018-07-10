extern crate futures;
extern crate rmpv;
extern crate tokio;
extern crate tower_service;

use futures::{
    future::Executor,
    sync::{mpsc, oneshot},
    Future,
};
use std::collections::HashMap;
use tower_service::{NewService, Service};

pub enum Request {
    #[deerive(Debug)]
    Index { data: rmpv::Value },
}

///
/// https://www.elastic.co/guide/en/elasticsearch/reference/current/docs-index_.html
///
pub enum Response {
    #[derive(Debug)]
    IndexStatus {
        space_id: u64,
        segment_id: u64,
        grid_id: u64,
    },
}

pub enum Error {}

type Sender = mpsc::Sender<(Request, oneshot::Receiver<Response>)>;

pub struct Indexer {
    tx: Sender,
}

pub struct IndexerFuture;

impl Future for IndexerFuture {}

impl Service for Indexer {
    type Request = Request;
    type Response = Response;
    type Error = Error;
    type Future = IndexerFuture;

    fn poll_ready(&mut self) -> Poll<(), Self::Error> {}

    fn call(&mut self, request: Self::Request) -> Self::ResponseFuture {}
}

pub struct Model;

pub struct Oracle {
    indexer: Indexer,
    model: Model,
}

impl Service for Oracle {
    type Request = Request;
}

fn main() {
    println!("Hello, world!");
}
