# `arduino-plotter`
## API bindings (protocol) with Client/Server crate for interacting with Arduino Serial Plotter

### Arduino Serial Plotter: https://github.com/arduino/arduino-serial-plotter-webapp

[![sponsor-us]](https://github.com/sponsors/LechevSpace)&ensp;[![crates-io]](https://crates.io/crates/arduino-plotter)&ensp;[![docs-rs]](https://docs.rs/arduino-plotter)&ensp;[![build-yml]](https://github.com/LechevSpace/arduino-plotter/actions/workflows/build.yml)

#### Running arduino-serial-plotter webapp

**Arduino Serial Plotter** uses WebSockets to communicate by sending or receiving commands.

Requirements:
- Node v10

```bash
git clone https://github.com/arduino/arduino-serial-plotter-webapp
# or when using ssh:
# `git clone git@github.com:arduino/arduino-serial-plotter-webapp.git`
cd arduino-serial-plotter-webapp
npm i && npm start
```

Default port: **3000**

#### Connecting

The **Arduino Serial Plotter** will send a request to our CLI and the CLI is waiting for a connection on the websocket, this is why you need to open the arduino plotter app after starting the CLI:

`http://localhost:3000` (with default port **3000**)

#### Running example

By default, the examples will run at `trace` log level for the `tracing`
subscriber, however, you can use the `RUST_LOG` env. variable to override it.

##### A minimal example

Running the `minimal` example will give you the most basic use of the crate
but it will not be able to send data or handle change of End of Line messages
from the Arduino Serial Plotter application.

`cargo run --example minimal`

##### A random data generator example

You can use `run` example for a basic usage of the Client and Server:

`cargo run --example run`

Refer to the documentation in the file for more details.
The example does all the basic main functionality that you need and sends
random data to the `arduino-serial-plotter-webapp`:

- Sends initial settings
- Sends Random data with 2 different data lines
- Receives settings from the **Arduino Serial Plotter** and confirms a new End of Line by sending a settings message back to it
- Receives data messages sent from the **Arduino Serial Plotter** UI and logs them using `tracing` to the console

### License
Licensed under either of [Apache License, Version 2.0](./LICENSE-APACHE) or [MIT license](./LICENSE-MIT) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this crate by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions. 

[build-yml]: https://img.shields.io/github/actions/workflow/status/LechevSpace/arduino-plotter/build.yml?branch=main&style=for-the-badge
[crates-io]: https://img.shields.io/crates/v/arduino-plotter?logo=rust&style=for-the-badge
[docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K
[sponsor-us]: https://img.shields.io/github/sponsors/LechevSpace?color=bf3989&label=Sponsor%20us&style=for-the-badge&logoColor=bf3989&logo=data%3Aimage%2Fsvg%2Bxml%3Bbase64%2CPHN2ZyBoZWlnaHQ9IjE2IiB2aWV3Qm94PSIwIDAgMTYgMTYiIHZlcnNpb249IjEuMSIgd2lkdGg9IjE2IiB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciPgogICAgPHBhdGggZmlsbD0iI2JmMzk4OSIgZmlsbC1ydWxlPSJldmVub2RkIiBkPSJNNC4yNSAyLjVjLTEuMzM2IDAtMi43NSAxLjE2NC0yLjc1IDMgMCAyLjE1IDEuNTggNC4xNDQgMy4zNjUgNS42ODJBMjAuNTY1IDIwLjU2NSAwIDAwOCAxMy4zOTNhMjAuNTYxIDIwLjU2MSAwIDAwMy4xMzUtMi4yMTFDMTIuOTIgOS42NDQgMTQuNSA3LjY1IDE0LjUgNS41YzAtMS44MzYtMS40MTQtMy0yLjc1LTMtMS4zNzMgMC0yLjYwOS45ODYtMy4wMjkgMi40NTZhLjc1Ljc1IDAgMDEtMS40NDIgMEM2Ljg1OSAzLjQ4NiA1LjYyMyAyLjUgNC4yNSAyLjV6TTggMTQuMjVsLS4zNDUuNjY2LS4wMDItLjAwMS0uMDA2LS4wMDMtLjAxOC0uMDFhNy42NDMgNy42NDMgMCAwMS0uMzEtLjE3IDIyLjA3NSAyMi4wNzUgMCAwMS0zLjQzNC0yLjQxNEMyLjA0NSAxMC43MzEgMCA4LjM1IDAgNS41IDAgMi44MzYgMi4wODYgMSA0LjI1IDEgNS43OTcgMSA3LjE1MyAxLjgwMiA4IDMuMDIgOC44NDcgMS44MDIgMTAuMjAzIDEgMTEuNzUgMSAxMy45MTQgMSAxNiAyLjgzNiAxNiA1LjVjMCAyLjg1LTIuMDQ1IDUuMjMxLTMuODg1IDYuODE4YTIyLjA4IDIyLjA4IDAgMDEtMy43NDQgMi41ODRsLS4wMTguMDEtLjAwNi4wMDNoLS4wMDJMOCAxNC4yNXptMCAwbC4zNDUuNjY2YS43NTIuNzUyIDAgMDEtLjY5IDBMOCAxNC4yNXoiPjwvcGF0aD4KPC9zdmc%2BCg%3D%3D
