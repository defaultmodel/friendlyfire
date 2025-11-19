```
friendlyfire/
│
├── shared/         # shared protocol / message types / utils
│
├── server/         # TCP/WebSocket relay server
├── receiver/       # splash-screen receiver
├── command_center/ # controller
│
└── resources/          # images, assets, icons
``

AVIF format is unsupported for the splash-screen
I think this will be resolved by [this pr](https://github.com/image-rs/image/issues/2621)
