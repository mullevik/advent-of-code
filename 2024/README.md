
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


## Google cloud function

Deploy AoC bot to Google Cloud Run
```
gcloud functions deploy aoc-bot \
--gen2 \
--runtime=nodejs22 \
--region=europe-central2 \
--entry-point=aocBotEntrypoint \
--trigger-topic=aoc-triggers
```

## AOC day results

```
day    p1 (ms)    p2 (ms)

 01       1.53       7.81
 02       2.22       6.26
 03       0.35       0.37
 04       7.02       5.28
 05      51.99     473.81
 06       6.88    4739.10
 07     233.47   22877.94
 08       2.84       3.82
 09       7.97     728.27
```
