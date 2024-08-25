use crate::controller::dto::{PublishRequestBodyDTO, PublishResponseDTO};
use crate::controller::state::Clients;
use actix_web::{post, web, Error, HttpResponse, Responder};
use serde_json::Value;
use tokio_stream::wrappers::UnboundedReceiverStream;
use tokio_stream::StreamExt;

#[post("/subscribe/{topic}")]
pub async fn subscribe(clients: web::Data<Clients>, path: web::Path<(String,)>) -> impl Responder {
    let topic = path.into_inner().0;
    let (tx, rx) = tokio::sync::mpsc::unbounded_channel::<Value>();
    {
        let mut clients = clients.lock().unwrap();
        clients.entry(topic).or_insert(vec![]).push(tx);
    };
    let stream = UnboundedReceiverStream::new(rx)
        .map(|msg| Ok::<_, Error>(web::Bytes::from(format!("data: {}\n\n", msg.to_string()))));

    HttpResponse::Ok()
        .insert_header(("Content-Type", "text/event-stream"))
        .insert_header(("Cache-Control", "no-cache"))
        .insert_header(("Connection", "keep-alive"))
        .streaming(stream)
}

#[post("/publish/{topic}")]
pub async fn publish(
    clients: web::Data<Clients>,
    path: web::Path<(String,)>,
    body: web::Json<PublishRequestBodyDTO>,
) -> impl Responder {
    let topic = path.into_inner().0;

    let mut clients = clients.lock().unwrap();
    if let Some(list_clients) = clients.get_mut(&topic) {
        // remove dead clients from the list
        let filtered_clients = list_clients
            .iter()
            .filter(|tx| tx.send(body.message.clone()).is_ok())
            .cloned()
            .collect::<Vec<_>>();
        // update the list of clients
        *list_clients = filtered_clients;
        HttpResponse::Ok().json(PublishResponseDTO {
            count: list_clients.len(),
        })
    } else {
        HttpResponse::Ok().json(PublishResponseDTO { count: 0 })
    }
}
