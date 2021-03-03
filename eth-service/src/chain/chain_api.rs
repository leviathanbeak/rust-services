use std::time::Duration;

use secp256k1::{PublicKey, Secp256k1, SecretKey};
use web3::types::{
    Address, Bytes, CallRequest, SignedTransaction, TransactionParameters, H256, U256,
};
use web3::{transports::Http, Web3};
#[derive(Debug, Clone)]
pub enum ChainApiError {
    ChainRequestFailed(String),
    PrivateKeyParseFailed(String)
}

impl From<web3::Error> for ChainApiError {
    fn from(_error: web3::Error) -> Self {
        Self::ChainRequestFailed("Requesting Data from the chain failed".to_owned())
    }
}

impl From<secp256k1::Error> for ChainApiError {
    fn from(_error: secp256k1::Error) -> Self {
        Self::PrivateKeyParseFailed("Parsing of private key failed".to_owned())
    }
}
pub struct ChainApi {}

impl ChainApi {
    pub async fn get_nonce(web3: &Web3<Http>, address: Address) -> Result<U256, ChainApiError> {
        let nonce = web3.eth().transaction_count(address, None).await?;
        Ok(nonce)
    }

    pub async fn build_tx(
        web3: &Web3<Http>,
        contract_address: &str,
        private_key: &str,
        chain_id: u64,
        data: Vec<u8>,
    ) -> Result<TransactionParameters, ChainApiError> {
        let contract_address: Address = Self::convert_to_address(contract_address);
        let contract_address = Some(contract_address);

        let address = Self::get_public_address(private_key);
        let nonce = Self::get_nonce(web3, address).await?;
        let nonce = Some(nonce);

        let gas = U256::from(55000);
        let gas_price = Some(U256::from(0));
        let wei_value = U256::from(0);
        let data = Bytes::from(data);
        let chain_id = Some(chain_id);

        let tx_params = TransactionParameters {
            nonce,
            to: contract_address,
            gas,
            gas_price,
            value: wei_value,
            data,
            chain_id,
        };

        Ok(tx_params)
    }

    pub async fn sign_tx(
        web3: &Web3<Http>,
        tx_params: TransactionParameters,
        private_key: &str,
    ) -> Result<SignedTransaction, ChainApiError> {
        let private_key = Self::format_key(private_key)?;
        let signed_tx = web3
            .accounts()
            .sign_transaction(tx_params, &private_key)
            .await?;

        Ok(signed_tx)
    }

    pub async fn send_transaction(
        web3: &Web3<Http>,
        signed_tx: SignedTransaction,
    ) -> Result<H256, ChainApiError> {
        let hash = web3
            .eth()
            .send_raw_transaction(signed_tx.raw_transaction)
            .await?;
        Ok(hash)
    }

    pub async fn send_transaction_with_confirmation(
        web3: &Web3<Http>,
        signed_tx: SignedTransaction,
    ) -> Result<H256, ChainApiError> {
        let result = web3
            .send_raw_transaction_with_confirmation(
                Bytes::from(signed_tx.raw_transaction),
                Duration::from_secs(1),
                0,
            )
            .await?;

        Ok(result.transaction_hash)
    }

    pub fn convert_to_address(address: &str) -> Address {
        let address = &address[2..];
        address.parse().unwrap()
    }

    pub async fn get_balances(
        web3: &Web3<Http>,
        contract_address: &str,
        data: Vec<u8>,
    ) -> Result<Bytes, ChainApiError> {
        let contract_address = Self::convert_to_address(contract_address);
        let data = Bytes::from(data);
        let call_req = Self::build_balances_call_request(contract_address, data);

        let balances = web3.eth().call(call_req, None).await?;
        Ok(balances)
    }

    fn build_balances_call_request(contract_address: Address, data: Bytes) -> CallRequest {
        CallRequest {
            from: None,
            to: Some(contract_address),
            gas: None,
            gas_price: None,
            value: None,
            data: Some(data),
        }
    }

    fn format_key(private_key: &str) -> Result<SecretKey, ChainApiError> {
        let private_key = &private_key[2..];
        let secret_key = private_key.parse()?;
        Ok(secret_key)
    }

    fn get_public_address(private_key: &str) -> Address {
        let key = Self::format_key(private_key).unwrap();
        let secp = Secp256k1::signing_only();
        let public_key = PublicKey::from_secret_key(&secp, &key);
        Self::public_key_address(&public_key)
    }

    fn public_key_address(public_key: &PublicKey) -> Address {
        let public_key = public_key.serialize_uncompressed();
        let hash = Self::keccak256(&public_key[1..]);
        Address::from_slice(&hash[12..])
    }

    fn keccak256(bytes: &[u8]) -> [u8; 32] {
        use tiny_keccak::{Hasher, Keccak};
        let mut output = [0u8; 32];
        let mut hasher = Keccak::v256();
        hasher.update(bytes);
        hasher.finalize(&mut output);
        output
    }
}
