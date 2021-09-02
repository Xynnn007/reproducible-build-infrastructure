use rbi_service::rbi_service_server::{RbiService, RbiServiceServer};
use rbi_service::{RbiQueryRequest, RbiQueryResponse, RbiAddRecordRequest, RbiAddRecordResponse};
use tonic::{transport::Server, Request, Response, Status};
use crate::cache::{KVStore};
use crate::rvps_handlers;

pub mod rbi_service {
    tonic::include_proto!("rbiservice");
}

#[derive(Debug)]
pub struct RbiServ {
    cache: Box<dyn KVStore + Send + Sync>,
    handler: rvps_handlers::RvpsHandler,
}

impl std::fmt::Debug for Box<dyn KVStore + Send + Sync> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Box")
    }
}

impl RbiServ {
    fn add(&self, id: &str, class: &str, file: &Vec<u8>) -> Result<String, Box<dyn std::error::Error>> {
        if self.cache.search(id.to_string()).is_ok() {
            return Err(Box::new(rvps_handlers::error::Error::new(rvps_handlers::error::Kind::IDduplicate)));
        }
        let res = (&self.handler).call(class, file)?;
        self.cache.insert(id.to_string(), res.clone())?;
        Ok(res)
    }
}

#[tonic::async_trait]
impl RbiService for RbiServ {
    async fn query(
        &self,
        request: Request<RbiQueryRequest>,
    ) -> Result<Response<RbiQueryResponse>, Status> {
        let id = request.into_inner().id;
        println!("[GRPC] Got a new query request. id: {}", id);

        let search_res = self.cache.search(id.clone());

        match search_res {
            Ok(s) => {
                println!("[GRPC] Query successeed. id: {}, sha256: {}", id, s);
                let res = RbiQueryResponse {
                    state: 0,
                    response: s.into_bytes(),
                };
                Ok(Response::new(res))
            }
            Err(e) => {
                println!("[GRPC] Query failed. id: {}", id);
                let res = RbiQueryResponse {
                    state: 1,
                    response: e.to_string().into_bytes(),
                };
                Ok(Response::new(res))
            }
        }
        
    }

    async fn add_record(
        &self,
        request: Request<RbiAddRecordRequest>,
    ) -> Result<Response<RbiAddRecordResponse>, Status> {
        let req =  request.into_inner();
        let id = req.id;
        let content = req.file;
        let class = req.class;
        println!("[GRPC] Got a new add-record request. id: {}, class: {}", id, class);
        match self.add(&id, &class, &content) {
            Ok(r) => {
                println!("[GRPC] Record added successfully. id: {}, sha256: {}", id, r);
                Ok(Response::new(RbiAddRecordResponse {
                    response: r.as_bytes().to_vec(),
                }))
            },
            Err(e) => {
                println!("[GRPC] Record failed to add. id: {}, err: {}", id, (*e).to_string());
                Ok(Response::new(RbiAddRecordResponse {
                    response: (*e).to_string().as_bytes().to_vec(),
                }))
            }
        }
    }
}

pub async fn server (addr: &str, cache: Box<dyn KVStore + Send + Sync>) -> Result<(), Box<dyn std::error::Error>> {
    let addr = addr.parse()?;

    let service = RbiServ {
        cache,
        handler : rvps_handlers::RvpsHandler::new(),
    };

    Server::builder()
        .add_service(RbiServiceServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}
