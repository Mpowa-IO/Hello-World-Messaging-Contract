# Cudos Messaging Smartcontract

This project is a simple messaging smartcontract. 

- The contract is initiated with a message, and users can reply with their responses. 
- An account can only respond once.
- Trying to respond more than once returns an error. 

This was developed using the [Cudos Blast CLI](https://github.com/CudoVentures/cudos-blast).


---
## Prerequisites


The smartcontract project is written in Rust, and the scripts and Javascript tests are written in (yeah, you guessed right) Javascript.

You should be able to read and understand the workings of the contract even if you are new to Rust.

You need to have the software listed below installed. You can watch [this Youtube tutorial](https://youtu.be/Ujq2Lh9fPRw) to setup your dev environment. 

NOTE: It is assumed that you are on a Linux distro (could be  Windows Subsystem for Linux [WSL](https://learn.microsoft.com/en-us/windows/wsl/) if you are on a Windows PC), or Mac.
NOTE: All `blast` commands are designed to be executed from the project root directory.

- [Rust with your compilation target set to wasm.](https://docs.cudos.org/docs/build/setup-rust/)
- [Node.js and npm.](https://nodejs.org/en/download/package-manager/)
- [Docker](https://docs.docker.com/engine/install/)
- [Cudos Blast CLI tool](https://github.com/CudoVentures/cudos-blast)

You may find it helpful to watch this [video](https://youtu.be/Pry1KxfSQuk) to orient yourself with CosmWasm smartcontract concepts.

---

## Project Setup


1. Ensure you have all your development environment already setup.
2. Create the repository to clone into and open the terminal.
2. Clone this repository by running `git clone https://github.com/Aishat-Akinyemi/cudos-messaging-dapp.git`.
3. Start Docker.
4. Run `npm install` to install the project dependencies.
5. You can run from the commands listed below.


---
## Available Commands
 
| **Command** 	| **Description** 	|
|---	|---	|
| `blast rusttest` 	| runs rust unit tests on the smartcontract. 	|
| `blast compile ` 	| compiles the smartcontract, the compilation is done using rust-optimizer and the optimised wasm files are in the {project_root}/artifacts/ folder. The compiled contract can be tested (with Javascript tests) and deployed. 	|
| `blast node start` 	| starts a local node, and creates a local-accounts.json file inside the project directory where it is run from. This contains the cudos accounts used in blast commands, and blast scripts  	|
| `blast node stop` 	| stops a local node. 	|
| `blast test` 	| runs javascript tests in the {project_root}/tests/ folder 	|
| `blast run [script file location]` 	| runs javascript script file from the specified  file location. For example, `blast run scripts/deploy.js` runs the deploy script and `blast run scripts/interact.js` runs the interact script. You can create your custom scripts and run them with this command.	|
| `blast run [file location] --network [network_name]` 	| runs javascript script file on the specified network, as defined in `blast.config.js` For example, `blast run scripts/deploy.js --network testnet` runs the deploy script on the testnet.	|
| `blast keys ls` 	| list all accounts in the local node key storage 	|
| `blast keys add myAccount1` 	| adds a new account named myAccount1 	|
| `blast keys rm myAccount1` 	| removes an account from the node key storage run 	|
| `blast keys fund myAccount1 --tokens [amount]` 	| funds an existing (local) account from the local node faucet  	|


---
## Project Structure

This is a representation of the project structure. It is not exhaustive but highlights the important files and directories.

```text

|── contracts                   // contains the smartcontract and its related files
│   ├── messaging               
│       ├── examples            
|       |    └──schema.rs     // contains code that generates JSON Schema files.
│       ├── schema            // contains the JSON schema file of the messages and response types.
│       ├── src               // contains the smartcontract specifically
|       |    ├── contract.rs  // contains the contract's entrypoints
│       |    ├── error.rs     //contains the definition of contract errors, can include custom errors
│       |    ├── lib.rs       
|       |    ├── msg.rs       // contains message definition
│       |    └── state.rs     // contains code that has to do with the contract storage  
|       │── Cargo.toml
|       └── ...
│──... 
│── scripts                   // contains scripts we use to interact with cudos blockchain network using the cudos-blast library
|       ├── deploy.js         // script that deploys the smartcontract 
|       └── interact.js       // script containing calls to the smartcontract's execute and query functions
│── tests                      //contains the contract javascript test files
|       └── message.test.js  
├── blast.config.js          // the configuration file used to set blast variables
└── ...
```
---
## Performing Tasks

NOTE: Make sure that Docker is running before you perform these tasks. 
NOTE: All `blast` commands are designed to be executed from the project root directory.

---
### Compiling the smart contract

To compile the  smart contracts run

```bash
blast compile
```

The contract is compilated using [rust-optimizer](https://github.com/CosmWasm/rust-optimizer) and the artifacts stored in `{project_root}/artifacts/` folder. These artifacts are optimized for deployment.

---
### Running Rust tests

Rust tests are defined in the `contract.rs` file. To run smart contracts' Rust tests:

```bash
blast rusttest
```

To run the Rust tests without printing cargo log messages use `--quiet` or `-q`

```bash
blast rusttest -q
```

---
### Testing contracts with JavaScript

Cudos Blast uses [Jest](https://jestjs.io) framework for testing. The Test file is  `{project_root}/tests/messaging.test.js`.

Run the test with:

```bash
blast test
blast test -n testnet
```

The last command runs the test on the testnet.

You can also run the test with disabled console logging and show only essential test result information. To do this use `--silent` or `-s`

```bash
blast test --silent
```

---
### Interacting with a Cudos node

You can interact with a local [`Cudos node`](https://github.com/CudoVentures/cudos-node) with `blast node` command.

**Starting a local node**

To start a fresh local Cudos node run

```bash
blast node start
```

or you can show the node logging output in current terminal window. To do this use `--log` or `-l`.

```bash
blast node start -l
```

**Stopping a running local node**

To stop a running node run

```bash
blast node stop
```

**Checking node status**

To check whether any Cudos node is online or offline run

```bash
blast node status
blast node status -n testnet
```
The last command checks the testnet node's status.

---
**Deploying smart contracts, interacting with them and running custom script files**

To deploy the smartcontract run

```bash
blast run scripts/deploy.js
```

The address at which the contract is deployed will be logged onto the console.

There is a `scripts/interact.js` file with which you can interact with the smart contract. 

To interact with the deployed contract:

- On line 5 of this file, subsitute the contract address with the contract address gotten from deploying the smart contract. 
- run
    ```bash
    blast run scripts/interact.js
    ```    

**Funding your account**

Usually, the accounts are funded with some acudos token, but in case you need to fund your account (if you get an insufficient token error), run the following command:

To specify tokens amount use `--tokens` or `-t`.

```bash
blast keys fund myAccount1 --tokens 1000000
```

The tokens are funded from the default local node faucet in `acudos`.


---











