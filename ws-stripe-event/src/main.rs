use axum::{
    async_trait,
    body::Body,
    extract::FromRequest,
    http::{Request, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, post},
    Router,
};
use stripe::{Event, EventObject, EventType};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/stripe_webhooks", post(handle_webhook));

    let endpoint = "127.0.0.1:4242";
    let listener = tokio::net::TcpListener::bind(endpoint).await.unwrap();
    println!("Started server on {}", endpoint);
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}

const WEBHOOK_SIGNING_SECRET: &str = "whsec_ccd6fd3a1244c0420f295b07d1e93ae2296cb9ec341360d7b1291204d4746c58";

struct StripeEvent(Event);
#[async_trait]
impl<S> FromRequest<S> for StripeEvent
where
    String: FromRequest<S>,
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request(req: Request<Body>, state: &S) -> Result<Self, Self::Rejection> {
        let signature = if let Some(sig) = req.headers().get("stripe-signature") {
            sig.to_owned()
        } else {
            println!("Fail to check signature");
            // return Err(StatusCode::BAD_REQUEST.into_response());
            return Err(StatusCode::INTERNAL_SERVER_ERROR.into_response());
        };

        let payload =
            String::from_request(req, state).await.map_err(IntoResponse::into_response)?;
        // println!("Received payload: {:?}", &payload);

        Ok(Self(
            stripe::Webhook::construct_event(&payload, signature.to_str().unwrap(), WEBHOOK_SIGNING_SECRET)
                .map_err(|_| {
                    println!("Fail to construct_event");
                    return StatusCode::INTERNAL_SERVER_ERROR.into_response();
                })?,
        ))
    }
}

async fn handle_webhook(StripeEvent(event): StripeEvent) {
    println!("Received webhook with id: {:?}", event.id);
    // println!("{:?}", event.data);
    match event.type_ {
        EventType::CheckoutSessionCompleted => {
            if let EventObject::CheckoutSession(session) = event.data.object {
                println!("Received checkout session completed webhook with id: {:?}", session.id);
            }
        }
        EventType::AccountUpdated => {
            if let EventObject::Account(account) = event.data.object {
                println!("Received account updated webhook for account: {:?}", account.id);
            }
        }
        _ => println!("Unknown event encountered in webhook: {:?}", event.type_),
    }
}
