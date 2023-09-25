# dot-random
.Random - randomness on Radix ledger


#### Uses Scrypto `v1.0.0`
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
package_tdx_e_1p5jt8mth9qfzaj4x6ned23dcss6wudjgxgwyr5uc6f6gja9gwz4nxh
component_tdx_e_1cpdq9kvkhnv2yp53zylvlmq74hp90263hxa3zxewxtsxmpdwqhvsxa
resource_tdx_e_1t4mscfp8qxku6pyxm69w0t2mrt8f5sn7f8ecu5agd299xgxtprg4vy
```
