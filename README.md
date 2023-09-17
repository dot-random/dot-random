# dot-random
.Random - randomness on Radix ledger


### Usage Examples
See [dot-random-examples](https://github.com/Mleekko/dot-random-examples).


##### Protecting your callback method.
There are two ways to ensure that only RandomComponent can call your callback method:
1. Use `request_random()`, which accepts a bucket, and pass it a token. The bucket will be passed back to you in the callback.  
Effectively you grant access for each particular request.  
See an example in [bucket_transfer_auth.rs](https://github.com/Mleekko/dot-random-examples/blob/master/bucket_transfer_auth/src/bucket_transfer_auth.rs).

2. Use `request_random2()`,  which works without a bucket, but you are expected to protect your callback with `method_auth` and require our special badge.  
See an example in  [badge_auth.rs](https://github.com/Mleekko/dot-random-examples/blob/master/badge_auth/src/badge_auth.rs).



#### IDs on RCNet v3.1:
```html
package_tdx_e_1pk3phmd2ux0r0755s2xxfkhsfs9z2ncm3z5vmyqjcmr0zf34hxnx8h
component_tdx_e_1czmysy7cy57af6d8z42dve9pfj0nwy5zhvtvftvm43mpr6uwyp3ggz
resource_tdx_e_1tkkcesj4fdz0tyan29wtc52fzsxajv077wz8f4hmruca9dtx56vvn4
```
