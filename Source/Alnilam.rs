// Copyright (c) TribuFu. All Rights Reserved

//! Discord bot. Listen to musics and playlist with your friends.

#![allow(non_snake_case)]
#![allow(unused_imports)]

use dotenv::dotenv;
use std::env;

pub async fn Main() {
    dotenv().ok();

    // Configure the client with your Discord bot token in the environment.
    let _token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let _prefix = env::var("PREFIX").expect("Expected a prefix in the environment");
}
