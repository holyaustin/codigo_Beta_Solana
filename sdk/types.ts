import { Buffer } from "buffer";
import * as utils from "./utils";
import { PublicKey } from "@solana/web3.js";

import { GetProgramId } from "./constants";

export class Record {
  /**
   * Create a new Record object
   *
   * @remarks
   * Through this data structure we will store the relevant information to track the income and outcome of a given user.
   *
   * @param name - The name of the user.
   * @param moves - Number incomes/outcomes registered.
   * @param outcome - Sum of all outcomes.
   * @param income - Sum of all incomes.
   * @param totalBalance - The current balance of the user
   * @param pubkey - A public key
   */
  constructor(
    public name: string,
    public moves: number,
    public outcome: number,
    public income: number,
    public totalBalance: bigint,
    public pubkey: PublicKey
  ) {}

  static decode(buffer: Buffer, pubkey: PublicKey): Record {
    let newBuffer = utils.copyBuffer(buffer);

    let name;
    let moves;
    let outcome;
    let income;
    let totalBalance;

    [name, newBuffer] = utils.unpackString(newBuffer, 54);
    [moves, newBuffer] = utils.unpackUInt16(newBuffer);
    [outcome, newBuffer] = utils.unpackUInt32(newBuffer);
    [income, newBuffer] = utils.unpackUInt32(newBuffer);
    [totalBalance, newBuffer] = utils.unpackInt64(newBuffer);

    return new Record(name, moves, outcome, income, totalBalance, pubkey);
  }

  static owner(): PublicKey {
    return GetProgramId();
  }
}
