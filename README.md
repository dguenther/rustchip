rustchip
========

A CHIP-8 Emulator written in Rust. Based on the tutorial at http://www.multigesture.net/articles/how-to-write-an-emulator-chip-8-interpreter/

Building
--------

First, you'll need to install the [SFML](http://www.sfml-dev.org/download/sfml/2.0/) and [CSFML](http://www.sfml-dev.org/download/csfml/) libraries. If you're using a GNU/Linux distribution that supports PPAs, you can run the following:
```
apt-add-repository ppa:sonkun/sfml-development
apt-get install libsfml-dev libcsfml-dev
```

Once you've done that, just run `cargo build`.

Testing
-------

Tests are found in `src/cpu.rs`. They can be executed by running `cargo test`.

Running
-------

Run `cargo run <rompath>` to start the emulator. The CHIP-8 system uses a hex keypad, with keymappings listed below. You can reset the emulator by pressing `ALT+R`.

###### CHIP-8 Keypad -> Keyboard

<table>
    <tr>
        <td>1</td>
        <td>2</td>
        <td>3</td>
        <td>C</td>
        <td></td>
        <td>1</td>
        <td>2</td>
        <td>3</td>
        <td>4</td>
    </tr>
    <tr>
        <td>4</td>
        <td>5</td>
        <td>6</td>
        <td>D</td>
        <td>-></td>
        <td>Q</td>
        <td>W</td>
        <td>E</td>
        <td>R</td>
    </tr>
    <tr>
        <td>7</td>
        <td>8</td>
        <td>9</td>
        <td>E</td>
        <td></td>
        <td>A</td>
        <td>S</td>
        <td>D</td>
        <td>F</td>
    </tr>
    <tr>
        <td>A</td>
        <td>0</td>
        <td>B</td>
        <td>F</td>
        <td></td>
        <td>Z</td>
        <td>X</td>
        <td>C</td>
        <td>V</td>
    </tr>
</table>
