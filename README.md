# GSICONFIG-CLI
This command line tool allows the creation and configuration of `.cfg` files required by Valve for Dota 2 and CS:GO Game State Integration.

Some resources:
* [CS:GO GSI Wiki](https://developer.valvesoftware.com/wiki/Counter-Strike:_Global_Offensive_Game_State_Integration)
* [xzion/dota2-gsi](https://github.com/xzion/dota2-gsi)
* [antonpup/Dota2GSI](https://github.com/antonpup/Dota2GSI)

---

## Usage

*to-do*

---

## Format of `.cfg` files

The required format is *kind of* similar to JSON, but not quite. I'm not sure if it has a well-known name, or used elsewhere.

Here is a sample configuration file for Dota 2:
```
"Dota 2 Integration Configuration"
{
    "uri"           "http://localhost:4000/"
    "timeout"       "5.0"
    "buffer"        "0.1"
    "throttle"      "0.1"
    "heartbeat"     "30.0"
    "data"
    {
        "provider"      "1"
        "map"           "1"
        "player"        "1"
        "hero"          "1"
        "abilities"     "1"
        "items"         "1"
    }
}
```

---

### Why a CLI tool?

**Why can't I just create a `.cfg` file by hand?**

This was an excuse for me to make a small foray into Rust and have something to show for it. I can't see this being very useful, but I'd love to be surprised.