# hangman-rust-cli
Hangman in Rust (CLI)

Word list was taken from a [10,000 most common English words list](https://github.com/first20hours/google-10000-english) list and sorted by length of word for difficulty selection. 

### Select difficulty:

After picking a difficulty, app will load words-{difficulty}.txt and pick a word at random. Note that for the easy words, the app will not allow a word less than MIN_SIZE (3) to be picked.
