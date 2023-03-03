The name is a bit weird but I found it funny because it is centered around housing.

What I want to do with this is run a few things in my TV and sound system at my home. I stopped using Chromium based browsers so it's a pain in the ass to
stream things from my computer's youtube to my tv. I ultimately want this to work with my Google Nest too but for now it is good since it can play youtube
videos through the WS that my TV exposes in my local network.

TODO:

- Do not hardcode the TV IP and issue an M-SEARCH (?) to find it in the local network
- Accept commands while the websocket client is running
- Include a search for YouTube videos, at the moment I can only paste whole youtube links
- The initial HELLO payload has a client-key hardcoded in there to avoid my TV's request for pairing every time. I will be moving this to SQLite
- Modularize
