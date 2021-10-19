const fs = require('fs')
const { execSync } = require('child_process')

const accounts = JSON.parse(fs.readFileSync('airdrop.json', { encoding: 'utf8' })).map(a => a.account_id);

function main () {
  for (const account of accounts) {
    try {
      execSync(`near call token.shrm.near ft_transfer '{"receiver_id": "${account}", "amount": "0"}' --accountId token.shrm.near`)
    } catch (err) {}
  }
}
main()
