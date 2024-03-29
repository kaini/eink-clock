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
Sheet 3 5
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
L ED060SC4V2 U4
U 1 1 58E0CC09
P 2900 5100
F 0 "U4" H 3600 4250 60  0000 R CNN
F 1 "ED060SC4V2" H 2900 5100 60  0000 C CNN
F 2 "einkclock:Hirose_FH26-39S-0.3SHW" H 3200 5250 60  0001 C CNN
F 3 "" H 3200 5250 60  0001 C CNN
F 4 "FH26W-39S-0.3SHW(98)" H 2900 5100 60  0001 C CNN "Manf#"
	1    2900 5100
	1    0    0    -1  
$EndComp
$Comp
L C C9
U 1 1 58E0CC2B
P 3050 2500
F 0 "C9" H 3075 2600 50  0000 L CNN
F 1 "100n" H 3075 2400 50  0000 L CNN
F 2 "Resistors_SMD:R_0805_HandSoldering" H 3088 2350 50  0001 C CNN
F 3 "" H 3050 2500 50  0000 C CNN
F 4 "885012207098" H 3050 2500 60  0001 C CNN "Manf#"
	1    3050 2500
	0    -1   1    0   
$EndComp
$Comp
L GND #PWR07
U 1 1 58E0CC32
P 3300 2600
F 0 "#PWR07" H 3300 2350 50  0001 C CNN
F 1 "GND" H 3300 2450 50  0000 C CNN
F 2 "" H 3300 2600 50  0000 C CNN
F 3 "" H 3300 2600 50  0000 C CNN
	1    3300 2600
	-1   0    0    -1  
$EndComp
$Comp
L R R4
U 1 1 58E0CC3A
P 1650 1950
F 0 "R4" V 1730 1950 50  0000 C CNN
F 1 "47k" V 1650 1950 50  0000 C CNN
F 2 "Resistors_SMD:R_0805_HandSoldering" V 1580 1950 50  0001 C CNN
F 3 "" H 1650 1950 50  0000 C CNN
F 4 "ERJ-6GEYJ473V" V 1650 1950 60  0001 C CNN "Manf#"
	1    1650 1950
	1    0    0    1   
$EndComp
$Comp
L POT RV1
U 1 1 58E0CC41
P 1650 2350
F 0 "RV1" V 1475 2350 50  0000 C CNN
F 1 "10k" V 1550 2350 50  0000 C CNN
F 2 "Potentiometers:Potentiometer_Trimmer_Vishay_TS53YL" H 1650 2350 50  0001 C CNN
F 3 "" H 1650 2350 50  0000 C CNN
F 4 "TS53YL103MR10" V 1650 2350 60  0001 C CNN "Manf#"
	1    1650 2350
	1    0    0    1   
$EndComp
$Comp
L GND #PWR08
U 1 1 58E0CC48
P 1650 2600
F 0 "#PWR08" H 1650 2350 50  0001 C CNN
F 1 "GND" H 1650 2450 50  0000 C CNN
F 2 "" H 1650 2600 50  0000 C CNN
F 3 "" H 1650 2600 50  0000 C CNN
	1    1650 2600
	-1   0    0    -1  
$EndComp
Wire Wire Line
	3700 4500 3800 4500
Wire Wire Line
	3700 4600 3800 4600
Wire Wire Line
	3700 4700 3800 4700
Wire Wire Line
	3700 4900 3800 4900
Wire Wire Line
	3700 5000 3800 5000
Wire Wire Line
	3700 5100 3800 5100
Wire Wire Line
	3700 5200 3800 5200
Wire Wire Line
	3700 5300 3800 5300
Wire Wire Line
	3700 5400 3800 5400
Wire Wire Line
	3700 5500 3800 5500
Wire Wire Line
	3700 5600 3800 5600
Wire Wire Line
	3700 5700 3800 5700
Wire Wire Line
	2000 4500 2100 4500
Wire Wire Line
	2600 2250 2800 2250
Wire Wire Line
	2800 2500 2900 2500
Wire Wire Line
	2200 2550 2200 2850
Wire Wire Line
	2200 1600 2200 1950
Wire Wire Line
	2800 1900 1900 1900
Wire Wire Line
	1900 1900 1900 2150
Wire Wire Line
	1900 2150 2000 2150
Wire Wire Line
	2200 1700 1650 1700
Wire Wire Line
	1650 1700 1650 1800
Connection ~ 2200 1700
Wire Wire Line
	1650 2100 1650 2200
Wire Wire Line
	1650 2500 1650 2600
Wire Wire Line
	1800 2350 2000 2350
Connection ~ 2800 2500
$Comp
L GND #PWR09
U 1 1 58E0CC71
P 2900 6500
F 0 "#PWR09" H 2900 6250 50  0001 C CNN
F 1 "GND" H 2900 6350 50  0000 C CNN
F 2 "" H 2900 6500 50  0000 C CNN
F 3 "" H 2900 6500 50  0000 C CNN
	1    2900 6500
	1    0    0    -1  
$EndComp
Wire Wire Line
	3700 4800 4150 4800
Wire Wire Line
	2000 4600 2100 4600
Wire Wire Line
	2000 4700 2100 4700
Wire Wire Line
	2000 4900 2100 4900
$Comp
L GND #PWR010
U 1 1 58E0CC80
P 1650 5000
F 0 "#PWR010" H 1650 4750 50  0001 C CNN
F 1 "GND" H 1650 4850 50  0000 C CNN
F 2 "" H 1650 5000 50  0000 C CNN
F 3 "" H 1650 5000 50  0000 C CNN
	1    1650 5000
	0    1    1    0   
$EndComp
Wire Wire Line
	1650 5000 2100 5000
Wire Wire Line
	2000 5100 2100 5100
Wire Wire Line
	2000 5200 2100 5200
$Comp
L GND #PWR011
U 1 1 58E0CC89
P 2700 4100
F 0 "#PWR011" H 2700 3850 50  0001 C CNN
F 1 "GND" H 2700 3950 50  0000 C CNN
F 2 "" H 2700 4100 50  0000 C CNN
F 3 "" H 2700 4100 50  0000 C CNN
	1    2700 4100
	-1   0    0    1   
$EndComp
Wire Wire Line
	2000 5400 2100 5400
Wire Wire Line
	2000 5500 2100 5500
Wire Wire Line
	2000 5600 2100 5600
Wire Wire Line
	2000 5700 2100 5700
Wire Wire Line
	3000 6000 3000 6100
Wire Wire Line
	2800 6000 2800 6100
Wire Wire Line
	2900 6500 2900 6000
Wire Wire Line
	2800 1900 2800 4200
Wire Wire Line
	2900 4100 2900 4200
Wire Wire Line
	3000 3550 3000 4200
Wire Wire Line
	3100 4100 3100 4200
Wire Wire Line
	3700 4400 3800 4400
Wire Wire Line
	3800 5800 3700 5800
$Comp
L GND #PWR012
U 1 1 58E0CCA1
P 4150 4800
F 0 "#PWR012" H 4150 4550 50  0001 C CNN
F 1 "GND" H 4150 4650 50  0000 C CNN
F 2 "" H 4150 4800 50  0000 C CNN
F 3 "" H 4150 4800 50  0000 C CNN
	1    4150 4800
	0    -1   -1   0   
$EndComp
$Comp
L C C10
U 1 1 58E0CCA8
P 3250 3700
F 0 "C10" H 3275 3800 50  0000 L CNN
F 1 "100n" H 3275 3600 50  0000 L CNN
F 2 "Resistors_SMD:R_0805_HandSoldering" H 3288 3550 50  0001 C CNN
F 3 "" H 3250 3700 50  0000 C CNN
F 4 "885012207098" H 3250 3700 60  0001 C CNN "Manf#"
	1    3250 3700
	0    -1   -1   0   
$EndComp
Wire Wire Line
	3000 3700 3100 3700
Text HLabel 1450 2850 0    60   Input ~ 0
+15V
Text HLabel 1450 1600 0    60   Input ~ 0
-15V
Text HLabel 2900 4100 1    60   Input ~ 0
+22V
Text HLabel 2800 6100 3    60   Input ~ 0
-20V
Wire Wire Line
	2200 2850 1450 2850
Wire Wire Line
	2200 1600 1450 1600
Wire Wire Line
	3200 2500 3300 2500
Wire Wire Line
	3300 2500 3300 2600
Connection ~ 2800 2250
Text HLabel 3100 4100 1    60   Input ~ 0
+15V
Text HLabel 3000 6100 3    60   Input ~ 0
-15V
Text HLabel 2000 4500 0    60   Input ~ 0
DGMODE
Text HLabel 2000 4600 0    60   Input ~ 0
DGMODE
Text HLabel 2000 4700 0    60   Input ~ 0
DON
Text HLabel 2000 4900 0    60   Input ~ 0
DON
Text HLabel 2000 5100 0    60   Input ~ 0
DSPV
Text HLabel 2000 5200 0    60   Input ~ 0
DCKV
Text HLabel 2000 5400 0    60   Input ~ 0
DON
Text HLabel 2000 5500 0    60   Input ~ 0
DON
Text HLabel 2000 5600 0    60   Input ~ 0
DSPV
Text HLabel 2000 5700 0    60   Input ~ 0
DCKV
Wire Wire Line
	2700 4200 2700 4100
Text HLabel 3000 3550 1    60   Input ~ 0
DON
Connection ~ 3000 3700
$Comp
L GND #PWR013
U 1 1 58E0F8B1
P 3500 3800
F 0 "#PWR013" H 3500 3550 50  0001 C CNN
F 1 "GND" H 3500 3650 50  0000 C CNN
F 2 "" H 3500 3800 50  0000 C CNN
F 3 "" H 3500 3800 50  0000 C CNN
	1    3500 3800
	1    0    0    -1  
$EndComp
Wire Wire Line
	3400 3700 3500 3700
Wire Wire Line
	3500 3700 3500 3800
Text HLabel 3800 4400 2    60   Input ~ 0
DCL
Text HLabel 3800 4500 2    60   Input ~ 0
DLE
Text HLabel 3800 4600 2    60   Input ~ 0
DOE
Text HLabel 3800 4700 2    60   Input ~ 0
DON
Text HLabel 3800 4900 2    60   Input ~ 0
DON
Text HLabel 3800 5000 2    60   Input ~ 0
DSPH
Text HLabel 3800 5100 2    60   Input ~ 0
DD0
Text HLabel 3800 5200 2    60   Input ~ 0
DD1
Text HLabel 3800 5300 2    60   Input ~ 0
DD2
Text HLabel 3800 5400 2    60   Input ~ 0
DD3
Text HLabel 3800 5500 2    60   Input ~ 0
DD4
Text HLabel 3800 5600 2    60   Input ~ 0
DD5
Text HLabel 3800 5700 2    60   Input ~ 0
DD6
Text HLabel 3800 5800 2    60   Input ~ 0
DD7
$Comp
L TS881 U3
U 1 1 58DE29E1
P 2300 2250
F 0 "U3" H 2300 2500 50  0000 L CNN
F 1 "TS321ILT" H 2300 2400 50  0000 L CNN
F 2 "TO_SOT_Packages_SMD:SOT-23-5_HandSoldering" H 2300 2250 50  0001 C CNN
F 3 "" H 2300 2250 50  0001 C CNN
F 4 "TS321ILT" H 2300 2250 60  0001 C CNN "Manf#"
	1    2300 2250
	1    0    0    1   
$EndComp
$EndSCHEMATC
