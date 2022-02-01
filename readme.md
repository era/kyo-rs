# kyo-rs

![](kyo.jpg)

**status**: wip

kyo-rs is a software to quick start a fake API in order to validate your front-end. You basically pass a list of resources and kyo-rs will generat REST endpoints for you. The objects are saved in memory, in other words, when you restart the server all data is lost.

## API usage
`kyo-rs PORT JSON_CONFIG_FILE`

PORT: The port where the server is suppose to run
JSON_CONFIG_FILE: a Json config file as in:
```
{
"objects": [
	"teachers",
	"students",
]}

```

In that example, you would have two endpoints: localhost:PORT/teachers and localhost:PORT/students. To insert, update or get an item you just have to send a POST, PUT or GET HTTP request. PUT and GET requires you to send the resource id in the path, such as: localhsot:PORT/students/42
