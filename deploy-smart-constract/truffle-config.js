module.exports = {
  networks: {
    development: {
     host: "127.0.0.1",     // Localhost 
     port: 7545,            // Port (Ganache)
     network_id: "*"        // Any network 
    }
  },
   compilers: {
    solc: {
      version: "0.8.0",     // Version Solidity
    }
  },
}