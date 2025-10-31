use anyhow::Result;
use std::path::PathBuf;
use std::process::Command;
use subxt::{OnlineClient, PolkadotConfig};
use subxt_signer::sr25519::dev;

/// Helper struct for managing contract deployments
struct ContractTest {
    client: OnlineClient<PolkadotConfig>,
    contract_path: PathBuf,
}

impl ContractTest {
    async fn new(exercise_path: &str) -> Result<Self> {
        // Connect to local node
        let client = OnlineClient::<PolkadotConfig>::from_url("ws://127.0.0.1:9944")
            .await
            .expect("Failed to connect to local node");

        let contract_path = PathBuf::from("exercises").join(exercise_path);

        Ok(Self {
            client,
            contract_path,
        })
    }

    fn build_contract(&self) -> Result<PathBuf> {
        println!("🔨 Building contract at {:?}", self.contract_path);

        let output = Command::new("cargo")
            .args(&["contract", "build", "--release"])
            .current_dir(&self.contract_path)
            .output()?;

        if !output.status.success() {
            anyhow::bail!(
                "Contract build failed: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }

        // Find the .contract file
        let target_dir = self.contract_path.join("target/ink");
        let contract_file = std::fs::read_dir(&target_dir)?
            .filter_map(|e| e.ok())
            .find(|e| e.path().extension().and_then(|s| s.to_str()) == Some("contract"))
            .ok_or_else(|| anyhow::anyhow!("No .contract file found"))?;

        Ok(contract_file.path())
    }

    async fn deploy_contract(&self, constructor: &str, args: Vec<String>) -> Result<String> {
        let contract_file = self.build_contract()?;

        println!("📤 Deploying contract from {:?}", contract_file);

        // Use cargo-contract to deploy
        let mut cmd = Command::new("cargo");
        cmd.args(&[
            "contract",
            "instantiate",
            "--suri",
            "//Alice",
            "--constructor",
            constructor,
            "--skip-confirm",
            "--execute",
        ]);

        for arg in args {
            cmd.arg("--args").arg(arg);
        }

        let output = cmd
            .current_dir(&self.contract_path)
            .output()?;

        if !output.status.success() {
            anyhow::bail!(
                "Contract deployment failed: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }

        // Parse contract address from output
        let output_str = String::from_utf8_lossy(&output.stdout);
        let address = extract_contract_address(&output_str)
            .ok_or_else(|| anyhow::anyhow!("Failed to extract contract address"))?;

        println!("✅ Contract deployed at: {}", address);
        Ok(address)
    }

    async fn call_contract(
        &self,
        contract_address: &str,
        message: &str,
        args: Vec<String>,
    ) -> Result<String> {
        println!("📞 Calling contract method: {}", message);

        let mut cmd = Command::new("cargo");
        cmd.args(&[
            "contract",
            "call",
            "--suri",
            "//Alice",
            "--contract",
            contract_address,
            "--message",
            message,
            "--skip-confirm",
            "--execute",
        ]);

        for arg in args {
            cmd.arg("--args").arg(arg);
        }

        let output = cmd
            .current_dir(&self.contract_path)
            .output()?;

        if !output.status.success() {
            anyhow::bail!(
                "Contract call failed: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    async fn read_contract(
        &self,
        contract_address: &str,
        message: &str,
        args: Vec<String>,
    ) -> Result<String> {
        println!("📖 Reading from contract: {}", message);

        let mut cmd = Command::new("cargo");
        cmd.args(&[
            "contract",
            "call",
            "--suri",
            "//Alice",
            "--contract",
            contract_address,
            "--message",
            message,
            "--dry-run",
        ]);

        for arg in args {
            cmd.arg("--args").arg(arg);
        }

        let output = cmd
            .current_dir(&self.contract_path)
            .output()?;

        if !output.status.success() {
            anyhow::bail!(
                "Contract read failed: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
}

fn extract_contract_address(output: &str) -> Option<String> {
    // Parse the contract address from cargo-contract output
    for line in output.lines() {
        if line.contains("Contract") && line.contains("5") {
            // Extract address that starts with 5
            if let Some(addr) = line.split_whitespace().find(|s| s.starts_with('5')) {
                return Some(addr.to_string());
            }
        }
    }
    None
}

// Integration test for intro1 - Basic flipper contract
#[tokio::test]
#[ignore] // Run only with --ignored flag or when explicitly requested
async fn test_intro1_integration() -> Result<()> {
    let test = ContractTest::new("01_intro/intro1").await?;

    // Deploy with default constructor
    let address = test.deploy_contract("default", vec![]).await?;

    // Read initial value
    let output = test.read_contract(&address, "get", vec![]).await?;
    assert!(output.contains("false") || output.contains("Ok(false)"));

    // Flip the value
    test.call_contract(&address, "flip", vec![]).await?;

    // Read updated value
    let output = test.read_contract(&address, "get", vec![]).await?;
    assert!(output.contains("true") || output.contains("Ok(true)"));

    println!("✅ intro1 integration test passed!");
    Ok(())
}

// Integration test for a contract with constructor args
#[tokio::test]
#[ignore]
async fn test_contract_with_args_integration() -> Result<()> {
    let test = ContractTest::new("01_intro/intro1").await?;

    // Deploy with custom constructor
    let address = test.deploy_contract("new", vec!["true".to_string()]).await?;

    // Read initial value (should be true)
    let output = test.read_contract(&address, "get", vec![]).await?;
    assert!(output.contains("true") || output.contains("Ok(true)"));

    println!("✅ Contract with args integration test passed!");
    Ok(())
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn test_address_extraction() {
        let output = "Contract 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY deployed";
        let addr = extract_contract_address(output);
        assert!(addr.is_some());
        assert!(addr.unwrap().starts_with('5'));
    }
}
