

reference  heap allocator for Cortex-M processors. [alloc-cortex-m](https://github.com/rust-embedded/alloc-cortex-m), implement a heap allocator for avr processors. 

# attention

- in you app, at least one device is selected in your app that use this crate.

- need avr-device version matching. 

this sample shows a vaild dependencies description:

```toml
[dependencies]

# its related avr-device version is "0.3" 
avr-allocator ={path = "../../../avr-allocator/" }

# notes, the version of avr-device used in arduino-hal, must be equal to the version of avr-device used in avr-allocator.
# below arduino-hal rev, its inner related avr-device version is "0.3"
arduino-hal = {git="https://github.com/rahix/avr-hal",rev="1aacefb335517f85d0de858231e11055d9768cdf",features = ["arduino-nano"]}
```
