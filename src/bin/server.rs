use log::{error, info};

use futures::{future, Future, Stream};
use hyper::{Body, Request, Response, Server};
use tokio;

use bulletproof_aggregator::Point;

fn main() {
    pretty_env_logger::init();

    tokio::run(future::lazy(|| {
        let addr = ([127, 0, 0, 1], 3000).into();
        info!("listening on addr {}", addr);

        Server::bind(&addr)
            .serve(move || {
                hyper::service::service_fn(move |req: Request<Body>| {
                    req.into_body()
                        .concat2()
                        .map(|body| {
                            info!("got body: {:?}", body);
                            let point: Result<Point, _> = serde_cbor::from_slice(&body.as_ref());
                            match point {
                                Ok(point) => {
                                    let sum = point.x + point.y;
                                    Response::new(Body::from(format!("here is your sum {}", sum)))
                                }
                                Err(_) => Response::new(Body::from("bad req")),
                            }
                        })
                })
            })
            .map_err(|e| error!("server error {}", e))
    }));
}
