import { ApiPromise, WsProvider, Keyring } from "@polkadot/api";
import { stringToHex, u8aToHex } from "@polkadot/util";
import { decodeAddress } from '@polkadot/util-crypto';
import { providers, connect, keyStores, utils, Account } from "near-api-js";
import { CodeResult } from "near-api-js/lib/providers/provider";
import BigNumber from "bignumber.js";
import Decimal from "decimal.js";
import BN from "bn.js";
import dotenv from 'dotenv';
dotenv.config();
const { DEMO_ACCOUNT_PK } = process.env;

export class Appchain {
  api: ApiPromise | null = null;
  nativeTokenDecimal = 18;
  alice = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY";
  aliceKeypair: any = null;

  async initAppchain() {
    const wsProvider = new WsProvider('ws://127.0.0.1:9945');
    const api = await ApiPromise.create({
      provider: wsProvider,
    });
    this.api = api;
    // Retrieve the chain & node information information via rpc calls
    const [chain, nodeName, nodeVersion] = await Promise.all([
      api.rpc.system.chain(),
      api.rpc.system.name(),
      api.rpc.system.version()
    ]);
    console.log(`You are connected to chain ${chain} using ${nodeName} v${nodeVersion}`);
    this.api.on("error", (error) =>
      console.log("api error: ", error)
    );
    const keyring = new Keyring({ type: 'sr25519' });
    this.aliceKeypair = keyring.addFromUri('//Alice');
    this.alice = this.aliceKeypair.address;
  }

  async getBalance(account: string) {
    let balance = ZERO_DECIMAL;
    if (this.api) {
      const res = await this.api.query.system.account(account);
      const resJSON: any = res?.toJSON();
      balance = DecimalUtil.fromString(resJSON?.data?.free, this.nativeTokenDecimal);
    }
    return balance;
  }

  async lock(amount: string, targetAccount: string, fromAccount: string) {
    const amountInDec = (DecimalUtil.power(new Decimal(amount), this.nativeTokenDecimal)).toFixed(0, Decimal.ROUND_DOWN);
    const targetAccountInHex = stringToHex(targetAccount);
    console.log(`octopusBridge.lock receiverId: ${targetAccountInHex}, amount: ${amountInDec}`);
    if (this.api) {
      const lock = this.api.tx.octopusBridge.lock(targetAccountInHex, amountInDec);
      await lock.signAndSend(fromAccount, ({ events = [], status }) => {
        console.log('Transaction status:', status.type);

        if (status.isInBlock) {
          console.log('Included at block hash', status.asInBlock.toHex());
          console.log('Events:');
  
          events.forEach(({ event: { data, method, section }, phase }) => {
            console.log('\t', phase.toString(), `: ${section}.${method}`, data.toString());
          });
        } else if (status.isFinalized) {
          console.log('Finalized block hash', status.asFinalized.toHex());
        }        
      });
    }
  }

  async disconnect() {
    if (this.api) {
      this.api.disconnect()
    }
  }
}

export const T_GAS: Decimal = new Decimal(10 ** 12)
export const SIMPLE_CALL_GAS = T_GAS.mul(50).toString()
export const COMPLEX_CALL_GAS = T_GAS.mul(200).toFixed()

export class Near {
  provider: providers.JsonRpcProvider | null = null;
  networkId = "testnet";
  nodeUrl = "https://near-testnet.infura.io/v3/4f80a04e6eb2437a9ed20cb874e10d55";
  demoAccount = "oct-pallet-test.testnet";
  anchorId = "barnacle-ci.registry.test_oct.testnet"
  wrappedTokenAccount = "barnacle-ci.testnet";
  wrappedTokenDecimal = 18;
  account: Account | null = null;
  DEFAULT_GAS = new BN("300000000000000");
  ZERO_DEPOSIT = new BN("0");

  public static yoctoToNearNumber(origin: string) {
    return new BigNumber(origin)
      .div(new BigNumber("1000000000000000000000000"))
      .toNumber();
  }

  async initNear() {
    const jsonRpcProvider = new providers.JsonRpcProvider({
      url: this.nodeUrl,
    });
    this.provider = jsonRpcProvider;
    if (DEMO_ACCOUNT_PK) {
      const keyPair = utils.KeyPair.fromString(DEMO_ACCOUNT_PK as string);
      const keyStore = new keyStores.InMemoryKeyStore();
      keyStore.setKey(this.networkId, this.demoAccount, keyPair);
      const near = await connect({
          networkId: this.networkId,
          keyStore,
          nodeUrl: this.nodeUrl,
          walletUrl: "https://wallet.testnet.near.org",
          helperUrl: "https://helper.testnet.near.org",
      });
      this.account = await near.account(this.demoAccount);
    }
  }

  async getBalance(accountId: string) {
    let balance = ZERO_DECIMAL;
    if (this.provider) {
      const res = await this.provider.query<CodeResult>({
        request_type: "call_function",
        account_id: this.wrappedTokenAccount,
        method_name: "ft_balance_of",
        args_base64: Buffer.from(JSON.stringify({ account_id: accountId })).toString('base64'),
        finality: "final",
      });
      // console.log('ft_balance_of result:', res);
      const balanceStr = Buffer.from(res.result).toString();
      balance = DecimalUtil.fromString(JSON.parse(balanceStr), this.wrappedTokenDecimal);
    }
    return balance;
  }

  async burn(amount: string, targetAccount: string) {
    const amountInU64 = DecimalUtil.toU64(DecimalUtil.fromString(amount), this.wrappedTokenDecimal);
    // decode ss58Address
    const u8a = decodeAddress(targetAccount);
    const targetAccountInHex = u8aToHex(u8a);
    console.log(`burn near provider: ${this.provider}, near account: ${this.account}`);
    if (this.provider && this.account) {
      const args = {
        receiver_id: targetAccountInHex,
        amount: amountInU64.toString(),
      };
      console.log(`burn_wrapped_appchain_token args: ${args}`);
      const result = await this.account.functionCall({
        contractId: this.anchorId,
        methodName: "burn_wrapped_appchain_token",
        args,
        gas: this.DEFAULT_GAS,
        attachedDeposit: this.ZERO_DEPOSIT,
      });
      console.log(result);
    }
  }

}


export const ZERO_DECIMAL = new Decimal(0);
export const ONE_DECIMAL = new Decimal(1);
export const ONE_HUNDRED_DECIMAL = new Decimal(100);

export class DecimalUtil {
  public static fromString(input: string | undefined, shift = 0): Decimal {
    if (!input) {
      return ZERO_DECIMAL;
    }
    return new Decimal(input || 0).div(new Decimal(10).pow(shift));
  }

  public static fromNumber(input: number, shift = 0): Decimal {
    return new Decimal(input).div(new Decimal(10).pow(shift));
  }

  public static fromValue(
    input: string | number | undefined,
    shift = 0
  ): Decimal {
    if (!input) {
      return ZERO_DECIMAL;
    }
    return new Decimal(input).div(new Decimal(10).pow(shift));
  }

  public static fromU64(input: BN, shift = 0): Decimal {
    return new Decimal(input.toString()).div(new Decimal(10).pow(shift));
  }

  public static toU64(input: Decimal, shift = 0): BN {
    if (input.isNeg()) {
      throw new Error(
        `Negative decimal value ${input} cannot be converted to u64.`
      );
    }

    const shiftedValue = new BN(
      input.mul(new Decimal(10).pow(new Decimal(shift))).toFixed()
    );
    return shiftedValue;
  }

  public static shift(input: Decimal, shift = 0): Decimal {
    return input.div(new Decimal(10).pow(new Decimal(shift)));
  }

  public static power(input: Decimal, shift = 0): Decimal {
    return input.mul(new Decimal(10).pow(new Decimal(shift)));
  }
}