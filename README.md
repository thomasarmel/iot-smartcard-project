# iot-smartcard-project
Java smartcard project

### Compile Java application:

`javac -source 1.2 -target 1.1 -g -cp /path/to/oracle_javacard_sdksmaster/jc211_kit/bin/api.jar helloWorld/HelloWorld.java`

### Convert .class to .cap:

`java -classpath $JC_HOME_TOOLS/bin/converter.jar:. com.sun.javacard.converter.Converter -verbose -exportpath $JC_HOME_TOOLS/api_export_files:helloWorld -classdir . -applet 0xa0:0x0:0x0:0x0:0x62:0x3:0x1:0xc:0x6:0x1:0x2 HelloWorld helloWorld 0x0a:0x0:0x0:0x0:0x62:0x3:0x1:0xc:0x6:0x1 1.0`

