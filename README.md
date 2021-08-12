# example-rust-app

## Installing AWS SAM

```bash
wget https://github.com/aws/aws-sam-cli/releases/latest/download/aws-sam-cli-linux-x86_64.zip
unzip aws-sam-cli-linux-x86_64.zip -d sam-installation
sudo ./sam-installation/install --update
```

## Installing Rust

```bash
curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain nightly
source $HOME/.cargo/env
```

## Testing

```bash
sam build
nohup sam local start-lambda --template .aws-sam/build/template.yaml &
cargo test -- --nocapture
```

## Updating the Infrastructure
```bash
aws cloudformation deploy --stack-name ExampleCodeBuildProject --template-file ./templates/infrastructure.yaml --capabilities CAPABILITY_IAM
```