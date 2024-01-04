# All TLDs with working websites
I was interested in looping over every single TLD and checking if they have working sites.  
This program makes a `GET request` and `DNS lookup` for every TLD and returns the working ones.

## Results (as of 2023-1-3)
### GET requests
The program showed that no TLD returns a webpage and none can be viewed in a browser

### DNS lookups
The results of this one were quite surprising. I found 11 TLDs with attached A or AAAA records and they were:
```rust
[
    ("AI", "209.59.119.34"),
    ("ARAB", "127.0.53.53"),
    ("CM", "195.24.205.60"),
    ("MUSIC", "127.0.53.53"),
    ("TK", "217.119.57.22"),
    ("UZ", "91.212.89.8"),
    ("VA", "2a01:b8:0:1:212:77:0:2"),
    ("WS", "64.70.19.33"),
    ("XN--L1ACC", "180.149.98.234"),
    ("XN--MXTQ1M", "127.0.53.53"),
    ("XN--NGBRX", "127.0.53.53"),
]
```
I entered all IPs into the browser, but the only one that worked as AI, the ccTLD of Anguilla: `209.59.119.34`

The IP address `127.0.53.53` occurred 4 times, which is quite interesting. That is a local IP address, which means that it will only work locally on one machine. So, theoretically, you can set up a webserver to listen to `127.0.53.53`, and open it by simply typing in `arab/` or `music/` in Chrome's address bar, even though I couldn't get it to work
