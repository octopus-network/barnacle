import '@polkadot/api-augment';
import { Appchain, Near } from './util';
import Decimal from 'decimal.js';
import { expect } from "chai";

describe("Appchain RPC (Bridge)", function () {
    let appchain = new Appchain();
    let near = new Near();

    before(async () => {
        await appchain.initAppchain();
        await near.initNear();
    });

    after(async function () {
        await appchain.disconnect();
    });

    it("wrapped appchain native token balance to be updated after cross-chain transfer appchain -> near", async function () {
        if (appchain.api && near.provider) {
            const preBalance = await near.getBalance(near.demoAccount);
            console.log(`Balance before: ${preBalance}`); 
            const amount = '1';
            await appchain.lock(amount, near.demoAccount, appchain.aliceKeypair);
            await new Promise(resolve => setTimeout(resolve, 180000));
            const latestBalance = await near.getBalance(near.demoAccount);
            console.log(`Balance after: ${latestBalance}`); 
            const rightBalance = new Decimal(preBalance).add(new Decimal(amount))
            console.log(`The right balance: ${rightBalance}`); 
            expect(latestBalance.toString()).to.equal(rightBalance.toString());
        }        
    });

    it("appchain native token balance to be updated after cross-chain transfer near -> appchain", async function () {
        if (appchain.api && near.provider) {
            const preBalance = await appchain.getBalance(appchain.alice);
            console.log(`Balance before: ${preBalance}`);
            const amount = '1';
            await near.burn(amount, appchain.alice);
            await new Promise(resolve => setTimeout(resolve, 180000));
            const latestBalance = await appchain.getBalance(appchain.alice);
            console.log(`Balance after: ${latestBalance}`); 
            const rightBalance = new Decimal(preBalance).add(new Decimal(amount))
            console.log(`The right balance: ${rightBalance}`);
            expect(latestBalance.toString()).to.equal(rightBalance.toString());
        } 
    });

    it("near asset balance to be updated after cross-chain transfer appchain -> near", async function () {
        if (appchain.api && near.provider) {
            const preBalance = await near.getAssetBalance(near.demoAccount);
            console.log(`Balance before: ${preBalance}`); 
            const amount = '1';
            await appchain.burnNep141(amount, near.demoAccount, appchain.aliceKeypair);
            await new Promise(resolve => setTimeout(resolve, 180000));
            const latestBalance = await near.getAssetBalance(near.demoAccount);
            console.log(`Balance after: ${latestBalance}`); 
            const rightBalance = new Decimal(preBalance).add(new Decimal(amount))
            console.log(`The right balance: ${rightBalance}`); 
            expect(latestBalance.toString()).to.equal(rightBalance.toString());
        }        
    });

    it("appchain asset balance to be updated after cross-chain transfer near -> appchain", async function () {
        if (appchain.api && near.provider) {
            const preBalance = await appchain.getAssetBalance(appchain.alice);
            console.log(`Balance before: ${preBalance}`);
            const amount = '1';
            await near.lock(amount, appchain.alice);
            await new Promise(resolve => setTimeout(resolve, 180000));
            const latestBalance = await appchain.getAssetBalance(appchain.alice);
            console.log(`Balance after: ${latestBalance}`); 
            const rightBalance = new Decimal(preBalance).add(new Decimal(amount))
            console.log(`The right balance: ${rightBalance}`);
            expect(latestBalance.toString()).to.equal(rightBalance.toString());
        } 
    });
});