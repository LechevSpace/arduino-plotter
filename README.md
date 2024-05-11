# arduino-plotter
## Protocol crate and CLI used to communicate with Arduino serial plotter

### Arduino serial plotter: https://github.com/arduino/arduino-serial-plotter-webapp


#### Running arduino-serial-plotter webapp

**Arduino Serial Plotter** uses websocket to communicate by sending or receiving commands.

Requirements:
- Node v10

```bash
git clone https://github.com/arduino/arduino-serial-plotter-webapp
cd arduino-serial-plotter-webapp
npm i && npm start
```

Default port: **3000**

#### Connecting

The **Arduino serial plotter** will send a request to our CLI and the CLI is waiting for a connection on the websocket, this is why you need to open the arduino plotter app after starting the CLI:

`http://localhost:3000` (with default port **3000**)

#### Running example
You can use `run` example for a basic usage of the Client and Server:


`cargo run --example run`

Refer to the documentation in the file for more details.
The example does all the basic main functionality that you need and sends
random data to the `arduino-serial-plotter-webapp`:

- Sends initial settings
- Sends Random data with 2 different data lines
- Receives settings from the **Arduino serial plotter** and confirms a new End of Line by sending a settings message back to it
- Receives data messages sent from the **Arduino serial plotter** UI and logs them using `tracing` to the console

### License
Licensed under either of [Apache License, Version 2.0](./LICENSE-APACHE) or [MIT license](./LICENSE-MIT) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this crate by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions. 