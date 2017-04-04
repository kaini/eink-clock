EESchema Schematic File Version 2
LIBS:power
LIBS:device
LIBS:transistors
LIBS:conn
LIBS:linear
LIBS:regul
LIBS:74xx
LIBS:cmos4000
LIBS:adc-dac
LIBS:memory
LIBS:xilinx
LIBS:microcontrollers
LIBS:dsp
LIBS:microchip
LIBS:analog_switches
LIBS:motorola
LIBS:texas
LIBS:intel
LIBS:audio
LIBS:interface
LIBS:digital-audio
LIBS:philips
LIBS:display
LIBS:cypress
LIBS:siliconi
LIBS:opto
LIBS:atmel
LIBS:contrib
LIBS:valves
LIBS:einkclock
LIBS:einkclock-cache
EELAYER 25 0
EELAYER END
$Descr A4 11693 8268
encoding utf-8
Sheet 1 5
Title "E-Ink Clock"
Date "2017-03-29"
Rev "1.0"
Comp ""
Comment1 ""
Comment2 ""
Comment3 ""
Comment4 ""
$EndDescr
$Comp
L Battery BT1
U 1 1 58DCADB5
P 1050 1650
F 0 "BT1" H 1150 1750 50  0000 L CNN
F 1 "6.4-4" H 1150 1650 50  0000 L CNN
F 2 "Wire_Connections_Bridges:WireConnection_1.00mmDrill" V 1050 1710 50  0001 C CNN
F 3 "" V 1050 1710 50  0000 C CNN
F 4 "12BH142A-GR" H 1050 1650 60  0001 C CNN "Manf#"
	1    1050 1650
	1    0    0    -1  
$EndComp
Wire Wire Line
	1050 1300 1050 1450
Wire Wire Line
	1050 1850 1050 1950
$Comp
L GND #PWR01
U 1 1 58DCB02E
P 1050 1950
F 0 "#PWR01" H 1050 1700 50  0001 C CNN
F 1 "GND" H 1050 1800 50  0000 C CNN
F 2 "" H 1050 1950 50  0000 C CNN
F 3 "" H 1050 1950 50  0000 C CNN
	1    1050 1950
	1    0    0    -1  
$EndComp
$Sheet
S 4300 1200 1050 2400
U 58DEA9FD
F0 "CPU" 60
F1 "cpu.sch" 60
F2 "DD0" I L 4300 2000 60 
F3 "DD1" I L 4300 2100 60 
F4 "DD2" I L 4300 2200 60 
F5 "DD3" I L 4300 2300 60 
F6 "DD4" I L 4300 2400 60 
F7 "DD5" I L 4300 2500 60 
F8 "DD6" I L 4300 2600 60 
F9 "DCL" I L 4300 2800 60 
F10 "DLE" I L 4300 2900 60 
F11 "DOE" I L 4300 3000 60 
F12 "TSIG" I R 5350 2250 60 
F13 "+3.3V" I L 4300 1300 60 
F14 "DGMODE" I L 4300 3500 60 
F15 "DSPV" I L 4300 3200 60 
F16 "DCKV" I L 4300 3300 60 
F17 "DSPH" I L 4300 3400 60 
F18 "ONPOS" I L 4300 1450 60 
F19 "ONNEG" I L 4300 1550 60 
F20 "DON" I L 4300 3100 60 
F21 "TON" I R 5350 2150 60 
F22 "DD7" I L 4300 2700 60 
$EndSheet
$Sheet
S 2750 1900 1100 1700
U 58E0C92D
F0 "E-Ink" 60
F1 "eink.sch" 60
F2 "+15V" I L 2750 2100 60 
F3 "-15V" I L 2750 2200 60 
F4 "+22V" I L 2750 2000 60 
F5 "-20V" I L 2750 2300 60 
F6 "DGMODE" I R 3850 3500 60 
F7 "DON" I R 3850 3100 60 
F8 "DSPV" I R 3850 3200 60 
F9 "DCKV" I R 3850 3300 60 
F10 "DCL" I R 3850 2800 60 
F11 "DLE" I R 3850 2900 60 
F12 "DOE" I R 3850 3000 60 
F13 "DSPH" I R 3850 3400 60 
F14 "DD0" I R 3850 2000 60 
F15 "DD1" I R 3850 2100 60 
F16 "DD2" I R 3850 2200 60 
F17 "DD3" I R 3850 2300 60 
F18 "DD4" I R 3850 2400 60 
F19 "DD5" I R 3850 2500 60 
F20 "DD6" I R 3850 2600 60 
F21 "DD7" I R 3850 2700 60 
$EndSheet
Wire Wire Line
	2300 1300 4300 1300
Wire Wire Line
	1050 1300 1450 1300
$Sheet
S 1450 1200 850  1200
U 58DA93AB
F0 "Voltage Sources" 60
F1 "voltage_sources.sch" 60
F2 "+3.3V" I R 2300 1300 60 
F3 "ONPOS" I R 2300 1450 60 
F4 "+BATT" I L 1450 1300 60 
F5 "ONNEG" I R 2300 1550 60 
F6 "+22V" I R 2300 2000 60 
F7 "+15V" I R 2300 2100 60 
F8 "-20V" I R 2300 2300 60 
F9 "-15V" I R 2300 2200 60 
$EndSheet
Wire Wire Line
	2300 1450 4300 1450
Wire Wire Line
	2300 1550 4300 1550
Wire Wire Line
	2300 2000 2750 2000
Wire Wire Line
	2300 2100 2750 2100
Wire Wire Line
	2300 2200 2750 2200
Wire Wire Line
	2300 2300 2750 2300
Wire Wire Line
	3850 2000 4300 2000
Wire Wire Line
	3850 2100 4300 2100
Wire Wire Line
	3850 2200 4300 2200
Wire Wire Line
	3850 2300 4300 2300
Wire Wire Line
	3850 2400 4300 2400
Wire Wire Line
	3850 2500 4300 2500
Wire Wire Line
	4300 2600 3850 2600
Wire Wire Line
	3850 2700 4300 2700
Wire Wire Line
	4300 2800 3850 2800
Wire Wire Line
	3850 2900 4300 2900
Wire Wire Line
	4300 3000 3850 3000
Wire Wire Line
	3850 3100 4300 3100
Wire Wire Line
	4300 3200 3850 3200
Wire Wire Line
	3850 3300 4300 3300
Wire Wire Line
	4300 3400 3850 3400
Wire Wire Line
	3850 3500 4300 3500
$Sheet
S 5700 2050 606  294 
U 58DE7EAB
F0 "DCF77" 60
F1 "dcf77.sch" 60
F2 "VCC" I L 5700 2150 60 
F3 "SIG" I L 5700 2250 60 
$EndSheet
Wire Wire Line
	5700 2150 5350 2150
Wire Wire Line
	5700 2250 5350 2250
$EndSCHEMATC
