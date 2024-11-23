# Silkroad Patch Compressor

(De)Compresses files for Silkroad Online such that they can be served from any normal file server and the client
will accept them.

Silkroad uses some weird LZMA setup that may not be repeatable in all scenarios.
Therefor, it would be easier to do the compression separately, such that the file
server can simply serve the files as usual.

This is an MVP essentially - it works, but isn't nice to use.

## Usage

Compress:

```shell
$ patch-compressor file.txt compressed.txt
```

Decompress:

```shell
$ patch-compressor -d compressed.txt file.txt
```