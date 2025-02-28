# 0.15.0 [unreleased]

- Add `WebRTC` instance for `Multiaddr`. See [PR 59].
- Add `Certhash` instance for `Multiaddr`. See [PR 59].

- Add support for Noise protocol. See [PR 53].

- Use `multibase` instead of `bs58` for base58 encoding. See [PR 56].

[PR 53]: https://github.com/multiformats/rust-multiaddr/pull/53
[PR 56]: https://github.com/multiformats/rust-multiaddr/pull/56
[PR 59]: https://github.com/multiformats/rust-multiaddr/pull/59

# 0.14.0 [2022-02-02]

- Add support for TLS protocol (see [PR 48]).

- Update to `multihash` `v0.15` (see [PR 50]).

- Update to `multihash` `v0.16` (see [PR 51]).

[PR 48]: https://github.com/multiformats/rust-multiaddr/pull/48
[PR 50]: https://github.com/multiformats/rust-multiaddr/pull/50
[PR 50]: https://github.com/multiformats/rust-multiaddr/pull/51

# 0.13.0 [2021-07-08]

- Update to multihash v0.14.0 (see [PR 44]).

- Update to rand v0.8.4 (see [PR 45]).

[PR 44]: https://github.com/multiformats/rust-multiaddr/pull/44
[PR 45]: https://github.com/multiformats/rust-multiaddr/pull/45

# 0.12.0 [2021-05-26]

- Merge  [multiaddr] and [parity-multiaddr] (see [PR 40]).

    - Functionality to go from a `u64` to a `multiadddr::Protocol` and back is
      removed. Please open an issue on [multiaddr] in case this is still needed.

    - Given that `multiaddr::Protocol` now represents both the protocol
      identifier as well as the protocol data (e.g. protocol identifier `55`
      (`dns6`) and protocol data `some-domain.example`) `multiaddr::Protocol` is
      no longer `Copy`.

[multiaddr]: https://github.com/multiformats/rust-multiaddr
[parity-multiaddr]: https://github.com/libp2p/rust-libp2p/blob/master/misc/multiaddr/
[PR 40]: https://github.com/multiformats/rust-multiaddr/pull/40

# 0.11.2 [2021-03-17]

- Add `Multiaddr::ends_with()`.

# 0.11.1 [2021-02-15]

- Update dependencies

# 0.11.0 [2021-01-12]

- Update dependencies

# 0.10.1 [2021-01-12]

- Fix compilation with serde-1.0.119.
  [PR 1912](https://github.com/libp2p/rust-libp2p/pull/1912)

# 0.10.0 [2020-11-25]

- Upgrade multihash to `0.13`.

# 0.9.6 [2020-11-17]

- Move the `from_url` module and functionality behind the `url` feature,
  enabled by default.
  [PR 1843](https://github.com/libp2p/rust-libp2p/pull/1843).

# 0.9.5 [2020-11-14]

- Limit initial memory allocation in `visit_seq`.
  [PR 1833](https://github.com/libp2p/rust-libp2p/pull/1833).

# 0.9.4 [2020-11-09]

- Update dependencies.

# 0.9.3 [2020-10-16]

- Update dependencies.

# 0.9.2 [2020-08-31]

- Add `Ord` instance for `Multiaddr`.

# 0.9.1 [2020-06-22]

- Updated dependencies.
