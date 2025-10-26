
command

# help
```

```

# debug parameter

```bash
cargo run -- -d
```

# command
```bash
  cargo run -- get http://www.sina.com.cn
  cargo run -- post http://www.sina.com.cn  a=11 b=44
```

```bash
  cargo run -- get http://www.sina.com.cn
cargo run -- post http://dd.com a=b c=d
```


#kv parse
```bash
http_pie$ cargo run -- post http://www.abc.com c=dd  d=33
```

# clap
- clap version 3.0 才能编译通过
- clap verison 4.5 版本修改比较大，原来代码会编译不过

# tokei 统计代码行数 [tokei](https://github.com/XAMPPRocky/tokei)
- install tokei
```bash
cargo install --git https://github.com/XAMPPRocky/tokei.git tokei

```

- tokei 统计代码行数
  ```bash
tokei ./
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 Language              Files        Lines         Code     Comments       Blanks
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 TOML                      1           14           13            0            1
─────────────────────────────────────────────────────────────────────────────────
 Markdown                  2           28            0           18           10
 |- BASH                   1            6            6            0            0
 |- Rust                   1            1            1            0            0
 (Total)                               35            7           18           10
─────────────────────────────────────────────────────────────────────────────────
 Rust                      1          206          173            8           25
 |- Markdown               1            2            0            2            0
 (Total)                              208          173           10           25
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 Total                     4          257          193           28           36
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  ```

  

  ```bash
  tokei ./ --sort code
tokei ./ --files
  ```


  ```bash
  http_pie get https://www.sina.com.cn


  
  ```