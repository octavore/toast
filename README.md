# toast

Monitor system temperature throttling on the command line.

## installation

With homebrew, run `brew install octavore/tap/toast`

Or clone and install with `cargo install --locked --path .`

## example usage

```
toast              # print thermal state
toast watch        # print thermal state continuously on a 5 second loop
toast watch --bar  # same as above but also render a bar chart
```

## credits

This was inspired by MacThrottle and Stan's [excellent writeup](https://stanislas.blog/2025/12/macos-thermal-throttling-app/) ([hn](https://news.ycombinator.com/item?id=46410402)).

Based on the article, I also referenced the [notify_get_state](https://developer.apple.com/documentation/darwinnotify/notify_get_state) docs, the [OSThermalNotification.h](https://github.com/tripleCC/Laboratory/blob/a7d1192f25d718e3b01a015ca35bfcef4419e883/AppleSources/Libc-1272.250.1/include/libkern/OSThermalNotification.h#L44-L48) header file, and this earlier post on [macOS thermals](https://dmaclach.medium.com/thermals-and-macos-c0db81062889) by Dave MacLachlan.
