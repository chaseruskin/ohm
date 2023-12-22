# `ohm`

A resistor color code calculator written in Rust.

A simple application to compute the resistance based on a resistor's sequence of color bands. This program supports resistors displaying 3, 4, 5, or 6 bands.

Entering the sequence of colors on the resistor will display the interpreted resistor and its computed resistance in Ohms (including minimum and maximum tolerance values).

```
$ ohm n k r d --no-color

Identification: -[brown,black,red  gold ]-
Resistance: 1000.0 Ω ± 5.0% (min: 950.0 Ω, max: 1050.0 Ω)
```

The latest binaries are available as workflow artifacts for Windows, Mac, and Linux [here](https://github.com/c-rus/ohm/actions).

## Interface
```
A resistor color code calculator.

Usage:
    ohm [options] <band>...

Arguments:
    <band>...       colors from left to right (expects between 3 and 6)  

Options:
    --help, -h      print quick help text
    --no-color      disable color formatting
    --list          print the possible color codes

```

## Supported Color Codes
```
Color Codes:
    k   black       0
    n   brown       1
    r   red         2
    o   orange      3
    y   yellow      4
    g   green       5
    b   blue        6
    v   violet      7
    a   gray        8
    w   white       9
    d   gold
    s   silver
```

## References
[1] https://www.codrey.com/tools/resistor-color-code-calculator/