import { ApiPromise, WsProvider, Keyring } from '@polkadot/api';

async function main() {
    const provider = new WsProvider('ws://127.0.0.1:9945');
    const api = await ApiPromise.create({
        provider,
    });
    // Retrieve the chain & node information information via rpc calls
    const [chain, nodeName, nodeVersion] = await Promise.all([
        api.rpc.system.chain(),
        api.rpc.system.name(),
        api.rpc.system.version()
    ]);
    console.log(`You are connected to chain ${chain} using ${nodeName} v${nodeVersion}`);
    // query and display sudo key
    const sudoKey = await api.query.sudo.key();
    console.log('sudoKey.toHuman()', sudoKey.toHuman());
    console.log('sudoKey.toHex()', sudoKey.toHex());
    const keyring = new Keyring({ type: 'sr25519' });
    // const sudoPair = keyring.getPair(sudoKey.toHex());
    const sudoPair = keyring.addFromUri('//Alice')
    console.log('sudoPair.address', sudoPair.address);
    const isActivated = await api.query.octopusAppchain.isActivated();
    console.log('isActivated:', isActivated.toHuman());
    if ('false' == isActivated.toString()) {
        const call = api.tx.octopusAppchain.forceSetIsActivated(true);
        await api.tx.sudo.sudo(call).signAndSend(sudoPair, (result) => {
            console.log('To activate appchain with sudo', result.toHuman());
        });
    }
    await api.disconnect();
}

main();
