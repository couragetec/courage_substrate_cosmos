pub fn get_server_url() -> String {
    match std::env::var("ABCI_SERVER_URL") {
        Ok(val) => val,
        Err(_) => DEFAULT_ABCI_URL.to_owned(),
    }
}

pub const DEFAULT_ABCI_URL: &str = "tcp://localhost:26658";

pub const DEFAULT_ABCI_APP_STATE: &str = r#"{
    "auth": {
        "params": {
            "max_memo_characters": "256",
            "sig_verify_cost_ed25519": "590",
            "sig_verify_cost_secp256k1": "1000",
            "tx_sig_limit": "7",
            "tx_size_cost_per_byte": "10"
        }
    },
    "bank": {
        "balances": [],
        "denom_metadata": [],
        "params": {
            "default_send_enabled": true
        },
        "supply": []
    },
    "capability": {
        "index": "1",
        "owners": []
    },
    "crisis": {
        "constant_fee": {
            "amount": "1000",
            "denom": "stake"
        }
    },
    "distribution": {
        "delegator_starting_infos": [],
        "delegator_withdraw_infos": [],
        "fee_pool": {
            "community_pool": []
        },
        "outstanding_rewards": [],
        "params": {
            "base_proposer_reward": "0.010000000000000000",
            "bonus_proposer_reward": "0.040000000000000000",
            "community_tax": "0.020000000000000000",
            "withdraw_addr_enabled": true
        },
        "validator_accumulated_commissions": [],
        "validator_current_rewards": [],
        "validator_historical_rewards": [],
        "validator_slash_events": []
    },
    "evidence": {},
    "genutil": {
        "gentxs": []
    },
    "gov": {
        "deposit_params": {
            "max_deposit_period": "172800000000000",
            "min_deposit": [
                {
                    "amount": "10000000",
                    "denom": "stake"
                }
            ]
        },
        "deposits": null,
        "proposals": null,
        "starting_proposal_id": "1",
        "tally_params": {
            "quorum": "0.334000000000000000",
            "threshold": "0.500000000000000000",
            "veto": "0.334000000000000000"
        },
        "votes": null,
        "voting_params": {
            "voting_period": "172800000000000"
        }
    },
    "ibc": {
        "channel_genesis": {
            "ack_sequences": [],
            "acknowledgements": [],
            "channels": [],
            "commitments": [],
            "recv_sequences": [],
            "send_sequences": []
        },
        "client_genesis": {
            "clients": [],
            "clients_consensus": [],
            "create_localhost": true
        },
        "connection_genesis": {
            "client_connection_paths": [],
            "connections": []
        }
    },
    "mint": {
        "minter": {
            "annual_provisions": "0.000000000000000000",
            "inflation": "0.130000000000000000"
        },
        "params": {
            "blocks_per_year": "6311520",
            "goal_bonded": "0.670000000000000000",
            "inflation_max": "0.200000000000000000",
            "inflation_min": "0.070000000000000000",
            "inflation_rate_change": "0.130000000000000000",
            "mint_denom": "stake"
        }
    },
    "nameservice": {},
    "params": null,
    "slashing": {
        "missed_blocks": [],
        "params": {
            "downtime_jail_duration": "600000000000",
            "min_signed_per_window": "0.500000000000000000",
            "signed_blocks_window": "100",
            "slash_fraction_double_sign": "0.050000000000000000",
            "slash_fraction_downtime": "0.010000000000000000"
        },
        "signing_infos": []
    },
    "staking": {
        "delegations": null,
        "last_total_power": "0",
        "last_validator_powers": null,
        "params": {
            "bond_denom": "stake",
            "historical_entries": 100,
            "max_entries": 7,
            "max_validators": 100,
            "unbonding_time": "1814400000000000"
        },
        "redelegations": null,
        "unbonding_delegations": null,
        "validators": null
    },
    "transfer": {
        "port_id": "transfer"
    },
    "upgrade": {}
}
"#;
