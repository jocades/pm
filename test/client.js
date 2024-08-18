// create a simple node tcp client and coonetct to 127.0.0.1:8421

import net from 'net'

const client = new net.Socket()

client.connect(8421, '127.0.0.1', () => {
  console.log('Connected')
  const json = { Request: { Ping: { msg: 10 } } }
  // const json = { Response: { Ok: 'Yo!' } }
  // const json = { name: 'jordi', age: 25 }

  // client.write('Hello, server! Love, Client.\n')
  client.write(JSON.stringify(json) + '\n')
  // read the reply from the server
})

client.on('data', (data) => {
  console.log('Received:', data, data.toString())
  client.destroy()
})

client.on('close', function () {
  console.log('Connection closed')
})
