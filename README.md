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

- [Rust with your compilation target set to wasm.](https://docs.cudos.org/docs/build/setup-rust/)
- [Node.js and npm.](https://nodejs.org/en/download/package-manager/)
- [Docker](https://docs.docker.com/engine/install/)
- [Cudos Blast CLI tool](https://github.com/CudoVentures/cudos-blast)

You may find it helpful to watch this [video](https://youtu.be/Pry1KxfSQuk) to orient yourself with CosmWasm smartcontract concepts.

---

## Project Setup


1. Ensue you have all your development environment already setup.
2. Create the repository to clone into and open the terminal.
2. Clone this repository by running `git clone https://github.com/Aishat-Akinyemi/cudos-messaging-dapp.git`.
3. Start Docker.
4. Run `npm install` to install the project dependencies.


---
## Available Commands
 
| **Command** 	| **Description** 	|
|---	|---	|
| `blast rusttest` 	| runs rust unit tests on the smartcontract. 	|
| `blast compile ` 	| compiles the smartcontract, the compilation is done using rust-optimizer and the optimised wasm files are in the {project_root}/artifacts/ folder. The compiled contract can be tested (with Javascript tests) and deployed. 	|
| `blast node start` 	| starts a local node, and creates a local-accounts.json file inside the project directory where it is run from. This contains the cudos accounts used in blast commands, and blast scripts  	|
| `blast node stop` 	| stops a local node. 	|
| `blast test` 	| runs javascript tests in the {project_root}/tests/ folder 	|
| `blast run [script file location]` 	| runs javascript script file from the specified location. For example, `blast run scripts/deploy.js` runs the deploy script 	|
| `blast keys ls` 	| list all accounts in the local node key storage 	|
| `blast keys add myAccount1` 	| adds a new account named myAccount1 	|
| `blast keys rm myAccount1` 	| removes an account from the node key storage run 	|
| `blast keys fund myAccount1 --tokens [amount]` 	| funds an existing (local) account from the local node faucet  	|


---
## Project Structure

This is a representation of the project structure. It is not exhaustive but highlights the important files and directories.

```text

|── contracts
│   ├── messaging
│       ├── examples
|       |    └──schema.rs   
│       ├── schema
│       ├── src
|       |    ├── contract.rs
│       |    ├── error.rs
│       |    ├── lib.rs
|       |    ├── msg.rs
│       |    └── state.rs        
|       │── Cargo.toml
|       └── ...
│──... 
│── scripts
|       ├── deploy.js
|       └── interact.js
│── tests       
|       └── message.test.js
├── blast.config.js
└── ...
```



---
## A short Overview







