# ansi_blue_on_yello

This is just a tiny demo usage of ansi_term.

## hack/

```shell
podman build -f Containerfile.rust-builder -t rust:1.62.2
podman build -f Containerfile -t tterm
podman run --rm -ti tterm
```
