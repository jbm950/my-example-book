I attempted a simple example using keyring with Rust. Looks like the keyring
crate needs you to specify the backend manually though unlike python so it
makes cross platform more manual. Also, the backend I found might be temporary
and get reset on reboot/logout according to AI. Therefore, it might be better
for temporary passwords/tokens rather than something that needs to persist.

The keyring ecosystem ended up being more complicated than originally
anticipated.
