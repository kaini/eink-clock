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
Sheet 5 5
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
L DCF77_Generic U10
U 1 1 58DE81CA
P 2400 1050
F 0 "U10" H 2650 850 60  0000 R CNN
F 1 "DCF77_Generic" H 2400 1250 60  0000 C CNN
F 2 "" H 2400 1200 60  0001 C CNN
F 3 "" H 2400 1200 60  0001 C CNN
	1    2400 1050
	1    0    0    -1  
$EndComp
$Comp
L GND #PWR21
U 1 1 58DE81D1
P 2850 2000
F 0 "#PWR21" H 2850 1750 50  0001 C CNN
F 1 "GND" H 2850 1850 50  0000 C CNN
F 2 "" H 2850 2000 50  0000 C CNN
F 3 "" H 2850 2000 50  0000 C CNN
	1    2850 2000
	1    0    0    -1  
$EndComp
$Comp
L C C30
U 1 1 58DE81D9
P 1850 1250
F 0 "C30" H 1875 1350 50  0000 L CNN
F 1 "100n" H 1875 1150 50  0000 L CNN
F 2 "" H 1888 1100 50  0001 C CNN
F 3 "" H 1850 1250 50  0000 C CNN
	1    1850 1250
	-1   0    0    1   
$EndComp
$Comp
L CP C29
U 1 1 58DE81E0
P 1550 1250
F 0 "C29" H 1575 1350 50  0000 L CNN
F 1 "100µ" H 1575 1150 50  0000 L CNN
F 2 "" H 1588 1100 50  0001 C CNN
F 3 "" H 1550 1250 50  0000 C CNN
	1    1550 1250
	-1   0    0    1   
$EndComp
$Comp
L R R19
U 1 1 58DE81E9
P 1300 1000
F 0 "R19" V 1380 1000 50  0000 C CNN
F 1 "1k" V 1300 1000 50  0000 C CNN
F 2 "" V 1230 1000 50  0001 C CNN
F 3 "" H 1300 1000 50  0000 C CNN
	1    1300 1000
	0    1    1    0   
$EndComp
$Comp
L R R20
U 1 1 58DE81F0
P 2850 1750
F 0 "R20" V 2930 1750 50  0000 C CNN
F 1 "1k" V 2850 1750 50  0000 C CNN
F 2 "" V 2780 1750 50  0001 C CNN
F 3 "" H 2850 1750 50  0000 C CNN
	1    2850 1750
	-1   0    0    1   
$EndComp
Text HLabel 1050 1000 0    60   Input ~ 0
VCC
Text HLabel 2850 1000 2    60   Input ~ 0
SIG
Wire Wire Line
	2750 1100 2850 1100
Wire Wire Line
	1450 1000 2050 1000
Wire Wire Line
	1950 1000 1950 1100
Wire Wire Line
	1950 1100 2050 1100
Connection ~ 1950 1000
Wire Wire Line
	1550 1100 1550 1000
Connection ~ 1550 1000
Wire Wire Line
	1850 1100 1850 1000
Connection ~ 1850 1000
Wire Wire Line
	2750 1000 2850 1000
Wire Wire Line
	2850 1900 2850 2000
Wire Wire Line
	1550 1400 1550 1500
Wire Wire Line
	1550 1500 2850 1500
Wire Wire Line
	1850 1400 1850 1500
Connection ~ 1850 1500
Wire Wire Line
	2850 1100 2850 1600
Connection ~ 2850 1500
Wire Wire Line
	1050 1000 1150 1000
$EndSCHEMATC
