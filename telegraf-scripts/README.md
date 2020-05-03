# Scripts for writing data to InfluxDB via Telegraf on Raspberry Pi

## Installation

Run the following to download the scripts to `tmp`:

```
wget https://raw.githubusercontent.com/mikemahoney218/pi-admin/master/telegraf-scripts -p /tmp
```

Now add any scripts you're interested in to your `telegraf.conf` file under an
`inputs.exec` header -- for instance, to use the system temp shell script:

```
[[inputs.exec]]
  commands = ["sh /tmp/telegraf-scripts/systemp.sh"]
  timeout = "5s"
  data_format = "influx"
```
