
./target/release/node-template \
--base-path /tmp/jasonruan-validator01 \
--chain ./customSpecRaw.json \
--port 30334 \
--ws-port 9945 \
--rpc-port 9934 \
--telemetry-url "wss://telemetry.polkadot.io/submit/ 0" \
--validator \
--rpc-methods Unsafe \
--name jasonruan-Validator01 \
--bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWCQyP5FDhGS1z9rZ8FVComanZG6XXsnLxP87L5innGTcE
