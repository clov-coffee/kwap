# Changelog

All notable changes to this project will be documented in this file. See [standard-version](https://github.com/conventional-changelog/standard-version) for commit guidelines.

## 0.10.0 (2022-06-17)


### ⚠ BREAKING CHANGES

* **kwap:** add dtls support (#131)

### Features

* **kwap:** add dtls support ([#131](https://github.com/clov-coffee/kwap/issues/131)) ([0e4b7ae](https://github.com/clov-coffee/kwap/commit/0e4b7ae7ffa5a94da2f5967ffa37735962ac51a0))

## 0.9.0 (2022-06-01)


### ⚠ BREAKING CHANGES

* **kwap:** add support for multicast, improve server ux (#129)

### Features

* **kwap:** add support for multicast, improve server ux ([#129](https://github.com/clov-coffee/kwap/issues/129)) ([0ebfcd0](https://github.com/clov-coffee/kwap/commit/0ebfcd0a7a2a74ca928b9d10602f9d24420d3404))

### 0.8.1 (2022-05-26)


### Features

* **kwap:** add logging ([#127](https://github.com/clov-coffee/kwap/issues/127)) ([73d538d](https://github.com/clov-coffee/kwap/commit/73d538d2793afbf4ae993a48668bdddaa9d5bf10))

## 0.8.0 (2022-05-25)


### ⚠ BREAKING CHANGES

* **kwap:** support ipv4 and ipv6 (#126)

### Features

* **kwap:** support ipv4 and ipv6 ([#126](https://github.com/clov-coffee/kwap/issues/126)) ([9150ca1](https://github.com/clov-coffee/kwap/commit/9150ca13950db5c8f17f0963f3ae111f8362ba79))

### 0.7.3 (2022-05-22)

### 0.7.2 (2022-05-21)


### Features

* **kwap:** calc EXCHANGE_LIFETIME & discard old tokens/ids ([#122](https://github.com/clov-coffee/kwap/issues/122)) ([b7001d5](https://github.com/clov-coffee/kwap/commit/b7001d5b24d6e74889ea86aad44b4c74f6c07d5e))

### 0.7.1 (2022-05-21)


### Features

* **kwap:** add support for coap runtime config ([9c8ef54](https://github.com/clov-coffee/kwap/commit/9c8ef541d89e8430010845f3b0f1ec7f06a220c7))

## 0.7.0 (2022-05-20)


### ⚠ BREAKING CHANGES

* **kwap:** Choose CON / NON responses to NON requests (#117)

### Features

* **kwap:** Choose CON / NON responses to NON requests ([#117](https://github.com/clov-coffee/kwap/issues/117)) ([5d39603](https://github.com/clov-coffee/kwap/commit/5d3960314ffef7cac4f896d92c056d6e9100f10e))

### 0.6.2 (2022-05-14)


### Features

* **kwap:** servers should automatically respond to ping requests  ([#113](https://github.com/clov-coffee/kwap/issues/113)) ([91de497](https://github.com/clov-coffee/kwap/commit/91de4976db8289c4e1fe5cf8c2e29d7067a0a207))

### 0.6.1 (2022-05-10)

## 0.6.0 (2022-05-08)


### ⚠ BREAKING CHANGES

* **kwap:** make module and type names more ergonomic (#88)

### Features

* **kwap:** make module and type names more ergonomic ([#88](https://github.com/clov-coffee/kwap/issues/88)) ([8d33a64](https://github.com/clov-coffee/kwap/commit/8d33a64884ddecce41b8c58e734d5edaa5b5c609))

### 0.5.5 (2022-05-07)

### 0.5.4 (2022-05-01)

### 0.5.3 (2022-05-01)

### 0.5.2 (2022-04-27)

### 0.5.1 (2022-04-27)


### Bug Fixes

* **kwap:** update common ([#72](https://github.com/clov-coffee/kwap/issues/72)) ([4301139](https://github.com/clov-coffee/kwap/commit/43011395ab6047c5a0b54784cbadfa2e171139e5))

## 0.5.0 (2022-04-25)


### ⚠ BREAKING CHANGES

* **kwap:** reduce complexity (#69)

### Features

* **kwap:** reduce complexity ([#69](https://github.com/clov-coffee/kwap/issues/69)) ([2762226](https://github.com/clov-coffee/kwap/commit/2762226634e2a538bb3c3f3173792c32e7c4b8b9))

## 0.4.0 (2022-04-01)


### ⚠ BREAKING CHANGES

* **kwap:** errors should be more ergonomic (#68)

### Features

* **kwap:** errors should be more ergonomic ([#68](https://github.com/clov-coffee/kwap/issues/68)) ([cdb018d](https://github.com/clov-coffee/kwap/commit/cdb018ddd04de63f385f22940e3e1f313a27d3b5))

### 0.3.7 (2022-02-12)


### Bug Fixes

* **kwap:** add EventIO type marker for fns that fire events ([#67](https://github.com/clov-coffee/kwap/issues/67)) ([5ed5d9c](https://github.com/clov-coffee/kwap/commit/5ed5d9c5db2c93afa20aa0034ad734789d400d87))

### 0.3.6 (2022-02-05)

### 0.3.5 (2022-02-04)


### Bug Fixes

* **kwap:** support nons that do not receive a response ([#64](https://github.com/clov-coffee/kwap/issues/64)) ([27f64f1](https://github.com/clov-coffee/kwap/commit/27f64f198dc8211d6a8d35cd5e54702f842a8da3))

### 0.3.4 (2022-01-29)

### 0.3.3 (2022-01-29)


### Features

* **kwap:** add platformless blocking::Client::new and Config struct ([#60](https://github.com/clov-coffee/kwap/issues/60)) ([45c6a65](https://github.com/clov-coffee/kwap/commit/45c6a65b72709d5ad37d042353e45c891ef88fc7))

### 0.3.2 (2022-01-28)

### 0.3.1 (2022-01-22)

## 0.3.0 (2022-01-15)


### ⚠ BREAKING CHANGES

* **kwap:** we should retry sending CONfirmable messages until ACKed (#56)

### Features

* **kwap:** we should retry sending CONfirmable messages until ACKed ([#56](https://github.com/clov-coffee/kwap/issues/56)) ([7de512d](https://github.com/clov-coffee/kwap/commit/7de512dcb8ed4e24b9a725bb4add9d175864aab7))

## 0.2.0 (2022-01-15)


### ⚠ BREAKING CHANGES

* **kwap:** remove interior mutability in Core (#55)

### Bug Fixes

* **kwap:** remove interior mutability in Core ([#55](https://github.com/clov-coffee/kwap/issues/55)) ([768f4b9](https://github.com/clov-coffee/kwap/commit/768f4b94c078958f54e18efa53d93ce1ab144182))

### 0.1.10 (2022-01-12)


### Features

* **kwap:** core should allow pinging coap servers ([#53](https://github.com/clov-coffee/kwap/issues/53)) ([de7cfda](https://github.com/clov-coffee/kwap/commit/de7cfda186b47ad1a41da2f9da922ceb2ea5e1ed))

### 0.1.9 (2022-01-11)

### 0.1.8 (2022-01-11)


### Bug Fixes

* **kwap:** add error types to send_req ([#50](https://github.com/clov-coffee/kwap/issues/50)) ([818904a](https://github.com/clov-coffee/kwap/commit/818904a039b3e7884d3411bc2cd0462f4f3f56a6))

### 0.1.7 (2022-01-11)

### 0.1.6 (2022-01-08)

### 0.1.5 (2022-01-08)


### Bug Fixes

* **kwap:** client should work ([#46](https://github.com/clov-coffee/kwap/issues/46)) ([0e8058c](https://github.com/clov-coffee/kwap/commit/0e8058c4e8a9828339e8c6d89e015f9a85c24242))

### 0.1.4 (2022-01-07)


### Features

* **kwap:** runtime should support acting as client ([#45](https://github.com/clov-coffee/kwap/issues/45)) ([0d63c52](https://github.com/clov-coffee/kwap/commit/0d63c52fa872e7a33ac7b298ae55e75870c1a147))

### 0.1.3 (2022-01-07)


### Features

* **kwap:** initial commit of client ([#43](https://github.com/clov-coffee/kwap/issues/43)) ([f9f13cb](https://github.com/clov-coffee/kwap/commit/f9f13cb67cc3d962c038f93a798261d8572c3fa5))

### 0.1.2 (2022-01-04)


### Features

* **kwap:** add request type ([#39](https://github.com/clov-coffee/kwap/issues/39)) ([2fb2622](https://github.com/clov-coffee/kwap/commit/2fb262260f80455e3649f99d7be763015a269b2d))

### 0.1.1 (2022-01-02)


### Features

* **kwap:** rough cut of response struct ([#31](https://github.com/clov-coffee/kwap/issues/31)) ([306f30d](https://github.com/clov-coffee/kwap/commit/306f30dbbb459cc7eae32db8b20f8d213dd23a2c))
