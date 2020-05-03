# Scripts for writing data to InfluxDB via Telegraf on Raspberry Pi

## Installation

To add the scripts to your `tmp` folder:

```
wget https://github.com/mikemahoney218/pi-admin/archive/master.zip -O telegraf-scripts.zip && \
    unzip telegraf-scripts.zip pi-admin-master/telegraf-scripts/* && \
    sudo mkdir /tmp/telegraf-scripts && \
    sudo mv pi-admin-master/telegraf-scripts/* /tmp/telegraf-scripts && \
    rm telegraf-scripts.zip && \
    rm -rf pi-admin-master
```

Now add any scripts you're interested in to your `telegraf.conf` file under an
`inputs.exec` header -- for instance, to use the system temp shell script:

```
[[inputs.exec]]
  commands = ["sh /tmp/telegraf-scripts/systemp.sh"]
  timeout = "5s"
  data_format = "influx"
```
