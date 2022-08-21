# IntoJSON

A tool to convert from **TOML** to **JSON**

<p align="center"> 
    <img src="./json.png" alt="IntoJSON" title="IntoJSON"/>
<p>

## Example

```
$ intojson Cargo.toml
```

```json
{
  "dependencies": {
    "regex": "1.6.0",
    "serde_json": "1.0.83"
  },
  "package": {
    "edition": "2021",
    "name": "intojson",
    "version": "0.1.0"
  }
}
```

For larger more complex files like [example.toml](./example.toml)

```bash
$ intojson example.toml
```

<details>
<summary> example.json</summary>
### Input

```toml
# This is a TOML document

# title = "TOML Example"

[owner]
name = "Tom Preston-Werner"
dob = 1979-05-27T07:32:00-08:00
index = 12

[database]
enabled = true
ports = [ 8000, 8001, 8002 ]
data = [ ["delta", "phi"], [3.14] ]
temp_targets = { cpu = 79.5, case = 72.0 }

[servers]

[servers.alpha]
ip = "10.0.0.1"
role = "frontend"

[servers.beta]
ip = "10.0.0.2"
role = "backend"

```

### Output

```json
{
  "database": {
    "data": [["delta", "phi"], [3.14]],
    "enabled": true,
    "ports": [8000, 8001, 8002],
    "temp_targets": {
      "case": 72.0,
      "cpu": 79.5
    }
  },
  "owner": {
    "dob": "1979-05-27T07:32:00-08:00",
    "index": 12,
    "name": "Tom Preston-Werner"
  },
  "servers": {},
  "servers.alpha": {
    "ip": "10.0.0.1",
    "role": "frontend"
  },
  "servers.beta": {
    "ip": "10.0.0.2",
    "role": "backend"
  }
}
```

<details>
