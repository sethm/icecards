# BÍN Scraper

BÍN (Beygingarlýsing íslensks nútímamáls) is an online database of
Icelandic grammatical forms. It contains full declensions and
conjugations for thousands of words in the Icelandic language. This
utility can compile Anki flash cards based on this data. It is very
much a work in progress, and probably only useful for myself at the
moment.

## Usage

    USAGE:
        binscrape [OPTIONS] <wordlist> --bindata <FILE> --category <CATEGORY> --deck <FILE>
    
    FLAGS:
        -h, --help       Prints help information
        -V, --version    Prints version information
    
    OPTIONS:
        -b, --bindata <FILE>         BÍN CSV File
            --category <CATEGORY>    Wordlist file category ('nouns' or 'adjectives')
        -d, --deck <FILE>            Anki Deck output file
            --deck-id <ID>           Optional numeric ID for the generated Anki deck
            --model-id <ID>          Optional numeric ID for the generated Anki model
    
    ARGS:
        <wordlist>    List of words and definitions, tab separated, one per line

This program requires the raw *Sigrúnarsnið* format data from BÍN to work.
The data is available as a CSV file from the following page:

https://bin.arnastofnun.is/gogn/mimisbrunnur/

Download the file named `SHsnid.csv.zip`, and unzip it. When running
binscraper, use the `-c` or `--csvfile` argument to point at the unzipped
`SHsnid.csv` file.

Example wordlist files can be found in the `sample-data` directory. To test:

    binscrape --bindata SHsnid.csv --deck noun.apkg \
        --category nouns sample-data/nouns.csv

    binscrape --bindata SHsnid.csv --deck adj.apkg \
        --category adjectives sample-data/adjectives.csv

The optional arguments `--deck-id` and `--model-id` can be used to set IDs on the
generated deck and model. If not supplied, random IDs based on the current 
timestamp will be used. 

Anki uses these IDs in its internal database to keep track of what decks contain what
cards, so they really do have to be unique. Setting IDs for your own decks rather than
using random IDs is the preferred way, because you can regenerate decks at any time 
after adding more words to the word list, and only the new words will be imported
into Anki on the next import.

# Links

- [genanki-rs](https://crates.io/crates/genanki-rs)
- [Anki Apps](https://apps.ankiweb.net/)
- [Beygingarlýsing íslensks nútímamáls](https://bin.arnastofnun.is/)
- [Raw data files](https://bin.arnastofnun.is/gogn/mimisbrunnur/)
