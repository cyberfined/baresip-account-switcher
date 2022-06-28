# baresip-account-switcher

Baresip module for switching accounts.

## Installation

Run `cargo build --release` and copy result shared library to your baresip modules directory as `account_switcher.so`. Then edit `~/.baresip/config` as follows:
```
#module		        account.so
module		        account_switcher.so
```

## Configuration

Create `~/.baresip/accounts.toml` with following accounts format:

```toml
[accounts]
    [accounts.account_1]
    login      = "login_1"
    domain     = "http://localhost:9765"
    transport  = "udp"
    answermode = "manual"
    auth_pass  = "password"

    [accounts.account_2]
    login      = "login_2"
    domain     = "http://localhost"
    transport  = "udp"
    answermode = "manual"
    auth_pass  = "password"
```

## Usage

This module adds use_accounts command to the baresip:

```
/use_accounts account_1[,...]
```
It registers given accounts and unregisters others. So, if you want to register only account_1, you should write:

```
/use_accounts account_1
```

If you want to register both accounts, you should write:

```
/use_accounts account_1,account_2
```

## Activate accounts with command line arguments

If you want to run baresip only with some registered account, you can use `-e` command line argument:

```bash
baresip -e "/use_accounts account_1"
```
