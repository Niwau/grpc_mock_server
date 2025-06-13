tonic::include_proto!("locality");

pub mod proto {
    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("locality_descriptor");
}

use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status, transport::Server};

use crate::locality_service_server::LocalityService;

#[derive(Debug)]
struct LocalityServer;

#[tonic::async_trait]
impl LocalityService for LocalityServer {
    type GetLocalitiesStream = ReceiverStream<Result<Locality, Status>>;

    // Locality
    async fn get_localities(
        &self,
        _request: Request<GetLocalitiesRequest>,
    ) -> Result<Response<Self::GetLocalitiesStream>, Status> {
        let (tx, rx) = mpsc::channel(4);

        let locality = Locality {
            id: "1".into(),
            company_id: "2".into(),
            name: "Localidade".into(),
            address: "Endereço".into(),
            code: Some("Código".into()),
            cnpj: Some("Código".into()),
            meta: Option::default(),
            phone: Some("+5521992307122".into()),
            radius: Some(100.into()),
            region: Some("Região".into()),
            latitude: 1.into(),
            longitude: 1.into(),
            responsible: Some("Responsável".into()),
        };

        let localities = vec![
            locality.clone(),
            locality.clone(),
            locality.clone(),
            locality.clone(),
            locality.clone(),
        ];

        let locality_stream = ReceiverStream::new(rx);

        tokio::spawn(async move {
            for locality in localities {
                if tx.send(Ok(locality)).await.is_err() {
                    break;
                }
            }
            drop(tx);
        });

        Ok(Response::new(locality_stream))
    }

    async fn create_locality(
        &self,
        _request: Request<CreateLocalityRequest>,
    ) -> Result<Response<CreateLocalityResponse>, Status> {
        let response = CreateLocalityResponse {
            id: "1".to_string(),
        };
        Ok(Response::new(response))
    }

    async fn update_locality(
        &self,
        _request: Request<UpdateLocalityRequest>,
    ) -> Result<Response<UpdateLocalityResponse>, Status> {
        let response = UpdateLocalityResponse {
            id: "1".to_string(),
        };
        Ok(Response::new(response))
    }

    async fn delete_locality(
        &self,
        _request: Request<DeleteLocalityRequest>,
    ) -> Result<Response<DeleteLocalityResponse>, Status> {
        let response = DeleteLocalityResponse {
            id: "1".to_string(),
        };
        Ok(Response::new(response))
    }
}

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:5051".parse().unwrap();

    let locality_service = LocalityServer {};

    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(proto::FILE_DESCRIPTOR_SET)
        .build_v1alpha()
        .unwrap();

    println!("Server listening on {}", addr);

    Server::builder()
        .add_service(reflection_service)
        .add_service(locality_service_server::LocalityServiceServer::new(
            locality_service,
        ))
        .serve(addr)
        .await
        .unwrap();
}
