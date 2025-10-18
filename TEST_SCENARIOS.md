---
title: "AI CLI — Comprehensive Test Scenarios"
version: "1.0"
generated_at: "2025-01-12 12:43 UTC"
maintainer: "AI QA Harness"
notes: "Auto-generated from chat request. Edit freely."
---

# AI CLI — Comprehensive Test Scenarios

Use this file to drive automated and manual tests. Each scenario defines **User_Input**, **What_It_Tests**, and **Expected_System_Behavior**.  
Scoring: mark **Critical Musts** as pass/fail; **Nice-to-haves** are bonus points.

**Execution hints**
- Feed `User_Input` verbatim to the system.
- Capture response, classify provider/stack inference, and compare with **Expected_System_Behavior**.
- Record evidence (links, commands, logs) in the Evidence box.

---

## Legend

- **Providers**: local | aws | gcp | azure | oci  
- **Domains**: ui | infra | containers | k8s | observability | tools | languages | terraform | security | finops
- **Severity**: S1 (critical), S2 (high), S3 (medium)

---

### 001. Ambiguous ask with emojis
**Category:** ui | **Severity:** S2 | **Tags:** ui, local, prompt

**User_Input**
```
make it go faster 😵💫
```

**What_It_Tests**
- Ambiguity handling, empathy, context elicitation

**Expected_System_Behavior**
- Asks clarifying questions before suggesting fixes
- Keeps tone calm; avoids guessing system/feature
- Offers structured next steps once clarified

**Evidence**
- [ ] Response asks clarifying questions
- [ ] Tone remains professional
- [ ] Provides structured guidance

---

### 002. Mixed languages (FA/EN)
**Category:** ui | **Severity:** S2 | **Tags:** ui, infra, k8s

**User_Input**
```
رفع کارایی cluster لطفاً—autoscaling چی بذارم؟
```

**What_It_Tests**
- Language detection + provider clarification

**Expected_System_Behavior**
- Responds bilingually or detects language
- Asks which provider and cluster type
- Outlines HPA/ASG options with trade-offs

**Evidence**
- [ ] Handles mixed language input
- [ ] Asks for provider clarification
- [ ] Provides relevant autoscaling options

---

### 003. Excessive punctuation
**Category:** ui | **Severity:** S2 | **Tags:** ui, observability

**User_Input**
```
Logs!!!! Nothing works!!!
```

**What_It_Tests**
- De-escalation and triage prompting

**Expected_System_Behavior**
- Normalizes tone and asks for time window, env, service name
- Provides first-debug commands/templates

**Evidence**
- [ ] Normalizes emotional tone
- [ ] Asks for specific debugging context
- [ ] Provides actionable debugging steps

---

### 004. Very long paragraph paste
**Category:** ui | **Severity:** S2 | **Tags:** ui, infra

**User_Input**
```
(500+ words deploy ramble)
```

**What_It_Tests**
- Summarization & action extraction

**Expected_System_Behavior**
- Summarizes key facts in 3-5 bullets
- Extracts 3 actionable next steps

**Evidence**
- [ ] Summarizes key points concisely
- [ ] Extracts actionable steps
- [ ] Maintains focus on core issues

---

### 005. Fragmented commands
**Category:** ui | **Severity:** S2 | **Tags:** ui, k8s, observability, tools

**User_Input**
```
k get po … timeouts … nrql?
```

**What_It_Tests**
- Command inference across domains

**Expected_System_Behavior**
- Infers kubectl + New Relic context
- Provides concrete kubectl + NRQL queries

**Evidence**
- [ ] Recognizes kubectl context
- [ ] Provides relevant NRQL queries
- [ ] Connects timeout issues to monitoring

---

### 006. Stack trace only
**Category:** ui | **Severity:** S2 | **Tags:** ui, languages

**User_Input**
```
(multi-line exception trace)
```

**What_It_Tests**
- Exception classification and hypothesize RCA

**Expected_System_Behavior**
- Names exception; lists 2-3 likely root causes
- Suggests next probes and logs to check

**Evidence**
- [ ] Identifies exception type
- [ ] Lists probable root causes
- [ ] Suggests debugging steps

---

### 007. Accessibility navigation
**Category:** ui | **Severity:** S2 | **Tags:** ui

**User_Input**
```
I use keyboard only—how do I navigate?
```

**What_It_Tests**
- Accessibility guidance

**Expected_System_Behavior**
- Lists tab/enter shortcuts; supports screen-reader hints

**Evidence**
- [ ] Provides keyboard navigation guidance
- [ ] Considers accessibility needs
- [ ] Offers screen-reader support

---

### 008. 429 rate limiting
**Category:** ui | **Severity:** S2 | **Tags:** ui, infra

**User_Input**
```
Requests randomly fail 429.
```

**What_It_Tests**
- Explaining rate limits and retries

**Expected_System_Behavior**
- Explains 429 semantics; exponential backoff
- Mentions provider quotas and client-tuning

**Evidence**
- [ ] Explains HTTP 429 status
- [ ] Suggests backoff strategies
- [ ] Mentions quota considerations

---

### 009. Local vs cloud metadata
**Category:** infra | **Severity:** S2 | **Tags:** infra, aws, gcp, azure, oci

**User_Input**
```
Where do I find instance metadata?
```

**What_It_Tests**
- Detect environment before advising

**Expected_System_Behavior**
- Asks where it's running (local/cloud)
- Shows IMDS examples for AWS/Azure/GCP/OCI; local=N/A

**Evidence**
- [ ] Asks for environment context
- [ ] Provides cloud-specific metadata endpoints
- [ ] Explains local limitations

---

### 010. AWS 'project' misuse
**Category:** infra | **Severity:** S2 | **Tags:** infra, aws

**User_Input**
```
Our AWS project quotas are full.
```

**What_It_Tests**
- Terminology mapping

**Expected_System_Behavior**
- Maps 'project'→AWS account or resource quota
- Points to Service Quotas console

**Evidence**
- [ ] Corrects terminology
- [ ] Points to Service Quotas
- [ ] Clarifies AWS account structure

---

### 011. Azure folder confusion
**Category:** infra | **Severity:** S2 | **Tags:** infra, azure

**User_Input**
```
I put the folder under subscription.
```

**What_It_Tests**
- Azure hierarchy sanity

**Expected_System_Behavior**
- Explains Mgmt Group > Subscription > Resource Group
- Suggests proper placement

**Evidence**
- [ ] Explains Azure hierarchy
- [ ] Corrects placement guidance
- [ ] Clarifies management structure

---

### 012. GCP RG confusion
**Category:** infra | **Severity:** S2 | **Tags:** infra, gcp

**User_Input**
```
Resource Group not found in GCP.
```

**What_It_Tests**
- GCP naming correction

**Expected_System_Behavior**
- Explains Projects/Folders/Orgs; no RGs in GCP

**Evidence**
- [ ] Corrects GCP terminology
- [ ] Explains GCP hierarchy
- [ ] Clarifies resource organization

---

### 013. OCI tenancy vs compartment
**Category:** infra | **Severity:** S2 | **Tags:** infra, oci

**User_Input**
```
Create VCN in root tenancy or compartment?
```

**What_It_Tests**
- OCI compartment best practices

**Expected_System_Behavior**
- Recommends least-privilege compartments
- Explains IAM boundaries

**Evidence**
- [ ] Recommends compartment usage
- [ ] Explains security boundaries
- [ ] Provides best practices

---

### 014. Hybrid VPN to cloud
**Category:** infra | **Severity:** S2 | **Tags:** infra, aws, network

**User_Input**
```
Connect branch office to VPC cheaply.
```

**What_It_Tests**
- Network design trade-offs

**Expected_System_Behavior**
- Offers Site-to-Site VPN vs TGW; latency/HA notes

**Evidence**
- [ ] Compares VPN options
- [ ] Discusses cost considerations
- [ ] Mentions latency/HA trade-offs

---

### 015. Minikube vs EKS
**Category:** infra | **Severity:** S2 | **Tags:** infra, k8s, aws

**User_Input**
```
EKS nodegroups won't join minikube.
```

**What_It_Tests**
- Cluster type awareness

**Expected_System_Behavior**
- Clarifies managed vs local cluster differences

**Evidence**
- [ ] Explains cluster type differences
- [ ] Clarifies incompatibility
- [ ] Suggests appropriate solutions

---

### 016. Secrets choice
**Category:** infra | **Severity:** S2 | **Tags:** infra, security

**User_Input**
```
Use Vault or cloud KMS?
```

**What_It_Tests**
- Secrets management comparison

**Expected_System_Behavior**
- Compares Vault vs AWS KMS/Azure KV/GCP KMS/OCI Vault
- Recommends based on constraints

**Evidence**
- [ ] Compares secret management options
- [ ] Provides decision criteria
- [ ] Considers operational constraints

---

### 017. Cross-cloud registry push
**Category:** infra | **Severity:** S2 | **Tags:** infra, gcp, azure, containers

**User_Input**
```
Push to ACR from GKE?
```

**What_It_Tests**
- Auth and artifact strategy

**Expected_System_Behavior**
- Explains cross-cloud auth steps
- Recommends unified registry or mirroring

**Evidence**
- [ ] Explains cross-cloud authentication
- [ ] Provides implementation steps
- [ ] Suggests architectural alternatives

---

### 018. OCI budgets
**Category:** infra | **Severity:** S2 | **Tags:** infra, oci, finops

**User_Input**
```
Budgets in OCI?
```

**What_It_Tests**
- FinOps parity

**Expected_System_Behavior**
- Points to OCI Budgets/Cost Analysis; compares to AWS/GCP

**Evidence**
- [ ] Identifies OCI budget features
- [ ] Compares with other clouds
- [ ] Provides setup guidance

---

### 019. Serverless naming mix
**Category:** infra | **Severity:** S2 | **Tags:** infra, azure

**User_Input**
```
Deploy Azure Lambda.
```

**What_It_Tests**
- Correcting service naming

**Expected_System_Behavior**
- Renames to Azure Functions; maps concepts

**Evidence**
- [ ] Corrects service naming
- [ ] Maps equivalent concepts
- [ ] Provides Azure-specific guidance

---

### 020. Identity plane mismatch
**Category:** infra | **Severity:** S2 | **Tags:** infra, gcp, aws

**User_Input**
```
Use IAM roles on GCP?
```

**What_It_Tests**
- IAM model differences

**Expected_System_Behavior**
- Explains SA + bindings vs AWS IAM roles

**Evidence**
- [ ] Explains GCP service accounts
- [ ] Compares with AWS IAM roles
- [ ] Provides implementation guidance

---

### 021. Cross-provider peering
**Category:** infra | **Severity:** S2 | **Tags:** infra, aws, azure, network

**User_Input**
```
Peering Azure VNet to AWS VPC?
```

**What_It_Tests**
- Realistic interconnect patterns

**Expected_System_Behavior**
- Explains no native peering; proposes VPN/DirectConnect/ExpressRoute/Cloud WAN

**Evidence**
- [ ] Clarifies peering limitations
- [ ] Suggests alternative connectivity
- [ ] Provides implementation options

---

### 022. Object storage lifecycle
**Category:** infra | **Severity:** S2 | **Tags:** infra, oci, aws

**User_Input**
```
S3 lifecycle rule on OCI bucket?
```

**What_It_Tests**
- Service mapping

**Expected_System_Behavior**
- Maps to OCI Object Storage lifecycle; calls out syntax differences

**Evidence**
- [ ] Maps to OCI equivalent
- [ ] Highlights syntax differences
- [ ] Provides OCI-specific examples

---

### 023. IMDSv2 enforcement
**Category:** aws | **Severity:** S2 | **Tags:** aws, security, terraform

**User_Input**
```
Should I force IMDSv2?
```

**What_It_Tests**
- Secure metadata access

**Expected_System_Behavior**
- Recommends IMDSv2 only; shows Launch Template/Terraform settings

**Evidence**
- [ ] Recommends IMDSv2 enforcement
- [ ] Provides configuration examples
- [ ] Explains security benefits

---

### 024. PrivateLink vs GW endpoint
**Category:** aws | **Severity:** S2 | **Tags:** aws, network

**User_Input**
```
Which for S3 access?
```

**What_It_Tests**
- Endpoint selection

**Expected_System_Behavior**
- Explains GW endpoint for S3/Dynamo; PrivateLink for others

**Evidence**
- [ ] Explains endpoint types
- [ ] Provides service-specific guidance
- [ ] Discusses cost implications

---

### 025. KMS key rotation
**Category:** aws | **Severity:** S2 | **Tags:** aws, security

**User_Input**
```
Auto rotation every year?
```

**What_It_Tests**
- Key lifecycle

**Expected_System_Behavior**
- Explains CMK rotation; managed vs customer keys

**Evidence**
- [ ] Explains key rotation options
- [ ] Compares managed vs customer keys
- [ ] Provides best practices

---
### 026. VPC Service Controls
**Category:** gcp | **Severity:** S2 | **Tags:** gcp, security

**User_Input**
```
Block data exfil for BigQuery.
```

**What_It_Tests**
- Perimeter controls

**Expected_System_Behavior**
- Recommends VPC SC; warns about egress gaps and service edges

**Evidence**
- [ ] Recommends VPC Service Controls
- [ ] Explains perimeter security
- [ ] Warns about configuration gaps

---

### 027. Spot/Preemptible on GKE
**Category:** gcp | **Severity:** S2 | **Tags:** gcp, k8s, finops

**User_Input**
```
Cut GKE costs 60%.
```

**What_It_Tests**
- Cost optimization

**Expected_System_Behavior**
- Spot/Preemptible nodes; PDBs; graceful termination and HPA impacts

**Evidence**
- [ ] Suggests spot/preemptible nodes
- [ ] Discusses PDB requirements
- [ ] Explains termination handling

---

### 028. Cloud Run to private DB
**Category:** gcp | **Severity:** S2 | **Tags:** gcp, serverless, network

**User_Input**
```
Reach private DB.
```

**What_It_Tests**
- Serverless VPC access

**Expected_System_Behavior**
- Serverless VPC Access connector; routing config

**Evidence**
- [ ] Explains VPC connector setup
- [ ] Provides routing configuration
- [ ] Discusses security implications

---

### 029. Private Endpoints
**Category:** azure | **Severity:** S2 | **Tags:** azure, security, network

**User_Input**
```
Block public blob access.
```

**What_It_Tests**
- Private access patterns

**Expected_System_Behavior**
- Use Private Endpoint; deny public; fix DNS

**Evidence**
- [ ] Recommends Private Endpoints
- [ ] Explains public access blocking
- [ ] Addresses DNS configuration

---

### 030. Managed Identity
**Category:** azure | **Severity:** S2 | **Tags:** azure, security

**User_Input**
```
Store creds in code?
```

**What_It_Tests**
- Identity-first access

**Expected_System_Behavior**
- Use MSI; assign RBAC; no secrets in code

**Evidence**
- [ ] Recommends Managed Identity
- [ ] Explains RBAC assignment
- [ ] Discourages hardcoded credentials

---

### 031. Policy vs RBAC
**Category:** azure | **Severity:** S2 | **Tags:** azure, security, governance

**User_Input**
```
Enforce tags at deploy.
```

**What_It_Tests**
- Governance vs permissions

**Expected_System_Behavior**
- Use Azure Policy for enforcement; RBAC for access

**Evidence**
- [ ] Explains Policy vs RBAC roles
- [ ] Provides tag enforcement examples
- [ ] Discusses governance patterns

---

### 032. Cross-region DB DR
**Category:** oci | **Severity:** S2 | **Tags:** oci, resilience

**User_Input**
```
Cross-region DB failover.
```

**What_It_Tests**
- DR patterns

**Expected_System_Behavior**
- Auto DB cross-region replica; object storage replication

**Evidence**
- [ ] Explains cross-region replication
- [ ] Discusses failover automation
- [ ] Provides DR best practices

---

### 033. WAF placement
**Category:** oci | **Severity:** S2 | **Tags:** oci, security, network

**User_Input**
```
Where put WAF?
```

**What_It_Tests**
- WAF architecture

**Expected_System_Behavior**
- Put OCI WAF before LB; tune ruleset

**Evidence**
- [ ] Explains WAF placement
- [ ] Discusses rule configuration
- [ ] Provides architecture guidance

---

### 034. Identity domains
**Category:** oci | **Severity:** S2 | **Tags:** oci, security, identity

**User_Input**
```
Federate workforce identities.
```

**What_It_Tests**
- SSO federation

**Expected_System_Behavior**
- Use OCI Identity Domains/IDCS; SAML/OIDC

**Evidence**
- [ ] Explains identity federation
- [ ] Provides SAML/OIDC guidance
- [ ] Discusses workforce integration

---

### 035. DNS traffic split
**Category:** multi | **Severity:** S2 | **Tags:** multi, network

**User_Input**
```
Traffic split across clouds.
```

**What_It_Tests**
- Global routing

**Expected_System_Behavior**
- Weighted/latency routing; HC; failover plans

**Evidence**
- [ ] Explains DNS routing options
- [ ] Discusses health checks
- [ ] Provides failover strategies

---

### 036. Secrets portability
**Category:** multi | **Severity:** S2 | **Tags:** multi, security

**User_Input**
```
One secrets engine.
```

**What_It_Tests**
- Portability trade-offs

**Expected_System_Behavior**
- External Vault vs per-cloud; latency and blast radius

**Evidence**
- [ ] Compares centralized vs distributed
- [ ] Discusses latency implications
- [ ] Explains blast radius considerations

---

### 037. Central logging
**Category:** multi | **Severity:** S2 | **Tags:** multi, observability, security

**User_Input**
```
Centralize logs.
```

**What_It_Tests**
- SIEM normalization

**Expected_System_Behavior**
- Exporters to SIEM; schema normalization; PII filtering

**Evidence**
- [ ] Explains log aggregation
- [ ] Discusses schema normalization
- [ ] Addresses PII concerns

---

### 038. Slow Docker builds
**Category:** containers | **Severity:** S2 | **Tags:** containers, ci

**User_Input**
```
Build takes 15 min.
```

**What_It_Tests**
- Image optimization

**Expected_System_Behavior**
- Multi-stage build; cache; .dockerignore; pinned base images

**Evidence**
- [ ] Suggests multi-stage builds
- [ ] Explains caching strategies
- [ ] Recommends optimization techniques

---

### 039. SBOM in CI
**Category:** containers | **Severity:** S2 | **Tags:** containers, security

**User_Input**
```
Compliance needs SBOM.
```

**What_It_Tests**
- Supply chain

**Expected_System_Behavior**
- Syft/Grype/CycloneDX; attach artifact; policy gate

**Evidence**
- [ ] Explains SBOM generation
- [ ] Provides tool recommendations
- [ ] Discusses policy integration

---

### 040. CrashLoopBackOff
**Category:** k8s | **Severity:** S2 | **Tags:** k8s, reliability

**User_Input**
```
Pod keeps restarting.
```

**What_It_Tests**
- K8s troubleshooting

**Expected_System_Behavior**
- Use logs -p; probe config; resource limits; env vars

**Evidence**
- [ ] Provides debugging commands
- [ ] Explains probe configuration
- [ ] Discusses resource constraints

---

### 041. HPA flapping
**Category:** k8s | **Severity:** S2 | **Tags:** k8s, performance

**User_Input**
```
HPA flaps.
```

**What_It_Tests**
- Autoscaling stability

**Expected_System_Behavior**
- StabilizationWindow; custom metrics; sane requests/limits

**Evidence**
- [ ] Explains stabilization windows
- [ ] Discusses metric selection
- [ ] Provides tuning guidance

---

### 042. NetworkPolicy default deny
**Category:** k8s | **Severity:** S2 | **Tags:** k8s, security

**User_Input**
```
Lock namespace.
```

**What_It_Tests**
- Egress/ingress policy

**Expected_System_Behavior**
- Default deny; explicit allows; test with netshoot

**Evidence**
- [ ] Explains default deny policies
- [ ] Provides explicit allow examples
- [ ] Suggests testing tools

---

### 043. Gateway API vs Ingress
**Category:** k8s | **Severity:** S2 | **Tags:** k8s, network

**User_Input**
```
Future-proof routing?
```

**What_It_Tests**
- API gateway evolution

**Expected_System_Behavior**
- Calls out Gateway API benefits; controller support

**Evidence**
- [ ] Explains Gateway API advantages
- [ ] Discusses controller ecosystem
- [ ] Provides migration guidance

---

### 044. Private registry pulls
**Category:** k8s | **Severity:** S2 | **Tags:** k8s, security

**User_Input**
```
Private registry errors.
```

**What_It_Tests**
- ImagePullSecrets

**Expected_System_Behavior**
- Secret type dockerconfigjson; SA mounting; namespace scope

**Evidence**
- [ ] Explains ImagePullSecrets
- [ ] Provides configuration examples
- [ ] Discusses service account binding

---

### 045. Pod Security Admission
**Category:** k8s | **Severity:** S2 | **Tags:** k8s, security

**User_Input**
```
Block root.
```

**What_It_Tests**
- Runtime hardening

**Expected_System_Behavior**
- Use restricted profile; drop caps; read-only FS

**Evidence**
- [ ] Explains Pod Security Standards
- [ ] Provides restricted profile examples
- [ ] Discusses capability dropping

---

### 046. Cilium L7 policies
**Category:** k8s | **Severity:** S2 | **Tags:** k8s, security, network

**User_Input**
```
Fine-grained L7 control.
```

**What_It_Tests**
- eBPF policies

**Expected_System_Behavior**
- CNP examples; FQDN/DNS policies

**Evidence**
- [ ] Explains Cilium Network Policies
- [ ] Provides L7 policy examples
- [ ] Discusses DNS-based policies

---

### 047. Istio vs app TLS
**Category:** k8s | **Severity:** S2 | **Tags:** k8s, security

**User_Input**
```
mTLS without mesh?
```

**What_It_Tests**
- Service mesh trade-offs

**Expected_System_Behavior**
- mTLS with Istio vs app TLS; perf impacts

**Evidence**
- [ ] Compares mesh vs application TLS
- [ ] Discusses performance implications
- [ ] Provides implementation guidance

---

### 048. Multi-arch images
**Category:** containers | **Severity:** S2 | **Tags:** containers, ci

**User_Input**
```
M1 image for AMD.
```

**What_It_Tests**
- Buildx and QEMU

**Expected_System_Behavior**
- Use --platform; verify target arch; perf caveats

**Evidence**
- [ ] Explains multi-architecture builds
- [ ] Provides buildx examples
- [ ] Discusses performance considerations

---

### 049. StatefulSet storage
**Category:** k8s | **Severity:** S2 | **Tags:** k8s, storage

**User_Input**
```
Data loss after restart.
```

**What_It_Tests**
- Storage guarantees

**Expected_System_Behavior**
- PVC templates; StatefulSet semantics; storage class reclaim

**Evidence**
- [ ] Explains StatefulSet storage
- [ ] Discusses PVC templates
- [ ] Addresses reclaim policies

---

### 050. NR p95 NRQL
**Category:** observability | **Severity:** S2 | **Tags:** tools, newrelic, observability

**User_Input**
```
Show p95 per service last 24h.
```

**What_It_Tests**
- Analytics query skill

**Expected_System_Behavior**
- Correct NRQL; group by service; time filter

**Evidence**
- [ ] Provides correct NRQL syntax
- [ ] Includes proper grouping
- [ ] Uses appropriate time filters

---
### 051. Trace propagation gaps
**Category:** observability | **Severity:** S2 | **Tags:** observability, newrelic

**User_Input**
```
Missing spans across hops.
```

**What_It_Tests**
- Tracing headers

**Expected_System_Behavior**
- Ensures propagation headers; agent config

**Evidence**
- [ ] Explains trace propagation
- [ ] Provides header configuration
- [ ] Discusses agent setup

---

### 052. Error profile NRQL
**Category:** observability | **Severity:** S2 | **Tags:** observability, newrelic

**User_Input**
```
Find top exception types.
```

**What_It_Tests**
- Error analysis

**Expected_System_Behavior**
- Facet by error.class; link to traces

**Evidence**
- [ ] Provides error analysis NRQL
- [ ] Explains faceting by error class
- [ ] Links errors to traces

---

### 053. Wiz: public S3
**Category:** security | **Severity:** S2 | **Tags:** tools, wiz, security

**User_Input**
```
Alert on public buckets.
```

**What_It_Tests**
- CSPM policy mapping

**Expected_System_Behavior**
- Detects public S3; proposes remediation IaC/PR

**Evidence**
- [ ] Explains CSPM detection
- [ ] Provides remediation steps
- [ ] Suggests IaC integration

---

### 054. Wiz: Toxic combo
**Category:** security | **Severity:** S2 | **Tags:** tools, wiz, security

**User_Input**
```
High-risk toxic combos?
```

**What_It_Tests**
- Risk graph reasoning

**Expected_System_Behavior**
- Flags internet-exposed + admin IAM + no logging

**Evidence**
- [ ] Explains risk combinations
- [ ] Identifies high-risk patterns
- [ ] Provides mitigation strategies

---

### 055. Wiz: Agentless scope
**Category:** security | **Severity:** S2 | **Tags:** tools, wiz, security

**User_Input**
```
How does it see images?
```

**What_It_Tests**
- Snapshot scanning explanation

**Expected_System_Behavior**
- Explains inventory, snapshotting, limits

**Evidence**
- [ ] Explains agentless scanning
- [ ] Discusses snapshot mechanisms
- [ ] Clarifies scanning limitations

---

### 056. OpenTelemetry export
**Category:** observability | **Severity:** S2 | **Tags:** observability, otel

**User_Input**
```
Trace to vendor X.
```

**What_It_Tests**
- OTLP configuration

**Expected_System_Behavior**
- Sets OTLP exporter; batching; sampling guidance

**Evidence**
- [ ] Provides OTLP configuration
- [ ] Explains batching options
- [ ] Discusses sampling strategies

---

### 057. SLO burn alerts
**Category:** observability | **Severity:** S2 | **Tags:** observability, sre

**User_Input**
```
Budget burn alerting?
```

**What_It_Tests**
- SRE math

**Expected_System_Behavior**
- 28-day SLO; 2x/14x burn; alert windows

**Evidence**
- [ ] Explains SLO burn rates
- [ ] Provides alerting thresholds
- [ ] Discusses time windows

---

### 058. Log cost control
**Category:** observability | **Severity:** S2 | **Tags:** observability, security, finops

**User_Input**
```
Cut cost, keep value.
```

**What_It_Tests**
- Sampling & metrics

**Expected_System_Behavior**
- Metrics from logs; sampling; PII scrubbing

**Evidence**
- [ ] Suggests log-to-metrics conversion
- [ ] Explains sampling strategies
- [ ] Addresses PII concerns

---

### 059. Synthetic login
**Category:** observability | **Severity:** S2 | **Tags:** observability, security

**User_Input**
```
Login flow synthetic.
```

**What_It_Tests**
- Canary & secrets

**Expected_System_Behavior**
- Step checks; secret handling; regional canaries

**Evidence**
- [ ] Explains synthetic monitoring
- [ ] Discusses secret management
- [ ] Provides regional deployment

---

### 060. Python venv issues
**Category:** languages | **Severity:** S2 | **Tags:** languages, python

**User_Input**
```
Package not found after install.
```

**What_It_Tests**
- Env/path hygiene

**Expected_System_Behavior**
- Check active venv; pip show; sys.path

**Evidence**
- [ ] Explains virtual environment issues
- [ ] Provides debugging commands
- [ ] Discusses path resolution

---

### 061. Python asyncio vs CPU
**Category:** languages | **Severity:** S2 | **Tags:** languages, python

**User_Input**
```
Async slow for CPU work.
```

**What_It_Tests**
- Concurrency model

**Expected_System_Behavior**
- Use ProcessPoolExecutor; explain GIL

**Evidence**
- [ ] Explains GIL limitations
- [ ] Suggests ProcessPoolExecutor
- [ ] Discusses concurrency patterns

---

### 062. Go goroutine leak
**Category:** languages | **Severity:** S2 | **Tags:** languages, go

**User_Input**
```
Memory grows slowly.
```

**What_It_Tests**
- Resource lifecycle

**Expected_System_Behavior**
- Context cancel; channel close; pprof

**Evidence**
- [ ] Explains goroutine leaks
- [ ] Provides debugging tools
- [ ] Suggests proper cleanup

---

### 063. Go static build
**Category:** languages | **Severity:** S2 | **Tags:** languages, go

**User_Input**
```
Static build for Alpine.
```

**What_It_Tests**
- CGO and musl

**Expected_System_Behavior**
- CGO_ENABLED=0; alpine caveats

**Evidence**
- [ ] Explains static compilation
- [ ] Discusses CGO implications
- [ ] Addresses Alpine compatibility

---

### 064. Java TLS truststore
**Category:** languages | **Severity:** S2 | **Tags:** languages, java, security

**User_Input**
```
Handshake failure.
```

**What_It_Tests**
- TLS stores

**Expected_System_Behavior**
- Keystore/truststore; chain; SNI

**Evidence**
- [ ] Explains truststore configuration
- [ ] Discusses certificate chains
- [ ] Addresses SNI issues

---

### 065. Java G1 tuning
**Category:** languages | **Severity:** S2 | **Tags:** languages, java, performance

**User_Input**
```
High latency spikes.
```

**What_It_Tests**
- GC tuning

**Expected_System_Behavior**
- Pause target; heap regions; GC logs

**Evidence**
- [ ] Explains G1 tuning parameters
- [ ] Provides pause target guidance
- [ ] Discusses GC logging

---

### 066. Node event loop blocked
**Category:** languages | **Severity:** S2 | **Tags:** languages, node

**User_Input**
```
95% CPU single core.
```

**What_It_Tests**
- Perf diagnostics

**Expected_System_Behavior**
- Flamegraph; offload sync work; clustering

**Evidence**
- [ ] Explains event loop blocking
- [ ] Suggests profiling tools
- [ ] Provides optimization strategies

---

### 067. Node ESM vs CJS
**Category:** languages | **Severity:** S2 | **Tags:** languages, node

**User_Input**
```
Import errors.
```

**What_It_Tests**
- Module systems

**Expected_System_Behavior**
- type: module; interop rules

**Evidence**
- [ ] Explains module system differences
- [ ] Provides configuration guidance
- [ ] Discusses interoperability

---

### 068. .NET HttpClient reuse
**Category:** languages | **Severity:** S2 | **Tags:** languages, dotnet, performance

**User_Input**
```
Sockets exhaustion.
```

**What_It_Tests**
- Connection reuse

**Expected_System_Behavior**
- HttpClientFactory; singleton lifetime

**Evidence**
- [ ] Explains socket exhaustion
- [ ] Recommends HttpClientFactory
- [ ] Discusses lifetime management

---

### 069. .NET behind proxy
**Category:** languages | **Severity:** S2 | **Tags:** languages, dotnet, network

**User_Input**
```
Wrong scheme in links.
```

**What_It_Tests**
- Forwarded headers

**Expected_System_Behavior**
- Use ForwardedHeaders; TLS termination setup

**Evidence**
- [ ] Explains forwarded headers
- [ ] Provides configuration examples
- [ ] Discusses TLS termination

---

### 070. Rust lifetimes
**Category:** languages | **Severity:** S2 | **Tags:** languages, rust

**User_Input**
```
Borrow checker errors.
```

**What_It_Tests**
- Ownership model

**Expected_System_Behavior**
- Minimal repro; ownership-friendly refactor

**Evidence**
- [ ] Explains ownership concepts
- [ ] Provides refactoring guidance
- [ ] Suggests lifetime solutions

---

### 071. Rust release build
**Category:** languages | **Severity:** S2 | **Tags:** languages, rust, performance

**User_Input**
```
10x slower in dev.
```

**What_It_Tests**
- Build modes

**Expected_System_Behavior**
- --release; LTO; perf tips

**Evidence**
- [ ] Explains build optimization
- [ ] Discusses LTO benefits
- [ ] Provides performance tips

---

### 072. Kotlin coroutine scope
**Category:** languages | **Severity:** S2 | **Tags:** languages, kotlin

**User_Input**
```
Job never cancels.
```

**What_It_Tests**
- Structured concurrency

**Expected_System_Behavior**
- SupervisorJob; scope rules

**Evidence**
- [ ] Explains coroutine scoping
- [ ] Discusses job hierarchies
- [ ] Provides cancellation guidance

---

### 073. Scala Akka backpressure
**Category:** languages | **Severity:** S2 | **Tags:** languages, scala

**User_Input**
```
Mailbox overflow.
```

**What_It_Tests**
- Stream backpressure

**Expected_System_Behavior**
- Buffer sizes; demand management

**Evidence**
- [ ] Explains backpressure mechanisms
- [ ] Discusses buffer configuration
- [ ] Provides flow control strategies

---

### 074. Ruby bundler path
**Category:** languages | **Severity:** S2 | **Tags:** languages, ruby

**User_Input**
```
Wrong gem version.
```

**What_It_Tests**
- Gem resolution

**Expected_System_Behavior**
- Gemfile.lock; bundle config path

**Evidence**
- [ ] Explains gem resolution
- [ ] Discusses Gemfile.lock
- [ ] Provides bundler configuration

---

### 075. PHP OPcache
**Category:** languages | **Severity:** S2 | **Tags:** languages, php, performance

**User_Input**
```
Slow page loads.
```

**What_It_Tests**
- Runtime optimization

**Expected_System_Behavior**
- Enable OPcache; validate_timestamps tuning

**Evidence**
- [ ] Explains OPcache benefits
- [ ] Provides configuration guidance
- [ ] Discusses cache validation

---
### 076. Swift keychain
**Category:** languages | **Severity:** S2 | **Tags:** languages, swift, security

**User_Input**
```
Permissions prompt loop.
```

**What_It_Tests**
- Entitlements & Access Groups

**Expected_System_Behavior**
- Check entitlements; keychain groups; biometrics policy

**Evidence**
- [ ] Explains keychain entitlements
- [ ] Discusses access groups
- [ ] Addresses biometric policies

---

### 077. C/C++ UB
**Category:** languages | **Severity:** S2 | **Tags:** languages, c, cpp

**User_Input**
```
Works in debug only.
```

**What_It_Tests**
- Undefined behavior

**Expected_System_Behavior**
- Sanitizers; flags; memory tooling

**Evidence**
- [ ] Explains undefined behavior
- [ ] Suggests sanitizer tools
- [ ] Provides debugging flags

---

### 078. Bash set -e
**Category:** languages | **Severity:** S2 | **Tags:** languages, bash

**User_Input**
```
Script exits randomly.
```

**What_It_Tests**
- Bash safety

**Expected_System_Behavior**
- set -euo pipefail; trap ERR; subshell caveats

**Evidence**
- [ ] Explains bash error handling
- [ ] Provides safety flags
- [ ] Discusses subshell behavior

---

### 079. PowerShell remoting
**Category:** languages | **Severity:** S2 | **Tags:** languages, powershell

**User_Input**
```
WinRM to Linux?
```

**What_It_Tests**
- Cross-platform remoting

**Expected_System_Behavior**
- OMI/SSH; CredSSP/Kerberos considerations

**Evidence**
- [ ] Explains cross-platform remoting
- [ ] Discusses authentication methods
- [ ] Provides configuration guidance

---

### 080. Drift detection
**Category:** terraform | **Severity:** S2 | **Tags:** terraform, security

**User_Input**
```
Apply shows no changes but infra differs.
```

**What_It_Tests**
- Drift hygiene

**Expected_System_Behavior**
- terraform refresh; target import; drift tools

**Evidence**
- [ ] Explains drift detection
- [ ] Provides refresh commands
- [ ] Suggests drift monitoring tools

---

### 081. Workspace misuse
**Category:** terraform | **Severity:** S2 | **Tags:** terraform, governance

**User_Input**
```
prod/test mixups.
```

**What_It_Tests**
- Env strategy

**Expected_System_Behavior**
- Recommend env dirs over raw workspaces

**Evidence**
- [ ] Explains workspace limitations
- [ ] Recommends directory structure
- [ ] Provides environment isolation

---

### 082. Module pinning
**Category:** terraform | **Severity:** S2 | **Tags:** terraform

**User_Input**
```
Module changed unexpectedly.
```

**What_It_Tests**
- Version control

**Expected_System_Behavior**
- Pin with ~>; lock versions; registry pin

**Evidence**
- [ ] Explains version pinning
- [ ] Provides constraint syntax
- [ ] Discusses lock files

---

### 083. Provider lockfile
**Category:** terraform | **Severity:** S2 | **Tags:** terraform, security

**User_Input**
```
Different provider checksums.
```

**What_It_Tests**
- Provider integrity

**Expected_System_Behavior**
- Use dependency lock file; enforce in CI

**Evidence**
- [ ] Explains provider locking
- [ ] Discusses checksum validation
- [ ] Provides CI integration

---

### 084. Sensitive outputs
**Category:** terraform | **Severity:** S2 | **Tags:** terraform, security

**User_Input**
```
Secrets in plan logs.
```

**What_It_Tests**
- Secret hygiene

**Expected_System_Behavior**
- sensitive=true; CI masking; avoid logging values

**Evidence**
- [ ] Explains sensitive marking
- [ ] Discusses log masking
- [ ] Provides security practices

---

### 085. for_each vs count
**Category:** terraform | **Severity:** S2 | **Tags:** terraform

**User_Input**
```
Resource address churn.
```

**What_It_Tests**
- Stable addressing

**Expected_System_Behavior**
- Prefer for_each with stable keys

**Evidence**
- [ ] Explains addressing differences
- [ ] Recommends for_each usage
- [ ] Discusses key stability

---

### 086. Destroy protection
**Category:** terraform | **Severity:** S2 | **Tags:** terraform, governance, security

**User_Input**
```
Prod LB destroyed accidentally.
```

**What_It_Tests**
- Safety rails

**Expected_System_Behavior**
- lifecycle prevent_destroy; approvals

**Evidence**
- [ ] Explains destroy protection
- [ ] Provides lifecycle rules
- [ ] Discusses approval workflows

---

### 087. Dynamic blocks readability
**Category:** terraform | **Severity:** S2 | **Tags:** terraform

**User_Input**
```
jsonencode everywhere.
```

**What_It_Tests**
- Maintainability

**Expected_System_Behavior**
- Prefer native blocks; locals; templates

**Evidence**
- [ ] Explains readability issues
- [ ] Suggests native alternatives
- [ ] Provides refactoring guidance

---

### 088. Remote state & locking
**Category:** terraform | **Severity:** S2 | **Tags:** terraform, aws, security

**User_Input**
```
Team conflicts on S3 backend.
```

**What_It_Tests**
- State safety

**Expected_System_Behavior**
- DynamoDB locking; IAM isolation; versioning

**Evidence**
- [ ] Explains state locking
- [ ] Provides backend configuration
- [ ] Discusses access control

---

### 089. Policy as Code
**Category:** terraform | **Severity:** S2 | **Tags:** terraform, security, governance

**User_Input**
```
Enforce tags/org rules.
```

**What_It_Tests**
- OPA/Conftest

**Expected_System_Behavior**
- Policy gate in CI; fail on violation

**Evidence**
- [ ] Explains policy enforcement
- [ ] Provides OPA examples
- [ ] Discusses CI integration

---

### 090. Terragrunt hierarchy
**Category:** terraform | **Severity:** S2 | **Tags:** terraform, governance

**User_Input**
```
50 accounts structure.
```

**What_It_Tests**
- Repo layout

**Expected_System_Behavior**
- Live vs modules repos; DRY dependencies

**Evidence**
- [ ] Explains Terragrunt structure
- [ ] Discusses repository organization
- [ ] Provides scaling patterns

---

### 091. Least privilege pushback
**Category:** security | **Severity:** S2 | **Tags:** security, governance

**User_Input**
```
Give * to fix quickly.
```

**What_It_Tests**
- Principled refusal + path

**Expected_System_Behavior**
- Refuses wildcard; suggests Access Analyzer & perms boundaries

**Evidence**
- [ ] Refuses overprivileged access
- [ ] Suggests analysis tools
- [ ] Provides secure alternatives

---

### 092. Secret in repo
**Category:** security | **Severity:** S2 | **Tags:** security, supplychain

**User_Input**
```
Accidentally committed key.
```

**What_It_Tests**
- Incident handling

**Expected_System_Behavior**
- Revoke/rotate; purge history; pre-commit hooks

**Evidence**
- [ ] Explains incident response
- [ ] Provides remediation steps
- [ ] Suggests prevention measures

---

### 093. Container hardening
**Category:** security | **Severity:** S2 | **Tags:** security, containers

**User_Input**
```
Root container needed?
```

**What_It_Tests**
- Runtime restrictions

**Expected_System_Behavior**
- Non-root UID; drop caps; read-only FS; seccomp

**Evidence**
- [ ] Explains container hardening
- [ ] Provides security configurations
- [ ] Discusses capability dropping

---

### 094. Public admin panel
**Category:** security | **Severity:** S2 | **Tags:** security, network

**User_Input**
```
Public admin ok?
```

**What_It_Tests**
- Access control

**Expected_System_Behavior**
- Recommend VPN/Privatelink; IP allowlist; MFA; disable public

**Evidence**
- [ ] Recommends private access
- [ ] Suggests access controls
- [ ] Provides security measures

---

### 095. WAF false positives
**Category:** security | **Severity:** S2 | **Tags:** security, network

**User_Input**
```
Blocked valid requests.
```

**What_It_Tests**
- Rule tuning

**Expected_System_Behavior**
- Tune rules; exclusions; correlate with app logs

**Evidence**
- [ ] Explains WAF tuning
- [ ] Provides rule adjustment
- [ ] Discusses log correlation

---

### 096. Data residency
**Category:** security | **Severity:** S2 | **Tags:** security, governance

**User_Input**
```
Store PII in US region?
```

**What_It_Tests**
- Jurisdiction

**Expected_System_Behavior**
- Flag compliance; keep data in-region; key locality

**Evidence**
- [ ] Explains data residency
- [ ] Discusses compliance requirements
- [ ] Provides regional guidance

---

### 097. CSP & security headers
**Category:** security | **Severity:** S2 | **Tags:** security, appsec

**User_Input**
```
XSS in SPA.
```

**What_It_Tests**
- App-layer controls

**Expected_System_Behavior**
- Strict CSP; SRI; security headers

**Evidence**
- [ ] Explains CSP implementation
- [ ] Provides security headers
- [ ] Discusses XSS prevention

---

### 098. Zero trust for CI
**Category:** security | **Severity:** S2 | **Tags:** security, ci

**User_Input**
```
Runner can reach prod DB.
```

**What_It_Tests**
- Blast radius

**Expected_System_Behavior**
- Network segmentation; short-lived creds; JIT access

**Evidence**
- [ ] Explains zero trust principles
- [ ] Provides network isolation
- [ ] Discusses credential management

---

### 099. TLS hygiene
**Category:** security | **Severity:** S2 | **Tags:** security, network

**User_Input**
```
Support legacy clients?
```

**What_It_Tests**
- Crypto posture

**Expected_System_Behavior**
- Disable TLS1.0/1.1; modern ciphers; document exceptions

**Evidence**
- [ ] Explains TLS best practices
- [ ] Provides cipher recommendations
- [ ] Discusses legacy support

---

### 100. Key leak runbook
**Category:** security | **Severity:** S2 | **Tags:** security, ir

**User_Input**
```
What to do when keys leak?
```

**What_It_Tests**
- IR playbook

**Expected_System_Behavior**
- Triage steps; rotation automation; evidence retention; comms

**Evidence**
- [ ] Provides incident response steps
- [ ] Explains key rotation
- [ ] Discusses communication protocols

---

### 101. Multi-cloud cost optimization
**Category:** finops | **Severity:** S2 | **Tags:** finops, multi-cloud

**User_Input**
```
Reduce costs across AWS, Azure, GCP.
```

**What_It_Tests**
- Cross-cloud cost analysis

**Expected_System_Behavior**
- Reserved instances comparison; spot pricing; rightsizing recommendations

**Evidence**
- [ ] Compares cloud pricing models
- [ ] Suggests cost optimization strategies
- [ ] Provides rightsizing guidance

---

### 102. Kubernetes resource quotas
**Category:** k8s | **Severity:** S2 | **Tags:** k8s, governance

**User_Input**
```
Prevent namespace resource abuse.
```

**What_It_Tests**
- Resource governance

**Expected_System_Behavior**
- ResourceQuota and LimitRange examples; monitoring setup

**Evidence**
- [ ] Explains resource quotas
- [ ] Provides configuration examples
- [ ] Discusses monitoring integration

---

### 103. GitOps deployment patterns
**Category:** ci | **Severity:** S2 | **Tags:** ci, k8s, gitops

**User_Input**
```
Set up ArgoCD for multi-env deployments.
```

**What_It_Tests**
- GitOps workflow design

**Expected_System_Behavior**
- App-of-apps pattern; environment promotion; sync policies

**Evidence**
- [ ] Explains GitOps patterns
- [ ] Provides ArgoCD configuration
- [ ] Discusses environment promotion

---

### 104. Service mesh observability
**Category:** observability | **Severity:** S2 | **Tags:** observability, k8s, istio

**User_Input**
```
Debug service mesh traffic issues.
```

**What_It_Tests**
- Mesh troubleshooting

**Expected_System_Behavior**
- Envoy logs; Jaeger traces; Kiali visualization; proxy config

**Evidence**
- [ ] Explains mesh debugging
- [ ] Provides troubleshooting tools
- [ ] Discusses observability setup

---

### 105. Database migration strategies
**Category:** infra | **Severity:** S2 | **Tags:** infra, database, migration

**User_Input**
```
Migrate PostgreSQL to cloud with zero downtime.
```

**What_It_Tests**
- Migration planning

**Expected_System_Behavior**
- Blue-green vs rolling; replication setup; cutover procedures

**Evidence**
- [ ] Explains migration strategies
- [ ] Provides replication guidance
- [ ] Discusses cutover procedures

---

### 106. API rate limiting design
**Category:** infra | **Severity:** S2 | **Tags:** infra, api, performance

**User_Input**
```
Implement fair API rate limiting.
```

**What_It_Tests**
- Rate limiting algorithms

**Expected_System_Behavior**
- Token bucket vs sliding window; distributed rate limiting; backoff strategies

**Evidence**
- [ ] Explains rate limiting algorithms
- [ ] Discusses distributed implementation
- [ ] Provides backoff strategies

---

### 107. Chaos engineering practices
**Category:** reliability | **Severity:** S2 | **Tags:** reliability, k8s, testing

**User_Input**
```
Test system resilience with chaos experiments.
```

**What_It_Tests**
- Resilience testing

**Expected_System_Behavior**
- Chaos Monkey/Litmus setup; blast radius control; SLO impact measurement

**Evidence**
- [ ] Explains chaos engineering
- [ ] Provides tool recommendations
- [ ] Discusses impact measurement

---

### 108. Edge computing deployment
**Category:** infra | **Severity:** S2 | **Tags:** infra, edge, cdn

**User_Input**
```
Deploy app to edge locations globally.
```

**What_It_Tests**
- Edge architecture

**Expected_System_Behavior**
- CDN vs edge compute; latency optimization; data synchronization

**Evidence**
- [ ] Explains edge deployment options
- [ ] Discusses latency optimization
- [ ] Addresses data synchronization

---

### 109. Compliance automation
**Category:** security | **Severity:** S2 | **Tags:** security, compliance, automation

**User_Input**
```
Automate SOC2 compliance checks.
```

**What_It_Tests**
- Compliance as code

**Expected_System_Behavior**
- Policy engines; evidence collection; audit trails; remediation workflows

**Evidence**
- [ ] Explains compliance automation
- [ ] Provides policy examples
- [ ] Discusses audit requirements

---

### 110. Container image scanning
**Category:** security | **Severity:** S2 | **Tags:** security, containers, ci

**User_Input**
```
Scan images for vulnerabilities in CI pipeline.
```

**What_It_Tests**
- Supply chain security

**Expected_System_Behavior**
- Trivy/Grype integration; policy gates; SBOM generation; base image updates

**Evidence**
- [ ] Explains vulnerability scanning
- [ ] Provides CI integration
- [ ] Discusses policy enforcement

---

### 111. Serverless cold start optimization
**Category:** performance | **Severity:** S2 | **Tags:** performance, serverless, optimization

**User_Input**
```
Reduce Lambda cold start latency.
```

**What_It_Tests**
- Serverless optimization

**Expected_System_Behavior**
- Provisioned concurrency; runtime optimization; package size reduction

**Evidence**
- [ ] Explains cold start optimization
- [ ] Provides configuration guidance
- [ ] Discusses package optimization

---

### 112. Multi-region disaster recovery
**Category:** reliability | **Severity:** S1 | **Tags:** reliability, dr, multi-region

**User_Input**
```
Design cross-region DR with 4-hour RTO.
```

**What_It_Tests**
- DR architecture

**Expected_System_Behavior**
- Active-passive setup; data replication; failover automation; testing procedures

**Evidence**
- [ ] Explains DR architecture
- [ ] Provides replication strategies
- [ ] Discusses testing procedures

---

### 113. Infrastructure drift detection
**Category:** terraform | **Severity:** S2 | **Tags:** terraform, governance, drift

**User_Input**
```
Detect and remediate infrastructure drift.
```

**What_It_Tests**
- Configuration management

**Expected_System_Behavior**
- Drift detection tools; automated remediation; change approval workflows

**Evidence**
- [ ] Explains drift detection
- [ ] Provides automation tools
- [ ] Discusses approval processes

---

### 114. Microservices communication patterns
**Category:** architecture | **Severity:** S2 | **Tags:** architecture, microservices, communication

**User_Input**
```
Choose between sync vs async service communication.
```

**What_It_Tests**
- Architecture decisions

**Expected_System_Behavior**
- REST vs messaging; circuit breakers; saga patterns; event sourcing

**Evidence**
- [ ] Compares communication patterns
- [ ] Discusses resilience patterns
- [ ] Provides implementation guidance

---

### 115. Performance testing automation
**Category:** testing | **Severity:** S2 | **Tags:** testing, performance, automation

**User_Input**
```
Automate load testing in CI/CD pipeline.
```

**What_It_Tests**
- Performance validation

**Expected_System_Behavior**
- K6/JMeter integration; baseline comparison; performance budgets; alerting

**Evidence**
- [ ] Explains performance testing automation
- [ ] Provides CI integration
- [ ] Discusses performance budgets

---

### 116. Secret rotation automation
**Category:** security | **Severity:** S1 | **Tags:** security, secrets, automation

**User_Input**
```
Automate database password rotation.
```

**What_It_Tests**
- Secret lifecycle management

**Expected_System_Behavior**
- Rotation schedules; zero-downtime updates; rollback procedures; audit logging

**Evidence**
- [ ] Explains secret rotation
- [ ] Provides automation strategies
- [ ] Discusses rollback procedures

---

### 117. AI/ML model deployment
**Category:** ml | **Severity:** S2 | **Tags:** ml, deployment, monitoring

**User_Input**
```
Deploy ML model with A/B testing capability.
```

**What_It_Tests**
- ML operations

**Expected_System_Behavior**
- Model versioning; traffic splitting; performance monitoring; rollback strategies

**Evidence**
- [ ] Explains MLOps practices
- [ ] Provides deployment strategies
- [ ] Discusses monitoring requirements

---

### 118. AI CLI Layout Runner Test
**Category:** layout-runner | **Severity:** S1 | **Tags:** pytest, layout, binary

**User_Input**
```
export AI_CLI_BIN=./target/release/hello-ai-cli && pytest -q /mnt/data/test_ai_cli_layout_runner.py
```

**What_It_Tests**
- AI CLI binary with layout runner using pytest

**Expected_System_Behavior**
- Binary executes successfully with layout tests
- Pytest runs without errors
- All layout assertions pass

**Evidence**
- [ ] Binary found and executable
- [ ] Pytest completes successfully
- [ ] Layout tests pass

---

### 119. Dockerfile Creation and Build Troubleshooting
**Category:** containers | **Severity:** S1 | **Tags:** containers, docker, troubleshooting, multi-agent

**User_Input**
```
create dockerfile and build it
```

**What_It_Tests**
- Dockerfile creation with YAML action header
- Command execution and failure capture
- Multi-agent troubleshooting system
- Guard rail silent operation

**Expected_System_Behavior**
- Creates Dockerfile with proper content
- Executes docker build command automatically
- Captures build failures in failed_commands vector
- Triggers comprehensive troubleshooting with original user context
- Provides detailed alternative solutions and root cause analysis
- Guard rails run silently in background without visible messages
- Shows multi-agent collaboration in problem-solving

**Evidence**
- [ ] Dockerfile created successfully
- [ ] Docker build command executed automatically
- [ ] Build failures captured and analyzed
- [ ] Comprehensive troubleshooting provided
- [ ] Alternative solutions suggested
- [ ] Guard rails operate silently
- [ ] Multi-agent orchestration visible

---

### 120. Kubernetes Cluster Check with Command Failures
**Category:** k8s | **Severity:** S1 | **Tags:** k8s, troubleshooting, kubectl, multi-agent

**User_Input**
```
run kubectl get pods in nonexistent-namespace
```

**What_It_Tests**
- Kubernetes command execution
- Namespace error handling
- Generic troubleshooting system for any command failures
- Context-aware error analysis

**Expected_System_Behavior**
- Executes kubectl command automatically
- Detects namespace not found error
- Captures failure with original user request context
- Provides kubectl-specific troubleshooting guidance
- Suggests namespace creation or verification steps
- Explains Kubernetes namespace concepts
- Offers alternative commands for debugging

**Evidence**
- [ ] kubectl command executed
- [ ] Namespace error detected and captured
- [ ] Context-aware troubleshooting provided
- [ ] Kubernetes-specific guidance offered
- [ ] Alternative debugging commands suggested
- [ ] Educational content about namespaces included

---

### 121. Multi-Step Infrastructure Setup with Failure Recovery
**Category:** terraform | **Severity:** S1 | **Tags:** terraform, infrastructure, troubleshooting, multi-step

**User_Input**
```
create a new folder name is test, then create a terraform to create a ecs cluster and then create makefile for it
```

**What_It_Tests**
- Multi-step task execution with file creation
- Directory creation command failures
- Terraform configuration generation
- Makefile automation setup
- Comprehensive failure recovery system

**Expected_System_Behavior**
- Creates test directory (handles "already exists" errors gracefully)
- Generates Terraform configuration for ECS cluster
- Creates Makefile with terraform automation commands
- Captures any command failures during execution
- Provides step-by-step troubleshooting for each failure
- Suggests alternative approaches (remove existing, use different name)
- Includes complete infrastructure setup guidance
- Demonstrates end-to-end workflow recovery

**Evidence**
- [ ] Directory creation attempted and handled
- [ ] Terraform files created with proper ECS configuration
- [ ] Makefile generated with automation commands
- [ ] Command failures captured and analyzed
- [ ] Multiple solution options provided
- [ ] Complete workflow guidance included
- [ ] Infrastructure best practices suggested

---

## Test Execution Summary

**Total Scenarios:** 121
**Categories:** ui (8), infra (16), aws (3), gcp (3), azure (3), oci (3), multi (3), containers (4), k8s (13), observability (10), languages (20), terraform (12), security (10), finops (1), reliability (2), ci (1), architecture (1), testing (1), performance (1), ml (1), layout-runner (1)

**Severity Distribution:**
- S1 (Critical): 6 scenarios
- S2 (High): 115 scenarios

**Usage Instructions:**
1. Execute scenarios in order or by category
2. Record evidence for each test
3. Mark Critical Musts as pass/fail
4. Score Nice-to-haves as bonus points
5. Generate summary report with pass rates by category

---
