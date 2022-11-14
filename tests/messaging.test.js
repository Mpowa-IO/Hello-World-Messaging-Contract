const bre = require('cudos-blast')

describe('alpha contract', () => {
  // Optional timeout. Default is 15000
  jest.setTimeout(30 * 1000)

  const MSG_INIT = { text: 'hello!' }
  const MSG_RESPOND = { respond: {response: 'hi'} }
  const MSG_RESET = { reset: { text:  'hey'} }
  const QUERY_GREETING = { get_greeting : {}}
  const QUERY_REPLIES = { get_replies: {} }

 

  let alice, bob, contract

  beforeAll(async () => {
    [alice, bob] = await bre.getSigners()
    contract = await bre.getContractFactory('messaging')
    await contract.deploy(MSG_INIT, 'messaging', { signer: bob })
  })

  // test('query greeting', async () => {
  //   return expect(contract.query(QUERY_GREETING))
  //     .resolves.toEqual('hello!')
  // })

  test('respond to greeting', async () => {
    await contract.execute(MSG_RESPOND, alice)
    let expectedReplies = {
      replies: [
        {
          addr: alice.address,
          reply: {
          text: 'hi'
         }
        }
      ]
    };
    return expect(contract.query(QUERY_REPLIES))
      .resolves.toEqual(expectedReplies)
  
  })

  // test('reset greeting from owner', async () => {
  //   await contract.execute(MSG_RESET, bob)
  //   return expect(contract.query(QUERY_GREETING))
  //   .resolves.toEqual('hey')
  // })

  // test('reset greeting from unauthorised user throws unauthorized', () => {
  //   return expect(contract.execute(MSG_RESET, alice))
  //     .rejects.toThrow('Unauthorized')
  // })
})
