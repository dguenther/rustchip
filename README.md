rustchip
========

A CHIP-8 Emulator written in Rust. Based on the tutorial at http://www.multigesture.net/articles/how-to-write-an-emulator-chip-8-interpreter/

Building
--------

First, you'll need to install the [SFML](http://www.sfml-dev.org/download/sfml/2.0/) and [CSFML](http://www.sfml-dev.org/download/csfml/) libraries. If you're on Linux, you can run the following: 
```
apt-add-repository ppa:sonkun/sfml-development
apt-get install libsfml-dev libcsfml-dev
```

After that, run `./configure` in the repository root to pull rust-sfml, build it, and move it into the `lib/` directory.

Finally, run `rustc main.rs` to build the emulator.

Testing
-------

After running `./configure`, run `rustc --test test.rs` followed by `./test` to run the test library.

Running
-------

Run `./main <rompath>` to start the emulator. The CHIP-8 system uses a hex keypad:

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
