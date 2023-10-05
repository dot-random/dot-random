# dot-random
.Random - randomness on Radix ledger


#### Uses Scrypto `v1.0.0`
### Usage Examples
See [examples](https://github.com/dot-random/examples).


#### Random - 1Kb dependency


##### Protecting your callback method.
There are two ways to ensure that only RandomComponent can call your callback method:
1. Use `request_random()`, which accepts a bucket, and pass it a token. The bucket will be passed back to you in the callback.  
Effectively you grant access for each particular request.  
See an example in [bucket_transfer_auth.rs](https://github.com/dot-random/examples/blob/master/bucket_transfer_auth/src/bucket_transfer_auth.rs).

2. Use `request_random2()`,  which works without a bucket, but you are expected to protect your callback with `method_auth` and require our special badge.  
See an example in  [badge_auth.rs](https://github.com/dot-random/examples/blob/master/badge_auth/src/badge_auth.rs).



#### IDs on Mainnet:
```html
Not deployed yet
```
#### IDs on StokeNet:
```html
package_tdx_2_1pkfavzgzkjqk8dunuyuuzkkewsy7n2w00hfms2ndmydf02eake86py
component_tdx_2_1cpj7d0zu3rruked2k4lk4y4l523u46vns544kewheppmcysngzraws
resource_tdx_2_1t4u529qgcyd439eldx9x6a93hpaw5f9wp8rkuqe3h88k4lyw3zq934
```
