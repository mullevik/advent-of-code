
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

 01       1.51       1.54
 02       1.64       1.29
 03       0.33       0.21
 04       6.68       5.67
 05      59.93      51.10
 06       6.74       2.21
 07     261.23     251.96
 08       2.55       2.08
 09       6.70       3.87
```
