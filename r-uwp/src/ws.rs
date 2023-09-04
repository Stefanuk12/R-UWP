// Dependencies
use std::str::FromStr;
use actix::{Actor, StreamHandler};
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer, middleware::Logger};
use actix_web_actors::ws;

use crate::commands::{Command, CommandResponse, CommandError};

/// The IP to run the WSS on.
pub const IP: &str = "127.0.0.1";

/// Define HTTP actor
#[derive(Default)]
pub struct MyWs;
impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => {
                // Grab the job id
                let split: Vec<&str> = text.split("|").collect();
                let id = match split.first() {
                    Some(x) => x.to_string(),
                    None => return ctx.text(CommandError::BadlyFormattedCommand)
                };
    
                // Grab the command
                let command = match Command::from_str(&text[2..].to_string()) {
                    Ok(x) => x,
                    Err(e) => return ctx.text(e)
                };

                // Run the command
                match command.execute() {
                    Ok(x) => {
                        ctx.text(CommandResponse {
                            id,
                            data: x.unwrap_or_default()
                        })
                    },
                    Err(e) => ctx.text(e)
                }
            },
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}

/// Handler for GET / request, starts the websocket server.
async fn index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    ws::start(MyWs::default(), &req, stream)
}

/// The main function that starts the websocket server
pub async fn start(port: u16) -> std::io::Result<()> {
    log::info!("Starting websocket server on {}:{}", IP, port);
    HttpServer::new(|| 
        App::new()
            .wrap(Logger::default())
            .route("/", web::get().to(index))
    )
        .bind((IP, port))?
        .run()
        .await
}