# track

[![DroneCI]](https://cloud.drone.io/keithnoguchi/track)
[![CircleCI]](https://circleci.com/gh/keithnoguchi/workflows/track)

A real-time [Twitter] trend tracker in [Rust].

Upon running this application, `track` will compare the two provided
channel keywords streaming on the [Twitter] platform to judge
a comparison of the two topics in terms of popularity and presence
on the [Twitter] platform.

- [Design](#design)
- [Prerequisite](#prerequisite)
  - [Key and Token](#key-and-token)
- [Test](#test)
- [Execution](#execution)
- [Installation](#installation)
- [To-do](#to-do)
- [Special Thanks](#special-thanks)

```sh
air2$ track -h
Usage: track [-h] [<first track name> <second track name>]

A real-time Twitter trend tracker in Rust.

Upon running this application, track will compare the two provided
channel keywords streaming on the Twitter platform to judge
a comparison of the two topics in terms of popularity and presence
on the Twitter platform.

'twitter' and 'facebook' are the default keywords.  You can override
those through the command line arguments, e.g. 'track love food'.

Here is the list of currently supported keywords:

twitter
facebook
google
travel
art
music
photography
love
fashion
food
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

[![AsciiCast]](https://asciinema.org/a/276420)

[Twitter]: https://twitter.com
[Rust]: https://www.rust-lang.org
[DroneCI]: https://cloud.drone.io/api/badges/keithnoguchi/track/status.svg
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
up and running, as in those two docker files, for [ArchLinux] and
[Ubuntu18.04], respectively.  Just install the standard [Rust] packages and
you're good to go except one thing, [Key and Token](#key-and-token).

Let's take care of that quick before the party starts.

[ArchLinux]: Dockerfile.arch64
[Ubuntu18.04]: Dockerfile.ubuntu64

### Key and Token

track uses [Twitter stream APIs] to track the real-time twitter trend.
To do that, you need to provide the consumer key and the access token
through the environment variables.  You can request key and token
through [Twitter developer site] under [Apps section]:


```sh
$ export TRACK_CONSUMER_KEY=your_consumer_key
$ export TRACK_CONSUMER_SECRET=your_consumer_secret
$ export TRACK_ACCESS_TOKEN=your_access_token
$ export TRACK_ACCESS_SECRET=your_access_token_secret
```

[Twitter stream APIs]: https://developer.twitter.com/en/docs/tweets/filter-realtime/api-reference/post-statuses-filter
[Twitter developer site]: https://developer.twitter.com/
[Apps section]: https://developer.twitter.com/en/apps

## Test

`make test` is a wrapper of `cargo test`.  I'll add more tests along the way.

```sh
$ make test
   Compiling track v0.1.1 (/home/kei/git/track)
    Finished dev [unoptimized + debuginfo] target(s) in 1.29s
     Running target/debug/deps/track-386d81d60ee5b79d

running 5 tests
test config::tests::default_tracks ... ok
test config::tests::delay_in_msec ... ok
test config::tests::sample_count ... ok
test config::tests::total_count ... ok
test event::tests::from_str ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

     Running target/debug/deps/track-c4ec65894b1782cc

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

   Doc-tests track

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

$
```

## Execution

`make run` is the wrapper of `cargo run`.  It shows the current real-time
trend of two default keywords, 'twitter' and 'facebook'.

```sh
$ make run
    Finished dev [unoptimized + debuginfo] target(s) in 0.05s
     Running `target/debug/track`
Twitter trend:     ##----------------------------------      35/1000    [00:00:19]
Facebook trend:    ####--------------------------------      85/1000    [00:00:19]
^C
```

Currently, `track` only checks the two trend simultaneously.  This is because
[Twitter streaming APIs] only support two concurrent TCP session from the same
source, due to the rate limitting.  One way to overcome this limitation is to
run the workers on different machines.

Here is the `netstat` output while running the the modified version of `track`,
which spawns 10 workers.  As you can see, there are only two established
TCP sessions created by `track`.

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

## Installation

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

- [ ] More unit tests
- [ ] Graceful shutdown
- [ ] Remove keyword limit
- [ ] Responsive output for less popular keywords
- [ ] Support more than two tracks
- [ ] Documentation

## Special Thanks

Thank you so much for those great crates to make track up and running!

- [The book]: The Rust Programming Language
- [twitter-stream]: A library for listening on Twitter Streaming API
- [indicatif]: A Rust library for indicating progress in command line apps

[The book]: https://github.com/rust-lang/book/
[twitter-stream]: https://docs.rs/crate/twitter-stream/0.8.0
[indicatif]: https://crates.io/crates/indicatif

Happy Hacking!
