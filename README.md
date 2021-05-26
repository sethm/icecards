# BÍN Scraper

BÍN (Beygingarlýsing íslensks nútímamáls) is an online database of
Icelandic grammatical forms. It contains full declensions and
conjugations for thousands of words in the Icelandic language. This
utility can compile Anki flash cards based on this data. It is very
much a work in progress, and probably only useful for myself at the
moment.

## Usage

    USAGE:
        binscrape <wordlist> --bindata <FILE> --category <CATEGORY> --deck <FILE>
    
    FLAGS:
        -h, --help       Prints help information
        -V, --version    Prints version information
    
    OPTIONS:
        -b, --bindata <FILE>         BÍN CSV File
            --category <CATEGORY>    Wordlist file category ('nouns' or 'adjectives')
        -d, --deck <FILE>            Anki Deck output file
    
    ARGS:
        <wordlist>    List of words and definitions, tab separated, one per line

This program requires the raw *Sigrúnarsnið* format data from BÍN to work.
The data is available as a CSV file from the following page:

https://bin.arnastofnun.is/gogn/mimisbrunnur/

Download the file named `SHsnid.csv.zip`, and unzip it. When running
binscraper, use the `-c` or `--csvfile` argument to point at the unzipped
`SHsnid.csv` file.

Example wordlist files can be found in the `sample-data` directory. To test:

    binscrape --bindata ~/SHsnid.csv --deck ~/noun.apkg \
        --category nouns sample-data/nouns.csv

    binscrape --bindata ~/SHsnid.csv --deck ~/adj.apkg \
        --category adjectives sample-data/adjectives.csv

# Links

- [Beygingarlýsing íslensks nútímamáls](https://bin.arnastofnun.is/)
- [Raw data files](https://bin.arnastofnun.is/gogn/mimisbrunnur/)
