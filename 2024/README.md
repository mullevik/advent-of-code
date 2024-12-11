
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

 01       1.88      10.27
 02       2.27       8.77
 03       0.45       0.45
 04       9.44       7.71
 05      76.31     588.67
 06       6.31    5469.66
 07     301.09   26343.31
 08       2.58       4.24
 09       7.89     818.88
 10      19.23       6.61
 11       4.31     228.25
```
