# ini_merge

A simple utility to merge `.ini` config files. Assumes that files have no duplicate keys.

Usage: ini_merge base_file patch_file output_file

Keys and values in patch_file will overwrite ones in base_file.