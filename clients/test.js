const { BloomFilter } = require('./BloomFilter')

async function main() {
  const bf = new BloomFilter()
  await bf.connect()

  console.log(await bf.has('a'))
  console.log(await bf.count('a'))

  await bf.add('a')

  console.log(await bf.has('a'))
  console.log(await bf.count('a'))

  await bf.add('a')
  console.log(await bf.count('a'))

  await bf.add('a')
  console.log(await bf.count('a'))

  await bf.remove('a')
  await bf.remove('a')
  await bf.remove('a')

  console.log(await bf.has('a'))
  console.log(await bf.count('a'))

  bf.end()
}

main().catch(console.error)
