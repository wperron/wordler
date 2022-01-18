# Wordler

A Wordle REPL thingy. The word is chosen at random from `src/dict.txt`, which is generated using the following command on Ubuntu:

```bash
cat /usr/share/dict/american-english | tr [:upper:] [:lower:] | egrep '^[a-z]{5}$' | iconv -f utf-8 -t ascii//translit > src/dict.txt
```