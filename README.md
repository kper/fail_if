# fail_if

fail_if is a tool which takes a string as input, a command and checks the output of the executed command. If the command crashes or the stdout/stderr contains a given string then fail_if will a non-zero exit code.

## Example

```
./target/debug/fail_if "123" echo "123"
# Exit Code: 1
``