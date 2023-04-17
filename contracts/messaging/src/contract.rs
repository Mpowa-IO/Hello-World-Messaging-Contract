use cosmwasm_std::{
    entry_point,  DepsMut, Env, MessageInfo, Response,  Addr, Binary, StdResult, Deps, to_binary, Order,
};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, self, QueryMsg, ReplyResponse, ReplyInfo};
use crate::state::{Config, CONFIG, Reply, REPLIES};


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
    
     let add_key: Addr = info.sender;
    if (REPLIES.may_load(deps.storage, &add_key.clone())?).is_some() {
        return Err(ContractError::AlreadyResponded {  });
    }
    // let addKey = info.sender;
    let resp = Reply { text: response}; 
    REPLIES.save(deps.storage, &add_key, &resp)?;
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


#[entry_point]
pub fn query(
    deps: Deps, 
    _env: Env,
    msg: msg::QueryMsg
    ) -> StdResult<Binary> {
        match msg {
            QueryMsg::GetReplies {  } => to_binary(&get_replies(deps)?),
            QueryMsg::GetGreeting {  } => to_binary(&get_greeting(deps)?)
        }
}

pub fn get_greeting (deps: Deps) -> StdResult<String> {
    let config= CONFIG.may_load(deps.storage)?.unwrap();
    Ok(config.message) 
}

pub fn get_replies(deps: Deps) -> StdResult<ReplyResponse> {
    let all: StdResult<Vec<_>> = REPLIES
    .range(deps.storage, None, None, Order::Ascending)
    .map(|item|  item.map(|(addr, reply)| ReplyInfo{ addr, reply}))
    .collect();
    
    all.map(|replies| ReplyResponse { replies } )
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info,};
    use cosmwasm_std::{coins, from_binary};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies();
        let msg = InstantiateMsg { text: "Hello world!".to_string()};
        let info = mock_info("owner",  &coins(1000, "acudos"));

        // we can just call .unwrap() to assert this was a success
        let init = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, init.messages.len());

        // it worked, let's query the state
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetGreeting {  }).unwrap();
        let value : String= from_binary(&res).unwrap();
        assert_eq!("Hello world!", value);
    }

    #[test]
    fn respond() {
        let mut deps = mock_dependencies();
        let msg = InstantiateMsg { text: "Hello world!".to_string()};
        let info = mock_info("owner",  &coins(1000, "acudos"));
        let _init = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        
        //should save the response to storage
        let info = mock_info("responder1", &coins(0, "acudos"));
        let msg = ExecuteMsg::Respond { response: "thanks for the warm welcome.".to_string() } ;
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        //querying the response should return the greeting
        let expected_info = mock_info("responder1", &coins(0, "acudos"));
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetReplies {  }).unwrap();
        let value: ReplyResponse = from_binary(&res).unwrap();
        let expected_resp_value:ReplyResponse = ReplyResponse {
            replies: vec![ReplyInfo{addr: expected_info.sender, reply: Reply { text: "thanks for the warm welcome.".to_string() } }]
        };
        assert_eq!(expected_resp_value, value);  
    
        //should fail when an addr tries responding more than once
        let info = mock_info("responder1", &coins(0, "acudos"));
        let msg = ExecuteMsg::Respond { response: "thanks for the warm welcome.".to_string() } ;
        let res = execute(deps.as_mut(), mock_env(), info, msg);
        match res {
            Err(ContractError::AlreadyResponded {}) => {}
            _ => panic!("Must return already responded error"),
        }
        
    }

    #[test]
    fn reset() {
        let mut deps = mock_dependencies();
        let msg = InstantiateMsg { text: "Hello world!".to_string()};
        let info = mock_info("owner",  &coins(1000, "acudos"));
        let _init = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        
        //should fail to reset and return unauthorised
        let unauth_info = mock_info("unauth", &coins(0, "acudos"));
        let msg = ExecuteMsg::Reset { text: "How can I help you today?".to_string() } ;
        let res = execute(deps.as_mut(), mock_env(), unauth_info, msg);
        match res {
            Err(ContractError::Unauthorized {}) => {}
            _ => panic!("Must return unauthorized error"),
        }

        //should allow owner to reset 
        let unauth_info = mock_info("owner", &coins(0, "acudos"));
        let msg = ExecuteMsg::Reset { text: "How can I help you today?".to_string() } ;
        let _res = execute(deps.as_mut(), mock_env(), unauth_info, msg).unwrap();

        // it worked, let's query the state
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetGreeting {  }).unwrap();
        let value : String= from_binary(&res).unwrap();
        assert_eq!("How can I help you today?", value);

       
        
    }

}



