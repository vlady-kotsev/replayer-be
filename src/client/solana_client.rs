use solana_client::nonblocking::rpc_client::RpcClient;
use solana_keypair::Address;

pub struct SolanaClient {
    rpc: RpcClient,
}

impl SolanaClient {
    pub fn new(rpc_url: String) -> SolanaClient {
        Self {
            rpc: RpcClient::new(rpc_url),
        }
    }

    pub async fn check_account_exists(&self, address: &Address) -> Option<Vec<u8>> {
        self.rpc.get_account_data(address).await.ok()
    }
}
