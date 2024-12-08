
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

| day    |    p1 (ms) |    p2 (ms) | 
| ------ | ---------- | ---------- | 
| day 01 |       1.48 |       1.66 | 
| day 02 |       1.72 |       1.53 | 
| day 03 |       0.39 |       0.23 | 
| day 04 |       6.95 |       6.97 | 
| day 05 |      60.86 |      52.37 | 
| day 06 |       6.16 |       1.74 | 
| day 07 |     256.98 |     278.90 | 
| day 08 |       2.43 |       1.91 | 
