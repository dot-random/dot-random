# dot-random
.Random - randomness on Radix ledger


##### Protecting your callback method.
There are two ways to ensure that only RandomComponent can call your callback method:
1. Use `request_random()`, which accepts a bucket, and pass it a token. The bucket will be passed back to you in the callback.  
Effectively you grant access for each particular request.  
See [example.rs](example/src/example.rs).

2. Use `request_random2()`,  which works without a bucket, but you are expected to protect your callback with `method_auth` and require our special badge.  
See [example_badge_auth.rs](example/src/example_badge_auth.rs).