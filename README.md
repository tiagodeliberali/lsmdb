# LSM DB
In the future, this project will be a Log-structured tree based database. For now, it is just a bunch of related data structures written in rust.

## Testing over a nice set of words
For now, the project simple try to create an BST on memory based on `leipzig1M.txt` inside `data` folder.

To download this file:

```
mkdir data
curl -o ./data/leipzig1M2.txt https://algs4.cs.princeton.edu/31elementary/leipzig1M.txt
```

After download the file, just run: 

```
cargo run --release
```