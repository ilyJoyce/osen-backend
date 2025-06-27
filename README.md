# O.S.E.N Web-Backend

- Rust-based
- Serve Log
  - We log which files get served to which IP's, devices and user-agents

This simple Rust-based Web backend runs the O.S.E.N Webpage
and does not connect to any other O.S.E.N services
or store/use any user data, except for:
the users IP, device OS and User-Agent (eg. Browser).

## Running yourself

- Move into directory
- Use 'Cargo run' to run the backend directly
- Use 'Cargo build' to build it for usage after

That's it, it just works :)

### NOTE!

You need to have to move this backends folder into the index directory of your own webpage,
otherwise it obviously won't work. The web-server is hosted on port: 6464, this can be changed in the main.rs.