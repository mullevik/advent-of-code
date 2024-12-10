
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

 01       1.41       7.85
 02       1.77      11.70
 03       0.31       0.34
 04       6.60       5.49
 05      53.86     491.62
 06       4.41    5173.50
 07     259.80   23494.37
 08       1.54       2.70
 09       6.28     743.99
 10      13.96       4.60
```
