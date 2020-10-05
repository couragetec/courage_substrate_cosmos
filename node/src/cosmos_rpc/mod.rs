mod types;

use jsonrpc_http_server::jsonrpc_core::{serde_json::json, IoHandler, Params};
use jsonrpc_http_server::ServerBuilder;
use node_template_runtime::cosmos_abci::ExtrinsicConstructionApi;
use node_template_runtime::opaque::Block;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::generic::BlockId;
use std::sync::Arc;

pub const DEFAULT_ABCI_RPC_URL: &str = "127.0.0.1:26657";

pub fn start_server(client: Arc<crate::service::FullClient>) {
    let mut io = IoHandler::new();

    // todo
    // Handle error response.
    io.add_method("abci_info", |_params: Params| async {
        let result = abci::get_abci_instance()
            .map_err(|_| "failed to setup connection")
            .unwrap()
            .info()
            .map_err(|err| println!("{}", err.to_string()) /* Ok(json!({
                "error": err.to_string(),
            }) */)
            .unwrap();
        let last_block_app_hash = result.get_last_block_app_hash();
        let last_block_height = result.get_last_block_height();
        println!("last block height: {:?}", last_block_height);
        println!("last block app hash: {:?}", last_block_app_hash);
        Ok(json!({
            "response": {
                "data": format!("{}", result.get_data()),
                "version": format!("{}", result.get_version()),
                "app_version": format!("{}", result.get_app_version())
            }
        }))
    });

    io.add_method("abci_query", |params: Params| async {
        let query_params: types::ABCIQueryParams = params.parse().unwrap();
        println!(
            "params path: {}, data: {}, height: {}, prove: {}",
            query_params.path, query_params.data, query_params.height, query_params.prove
        );
        let result = abci::get_abci_instance()
            .map_err(|_| "failed to setup connection")
            .unwrap()
            .query(
                query_params.path,
                hex::decode(query_params.data).expect("Decoding failed"),
                query_params.height.parse::<i64>().unwrap(),
                query_params.prove,
            )
            .map_err(|_| "query failed")
            .unwrap();

        // TODO: parse result.proof and if it is qual to None in the json proof field put null
        // TODO: if key len == 0 put null in the json key field
        Ok(json!({
            "response": {
                "log" : format!("{}", result.get_log()),
                "height" : format!("{}", result.get_height()),
                "proof" : null,
                "value" : format!("{}", base64::encode(result.get_value())),
                "key" : format!("{}", std::str::from_utf8(&result.get_key()).unwrap()),
                "index" : format!("{}", result.get_index()),
                "code" : format!("{}", result.get_code()),
            }
        }))
    });

    io.add_method("broadcast_tx_commit", move |params: Params| {
        let client = client.clone();
        async move {
            let params: types::ABCITxCommitParams = params.parse().unwrap();
            let tx_value = base64::decode(params.tx).unwrap();
            let result = abci::get_abci_instance()
                .map_err(|_| "failed to setup connection")
                .unwrap()
                .check_tx(tx_value.clone(), 0)
                .map_err(|_| "query failed")
                .unwrap();

            let info = client.info();
            let best_hash = info.best_hash;
            let best_height: u32 = info.best_number.into();
            let at = BlockId::<Block>::hash(best_hash);
            client
                .runtime_api()
                .sign_and_send_deliver_tx(&at, &tx_value)
                .ok();
            Ok(json!({
                "height": (best_height + 1).to_string(),
                "hash": "",
                "deliver_tx": {
                    "log": format!("{}", result.get_log()),
                    "data": format!("{}", base64::encode(result.get_data().clone())),
                    "code": format!("{}", result.get_code())
                },
                "check_tx": {
                    "log": format!("{}", result.get_log()),
                    "data": format!("{}", base64::encode(result.get_data())),
                    "code": format!("{}", result.get_code())
                }
            }))
        }
    });

    std::thread::spawn(move || {
        let server = ServerBuilder::new(io)
            .threads(3)
            .start_http(&DEFAULT_ABCI_RPC_URL.parse().unwrap())
            .unwrap();
        server.wait();
    });
}
