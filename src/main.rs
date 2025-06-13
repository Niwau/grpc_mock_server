tonic::include_proto!("locality");

use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};

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
        unimplemented!();
    }

    async fn create_locality(
        &self,
        _request: Request<CreateLocalityRequest>,
    ) -> Result<Response<CreateLocalityResponse>, Status> {
        unimplemented!();
    }

    async fn update_locality(
        &self,
        _request: Request<UpdateLocalityRequest>,
    ) -> Result<Response<UpdateLocalityResponse>, Status> {
        unimplemented!();
    }

    async fn delete_locality(
        &self,
        _request: Request<DeleteLocalityRequest>,
    ) -> Result<Response<DeleteLocalityResponse>, Status> {
        unimplemented!();
    }
}

#[tokio::main]
async fn main() {
    println!("Hello, world!");
}
