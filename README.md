friendlyfire/\
│\
├── shared/         # Share protocol/messages\
│\
├── command-center/ # Sender of media, main interaction point for the software\
├── server/         # Websocket relay server\
└── splash-screen/  # Receiver of media, will show media in a splash-screen manner\
│\
└── resources/      # images, assets, icons

AVIF format is unsupported for the splash-screen
I think this will be resolved by [this pr](https://github.com/image-rs/image/issues/2621)
In the mean time, the standard image codec will be PNG I think

Overlay : A layer of rgba that can be composited into a Frame
Frame : Structure drawable onto a Window
Compositor : How to blend multiple Overlay into a Frame
Window : Handles the Window (duh) and the rendering (via OS-level APIs)


**Joining a party**, much like Discord
```mermaid
sequenceDiagram
    autonumber

    box Local
    actor U as User
    participant C as Command-center
    end
    box Remote
    participant S as Server
    actor A as Party-Admin
    end

    A->>+S : Ask for invitation link for party
    S-->>-A : Invitation link
    A-->>U : Invitation link, through another canal
    U->>+C: Connect to party using inviation link
    C->>S: Invitation token
    S-->>C: Accept invitation
    C-->>-U: Show acceptance
```

**Sending Overlays**
```mermaid
sequenceDiagram
    autonumber

    actor U as User
    participant C as Command-center
    participant S as Server
    participant SS as Splash-screen

    U->>+C: Create some overlays
    C-->>S: overlays
    S->>S: Sanitize/Check input
    S-->>SS: overlays
    SS->>SS: Check overlays for errors/corruption
    SS-->>S: Overlays ACK
    S-->>C: Overlays ACK
    C->>U: Show that everyone received the overlays

    SS->>SS: rasterizing overlays
    SS-->>S: Rasterization ACK
    S-->>C: Rasterization ACK
    C->>C: Unlock "Fire" button
    C->>U: Show that everyone rasterized
    U->>C: Click "Fire" button
    C-->>S: "Fire" Message
    S->>S: Check that everyone is indeed ready
    S-->>SS: "Fire" message
    SS->>SS: Show the rasterized overlays
```
