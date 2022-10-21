# Keep my JCloud!

## Intro

[Explainer video on YouTube!](https://www.youtube.com/watch?v=yJvBKjqns7s)

Since launching [Thingy](https://github.com/peterwilli/Thingy) on [JCloud](https://docs.jina.ai/fundamentals/jcloud/), we had moments our instances randomly disappeared.

Since Jina is still testing its services, there's no harm in this, but I wanted to be able to run things in what I call "quasi-production" (as in: It's live, but people know it can get offline).

Keep my JCloud allows anyone to get a single URL to access their JCloud instances, re-spawn them when they go offline, and even re-route traffic to your own devices if things really don't work!

## Features

- Keep the same URL for your flow (no random URL)
- Simple to integrate: All you need to do is parse the response to get the URL to forward your request to.
- Safe: Uses [the official](https://docs.jina.ai/fundamentals/jcloud) `jcloud` CLI in the background and runs locally. No need to provide your account details to third parties!
- Lightweight server written in Rust. No heavy runtimes, can run on ARM (Raspberry Pi, Odroid) too
- Protocol agnostic: Works with JCloud GRPC, HTTP, WebSocket
- Open: Allows you to use your own (Jina-based) instances in case JCloud doesn't work, or even run your own instance entirely, without having to rewrite the connection in your app.

## How to run

- Make sure you have Rust installed: https://rustup.rs
- Run `cargo install --git https://github.com/peterwilli/KeepMyJCloud.git --branch=release`
- Now you can run `keep_my_jcloud`! (See [Examples](#examples) below)
- Sending a request to `http://localhost:8000` will give you something like: `{ endpoint: grpcs://527fae43ba.wolf.jina.ai }`. You can use your application to link to that URL. See [Thingy's JCloudClient.kt](https://github.com/peterwilli/Thingy/blob/8a925b93f121620b799d2b30e494e5f59154c35a/src/main/kotlin/utils/JCloudClient.kt) for a real-world example.

# Examples

We set project name to `myservice` but you're encouraged to set unique names per project to make sure your instances are correctly tracked!

- Keep a JCloud flow online (if `myservice` already exists, use that instance, i.e. keeping track on an already running instance, if none exists, re-deploy one):
    - `keep_my_jcloud --project-name=myservice --flow-yml-path=/path/to/flow.yml`
- Keep a JCloud flow online, with fallback to your own instance:
    - `keep_my_jcloud --project-name=myservice --flow-yml-path=/path/to/flow.yml --alternate-url=grpcs://emeraldsarecool.ai:51001`