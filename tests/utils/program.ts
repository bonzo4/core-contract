import * as anchor from "@coral-xyz/anchor";
import { AnchorProvider, Program } from "@coral-xyz/anchor";
import { CoreContract } from "../../target/types/core_contract";

export function getProgram(): Program<CoreContract> {
  const provider = AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.CoreContract as Program<CoreContract>;
  return program;
}
