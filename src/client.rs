use gcp_auth;
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let authentication_manager = gcp_auth::init().await?;
    let token = authentication_manager
        .get_token(&["https://www.googleapis.com/auth/datastore"])
        .await?;

    let bearer = format!("Bearer {}", token.as_str());
    let header_value = tonic::metadata::MetadataValue::from_str(&bearer)?;

    let tls = ClientTlsConfig::new().domain_name(DOMAIN);

    let channel = Channel::from_static(URL).tls_config(tls)?.connect().await?;

    let mut client = FirestoreClient::with_interceptor(channel, move |mut req: Request<()>| {
        req.metadata_mut()
            .insert("authorization", header_value.clone());
        Ok(req)
    });

    let request = tonic::Request::new(GetDocumentRequest {
        name: "projects/projectmap-develop/databases/(default)/documents/Pins/04NkBxn906NOb7LpSzh9"
            .to_string(),
        ..Default::default()
    });

    let response = client.get_document(request).await?;

    println!("{:?}", response.get_ref());

    Ok(())
}
