
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
