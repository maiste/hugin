### Run Hugin

Hugin is a program that runs only in CLI. It provides some options to configure it.

* To run a web server on a specific port you must use:
```sh
$ hugin run --port [PORT]
```
By default, hugin runs on port `8080`.

* You can also specify the directory where Hugin must look for the JSON files:
```sh
$ hugin run --dir [DIR]
```
By default, hugins uses the directory `content`.
