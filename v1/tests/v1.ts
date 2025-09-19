import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { V1 } from "../target/types/v1";

describe("v1", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.v1 as Program<V1>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
