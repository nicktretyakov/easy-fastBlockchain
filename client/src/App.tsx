import {BlockchainRequest, BlockchainServiceClient} from "./grpc-api/blockchain.ts";
import React from "react";


export const App: React.FC = () => {
  const [message, setMessage] = React.useState("")
  const [response, setResponse] = React.useState("")
  const [isLoading, setIsLoading] = React.useState(false)

  return (
    <div>
      <h3>
        Server blockchain
      </h3>
      <form onSubmit={async (e) => {
        e.preventDefault()

        setIsLoading(true)
        const blockchainService = new BlockchainServiceClient('http://localhost:50051');
        const req = new BlockchainRequest({ message })
        const res = await blockchainService.Blockchain(req, {})
        console.log('Received',  res.message)
        setResponse(res.message)
        setIsLoading(false)
      }}>
        <div style={{ marginBottom: '0.5rem' }}>
          <label htmlFor='messageInput'>Message:</label>
          <br/>
          <input id='messageInput' value={message} onChange={(event) => setMessage(event.currentTarget.value)}/>
        </div>

        <div>
          <button type='submit'>Send blockchain request</button>
        </div>
        <p>
          {isLoading ? 'Loading...' : `Response: "${response}"`}
        </p>
      </form>
    </div>
  )
}
