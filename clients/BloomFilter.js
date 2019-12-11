const net = require('net')

class BloomFilter {

  /**
   * @param {string} host
   * @param {number} port
   */
  constructor(host = '127.0.0.1', port = 1337) {
    this.host = host
    this.port = port
    this.isOpen = false
    this.client = new net.Socket()
    this.nextData = null
    this.nextError = null

    this.client.on('close', () => {
      this.isOpen = false
    })

    this.client.on('data', (data) => {
      if (this.nextData) {
        this.nextData(data)
      }
    })

    this.client.on('error', (error) => {
      if (this.nextError) {
        this.nextError(error)
      }
    })
  }

  connect() {
    return new Promise((resolve, reject) => {
      this.client.connect(this.port, this.host, () => {
        this.isOpen = true
        resolve()
      })
    })
  }

  end() {
    this.client.end()
  }

  send(cmd, ...data) {
    return new Promise((resolve, reject) => {
      this.nextData = resolve
      this.nextError = reject
      this.client.write(`${cmd} ${data.join(' ')}\n`)
    })
  }

  async add(...items) {
    const response = await this.send('ADD', ...items)
    if (response.toString() !== 'OK.\n') {
      throw new Error(`Server responded with ${response}`)
    }
  }

  async remove(...items) {
    const response = await this.send('RMV', ...items)
    if (response.toString() !== 'OK.\n') {
      throw new Error(`Server responded with ${response}`)
    }
  }

  async has(item) {
    const r = await this.send('HAS', item)
    switch (r.toString()) {
      case 'Yes.\n':
        return true
      case 'No.\n':
        return false
      default:
        throw new Error(`Server responded with ${r}`)
    }
  }

  async count(item) {
    const response = await this.send('CNT', item)
    const s = response.toString()
    if (s.startsWith('ERROR.')) {
      throw new Error(s)
    }

    return parseInt(s, 10)
  }

  async binary() {
    return await this.send('BIN')
  }
}

exports.BloomFilter = BloomFilter
