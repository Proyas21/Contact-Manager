This is a simple cli contact manager built with Rust and clap.

This is a hobby project inspired from https://github.com/Proyas21/Contact-Manager

# Usage

```
Usage: conman.exe <COMMAND>

Commands:
  add     Add a contact
  delete  Delete a contact
  edit    Edit a contact
  search  Search a contact
  show    Shows all contacts
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help information
  -V, --version  Print version information
```

# conman show

```
╭────┬──────────────────────────┬─────────────────╮
│ Sl ┆ Name                     ┆ Phone           │
╞════╪══════════════════════════╪═════════════════╡
│ 1  ┆ Leanne Graham            ┆ 1-770-736-8031  │
├╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ 2  ┆ Ervin Howell             ┆ 010-692-6593    │
├╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ 3  ┆ Clementine Bauch         ┆ 1-463-123-4447  │
├╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ 4  ┆ Patricia Lebsack         ┆ 493-170-9623    │
├╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ 5  ┆ Chelsey Dietrich         ┆ (254)954-1289   │
├╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ 6  ┆ Mrs. Dennis Schulist     ┆ 1-477-935-8478  │
├╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ 7  ┆ Kurtis Weissnat          ┆ 210.067.6132    │
├╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ 8  ┆ Nicholas Runolfsdottir V ┆ 586.493.6943    │
├╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ 9  ┆ Glenna Reichert          ┆ (775)976-6794   │
├╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ 10 ┆ Clementina DuBuque       ┆ 024-648-3804    │
╰────┴──────────────────────────┴─────────────────╯
```
