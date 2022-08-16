import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { MatoOnchainSdk } from '../target/types/mato_onchain_sdk';

describe('mato-onchain-sdk', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.MatoOnchainSdk as Program<MatoOnchainSdk>;

  it('Is initialized!', async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
