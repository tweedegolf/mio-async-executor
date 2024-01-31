# mio async executor

An example async reactor that uses [`mio`](https://docs.rs/mio/latest/mio/) to await OS events.

Most examples don't actually show how to respond to real events like a socket becoming readable or writable. Mature executors are complex and it is hard to see the essence. This example shows the whole setup in ~250 lines. It is still an example, so simplicity has been chosen over performance in some cases. 

## Funders

This project is funded by the [NLnet Foundation].

<img style="margin: 1rem 5% 1rem 5%;" src="https://nlnet.nl/logo/banner.svg" alt="Logo NLnet"  width="200px" />

[NLnet Foundation]: https://nlnet.nl/
