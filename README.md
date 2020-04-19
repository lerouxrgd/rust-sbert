# Rust SBert

Rust port of [sentence-transformers](https://github.com/UKPLab/sentence-transformers) using [rust-bert](https://github.com/guillaume-be/rust-bert), [tch-rs](https://github.com/LaurentMazare/tch-rs) and [rust-tokenizers](https://github.com/guillaume-be/rust-tokenizers).

Might consider replacing [rust-tokenizers](https://github.com/guillaume-be/rust-tokenizers) with Hugging Face's [tokenizers](https://github.com/huggingface/tokenizers/tree/master/tokenizers).

## Supported models

### Multilingual Models

- **distiluse-base-multilingual-cased**: Supported languages: Arabic, Chinese, Dutch, English, French, German,  Italian, Korean, Polish, Portuguese, Russian, Spanish, Turkish. Performance on the extended STS2017: 80.1

## Usage

### Example

The API is made to be very easy to use and enables you to create a sentence embedding very simply.

Load SBert model with weights by specifying the directory of the model:

```Rust
let mut home: PathBuf = env::current_dir().unwrap();
home.push("path-to-model");

let sbert_model = SBert::new(home.to_str().unwrap());
```

Encode a sentence and get its sentence embedding:

```Rust
let texts = ["You can encode",
             "As many sentences",
             "As you want",
             "Enjoy ;)"];

let output = sbert_model.encode(texts.to_vec()).unwrap();
```

Then you can use the `output` sentence embedding in any application you want. 

### Convert models from Python to Rust

To be able to use the models provided [here](https://public.ukp.informatik.tu-darmstadt.de/reimers/sentence-transformers/v0.2/) by UKPLabs, you need to run this script to convert the model in a suitable format:

```Bash
cd model-path/
python3 utils/prepare_distilbert.py
```