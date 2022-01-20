#!/usr/bin/env python3
# Copyright © 2022 Mark Summerfield. All rights reserved.
# License: GPLv3

# Unicode data:
#   https://www.unicode.org/Public/UCD/latest/ucdxml/ucd.nounihan.flat.zip
#   http://www.unicode.org/reports/tr42/

import gzip
import io
import re
import string
import time
import xml.dom.minidom
import zipfile

INFILE = 'ucd.nounihan.flat.zip'
OUTFILE = 'chardata.txt.gz'


def main():
    t = time.monotonic()
    print(f'reading {INFILE} …', flush=True)
    with zipfile.ZipFile(INFILE) as zinfile:
        with zinfile.open(INFILE.replace('.zip', '.xml')) as binfile:
            with io.TextIOWrapper(binfile, 'utf-8') as infile:
                dom = xml.dom.minidom.parse(infile)
    print(f'writing {OUTFILE} …', flush=True)
    with gzip.open(OUTFILE, 'wt', encoding='utf-8') as outfile:
        for element in dom.getElementsByTagName('char'):
            try:
                cp = int(element.getAttribute('cp'), 16)
            except ValueError:
                continue
            if cp < 33:
                continue
            ws = element.getAttribute('WSpace')
            if ws in 'Yy':
                continue
            ignore = math = dash = False
            aliases = set()
            for subelement in element.getElementsByTagName('name-alias'):
                kind = subelement.getAttribute('type')
                if kind in {'control', 'figment'}:
                    ignore = True
                    break
                alias = subelement.getAttribute('alias')
                if alias:
                    aliases |= settle(alias)
            if ignore:
                continue
            name = element.getAttribute('na').upper()
            name1 = element.getAttribute('na1')
            if not name and name1:
                name = name1
            name1 = settle(name1)
            blocks = settle(element.getAttribute('blk'))
            if not name and aliases:
                lst = sorted(aliases, key=lambda a: len(a))
                name = lst[-1]
                aliases.discard(name)
            if not name:
                continue
            keywords = blocks | name1 | settle(name)
            for alias in aliases:
                keywords |= settle(alias)
            keywords -= {'WITH', 'OF'}
            if keywords & {'ACCENT', 'COMBINING', 'COMPATIBILITY',
                           'IDEOGRAPH', 'INDICATOR', 'MODIFIER', 'PRIVATE',
                           'SYLLABLE', 'VARIATION'}:
                continue
            if math or 'MATHEMATICAL' in keywords:
                keywords |= {'MATHEMATICAL', 'MATH', 'MATHS'}
            if dash:
                keywords |= {'DASH', 'HYPHEN'}
            keywords = '\v'.join(sorted(keywords))
            outfile.write(f'{cp:X}\t{name}\t{keywords}\n')
    print(f'wrote {OUTFILE} • {time.monotonic() - t:.01f} secs', flush=True)


def settle(text):
    extras = set()
    words = set([word.strip(STRIP_CHARS)
                for word in re.split(r'[-_\s]+', text.upper()) if word])
    for word in words:
        if len(word) > 5 and word.endswith('WARDS'):
            extras.add(word[:-5])
    return words | extras


STRIP_CHARS = string.punctuation + string.whitespace


if __name__ == '__main__':
    main()
