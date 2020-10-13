use gcp_auth;
use std::time::SystemTime;
use tonic::{
    transport::{Channel, ClientTlsConfig},
    Request,
};

pub mod google {
    pub mod firestore {
        pub mod v1 {
            tonic::include_proto!("google.firestore.v1");
        }
    }
    pub mod rpc {
        tonic::include_proto!("google.rpc");
    }
    pub mod r#type {
        tonic::include_proto!("google.r#type");
    }
}

use google::firestore::v1::firestore_client::FirestoreClient;
use google::firestore::v1::GetDocumentRequest;

const URL: &str = "https://firestore.googleapis.com";
const DOMAIN: &str = "firestore.googleapis.com";

struct Stopwatch {
    name: String,
    started_at: SystemTime,
}

impl Stopwatch {
    fn new(name: &str) -> Stopwatch {
        Stopwatch {
            name: name.to_string(),
            started_at: SystemTime::now(),
        }
    }

    fn print(&self) {
        let elapsed_millis = self.started_at.elapsed().expect("unexpected").as_millis() as f64;
        println!("{}: {}", self.name, elapsed_millis / 1000 as f64);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let document_paths = [
        "Pins/04NkBxn906NOb7LpSzh9",
        "Users/3CRKoFAOygcl7xK60WamxvcrjYY2",
    ];

    env_logger::init();
    let authentication_manager = gcp_auth::init().await?;
    let token = authentication_manager
        .get_token(&["https://www.googleapis.com/auth/datastore"])
        .await?;

    let create_channel_watch = Stopwatch::new("create channel");
    let bearer = format!("Bearer {}", token.as_str());
    let header_value = tonic::metadata::MetadataValue::from_str(&bearer)?;
    let tls = ClientTlsConfig::new().domain_name(DOMAIN);
    let channel = Channel::from_static(URL).tls_config(tls)?.connect().await?;
    let mut client = FirestoreClient::with_interceptor(channel, move |mut req: Request<()>| {
        req.metadata_mut()
            .insert("authorization", header_value.clone());
        Ok(req)
    });
    create_channel_watch.print();

    for document_path in document_paths.iter() {
        let request_watch = Stopwatch::new("request");
        let request = tonic::Request::new(GetDocumentRequest {
            name: format!(
                "projects/projectmap-develop/databases/(default)/documents/{}",
                document_path
            )
            .to_string(),
            ..Default::default()
        });
        let response = client.get_document(request).await?;
        request_watch.print();
        println!("{:?}", response.get_ref());
    }

    Ok(())
}
