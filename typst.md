## Typst
App: [https://typst.app](https://typst.app/)

Document: [https://typst.app/docs](https://typst.app/docs/)

Packages: [Typst Universe](https://typst.app/universe/)

Unofficial book: [Typst Example Book](https://sitandr.github.io/typst-examples-book/book/)

## Restriction of this embeded Typst
- use Typst version 0.12 ([`github`](https://github.com/typst/typst))
- Typst run in a sandbox `world`, every files must loaded to `world` before compile to pdf or image
- files we included to `world` were
    - font
        - No math, no emoji
        - TH Sarabun New (Regular, Bold, Italic, BoldItalic) with..
            - &#3662; using `\u{0e4e}`
            - &#3663; using `\u{0e4f}`
            - &#3674; using `\u{0e5a}`
            - &#3675; using `\u{0e5b}`
            - &#8226; using `\u{2022}`
            - &#8230; using `#sym.dots.h` or `\u{2026}`
            - &#8240; using `\u{2030}`

            - &#169; using `#sym.copyright` or `\u{00a9}`
            - &#174; using `\u{20ae}`
            - &#176; using `#sym.degree` or `\u{00b0}`
            - &#177; using `#sym.plus.minus` or `\u{00b1}`
            - &#181; using `\u{00b5}`

            - &#8210; using `#sym.dash.fig` or `\u{2012}`
            - &#8211; using `#sym.dash.en` or `\u{2013}`
            - &#8212; using `#sym.dash.em` or `\u{2014}`
            - &#8213; using `#sym.bar.h` or `\u{2015}`

            - &#8321; using `\u{2081}`
            - &#8322; using `\u{2082}`
            - &#8323; using `\u{2083}`

            - &#8364; using `#sym.euro` or `\u{20ac}`
            - &#8710; using `#sym.laplace` or `\u{2206}`
            - &#8722; using `#sym.minus` or `\u{2212}`
            - &#8776; using `#sym.approx` or `\u{2248}`
            - &#8800; using `#sym.eq.not` or `\u{2260}` or `<>` or `!=`
            - &#8804; using `#sym.lt.eq` or `\u{2264}` or `<=`
            - &#8805; using `#sym.gt.eq` or `\u{2265}` or `>=`
        - Added symbols below to TH Sarabun New Regular subset
            - &#9744; using `#sym.ballot` or `\u{2610}`
            - &#9745; using `#sym.ballot.check` or `\u{2611}`
            - &#9746; using `#sym.ballot.cross` or `\u{2612}`
            - &#10003; using `#sym.checkmark` or `\u{2713}`
            - &#10007; using `\u{2717}`
            - &#8195; using `#sym.space.quad` or `\u{2003}`

- `data.json` can fetch from client
- files in `volume/pwa/` can fetch from client
> ex: `volume/pwa/statics/picture/krut.svg` can call with `image("statics/picture/krut.svg")`

## Table of listed items
- [Markups](https://typst.app/docs/reference/syntax/#markup)
- [Maths](https://typst.app/docs/reference/syntax/#math)
- [Codes](https://typst.app/docs/reference/syntax/#code)
- [Operators](https://typst.app/docs/reference/scripting#operators)
- [Symbol shorthands](https://typst.app/docs/reference/symbols/#shorthands)
- [Syms](https://typst.app/docs/reference/symbols/sym/)
- [Emojis](https://typst.app/docs/reference/symbols/emoji/)

## General
- [Guides](https://typst.app/docs/guides/): [Guide for LaTeX users](https://typst.app/docs/guides/guide-for-latex-users/), [Page setup guide](https://typst.app/docs/guides/page-setup-guide/), [Table guide](https://typst.app/docs/guides/table-guide/)
- [Function](https://typst.app/docs/reference/foundations/function/) a mapping from argument values to a return value
- [Module](https://typst.app/docs/reference/foundations/module/) an evaluated module, either built-in or resulting from a file
- [Syntax](https://typst.app/docs/reference/syntax/)
- [context](https://typst.app/docs/reference/context/) keyword is a getter for the element function field (Set Rule) value `#context text.lang` `#let a = context text.lang`
- [Data](https://typst.app/docs/reference/data-loading/)

### Comment
- `//` single line comment
- `/*` .. `*/` multiple lines comment
### Escape sequences
- `\` to display Typst special charactor ex: `\#`,`\$`
- `\u{1f600}` to display Unicode codepoint using hexadecimal value inside `{}`
### New line
- normal line break = single line with ` ` between each item(line)
- end line with `\` = new line
- blank line = new line

## Scripting
### hash `#` markup to [scripting](https://typst.app/docs/reference/scripting) an expression
> `#` + expression (+ content block)
- call `emph` function with content `[Hellp]`: `#emph[Hellp]`
- call `emoji` function with field `face`: `#emoji.face`
- call `len` method of string : `#"Hello".len()`
- call binary operator `+`: `#(1 + 2)`
- render variable: `#{ let a = 1; a + 4 }` return 5

### [function](https://typst.app/docs/reference/foundations/function/) call
- using content as argument: `#list([A],[B])`
- using content as argument without `()` (short version): `#list[A][B]`
- using function's named-argument: `#enum(start: 2)[A][B]`

### block
#### [code](https://typst.app/docs/reference/syntax/#code) block: start with `#` or in `{ .. }`
- remove `#` in code block : `#let a = {let b = 2; b + 1}`
- seperate multiple statements with line-break or `;`
- return one content or `none`
#### content block: `[ .. ]`
- as a single variable, use as function argument, return from function
- can use [markups](https://typst.app/docs/reference/syntax/#markup)
- return [`content](https://typst.app/docs/reference/foundations/content/) type
#### [math](https://typst.app/docs/reference/syntax/#math) block: `$ .. $`
- inline using `$x^2$`
- block using `$ x^2 $`
- commemt using `$/* comment */$`

### binding value to variable with `let` (or `#let` in content block)
- assign content to variable: `#let a = [*text*]`
- assign array: `#let ar = (1,2,3,4,5)`
- assign multiple variables using destructure: `#let (x,y) = (1,2)`
- assign multiple variables to tuple: `#{let a = (x:1,y:2); let (y,) = a; let (x:m,) = a; [#m, #y]}`
- assign with spread operator `..`: `#let (x,..,y) = (1,2,3,4,5)` x is 1, y is 5
- assign new variable with spread operator `..`: `#{let a = (x:1,y:2,z:3); let (x, ..other) = a; for (p,q) in other [(#p is #q)]}` return (y is 2)(z is 3)

### condition operator
#### if
> `if` + condition + block ( `else if` + condition + block (`else` block ))
- `#if a in "ABC" [#a]`
- `#if a in "ABC" {a} else [not in ABC]`
- `#if a in "ABC" [#a] else if x in DEF {x}`
#### for
> `for` variable `in` array + block that will return single content
- value in array `#for c in (1,2,3) [#c is a number. ]`
- char in string `#for c in "ABC" [#c is a letter. ]`
- pair in dictionary `#for (k,v) in (x:1,y:2) [#k is #v ]` return x is 1 y is 2
- byte in bytes `#for b in bytes("😀") { (b,) }` return (240, 159, 152, 128)
- can use `break`(end) and `continue`(skip)

#### while
> `for` condition + block that will return single content
- `#{ let n = 2; while n < 5 { n += 1; (n,) } }` return (3,4,5)
- can use `break`(end) and `continue`(skip)

### variable field
- key of dictionary: `#{ let a = (x:1,y:2); a.x }`
- modifier of symbol: `#sym.arrow.r`
- modifier of emoji: `#emoji.face.halo`
- definition of module: <br>`#import "utils.typ"`<br>`#utils.add(2,5)`
- field of content's [element function](https://typst.app/docs/reference/foundations/function/#element-functions): `#{ let a = [=title]; a.depth }` return [heading](https://typst.app/docs/reference/model/heading/) depth field of `[=title]`
- method of [type](https://typst.app/docs/reference/foundations/type/): `#"abc".len()` "abc" type is [str](https://typst.app/docs/reference/foundations/str/) (equal to `#str.len("abc")`)

### iterator
- map: `#{let a = (1,2,3); let b = (7,6,5); [#a.zip(b).map( ((x,y)) => x + y )]}` return (8, 8, 8)

### modules
- `#include "a.typ"` return content from file
- `#import "a.typ" as egg` return all code to be used in current scope
- `#import "a.typ": a as one, b as two` return variable a and b (has `let a = ..` in a.typ)
- `#import egg: a as shell` return variable a from a.typ (has `let a = ..` in a.typ)

### packages (THIS APP NOT SUPPORTED) from [Typst Universe](https://typst.app/universe/) ([GIT](https://github.com/typst/packages))
> `#import` + namespace + `/` + name + `:` + version
- `#import "@preview/example:0.1.0": add` import "example" community package version 0.1.0

## Styling
### [Set Rule](https://typst.app/docs/reference/styling/#set-rules)
> `#set` function name `(` field `:` value `)`
> `#set` function name `(` field `:` value `) if` condition
- set new default(for every call after 'set') for 'Settable' field of Element functions
- set once effect entire scope, set at top-level is effect to all in the file
- set-if will depended on outer-scope condition
- `#set heading(numbering: "I.")` change `=text` to `I.text`
- `#let task(body, critical: false) = {set text(red) if critical; [- #body]}` set in function scope

### [Show Rules](https://typst.app/docs/reference/styling/#show-rules)
#### show-set
> `show` [selector](https://typst.app/docs/reference/foundations/selector/) `:` [Set Rule](https://typst.app/docs/reference/styling/#set-rules)
> selector can be element function, field (using [where](https://typst.app/docs/reference/foundations/function/#definitions-where)), [str](https://typst.app/docs/reference/foundations/str/), [regex](https://typst.app/docs/reference/foundations/regex/), [lebel](https://typst.app/docs/reference/foundations/label/), [location](https://typst.app/docs/reference/introspection/location/) or [selector](https://typst.app/docs/reference/foundations/selector/) itself
- `#show heading: set text(navy)`
- `#show "SomeText" set text(navy)`
- `#show regex("\w+"): set ..`
- `#show heading.where(level: 1): set ..`
- `#show <intro>: set ..`
#### show-function
> `show` [selector](https://typst.app/docs/reference/foundations/selector/) `:` [function](https://typst.app/docs/reference/foundations/function/)
- `#show heading: it => { set align(center); emph(it.body) }`
- `#show heading: smallcaps`
#### show-content
> `show` [selector](https://typst.app/docs/reference/foundations/selector/) `:` [content](https://typst.app/docs/reference/foundations/content/) or [str](https://typst.app/docs/reference/foundations/str/)
#### show all
> `show:` [Set Rule](https://typst.app/docs/reference/styling/#set-rules) or [function](https://typst.app/docs/reference/foundations/function/)
- `#show: rest => { set .. }`

## List of Functions

### [Foundation](https://typst.app/docs/reference/foundations/)
- [arguments](https://typst.app/docs/reference/foundations/arguments/) <sup><span style="color:red">type</span></sup> captured arguments to a function
- [array](https://typst.app/docs/reference/foundations/array/) <sup><span style="color:red">type</span></sup> a sequence of values
- [assert](https://typst.app/docs/reference/foundations/assert/) ensures that a condition is fulfilled
- [bytes](https://typst.app/docs/reference/foundations/bytes/) <sup><span style="color:red">type</span></sup> a sequence of bytes
- [calc](https://typst.app/docs/reference/foundations/calc) <sup><span style="color:silver">module</span></sup> module for calculations and processing of numeric values
- [content](https://typst.app/docs/reference/foundations/content/) <sup><span style="color:red">type</span></sup> a piece of document content
- [datetime](https://typst.app/docs/reference/foundations/datetime/) <sup><span style="color:red">type</span></sup> represents a date, a time, or a combination of both
- [dictionary](https://typst.app/docs/reference/foundations/dictionary/) <sup><span style="color:red">type</span></sup> a map from string keys to values
- [duration](https://typst.app/docs/reference/foundations/duration/) <sup><span style="color:red">type</span></sup> represents a positive or negative span of time
- [eval](https://typst.app/docs/reference/foundations/eval/) evaluates a string as Typst code
- [false](https://typst.app/docs/reference/foundations/bool/) <sup><span style="color:red">type</span></sup> a bool type with two states (false)
- [float](https://typst.app/docs/reference/foundations/float/) <sup><span style="color:red">type</span></sup> a floating-point number
- [int](https://typst.app/docs/reference/foundations/int/) <sup><span style="color:red">type</span></sup> a whole number
- [label](https://typst.app/docs/reference/foundations/label/) <sup><span style="color:red">type</span>, <span style="color:fuchsia">markup</span></sup> a label for an element `<intro>`
- [panic](https://typst.app/docs/reference/foundations/panic/) fails with an error
- [plugin](https://typst.app/docs/reference/foundations/plugin/) <sup><span style="color:red">type</span></sup> a WebAssembly plugin
- [regex](https://typst.app/docs/reference/foundations/regex/) <sup><span style="color:red">type</span></sup> a regular expression
- [repr](https://typst.app/docs/reference/foundations/repr/) returns the string representation of a value
- [selector](https://typst.app/docs/reference/foundations/selector/) <sup><span style="color:red">type</span></sup> a filter for selecting elements within the document
- [str](https://typst.app/docs/reference/foundations/str/) <sup><span style="color:red">type</span></sup> a sequence of Unicode codepoints
- [style](https://typst.app/docs/reference/foundations/style/) provides access to active styles
- [sys](https://typst.app/docs/reference/foundations/sys) <sup><span style="color:silver">module</span></sup> module for system interactions
- [true](https://typst.app/docs/reference/foundations/bool/) <sup><span style="color:red">type</span></sup> a bool type with two states (true)
- [type](https://typst.app/docs/reference/foundations/type/) <sup><span style="color:red">type</span></sup> describes a kind of value
- [version](https://typst.app/docs/reference/foundations/version/) <sup><span style="color:red">type</span></sup> a version with an arbitrary number of components

### [Model](https://typst.app/docs/reference/model/)
- [bibliography](https://typst.app/docs/reference/model/bibliography/) <sup><span style="color:gold">element</span></sup> create a bibliography / reference listing
- [cite](https://typst.app/docs/reference/model/cite/) <sup><span style="color:gold">element</span></sup> insert bibliography citing note
- [document](https://typst.app/docs/reference/model/document/) <sup><span style="color:gold">element</span></sup> is the root element of a document and its metadata
- [emph](https://typst.app/docs/reference/model/emph/) <sup><span style="color:gold">element</span>, <span style="color:fuchsia">markup</span></sup> create italic text `_emphasis_`
- [enum](https://typst.app/docs/reference/model/enum/) <sup><span style="color:gold">element</span>, <span style="color:fuchsia">markup</span></sup> create a numbered list `+ item`
- [figure](https://typst.app/docs/reference/model/figure/) <sup><span style="color:gold">element</span>, <span style="color:lime">referenceable</span></sup> create a image/table figure with an optional caption
- [footnote](https://typst.app/docs/reference/model/footnote/) <sup><span style="color:gold">element</span>, <span style="color:lime">referenceable</span></sup> will insert a superscript number that links to the note at the bottom of the page
- [heading](https://typst.app/docs/reference/model/heading/) <sup><span style="color:gold">element</span>, <span style="color:lime">referenceable</span>, <span style="color:fuchsia">markup</span></sup> create heading for each level of section `= Heading`
- [link](https://typst.app/docs/reference/model/link/) <sup><span style="color:gold">element</span>, <span style="color:fuchsia">markup</span></sup> create links to a URL or a location in the document `https://typst.app/`
- [list](https://typst.app/docs/reference/model/list/) <sup><span style="color:gold">element</span>, <span style="color:fuchsia">markup</span></sup> create a bullet list `- item`
- [numbering](https://typst.app/docs/reference/model/numbering/) applies a numbering to a sequence of numbers
- [outline](https://typst.app/docs/reference/model/outline/) <sup><span style="color:gold">element</span></sup> create a table of contents, figures, or other elements
- [par](https://typst.app/docs/reference/model/par/) <sup><span style="color:gold">element</span></sup> can arranges text, spacing and inline-level elements into a paragraph
- [parbreak](https://typst.app/docs/reference/model/parbreak/) <sup><span style="color:gold">element</span>, <span style="color:fuchsia">markup</span></sup>  start a new paragraph `Blank line`
- [quote](https://typst.app/docs/reference/model/quote/) <sup><span style="color:gold">element</span></sup> displays a quote alongside an optional attribution
- [ref](https://typst.app/docs/reference/model/ref/) <sup><span style="color:gold">element</span>, <span style="color:fuchsia">markup</span></sup> create a reference to a referenceable elements or to cite from a bibliography `@intro`
- [strong](https://typst.app/docs/reference/model/strong/) <sup><span style="color:gold">element</span>, <span style="color:fuchsia">markup</span></sup> create bold text `*strong*`
- [table](https://typst.app/docs/reference/model/table/) <sup><span style="color:gold">element</span></sup> create table
- [terms](https://typst.app/docs/reference/model/terms/) <sup><span style="color:gold">element</span>, <span style="color:fuchsia">markup</span></sup> create a list of terms and their descriptions `/ Term: description`

### [Text](https://typst.app/docs/reference/text/)
- [highlight](https://typst.app/docs/reference/text/highlight/) <sup><span style="color:gold">element</span></sup> text with background color
- [linebreak](https://typst.app/docs/reference/text/linebreak/) <sup><span style="color:gold">element</span>, <span style="color:fuchsia">markup</span></sup> apply line break `\`
- [lorem](https://typst.app/docs/reference/text/lorem/) creates lorem ipsum.. text repeatedly
- [lower](https://typst.app/docs/reference/text/lower/) converts a string or content to lowercase
- [overline](https://typst.app/docs/reference/text/overline/) <sup><span style="color:gold">element</span></sup> adds a line over text
- [raw](https://typst.app/docs/reference/text/raw/) <sup><span style="color:gold">element</span>, <span style="color:fuchsia">markup</span></sup> show raw text/code with optional syntax highlighting  `` `print(1)` ``
- [smallcaps](https://typst.app/docs/reference/text/smallcaps/) displays text in small capitals
- [smartquote](https://typst.app/docs/reference/text/smartquote/) <sup><span style="color:gold">element</span>, <span style="color:fuchsia">markup</span></sup> create a language-aware quote `'single' or "double"`
- [strike](https://typst.app/docs/reference/text/strike/) <sup><span style="color:gold">element</span></sup> add strikes line through text
- [sub](https://typst.app/docs/reference/text/sub/) <sup><span style="color:gold">element</span></sup> renders text in subscript
- [super](https://typst.app/docs/reference/text/super/) <sup><span style="color:gold">element</span></sup> renders text in superscrip
- [text](https://typst.app/docs/reference/text/text/) <sup><span style="color:gold">element</span></sup> customizes the look and layout of text
- [underline](https://typst.app/docs/reference/text/underline/) <sup><span style="color:gold">element</span></sup> add underlines to text
- [upper](https://typst.app/docs/reference/text/upper/) converts a string or content to uppercase

### [Math](https://typst.app/docs/reference/math/)
- [accent](https://typst.app/docs/reference/math/accent/) <sup><span style="color:gold">element</span></sup> attaches an accent to a base
- [attach](https://typst.app/docs/reference/math/attach) subscript, superscripts, and limits
- [cancel](https://typst.app/docs/reference/math/cancel/) <sup><span style="color:gold">element</span></sup> displays a diagonal line over a part of an equation
- [cases](https://typst.app/docs/reference/math/cases/) <sup><span style="color:gold">element</span></sup> create a case distinction by curly bracket and list
- [class](https://typst.app/docs/reference/math/class/) <sup><span style="color:gold">element</span></sup> create a class element
- [equation](https://typst.app/docs/reference/math/equation/) <sup><span style="color:gold">element</span>, <span style="color:lime">referenceable</span></sup> displayed mathematical equation inline with text or as a separate block
- [frac](https://typst.app/docs/reference/math/frac/) <sup><span style="color:gold">element</span></sup> create a mathematical fraction
- [lr](https://typst.app/docs/reference/math/lr) delimiter matching
- [mat](https://typst.app/docs/reference/math/mat/) <sup><span style="color:gold">element</span></sup> create a matrix
- [op](https://typst.app/docs/reference/math/op/) <sup><span style="color:gold">element</span></sup> create a text operator in an equation.
- [primes](https://typst.app/docs/reference/math/primes/) <sup><span style="color:gold">element</span></sup> insert prime symbol, double prime or triple or .. prime symbol
- [roots](https://typst.app/docs/reference/math/roots) square and non-square roots
- [sizes](https://typst.app/docs/reference/math/sizes) forced size styles for expressions within formulas
- [styles](https://typst.app/docs/reference/math/styles) alternate letterforms within formulas
- [underover](https://typst.app/docs/reference/math/underover) delimiters above or below parts of an equation
- [variants](https://typst.app/docs/reference/math/variants) alternate typefaces within formulas
- [vec](https://typst.app/docs/reference/math/vec/) <sup><span style="color:gold">element</span></sup> create a column vector

### [Layout](https://typst.app/docs/reference/layout/)
- [align](https://typst.app/docs/reference/layout/align/) <sup><span style="color:gold">element</span></sup> aligns content horizontally and vertically
- [alignment](https://typst.app/docs/reference/layout/alignment/) where to [align](https://typst.app/docs/reference/layout/align/) something along an axis
- [angle](https://typst.app/docs/reference/layout/angle/) an angle describing a rotation
- [block](https://typst.app/docs/reference/layout/block/) <sup><span style="color:gold">element</span></sup> a block-level container can be used to separate content, size it, and give it a background or border
- [box](https://typst.app/docs/reference/layout/box/) <sup><span style="color:gold">element</span></sup> an inline-level container that sizes content
- [colbreak](https://typst.app/docs/reference/layout/colbreak/) <sup><span style="color:gold">element</span></sup> insert a column break, cut remains to next column
- [columns](https://typst.app/docs/reference/layout/columns/) <sup><span style="color:gold">element</span></sup> separates a region into multiple equally sized columns
- [direction](https://typst.app/docs/reference/layout/direction/) the four directions into which content can be laid out
- [fraction](https://typst.app/docs/reference/layout/fraction/) defines how the remaining space in a layout is distributed
- [grid](https://typst.app/docs/reference/layout/grid/) <sup><span style="color:gold">element</span></sup> arranges content in a grid, a table without lines
- [h](https://typst.app/docs/reference/layout/h/) inserts horizontal spacing into a paragraph
- [hide](https://typst.app/docs/reference/layout/hide/) <sup><span style="color:gold">element</span></sup> will hides content without affecting layout, see blank space instead
- [layout](https://typst.app/docs/reference/layout/layout/) provides access to the current outer container's (or page's, if none) size
- [length](https://typst.app/docs/reference/layout/length/) a size or distance, possibly expressed with contextual units
- [measure](https://typst.app/docs/reference/layout/measure/) <sup><span style="color:cyan">contextual</span></sup> get the layouted size of content
- [move](https://typst.app/docs/reference/layout/move/) <sup><span style="color:gold">element</span></sup> create a moved content without affecting layout
- [pad](https://typst.app/docs/reference/layout/pad/) <sup><span style="color:gold">element</span></sup> adds spacing around content
- [page](https://typst.app/docs/reference/layout/page/) <sup><span style="color:gold">element</span></sup> layouts its child onto one or multiple pages
- [pagebreak](https://typst.app/docs/reference/layout/pagebreak/) <sup><span style="color:gold">element</span></sup> a manual page break
- [place](https://typst.app/docs/reference/layout/place/) <sup><span style="color:gold">element</span></sup> places content at an absolute position
- [ratio](https://typst.app/docs/reference/layout/ratio/) a ratio of a whole
- [relative](https://typst.app/docs/reference/layout/relative/) a length in relation to some known length
- [repeat](https://typst.app/docs/reference/layout/repeat/) <sup><span style="color:gold">element</span></sup> fill content repeatedly to the available space
- [rotate](https://typst.app/docs/reference/layout/rotate/) <sup><span style="color:gold">element</span></sup> rotates content without affecting layout
- [scale](https://typst.app/docs/reference/layout/scale/) <sup><span style="color:gold">element</span></sup> create a mirror image of the content without affecting layout
- [stack](https://typst.app/docs/reference/layout/stack/) <sup><span style="color:gold">element</span></sup> arranges content and spacing horizontally or vertically
- [v](https://typst.app/docs/reference/layout/v/) inserts vertical spacing into a flow of blocks

### [Visualize](https://typst.app/docs/reference/visualize/)
- [circle](https://typst.app/docs/reference/visualize/circle/) <sup><span style="color:gold">element</span></sup> a circle with optional content insided
- [color](https://typst.app/docs/reference/visualize/color/) a color in a specific color space
- [ellipse](https://typst.app/docs/reference/visualize/ellipse/) <sup><span style="color:gold">element</span></sup> an ellipse with optional content insided
- [gradient](https://typst.app/docs/reference/visualize/gradient/) a color gradient
- [image](https://typst.app/docs/reference/visualize/image/) <sup><span style="color:gold">element</span></sup> a raster or vector graphic
- [line](https://typst.app/docs/reference/visualize/line/) <sup><span style="color:gold">element</span></sup> a line from one point to another.
- [path](https://typst.app/docs/reference/visualize/path/) <sup><span style="color:gold">element</span></sup> through a list of points, connected by Bezier curves
- [pattern](https://typst.app/docs/reference/visualize/pattern/) a repeating pattern fill
- [polygon](https://typst.app/docs/reference/visualize/polygon/) <sup><span style="color:gold">element</span></sup> is defined by its corner points and is closed automatically
- [rect](https://typst.app/docs/reference/visualize/rect/) <sup><span style="color:gold">element</span></sup> a rectangle with optional content insided
- [square](https://typst.app/docs/reference/visualize/square/) <sup><span style="color:gold">element</span></sup> a square with optional content insided
- [stroke](https://typst.app/docs/reference/visualize/stroke/) defines how to draw a line

### [Introspection](https://typst.app/docs/reference/introspection/)
- [counter](https://typst.app/docs/reference/introspection/counter/) <sup><span style="color:cyan">contextual methods</span></sup> counts through pages, elements, and more
- [here](https://typst.app/docs/reference/introspection/here/) <sup><span style="color:cyan">contextual</span></sup> provides the current location in the document
- [locate](https://typst.app/docs/reference/introspection/locate/) <sup><span style="color:cyan">contextual</span></sup> determines the location of an selected element in the document
- [location](https://typst.app/docs/reference/introspection/location/) identifies an element in the document
- [metadata](https://typst.app/docs/reference/introspection/metadata/) <sup><span style="color:gold">element</span></sup> exposes a value to the query system without producing visible content
- [query](https://typst.app/docs/reference/introspection/query/) <sup><span style="color:cyan">contextual</span></sup> finds elements in the document
- [state](https://typst.app/docs/reference/introspection/state/) <sup><span style="color:cyan">contextual methods</span></sup> manages stateful parts of your document
