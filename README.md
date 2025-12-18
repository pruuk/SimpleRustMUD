This is a *very* simple Multi-User-Dungeon (MUD) client written in the Rust language.

Only a small number of commands are currently implemented, but the telnet client is up and working.

To login in:
1. clone the repo
2. cargo build
3. cargo run
4. use a telnet client to telnet to: localhost port 4000
5. Register a new user or login as "admin" with a password "12345"
6. type "help" to see commands available

Only the admin can use the admin commands.

Future expandsion could include:
1. Fleshing out the websocket to run a web page interface
2. Implementing item manipulation
3. Add combat
