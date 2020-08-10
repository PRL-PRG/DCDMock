# Quick and dirty assumptions

* `https://github.com/PRL-PRG/dejacode-downloader` and `DCDMock` are subdirectories of the same directory
* the working directory contains `data/mock_commit_messages.csv` (this file can be found in `DCDMock/data/mock_commit_messages.csv`
* the GitHub dataset can be found at `/dejavuii/dejacode/dataset-small/`

Since this is all temporary, all those paths are hardcoded in DCDMock.

# Tests

There are two tests that do very basic checks. They are not exhaustive by any means. They both take a while to complete.
