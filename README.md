A simple program that retrieves every country's flag image from an input file and downloads it to the `images` output directory. The input file (`input.txt`) should list the short code of each country on a new line.

Additionally, it also comes with a `-f --fmt` argument that allows you to output all country codes using a format you specified to the `output.txt` file.

## Running The Program

Simply run the program using `cargo run`. You may pass the following arguments.

| Name           | Default                  | Description                                                                                                  |
| -------------- | ------------------------ | ------------------------------------------------------------------------------------------------------------ |
| `-d --data`    | `"./data"`               | The path to the data directory. This is where the input and output files are stored along downloaded images. |
| `-f --fmt`     | `"Code: {{short_code}}"` | The formatted line to append to the output file.                                                             |
| `-t --timeout` | `5`                      | The HTTPS request timeout in seconds.                                                                        |

Run with arguments.

```bash
cargo run -- -f "Short Code: {{short_code}}"
```

## Credits

- [Christian Deacon](https://github.com/gamemann)
