# ini_merge

A simple utility to merge `.ini` config files. Assumes that files have no duplicate keys.

```
Usage: ini_merge base_file patch_file output_file
```

Semantics of ini_merge behavior:
 - Comments are stripped and not parsed
 - Keys that do not overlap will both be inserted into the output, with `patch_file` keys coming after `base_file` keys
 - Keys and values in `patch_file` will overwrite ones in `base_file`
 - Keys overwritten with `patch_file` changes will be placed separately from keys in a section
 - If a key occurs multiple times in `base_file`, all occurences of it will be removed if replaced by a key in the `patch_file`.
 - If a key occurs multiple times in `patch_file`, only the "final" occurence of the key will be included in output_file
