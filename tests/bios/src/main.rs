use rbi_service::rbi_service_client::RbiServiceClient;
use rbi_service::{RbiQueryRequest, RbiAddRecordRequest};
use std::fs::File;
use std::io::Read;

pub mod rbi_service {
    tonic::include_proto!("rbiservice");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = RbiServiceClient::connect("http://[::1]:7654").await?;
    let filename = "testfile/in_toto.tar.gz".to_string();
    let mut f = File::open(&filename)?;
    let metadata = f.metadata().expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");

    let add_req = tonic::Request::new(RbiAddRecordRequest {
        class : "in-toto".into(),
        id: "123".into(),
        file: buffer,
    });
    println!("Send record add request...");
    let add_res = client.add_record(add_req).await?;
    let res = String::from_utf8(add_res.into_inner().response)?;
    println!("RESPONSE={:?}", res);

    let query_req = tonic::Request::new(RbiQueryRequest {
        id: "123".into(),
    });

    println!("Send query request...");
    let query_res = client.query(query_req).await?;
    let res = String::from_utf8(query_res.into_inner().response)?;
    println!("RESPONSE={:?}", res);

    Ok(())
}