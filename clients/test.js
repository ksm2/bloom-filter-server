const net = require('net')

const client = new net.Socket()

function send(data) {
  return new Promise((resolve, reject) => {
    client.once('error', err => reject(err))
    client.once('data', data => resolve(data.toString().trimRight()))
    client.write(data)
  })
}

async function add(item) {
  const response = await send(`ADD ${item}`)
  if (response !== 'OK.') {
    throw new Error(`Server responded with ${response}`)
  }
}

async function has(item) {
  const r = await send(`HAS ${item}`)
  switch (r) {
    case 'Yes.':
      return true
    case 'No.':
      return false
    default:
      throw new Error(`Server responded with ${r}`)
  }
}

client.connect(1337, '127.0.0.1', async () => {
  console.log('Connected to server!')

  console.log(await has('a'))
  await add('a')
  console.log(await has('a'))
  client.destroy()
})

client.on('close', function () {
  console.log('Connection closed')
})
