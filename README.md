# SHAAA - SHA3 实现
> SH + A * 3

## 实例

使用以下方式获得内部状态的输出：
```
$ echo -n "DATA" | cargo run --bin file --release --features internal -- --variant 512
[...verbose output...]
```

对于 ISO 文档中的实例：
- **SHA-3-224**: ``
  - `6b4e03423667dbb73b6e15454f0eb1abd4597f9a1b078e3f5b5a6bc7`
  - 具体输出位于 outputs/SHA-3-224-empty.txt
- **SHA-3-224**: Bit-stream: 1 1 0 0 1
  - 需要对源代码略加修改，因为之前通过 Pipe 传输数据时最小单位就是字节
  - `ffbad5da96bad71789330206dc6768ecaeb1b32dca6b3301489674ab`
  - 具体输出位于 outputs/SHA-3-224-short.txt
- **SHA-3-224**: Bit-stream: 1 1 0 0 1 0 1 0 0 0 0 1 1 0 1 0 1 1 0 1 1 1 1 0 1 0 0 1 1 0
  - `d666a514cc9dba25ac1ba69ed3930460deaac9851b5f0baab007df3b`
  - 具体输出位于 outputs/SHA-3-224-long.txt

- **SHA-3-256**: ``
  - `a7ffc6f8bf1ed76651c14756a061d662f580ff4de43b49fa82d80a4b80f8434a`
  - 具体输出位于 outputs/SHA-3-256-empty.txt
- **SHA-3-256**: Bit-stream: 1 1 0 0 1
  - 需要对源代码略加修改，因为之前通过 Pipe 传输数据时最小单位就是字节
  - `7b0047cf5a456882363cbf0fb05322cf65f4b7059a46365e830132e3b5d957af`
  - 具体输出位于 outputs/SHA-3-256-short.txt
- **SHA-3-256**: Bit-stream: 1 1 0 0 1 0 1 0 0 0 0 1 1 0 1 0 1 1 0 1 1 1 1 0 1 0 0 1 1 0
  - `c8242fef409e5ae9d1f1c857ae4dc624b92b19809f62aa8c07411c54a078b1d0`
  - 具体输出位于 outputs/SHA-3-256-long.txt

- **SHA-3-384**: ``
  - `0c63a75b845e4f7d01107d852e4c2485c51a50aaaa94fc61995e71bbee983a2ac3713831264adb47fb6bd1e058d5f004`
  - 具体输出位于 outputs/SHA-3-384-empty.txt
- **SHA-3-384**: Bit-stream: 1 1 0 0 1
  - 需要对源代码略加修改，因为之前通过 Pipe 传输数据时最小单位就是字节
  - `737c9b491885e9bf7428e792741a7bf8dca9653471c3e148473f2c236b6a0a6455eb1dce9f779b4b6b237fef171b1c64`
  - 具体输出位于 outputs/SHA-3-384-short.txt
- **SHA-3-384**: Bit-stream: 1 1 0 0 1 0 1 0 0 0 0 1 1 0 1 0 1 1 0 1 1 1 1 0 1 0 0 1 1 0
  - `955b4dd1be03261bd76f807a7efd432435c417362811b8a50c564e7ee9585e1ac7626dde2fdc030f876196ea267f08c3`
  - 具体输出位于 outputs/SHA-3-384-long.txt

- **SHA-3-512**: ``
  - `a69f73cca23a9ac5c8b567dc185a756e97c982164fe25859e0d1dcc1475c80a615b2123af1f5f94c11e3e9402c3ac558f500199d95b6d3e301758586281dcd26`
  - 具体输出位于 outputs/SHA-3-512-empty.txt
- **SHA-3-512**: Bit-stream: 1 1 0 0 1
  - 需要对源代码略加修改，因为之前通过 Pipe 传输数据时最小单位就是字节
  - `a13e01494114c09800622a70288c432121ce70039d753cadd2e006e4d961cb27544c1481e5814bdceb53be6733d5e099795e5e81918addb058e22a9f24883f37`
  - 具体输出位于 outputs/SHA-3-512-short.txt
- **SHA-3-512**: Bit-stream: 1 1 0 0 1 0 1 0 0 0 0 1 1 0 1 0 1 1 0 1 1 1 1 0 1 0 0 1 1 0
  - `9834c05a11e1c5d3da9c740e1c106d9e590a0e530b6f6aaa7830525d075ca5db1bd8a6aa981a28613ac334934a01823cd45f45e49b6d7e6917f2f16778067bab`
  - 具体输出位于 outputs/SHA-3-512-long.txt


## 速度

使用以下方式运行可以随机生成 16 * 128KiB = 2MiB 的随机数据，进行 Digest

```bash
$ cargo run --release --bin test --quiet -- --variant 224
Digest:
1e03422c388e9d4a3d6f9d344e6c6feecd11b0b51aa4398426f16235
Time:
0.009309400s
Speed:
214.83661675295937 MiB/s

$ cargo run --release --bin test --quiet -- --variant 256
Digest:
bf3e801a8715c6a1a8daa6a57561aba22f5675bd42b3fa408288b0b156d91491
Time:
0.009503600s
Speed:
210.44656761648218 MiB/s

$ cargo run --release --bin test --quiet -- --variant 384
Digest:
76f237b23126518e0fd98ac5f694453b10db613c0491dbd2f9b1678e09536faead090945833a6a00ba2a08d56089e605
Time:
0.012255200s
Speed:
163.1960310725243 MiB/s

$ cargo run --release --bin test --quiet -- --variant 512
Digest:
283ba6c87fa6c5b44a1dcaa92f24677d9062310a6df5bbfa48dfde67a1dc0e5e3e29a96ac081afb4300f0c9dd1c0ef7decd917ec2316316a82506349dbe7d8d4
Time:
0.016661300s
Speed:
120.03865244608764 MiB/s
```

速度分别为:
- **SHA-3-224**: 214.8 MiB/s
- **SHA-3-256**: 210.4 MiB/s
- **SHA-3-384**: 163.2 MiB/s
- **SHA-3-512**: 120.0 MiB/s

可以发现，SHA-3-224 和 SHA-3-256 的速率接近，而 SHA-3-512 速率只有接近一半。考虑每次吸收到状态中的比特数:
- **SHA-3-224**: 1152
- **SHA-3-256**: 1088
- **SHA-3-384**: 832
- **SHA-3-512**: 576

基本和速率成正比。

由于没有有效的对比参考，因此我也不知道这个速率是高是低...Profiler 给出的统计是每一轮中 Rho + Pi + Chi 和 Theta 所消耗的时间对半分。

Theta 总共遍历了两次整个状态，Rho + Pi + Chi 也是两次，而且多数循环都是可以在 3 到 4 个周期内完成，因此时间接近。

进一步可以把 Theta 的第一部分和 Rho + Pi 使用 SSE / AVX2 并行。Theta 的第一部分是一个五列的 Xor，Rho + Pi 是一个查表，向量化后分别可以用 SSE2 和 AVX2 解决。

但是这个功能在 Rust 内没有 stablize，我也没有找到我能看懂的文档，所以只能摸鱼。
