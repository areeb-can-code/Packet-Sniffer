# 👃 Smelly Packs 👃

> To Note this progam has not tested on Linux. Packet headers and lengths are different, so when I get the time I will adjust the program for both versions.

> To get Windows to work, Wpcap and its required libraries are needed. Linking this is a pain so I\'ve provided the lib files for you to place in the relevant crates.

For further documentation: feel free to look at this
https://docs.google.com/document/d/1NkGOZDVPglB4L6P7EOpB9ycqhHF2H3RxN193PGxkMwM/edit?usp=sharing

This project is a very simple terminal program to capture packets that are being sent and received by your device. In the current stage of this application, there are plans to use a DNS resolver and port scans for better smelling ;)
Compiled with `rustc 1.65.0` || `cargo 1.65.0`. Any other builds or toolchains not tested...

### To compile this program successfuly, add these to your cargo.toml file:
* `pcap = "1.0.0"`
* `etherparse = "0.12.0"`
* `online = "4.0.0"`
* `libc = "0.2.92"`

If you have the desire to run this on Linux, you will need the `libpcap` libary. However because packets are dealt differently on linux, it requires you to use an older version as the `libc` library will not work with it due to errors. `libpcap` hasn't been updated so running on Linux requires more work to set up. In a future version, I will allow this to be multiplatform.


If you are running on linux you will need to run the following command to allow your device to capture packets in a protected folder. Windows handles raw packets differently.

`sudo setcap cap_net_raw,cap_net_admin=eip target/debug/smellyPacks`

### Running the program

To run the program you can either `cargo run` or go into the ```target/debug/``` and run the relevant binary with the relevant arguments. Check out the arguments below or write `h` or `help

##### Modes (required)

There are currently 4 different modes to run this program:
* `0` - print each packet individually on the screen
* `1` - print each packet sequentially on the screen
* `2` - print each packet individually and include the hex data on the screen
* `3` - print each packet sequentially and include the hex data on the screen

The argument flag is `-m`.

##### Filename (required)

Pass in the name of the file to which Smelly Packs will save the captured packets in pcap format. There will be a future implementation that allows you to read the pcap file within the program.

The argument flag is `-o`.

##### Count (Not Required)

This allows you to run the Smeller for longer than the default of 100 packets. I would recommend running for a small amount of time since this program is very buggy :(

The argument flag is `-c`.
#### Example

```$ > cargo run -- -m 1 -o output -c 5```
The output would be something like:
```

Using device wlan0
Num:       | Time:                | Source:                                  | Destination:                             | Protocol:  | Length:
=================================================================================================================================================
1          | 1609421360.325228    | 192.168.0.13                             | 192.168.0.11                             | TCP        | 176
2          | 1609421360.325347    | 192.168.0.11                             | 192.168.0.13                             | TCP        | 66
3          | 1609421360.523991    | 192.168.0.11                             | 199.232.57.140                           | TCP        | 66
4          | 1609421360.567946    | 199.232.57.140                           | 192.168.0.11                             | TCP        | 66
5          | 1609421360.715769    | 192.168.0.11                             | 151.101.37.140                           | TCP        | 66

```
