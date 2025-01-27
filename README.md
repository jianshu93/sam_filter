# Filter SAM file by sequence identity and alignment ratio

## usage
```bash
Filters CIGAR strings by sequence identity and alignment ratio in SAM files

Usage: sam_filter [OPTIONS] --identity <min_identity> --ratio <min_ratio>

Options:
  -i, --identity <min_identity>  Minimum matching identity as a float
  -r, --ratio <min_ratio>        Minimum query alignment ratio as a float
      --reverse                  If used, print lines where identity <= min_identity (ignoring ratio), otherwise print lines where identity > min_identity and ratio > min_ratio
  -h, --help                     Print help
  -V, --version                  Print version

```

## Generate sensitive mappings without SAM header (Bowtie2 v2.5.4 or later)
```bash
### interleaved mode
bowtie2 -p 64 -f --interleaved ./359250487_S94_L007_interleaved.fasta --seed 42 --very-sensitive -k 16 --np 1 --mp "1,1" --rdg "0,1" --rfg "0,1" --score-min "L,0,-0.05" --no-head --no-unal --no-1mm-upfront -S 359250487_S94_L007.sam -x pUC57.fasta

### R1 and R2 mode
bowtie2 -p 64 -q -1 359250487_S94_L007_R1.fastq.gz -2 359250487_S94_L007_R2.fastq.gz --seed 42 --very-sensitive -k 16 --np 1 --mp "1,1" --rdg "0,1" --rfg "0,1" --score-min "L,0,-0.05" --no-head --no-unal --no-1mm-upfront -S 359250487_S94_L007.sam -x pUC57.fasta

### R1 and R2 seperate mapping
bowtie2 -p 64 -q 359250487_S94_L007_R1.fastq.gz --seed 42 --very-sensitive -k 16 --np 1 --mp "1,1" --rdg "0,1" --rfg "0,1" --score-min "L,0,-0.05" --no-head --no-unal --no-1mm-upfront -S 359250487_S94_L007_R1.sam -x pUC57.fasta

bowtie2 -p 64 -q 359250487_S94_L007_R1.fastq.gz --seed 42 --very-sensitive -k 16 --np 1 --mp "1,1" --rdg "0,1" --rfg "0,1" --score-min "L,0,-0.05" --no-head --no-unal --no-1mm-upfront -S 359250487_S94_L007_R2.sam -x pUC57.fasta

```

```bash

git clone https://github.com/jianshu93/sam_filter.git
cd sam_filter
cargo build --release
### extract high identity matches
cat data/test.sam | ./target/release/sam_filter -i 0.98 -r 0.90 > test.filtered.sam

### extract low identity matches
cat data/test.sam | ./target/release/sam_filter -i 0.98 -r 0.90 --reverse > test.filtered.sam


### Extracting match or unmatched reads from original fasta file after the filtering above. Seqkit can be used (newest version v2.9.0 or later)

for R1_file in *_R1.*.sam; do base=${R1_file%_R1.*}; R2_file="${base}_R2.filtered.sam"; cat $R1_file | awk '{print $1}' > ${base}_R1.match.txt; seqkit grep -f ${base}_R1.match.txt /qmounts/qiita_data/per_sample_FASTQ/194282/${base}_R1_001.trimmed.fastq.gz > ../filter_matched_fasta/${base}_R1.fastq; seqkit grep -v -f ${base}_R1.match.txt /qmounts/qiita_data/per_sample_FASTQ/194282/${base}_R1_001.trimmed.fastq.gz > ../filter_unmatched_fasta/${base}_R1.fastq; cat $R2_file | awk '{print $1}' > ${base}_R2.match.txt; seqkit grep -f ${base}_R2.match.txt /qmounts/qiita_data/per_sample_FASTQ/194282/${base}_R2_001.trimmed.fastq.gz > ../filter_matched_fasta/${base}_R2.fastq; seqkit grep -v -f ${base}_R2.match.txt /qmounts/qiita_data/per_sample_FASTQ/194282/${base}_R2_001.trimmed.fastq.gz > ../filter_unmatched_fasta/${base}_R2.fastq; done

```
