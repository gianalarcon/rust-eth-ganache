use ethers::contract::Contract;
use ethers::prelude::{
  BlockNumber, ConfigurableArtifacts, ContractFactory, LocalWallet, Project, ProjectCompileOutput,
  ProjectPathsConfig, Signer, SignerMiddleware, U256,
};
use ethers::utils::Ganache;
use ethers_providers::{Middleware, Provider};
use ethers_solc::Artifact;
use eyre::Result;
use eyre::{eyre, ContextCompat};
use hex::ToHex;
use std::path::PathBuf;
use std::time::Duration;

pub type SignerDeployedContract<T> = Contract<SignerMiddleware<Provider<T>, LocalWallet>>;

#[tokio::main]
async fn main() -> Result<()> {
  // Spawn a ganache instance
  let mnemonic = "gas monster ski craft below illegal discover limit dog bundle bus artefact";
  let ganache = Ganache::new().mnemonic(mnemonic).spawn();
  println!("HTTP Endpoint: {}", ganache.endpoint());

  // Get the first wallet managed by ganache
  let wallet: LocalWallet = ganache.keys()[0].clone().into();
  let first_address = wallet.address();
  println!(
    "Wallet first address: {}",
    first_address.encode_hex::<String>()
  );

  // A provider is an Ethereum JsonRPC client
  let provider = Provider::try_from(ganache.endpoint())?.interval(Duration::from_millis(10));
  let chain_id = provider.get_chainid().await?.as_u64();

  println!("Ganache started with chain_id: {}", chain_id);
  let project = compile("examples/").await?;

  //print_project(project.clone()).await?;

  let balance = provider.get_balance(wallet.address(), None).await?;
  println!(
    "Wallet first address {} balance: {}",
    wallet.address().encode_hex::<String>(),
    balance
  );

  pub async fn compile(root: &str) -> Result<ProjectCompileOutput<ConfigurableArtifacts>> {
    todo!()
  }

  pub async fn print_project(Project: ProjectCompileOutput<ConfigurableArtifacts>) {
    todo!()
  }
  todo!()
}
