## Distributed Operations

### Node Management
```bash
# Target specific nodes
--node=node1.example.com
--nodes=node1,node2,node3
--node-group=web-servers

# Broadcast operations
--broadcast              # Run on all nodes
--parallel-nodes=5       # Concurrent node operations
```

### Cluster Operations
```bash
# Cluster awareness
--cluster=production
--cluster-config=/etc/mytool/cluster.yaml

# Leader election
--require-leader        # Only run on leader node
--leader-election-timeout=30s

# Quorum requirements
--quorum=3              # Minimum nodes required
--quorum-percentage=51  # Percentage of nodes required
```

### Distributed Coordination
```bash
# Locking
--distributed-lock=operation-name
--lock-timeout=60s
--lock-wait            # Wait for lock availability

# Consensus
--consensus-timeout=30s
--consistency-level=strong|eventual|quorum
```

### Health and Discovery
```bash
# Service discovery
--discovery-service=consul|etcd|zookeeper
--discovery-endpoint=http://consul:8500
--service-name=mytool-worker

# Health checks
mytool cluster health
mytool cluster health --format=json
mytool node status node1
```

### Distributed Tracing
```bash
# Trace context propagation
--trace-id=abc123def456
--parent-span-id=span789
--baggage="user=alice,tenant=acme"

# Correlation
--correlation-id=request-123
--causation-id=parent-operation-456
```

