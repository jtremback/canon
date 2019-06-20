This is a command line utility to put JSON into a canonical form.

WARNING: Currently this just deserializes and researializes the JSON with serde_json, which sorts the keys in objects. It should probably be using something more intentionally canonical.

Usage:

```
$ echo '{ "zoology": "bar", "ambergris": "foo" }' | canon
{"ambergris":"foo","zoology":"bar"}
```

Installation:

You're going to need to have `cargo` and the other rust stuff installed.
