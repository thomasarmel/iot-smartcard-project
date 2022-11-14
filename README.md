# iot-smartcard-project
Java smartcard project

### Compile Java application:

`javac -source 1.2 -target 1.1 -g -cp /path/to/oracle_javacard_sdksmaster/jc211_kit/bin/api.jar helloWorld/HelloWorld.java`

### Convert .class to .cap:

`java -classpath $JC_HOME_TOOLS/bin/converter.jar:. com.sun.javacard.converter.Converter -verbose -exportpath $JC_HOME_TOOLS/api_export_files:helloWorld -classdir . -applet 0xa0:0x0:0x0:0x0:0x62:0x3:0x1:0xc:0x6:0x1:0x2 HelloWorld helloWorld 0x0a:0x0:0x0:0x0:0x62:0x3:0x1:0xc:0x6:0x1 1.0`

## GPSHELL scripts

### List applets installed on card:

```
mode_201
gemXpressoPro
enable_trace
establish_context
card_connect
select -AID A000000018434D
open_sc -security 0 -keyind 0 -keyver 0 -keyDerivation visa2 -key 47454d5850524553534f53414d504c45
get_status -element 40
card_disconnect
release_context
```
