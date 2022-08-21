# Exploring Registry

## Use Case

Parallelize a mean (average) computation for some data array. Basic algo is a simple map-reduce: split the data array into a number of sub-arrays equal to the number of parallelizable mean services, calculate the mean for each subarray in parallel  and then to run one final mean calculation over the subarrays.

On the services side, we have the follwing instances:

```
```

and

* want to add them as a registry source
* we only want to use one service for each unique peer and use the second instance as a fallback, for example


## Mean Service

wasm

aqua

We deploy the services two times to three different peers with the following service addresses in *data/service_addrs.json*:

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

Now that we have a resource instance, we can add our service records, from *data/service_addrs.json*, to the resource instance using the *register_service* fundtion:

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

