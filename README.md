# `ohm`

A resistor color code calculator.

A simple application to compute the resistance based on a resistor's sequence of color bands. This program supports resistors displaying 3, 4, 5, or 6 bands.

Entering the sequence of colors on the resistor will display the interpreted resistor and its computed resistance in Ohms (including minimum and maximum tolerance values).

## Interface
```
A resistor color code calculator.

Usage:
    ohm [options] <band>...

Arguments:
    <band>...       colors from left to right (expects between 3 and 6)  

Options:
    --help, -h      print quick help text
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
[1] https://www.google.com/url?sa=i&url=https%3A%2F%2Fwww.codrey.com%2Ftools%2Fresistor-color-code-calculator%2F&psig=AOvVaw2vcPlfhLJ6c7jBsj8ZZzjo&ust=1673108282285000&source=images&cd=vfe&ved=0CAwQjRxqFwoTCNiehZmss_wCFQAAAAAdAAAAABAo