import { PublicKey, PublicKeyInitData } from "@solana/web3.js";

let _PROGRAM_ID: PublicKey;

export function SetProgramId(newProgId: PublicKeyInitData) {
  _PROGRAM_ID = new PublicKey(newProgId);
}

export function GetProgramId(): PublicKey {
  return _PROGRAM_ID;
}

export const Instructions = {
  createUserRecord: 0,
  registerIncome: 1,
  registerOutcome: 2,
};
