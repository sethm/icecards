# Icelandic Anki Deck Generator

BÍN (Beygingarlýsing íslensks nútímamáls) is an online database of
Icelandic grammatical forms. It contains full declensions and
conjugations for thousands of words in the Icelandic language. This
utility can compile Anki flash cards based on this data. It is very
much a work in progress, and probably only useful for myself at the
moment.

This program requires the raw *Sigrúnarsnið* format data from BÍN to
work.  The data is available as a CSV file from the following page:

https://bin.arnastofnun.is/gogn/mimisbrunnur/

The program can automatically download and uncompress the correct file
on initial run.

## Usage

    USAGE:
        is-anki-gen [OPTIONS] <wordlist>

    FLAGS:
        -h, --help       Prints help information
        -V, --version    Prints version information
    
    OPTIONS:
            --binurl <URL>    URL to fetch BÍN CSV [default:
                              https://bin.arnastofnun.is/django/api/nidurhal/?file=SHsnid.csv.zip]
        -d, --deck <FILE>     Anki Deck output file [default: deck.apkg]
            --deck-id <ID>    Optional numeric ID for the generated Anki deck

    ARGS:
        <wordlist>    List of words and categories, tab separated, one per line

The optional argument `--deck-id` can be used to set the Anki ID on 
the generated deck. If not supplied, a random ID based on the current
timestamp will be used.

Anki uses these IDs in its internal database to keep track of what
decks contain what cards, so they really do have to be unique. Setting
IDs for your own decks rather than using random IDs is the preferred
way, because you can regenerate decks at any time after adding more
words to the word list, and only the new words will be imported into
Anki on the next import.

# Links

- [genanki-rs](https://crates.io/crates/genanki-rs)
- [Anki Apps](https://apps.ankiweb.net/)
- [Beygingarlýsing íslensks nútímamáls](https://bin.arnastofnun.is/)
- [Raw data files](https://bin.arnastofnun.is/gogn/mimisbrunnur/)
