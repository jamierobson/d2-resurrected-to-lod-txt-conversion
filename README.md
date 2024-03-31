# d2-resurrected-to-lod-txt-conversion
An attempt to write converters for some txt files, to mod some D2 Resurrected content into D2 LoD

# Getting started
- You need to own Diablo 2, Lord of Destruction, and Resurrected legitimately to be able to use this tooling, and you'll need them installed to get access to reference files.
- Install both a [Casc viewer](http://www.zezula.net/en/casc/main.html) for extracting resurrected content, and an [MPQ viewer](http://www.zezula.net/en/mpq/download.html) for extracting D2 Classic/LoD content
- Follow the instructions to export the `global\excel`folders
- Install the [Rust toolchain](https://www.rust-lang.org/learn/get-started)

# Converters
1. [Runes.txt](#runestxt)

# Runes.txt
The format changes between 1.14D and 2.7 aren't too extensive:

Rename index 3 to "server"
set index 2/3 = 1 (assuming that we want to runeword to be available, and availble in single player)
delete 4,5
delete whatever is in position *RunesUsed (index 15)
Delete the last 5 entries (indices 46-50): `T1Code7	T1Param7	T1Min7	T1Max7	*eol`. Note that this means that any runeswords with a 7th effect loses that effect. Manual intervention would be required here