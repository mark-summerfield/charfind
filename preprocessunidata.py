#!/usr/bin/env python3
# Copyright © 2022 Mark Summerfield. All rights reserved.
# License: GPLv3

import gzip
import string


INFILE = 'UnicodeData.txt.gz'
OUTFILE = 'chardata.txt.gz'


def main():
    with gzip.open(INFILE, 'rt', encoding='utf-8') as infile:
        with gzip.open(OUTFILE, 'wt', encoding='utf-8') as outfile:
            for line in infile:
                parts = line.split(';')
                codepoint = int(parts[0], 16)
                if codepoint < 33:
                    continue
                keywords = set()
                if parts[1] == '<control>':
                    description = parts[10]
                    keywords.add('CONTROL')
                else:
                    description = parts[1]
                    keywords |= get_keywords(parts[10])
                if description.lstrip().startswith('<'):
                    continue
                keywords |= get_keywords(description)
                keywords = '\f'.join(sorted(keywords))
                outfile.write(f'{codepoint}\t{description}\t{keywords}\n')
    print('wrote', OUTFILE)


def get_keywords(part):
    words = set()
    for word in part.split():
        if len(word) < 3:
            continue
        word = word.strip(STRIP_CHARS)
        words.add(word)
        if word == 'MATHEMATICAL':
            words |= {'MATH', 'MATHS'}
    words.discard('WITH')
    return words


STRIP_CHARS = string.punctuation + string.whitespace


if __name__ == '__main__':
    main()
