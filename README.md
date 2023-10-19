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
Not deployed yet
```
#### IDs on StokeNet:
```html
package_tdx_2_1pk56nm7yuy3dcjx6awtj72ykx5grte0vukd0j8vl8algxnphwe8yz7
component_tdx_2_1czgsfkdazhyhrs5238wh5phfk80ky8xzqvjwf7cpxwu76efl9jehcx
resource_tdx_2_1t4hgu0a4tav5ydekqz3zd47r6w8kykcg9u4gsmrwnh5k8ef8uh625f
```
#### IDs in scrypto-test:
```html
package_sim1p5qqqqqqqyqszqgqqqqqqqgpqyqsqqqqxumnwqgqqqqqqycnnzj0hj
component_sim1cqqqqqqqqyqszqgqqqqqqqgpqyqsqqqqxumnwqgqqqqqqycnf7v0gx
resource_sim1t5qqqqqqqyqszqgqqqqqqqgpqyqsqqqqxumnwqgqqqqqqycn38dnjs
```
