# iot-smartcard-project
Java smartcard project

## Java applet - example

### Compile Java applet:

`javac -source 1.2 -target 1.1 -g -cp /path/to/oracle_javacard_sdksmaster/jc211_kit/bin/api.jar helloWorld/HelloWorld.java`

### Convert .class to .cap:


`java -classpath $JC_HOME_TOOLS/bin/converter.jar:. com.sun.javacard.converter.Converter -verbose -exportpath $JC_HOME_TOOLS/api_export_files:helloWorld -classdir . -applet 0xa0:0x0:0x0:0x0:0x62:0x3:0x1:0xc:0x6:0x1:0x2 HelloWorld helloWorld 0x0a:0x0:0x0:0x0:0x62:0x3:0x1:0xc:0x6:0x1 1.0`

Here, we defined:
- The applet name (helloWorld)
- The applet AID: 0xA00000006203010C060102
- The package AID: 0x0A0000006203010C0601
- You can choose shorter AIDs (according to https://docs.oracle.com/javacard/3.0.5/api/javacard/framework/AID.html, the AID is a sequence of bytes between 5 and 16 bytes in length …)

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

### Upload cap to card:

```
mode_201
enable_trace
enable_timer
establish_contex
card_connect
select -AID A000000018434D00
open_sc -security 3 -keyind 0 -keyver 0 -key 47454d5850524553534f53414d504c45 -keyDerivation visa2
install -file helloWorld/javacard/helloWorld.cap -sdAID A000000018434D00 -nvCodeLimit 4000
card_disconnect
release_context
```

### Delete installed applet on card:

```
mode_201
gemXpressoPro
enable_trace
enable_timer
establish_context
card_connect
select -AID A000000018434D00
open_sc -security 0 -keyind 0 -keyver 0 -key 47454d5850524553534f53414d504c45
delete -AID a00000006203010c060102
delete -AID 0a0000006203010c0601
card_disconnect
release_context
```

## APDU commands

Type these commands on gscriptor

`RESET`

### SELECT installed applet:

`00 A4 04 00 08 A0 00 00 00 62 03 01 0C 06 01 02`

https://www.infoworld.com/article/2076450/how-to-write-a-java-card-applet--a-developer-s-guide.html?page=2

### Run command 0x40 without parameter:

`00 40 00 00 0C`

## Build project

First set **JAVACARD_SDK_211_JAR** env variable to your javacard SDK 211 api.jar

`export JAVACARD_SDK_211_JAR=/path/to/oracle_javacard_sdks/jc211_kit/bin/api.jar`

Then build the project

```
chmod +x build.sh
./build.sh
```
