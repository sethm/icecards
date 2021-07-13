# Icelandic Anki Deck Generator

BÍN (Beygingarlýsing íslensks nútímamáls) is an online database of
Icelandic grammatical forms. It contains full declensions and
conjugations for thousands of words in the Icelandic language. This
utility can compile Anki flash cards based on this data. It is very
much a work in progress, and probably only useful for myself at the
moment.

This program requires the raw *Sigrúnarsnið* format data from BÍN to
work.  The data is available as a CSV file from the following page:

- https://bin.arnastofnun.is/gogn/mimisbrunnur/

The program can automatically download and uncompress the correct file
on initial run.

## Usage

    USAGE:
        is-anki-gen [OPTIONS] <wordlist>
    
    FLAGS:
        -h, --help       Prints help information
        -V, --version    Prints version information
    
    OPTIONS:
            --binurl <URL>                 URL to fetch BÍN CSV [default:
                                           https://bin.arnastofnun.is/django/api/nidurhal/?file=SHsnid.csv.zip]
            --deck <FILE>                  Anki Deck output file [default: deck.apkg]
            --description <DESCRIPTION>    Anki Deck description [default: Deck for studying Icelandic Vocabulary]
            --name <NAME>                  Anki Deck name [default: Icelandic Vocabulary]
    
    ARGS:
        <wordlist>    List of words, categories, and definitions (tab separated)

The input should be a list of root words, their category, and their
definition, tab-separated. For example:

    birta    noun        light, brightness
    dagblað  noun        daily newspaper
    ungur    adjective   young
    vinsæll  adjective   popular
    læra     verb        to learn, study
    tala     verb        to talk, speak

The categories *noun*, *adjective*, and *verb* are currently supported.

An example input file can be found in the **sample-data** directory.

# Links

- [genanki-rs](https://crates.io/crates/genanki-rs)
- [Anki Apps](https://apps.ankiweb.net/)
- [Beygingarlýsing íslensks nútímamáls](https://bin.arnastofnun.is/)
- [Raw data files](https://bin.arnastofnun.is/gogn/mimisbrunnur/)
