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
Date "2017-05-08"
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
F 2 "einkclock:Reichelt_DCF77_Module" H 2400 1200 60  0001 C CNN
F 3 "" H 2400 1200 60  0001 C CNN
F 4 "DCF77 MODUL" H 2400 1050 60  0001 C CNN "Manf#"
	1    2400 1050
	1    0    0    -1  
$EndComp
$Comp
L GND #PWR021
U 1 1 58DE81D1
P 2850 2000
F 0 "#PWR021" H 2850 1750 50  0001 C CNN
F 1 "GND" H 2850 1850 50  0000 C CNN
F 2 "" H 2850 2000 50  0000 C CNN
F 3 "" H 2850 2000 50  0000 C CNN
	1    2850 2000
	1    0    0    -1  
$EndComp
$Comp
L C C30
U 1 1 58DE81D9
P 1750 1250
F 0 "C30" H 1775 1350 50  0000 L CNN
F 1 "100n" H 1775 1150 50  0000 L CNN
F 2 "Resistors_SMD:R_0805_HandSoldering" H 1788 1100 50  0001 C CNN
F 3 "" H 1750 1250 50  0000 C CNN
F 4 "885012207098" H 1750 1250 60  0001 C CNN "Manf#"
	1    1750 1250
	-1   0    0    1   
$EndComp
$Comp
L R R19
U 1 1 58DE81E9
P 1200 1000
F 0 "R19" V 1280 1000 50  0000 C CNN
F 1 "1k" V 1200 1000 50  0000 C CNN
F 2 "Resistors_SMD:R_0805_HandSoldering" V 1130 1000 50  0001 C CNN
F 3 "" H 1200 1000 50  0000 C CNN
F 4 "ERJ-6ENF1001V" V 1200 1000 60  0001 C CNN "Manf#"
	1    1200 1000
	0    1    1    0   
$EndComp
$Comp
L R R20
U 1 1 58DE81F0
P 2850 1750
F 0 "R20" V 2930 1750 50  0000 C CNN
F 1 "1k" V 2850 1750 50  0000 C CNN
F 2 "Resistors_SMD:R_0805_HandSoldering" V 2780 1750 50  0001 C CNN
F 3 "" H 2850 1750 50  0000 C CNN
F 4 "ERJ-6ENF1001V" V 2850 1750 60  0001 C CNN "Manf#"
	1    2850 1750
	-1   0    0    1   
$EndComp
Text HLabel 950  1000 0    60   Input ~ 0
VCC
Text HLabel 2850 1000 2    60   Input ~ 0
SIG
Wire Wire Line
	2750 1100 2850 1100
Wire Wire Line
	1350 1000 2050 1000
Wire Wire Line
	1950 1100 2050 1100
Wire Wire Line
	1450 1100 1450 1000
Connection ~ 1450 1000
Wire Wire Line
	1750 1100 1750 1000
Connection ~ 1750 1000
Wire Wire Line
	2750 1000 2850 1000
Wire Wire Line
	2850 1900 2850 2000
Wire Wire Line
	1450 1400 1450 1500
Wire Wire Line
	1750 1400 1750 1500
Connection ~ 1750 1500
Wire Wire Line
	2850 1100 2850 1600
Connection ~ 2850 1500
Wire Wire Line
	950  1000 1050 1000
Wire Wire Line
	1950 1500 1950 1100
Wire Wire Line
	1450 1500 2850 1500
Connection ~ 1950 1500
$Comp
L C C29
U 1 1 58E46F0E
P 1450 1250
F 0 "C29" H 1475 1350 50  0000 L CNN
F 1 "100µ" H 1475 1150 50  0000 L CNN
F 2 "Resistors_SMD:R_1206_HandSoldering" H 1488 1100 50  0001 C CNN
F 3 "" H 1450 1250 50  0000 C CNN
F 4 "GRM31CD80J107ME39K" H 1450 1250 60  0001 C CNN "Manf#"
	1    1450 1250
	-1   0    0    1   
$EndComp
$EndSCHEMATC
