// Copyright 2015-2020 Parity Technologies (UK) Ltd.
// This file is part of OpenEthereum.

// OpenEthereum is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// OpenEthereum is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with OpenEthereum.  If not, see <http://www.gnu.org/licenses/>.

//! Spec params deserialization.

use crate::{
    bytes::Bytes,
    hash::{Address, H256},
    uint::{self, Uint},
};

/// Spec params.
#[derive(Debug, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct Params {
    /// Account start nonce, defaults to 0.
    pub account_start_nonce: Option<Uint>,
    /// Maximum size of extra data.
    pub maximum_extra_data_size: Uint,
    /// Minimum gas limit.
    pub min_gas_limit: Uint,

    /// Network id.
    #[serde(rename = "networkID")]
    pub network_id: Uint,
    /// Chain id.
    #[serde(rename = "chainID")]
    pub chain_id: Option<Uint>,

    /// Name of the main ("eth") subprotocol.
    pub subprotocol_name: Option<String>,

    /// Option fork block number to check.
    pub fork_block: Option<Uint>,
    /// Expected fork block hash.
    #[serde(rename = "forkCanonHash")]
    pub fork_hash: Option<H256>,

    /// See main EthashParams docs.
    pub eip150_transition: Option<Uint>,

    /// See main EthashParams docs.
    pub eip160_transition: Option<Uint>,

    /// See main EthashParams docs.
    pub eip161abc_transition: Option<Uint>,
    /// See main EthashParams docs.
    pub eip161d_transition: Option<Uint>,

    /// See `CommonParams` docs.
    pub eip98_transition: Option<Uint>,
    /// See `CommonParams` docs.
    pub eip155_transition: Option<Uint>,
    /// See `CommonParams` docs.
    pub validate_chain_id_transition: Option<Uint>,
    /// See `CommonParams` docs.
    pub validate_receipts_transition: Option<Uint>,
    /// From which block apply validator set transition fix.
    pub fix_validator_set_transition: Option<Uint>,
    /// See `CommonParams` docs.
    pub eip140_transition: Option<Uint>,
    /// See `CommonParams` docs.
    pub eip210_transition: Option<Uint>,
    /// See `CommonParams` docs.
    pub eip210_contract_address: Option<Address>,
    /// See `CommonParams` docs.
    pub eip210_contract_code: Option<Bytes>,
    /// See `CommonParams` docs.
    pub eip210_contract_gas: Option<Uint>,
    /// See `CommonParams` docs.
    pub eip211_transition: Option<Uint>,
    /// See `CommonParams` docs.
    pub eip145_transition: Option<Uint>,
    /// See `CommonParams` docs.
    pub eip214_transition: Option<Uint>,
    /// See `CommonParams` docs.
    pub eip658_transition: Option<Uint>,
    /// See `CommonParams` docs.
    pub eip1052_transition: Option<Uint>,
    /// See `CommonParams` docs.
    pub eip1283_transition: Option<Uint>,
    /// See `CommonParams` docs.
    pub eip1283_disable_transition: Option<Uint>,
    /// See `CommonParams` docs.
    pub eip1283_reenable_transition: Option<Uint>,
    /// See `CommonParams` docs.
    pub eip1014_transition: Option<Uint>,
    /// See `CommonParams` docs.
    pub eip1706_transition: Option<Uint>,
    /// See `CommonParams` docs.
    pub eip1344_transition: Option<Uint>,
    /// See `CommonParams` docs.
    pub eip1884_transition: Option<Uint>,
    /// See `CommonParams` docs.
    pub eip2028_transition: Option<Uint>,
    /// See `CommonParams` docs.
    pub eip2315_transition: Option<Uint>,
    /// See `CommonParams` docs.
    pub eip2929_transition: Option<Uint>,
    /// See `CommonParams` docs.
    pub eip2930_transition: Option<Uint>,
    /// See `CommonParams` docs.
    pub eip1559_transition: Option<Uint>,
    /// See `CommonParams` docs.
    pub eip3198_transition: Option<Uint>,
    /// See `CommonParams` docs.
    pub eip3529_transition: Option<Uint>,
    /// See `CommonParams` docs.
    pub eip3541_transition: Option<Uint>,
    /// See `CommonParams` docs.
    pub eip3607_transition: Option<Uint>,
    /// See `CommonParams` docs.
    pub dust_protection_transition: Option<Uint>,
    /// See `CommonParams` docs.
    pub nonce_cap_increment: Option<Uint>,
    /// See `CommonParams` docs.
    pub remove_dust_contracts: Option<bool>,
    /// See `CommonParams` docs.
    #[serde(deserialize_with = "uint::validate_non_zero")]
    pub gas_limit_bound_divisor: Uint,
    /// See `CommonParams` docs.
    pub registrar: Option<Address>,
    /// Apply reward flag
    pub apply_reward: Option<bool>,
    /// Node permission contract address.
    pub node_permission_contract: Option<Address>,
    /// See main EthashParams docs.
    pub max_code_size: Option<Uint>,
    /// Maximum size of transaction RLP payload.
    pub max_transaction_size: Option<Uint>,
    /// See main EthashParams docs.
    pub max_code_size_transition: Option<Uint>,
    /// Transaction permission contract address.
    pub transaction_permission_contract: Option<Address>,
    /// Block at which the transaction permission contract should start being used.
    pub transaction_permission_contract_transition: Option<Uint>,
    /// Wasm activation block height, if not activated from start
    pub wasm_activation_transition: Option<Uint>,
    /// Wasm deactivation block height, if activated.
    pub wasm_disable_transition: Option<Uint>,
    /// KIP4 activiation block height.
    pub kip4_transition: Option<Uint>,
    /// KIP6 activiation block height.
    pub kip6_transition: Option<Uint>,
    /// Base fee max change denominator
    pub eip1559_base_fee_max_change_denominator: Option<Uint>,
    /// Elasticity multiplier
    pub eip1559_elasticity_multiplier: Option<Uint>,
    /// Default value for the block base fee
    pub eip1559_base_fee_initial_value: Option<Uint>,
    /// Min value for the block base fee.
    pub eip1559_base_fee_min_value: Option<Uint>,
    /// Block at which the min value for the base fee starts to be used.
    pub eip1559_base_fee_min_value_transition: Option<Uint>,
    /// Address where EIP-1559 burnt fee will be accrued to.
    pub eip1559_fee_collector: Option<Address>,
    /// Block at which the fee collector should start being used.
    pub eip1559_fee_collector_transition: Option<Uint>,
    /// Block at which zero gas price transactions start being checked with Certifier contract.
    pub validate_service_transactions_transition: Option<Uint>,
}

#[cfg(test)]
mod tests {
    use crate::{spec::params::Params, uint::Uint};
    use ethereum_types::U256;
    use serde_json;

    #[test]
    fn params_deserialization() {
        let s = r#"{
			"maximumExtraDataSize": "0x20",
			"networkID" : "0x1",
			"chainID" : "0x15",
			"subprotocolName" : "exp",
			"minGasLimit": "0x1388",
			"accountStartNonce": "0x01",
			"gasLimitBoundDivisor": "0x20",
			"maxCodeSize": "0x1000",
			"wasmActivationTransition": "0x1010",
            "wasmDisableTransition": "0x2010"
		}"#;

        let deserialized: Params = serde_json::from_str(s).unwrap();
        assert_eq!(deserialized.maximum_extra_data_size, Uint(U256::from(0x20)));
        assert_eq!(deserialized.network_id, Uint(U256::from(0x1)));
        assert_eq!(deserialized.chain_id, Some(Uint(U256::from(0x15))));
        assert_eq!(deserialized.subprotocol_name, Some("exp".to_owned()));
        assert_eq!(deserialized.min_gas_limit, Uint(U256::from(0x1388)));
        assert_eq!(
            deserialized.account_start_nonce,
            Some(Uint(U256::from(0x01)))
        );
        assert_eq!(deserialized.gas_limit_bound_divisor, Uint(U256::from(0x20)));
        assert_eq!(deserialized.max_code_size, Some(Uint(U256::from(0x1000))));
        assert_eq!(
            deserialized.wasm_activation_transition,
            Some(Uint(U256::from(0x1010)))
        );
        assert_eq!(
            deserialized.wasm_disable_transition,
            Some(Uint(U256::from(0x2010)))
        );
    }

    #[test]
    #[should_panic(expected = "a non-zero value")]
    fn test_zero_value_divisor() {
        let s = r#"{
			"maximumExtraDataSize": "0x20",
			"networkID" : "0x1",
			"chainID" : "0x15",
			"subprotocolName" : "exp",
			"minGasLimit": "0x1388",
			"accountStartNonce": "0x01",
			"gasLimitBoundDivisor": "0x0",
			"maxCodeSize": "0x1000"
		}"#;

        let _deserialized: Params = serde_json::from_str(s).unwrap();
    }
}
