# *f*uzzy *A*WS *sts* *A*ssume*R*ole
A small program to switch between
[AWS Roles](https://docs.aws.amazon.com/STS/latest/APIReference/API_AssumeRole.html)
quickly using fuzzy matching

![](Why-Cant-I-Hold-All-These-AWS-Accounts.jpg)

## Features
- Assume AWS role with minimum keystrokes
- All AWS roles accessible in one place
- Pre-selects last used account/role (TODO)
- Only requires user input if there is more than one account/role

## Usage
Fastsar can be configured via command line arguments,
see `fastsar -h` for more details.

### Setting environment variables for assumed role
Most shells do not allow to set environments from within sub-shells or programs,
so without any argument fastsar will returns a string
that can be evaluated in the current shell.

In bash this can be done with `eval $(fastsar)`,
in fish this can be done with `eval (fastsar)`.

You can also bind the evaluation to some hotkey, like 'Alt+s':

- bash: `bind -x '"\es": eval $(fastsar)'`
- fish: `bind \es 'eval (fastsar)'`

### Shell auto detection
Fastsar will try to detect your shell by reading the environment variable
`SHELL`.
If fastsar has problems detecting your shell, you can pass the shell's name
as parameter (e.g. `fastsar -s fish`).

### Multi-factor authentication
If the role you like to assume requires the use of MFA you can pass the "MFA
serial number" (ARN of the MFA device) and MFA token as parameters (e.g.
`fastsar -m arn:aws:iam::123123123:mfa/user -t 123123`).

### Profiles
TODO: Support profiles

### Non-interactive mode
Account id and role name can be passed as parameters to skip the interactive
selection (e.g. `fastsar -a 123123123 -r user`).

### Use a different region
Fastsar will use the default region specified in `~/.aws/config`.
If no region is set it will fallback to us-east-1.
If a different region should be used it can be passed as parameter (e.g.
`fastsar -R eu-central-1`).

### Execute shell command with assumed role
Instead of printing and evaluating environment variables to use the assumed
role in your shell, you can pass a shell command to fastsar and have it
executed for you (e.g. `fastsar -x 'aws s3api list-buckets'`).
In this case fastsar will print the command output instead of the environment variable.


## How to install
- TODO: release to crates.io
- TODO: cross-platform build + binaries as releases

## Config
Create a json file with all your accounts and roles in the following format in
`$HOME/.aws/sts.json` (default file path):

```
[
    {
        "name": "Production",
        "id": "1234",
        "roles": [
            "abc",
            "def"
        ]
    },
    {
        "name": "Testing",
        "id": "14253",
        "roles": [ "abc" ]
    }
]
```

- `name`: Some name for your account
- `id`: The AWS account ID
- `roles`: List of AWS role names you want to assume

## Requirements
Lots of AWS accounts
