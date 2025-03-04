# EBNSF
> Extended Backus-Naur Syntax FDiagrams

Create railroad diagrams from EBNF

## Todo
- support repitition specifiers and ranges (i.e. [a-z], +, *)
    - `?`, `+`, and `*` now supported
    - ranges can be specified by cheating and treating them as literals, e.g. "[a-z]".
      first-class support for ranges without wrapping them in quotes will be added later
      (but they probably won't be validated).
- support grouping
    - grouping now supported (with repitition modifiers) using parenthesis
- support rules spanning multiple lines
- decide on the full syntax I want to use/support for EBNF.
  See dwheeler.com, grammarware.net, w3.org, and cl.cam.ac.uk links below.
- figure out if/how I want to handle bounded repitition
- figure out if/how I want to handle negation
- support escape characters

## Credit
Initial parser and `test/bnf.ebnf` file from BNF specification in EBNF form from https://bnfplayground.pauliankline.com/ > Examples > BNF

Additional reading on EBNF syntax:
https://www.cl.cam.ac.uk/~mgk25/iso-14977.pdf
https://dwheeler.com/essays/dont-use-iso-14977-ebnf.html
https://www.grammarware.net/text/2012/bnf-was-here.pdf
https://www.w3.org/TR/xml/#sec-notation
