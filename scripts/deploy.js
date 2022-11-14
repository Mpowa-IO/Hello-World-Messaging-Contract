const bre = require('cudos-blast')

async function main() {
  const [alice] = await bre.getSigners()
  
  const contract = await bre.getContractFactory('messaging')

  const MSG_INIT = { text: 'hello world!' }
  await contract.deploy(MSG_INIT, 'messaging', { signer: alice })
  console.log(`Contract deployed at: ${contract.getAddress()}`)
}

module.exports = { main: main }