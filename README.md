# buttplug-morse
I had really hoped I wouldn't need to explain, but here we are.

I'm not sure how I'll justify this one if a future employer
sees it. Too much aspartame in the diet, maybe? Or I could just say I drank too much. Let's hope it doesn't come to
that.

## What is it?
It's a library that hooks into `buttplug` to play morse code on your butt plug. You can control vibration intensity and
duration if you feel the need. That's really it.

Most of the library's common functionality can be found in the `prelude` module. Generally, something like the below is
all you'll need:
```rs
use buttplug_morse::prelude::*;
```

The core of the library is the `Play` trait, representing something that can be converted to morse signals (dots and
dashes) and played on a `buttplug`-compatible device. For the sake of convenience, there's also a top-level `play`
function that can be used to just chuck some text at a device without really caring.

If you want to expand your own types to be morse-compatible, you should note that `Play` is an *async trait*, so it will
need to be used asynchronously. `buttplug` uses `tokio` as its async runtime by default, so that's probably what you'll
end up using too. Don't fear the reaper, eh?

## But.. why?
I wish I could tell you. I don't know how this happened. I just sort of.. had the idea? And then for some reason I spent
an afternoon making it into a reality. Please forgive me.

## Can I try it?
Yes. `cd` into the `examples` directory, and then run `cargo run`. That's all there is to it!

To run the example you'll need to have Intiface installed and open (or a buttplug server running) with the default
settings. This means an address of `127.0.0.1` and a port of `12345`. Creative I know. And make sure you have a device
or two connected, otherwise, well, it'll do nothing. And that would be boring. If you're not a complete degenerate and
don't *own* a compatible toy, an Xbox gamepad should suffice.

(Don't ask me why. It can vibrate. People will find some way to use it as a sex toy)

## Will you ever be absolved of your guilt?
No, I don't expect I will. Writing these words now, I feel a deep sense of shame hanging over me. I have done
something unforgivable, and I know deep within my heart that trying to undo this atrocity would be futile. A strange
force lurks behind me, driving me to complete this. Even now I know that the days I have left are few. I only hope that
they may be peaceful.

## Rust btw
...

## License
MIT. Yeah.

## Tomato
Tomato

As much as I joke about this, it's pretty cool that the `buttplug` library that this builds on top of even exists in the
first place. And it's pretty cool that we live in a world where I can spend an afternoon getting my vibrator to play
morse code instead of days reverse engineering it. Sure, this was a terrible idea, but it was fun. Thanks for reading to
the end!

Love,  
\- Kaylynn
