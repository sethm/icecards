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
        icecards [OPTIONS] <wordlist>
    
    FLAGS:
        -h, --help       Prints help information
        -V, --version    Prints version information
    
    OPTIONS:
            --description <DESCRIPTION>    Anki deck description
            --name <NAME>                  Anki deck name
            --output <FILE>                Anki deck output file [default: deck.apkg]
    
    ARGS:
        <wordlist>    List of words, categories, and definitions (tab separated)


The arguments `--description`, `--name`, and `--output` are optional.  If not
supplied, the default deck will be named *"Icelandic Vocabulary"*, and will be
written to the file `deck.apkg`.

The input should be a list of root words, their category, and their definition,
tab-separated. For example:

    birta    noun        light, brightness
    dagblað  noun        daily newspaper
    ungur    adjective   young
    vinsæll  adjective   popular
    læra     verb        to learn, study
    tala     verb        to talk, speak

The categories *noun*, *adjective*, *verb*, *pronoun*, and *phrase* are
currently supported.

# Sample Data

An example input file can be found in the **sample-data** directory.

This vocabulary list was compiled from the *Icelandic Online 1* course
available at [https://icelandiconline.com/](https://icelandiconline.com/).

# Links

- [genanki-rs](https://crates.io/crates/genanki-rs)
- [Anki Apps](https://apps.ankiweb.net/)
- [Beygingarlýsing íslensks nútímamáls](https://bin.arnastofnun.is/)
- [Raw data files](https://bin.arnastofnun.is/gogn/mimisbrunnur/)
