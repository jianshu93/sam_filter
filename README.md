# Filter SAM file by sequence identity and alignment ratio

## Generate sensitive mappings with no SAM header
```bash
bowtie2 -p 64 -f --interleaved ./359250487_S94_L007_interleaved.fasta --seed 42 --very-sensitive -k 16 --np 1 --mp "1,1" --rdg "0,1" --rfg "0,1" --score-min "L,0,-0.05" --no-head --no-unal -S 359250487_S94_L007.sam -x pUC57.fasta

```

```bash

git clone https://github.com/jianshu93/sam_filter.git
cd sam_filter
cargo build --release
cat 359250487_S94_L007.sam ./target/release/sam_filter -i 0.98 -r 0.95 > 359250487_S94_L007.filtered.sam

```