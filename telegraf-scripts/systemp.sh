# [[inputs.temp]] doesn't work on pi
# instead, collect the data ourselves and send it to influx in influx line protocol:
# https://v2.docs.influxdata.com/v2.0/reference/syntax/line-protocol/
echo "systemp temp=`cat /sys/class/thermal/thermal_zone0/temp`"
