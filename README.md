## dumb_json_parser
### a dumb json parser experiment, as the name suggests, please don't use it

Is it easy to use? **No.** Is it fast? **Also no.** Is it production ready? **Are you serious?** Is there any error reporting? **Um.** Then, why write it? **Because of this tweet from gingerBill**:

<img src="./assets/tweet.png" width="60%" height="60%">

Okay, does it at least work? **Yes!**. What about the code? Do you want constructive feedback? **Of course! Though I may not apply what I learn from you in this experiment.**

---

It is spec compliant (or so I believe, I should run a test suite). Most of the verboseness comes from the weird design of the AST (which kind of nullifies the point of an AST in the first place, I guess), while I could spend more time polishing it, I think this is fine as it is as an experiment - looks like I *can* write a JSON parser.

#### Try it out
```bash
$ git clone https://github.com/furtidev/dumb_json_parser
$ cargo run --package crate_checker -- bevy # example app
```
