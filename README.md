# Rust Lambda Function for Summary Statistics

This project is a Rust-based AWS Lambda function that calculates summary statistics from a JSON input file and writes the results to an output text file.

## Getting Started

### Prerequisites

- Rust: You can install Rust from the official website [here](https://www.rust-lang.org/tools/install).
- AWS CLI: You can install AWS CLI from the official AWS guide [here](https://aws.amazon.com/cli/).
- AWS SAM CLI: You can install AWS SAM CLI from the official AWS guide [here](https://docs.aws.amazon.com/serverless-application-model/latest/developerguide/serverless-sam-cli-install.html).

### Dependencies

This project uses the following Rust crates:

- `anyhow`
- `lambda_runtime`
- `log`
- `serde_json`
- `tokio`
- `env_logger`

You can add these dependencies to your project by including them in your `Cargo.toml` file.

### Installation

1. Clone the repository:

```bash
git clone https://gitlab.com/osama-shawir/rust-lambda-function.git
cd rust-lambda-function
```

2. Build the project:

```bash
cargo build --release
```

3. Create a deployment package:

```bash
zip function.zip target/release/rust-lambda-function
```

4. Create a new Lambda function:

```bash
aws lambda create-function --function-name rust-lambda-function --handler doesn't.matter --zip-file fileb://function.zip --runtime provided --role arn:aws:iam::YOUR_AWS_ACCOUNT_ID:role/service-role/YourServiceRoleForLambda
```

Replace YOUR_AWS_ACCOUNT_ID with your AWS account ID and YourServiceRoleForLambda with your Lambda service role.

### Testing

After deploying the Lambda function, you can test it by invoking it with a JSON file as input:

```bash
aws lambda invoke --function-name rust-lambda-function --payload file://input.json output.txt
```
Replace input.json with the path to your input JSON file. The results will be written to output.txt.
To expose your function via HTTP, you can create an API Gateway and connect it to your Lambda function. The input JSON file can be sent as a POST request to the API Gateway endpoint and the results will be returned in the response. For security reasons, we will not be posting the API gateway constructed for this project publically here, but a screenshot with redacted information can be found below.

#### Request:

![alt text](image.png)

#### Response:

![alt text](image-1.png)

#### Sample input JSON file:

```json

[1.0, 2.0, 3.0, 4.0, 5.0]

```

#### Sample output text file:

```txt
{"max":5.0,"mean":3.0,"median":3.0,"min":1.0,"std_dev":1.4142135623730951}
```
