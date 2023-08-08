import {
  Connection,
  Keypair,
  PublicKey,
  TransactionInstruction,
  SystemProgram,
  sendAndConfirmTransaction,
  Transaction,
} from "@solana/web3.js";
import { Buffer } from "buffer";
import { GetProgramId, Instructions } from "./constants";
import * as utils from "./utils";
import { Record } from "./types";

/**
 * (Instruction constructor)
 * To call once per account. Initialize a Record account. The total_balance of the account will be set to 0.
 *
 * @param userName - The username to be assigned to the Record.name property
 * @param userRecordSeedSigner
 * @param feePayer - required signer
 */
export function createUserRecord(
  userName: string,
  userRecordSeedSigner: PublicKey,
  feePayer: PublicKey
): TransactionInstruction {
  let dataBuffer = Buffer.from("");

  dataBuffer = utils.packUInt8(dataBuffer, Instructions.createUserRecord);
  dataBuffer = utils.packString(dataBuffer, userName);

  dataBuffer = utils.packPubkey(dataBuffer, userRecordSeedSigner);

  // Check every seed length is smaller than 32 bytes
  let userRecordSeedsBuffer = [
    Buffer.from("record"),
    utils.bufferFromPubkey(userRecordSeedSigner),
  ];
  utils.checkSeedsLength(userRecordSeedsBuffer);
  const [userRecordAddress, _userRecordBump] = PublicKey.findProgramAddressSync(
    userRecordSeedsBuffer,
    Record.owner()
  );

  return new TransactionInstruction({
    programId: GetProgramId(),
    keys: [
      { pubkey: userRecordAddress, isSigner: false, isWritable: true },
      { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
      { pubkey: feePayer, isSigner: true, isWritable: true },
    ],
    data: dataBuffer,
  });
}

/**
 * (Transaction handler)
 * To call once per account. Initialize a Record account. The total_balance of the account will be set to 0.
 *
 * @param connection - A connection to a fullnode JSON RPC endpoint
 * @param userName - The username to be assigned to the Record.name property
 * @param userRecordSeedSigner
 * @param feePayer - required signer
 */
export async function createUserRecordSendAndConfirm(
  connection: Connection,
  userName: string,
  userRecordSeedSigner: PublicKey,
  feePayer: Keypair
): Promise<string> {
  const tx = new Transaction().add(
    createUserRecord(userName, userRecordSeedSigner, feePayer.publicKey)
  );
  return await sendAndConfirmTransaction(connection, tx, [feePayer]);
}

/**
 * (Instruction constructor)
 * Register the given amount as an income for the given record account. The total total_balance of the account will be increased.
 *
 * @param amount - The amount to be registered as the income.
 * @param userRecordSeedSigner
 * @param feePayer - required signer
 */
export function registerIncome(
  amount: number,
  userRecordSeedSigner: PublicKey
): TransactionInstruction {
  let dataBuffer = Buffer.from("");

  dataBuffer = utils.packUInt8(dataBuffer, Instructions.registerIncome);
  dataBuffer = utils.packUInt32(dataBuffer, amount);

  dataBuffer = utils.packPubkey(dataBuffer, userRecordSeedSigner);

  // Check every seed length is smaller than 32 bytes
  let userRecordSeedsBuffer = [
    Buffer.from("record"),
    utils.bufferFromPubkey(userRecordSeedSigner),
  ];
  utils.checkSeedsLength(userRecordSeedsBuffer);
  const [userRecordAddress, _userRecordBump] = PublicKey.findProgramAddressSync(
    userRecordSeedsBuffer,
    Record.owner()
  );

  return new TransactionInstruction({
    programId: GetProgramId(),
    keys: [{ pubkey: userRecordAddress, isSigner: false, isWritable: true }],
    data: dataBuffer,
  });
}

/**
 * (Transaction handler)
 * Register the given amount as an income for the given record account. The total total_balance of the account will be increased.
 *
 * @param connection - A connection to a fullnode JSON RPC endpoint
 * @param amount - The amount to be registered as the income.
 * @param userRecordSeedSigner
 * @param feePayer - required signer
 */
export async function registerIncomeSendAndConfirm(
  connection: Connection,
  amount: number,
  userRecordSeedSigner: PublicKey,
  feePayer: Keypair
): Promise<string> {
  const tx = new Transaction().add(
    registerIncome(amount, userRecordSeedSigner)
  );
  return await sendAndConfirmTransaction(connection, tx, [feePayer]);
}

/**
 * (Instruction constructor)
 * Register the given amount as an outcome for the given record account. The total total_balance of the account will be decreased.
 *
 * @param amount - Number to be added to the outcome accumulator
 * @param userRecordSeedSigner
 * @param feePayer - required signer
 */
export function registerOutcome(
  amount: number,
  userRecordSeedSigner: PublicKey
): TransactionInstruction {
  let dataBuffer = Buffer.from("");

  dataBuffer = utils.packUInt8(dataBuffer, Instructions.registerOutcome);
  dataBuffer = utils.packUInt32(dataBuffer, amount);

  dataBuffer = utils.packPubkey(dataBuffer, userRecordSeedSigner);

  // Check every seed length is smaller than 32 bytes
  let userRecordSeedsBuffer = [
    Buffer.from("record"),
    utils.bufferFromPubkey(userRecordSeedSigner),
  ];
  utils.checkSeedsLength(userRecordSeedsBuffer);
  const [userRecordAddress, _userRecordBump] = PublicKey.findProgramAddressSync(
    userRecordSeedsBuffer,
    Record.owner()
  );

  return new TransactionInstruction({
    programId: GetProgramId(),
    keys: [{ pubkey: userRecordAddress, isSigner: false, isWritable: true }],
    data: dataBuffer,
  });
}

/**
 * (Transaction handler)
 * Register the given amount as an outcome for the given record account. The total total_balance of the account will be decreased.
 *
 * @param connection - A connection to a fullnode JSON RPC endpoint
 * @param amount - Number to be added to the outcome accumulator
 * @param userRecordSeedSigner
 * @param feePayer - required signer
 */
export async function registerOutcomeSendAndConfirm(
  connection: Connection,
  amount: number,
  userRecordSeedSigner: PublicKey,
  feePayer: Keypair
): Promise<string> {
  const tx = new Transaction().add(
    registerOutcome(amount, userRecordSeedSigner)
  );
  return await sendAndConfirmTransaction(connection, tx, [feePayer]);
}

/******* GETTERS ********/

/**
 * Default getter for type Record
 *
 * @param connection - A connection to a fullnode JSON RPC endpoint
 * @param signer
 */
export async function getRecord(
  connection: Connection,
  signer: PublicKey
): Promise<Record> {
  const [recordAddress, _recordBump] = PublicKey.findProgramAddressSync(
    [Buffer.from("record"), utils.bufferFromPubkey(signer)],
    Record.owner()
  );

  const recordInfo = await connection.getAccountInfo(
    recordAddress,
    "processed"
  );
  const data = recordInfo ? recordInfo.data : null;
  if (!data) {
    throw new Error("No data retrieved");
  }
  return Record.decode(data, recordAddress);
}
