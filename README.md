# FASTQE in Rust

A Rust and WebAssembly beta implementation of FASTQE (https://fastqe.com): only command line options with a  ✅  are functional  

 ```
Usage: fastqe [OPTIONS] [FASTQ_FILE]...

Arguments:
  [FASTQ_FILE]...  Input FASTQ files

Options:
      --noheader              Hide the header before sample output ❌
      --output   Write output to OUTPUT_FILE instead of stdout
      --long     Buffer memory for long reads up to READ_BUFFER bp long ❌ [default: 500]
      --log         Record program progress in LOG_FILE ✅
  -h, --help                  Print help
  -V, --version               Print version

Emoji options:
      --bin                   Use binned scores (🚫 💀 💩 🚨 😄 😆 😎 😍 ) ❌ 
      --custom   Use a mapping of custom emoji to quality in CUSTOM_DICT (🐍🌴) ❌
      --noemoji               Use mapping without emoji (▁▂▃▄▅▆▇█) ❌

Statistics options:
      --minlen   Minimum length sequence to include in stats ✅ [default: 0]
      --scale       Show relevant scale in output ❌
      --nomean      Hide mean quality per position ❌
      --min         Show minimum quality per position  ❌
      --max         Show maximum quality per position ❌

HTML report options:
      --html         Output all data as html ❌
      --window    Window length to summarise reads in HTML report ❌ [default: 1]
      --html_escape  ❌ Escape html within output, e.g. for Galaxy parsing

```


## Release

Each release contains the biowasm compatible files: https://github.com/fastqe/fastqe_rust/releases/latest

Files are hosted under: https://fastqe.github.io/fastqe_rust/

Example of sourcing files and Biowasm/Aoili compatible code is also at: https://fastqe.github.io/fastqe_rust/index.html

## Build

Emscriptem is needed; GitHub actions will do this as part of build process, `scripts/setup_emsdk.sh` also should work to setup local environment or other integrations. 

 Requires: 
   - rustc 1.75.0-beta.5 or greater due to `open64` bugs.
   - emcc 3.1.50

Tag a release and use `git push  --tags` to update both the hosted files and website. 

 
## Data

Example data is located in the `data` folder and is available in the virtual filesystem under `/shared/fastqe/` when `fastqe.data` is mounted. It contains the `fastq` files from the FASTQE lesson from QUBES:

```
Rachael St. Jacques, Max Maza, Sabrina Robertson, Guoqing Lu, Andrew Lonsdale, Ray A Enke (2019). A Fun Introductory Command Line Exercise: Next Generation Sequencing Quality Analysis with Emoji!. NIBLSE Incubator: Intro to Command Line Coding Genomics Analysis, (Version 2.0). QUBES Educational Resources. doi:10.25334/Q4D172
```

