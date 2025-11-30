
## Install

```
npm install
npm run build  # to build .js from .ts
```

## Use
```
npm run benchmark  # to print benchmark output
npm run test  # to run unit tests
```


## Deploy

### GCP Infrastructure

1. There must be a GCP Cloud Scheduler which triggers a topic in GCP EventArc trigger.
2. There must be a GCP EventArc Trigger sends message to Cloud Run function.
3. There must be a GCP Cloud Run function deployed with this project.

### Google Cloud Run
Deploy the AoC bot to Cloud Run:
```
gcloud functions deploy aoc-bot \
--gen2 \
--runtime=nodejs22 \
--region=europe-central2 \
--entry-point=aocBotEntrypoint \
--trigger-topic=aoc-triggers
```

### Events
Check that there is an existing GCP EventArc trigger `aoc-bot-476273`.

Check that there is an existing GCP Cloud Scheduler `run-aoc-bot` and resume it.



## AOC day results

```
day    p1 (ms)    p2 (ms)

 01       1.36       7.18
 02       1.93       7.74
 03       0.30       0.30
 04      10.30       7.26
 05      74.34     686.95
 06       4.66    5359.73
 07     337.31   26482.69
 08       0.95       2.02
 09       5.25     864.66
 10      17.11       5.17
 11       3.84     251.41
```
