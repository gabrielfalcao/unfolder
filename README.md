# unfolder

unfold a file into a folder and fold a previously unfolded folder into a file


```shell
dd if=/dev/random bs=4 count=211776 of=random-file.bin
unfolder unfold random-file.bin random-file-unfolded
unfolder fold random-file-unfolded random-folded.bin
```
