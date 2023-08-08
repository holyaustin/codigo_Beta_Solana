import {
    createUserRecordSendAndConfirm,
    getRecord,
    registerIncomeSendAndConfirm,
    registerOutcomeSendAndConfirm,
    SetProgramId
} from "./index";
import {Connection, Keypair} from "@solana/web3.js";
import * as fs from "fs/promises";
import * as path from "path";
import * as os from "os";

async function main(feePayer: Keypair) {
    // TODO: Specify the smart contract Program Id we saved from when we deploy the smart contract
    SetProgramId("2E8j6h6wwzztt6r9SR9P1D9o7ygfRGpYV6akeLdT9qBZ");

    // Instantiate a new Solana connection
    const connection = new Connection("http://127.0.0.1:8899");

    // 1. Create a user record, logs the state of the account after creating it
    await createUserRecordSendAndConfirm(connection, "John Doe", feePayer.publicKey, feePayer);
    let record = await getRecord(connection, feePayer.publicKey);
    console.info(record);

    // 2. Registered a new income with a value of 100
    await registerIncomeSendAndConfirm(connection, 100, feePayer.publicKey, feePayer);
    record = await getRecord(connection, feePayer.publicKey);
    console.info(record);

    // 3. Registered a new outcome with a value of 50
    await registerOutcomeSendAndConfirm(connection, 50, feePayer.publicKey, feePayer);
    record = await getRecord(connection, feePayer.publicKey);
    console.info(record);
}

fs.readFile(path.join(os.homedir(), ".config/solana/id.json"))
    .then(file => main(Keypair.fromSecretKey(new Uint8Array(JSON.parse(file.toString())))));