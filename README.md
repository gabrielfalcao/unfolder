# unfolder

unfold a file into a folder and fold a previously unfolded folder into a file

```shell
$ dd if=/dev/random bs=4 count=211776 of=random-file.bin
```

```shell
$ unfolder unfold random-file.bin random-file-unfolded
```

```shell
$ unfolder fold random-file-unfolded random-folded.bin
```
