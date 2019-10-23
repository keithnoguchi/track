# track

[![CircleCI]](https://circleci.com/gh/keithnoguchi/workflows/track)

Real time [Twitter] trend tracker in [Rust].

```sh
$ track -h
Usage: track [-h] [<first track name> <second track name>]

'twitter' and 'facebook' are the default track names.  You can override
those through the command line arguments, e.g. track love food.

Here are the list of supported tracks:

art
facebook
fashion
food
google
love
music
photography
travel
twitter
```
```sh
$ track
Twitter trend:     ######################--------------     611/1000    [00:00:10]
Facebook trend:    #######-----------------------------     186/1000    [00:00:11]
```
```sh
$ track love food
Love trend:        #################-------------------     457/1000    [00:00:12]
Food trend:        ##----------------------------------      52/1000    [00:00:12]
```

- [Design](#design)
- [Prerequisite](#prerequisite)
- [Run](#run)
- [Test](#test)
- [Install](#install)
- [To-do](#to-do)
- [Special Thanks](#special-thanks)

[![AsciiCast]](https://asciinema.org/a/276420)

[Twitter]: https://twitter.com
[Rust]: https://www.rust-lang.org
[CircleCI]: https://circleci.com/gh/keithnoguchi/track.svg?style=svg
[AsciiCast]: https://asciinema.org/a/276420.svg

## Design

Here is the high level design diagram of the `track`.  It's based
on the standard worker pattern, which runs multiple `track::Workers`
to retrieve the specific trend through [twitter-stream] [Rust]
crate and send it through the `std::sync::mpsc` channel to report
it back to the `track::Tracker` aggregator.  `track::Tracker`
creates a dedicate `std::thread` to report the live update through
[indicatif] [Rust] crate.  Since the communication between
`track::Tracker` and `track::Worker` is over `std::sync::mpsc` channel,
it can easily add more workers to support multiple tracks.

But due to the [Twitter stream API] rate limiting, you can't have
more than two TCP sessions from the same IP.  To overcome this
challenge, we'll move to the distributed design by running those
workers on a different machines, as mentioned in [to-do](#to-do).

```
+---------------------------------------------------------------+
|                         indicatif crate                       |
+---------------------------------------------------------------+
+---------------------------------------------------------------+
|                          track::Tracker                       |
+---------------------------------------------------------------+
                                ^
                                |
                 +------------------------------+
                 |    std::sync::mpsc channel   |
                 +------------------------------+
                     ^                       ^
                     |                       |
+--------------------+---------+ +-----------+------------------+
| Twitter track track::Worker  | | Facebook track track::Worker |
+------------------------------+ +------------------------------+
+---------------------------------------------------------------+
|                     twitter-stream crate                      |
+---------------------------------------------------------------+
```

## Prerequisite

Thanks to [Rust]'s clean design, there is not much you need to make `track`
up and running, as in those two docker files, for [ArchLinux] and [Ubuntu18.04],
respectively.  Just install the standard [Rust] packages and you're good
to go.

[ArchLinux]: Dockerfile.arch64
[Ubuntu18.04]: Dockerfile.ubuntu64

## Environment variables

track uses [Twitter stream APIs] to track the real time twitter trend.
To do that, you need the consumer and access keys set through the
environment variable.  You can request yours through [Twitter developer site]:


```sh
$ export TRACK_CONSUMER_KEY=your_consumer_key
$ export TRACK_CONSUMER_SECRET=your_consumer_secret
$ export TRACK_ACCESS_TOKEN=your_access_token
$ export TRACK_ACCESS_SECRET=your_access_token_secret
```

[Twitter stream APIs]: https://developer.twitter.com/en/docs/tweets/filter-realtime/api-reference/post-statuses-filter
[Twitter developer site]: https://developer.twitter.com/

## Run

`make run` execute the `cargo run`, which dumps the real time trend of multiple tracks.

```sh

$ make run
    Finished dev [unoptimized + debuginfo] target(s) in 0.05s
     Running `target/debug/track`
Twitter trend:     ##----------------------------------      35/1000    [00:00:19]
Facebook trend:    ####--------------------------------      85/1000    [00:00:19]
^C
```

Currently, `track` only tracks two trends, Twitter and Facebook, as in [config.rs],
due to the [420 HTTP error].  Here is the `netstat` output and it only allows
two concurrent TCP sessions.

```sh
$ netstat -cntp
(Not all processes could be identified, non-owned process info
 will not be shown, you would have to be root to see it all.)
Active Internet connections (w/o servers)
Proto Recv-Q Send-Q Local Address           Foreign Address         State       PID/Program name
tcp        0      0 192.168.255.198:36486   199.59.150.42:443       ESTABLISHED 6962/target/debug/t
tcp        0      0 192.168.255.198:36488   199.59.150.42:443       ESTABLISHED 6962/target/debug/t
^C
```

[config.rs]: src/config.rs
[420 HTTP error]: https://developer.twitter.com/en/docs/basics/response-codes

## Test

Only two tests, but hey, it's still day one. :)

```sh
$ make test
    Finished dev [unoptimized + debuginfo] target(s) in 0.04s
     Running target/debug/deps/track-00858257248e53cb

running 2 tests
test config::tests::default_track_entries ... ok
test config::tests::delay_in_sec ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

     Running target/debug/deps/track-88188b4c62ebad95

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

   Doc-tests track

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

## Install

make install will call `cargo install --force --path .` to install `track`
into your default cargo executable path:

```sh
$ make install
  Installing track v0.1.1 (/home/kei/git/track)
    Updating crates.io index
    Finished release [optimized] target(s) in 0.29s
   Replacing /home/kei/.cargo/bin/track
    Replaced package `track v0.1.1 (/home/kei/git/track)` with `track v0.1.1 (/home/kei/git/track)` (executable `track`)
```

## To-do

Here is the list of to-dos:

- [ ] Support more than two tracks
- [ ] More unit tests
- [ ] Documentation
- [ ] Better ownership handling with Drop trait support
- [ ] Better summary output

## Special Thanks

Thank you so much for those great crates to make track up and running!

- [twitter-stream]: A library for listening on Twitter Streaming API
- [indicatif]: A Rust library for indicating progress in command line apps

[twitter-stream]: https://docs.rs/crate/twitter-stream/0.8.0
[indicatif]: https://crates.io/crates/indicatif

Happy Hacking!
