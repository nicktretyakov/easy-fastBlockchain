use crate::services::block::Block;
use crate::services::blockchain::Blockchain;
use crate::services::blockchain_service::blockchain::{AddBlockRequest, AddBlockResponse,GetBlockRequest,GetBlockResponse,Empty, Transaction};
use tonic::{transport::Server,Request, Response, Status};

pub mod blockchain {
    tonic::include_proto!("blockchain");
}

use blockchain::blockchain_service_server::{BlockchainService, BlockchainServiceServer};


#[derive(Default)]
pub struct BlockchainServiceImpl{
    blockchain: Blockchain,
}

#[tonic::async_trait]
impl BlockchainService for BlockchainServiceImpl {
    async fn add_block(&self, request: Request<AddBlockRequest>) -> Result<Response<AddBlockResponse>, Status> {
        let req = request.into_inner();
        let data = request.into_inner().data;
        // Add block to the blockchain (implement your logic here)
        // Example:
        let block = self.blockchain.get_block_by_hash(req.hash);
        let new_block = self.blockchain.add_block(data);  // Add block logic in your blockchain struct
        let response = AddBlockResponse { hash: new_block.hash.clone() };

        let response = AddBlockResponse {
            timestamp: block.timestamp,
            pre_block_hash: block.pre_block_hash,
            hash: block.hash,
            transactions: block.transactions.into_iter().map(|tx| Transaction {
                id: tx.get_id(),
                from: tx.get_from(),
                to: tx.get_to(),
                amount: tx.get_amount(),
            }).collect(),
            nonce: block.nonce,
            height: block.height as i64,
        };

        Ok(Response::new(response))
    }

    async fn get_block(&self, request: Request<AddBlockRequest>) -> Result<Response<AddBlockResponse>, Status> {
        let req = request.into_inner();
        let block = self.blockchain.get_block_by_hash(req.hash);

        let response = AddBlockResponse {
            timestamp: block.timestamp,
            pre_block_hash: block.pre_block_hash,
            hash: block.hash,
            transactions: block.transactions.into_iter().map(|tx| Transaction {
                id: tx.get_id(),
                from: tx.get_from(),
                to: tx.get_to(),
                amount: tx.get_amount(),
            }).collect(),
            nonce: block.nonce,
            height: block.height as i64,
        };

        Ok(Response::new(response))
    }

    type GetBlockchainStream = tokio::sync::mpsc::Receiver<Result<AddBlockResponse, Status>>;

    async fn get_blockchain(&self, _request: Request<Empty>) -> Result<Response<Self::GetBlockchainStream>, Status> {
        let (mut tx, rx) = tokio::sync::mpsc::channel(4);
        let blockchain = self.blockchain.clone();

        tokio::spawn(async move {
            for block in blockchain.iter() {
                let response = AddBlockResponse {
                    timestamp: block.timestamp,
                    pre_block_hash: block.pre_block_hash,
                    hash: block.hash,
                    transactions: block.transactions.into_iter().map(|tx| Transaction {
                        id: tx.get_id(),
                        from: tx.get_from(),
                        to: tx.get_to(),
                        amount: tx.get_amount(),
                    }).collect(),
                    nonce: block.nonce,
                    height: block.height as i64,
                };
                tx.send(Ok(response)).await.unwrap();
            }
        });

        Ok(Response::new(rx))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let blockchain_service = BlockchainServiceImpl::default();

    Server::builder()
        .add_service(BlockchainServiceServer::new(blockchain_service))
        .serve(addr)
        .await?;

    Ok(())
}
