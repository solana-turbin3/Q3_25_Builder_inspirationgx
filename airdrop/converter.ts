import * as prompt from "prompt-sync";
import bs58 from "bs58";

const message = prompt.default({ sigint: false });

const wallet_to_base58 = () => {
  let new_response = message("hello there input your uint8Array: ");

  let pk = Uint8Array.from(JSON.parse(new_response));
  let converted_pk = bs58.encode(pk);

  console.log("your private key is: ");
  console.log(converted_pk);
  console.log("you can use import this to Phantom");
};

const base58_to_wallet = () => {
  let new_response = message(
    "hello there input your base58 privateKey from phantom: "
  );

  let converted_wallet = bs58.decode(new_response);

  console.log("your wallet key is: ");
  console.log(converted_wallet);
  console.log("you can use it in CLI");
};

let entry = message(
  "hello enter A to convert base58 to wallet keys or B to convert from wallet keys to base58: "
);

if (entry && entry.toLowerCase() == "a") {
  base58_to_wallet();
} else if (entry && entry.toLowerCase() == "b") {
  wallet_to_base58();
} else {
  throw "invalid response";
}
