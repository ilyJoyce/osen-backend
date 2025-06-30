# O.S.E.N Web-Backend

- Rust-based
- Serve Log
  - We log which files get served to which IP's, devices, user-agents and at which time

This simple Rust-based Web backend runs the O.S.E.N Webpage  
and does not connect to any other O.S.E.N services  
or store/use any user data, except for:  
date, time, user IP, device OS and User-Agent (eg. Browser).  

## Running yourself

- Move into directory
- Use `Cargo run` to run the backend directly
- Use `Cargo build` to build it for usage after (move the binary after or prepare the code accordingly)

That's it, it just works :)  

### File structure

The Server reads the `index.html` from the directory before the one it is in `(..)`,  
this is also where the index is located for its logic eg. every link originates there:  
`your.adresse.com/index.html` & `your.adresse.com/subsite/sitename.html` etc.

## HTTPS

The recommended way to integrate HTTPS into the backend is a reverse proxy (hence the port 6464)  
Altough we are considering adding let's encrypt support directly in the backend  
This is something for the future tho

For help with setting up a reverse proxy like Nginx,  
refer to the [its wiki](https://docs.nginx.com/nginx/admin-guide/web-server/reverse-proxy/)  

## NOTE!

You need to move this backends folder into the index directory of your own webpage,  
or edit the code to use your own path,  
otherwise it obviously won't work. The web-server is hosted on port: 6464, this can be changed in the main.rs  
or by using the --port {port} argument:  
  
`cargo run -- --port {port}` or `./path/to/binary/osen-backend --port {port}`  

Default ports for http and https are `80` & `443` use these if you dont wish to use a reverse proxy to route your traffic.  
(Note: you still have to use a reverse proxy to manage your SSL certificates)
  
Don't forget to put the backend binary or project into a folder IN your index directory:  
  
`path/to/index/osen-backend/'binary goes here'`
`path/to/index` <-- This is where your index file is located