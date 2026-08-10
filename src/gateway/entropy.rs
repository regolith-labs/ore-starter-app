use entropy_types::response::GetSeedResponse;
use steel::Pubkey;

use crate::gateway::GatewayError;

use super::{GatewayResult, Rpc};

const ENTROPY_API_URL: &str = "https://entropy-api.onrender.com";

pub trait EntropyGateway {
    async fn get_seed(&self, var: Pubkey, samples: Option<u64>) -> GatewayResult<GetSeedResponse>;
}

impl<R: Rpc> EntropyGateway for R {
    async fn get_seed(&self, var: Pubkey, samples: Option<u64>) -> GatewayResult<GetSeedResponse> {
        let url = if let Some(samples) = samples {
            format!("{}/var/{}/seed?samples={}", ENTROPY_API_URL, var, samples)
        } else {
            format!("{}/var/{}/seed", ENTROPY_API_URL, var)
        };
        log::info!("get_seed with samples: {:?}", samples);
        let client = reqwest::Client::new();
        client
            .get(url)
            .send()
            .await?
            .json::<GetSeedResponse>()
            .await
            .map_err(|_| GatewayError::RequestFailed)
    }
}
