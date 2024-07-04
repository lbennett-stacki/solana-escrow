import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { TokenFactory } from "../target/types/token_factory";
import { TOKEN_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";

describe("token-factory", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.TokenFactory as Program<TokenFactory>;

  const mintToken = anchor.web3.Keypair.generate();

  const associateTokenProgram = new anchor.web3.PublicKey(
    "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"
  );

  console.log("Get token account");

  const tokenAccount = anchor.utils.token.associatedAddress({
    mint: mintToken.publicKey,
    owner: provider.publicKey,
  });

  console.log("Get token address");
  const tokenAddress = anchor.web3.PublicKey.findProgramAddressSync(
    [
      provider.publicKey.toBuffer(),
      TOKEN_PROGRAM_ID.toBuffer(),
      mintToken.publicKey.toBuffer(),
    ],
    associateTokenProgram
  )[0];

  let tokenAccountKeyPair = anchor.web3.Keypair.generate();

  it("Create token!", async () => {
    console.log(mintToken.publicKey.toBase58());
    console.log(tokenAccount.toBase58());

    console.log("Create token...");
    try {
      const tx = await program.methods
        .createToken(9, new anchor.BN(10 ** 9 * 100))
        .accounts({
          mintToken: mintToken.publicKey,
          tokenAccount: tokenAccount,
          associateTokenProgram,
        })
        .signers([mintToken])
        .rpc();
      console.log("Your transaction signature", tx);
    } catch (error) {
      console.log(error);
    }
  });
});
