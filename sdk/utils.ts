import { Buffer } from "buffer";
import { PublicKey } from "@solana/web3.js";

// PACK

export const packOption = (
  buf: Buffer,
  data: any,
  packF: (buf: Buffer, data: any) => Buffer
): Buffer => {
  if (data) {
    buf = packBool(buf, true);
    buf = packF(buf, data);
    return buf;
  } else {
    buf = packBool(buf, false);
    return buf;
  }
};

export const packVec = (
  buf: Buffer,
  cap: number,
  data: any[],
  packF: (buf: Buffer, data: any) => Buffer
): Buffer => {
  buf = packUInt32(buf, data.length);
  for (let i = 0; i < data.length; i++) {
    buf = packF(buf, data[i]);
  }
  return buf;
};

export const packString = (buf: Buffer, data: string): Buffer => {
  let dataBytes = new TextEncoder().encode(data);
  buf = packUInt32(buf, dataBytes.length);
  let newArrayBuffer = new ArrayBuffer(buf.length + dataBytes.length);
  let newBuffer = Buffer.from(newArrayBuffer);
  newBuffer.set(buf);
  newBuffer.set(dataBytes, buf.length);

  return newBuffer;
};

export const packBool = (buf: Buffer, data: boolean): Buffer => {
  let newArrayBuffer = new ArrayBuffer(buf.length + 1);
  let newBuffer = Buffer.from(newArrayBuffer);
  newBuffer.set(buf);
  data
    ? newBuffer.writeUInt8(1, buf.length)
    : newBuffer.writeUInt8(0, buf.length);
  return newBuffer;
};

export const packUInt8 = (buf: Buffer, data: number): Buffer => {
  let newArrayBuffer = new ArrayBuffer(buf.length + 1);
  let newBuffer = Buffer.from(newArrayBuffer);
  newBuffer.set(buf);
  newBuffer.writeUInt8(data, buf.length);
  return newBuffer;
};

export const packInt8 = (buf: Buffer, data: number): Buffer => {
  let newArrayBuffer = new ArrayBuffer(buf.length + 1);
  let newBuffer = Buffer.from(newArrayBuffer);
  newBuffer.set(buf);
  newBuffer.writeInt8(data, buf.length);
  return newBuffer;
};

export const packUInt16 = (buf: Buffer, data: number): Buffer => {
  let newArrayBuffer = new ArrayBuffer(buf.length + 2);
  let newBuffer = Buffer.from(newArrayBuffer);
  newBuffer.set(buf);
  newBuffer.writeUInt16LE(data, buf.length);
  return newBuffer;
};

export const packInt16 = (buf: Buffer, data: number): Buffer => {
  let newArrayBuffer = new ArrayBuffer(buf.length + 2);
  let newBuffer = Buffer.from(newArrayBuffer);
  newBuffer.set(buf);
  newBuffer.writeInt16LE(data, buf.length);
  return newBuffer;
};

export const packUInt32 = (buf: Buffer, data: number): Buffer => {
  let newArrayBuffer = new ArrayBuffer(buf.length + 4);
  let newBuffer = Buffer.from(newArrayBuffer);
  newBuffer.set(buf);
  newBuffer.writeUInt32LE(data, buf.length);
  return newBuffer;
};

export const packInt32 = (buf: Buffer, data: number): Buffer => {
  let newArrayBuffer = new ArrayBuffer(buf.length + 4);
  let newBuffer = Buffer.from(newArrayBuffer);
  newBuffer.set(buf);
  newBuffer.writeInt32LE(data, buf.length);
  return newBuffer;
};

export const packUInt64 = (buf: Buffer, data: bigint): Buffer => {
  let newArrayBuffer = new ArrayBuffer(buf.length + 8);
  let newBuffer = Buffer.from(newArrayBuffer);
  newBuffer.set(buf);
  newBuffer.writeBigUInt64LE(data, buf.length);
  return newBuffer;
};

export const packInt64 = (buf: Buffer, data: bigint): Buffer => {
  let newArrayBuffer = new ArrayBuffer(buf.length + 8);
  let newBuffer = Buffer.from(newArrayBuffer);
  newBuffer.set(buf);
  newBuffer.writeBigInt64LE(data, buf.length);
  return newBuffer;
};

export const packUInt128 = (buf: Buffer, data: bigint): Buffer => {
  let newArrayBuffer = new ArrayBuffer(buf.length + 16);
  let newBuffer = Buffer.from(newArrayBuffer);
  newBuffer.set(buf);
  newBuffer = writeBigUInt128LE(newBuffer, data, buf.length);

  return newBuffer;
};

export const packInt128 = (buf: Buffer, data: bigint): Buffer => {
  let newArrayBuffer = new ArrayBuffer(buf.length + 16);
  let newBuffer = Buffer.from(newArrayBuffer);
  newBuffer.set(buf);
  newBuffer = writeBigInt128LE(newBuffer, data, buf.length);
  return newBuffer;
};

export const packFloat = (buf: Buffer, data: number): Buffer => {
  let newArrayBuffer = new ArrayBuffer(buf.length + 4);
  let newBuffer = Buffer.from(newArrayBuffer);
  newBuffer.set(buf);
  newBuffer.writeFloatLE(data, buf.length);
  return newBuffer;
};

export const packDouble = (buf: Buffer, data: number): Buffer => {
  let newArrayBuffer = new ArrayBuffer(buf.length + 8);
  let newBuffer = Buffer.from(newArrayBuffer);
  newBuffer.set(buf);
  newBuffer.writeDoubleLE(data, buf.length);
  return newBuffer;
};

// UNPACK

export const unpackOption = (
  buf: Buffer,
  unpackF: (buffer: Buffer) => [any, Buffer]
): [any, Buffer] => {
  let isSome, data, _;
  [isSome, buf] = unpackBool(buf);
  if (isSome) {
    [data, buf] = unpackF(buf);
    return [data, buf];
  } else {
    [_, buf] = unpackF(buf);
    return [null, buf];
  }
};

export const unpackVec = (
  buf: Buffer,
  unpackF: (buffer: Buffer) => [any, Buffer],
  fieldSize: number
): [any, Buffer] => {
  let vecLength, element, new_buf;
  new_buf = buf;
  [vecLength, new_buf] = unpackUInt32(new_buf);
  let array = [];
  for (let i = 0; i < vecLength; i++) {
    [element, new_buf] = unpackF(new_buf);
    array.push(element);
  }
  return [array, buf.subarray(fieldSize)];
};

export const unpackString = (
  buf: Buffer,
  fieldSize: number
): [string, Buffer] => {
  let strLen = buf.readUInt32LE(0);
  let str = buf.subarray(4, 4 + strLen).toString();

  let newBuf = trimBuffer(buf, fieldSize);

  return [str, newBuf];
};

export const unpackUInt8 = (buf: Buffer): [number, Buffer] => {
  let uint8 = buf.readUInt8(0);
  let newBuf = trimBuffer(buf, 1);

  return [uint8, newBuf];
};

export const unpackBool = (buf: Buffer): [boolean, Buffer] => {
  check_valid_bool_value(buf);
  let newBuf = trimBuffer(buf, 1);

  return [buf[0] == 0x01, newBuf];
};

export const unpackInt8 = (buf: Buffer): [number, Buffer] => {
  let int8 = buf.readInt8(0);
  let newBuf = trimBuffer(buf, 1);

  return [int8, newBuf];
};

export const unpackUInt16 = (buf: Buffer): [number, Buffer] => {
  let uint16 = buf.readUInt16LE(0);
  let newBuf = trimBuffer(buf, 2);

  return [uint16, newBuf];
};

export const unpackInt16 = (buf: Buffer): [number, Buffer] => {
  let int16 = buf.readInt16LE(0);
  let newBuf = trimBuffer(buf, 2);

  return [int16, newBuf];
};

export const unpackUInt32 = (buf: Buffer): [number, Buffer] => {
  let uint32 = buf.readUInt32LE(0);
  let newBuf = trimBuffer(buf, 4);

  return [uint32, newBuf];
};

export const unpackInt32 = (buf: Buffer): [number, Buffer] => {
  let int32 = buf.readInt32LE(0);
  let newBuf = trimBuffer(buf, 4);

  return [int32, newBuf];
};

export const unpackUInt64 = (buf: Buffer): [bigint, Buffer] => {
  let uint64 = buf.readBigUInt64LE(0);
  let newBuf = trimBuffer(buf, 8);

  return [uint64, newBuf];
};

export const unpackInt64 = (buf: Buffer): [bigint, Buffer] => {
  let int64 = buf.readBigInt64LE(0);

  let newBuf = trimBuffer(buf, 8);

  return [int64, newBuf];
};

export const unpackUInt128 = (buf: Buffer): [bigint, Buffer] => {
  let uint128 = readBigUInt128LE(buf);
  let newBuf = trimBuffer(buf, 16);

  return [uint128, newBuf];
};

export const unpackInt128 = (buf: Buffer): [bigint, Buffer] => {
  let uint128 = readBigInt128LE(buf);
  let newBuf = trimBuffer(buf, 16);

  return [uint128, newBuf];
};

export const unpackFloat = (buf: Buffer): [number, Buffer] => {
  let float = buf.readFloatLE(0);
  let newBuf = trimBuffer(buf, 4);
  return [float, newBuf];
};

export const unpackDouble = (buf: Buffer): [number, Buffer] => {
  let double = buf.readDoubleLE(0);
  let newBuf = trimBuffer(buf, 8);
  return [double, newBuf];
};
//////////////////////// Generics

// Creates a copy of a subarray of the original Buffer and returns it
const trimBuffer = (buf: Buffer, startIndex: number): Buffer => {
  let newArrayBuffer = new ArrayBuffer(buf.length - startIndex);
  let newBuf = Buffer.from(newArrayBuffer);
  newBuf.set(buf.subarray(startIndex));

  return newBuf;
};

export const copyBuffer = (buf: Buffer): Buffer => {
  let newArrayBuffer = new ArrayBuffer(buf.length);
  let newBuf = Buffer.from(newArrayBuffer);

  newBuf.set(buf);

  return newBuf;
};

const check_valid_bool_value = (buf: Buffer) => {
  let [number, _] = unpackUInt8(buf);
  if (number > 1) throw new Error(`${buf}: Invalid boolean value`);
};

export const bufferFromU8 = (number: number) => {
  let buffer = Buffer.alloc(1);
  buffer.writeUInt8(number, 0);
  return buffer;
};

export const bufferFromI8 = (number: number) => {
  let buffer = Buffer.alloc(1);
  buffer.writeInt8(number, 0);
  return buffer;
};

export const bufferFromU16 = (number: number) => {
  let buffer = Buffer.alloc(2);
  buffer.writeUInt16LE(number, 0);
  return buffer;
};

export const bufferFromI16 = (number: number) => {
  let buffer = Buffer.alloc(2);
  buffer.writeInt16LE(number, 0);
  return buffer;
};

export const bufferFromU32 = (number: number) => {
  let buffer = Buffer.alloc(4);
  buffer.writeUInt32LE(number, 0);
  return buffer;
};

export const bufferFromI32 = (number: number) => {
  let buffer = Buffer.alloc(4);
  buffer.writeInt32LE(number, 0);
  return buffer;
};

export const bufferFromU64 = (number: bigint) => {
  let buffer = Buffer.alloc(8);
  buffer.writeBigUInt64LE(number, 0);
  return buffer;
};

export const bufferFromI64 = (number: bigint) => {
  let buffer = Buffer.alloc(8);
  buffer.writeBigInt64LE(number, 0);
  return buffer;
};

export const bufferFromU128 = (number: bigint) => {
  let buffer = Buffer.alloc(16);
  buffer = writeBigUInt128LE(buffer, number);
  return buffer;
};

export const bufferFromFloat = (number: number) => {
  let buffer = Buffer.alloc(4);
  buffer.writeFloatLE(number, 0);
  return buffer;
};

export const bufferFromDouble = (number: number) => {
  let buffer = Buffer.alloc(8);
  buffer.writeDoubleLE(number, 0);
  return buffer;
};

export const bufferFromBool = (data: boolean) => {
  let buffer = Buffer.alloc(1);
  data ? buffer.writeUInt8(1, 0) : buffer.writeUInt8(0, 0);
  return buffer;
};

export const packPubkey = (buf: Buffer, data: PublicKey): Buffer => {
  let buffer = Buffer.alloc(buf.length + 32);
  buf.copy(buffer, 0);
  data.toBuffer().copy(buffer, buf.length);
  return buffer;
};

export const bufferFromPubkey = (data: PublicKey) => {
  return data.toBuffer();
};

export const unpackPubkey = (buf: Buffer): [PublicKey, Buffer] => {
  let pubkeyBuffer = buf.slice(0, 32);
  return [new PublicKey(pubkeyBuffer), trimBuffer(buf, 32)];
};

export const bufferFromString = (data: string) => {
  return Buffer.from(data);
};

export const writeBigUInt128LE = (
  buf: Buffer,
  value: bigint,
  offset = 0
): Buffer => {
  // Splits the bigint into halfs of 64bit
  const low = value & BigInt("0xffffffffffffffff");
  const high = value >> BigInt(64);

  // Writes each half on buffer, creating a 128bit representation
  buf.writeBigUInt64LE(low, offset);
  buf.writeBigUInt64LE(high, offset + 8);

  return buf;
};

export const readBigUInt128LE = (buf: Buffer): bigint => {
  // Reads a 128bit num by reading 64bits halves
  let low = buf.readBigUInt64LE(0);
  let high = buf.readBigUInt64LE(8);
  return (high << BigInt(64)) + low;
};

function writeBigInt128LE(buf: Buffer, value: bigint, offset = 0): Buffer {
  // If value is negative, flip it over to its two's complement,
  // with the first byte set to 1
  if (value < BigInt(0)) {
    value = (BigInt(1) << BigInt(128)) + value;
  }
  buf.writeBigUInt64LE(value & BigInt("0xffffffffffffffff"), offset);
  buf.writeBigUInt64LE(value >> BigInt(64), offset + 8);
  return buf;
}

// The sign is kept in the sum at the return statement, because high is read as a signed int
export const readBigInt128LE = (buf: Buffer): bigint => {
  let low = buf.readBigUInt64LE(0);
  let high = buf.readBigInt64LE(8);
  return (high << BigInt(64)) + low;
};

export const checkSeedsLength = (seedsBuffer: Buffer[]) => {
  for (const seedBuffer of seedsBuffer) {
    if (seedBuffer != null && seedBuffer.length > 32) {
      throw new Error("Seeds cannot be bigger than 32 bytes long");
    }
  }
};
