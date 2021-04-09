# levelhead-stats
A simple tool to download Levelhead level statistics. To use run the program and enter the Levelhead user ID for the user you want to download created level stats from. It will create a CSV file that can then be loaded into Excel or whatever spreadsheet program you prefer.

## Building
To build this program from source you will need your own free Rumpus API key from https://www.bscotch.net/account. This program only needs read permisions on the API key. Once you get the key put it in a file called `rumpus_key` in this folder. Then run `cargo build`.

## License
This program is released under the MIT license. See `LICENSE` for details.