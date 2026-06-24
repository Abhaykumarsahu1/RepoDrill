use crate::types::{ScannerEvent, SimpleFinding};
use std::time::Duration;
use tokio::sync::mpsc::Sender;
use tokio::time::sleep;

pub struct MockScanner{
    tx: Sender<ScannerEvent>,
}

impl MockScanner{
    pub fn new(tx: Sender<ScannerEvent>)->Self{
        Self {tx}
    }

    pub async fn run_scan(self){
        let mock_files = [
            "src/main.rs",
            "src/auth/solana_engine.rs",
            "src/utils/crypto.rs",
            ".env",
            "src/ui/dashboard.rs",
            "package.json",
            "src/scanner.rs",
            "contracts/solidity_bridge.sol",
        ];

        for file in mock_files.iter(){
            let _ = self.tx.send(ScannerEvent::ProcessingFile(file.to_string())).await;
            sleep(Duration::from_millis(300)).await;

            if *file == ".env"{
                let finding = SimpleFinding{
                    file_path : file.to_string(),
                    line_number:4,
                    message:"Critical Security leak: Exposed Api".to_string()
                };
                let _ = self.tx.send(ScannerEvent::FoundIssue(finding)).await;
            }

            if *file == "contracts/solidity_bridge.sol" {
                let finding = SimpleFinding {
                    file_path: file.to_string(),
                    line_number: 112,
                    message: "WARNING: TODO: Implement re-entrancy locking mechanisms".to_string(),
                };
                let _ = self.tx.send(ScannerEvent::FoundIssue(finding)).await;
            }
        }

        let _ = self.tx.send(ScannerEvent::Finished).await;
    }
}