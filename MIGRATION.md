# Migration

## `v1.1.0`

This isn't breaking, but it's recommended to specify AmneziaWG version

Currently `amneziawg` enables `amneziawg-1`, but in next major release this flag (`amneziawg`) will be removed.

Either:

- Change to `amneziawg-1`

  ```patch
  -wireguard-conf = { version = "1.0.1", features = ["amneziawg"] }
  +wireguard-conf = { version = "1.1.0", features = ["amneziawg-1"] }
  ```

- Migrate to `amneziawg-2` (this will break old configs, due to `H` values now being ranged)

  ```toml
  wireguard-conf = { version = "1.1.0", features = ["amneziawg-2"] }
  ```

## `v1.0.0`

First major release brought many changes. Breaking changes are builder structure and `Peer::to_interface` signature.

### Builders & models themselves

Now they are generated with `derive_builder`.

1. Removal of `set_*` and `add_*` builder methods.
   ```diff
    let interface = InterfaceBuilder::new()
   -    .add_dns("1.1.1.1".to_string())
   -    .add_dns("1.0.0.1".to_string())
   -    .set_peers(vec![peer1, peer2])
   -    .add_peer(somepeer)
   -    .add_post_up("rm -rf /".to_string())
   +    .dns(["1.1.1.1".to_string(), "1.0.0.1".to_string()])
   +    .peers([peer1, peer2, somepeer])
   +    // or .peers(vec![peer1, peer2, somepeer])
   +    .post_up(["rm -rf /".to_string()])
        .build();
   ```
2. Support for multiple addresses
    ```diff
     let interface = InterfaceBuilder::new()
    -    .address(as_ipnet!("10.0.0.1/24"))
    +    .address([as_ipnet!("10.0.0.1/24")])
    +    // or .add_network(as_ipnet!("10.0.0.1/24"))
         .build();
    ```

### `Peer::to_interface()`

Now this method requires additional 2nd argument `ToInterfaceOptions`.

```diff
 let client_interface = client_peer.to_interface(
     &server_interface,
+    ToInterfaceOptions::new(),
 );
```

With these options, you skip writing boilerplate.

- Default gateway
  ```diff
   let client_interface = client_peer.to_interface(
       &server_interface,
  +    ToInterfaceOptions::new()
  +        .default_gateway(true),
   );

  -client_interface.peers[0].allowed_ips = vec![as_ipnet("0.0.0.0/0")];
  ```

- `PersistentKeepalive`
  ```diff
   let client_interface = client_peer.to_interface(
       &server_interface,
  +    ToInterfaceOptions::new()
  +        .persistent_keepalive(25),
   );

  -client_interface.peers[0].persistent_keepalive = 25;
  ```
