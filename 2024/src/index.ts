import * as functions from "@google-cloud/functions-framework";

functions.http('helloWorld', (req, res) => {
    console.log("hello world");
    res.send('Hello, World');
});