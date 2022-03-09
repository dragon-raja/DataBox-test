use candid::{CandidType, Decode, Encode};
use ic_agent::agent::http_transport::ReqwestHttpReplicaV2Transport;
use ic_agent::Agent;
use ic_types::Principal;
use ring::digest::digest;
use serde::Deserialize;

#[derive(CandidType, Deserialize, Debug)]
struct thumb_nail {
    aes_pub_key: Option<String>,
    file_key: String,
    file_name: String,
    image: Vec<u8>,
    file_extension: String,
}

#[derive(CandidType, Deserialize, Debug)]
struct Chunk {
    digest: Vec<u8>,
    data: Vec<u8>,
}

#[derive(CandidType, Deserialize, Debug)]
struct segment {
    aes_pub_key: Option<String>,
    file_key: String,
    file_name: String,
    file_extension: String,
    chunk: Chunk,
    chunk_number: u128,
    order: u128,
    total_size: u128,
}

#[derive(CandidType, Deserialize, Debug)]
enum PUT {
    thumb_nail(thumb_nail),
    segment(segment),
}

#[derive(CandidType, Deserialize, Debug)]
struct SharedFilePut {
    file_name: String,
    description: String,
    file_extension: String,
    isPublic: bool,
    file_key: String,
    other: Principal,
}

#[derive(CandidType, Deserialize, Debug)]
struct AssetExt {
    aes_pub_key: Option<String>,
    file_name: String,
    bucket_id: Principal,
    file_key: String,
    total_size: u128,
    file_extension: String,
    upload_status: bool,
    need_query_times: u128,
}

#[derive(CandidType, Deserialize, Debug)]
enum DataErr {
    MemoryInsufficient,
    PermissionDenied,
    DefaultKeyNotSet,
    DeviceNotExist,
    UserAccessErr,
    BlobSizeError,
    SharedNotSet,
    SharedRepeat,
    ShareRepeat,
    FileKeyErr,
    FilePublic,
    FlagErr,
}

#[derive(CandidType, Deserialize, Debug)]
enum FilePut {
    PlainFilePut(PUT),
    EncryptFilePut(PUT),
    SharedFilePut(SharedFilePut),
}

#[derive(CandidType, Deserialize, Debug)]
enum FileExt {
    PlainFileExt(AssetExt),
    EncryptFileExt(AssetExt),
    SharedFileExt(SharedFilePut),
}

#[derive(CandidType, Deserialize, Debug)]
enum R {
    ok(FileExt),
    err(DataErr),
}

#[tokio::main]
async fn main() {
    let url = "http://127.0.0.1:8004".to_string();
    let transport = ReqwestHttpReplicaV2Transport::create(url).expect("transport error");
    let waiter1 = garcon::Delay::builder()
        .throttle(std::time::Duration::from_millis(500))
        .timeout(std::time::Duration::from_secs(60 * 5))
        .build();
    let waiter2 = garcon::Delay::builder()
        .throttle(std::time::Duration::from_millis(500))
        .timeout(std::time::Duration::from_secs(60 * 5))
        .build();
    let waiter3 = garcon::Delay::builder()
        .throttle(std::time::Duration::from_millis(500))
        .timeout(std::time::Duration::from_secs(60 * 5))
        .build();
    let agent = Agent::builder()
        .with_transport(transport)
        .build()
        .expect("build agent error");
    let _ = agent.fetch_root_key().await;
    
    let p1 = FilePut::PlainFilePut(PUT::segment(segment {
        aes_pub_key: None,
        file_key: String::from("key332"),
        file_name: String::from("test332"),
        file_extension: String::from("t"),
        chunk: Chunk {
            digest: vec![0x00u8],
            data: vec![0x00u8; 1992295],
        },
        chunk_number: 3,
        order: 0,
        total_size: 4089446,
    }));
    let p2 = FilePut::PlainFilePut(PUT::segment(segment {
        aes_pub_key: None,
        file_key: String::from("key332"),
        file_name: String::from("test332"),
        file_extension: String::from("t"),
        chunk: Chunk {
            digest: vec![0x00u8],
            data: vec![0x00u8; 1992295],
        },
        chunk_number: 3,
        order: 1,
        total_size: 4089446,
    }));
    let p3 = FilePut::PlainFilePut(PUT::segment(segment {
        aes_pub_key: None,
        file_key: String::from("key332"),
        file_name: String::from("test332"),
        file_extension: String::from("t"),
        chunk: Chunk {
            digest: vec![0x00u8],
            data: vec![0x00u8; 104856],
        },
        chunk_number: 3,
        order: 2,
        total_size: 4089446,
    }));
    let response1 = agent
        .update(
            &Principal::from_text("ai7t5-aibaq-aaaaa-aaaaa-c").unwrap(),
            "put",
        )
        .with_arg(Encode!(&p1).unwrap())
        .call_and_wait(waiter1)
        .await;
    let response2 = agent
        .update(
            &Principal::from_text("ai7t5-aibaq-aaaaa-aaaaa-c").unwrap(),
            "put",
        )
        .with_arg(Encode!(&p2).unwrap())
        .call_and_wait(waiter2)
        .await;
    let response3 = agent
        .update(
            &Principal::from_text("ai7t5-aibaq-aaaaa-aaaaa-c").unwrap(),
            "put",
        )
        .with_arg(Encode!(&p3).unwrap())
        .call_and_wait(waiter3)
        .await;
    match response1 {
        Ok(res) => {
            println!("send successfully");
            println!("{:?}", Decode!(&res, R).unwrap())
        }
        Err(e) => {
            println!("agent error : {:?}", e);
        }
    }
    match response2 {
        Ok(res) => {
            println!("send successfully");
            println!("{:?}", Decode!(&res, R).unwrap())
        }
        Err(e) => {
            println!("agent error : {:?}", e);
        }
    }
    match response3 {
        Ok(res) => {
            println!("send successfully");
            println!("{:?}", Decode!(&res, R).unwrap())
        }
        Err(e) => {
            println!("agent error : {:?}", e);
        }
    }
}