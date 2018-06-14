bloom-filter-server
===================

> A TCP server to store Bloom filter information written in Rust.


Installation
------------

Install it via **Docker**:

    docker run -it -p 1337:1337 ksm2/bloom-filter-server

The server comes with a very lightweight alpine image (< 10 MB).
    
Or build it with **Rust** and **Cargo**:

    cargo run


Usage
-----

Connect to the server e.g. with **Telnet**:

    telnet 127.0.0.1 1337

Then you can run the following commands.


Commands
--------

* **`ADD`** `<item1>` `[<item2>]` `[<item3>]` `[...]`

  Adds _item1_... to the Bloom filter.
  
  The server returns `OK.` on success.

* **`RM`** `<item1>` `[<item2>]` `[<item3>]` `[...]`

  Removes _item1_ ... from the Bloom filter.
  
  The server returns `OK.` on success.

* **`HAS`** `<item>`

  Checks whether _item_ is contained in the Bloom filter.

  The server returns `Yes.` if so, or `No.` otherwise.

* **`COUNT`** `<item>`

  Counts how often _item_ is contained in the Bloom filter.

  The server returns `<count>.`, with _count_ being the count.

* **`BITS`**

  Returns the binary Bloom filter bits.

  The server returns an octet stream of the Bloom filter (fixed to 32 bytes currently).
