# github-actions-serial-waiter

This is a small helper tool for working with github actions and virtualbox.

Creates a UDS server at the specified `SERIAL` path and then waits for the
first client connection to be made. Once the client has connected the tool will
begin logging the console messages to stdout.  It will then search for the
specified string of text passed in as `TAG` and will exit once found.

VirtualBox should be configured with a serial port that functions as a client:

```
vboxmanage modifyvm NAME --uart1 0x3F8 4 --uartmode1 client PATH
```

## Usage

```
Usage: github-actions-serial-waiter -s SERIAL -t TAG

Options:
    -s, --serial SERIAL serial port
    -t, --tag TAG       exit on finding tag
```

## Example

```sh
github-actions-serial-waiter -s /tmp/omnios.com1 -t GITHUB-ACTIONS-READY
```
