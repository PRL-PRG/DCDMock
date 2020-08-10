This crate is a mostyl--drop=in replacement that will patch the missing commit
messages in GH datasets using a collection of 1M real commits. The commit
messages will not reflect the messages of the original commits and can repeat.

# Quick and dirty assumptions

* `https://github.com/PRL-PRG/dejacode-downloader` and `DCDMock` are subdirectories of the same directory

Since this is all temporary, all those paths are hardcoded in DCDMock.

# Tests

There are two tests that do very basic checks. They are not exhaustive by any means. They both take a while to complete.

* the tests assume GitHub dataset can be found at `/dejavuii/dejacode/dataset-small/`

# Usage

In your `Cargo.toml` file add a line in the `[dependency]` section:

```
dcd_mock = {path="PATH_TO_DCDMOCK"}
```

The path should point to the location of the directory containing the DCDMock package.

In your project, `dcd_mock` is included like this:

```
use dcd::Database;
use dcd_mock::DCDMock;
```

Then:

```
let dcd = DCDMock::new("/dejavuii/dejacode/dataset-small/".to_string(), "PATH_TO_DCDMOCK/data/mock_commit_messages".to_string());
```

Then you can use the method from `dcd:Database` to access data from the dataset, eg.:

```
dcd.get_commit(0)
```
