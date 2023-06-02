use std::{
	path::{Path, PathBuf},
	sync::Arc,
	time::Duration,
};

use ethers::{abi::Token, signers::LocalWallet};

use ethers::{
	contract::ContractFactory,
	core::utils::Anvil,
	middleware::SignerMiddleware,
	prelude::*,
	providers::{Http, Provider},
	signers::Signer,
	solc::{
		artifacts::{
			output_selection::OutputSelection, Libraries, Optimizer, OptimizerDetails, Settings,
		},
		Artifact, Project, ProjectPathsConfig,
	},
	utils::AnvilInstance,
};

#[track_caller]
pub fn yui_ibc_solidity_path() -> PathBuf {
	let base = env!("CARGO_MANIFEST_DIR");
	let default = PathBuf::from(base).join("yui-ibc-solidity-private");

	if let Ok(path) = std::env::var("YUI_IBC_SOLIDITY_PATH") {
		path.into()
	} else {
		default
	}
}

#[track_caller]
pub fn spawn_anvil() -> (AnvilInstance, Arc<SignerMiddleware<Provider<Http>, LocalWallet>>) {
	let anvil = Anvil::new().spawn();
	let wallet: LocalWallet = anvil.keys()[0].clone().into();

	let provider = Provider::<Http>::try_from(anvil.endpoint())
		.unwrap()
		.interval(Duration::from_millis(10u64));

	let client = SignerMiddleware::new(provider, wallet.with_chain_id(anvil.chain_id()));
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
				enabled: Some(true),
				runs: Some(20),
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
			evm_version: None,
			via_ir: Some(false),
			debug: None,
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
	let factory = ContractFactory::new(abi.unwrap(), bytecode.unwrap(), client.clone());
	factory.deploy(constructor_args).unwrap().send().await.unwrap()
}

#[derive(Debug)]
pub struct DeployYuiIbc<B, M> {
	pub ibc_client: ContractInstance<B, M>,
	pub ibc_connection: ContractInstance<B, M>,
	pub ibc_channel_handshake: ContractInstance<B, M>,
	pub ibc_packet: ContractInstance<B, M>,
	pub ibc_handler: ContractInstance<B, M>,
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
	let contract = project_output.find_first("IBCClient").unwrap();
	let (abi, bytecode, _) = contract.clone().into_parts();
	let factory = ContractFactory::new(abi.unwrap(), bytecode.unwrap(), client.clone());
	let ibc_client = factory.deploy(()).unwrap().send().await.unwrap();

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

	let contract = project_output.find_first("OwnableIBCHandler").unwrap();
	let (abi, bytecode, _) = contract.clone().into_parts();
	let factory = ContractFactory::new(abi.unwrap(), bytecode.unwrap(), client.clone());
	let ibc_handler = factory
		.deploy((
			Token::Address(ibc_client.address()),
			Token::Address(ibc_connection.address()),
			Token::Address(ibc_channel_handshake.address()),
			Token::Address(ibc_packet.address()),
		))
		.unwrap()
		.send()
		.await
		.expect("failed to deploy OwnableIBCHandler");

	DeployYuiIbc { ibc_client, ibc_connection, ibc_channel_handshake, ibc_packet, ibc_handler }
}
