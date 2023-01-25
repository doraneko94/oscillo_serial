# oscillo_serial
Rust application that plots values received via serial communication like an oscilloscope.

## How to Use

### Modes

| Name | Description |
| :---: | :--- |
| plot | Plot values. |
| text | Display the retrieved value as text. |

### Options

| Name | Description |
| :---: | :--- |
| -mo | Modes. |
| -xs | X-sizze of plot screen. |
| -ys | Y-size of plot screen. |
| -de | Delimiter of elements. |
| -db | Delimiter of data blocks. |
| -ne | Number of elements. |

### Example

```Rust
> cargo run -- -mo plot -xs 100 -ys 20 -de , -db \r\n -ne 2
```