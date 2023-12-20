# fastqe_rust

## Release

Each release contains the biowasm compatible files: https://github.com/fastqe/fastqe_rust/releases/latest

Files are hosted at:

Example Biowasm at:

## Build

Emscriptem is needed; GitHub actions will do this as part of build process, `scripts/setup_emsdk.sh` also should work to setup local environment.. 


## Data

Example data is located in the `data` folder and is available in the virtual filesystem under `/shared/fastqe/`. It contains the `fastq` files from the FASTQE lesson from QUBES:

```
Rachael St. Jacques, Max Maza, Sabrina Robertson, Guoqing Lu, Andrew Lonsdale, Ray A Enke (2019). A Fun Introductory Command Line Exercise: Next Generation Sequencing Quality Analysis with Emoji!. NIBLSE Incubator: Intro to Command Line Coding Genomics Analysis, (Version 2.0). QUBES Educational Resources. doi:10.25334/Q4D172
```
