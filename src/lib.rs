mod errors;

use near_contract_standards::fungible_token::{
    metadata::{FungibleTokenMetadata, FungibleTokenMetadataProvider, FT_METADATA_SPEC},
    FungibleToken,
};
#[cfg(test)]
use near_sdk::serde::Deserialize;
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    collections::LookupMap,
    env,
    json_types::{ValidAccountId, U128},
    log, near_bindgen, AccountId, Balance, PanicOnDefault, PromiseOrValue,
};

near_sdk::setup_alloc!();

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    token: FungibleToken,
    owner: AccountId,
    dao: AccountId,
    pending_ft_rewards: LookupMap<AccountId, Balance>,
}

#[derive(BorshDeserialize, BorshSerialize)]
#[cfg_attr(test, derive(Deserialize))]
#[cfg_attr(test, serde(crate = "near_sdk::serde"))]
pub struct AirdropRewards(Vec<AirdropReward>);

#[derive(BorshDeserialize, BorshSerialize, Clone)]
#[cfg_attr(test, derive(Deserialize))]
#[cfg_attr(test, serde(crate = "near_sdk::serde"))]
pub struct AirdropReward {
    account_id: ValidAccountId,
    amount: U128,
}

const DATA_IMAGE_ICON: &str = "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' xmlns:xlink='http://www.w3.org/1999/xlink' viewBox='0 0 310 310' width='100' height='100'%3E%3Cdefs%3E%3ClinearGradient id='gradient0'%3E%3Cstop stop-color='%23a18732' offset='0' /%3E%3Cstop stop-color='%23eee68b' offset='.5' /%3E%3Cstop stop-color='%23a18732' offset='1' /%3E%3C/linearGradient%3E%3ClinearGradient xlink:href='%23gradient0' id='gradient1' gradientTransform='scale(1.28426 .77866)' x1='15.417' y1='40.839' x2='62.293' y2='40.839' gradientUnits='userSpaceOnUse' /%3E%3ClinearGradient xlink:href='%23gradient0' id='gradient2' gradientUnits='userSpaceOnUse' gradientTransform='scale(2.73872 .36513)' x1='1.398' y1='3.424' x2='103.887' y2='3.424' /%3E%3C/defs%3E%3Ccircle cx='156.7' cy='155.5' r='154.0' fill='%231a1a1a' /%3E%3Cpath d='m 83.163,26.107 c -2.458,0.567 -2.603,8.422 -0.488,17.583 2.114,9.16 5.689,16.156 8.146,15.589 2.458,-0.568 4.004,-7.36 1.89,-16.52 -2.115,-9.16 -7.09,-17.22 -9.548,-16.652 z M 8.431,71.674 C 4.05,66.428 2.62,59.166 2.62,50.076 2.62,36.088 8.464,24.448 17.981,15.782 26.251,8.25 37.331,4 49.326,4 c 14.072,0 24.858,4.33 33.422,14.307 7.134,8.312 13.285,19.887 13.285,31.77 0,8.649 -1.4,15.198 -5.397,20.351 -3.28,4.23 -7.858,7.419 -13.293,9.716 -0.067,4.993 -0.69,10.405 -6.23,13.534 C 66.269,96.416 55.496,97 48.664,97 39.779,97 30.898,97.137 25.797,92.848 22.521,90.093 21.152,84.43 20.867,80.335 15.851,78.277 11.582,75.445 8.432,71.674 Z M 49.118,55.482 c 12.382,0 22.42,-10.038 22.42,-22.42 0,-12.382 -10.038,-22.42 -22.42,-22.42 -12.382,0 -22.42,10.038 -22.42,22.42 0,12.382 10.038,22.42 22.42,22.42 z m -41.403,2.67 c 2.437,0.653 6.253,-6.215 8.686,-15.296 2.433,-9.08 2.562,-16.936 0.126,-17.589 -2.437,-0.653 -6.99,4.618 -9.424,13.7 -2.433,9.08 -1.824,18.532 0.612,19.185 z m 19.399,31.665 c 4.469,2.933 13.044,3.446 21.174,3.446 9.017,0 19.014,-0.032 23.665,-3.939 2.741,-2.302 2.906,-8.778 2.906,-11.753 0,-1.238 -0.327,-3.081 -1.245,-4.237 -1.964,-2.47 -5.76,-4.016 -8.719,-5.397 -4.619,-2.156 -8.181,-2.49 -15.569,-2.49 -6.813,0 -11.988,0.655 -16.815,2.49 -3.87,1.472 -8.576,3.76 -9.964,6.228 -0.522,0.927 0,2.256 0,3.406 0,3.47 1.16,10.009 4.567,12.246 z' fill='url(%23gradient2)' transform='matrix(.84842 0 0 .80002 114.213 33.025)' fill-rule='evenodd' /%3E%3Cg transform='matrix(3.2578 0 0 3.2578 -6.708 25.992)'%3E%3Cpath d='M29.9 66l2.9-11.5h-5.6c-.8 0-1.5-.7-1.5-1.5v-4.7c0-.8.7-1.5 1.5-1.5h5.6c.8 0 1.5.7 1.5 1.5v.3L37 37.8h-4.2c-.8 0-1.5.7-1.5 1.5v3.6h-2.6v-3.6c0-.8-.7-1.5-1.5-1.5h-5.9c-.8 0-1.5.7-1.5 1.5v27.5c0 .8.7 1.5 1.5 1.5H30c-.1-.8-.3-1.6-.1-2.3z' fill='url(%23gradient1)' /%3E%3Cpath d='M78.5 37.8h-5.9c-.8 0-1.5.7-1.5 1.5v3.6h-2.6v-3.6c0-.8-.7-1.5-1.5-1.5h-4.2l2.7 10.8v-.3c0-.8.7-1.5 1.5-1.5h5.6c.8 0 1.5.7 1.5 1.5V53c0 .8-.7 1.5-1.5 1.5H67L70.1 66c.2.8.1 1.5-.3 2.2h8.7c.8 0 1.5-.7 1.5-1.5V39.3c0-.8-.7-1.5-1.5-1.5z' fill='url(%23gradient1)' /%3E%3Cpath d='M60 32.9c-.2-.6-.8-1.1-1.4-1.1H53c-.8 0-1.5.7-1.5 1.5V37h-3.1v-3.7c0-.8-.7-1.5-1.5-1.5h-5.5c-.7 0-1.3.5-1.4 1.1l-8.5 33.5c-.1.4 0 .9.3 1.3s.7.6 1.2.6h11.6v-7c0-3 2.5-5.5 5.5-5.5 3.1 0 5.5 2.5 5.5 5.5v7h11.6c.5 0 .9-.2 1.2-.6.3-.4.4-.8.3-1.3zm-5.2 17.8c0 .8-.7 1.5-1.5 1.5h-6.6c-.8 0-1.5-.7-1.5-1.5v-6.5c0-.8.7-1.5 1.5-1.5h6.6c.8 0 1.5.7 1.5 1.5z' fill='url(%23gradient1)' /%3E%3C/g%3E%3C/svg%3E";

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(owner: ValidAccountId, dao: ValidAccountId, initial_supply: U128) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        let mut contract = Self {
            token: FungibleToken::new(b"t".to_vec()),
            owner: owner.as_ref().clone(),
            dao: dao.as_ref().clone(),
            pending_ft_rewards: LookupMap::new(b"r"),
        };
        let amount: Balance = initial_supply.into();
        contract.token.internal_register_account(owner.as_ref());
        contract.token.internal_deposit(owner.as_ref(), initial_supply.into());
        log!("Deposit {} token to {}", amount, owner);
        contract
    }

    pub fn add_pending_ft_rewards(&mut self, rewards: Vec<(ValidAccountId, U128)>) {
        if env::signer_account_id() != self.owner && env::signer_account_id() != self.dao {
            panic!("{}", errors::ERR01_UNAUTHORIZED)
        }
        for reward in rewards {
            let id = reward.0.to_string();
            let prev = self.pending_ft_rewards.get(&id).unwrap_or_default();
            if let Some(res) = u128::checked_add(prev, reward.1 .0) {
                self.pending_ft_rewards.insert(&id, &res);
            } else {
                panic!("{}", errors::ERR02_OVERFLOW);
            }
        }
    }

    pub fn set_pending_ft_rewards(&mut self, rewards: Vec<(ValidAccountId, U128)>) {
        if env::signer_account_id() != self.owner && env::signer_account_id() != self.dao {
            panic!("{}", errors::ERR01_UNAUTHORIZED)
        }
        for reward in rewards {
            let id = reward.0.to_string();
            self.pending_ft_rewards.insert(&id, &reward.1 .0);
        }
    }

    #[payable]
    pub fn mint(&mut self, account_id: ValidAccountId, amount: U128) {
        let id = account_id.to_string();
        if let Some(pending_rewards) = self.pending_ft_rewards.get(&id) {
            if let Some(res) = u128::checked_sub(pending_rewards, amount.0) {
                self.pending_ft_rewards.insert(&id, &res);
                self.token.internal_deposit(account_id.as_ref(), amount.into());
            } else {
                panic!("{}", errors::ERR03_INSUFFICIENT_BALANCE);
            }
        } else {
            panic!("{}", errors::ERR03_INSUFFICIENT_BALANCE);
        }
    }

    pub fn register_accounts_for_airdrop(&mut self, #[serializer(borsh)] rewards: AirdropRewards) {
        for reward in rewards.0 {
            let id = reward.account_id.to_string();
            if !self.token.accounts.contains_key(&id) {
                self.token.internal_register_account(&id);
            }
        }
    }

    #[payable]
    pub fn airdrop(&mut self, #[serializer(borsh)] rewards: AirdropRewards) {
        if env::signer_account_id() != self.owner && env::signer_account_id() != self.dao {
            panic!("{}", errors::ERR01_UNAUTHORIZED)
        }
        for reward in rewards.0 {
            self.token.internal_deposit(&reward.account_id.into(), reward.amount.0);
        }
    }
}

near_contract_standards::impl_fungible_token_core!(Contract, token);
near_contract_standards::impl_fungible_token_storage!(Contract, token);

#[near_bindgen]
impl FungibleTokenMetadataProvider for Contract {
    fn ft_metadata(&self) -> FungibleTokenMetadata {
        FungibleTokenMetadata {
            spec: FT_METADATA_SPEC.to_string(),
            name: "Shroom Kingdom https://shroomkingdom.net".to_string(),
            symbol: "SHRM".to_string(),
            icon: Some(DATA_IMAGE_ICON.to_string()),
            reference: None,
            reference_hash: None,
            decimals: 18,
        }
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;

    use near_sdk::{
        serde_json,
        test_utils::{accounts, VMContextBuilder},
        testing_env, Balance, MockedBlockchain,
    };

    const INITIAL_SUPPLY: Balance = 20_000_000;

    fn get_context(predecessor_account_id: ValidAccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(accounts(0))
            .signer_account_id(predecessor_account_id.clone())
            .predecessor_account_id(predecessor_account_id);
        builder
    }

    #[test]
    fn test_new() {
        let mut context = get_context(accounts(1));
        testing_env!(context.build());
        let contract = Contract::new(accounts(1), accounts(5), INITIAL_SUPPLY.into());
        testing_env!(context.is_view(true).build());
        assert_eq!(contract.ft_total_supply().0, INITIAL_SUPPLY);
        assert_eq!(contract.ft_balance_of(accounts(1)).0, INITIAL_SUPPLY);
    }

    #[test]
    #[should_panic(expected = "The contract is not initialized")]
    fn test_default() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        let _contract = Contract::default();
    }

    #[test]
    fn test_transfer() {
        let (mut context, mut contract) = setup_contract();

        let transfer_amount = INITIAL_SUPPLY / 3;
        contract.ft_transfer(accounts(1), transfer_amount.into(), None);

        testing_env!(context
            .storage_usage(env::storage_usage())
            .account_balance(env::account_balance())
            .is_view(true)
            .attached_deposit(0)
            .build());
        assert_eq!(contract.ft_balance_of(accounts(2)).0, (INITIAL_SUPPLY - transfer_amount));
        assert_eq!(contract.ft_balance_of(accounts(1)).0, transfer_amount);
    }

    #[test]
    #[should_panic(expected = "E01: unauthorized")]
    fn test_add_rewards_unauthorized() {
        let (_, mut contract) = setup_contract();
        let context = get_context(accounts(1));
        testing_env!(context.build());

        contract.add_pending_ft_rewards(vec![(accounts(1), U128::from(1))]);
    }

    #[test]
    fn test_add_rewards_success() {
        let (_, mut contract) = setup_contract();

        contract.add_pending_ft_rewards(vec![(accounts(1), U128::from(12))]);

        assert_eq!(contract.pending_ft_rewards.get(&accounts(1).to_string()), Some(12));
    }

    #[test]
    fn test_add_rewards_success_2() {
        let (_, mut contract) = setup_contract();

        contract.add_pending_ft_rewards(vec![(accounts(1), U128::from(12))]);
        contract.add_pending_ft_rewards(vec![(accounts(1), U128::from(2))]);

        assert_eq!(contract.pending_ft_rewards.get(&accounts(1).to_string()), Some(14));
    }

    #[test]
    #[should_panic(expected = "E01: unauthorized")]
    fn test_set_rewards_unauthorized() {
        let (_, mut contract) = setup_contract();
        let context = get_context(accounts(1));
        testing_env!(context.build());

        contract.set_pending_ft_rewards(vec![(accounts(1), U128::from(1))]);
    }

    #[test]
    fn test_set_rewards_success() {
        let (_, mut contract) = setup_contract();

        contract.set_pending_ft_rewards(vec![(accounts(1), U128::from(12))]);

        assert_eq!(contract.pending_ft_rewards.get(&accounts(1).to_string()), Some(12));
    }

    #[test]
    fn test_set_rewards_success_2() {
        let (_, mut contract) = setup_contract();

        contract.set_pending_ft_rewards(vec![(accounts(1), U128::from(12))]);
        contract.set_pending_ft_rewards(vec![(accounts(1), U128::from(8))]);

        assert_eq!(contract.pending_ft_rewards.get(&accounts(1).to_string()), Some(8));
    }

    #[test]
    #[should_panic(expected = "E03: insufficient balance")]
    fn test_mint_no_balance() {
        let (_, mut contract) = setup_contract();

        contract.mint(accounts(1), U128::from(42));
    }

    #[test]
    #[should_panic(expected = "E03: insufficient balance")]
    fn test_mint_insufficient_balance() {
        let (_, mut contract) = setup_contract();
        contract.add_pending_ft_rewards(vec![(accounts(1), U128::from(41))]);

        contract.mint(accounts(1), U128::from(42));
    }

    #[test]
    fn test_mint_success() {
        let (_, mut contract) = setup_contract();
        contract.add_pending_ft_rewards(vec![(accounts(1), U128::from(45))]);

        contract.mint(accounts(1), U128::from(42));

        assert_eq!(contract.ft_balance_of(accounts(1)).0, 42);
        assert_eq!(contract.pending_ft_rewards.get(&accounts(1).to_string()), Some(3));
    }

    #[test]
    fn test_mint_success_2() {
        let (_, mut contract) = setup_contract();
        contract.add_pending_ft_rewards(vec![(accounts(1), U128::from(12))]);
        contract.set_pending_ft_rewards(vec![(accounts(1), U128::from(90))]);

        contract.mint(accounts(1), U128::from(42));
        contract.mint(accounts(1), U128::from(3));

        assert_eq!(contract.ft_balance_of(accounts(1)).0, 45);
        assert_eq!(contract.pending_ft_rewards.get(&accounts(1).to_string()), Some(45));
    }

    #[derive(BorshDeserialize, BorshSerialize)]
    pub struct AirdropParams {
        rewards: AirdropRewards,
    }

    #[test]
    fn generate_airdrop_borsh_output() {
        use std::{fs, path::Path};

        let airdrop_path = Path::new("proposals/airdrop");
        fs::remove_dir_all(airdrop_path).unwrap();
        fs::create_dir_all(airdrop_path).unwrap();

        let max_deposit_size = 150;
        let rewards: AirdropRewards =
            serde_json::from_str(include_str!("../proposals/airdrop.json")).unwrap();
        let max_deposit_fits = rewards.0.len() / max_deposit_size + 1;
        let chunk_deposit_size = rewards.0.len() / max_deposit_fits + 1;

        for (index, chunk) in rewards.0.chunks(chunk_deposit_size).enumerate() {
            let borsh_input = AirdropParams { rewards: AirdropRewards(chunk.to_vec()) };
            let borsh_serialized: Vec<u8> = borsh_input.try_to_vec().unwrap();
            let base64_encoded = near_primitives::serialize::to_base64(borsh_serialized.as_slice());
            fs::write(format!("proposals/airdrop/deposit{}", index), base64_encoded).unwrap();
        }

        let max_size = 20;
        let rewards: AirdropRewards =
            serde_json::from_str(include_str!("../proposals/airdrop.json")).unwrap();
        let max_fits = rewards.0.len() / max_size + 1;
        let chunk_size = rewards.0.len() / max_fits + 1;

        for (index, chunk) in rewards.0.chunks(chunk_size).enumerate() {
            let borsh_input = AirdropParams { rewards: AirdropRewards(chunk.to_vec()) };
            let borsh_serialized: Vec<u8> = borsh_input.try_to_vec().unwrap();
            let base64_encoded = near_primitives::serialize::to_base64(borsh_serialized.as_slice());
            fs::write(format!("proposals/airdrop/dao{}", index), base64_encoded).unwrap();
        }
    }

    fn setup_contract() -> (VMContextBuilder, Contract) {
        let mut context = get_context(accounts(2));
        testing_env!(context.build());
        let mut contract = Contract::new(accounts(2), accounts(5), INITIAL_SUPPLY.into());
        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(contract.storage_balance_bounds().min.into())
            .predecessor_account_id(accounts(1))
            .build());
        contract.storage_deposit(None, None);

        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(1)
            .predecessor_account_id(accounts(2))
            .build());
        (context, contract)
    }
}
