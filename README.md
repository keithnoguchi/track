# track

[![CircleCI]](https://circleci.com/gh/keithnoguchi/workflows/track)

Twitter realtime trend tracker in [Rust].

[Rust]: https://www.rust-lang.org
[CircleCI]: https://circleci.com/gh/keithnoguchi/track.svg?style=svg

## Pre-requisite

Thanks to [Rust]'s clean design, there is not much you need to make `track`
up and running, as in those two docker files, for [ArchLinux] and [Ubuntu18.04],
respectively.  Just install the standard [Rust] packages and you're good
to go.

[ArchLinux]: Dockerfile.arch64
[Ubuntu18.04]: Dockerfile.ubuntu64

## Environment variables

track uses [Twitter stream APIs] to track the realtime twitter trend.
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

`make run` execute the `carge run`, which dumps the realtime trend of multiple tracks.

```sh
$ make run
    Finished dev [unoptimized + debuginfo] target(s) in 0.05s
     Running `target/debug/track`
TTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTFFF
FFTTTTTTFTTTTTTTTTTTTTTTTTTTTTFTTTTTTTTTTTTTTTTTTTTTTTTFFFFFFFFFFFFTTTTTTTTTTTTTTTTTTTTTTTTTFTTTTTTTTFTT
TTTTTTTTTTTTTTTTFFFFFFFFFFFFFFFFFFFFTTTTTTTTTTTTTTTTTTTFTTFTTTTTTTTFTTTTTTTFTFTTTTTTTTTTTTTTFFFFFFFFFFFF
FFFFFFFFTFTTTTTTTTTTTTTTFFTTTTTTTTTTTTTTTTTTTTTTTTTTTTTT^C
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

## Special Thanks

Thank you so much for those great crates to make track up and running!

- [twitter-stream]: A library for listening on Twitter Streaming API

[twitter-stream]: https://docs.rs/crate/twitter-stream/0.8.0

Happy Hacking!
