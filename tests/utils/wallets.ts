import * as anchor from "@coral-xyz/anchor";
import user from "../../user.json";
import owner from "../../owner.json";
import manager from "../../manager.json";
import { PublicKey } from "@solana/web3.js";

export function getUserKeypair(): anchor.web3.Keypair {
  return anchor.web3.Keypair.fromSecretKey(Uint8Array.from(user));
}

export function getOwnerKeypair(): anchor.web3.Keypair {
  return anchor.web3.Keypair.fromSecretKey(Uint8Array.from(owner));
}

export function getManagerKeypair(): anchor.web3.Keypair {
  return anchor.web3.Keypair.fromSecretKey(Uint8Array.from(manager));
}
