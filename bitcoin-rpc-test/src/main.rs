use bitcoincore_rpc::{Auth, Client, RpcApi};
use env_logger;
use log::info;
fn main() {
    env_logger::init();
    let rpc = Client::new(
        "http://8.219.145.96:6332",  // RPC endpoint Bitcoin Core
        Auth::UserPass("edricnguyen".into(), "Bigcoin@$2024".into()),  // RPC credentials
    ).unwrap();

     let info = rpc.get_blockchain_info().unwrap();
    println!("{:#?}", info);

    // let blockchain_info = rpc.get_blockchain_info().unwrap();
    // println!("{:#?}", blockchain_info);
}