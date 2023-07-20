mod render_abi_json;
use render_abi_json::createfile::create_file_json;

use std::str::FromStr;
use web3::{
    contract::Contract,
    contract::Options,
    transports::Http,
    types::{Address, RawTransaction, TransactionRequest, H160, U256, Transaction},
    Web3,
};
 // const 1ETH=1000000000000000000;
#[tokio::main]
async fn main() {
    //-> web3::Result<()>
    let transport = web3::transports::Http::new("http://127.0.0.1:7545").unwrap();
    let web3 = web3::Web3::new(transport);

    //Define the contract ABI and address
    let contract_abi = include_bytes!("contracts/Hello.abi.json");
    let contract_address = Address::from_str("0x63954603907fe87a68aAf182d0F79C2bD6101dca").unwrap();

    //Connect to the contract
    let contract = Contract::from_json(web3.eth(), contract_address, contract_abi).unwrap();

    // Call a function printGreeting on the contract
    let result = contract.query("printGreeting", (), None, Options::default(), None);
    let va: String = result.await.unwrap();
    // Get the result of the function call
    //println!("Value of Smart Contract Old: {}", va);

   
    // let account0=Address::from_str("0x1Bdd9f2578843a7C2E69967F0e608eC95b987C70").unwrap();
    ////Call function setGreeting on the contract
    //let greeting="Hello Huy".to_string();
    //let tx = contract.call("setGreeting", (greeting,), account0, Options::default()).await.unwrap();
    //println!("TxHash: {}", tx);

    //let result1 = contract.query("printGreeting", (), None, Options::default(), None);
    //let va: String = result1.await.unwrap();
    //// Get the result of the function call
    //println!("Value of Smart Contract New: {}", va);

   
    //transfer 3 Eth form "0x1Bdd9f2578843a7C2E69967F0e608eC95b987C70" to "0x53B61DbBF3a81f9b1a45F623bfF957Aa527ca083"
    let tx_tran=TransactionRequest{
        from:Address::from_str("0x1Bdd9f2578843a7C2E69967F0e608eC95b987C70").unwrap(),
        to:Some(Address::from_str("0x53B61DbBF3a81f9b1a45F623bfF957Aa527ca083").unwrap()),
        value:Some(U256::from(3000000000000000000 as i128)),
        ..Default::default()
    };
    web3.eth().send_transaction(tx_tran).await.unwrap();

    


    
    /*
    //     let mut accounts = web3.eth().accounts().await?;
    //    // show_balance(accounts,web3).await;
    //     let _tx=TransactionRequest{
    //         from: accounts[1],
    //         to: Some(accounts[2]),
    //         gas: None,
    //         gas_price: None,
    //         value: Some(U256::from(2712000)),
    //         data: Some("Hello,World".as_bytes().into()),
    //         nonce: None,
    //         condition: None,
    //         transaction_type: None,
    //         access_list: None,
    //         max_fee_per_gas: None,
    //         max_priority_fee_per_gas: None,
    //     };

    //     web3.eth().send_transaction(_tx).await?;
    */
}

//show account
async fn show_balance(accounts: Vec<H160>, web3: Web3<Http>) {
    println!("Calling balance.");
    for account in accounts {
        let balance = web3.eth().balance(account, None).await.unwrap();
        println!("Balance of {:?}: {}", account, balance);
    }
}
