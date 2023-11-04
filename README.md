# dot-random
.Random - randomness on Radix ledger


#### Uses Scrypto `v1.0.1`
### Usage Examples
See [examples](https://github.com/dot-random/examples).


#### Random - 1Kb dependency


##### Protecting your callback method.
There are two ways to ensure that only RandomComponent can call your callback method:
1. Pass a bucket with some token to `request_random()`. Then you can verify that the same bucket is returned back to you in the callback.  
Effectively you grant access for each particular request.  
See an example in [bucket_transfer_auth.rs](https://github.com/dot-random/examples/blob/master/bucket_transfer_auth/src/bucket_transfer_auth.rs).

2. When you chose not to use your own token, you are expected to protect your callback with `method_auth` and require our special badge.  
See an example in  [badge_auth.rs](https://github.com/dot-random/examples/blob/master/badge_auth/src/badge_auth.rs).



#### IDs on Mainnet:
```html
package_rdx1p55tuj30yf842s6cjraqz5arhtf98jcjtmkjcmxrn6efvvc829g2jf
component_rdx1cqz6m403yq9xzqj7g5ujq3yd6w0ge8shur53z8754gj8rxde8xd0sr
resource_rdx1thufp23mqn3hefdza383tk2fxs3rvwv97djzq8x5czzdqrgkc807wj
```
#### IDs on StokeNet:
```html
package_tdx_2_1p527rqesssgtadvr23elxrnrt6rw2jnfa5ke8n85ykcxmvjt06cvv6
component_tdx_2_1czzxynn4m4snhattvdf6knlyfs3ss70yufj975uh2mdhp8jes938sd
resource_tdx_2_1t59tdtsvv7sc0nej3z585w5nmqpq3z5cms7xdwvkyqaqreu9j3rvyu
```
#### IDs in scrypto-test:
```html
package_sim1p5qqqqqqqyqszqgqqqqqqqgpqyqsqqqqxumnwqgqqqqqqycnnzj0hj
component_sim1cqqqqqqqqyqszqgqqqqqqqgpqyqsqqqqxumnwqgqqqqqqycnf7v0gx
resource_sim1t5qqqqqqqyqszqgqqqqqqqgpqyqsqqqqxumnwqgqqqqqqycn38dnjs
```
