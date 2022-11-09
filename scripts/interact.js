const bre = require('cudos-blast')

async function main() {
  const [alice, bob, bree] = await bre.getSigners()
  const contract = await bre.getContractFromAddress('cudos1zz89vvmdwxuww63034jhdvtu449w7hxyd4zt3rwylryd2nm3rkuqm37cut')
  // cudos14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9strccpl
  // cudos1pryug3pp92fhn5qavdt2uxu32j3gv0v7vueuzs3ep8xelqd6exlsdgndla

  const MSG_INIT = { text: 'hello!' }
 
  const MSG_RESET = { reset: { text:  'hey'} }
  const QUERY_GREETING = { get_greeting : {}}
  const QUERY_REPLIES = { get_replies: {} }

  const replies = await contract.query(QUERY_REPLIES, alice)
  console.log(replies);
}

module.exports = { main: main }
