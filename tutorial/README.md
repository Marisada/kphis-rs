# Tutorial

This is `in situ` tutorial that serve together with KPHIS cloned from [kphis-book](https://github.com/Marisada/kphis-book)

We created <https://github.com/Marisada/kphis-book> with [mdBook](https://github.com/rust-lang/mdBook) for the development of KPHIS tutorial `kphis-book` which published with GitHub Page <https://marisada.github.io/kphis-book/>

Now we can build `in situ` tutorial in KPHIS by
- Delete `src` folder (if exists)
- Copy `src` folder from <https://github.com/Marisada/kphis-book/tree/main/src> to `src`
- run `tutorial-build` in Windows or `./tutorial-build.sh` in Linux from KPHIS root directory

## Differences from kphis-book
- Added `custom.js` that create a `home` button for navigate back to KPHIS
- Changed `build-dir` from default `book` to `../volume/pwa/book`
