mod error;
pub mod query;
pub mod state;
pub mod utils;
pub mod execute;
pub mod handler;

pub use crate::error::ContractError;
pub use crate::state::Cw721Contract;
use cosmwasm_std::Empty;
pub use dotlabs::registrar::{
    ExecuteMsg, Extension, InstantiateMsg, MintMsg, MinterResponse, QueryMsg,MigrateMsg
};

mod test;

#[cfg(not(feature = "library"))]
pub mod entry {
    use super::*;

    use cosmwasm_std::{entry_point};
    use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

    // This makes a conscious choice on the various generics used by the contract
    #[entry_point]
    pub fn instantiate(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: InstantiateMsg,
    ) -> Result<Response, ContractError> {
        let tract = Cw721Contract::<Extension, Empty, Empty, Empty>::default();
        tract.instantiate(deps, env, info, msg)
    }

    #[entry_point]
    pub fn execute(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg<Extension>,
    ) -> Result<Response, ContractError> {
        let tract = Cw721Contract::<Extension, Empty, Empty, Empty>::default();
        tract.execute(deps, env, info, msg)
    }

    #[entry_point]
    pub fn query(deps: Deps, env: Env, msg: QueryMsg<Empty>) -> StdResult<Binary> {
        let tract = Cw721Contract::<Extension, Empty, Empty, Empty>::default();
        tract.query(deps, env, msg)
    }

    #[entry_point]
    pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
        Ok(Response::default())
    }
}