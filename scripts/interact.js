const bre = require('cudos-blast')

async function main() {
  const [alice, bob, bree] = await bre.getSigners()
  const contract = await bre.getContractFromAddress('Pass in your wallet address here') 

  const MSG_INIT = { text: 'hello!' }
 
  const MSG_RESET = { reset: { text:  'hey'} }
  const QUERY_GREETING = { get_greeting : {}}
  const QUERY_REPLIES = { get_replies: {} }

  const replies = await contract.query(QUERY_REPLIES, alice)
  console.log(replies);
}

module.exports = { main: main }
