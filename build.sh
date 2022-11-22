#!/bin/bash

javac -source 1.2 -target 1.1 -g -cp ./lib/api_classic.jar -Xlint:-options smartCardProject/SmartCardProject.java

java -classpath $JC_HOME_TOOLS/bin/converter.jar:. com.sun.javacard.converter.Converter -verbose -exportpath $JC_HOME_TOOLS/api_export_files:smartCardProject -classdir . -applet 0xa0:0x0:0x0:0x0:0x62:0x3:0x1:0xc:0x6:0x1:0x2 SmartCardProject smartCardProject 0x0a:0x0:0x0:0x0:0x62:0x3:0x1:0xc:0x6:0x1 1.0

