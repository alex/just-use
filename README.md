# just-use

Because you should Just Use `/dev/urandom` (TM) [^1]. This has been the correct advice for generating random numbers on Linux (and other Unixes) for a long time, however there's one situation in which it's not correct: At early boot.

`/dev/urandom` will hapilly return data before the kernel's random number generator has been fully seeded. This kernel module solves that. It will refuse to return random numbers before it can do so safely -- after that point (which is generally hit in early boot), it'll never block. It's a completely safe drop-in replacement [^2] for the kernel's existing `/dev/urandom` as a result. You can also use it to replace `/dev/random`.

Usage:

```console
$ make
$ sudo insmod justuse.ko
$ # Copy down the device number from here
$ grep "justuse" /proc/devices
$ sudo mknod /dev/urandom c $DEVICE_NUMBER 0
$ sudo mknod /dev/random c $DEVICE_NUMBER 0
```

And that's it! Include in your system boot configuration to ensure that your random numbers are great on every boot!

Currently only builds on x86-64, but if you're interested in other architectures, please file an issue and we'll make it happen! Also it's written in Rust (currently requires a nightly Rust), so memory safety!

[^1]: Nowadays you should probably Just Use `getrandom(2)`, but that's besides the point.
[^2]: We're actually missing one compatibility feature: `poll()` support, when used as `/dev/random`.