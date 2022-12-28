use futures_util::StreamExt;
use twilight_http;
use std::{env, process::exit, fs, fs::*};
use twilight_gateway::{Event, Intents, Shard};
use twilight_model::{
    gateway::{
        payload::outgoing::UpdatePresence,
        presence::{
            Activity, ActivityType, MinimalActivity, Status
        }
    }, 
    id::Id,
    guild::Permissions
};
use twilight_http::Client;
use std::io;
use std::io::Write;


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize the tracing subscriber.
    tracing_subscriber::fmt::init();

    let token = env::var("DISCORD_TOKEN")?;
    // To interact with the gateway we first need to connect to it (with a shard or cluster)
    let (shard, mut events) = Shard::new(
        token.to_owned(),
        Intents::GUILD_MEMBERS | Intents::GUILDS,
    );

    let client = Client::new(token.to_owned());

    shard.start().await?;
    println!("Created shard");

    while let Some(event) = events.next().await {

        // i'm not sure if this is the best way to wait for this, but it's what
        // the example gave me

        match &event {
            Event::Ready(_) => {

                println!("Ready for action!");

                let minimal_activity = MinimalActivity {
                    kind: ActivityType::Playing,
                    name: "Russian roulette but with people instead of bullets".to_owned(),
                    url: None,
                };
                let command = UpdatePresence::new(
                    Vec::from([Activity::from(minimal_activity)]),
                    false,
                    Some(1),
                    Status::Online,
                )?;

                shard.command(&command).await?;
                println!("Status set!");

                let mut new_file = match File::options().read(true).write(true).create_new(true).open("serverid.txt") {
                    Ok(file) => file,
                    Err(_) => {
                        let server_id: u64 = fs::read_to_string("serverid.txt")?.parse().expect("crap");

                        println!("Looks like you already have a server. Press enter to continue or \"RESTART WITH NEW SERVER\" to restart with a new server.");
            
                        let mut response = String::new();
            
                        io::stdin()
                            .read_line(&mut response)
                            .expect("Something went wrong reading input");
            
                        //let destroy_server = response.eq("RESTART WITH NEW SERVER");
            
                        println!("Response: {response}");//, {destroy_server}");//

                        if response.trim() == "RESTART WITH NEW SERVER" {
                            let discord_response = client
                                .delete_guild(Id::new_checked(server_id).unwrap())
                                .await?;

                            println!("Response from discord: {discord_response:?}");
                            println!("Destroyed old server!");

                            fs::remove_file("serverid.txt").unwrap();

                            
                        }

                        File::options().read(true).write(true).create_new(true).open("serverid.txt")? //better not error pls
                        
                    }
                };

                let new_guild = client
                    .create_guild(String::from("Brand New Server"))
                    .expect("Invalid Name!")
                    .await?
                    .model()
                    .await?;
                    
                let new_system_channel_id = new_guild.system_channel_id.expect("Crap, couldn't get the system channel ID");
                println!("Guild created!");

                //save the server id
                new_file.write_all(&new_guild.id.get().to_be_bytes())?;
                println!("Guild ID saved");

                // This doesn't return the code
                let new_invite = client.create_invite(new_system_channel_id).await?;
                // Get the code from here instead
                let new_channel_id = new_invite.model().await?.channel.expect("oops no channel").id;
                let channel_invites = client.channel_invites(new_channel_id).await?;
                let new_invite_code = &channel_invites.model().await?[0].code;

                println!("Invite code: discord.gg/{new_invite_code}");

                let admin_permission = Permissions::ADMINISTRATOR;

                let admin_role = client.create_role(new_guild.id)
                    .name("The Administrator")
                    .permissions(admin_permission)
                    .await?;

                let admin_role_id = admin_role.model().await?.id;

                let all_role_ids = vec![admin_role_id];

                //TODO
                // wait for everyone to join the server, have slash command to start the administration
                // (destroy slash command after use)
                // include options like period of role changing

                println!("Administrator role created. Waiting for first person to join");

                while let Some(event) = events.next().await {
                    match &event {
                        Event::MemberAdd(member) => {
                            let _member = client
                            .update_guild_member(
                                new_guild.id, 
                                member.user.id
                            )
                            .roles(&all_role_ids)
                            .await?
                            .model()
                            .await?;

                            //TODO this should be running like 24/7 and changing the admin periodically
                            println!("Administrator assigned, quitting program");
                            exit(0);
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }

    Ok(())
}