# an_admin_a_day
## Description
Can't agree on who should be the server owner? How about everyone?

This project will rotate the crown around all the members of a brand new server, giving you the opportunity to change each others' nicknames, make silly rules, or restructure the entire server to your every whim. Whatever you want -- as long as it's y our turn!

## How to build:
1) `cargo run --bin setup`
2) `cargo run --bin switch_admins`
3) ???
4) profit

## How to run:
1) go to the Discord developer portal
2) Make an application, then a bot
3) go get the bot's token, export it into your environment
4) run setup, and then switch_admins as needed (i have systemd run it once a day)

## TODO
* add customizability for repeat admins or something
* ~~scheduling system, for example 8am-8am~~
* ~~announce new administrator in channel (or DM new administrator)~~
* ~~change server icon to the administrator's PFP~~
* bot PFP
* generate `.service`, `.timer`, and `.sh` files for use in automation, using the setup program.
