# EBNSF
> Extended Backus-Naur Syntax FDiagrams

Create railroad diagrams from EBNF

## Usage
```
Usage: ebnsf [OPTIONS] <INPUT>

Arguments:
  <INPUT>  File to read EBNF spec from

Options:
  -o, --output <OUTPUT>  Where to save the rendered SVG
  -h, --help             Print help
```

## Example
Run `ebnsf ./test/bnf.ebnf -o ./doc/bnf.svg` to turn

```ebnf
<syntax>         ::= <rule>+
<rule>           ::= <opt_whitespace> "<" <rule_name> ">" <opt_whitespace> "::=" <opt_whitespace> <group> <line_end>
<opt_whitespace> ::= " "*
<expression>     ::= <list> (<opt_whitespace> "|" <opt_whitespace> <expression>)?
<group>          ::= "(" <expression> ")" | <expression>
<line_end>       ::= <opt_whitespace> "\n"+
<list>           ::= <term> | <term> <opt_whitespace> <list>
<term>           ::= <literal> | "<" <rule_name> ">"
<literal>        ::= '"' <text> '"'
<text>           ::= <character>+
<character>      ::= <letter> | <digit>
<letter>         ::= "[A-Za-z]"
<digit>          ::= "[0-9]"
<rule_name>      ::= <letter> <rule_char>*
<rule_char>      ::= <letter> | <digit> | "_"
```
into
![bnf syntax diagram](https://github.com/user-attachments/assets/1971639c-0420-4878-b05e-c3884fbc5b61)
## Todo
- [x] support repitition specifiers and ranges (i.e. [a-z], +, *)
    - `?`, `+`, and `*` now supported
    - ranges can be specified by cheating and treating them as literals, e.g. "[a-z]".
      first-class support for ranges without wrapping them in quotes will be added later
      (but they probably won't be validated).
- [x] support grouping
    - grouping now supported (with repitition modifiers) using parenthesis
- [ ] support rules spanning multiple lines
- [ ] decide on the full syntax I want to use/support for EBNF.
    See dwheeler.com, grammarware.net, w3.org, and cl.cam.ac.uk links below.
- [ ] figure out if/how I want to handle bounded repitition
- [ ] figure out if/how I want to handle negation
- [x] support escape characters
    - supports escaping nested quotes and backslashes

## Credit
Initial parser and `test/bnf.ebnf` file from BNF specification in EBNF form from https://bnfplayground.pauliankline.com/ > Examples > BNF

Additional reading on EBNF syntax:

https://www.cl.cam.ac.uk/~mgk25/iso-14977.pdf
https://dwheeler.com/essays/dont-use-iso-14977-ebnf.html
https://www.grammarware.net/text/2012/bnf-was-here.pdf
https://www.w3.org/TR/xml/#sec-notation
