use cosmwasm_std::{
    entry_point,  DepsMut, Env, MessageInfo, Response,  Addr, Binary, StdResult, Deps, to_binary, Order,
};
use cw_storage_plus::Bound;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, self, QueryMsg, ReplyResponse};
use crate::state::{Config, CONFIG, Reply, REPLIES};

// Note, you can use StdResult in some functions where you do not
// make use of the custom errors
#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let config = Config {       
        owner: info.sender,
        message: msg.text,
    };
    CONFIG.save(deps.storage, &config)?;

    Ok(Response::default())
}

// And declare a custom Error variant for the ones where you will want to make use of it
#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Respond { response } => try_respond(deps, info, response),
        ExecuteMsg::Reset { text } => try_reset(deps, info, text ),
    }
}

pub fn try_respond(
    deps: DepsMut, 
    info: MessageInfo, 
    response: String 
) -> Result<Response, ContractError> {
    //check if the user has replied in the past, if may_load is none, load, otherwise return already signed error
    let add_key: Addr = info.sender;
    if (REPLIES.may_load(deps.storage, &add_key.clone())?).is_some() {
        return Err(ContractError::AlreadyResponded {  });
    }
    // let addKey = info.sender;
    let resp = Reply { text: response}; 
    REPLIES.save(deps.storage, &add_key, &resp)?;
    Ok(Response::default())
}

pub fn try_respond1(
    deps: DepsMut, 
    info: MessageInfo, 
    response: String 
) -> Result<Response, ContractError> {
    
    let reply = REPLIES.key(&info.sender);


    if (reply.may_load(deps.storage)?).is_some() {
        return Err(ContractError::AlreadyResponded {  });
    }
    // let addKey = info.sender;
    let resp = Reply { text: response}; 
    reply.save(deps.storage, &resp)?;
    Ok(Response::default())
}

pub fn try_reset(deps: DepsMut, info: MessageInfo, text: String) -> Result<Response, ContractError> {
    //updating an item is done with a closure (anonymous function)
    //the closure includes read and write and returns the newly saved value
    CONFIG.update(deps.storage, |mut config| -> Result<_, ContractError> {
        if info.sender != config.owner {
            return Err(ContractError::Unauthorized {});
        }
        config.message = text;
        Ok(config)
    })?;
    Ok(Response::default())
}


// #[entry_point]
// pub fn query(
//     deps: Deps, 
//     env: Env,
//     msg: msg::QueryMsg
//     ) -> StdResult<Binary> {

//         match msg {
//             QueryMsg::GetReplies {  } => to_binary()
//         }
// }

fn get_replies(deps: Deps) -> StdResult<ReplyResponse> {
    let all = REPLIES
    .keys(deps.storage, None, None, Order::Ascending)
    .map(|item| item.map(Into::into))
    .collect::<StdResult<_>>()?;
    Ok(ReplyResponse { replies: all })
}
//     let all: StdResult<Vec<_>> = REPLIES
//     .range(deps.storage, None, None, Order::Ascending)
//     .collect();
//     Ok((all))
// }

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_dependencies_with_balance, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies();
        let msg = InstantiateMsg { text: "Hello world!".to_string()};
        let info = mock_info("owner",  &coins(1000, "acudos"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // it worked, let's query the state
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetReplies {}).unwrap();
        let value: ReplyResponse = from_binary(&res).unwrap();
        // assert_eq!(, value.count);
    }

}



