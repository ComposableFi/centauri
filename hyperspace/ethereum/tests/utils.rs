#![allow(dead_code)]

use std::{
	path::{Path, PathBuf},
	sync::Arc,
	time::Duration,
};

use ethers::{
	abi::Token,
	contract::ContractFactory,
	core::{rand::rngs::ThreadRng, utils::Anvil},
	middleware::SignerMiddleware,
	prelude::{ContractInstance, *},
	providers::{Http, Middleware, Provider},
	signers::{LocalWallet, Signer},
	utils::AnvilInstance,
};
use ethers_solc::{
	artifacts::{
		output_selection::OutputSelection, DebuggingSettings, Libraries, Optimizer,
		OptimizerDetails, RevertStrings, Settings, SettingsMetadata,
	},
	Artifact, ConfigurableContractArtifact, EvmVersion, Project, ProjectCompileOutput,
	ProjectPathsConfig, SolcConfig,
};
use hyperspace_ethereum::contract::UnwrapContractError;
use ibc::{
	core::{
		ics04_channel::packet::Packet,
		ics24_host::identifier::{ChannelId, PortId},
	},
	timestamp::Timestamp,
	Height,
};

pub const USE_GETH: bool = false;

#[track_caller]
pub fn yui_ibc_solidity_path() -> PathBuf {
	let base = env!("CARGO_MANIFEST_DIR");
	let default = PathBuf::from(base).join("yui-ibc-solidity");

	if let Ok(path) = std::env::var("YUI_IBC_SOLIDITY_PATH") {
		path.into()
	} else {
		default
	}
}

#[track_caller]
pub fn spawn_anvil() -> (AnvilInstance, Arc<SignerMiddleware<Provider<Http>, LocalWallet>>) {
	let anvil = Anvil::new().spawn();
	let wallet: LocalWallet = if USE_GETH {
		LocalWallet::decrypt_keystore(
			"keys/0x73db010c3275eb7a92e5c38770316248f4c644ee",
			option_env!("KEY_PASS").expect("KEY_PASS not set"),
		)
		.unwrap()
		.into()
	} else {
		anvil.keys()[0].clone().into()
	};

	let endpoint = if USE_GETH { "http://localhost:6001".to_string() } else { anvil.endpoint() };
	let provider = Provider::<Http>::try_from(endpoint)
		.unwrap()
		.interval(Duration::from_millis(10u64));
	let chain_id = if USE_GETH { 4242u64 } else { anvil.chain_id() };
	let client = SignerMiddleware::new(provider, wallet.with_chain_id(chain_id));
	let client = Arc::new(client);

	(anvil, client)
}

#[track_caller]
pub fn compile_solc(project_paths: ProjectPathsConfig) -> ProjectCompileOutput {
	// custom solc config to solve Yul-relatated compilation errors
	let solc_config = SolcConfig {
		settings: Settings {
			stop_after: None,
			remappings: vec![],
			optimizer: Optimizer {
				enabled: Some(false),
				runs: Some(256),
				details: Some(OptimizerDetails {
					peephole: Some(true),
					inliner: Some(true),
					jumpdest_remover: Some(true),
					order_literals: Some(true),
					deduplicate: Some(true),
					cse: Some(true),
					constant_optimizer: Some(true),
					yul: Some(false),
					yul_details: None,
				}),
			},
			model_checker: None,
			metadata: None,
			output_selection: OutputSelection::default_output_selection(),
			evm_version: Some(EvmVersion::Paris),
			via_ir: Some(false),
			debug: Some(DebuggingSettings {
				revert_strings: Some(RevertStrings::Debug),
				debug_info: vec!["location".to_string()],
			}),
			libraries: Libraries { libs: Default::default() },
		},
	};

	let project = Project::builder()
		.paths(project_paths)
		.ephemeral()
		.no_artifacts()
		.solc_config(solc_config)
		.build()
		.expect("project build failed");

	let project_output = project.compile().expect("compilation failed");

	if project_output.has_compiler_errors() {
		for err in project_output.output().errors {
			eprintln!("{}", err);
		}
		panic!("compiler errors");
	}

	return project_output
}

/// Uses solc to compile the yui-ibc-solidity contracts.
///
/// first argument is the path to the yui-ibc-solidity repo.
/// the second argument is the path to the solidity sources, relative to the first argument.
///
/// so if you have the yui-ibc-solidity as the path to yui then sources should be "contracts/core"
/// for IBCHandler or "contracts/clients" for the clients.
#[track_caller]
pub fn compile_yui(path_to_yui: &Path, sources: &str) -> ProjectCompileOutput {
	assert!(
		path_to_yui.exists(),
		"path to yui-ibc-solidity does not exist: {}",
		path_to_yui.display()
	);

	let project_paths = ProjectPathsConfig::builder()
		.root(&path_to_yui)
		.sources(path_to_yui.join(sources))
		.build()
		.unwrap();

	compile_solc(project_paths)
}

#[allow(dead_code)]
pub async fn deploy_contract<M, T>(
	contract: &ConfigurableContractArtifact,
	constructor_args: T,
	client: Arc<M>,
) -> ContractInstance<Arc<M>, M>
where
	M: Middleware,
	T: abi::Tokenize,
{
	let (abi, bytecode, _) = contract.clone().into_parts();
	let mut factory = ContractFactory::new(abi.unwrap(), bytecode.unwrap(), client.clone());
	factory.deploy(constructor_args).unwrap().send().await.unwrap()
}

pub mod mock {
	use ethers::abi::Token;
	use prost::Message;

	#[derive(Clone, PartialEq, ::prost::Message)]
	pub struct ClientState {
		#[prost(message, required, tag = "1")]
		pub height: ibc_proto::ibc::core::client::v1::Height,
	}

	#[derive(Clone, PartialEq, ::prost::Message)]
	pub struct ConsensusState {
		#[prost(uint64, tag = "1")]
		pub timestamp: u64,
	}

	pub fn create_client_msg(kind: &str) -> Token {
		let client_state_bytes = client_state_bytes();

		let consensus_state_bytes = ibc_proto::google::protobuf::Any {
			type_url: "/ibc.lightclients.mock.v1.ConsensusState".into(),
			value: ConsensusState { timestamp: 1 }.encode_to_vec(),
		}
		.encode_to_vec();

		Token::Tuple(vec![
			Token::String(kind.into()),
			Token::Bytes(client_state_bytes),
			Token::Bytes(consensus_state_bytes),
		])
	}

	pub fn client_state_bytes() -> Vec<u8> {
		ibc_proto::google::protobuf::Any {
			type_url: "/ibc.lightclients.mock.v1.ClientState".into(),
			value: ClientState {
				height: ibc_proto::ibc::core::client::v1::Height {
					revision_number: 0,
					revision_height: 1,
				},
			}
			.encode_to_vec(),
		}
		.encode_to_vec()
	}
}

#[derive(Debug)]
pub struct DeployYuiIbc<B, M> {
	pub ibc_client: ContractInstance<B, M>,
	pub ibc_connection: ContractInstance<B, M>,
	pub ibc_channel_handshake: ContractInstance<B, M>,
	pub ibc_packet: ContractInstance<B, M>,
	pub ibc_handler: ContractInstance<B, M>,
	pub tendermint_client: ContractInstance<B, M>,
}

impl<B, M> DeployYuiIbc<B, M>
where
	B: Clone + std::borrow::Borrow<M>,
	M: Middleware,
{
	pub async fn bind_port(&self, port_id: &str, address: Address) {
		let bind_port = self
			.ibc_handler
			.method::<_, ()>("bindPort", (Token::String(port_id.into()), Token::Address(address)))
			.unwrap();
		let () = bind_port.call().await.unwrap_contract_error();
		let tx_recp = bind_port.send().await.unwrap_contract_error().await.unwrap().unwrap();
		assert_eq!(tx_recp.status, Some(1.into()));
	}

	pub async fn connection_open_init(&self, client_id: &str) -> String {
		let connection_open_init = self
			.ibc_handler
			.method::<_, String>(
				"connectionOpenInit",
				(Token::Tuple(vec![
					Token::String(client_id.into()),
					Token::Tuple(vec![
						Token::String(client_id.into()),
						Token::String("port-0".into()),
						Token::Tuple(vec![Token::Bytes(vec![])]),
					]),
					Token::Uint(0.into()),
				]),),
			)
			.unwrap();
		let connection_id = connection_open_init.call().await.unwrap_contract_error();
		let tx_recp = connection_open_init
			.send()
			.await
			.unwrap_contract_error()
			.await
			.unwrap()
			.unwrap();
		assert_eq!(tx_recp.status, Some(1.into()));
		connection_id
	}

	pub async fn connection_open_ack(&self, connection_id: &str, client_state_bytes: Vec<u8>) {
		let connection_open_ack = self
			.ibc_handler
			.method::<_, ()>(
				"connectionOpenAck",
				(Token::Tuple(vec![
					Token::String(connection_id.to_string()),
					Token::Bytes(client_state_bytes), // clientStateBytes
					Token::Tuple(vec![
						Token::String("counterparty-version".into()),
						Token::Array(vec![]),
					]), // Version.Data
					Token::String("counterparty-connection-id".into()), // counterpartyConnectionID
					Token::Bytes(vec![]),             // proofTry
					Token::Bytes(vec![]),             // proofClient
					Token::Bytes(vec![]),             // proofConsensus
					Token::Tuple(vec![Token::Uint(0.into()), Token::Uint(1.into())]), // proofHeight
					Token::Tuple(vec![Token::Uint(0.into()), Token::Uint(1.into())]), // consesusHeight
				]),),
			)
			.unwrap();

		let () = connection_open_ack.call().await.unwrap_contract_error();
		let tx_recp =
			connection_open_ack.send().await.unwrap_contract_error().await.unwrap().unwrap();
		assert_eq!(tx_recp.status, Some(1.into()));
	}

	pub async fn channel_open_init(&self, port_id: &str, connection_id: &str) -> String {
		let fut = self
			.ibc_handler
			.method::<_, String>(
				"channelOpenInit",
				(Token::Tuple(vec![
					Token::String(port_id.into()),
					Token::Tuple(vec![
						// Channel.Data
						Token::Uint(1.into()), // State, Init
						Token::Uint(1.into()), // Ordering
						Token::Tuple(vec![
							Token::String("port-0".into()),
							Token::String("channel-0".into()),
						]), // counterparty
						Token::Array(vec![Token::String(connection_id.into())]), // connectionHops
						Token::String("1".into()), // version
					]),
				]),),
			)
			.unwrap();

		let channel_id = fut.call().await.unwrap_contract_error();
		let tx = fut.send().await.unwrap_contract_error().await.unwrap().unwrap();
		assert_eq!(tx.status, Some(1.into()));
		channel_id
	}

	pub async fn channel_open_ack(&self, channel_id: &str, port_id: &str) {
		let fut = self
			.ibc_handler
			.method::<_, ()>(
				"channelOpenAck",
				(Token::Tuple(vec![
					Token::String(port_id.into()),     // port-id
					Token::String(channel_id.into()),  // channel-id
					Token::String("1".into()),         // counterparty-version
					Token::String("channel-0".into()), // counterparty-channel-id
					Token::Bytes(vec![]),              // proof-try
					Token::Tuple(vec![
						// proof-height
						Token::Uint(0.into()),
						Token::Uint(1.into()),
					]),
				]),),
			)
			.unwrap();

		let () = fut.call().await.unwrap_contract_error();
		let tx = fut.send().await.unwrap_contract_error().await.unwrap().unwrap();
		assert_eq!(tx.status, Some(1.into()));
	}

	pub async fn recv_packet(&self, packet: Packet) -> TransactionReceipt {
		let fut = self
			.ibc_handler
			.method::<_, ()>(
				"recvPacket",
				(Token::Tuple(vec![
					Token::Tuple(vec![
						Token::Uint(packet.sequence.0.into()),              // sequence
						Token::String(packet.source_port.to_string()),      // port-id
						Token::String(packet.source_channel.to_string()),   // channel-id
						Token::String(packet.destination_port.to_string()), // port-id
						Token::String(packet.destination_channel.to_string()), // channel-id
						Token::Bytes(packet.data),                          // data
						Token::Tuple(vec![
							// timeout-height
							Token::Uint(packet.timeout_height.revision_number.into()),
							Token::Uint(packet.timeout_height.revision_height.into()),
						]),
						Token::Uint(
							packet
								.timeout_timestamp
								.into_tm_time()
								.map(|x| x.unix_timestamp_nanos() as u64)
								.unwrap_or(0u64)
								.into(),
						), /* timeout-timestamp */
					]),
					Token::Bytes(vec![]), /* proof */
					Token::Tuple(vec![
						// proof-height
						Token::Uint(0.into()),
						Token::Uint(1.into()),
					]),
				]),),
			)
			.unwrap();

		let () = fut.call().await.unwrap_contract_error();
		// let trace = self
		// 	.ibc_handler
		// 	.client()
		// 	.borrow()
		// 	.debug_trace_call(fut.tx.clone(), None, GethDebugTracingCallOptions::default())
		// 	.await
		// 	.unwrap();
		// std::fs::write("trace.txt", format!("{:#?}", trace)).unwrap();
		// println!("trace: {:?}", trace);
		let tx = fut.send().await.unwrap_contract_error().await.unwrap().unwrap();
		// dbg!(tx.logs);
		let status = tx.status.expect("status not found");

		if status == 0.into() {
			panic!("status is 0");
		}
		tx
	}

	pub async fn register_client(&self, kind: &str, address: Address) {
		let method = self
			.ibc_handler
			.method::<_, ()>(
				"registerClient",
				(Token::String(kind.into()), Token::Address(address)),
			)
			.unwrap();

		let _ = method.call().await.unwrap_contract_error();

		let receipt = method.send().await.unwrap().await.unwrap().unwrap();
		assert_eq!(receipt.status, Some(1.into()));
	}

	pub async fn create_client(&self, msg: Token) -> String {
		let method = self.ibc_handler.method::<_, String>("createClient", (msg,)).unwrap();

		let client_id = method.call().await.unwrap_contract_error();

		let receipt = method.send().await.unwrap().await.unwrap().unwrap();
		assert_eq!(receipt.status, Some(1.into()));

		client_id
	}
}

impl<B: Clone, M: Clone> Clone for DeployYuiIbc<B, M>
where
	B: Clone + std::borrow::Borrow<M>,
{
	fn clone(&self) -> Self {
		Self {
			ibc_client: self.ibc_client.clone(),
			ibc_connection: self.ibc_connection.clone(),
			ibc_channel_handshake: self.ibc_channel_handshake.clone(),
			ibc_packet: self.ibc_packet.clone(),
			ibc_handler: self.ibc_handler.clone(),
			tendermint_client: self.tendermint_client.clone(),
		}
	}
}

pub async fn deploy_yui_ibc<M>(
	project_output: &ProjectCompileOutput,
	client: Arc<M>,
) -> DeployYuiIbc<Arc<M>, M>
where
	M: Middleware,
{
	let contract = project_output.find_first("OwnableIBCHandler").unwrap();
	let (abi, bytecode, _) = contract.clone().into_parts();
	let handler_bytecode = bytecode.unwrap();
	let handler_abi = abi.unwrap();
	dbg!(&handler_bytecode.len() / 1);

	if handler_bytecode.len() > dbg!(24 * 1024) {
		panic!("handler bytecode too large");
	}

	let contract = project_output.find_first("IBCClient").unwrap();
	let (abi, bytecode, _) = contract.clone().into_parts();
	let factory = ContractFactory::new(abi.unwrap(), bytecode.unwrap(), client.clone());
	let deployer1 = factory.deploy(()).unwrap();
	let acc = deployer1.client().default_sender().unwrap();
	dbg!(client.get_balance(acc, None).await.unwrap());
	let ibc_client = deployer1.send().await.unwrap();

	let contract = project_output.find_first("IBCConnection").unwrap();
	let (abi, bytecode, _) = contract.clone().into_parts();
	let factory = ContractFactory::new(abi.unwrap(), bytecode.unwrap(), client.clone());
	let ibc_connection = factory.deploy(()).unwrap().send().await.unwrap();

	let contract = project_output.find_first("IBCChannelHandshake").unwrap();
	let (abi, bytecode, _) = contract.clone().into_parts();
	let factory = ContractFactory::new(abi.unwrap(), bytecode.unwrap(), client.clone());
	let ibc_channel_handshake = factory.deploy(()).unwrap().send().await.unwrap();

	let contract = project_output.find_first("IBCPacket").unwrap();
	let (abi, bytecode, _) = contract.clone().into_parts();
	let factory = ContractFactory::new(abi.unwrap(), bytecode.unwrap(), client.clone());
	let ibc_packet = factory.deploy(()).unwrap().send().await.unwrap();

	//TODO deploy tendermint client as well
	//TODO
	let contract = project_output.find_first("IBCPacket").unwrap();
	let (abi, bytecode, _) = contract.clone().into_parts();
	let factory = ContractFactory::new(abi.unwrap(), bytecode.unwrap(), client.clone());
	let tendermint_client = factory.deploy(()).unwrap().send().await.unwrap();

	let factory = ContractFactory::new(handler_abi, handler_bytecode, client.clone());
	let mut deployer = factory
		.deploy((
			Token::Address(ibc_client.address()),
			Token::Address(ibc_connection.address()),
			Token::Address(ibc_channel_handshake.address()),
			Token::Address(ibc_packet.address()),
		))
		.unwrap();
	// let estimated_gas = client.estimate_gas(&deployer.tx, None).await.unwrap();
	// dbg!(estimated_gas);
	dbg!(client.get_balance(acc, None).await.unwrap());
	let ibc_handler = deployer.clone().send().await.expect("failed to deploy OwnableIBCHandler");

	println!("IBC Handler address: {:?}", ibc_handler.address());

	DeployYuiIbc { ibc_client, ibc_connection, ibc_channel_handshake, ibc_packet, ibc_handler, tendermint_client }
}
