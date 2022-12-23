# mlg
MLG - Markup Language Generator

I think HTML is too verbose. 
The goal of this project is to write a generator which will parse a less verbose language and generate valid html files.

## Usage 

```sh
$ mlg [FLAGS]
```

| Flag | Function                                     |
|------|----------------------------------------------|
| -h   | Prints a help dialogue                       |
| -p   | Designate the input file path *[REQUIRED]*   |
| -o   |  Designate an output file path *[OPTIONAL]*  |

By deafult the output file will be produced next to the input file with the name `out.html`

## Example

```sh
$ mlg -p ./examples/basic.mlg
$ HTML GENERATED: ./examples/out.html
```