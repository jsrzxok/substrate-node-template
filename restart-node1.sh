
./target/release/node-template \
--base-path /tmp/jasonruan-bootnode01 \
--chain ./customSpecRaw.json \
--port 30333 \
--ws-port 9944 \
--rpc-port 9933 \
--telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
--validator \
--rpc-methods Unsafe \
--no-mdns \
--name jasonruan-BootNode01
