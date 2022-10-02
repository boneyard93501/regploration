# Exploring Registry

## Use Case

Parallelize a mean (average) computation for some data array. Basic algo is a simple map-reduce: split the data array into a number of sub-arrays equal to the number of parallelizable mean services, calculate the mean for each subarray in parallel  and then to run one final mean calculation over the subarrays.


## Mean Service

See [Wasm code](./services/mean-calc/src/../../../services/mean-calc/src/main.rs) and corresponding
[Aqua code](./aqua/mean_services.aqua).

We deploy the services two times to three different peers, e.g.

```bash
aqua remote deploy_service \
    --addr <TARGET RELAY MULTIADDR> \
    --config-path configs/deployment_cfg.json \
    --service mean-calc \
    --sk <YOUR-SECRET_KEY>
```

Which gives us the following service addresses in *data/service_addrs.json*:

```json
[
  {
    "peer_id": "12D3KooWCMr9mU894i8JXAFqpgoFtx6qnV1LFPSfVc3Y34N4h4LS",
    "service_id": "06b8a8bf-8952-479b-9160-2ee965e27892"
  },
  {
    "peer_id": "12D3KooWCMr9mU894i8JXAFqpgoFtx6qnV1LFPSfVc3Y34N4h4LS",
    "service_id": "202228be-c053-4a04-84be-59ec655f6103"
  },
  {
    "peer_id": "12D3KooWEFFCZnar1cUJQ3rMWjvPQg6yMV2aXWs2DkJNSRbduBWn",
    "service_id": "48ee1ff2-f017-4780-b293-ede068eabbcf"
  },
  {
    "peer_id": "12D3KooWEFFCZnar1cUJQ3rMWjvPQg6yMV2aXWs2DkJNSRbduBWn",
    "service_id": "0f4ae801-240e-4346-985d-85f095177185"
  },
  {
    "peer_id": "12D3KooWD7CvsYcpF9HE9CCV9aY3SJ317tkXVykjtZnht2EbzDPm",
    "service_id": "41f5c5f9-0dc9-40b8-b6b7-ae6b87ae9d0f"
  },
  {
    "peer_id": "12D3KooWD7CvsYcpF9HE9CCV9aY3SJ317tkXVykjtZnht2EbzDPm",
    "service_id": "18ee33c5-68ae-4896-8a04-dace728bbdc8"
  }
]
```

We can test our services in the REPL or with Aqua, e.g.:

```bash
aqua run \
    --addr /dns4/kras-09.fluence.dev/tcp/19001/wss/p2p/12D3KooWD7CvsYcpF9HE9CCV9aY3SJ317tkXVykjtZnht2EbzDPm \
    --input aqua/mean_services.aqua \
    --func 'mean_i64([1,2,3], dict)' \
    --data '{"dict":{"peer_id": "12D3KooWCMr9mU894i8JXAFqpgoFtx6qnV1LFPSfVc3Y34N4h4LS", "service_id": "06b8a8bf-8952-479b-9160-2ee965e27892"}}'

2
```

Now that we got our services created, deployed and tested, let's work on our main problem in a minimalist fashion: we have an array of data [2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0] of length 12 and we have six service instances. So we can split the array into two slices of six, calculate the mean for each slice join the results and calculate the mean over those two values. Obviously, this makes a lot more sense if you got a lot of data 

```python
-- mean reduce
n_data = array_length(payload)
n_services =  array_length(services)
n_slices = 


```

### Create The Resource

```python
-- register_services.aqua
import "@fluencelabs/registry/resources-api.aqua"
import ServiceAddress from "types.aqua"

func register_resource(resource_label:string) -> ?string:
    resource_id, error <- createResource(resource_label)
    <- resource_id
```

In order to create a resource, we need a label, which for our purposes is *bb-mean-calc-1*:

```bash
aqua run \
    --addr /dns4/kras-05.fluence.dev/tcp/19001/wss/p2p/12D3KooWCMr9mU894i8JXAFqpgoFtx6qnV1LFPSfVc3Y34N4h4LS \
    -i aqua/register_services.aqua \
    --import aqua/node_modules \
    -f 'register_resource("bb-mean-calc-1")'
```

Which gives us *resource_id*:

```bash
[
  "bb-mean-calc-112D3KooWGXTn9J78jz8ymbbu7pk87uj3CSisYZDYXgNsaqeiPdPg"
]
```

### Populate The Resource

Now that we have a resource instance, we can add our service records, from *data/service_addrs.json*, to the resource instance using the *register_service* function:

```python
-- register_services.aqua
func register_service(resource_id: string, service_addrs: []ServiceAddress) -> []bool:
    results: *bool
    for service_addr <- service_addrs:
        on service_addr.peer_id:
            results, err <- registerNodeProvider(service_addr.peer_id,resource_id, "", ?[service_addr.service_id])
    <- results
```

Running the register function with a little extra timeout:

```bash
aqua run \
    --addr /dns4/kras-05.fluence.dev/tcp/19001/wss/p2p/12D3KooWCMr9mU894i8JXAFqpgoFtx6qnV1LFPSfVc3Y34N4h4LS \
    -i aqua/register_services.aqua \
    --import aqua/node_modules \
    -f 'register_service("bb-mean-calc-112D3KooWGXTn9J78jz8ymbbu7pk87uj3CSisYZDYXgNsaqeiPdPg", service_addrs)' \
    --data-path data/service_addrs.json \
    --timeout 60000
```

Gives us confirmation that all six service addresses have been successfully added:

```bash
[
  true,
  true,
  true,
  true,
  true,
  true
]
```

Please note that we could have specified more insightful return data for *register_service* saving us the following queries. 

More to come


### Experiments


#### getResource(resource_id: ResourceId)

Aqua Wrapper:

```python
-- aqua/consume_registry.aqua

func get_resources(resource_id: string) -> ?Key, *Error:
    res, err <- getResource(resource_id)
    <- res, err
```

Aqua Call:

```bash
aqua run \
    --addr /dns4/kras-05.fluence.dev/tcp/19001/wss/p2p/12D3KooWCMr9mU894i8JXAFqpgoFtx6qnV1LFPSfVc3Y34N4h4LS \
    -i aqua/consume_registry.aqua \
    --import aqua/node_modules \
    -f 'get_resources("bb-mean-calc-112D3KooWGXTn9J78jz8ymbbu7pk87uj3CSisYZDYXgNsaqeiPdPg")'
```

Response:

```bash
aqua run \
    --addr /dns4/kras-05.fluence.dev/tcp/19001/wss/p2p/12D3KooWCMr9mU894i8JXAFqpgoFtx6qnV1LFPSfVc3Y34N4h4LS \
    -i aqua/consume_registry.aqua \
    --import aqua/node_modules \
    -f 'get_resources("bb-mean-calc-112D3KooWGXTn9J78jz8ymbbu7pk87uj3CSisYZDYXgNsaqeiPdPg")'
[
[
  {
    "challenge": [],
    "challenge_type": "",
    "id": "bb-mean-calc-112D3KooWGXTn9J78jz8ymbbu7pk87uj3CSisYZDYXgNsaqeiPdPg",
    "label": "bb-mean-calc-1",
    "owner_peer_id": "12D3KooWGXTn9J78jz8ymbbu7pk87uj3CSisYZDYXgNsaqeiPdPg",
    "signature": [
      155,
      254,
      186,
      229,
      214,
      140,
      162,
      29,
      98,
      236,
      193,
      44,
      184,
      213,
      49,
      149,
      253,
      209,
      119,
      110,
      215,
      29,
      70,
      201,
      20,
      35,
      223,
      163,
      166,
      42,
      51,
      154,
      91,
      61,
      84,
      126,
      229,
      16,
      46,
      202,
      189,
      175,
      208,
      2,
      255,
      196,
      183,
      198,
      172,
      35,
      9,
      126,
      11,
      70,
      176,
      220,
      28,
      143,
      26,
      111,
      96,
      51,
      240,
      7
    ],
    "timestamp_created": 1661063586
  }
],
[
  [
    [
      "Requested key bb-mean-calc-112D3KooWGXTn9J78jz8ymbbu7pk87uj3CSisYZDYXgNsaqeiPdPg does not exist on 12D3KooWB4Rigrxc9dB4rsRMG1y7i5LLpDsjx7M4tRXaybE4pKak"
    ]
  ]
]
]
```

Changing to:

```python
func get_resources_with_peer(resource_id: string, peer_id:string) -> ?Key, *Error:
    on peer_id:
        res, err <- getResource(resource_id)
        <- res, err
```

Running with the relay specified in *addr*:

```bash
aqua run \
    --addr /dns4/kras-05.fluence.dev/tcp/19001/wss/p2p/12D3KooWCMr9mU894i8JXAFqpgoFtx6qnV1LFPSfVc3Y34N4h4LS \
    -i aqua/consume_registry.aqua \
    --import aqua/node_modules \
    -f 'get_resources_with_peer("bb-mean-calc-112D3KooWGXTn9J78jz8ymbbu7pk87uj3CSisYZDYXgNsaqeiPdPg", "12D3KooWCMr9mU894i8JXAFqpgoFtx6qnV1LFPSfVc3Y34N4h4LS")' --timeout 30000
Function 'get_resources_with_peer' timed out after 30000 milliseconds. Increase the timeout with '--timeout' option or check if your code can hang while executing (particle id: be1b20c9-a5a0-404c-9d55-7c63f65d751e).

Try 'aqua --help' for usage instructions
```

Running it again:

```bash
aqua run \
    --addr /dns4/kras-05.fluence.dev/tcp/19001/wss/p2p/12D3KooWCMr9mU894i8JXAFqpgoFtx6qnV1LFPSfVc3Y34N4h4LS \
    -i aqua/consume_registry.aqua \
    --import aqua/node_modules \
    -f 'get_resources_with_peer("bb-mean-calc-112D3KooWGXTn9J78jz8ymbbu7pk87uj3CSisYZDYXgNsaqeiPdPg", "12D3KooWCMr9mU894i8JXAFqpgoFtx6qnV1LFPSfVc3Y34N4h4LS")' --timeout 30000
[
[
  {
    "challenge": [],
    "challenge_type": "",
    "id": "bb-mean-calc-112D3KooWGXTn9J78jz8ymbbu7pk87uj3CSisYZDYXgNsaqeiPdPg",
    "label": "bb-mean-calc-1",
    "owner_peer_id": "12D3KooWGXTn9J78jz8ymbbu7pk87uj3CSisYZDYXgNsaqeiPdPg",
    "signature": [
      155,
      254,
      186,
      229,
      214,
      140,
      162,
      29,
      98,
      236,
      193,
      44,
      184,
      213,
      49,
      149,
      253,
      209,
      119,
      110,
      215,
      29,
      70,
      201,
      20,
      35,
      223,
      163,
      166,
      42,
      51,
      154,
      91,
      61,
      84,
      126,
      229,
      16,
      46,
      202,
      189,
      175,
      208,
      2,
      255,
      196,
      183,
      198,
      172,
      35,
      9,
      126,
      11,
      70,
      176,
      220,
      28,
      143,
      26,
      111,
      96,
      51,
      240,
      7
    ],
    "timestamp_created": 1661063586
  }
],
[
  [
    []
  ]
]
]
```

#### resolveProvider

Aqua:

```python
func resolve_provider(resource_id:string, n_services: i16) -> []Record, *Error:
    on HOST_PEER_ID:
        res, err <- resolveProviders(resource_id, n_services)
        <- res, err
```

Run:

```bash
aqua run \
    --addr /dns4/kras-05.fluence.dev/tcp/19001/wss/p2p/12D3KooWCMr9mU894i8JXAFqpgoFtx6qnV1LFPSfVc3Y34N4h4LS \
    -i aqua/consume_registry.aqua \
    --import aqua/node_modules \
    -f 'resolve_provider("bb-mean-calc-112D3KooWGXTn9J78jz8ymbbu7pk87uj3CSisYZDYXgNsaqeiPdPg", 6)' --timeout 30000
Function 'resolve_provider' timed out after 30000 milliseconds. Increase the timeout with '--timeout' option or check if your code can hang while executing (particle id: 19380c02-c452-4767-98f4-67994c0e897e).

Try 'aqua --help' for usage instructions
```

Note: got same result when changing 6 to 2.


Revised Aqua:

```python
func resolve_provider_with_peer(resource_id:string, n_services: i16, peer_id:string) -> []Record, *Error:
    on peer_id:
        res, err <- resolveProviders(resource_id, n_services)
        <- res, err
```

Run with resource peer:

```bash
aqua run \
    --addr /dns4/kras-05.fluence.dev/tcp/19001/wss/p2p/12D3KooWCMr9mU894i8JXAFqpgoFtx6qnV1LFPSfVc3Y34N4h4LS \
    -i aqua/consume_registry.aqua \
    --import aqua/node_modules \
    -f 'resolve_provider_with_peer("bb-mean-calc-112D3KooWGXTn9J78jz8ymbbu7pk87uj3CSisYZDYXgNsaqeiPdPg", 2, "12D3KooWGXTn9J78jz8ymbbu7pk87uj3CSisYZDYXgNsaqeiPdPg")' --timeout 60000
Function 'resolve_provider_with_peer' timed out after 60000 milliseconds. Increase the timeout with '--timeout' option or check if your code can hang while executing (particle id: 95eb2503-4da2-4d69-bb3d-89c5d661097a).

Try 'aqua --help' for usage instructions
```

Run with another peer:

```bash
aqua run \
    --addr /dns4/kras-05.fluence.dev/tcp/19001/wss/p2p/12D3KooWCMr9mU894i8JXAFqpgoFtx6qnV1LFPSfVc3Y34N4h4LS \
    -i aqua/consume_registry.aqua \
    --import aqua/node_modules \
    -f 'resolve_provider_with_peer("bb-mean-calc-112D3KooWGXTn9J78jz8ymbbu7pk87uj3CSisYZDYXgNsaqeiPdPg", 2, "12D3KooWCMr9mU894i8JXAFqpgoFtx6qnV1LFPSfVc3Y34N4h4LS")' --timeout 60000
Function 'resolve_provider_with_peer' timed out after 60000 milliseconds. Increase the timeout with '--timeout' option or check if your code can hang while executing (particle id: a81c8201-3ded-4311-8b03-6d74340b2a36).

Try 'aqua --help' for usage instructions
```

