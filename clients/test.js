const net = require('net')

const client = new net.Socket()

function send(data) {
  return new Promise((resolve, reject) => {
    client.once('error', err => reject(err))
    client.once('data', data => resolve(data))
    client.write(data)
  })
}

async function add(item) {
  const response = await send(`ADD ${item}`)
  if (response.toString() !== 'OK.\n') {
    throw new Error(`Server responded with ${response}`)
  }
}

async function has(item) {
  const r = await send(`HAS ${item}`)
  switch (r.toString()) {
    case 'Yes.\n':
      return true
    case 'No.\n':
      return false
    default:
      throw new Error(`Server responded with ${r}`)
  }
}

async function bits() {
  return await send('BITS')
}

client.connect(1337, '127.0.0.1', async () => {
  console.log('Connected to server!')

  console.log(await has('a'))
  await add('a')
  console.log(await has('a'))

  const buffer = await bits()
  console.log(buffer)

  client.destroy()
})

client.on('close', function () {
  console.log('Connection closed')
})
