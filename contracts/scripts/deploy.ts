import { deployContract } from '@scio-labs/use-inkathon'
import * as dotenv from 'dotenv'
import { getDeploymentData } from './utils/getDeploymentData'
import { initPolkadotJs } from './utils/initPolkadotJs'
import { ApiPromise, Keyring } from '@polkadot/api'
import { writeContractAddresses } from './utils/writeContractAddresses'
import { ContractPromise } from '@polkadot/api-contract'
// [KEEP THIS] Dynamically load environment from `.env.{chainId}`
const chainId = process.env.CHAIN || 'development'
dotenv.config({
  path: `.env.${chainId}`,
})

/**
 * Script that deploys the greeter contract and writes its address to a file.
 *
 * Parameters:
 *  - `DIR`: Directory to read contract build artifacts (optional, defaults to `./deployments`)
 *  - `CHAIN`: Chain ID (optional, defaults to `development`)
 *
 * Example usage:
 *  - `pnpm run deploy`
 *  - `CHAIN=alephzero-testnet pnpm run deploy`
 */
const main = async () => {
  // [KEEP THIS] Initialization
  const accountUri = process.env.ACCOUNT_URI || '//Alice'
  const accountUri2 = process.env.ACCOUNT_URI || '//BOBOB'
  const { api, chain, account } = await initPolkadotJs(chainId, accountUri)
  const keyring = new Keyring({ type: 'sr25519' })
  const account2 = keyring.addFromUri(accountUri2)
  // Deploy greeter contract
  const { abi, wasm } = await getDeploymentData('greeter')
  const greeter = await deployContract(api, account, abi, wasm, 'default', [])
  const token_data = await getDeploymentData('share_token')
  const share_token = await deployContract(api, account, token_data.abi, token_data.wasm, 'new', [
    'TEST',
    'TS',
  ])
  const token_instance = new ContractPromise(api, token_data.abi, share_token.address)
  const value = 10000 // only for payable messages, call will fail otherwise
  const gasLimit = 3000n * 1000000n
  const storageDepositLimit = null
  const incValue = 1
  //console.log(token_instance.tx)
  // Send the transaction, like elsewhere this is a normal extrinsic
  // with the same rules as applied in the API (As with the read example,
  // additional params, if required can follow - here only one is needed)
  //transfer_ownership to vault.address

  const transfer_ownership = await token_instance.tx['ownable::transferOwnership'](
    { storageDepositLimit, gasLimit },
    account2.address,
  ).signAndSend(account, (result) => {
    if (result.status.isInBlock) {
      console.log('in a block')
    } else if (result.status.isFinalized) {
      console.log('finalized')
    }
  })
  console.log(transfer_ownership)
  // Write contract addresses to `{contract}/{network}.ts` file(s)
  await writeContractAddresses(chain.network, {
    greeter,
  })
}

main()
  .catch((error) => {
    console.error(error)
    process.exit(1)
  })
  .finally(() => process.exit(0))
