# `postgres-deno`

![Postgres Deno](./assets/pldeno.png)

A PostgreSQL extension for Deno: run Typescript in PostgreSQL functions and triggers.

**Contents** 

- [About](#about)
- [Status](#status)
- [FAQ's](#faqs)
  - [What is Deno?](#what-is-deno)
  - [Why use this over plv8?](#why-use-this-over-plv8)
- [Credits](#credits)


## About

This is a PostgreSQL extension for [Deno](https://deno.land). It is still a proof of concept. This will theoritically allow users to create database functions and triggers in TypeScript. 

## Status

Still a Proof of concept! We're developing in public.

- [X] POC: this is a concept. We're testing to see if it's even possible.
- [ ] Alpha: Under heavy development.
- [ ] Beta: Ready for use. But go easy on us, there may be a few kinks.
- [ ] 1.0: Use in production!


## FAQ's 

### What is Deno?

Deno is a simple, modern and secure runtime for JavaScript and TypeScript that uses V8 and is built in Rust. It was created by Ryan Dahl, original creator of Node.js, and is focused on productivity.

### Why use this over plv8?

plv8 requires you to install node packages on your server. Deno can import modules from any location on the web, like GitHub, or a CDN like [Skypack](skypack.dev) or [jspm.io](jspm.io).


## Credits

Logo derived from the orignal Deno icon and an icon made by [Freepik](https://www.flaticon.com/authors/freepik).


![Watch this repo](https://gitcdn.xyz/repo/supabase/monorepo/master/web/static/watch-repo.gif "Watch this repo")
