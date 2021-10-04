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
Sheet 2 5
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
L LPC1227FBD48/301 U2
U 1 1 58DEBAB0
P 5500 3600
F 0 "U2" H 6100 1350 60  0000 R CNN
F 1 "LPC1227FBD48/301" H 5500 5850 60  0000 C CNN
F 2 "Housings_QFP:LQFP-48_7x7mm_Pitch0.5mm" H 5300 4850 60  0001 C CNN
F 3 "" H 5300 4850 60  0001 C CNN
F 4 "LPC1227FBD48/301,1" H 5500 3600 60  0001 C CNN "Manf#"
	1    5500 3600
	1    0    0    -1  
$EndComp
NoConn ~ 4800 5700
NoConn ~ 4800 5600
NoConn ~ 4800 5500
NoConn ~ 4800 5300
NoConn ~ 4800 5200
NoConn ~ 4800 5100
NoConn ~ 4800 4900
NoConn ~ 4800 4800
$Comp
L C C6
U 1 1 58DEBAD4
P 6700 3750
F 0 "C6" H 6725 3850 50  0000 L CNN
F 1 "100n" H 6725 3650 50  0000 L CNN
F 2 "Resistors_SMD:R_0805_HandSoldering" H 6738 3600 50  0001 C CNN
F 3 "" H 6700 3750 50  0000 C CNN
F 4 "885012207098" H 6700 3750 60  0001 C CNN "Manf#"
	1    6700 3750
	1    0    0    -1  
$EndComp
$Comp
L C C7
U 1 1 58DEBADB
P 6950 3750
F 0 "C7" H 6975 3850 50  0000 L CNN
F 1 "100n" H 6975 3650 50  0000 L CNN
F 2 "Resistors_SMD:R_0805_HandSoldering" H 6988 3600 50  0001 C CNN
F 3 "" H 6950 3750 50  0000 C CNN
F 4 "885012207098" H 6950 3750 60  0001 C CNN "Manf#"
	1    6950 3750
	1    0    0    -1  
$EndComp
$Comp
L GND #PWR02
U 1 1 58DEBAE8
P 7150 3500
F 0 "#PWR02" H 7150 3250 50  0001 C CNN
F 1 "GND" H 7150 3350 50  0000 C CNN
F 2 "" H 7150 3500 50  0000 C CNN
F 3 "" H 7150 3500 50  0000 C CNN
	1    7150 3500
	0    -1   -1   0   
$EndComp
$Comp
L Crystal Y1
U 1 1 58DEBAEE
P 6750 4300
F 0 "Y1" H 6750 4450 50  0000 C CNN
F 1 "32.768k 12.5p" H 6750 4150 50  0000 C CNN
F 2 "einkclock:Crystal_Lying_1" H 6750 4300 50  0001 C CNN
F 3 "" H 6750 4300 50  0000 C CNN
F 4 "AB26TRQ-32.768kHz-T" H 6750 4300 60  0001 C CNN "Manf#"
	1    6750 4300
	-1   0    0    -1  
$EndComp
$Comp
L C C5
U 1 1 58DEBAF5
P 6450 4700
F 0 "C5" H 6475 4800 50  0000 L CNN
F 1 "22p" H 6475 4600 50  0000 L CNN
F 2 "Resistors_SMD:R_0805_HandSoldering" H 6488 4550 50  0001 C CNN
F 3 "" H 6450 4700 50  0000 C CNN
F 4 "885012007053" H 6450 4700 60  0001 C CNN "Manf#"
	1    6450 4700
	1    0    0    -1  
$EndComp
$Comp
L C C8
U 1 1 58DEBAFC
P 7050 4700
F 0 "C8" H 7075 4800 50  0000 L CNN
F 1 "22p" H 7075 4600 50  0000 L CNN
F 2 "Resistors_SMD:R_0805_HandSoldering" H 7088 4550 50  0001 C CNN
F 3 "" H 7050 4700 50  0000 C CNN
F 4 "885012007053" H 7050 4700 60  0001 C CNN "Manf#"
	1    7050 4700
	1    0    0    -1  
$EndComp
$Comp
L GND #PWR03
U 1 1 58DEBB03
P 6750 5000
F 0 "#PWR03" H 6750 4750 50  0001 C CNN
F 1 "GND" H 6750 4850 50  0000 C CNN
F 2 "" H 6750 5000 50  0000 C CNN
F 3 "" H 6750 5000 50  0000 C CNN
	1    6750 5000
	1    0    0    -1  
$EndComp
$Comp
L CONN_02X05 P1
U 1 1 58DEBB09
P 3050 4500
F 0 "P1" H 3050 4800 50  0000 C CNN
F 1 "SWD" H 3050 4200 50  0000 C CNN
F 2 "Connectors:IDC_Header_Straight_10pins" H 3050 3300 50  0001 C CNN
F 3 "" H 3050 3300 50  0000 C CNN
F 4 "61201021621" H 3050 4500 60  0001 C CNN "Manf#"
	1    3050 4500
	1    0    0    -1  
$EndComp
$Comp
L R R1
U 1 1 58DEBB16
P 3550 2650
F 0 "R1" V 3630 2650 50  0000 C CNN
F 1 "10k" V 3550 2650 50  0000 C CNN
F 2 "Resistors_SMD:R_0805_HandSoldering" V 3480 2650 50  0001 C CNN
F 3 "" H 3550 2650 50  0000 C CNN
F 4 "AS08J1002ET" V 3550 2650 60  0001 C CNN "Manf#"
	1    3550 2650
	-1   0    0    1   
$EndComp
$Comp
L GND #PWR04
U 1 1 58DEBB1D
P 2450 5150
F 0 "#PWR04" H 2450 4900 50  0001 C CNN
F 1 "GND" H 2450 5000 50  0000 C CNN
F 2 "" H 2450 5150 50  0000 C CNN
F 3 "" H 2450 5150 50  0000 C CNN
	1    2450 5150
	1    0    0    -1  
$EndComp
NoConn ~ 3300 4600
$Comp
L C C1
U 1 1 58DEBB24
P 2250 4550
F 0 "C1" H 2275 4650 50  0000 L CNN
F 1 "100n" H 2275 4450 50  0000 L CNN
F 2 "Resistors_SMD:R_0805_HandSoldering" H 2288 4400 50  0001 C CNN
F 3 "" H 2250 4550 50  0000 C CNN
F 4 "885012207098" H 2250 4550 60  0001 C CNN "Manf#"
	1    2250 4550
	-1   0    0    1   
$EndComp
$Comp
L C C3
U 1 1 58DEBB2B
P 3550 3150
F 0 "C3" H 3575 3250 50  0000 L CNN
F 1 "100n" H 3575 3050 50  0000 L CNN
F 2 "Resistors_SMD:R_0805_HandSoldering" H 3588 3000 50  0001 C CNN
F 3 "" H 3550 3150 50  0000 C CNN
F 4 "885012207098" H 3550 3150 60  0001 C CNN "Manf#"
	1    3550 3150
	-1   0    0    1   
$EndComp
$Comp
L GND #PWR05
U 1 1 58DEBB38
P 3550 3400
F 0 "#PWR05" H 3550 3150 50  0001 C CNN
F 1 "GND" H 3550 3250 50  0000 C CNN
F 2 "" H 3550 3400 50  0000 C CNN
F 3 "" H 3550 3400 50  0000 C CNN
	1    3550 3400
	1    0    0    -1  
$EndComp
NoConn ~ 6200 3100
NoConn ~ 6200 3200
NoConn ~ 6200 3400
NoConn ~ 3300 4500
Wire Wire Line
	4800 1500 4700 1500
Wire Wire Line
	4800 1600 4700 1600
Wire Wire Line
	4800 1700 4700 1700
Wire Wire Line
	4800 1800 4700 1800
Wire Wire Line
	4800 1900 4700 1900
Wire Wire Line
	4800 2000 4700 2000
Wire Wire Line
	4800 2100 4700 2100
Wire Wire Line
	4800 2200 4700 2200
Wire Wire Line
	4700 3500 4800 3500
Wire Wire Line
	4800 2600 4700 2600
Wire Wire Line
	4700 2500 4800 2500
Wire Wire Line
	4800 2400 4700 2400
Wire Wire Line
	6300 3700 6200 3700
Wire Wire Line
	6300 3500 6300 3700
Wire Wire Line
	6300 3600 6200 3600
Wire Wire Line
	6200 3900 6300 3900
Wire Wire Line
	6300 3800 6300 4000
Wire Wire Line
	6300 3800 6200 3800
Wire Wire Line
	6950 4000 6950 3900
Wire Wire Line
	6300 4000 7150 4000
Connection ~ 6300 3900
Wire Wire Line
	6700 3900 6700 4000
Connection ~ 6700 4000
Wire Wire Line
	6450 3900 6450 4000
Connection ~ 6450 4000
Wire Wire Line
	6950 3500 6950 3600
Wire Wire Line
	6300 3500 7150 3500
Connection ~ 6300 3600
Wire Wire Line
	6450 3600 6450 3500
Connection ~ 6450 3500
Wire Wire Line
	6700 3600 6700 3500
Connection ~ 6700 3500
Connection ~ 6950 3500
Connection ~ 6950 4000
Wire Wire Line
	6450 4200 6450 4550
Wire Wire Line
	6450 4300 6600 4300
Wire Wire Line
	6900 4300 7050 4300
Wire Wire Line
	7050 4100 7050 4550
Wire Wire Line
	6450 4200 6200 4200
Connection ~ 6450 4300
Wire Wire Line
	6200 4100 7050 4100
Connection ~ 7050 4300
Wire Wire Line
	7050 4950 7050 4850
Wire Wire Line
	6450 4950 7050 4950
Wire Wire Line
	6450 4850 6450 4950
Wire Wire Line
	6750 5000 6750 4950
Connection ~ 6750 4950
Wire Wire Line
	2150 4300 2800 4300
Wire Wire Line
	3300 4300 4800 4300
Wire Wire Line
	3300 4400 4800 4400
Wire Wire Line
	4700 2700 4800 2700
Wire Wire Line
	4700 3600 4800 3600
Wire Wire Line
	4700 4700 4800 4700
Wire Wire Line
	4700 4600 4800 4600
$Comp
L R R2
U 1 1 58DEBBA3
P 3800 4800
F 0 "R2" V 3880 4800 50  0000 C CNN
F 1 "10k" V 3800 4800 50  0000 C CNN
F 2 "Resistors_SMD:R_0805_HandSoldering" V 3730 4800 50  0001 C CNN
F 3 "" H 3800 4800 50  0000 C CNN
F 4 "AS08J1002ET" V 3800 4800 60  0001 C CNN "Manf#"
	1    3800 4800
	-1   0    0    1   
$EndComp
$Comp
L R R3
U 1 1 58DEBBAA
P 4000 4800
F 0 "R3" V 4080 4800 50  0000 C CNN
F 1 "10k" V 4000 4800 50  0000 C CNN
F 2 "Resistors_SMD:R_0805_HandSoldering" V 3930 4800 50  0001 C CNN
F 3 "" H 4000 4800 50  0000 C CNN
F 4 "AS08J1002ET" V 4000 4800 60  0001 C CNN "Manf#"
	1    4000 4800
	-1   0    0    1   
$EndComp
Wire Wire Line
	4700 3000 4800 3000
Wire Wire Line
	4700 3100 4800 3100
Wire Wire Line
	4700 3300 4800 3300
Wire Wire Line
	4700 3400 4800 3400
NoConn ~ 4800 4200
$Comp
L C C4
U 1 1 58DEBBC8
P 6450 3750
F 0 "C4" H 6475 3850 50  0000 L CNN
F 1 "4.7µ" H 6475 3650 50  0000 L CNN
F 2 "Resistors_SMD:R_0805_HandSoldering" H 6488 3600 50  0001 C CNN
F 3 "" H 6450 3750 50  0000 C CNN
F 4 "GRM21BR61E475MA12L" H 6450 3750 60  0001 C CNN "Manf#"
	1    6450 3750
	1    0    0    -1  
$EndComp
Text HLabel 4700 1500 0    60   Input ~ 0
DD0
Text HLabel 4700 1600 0    60   Input ~ 0
DD1
Text HLabel 4700 1700 0    60   Input ~ 0
DD2
Text HLabel 4700 1800 0    60   Input ~ 0
DD3
Text HLabel 4700 1900 0    60   Input ~ 0
DD4
Text HLabel 4700 2000 0    60   Input ~ 0
DD5
Text HLabel 4700 2100 0    60   Input ~ 0
DD6
Text HLabel 4700 2200 0    60   Input ~ 0
DD7
Text HLabel 4700 3700 0    60   Input ~ 0
DCL
Text HLabel 4700 3800 0    60   Input ~ 0
DLE
Text HLabel 4700 3900 0    60   Input ~ 0
DOE
Text HLabel 4700 2700 0    60   Input ~ 0
TSIG
Text HLabel 2150 4300 0    60   Input ~ 0
+3.3V
Text HLabel 7150 4000 2    60   Input ~ 0
+3.3V
Text HLabel 4700 2400 0    60   Input ~ 0
DGMODE
Text HLabel 4700 2500 0    60   Input ~ 0
DSPV
Text HLabel 4700 2600 0    60   Input ~ 0
DCKV
Text HLabel 4700 4000 0    60   Input ~ 0
DSPH
Text HLabel 4700 3500 0    60   Input ~ 0
ONPOS
Text HLabel 4700 3600 0    60   Input ~ 0
ONNEG
Text HLabel 4700 4700 0    60   Input ~ 0
DON
Text HLabel 4700 4600 0    60   Input ~ 0
TON
Wire Wire Line
	3550 3400 3550 3300
Text HLabel 3450 2400 0    60   Input ~ 0
+3.3V
Wire Wire Line
	3450 2400 3550 2400
Wire Wire Line
	3550 2400 3550 2500
Text Label 3450 2900 2    60   ~ 0
RESET
Wire Wire Line
	3550 2800 3550 3000
Connection ~ 3550 2900
Wire Wire Line
	3450 2900 4800 2900
Text Label 4700 5400 2    60   ~ 0
RESET
Wire Wire Line
	4700 5400 4800 5400
Text Label 3400 4700 0    60   ~ 0
RESET
Wire Wire Line
	3400 4700 3300 4700
Wire Wire Line
	2250 4300 2250 4400
Wire Wire Line
	2250 4700 2250 5050
Connection ~ 2250 4300
Text Label 4300 2800 2    60   ~ 0
ISP
Wire Wire Line
	4300 2800 4800 2800
Text Label 2700 4600 2    60   ~ 0
ISP
Wire Wire Line
	2700 4600 2800 4600
Wire Wire Line
	2800 4400 2450 4400
Wire Wire Line
	2450 4400 2450 5150
Wire Wire Line
	2800 4500 2450 4500
Connection ~ 2450 4500
Wire Wire Line
	2450 4700 2800 4700
Connection ~ 2450 4700
Wire Wire Line
	3800 4650 3800 4400
Connection ~ 3800 4400
Wire Wire Line
	4000 4650 4000 4300
Connection ~ 4000 4300
Wire Wire Line
	4000 5050 4000 4950
Wire Wire Line
	2250 5050 4000 5050
Wire Wire Line
	3800 4950 3800 5050
Connection ~ 3800 5050
Connection ~ 2450 5050
Wire Wire Line
	4800 4000 4700 4000
Wire Wire Line
	4800 3900 4700 3900
Wire Wire Line
	4800 3800 4700 3800
Wire Wire Line
	4800 3700 4700 3700
Text Label 4700 3000 2    60   ~ 0
SCK
Text Label 4700 3100 2    60   ~ 0
SSEL
Text Label 4700 3400 2    60   ~ 0
MOSI
Text Label 4700 3300 2    60   ~ 0
MISO
Text Label 4700 4500 2    60   ~ 0
RAMON
Wire Wire Line
	4700 4500 4800 4500
Text Label 3350 1050 0    60   ~ 0
RAMON
Text Label 2850 1350 0    60   ~ 0
MOSI
Text Label 1450 1150 2    60   ~ 0
MISO
Text Label 1450 1050 2    60   ~ 0
SSEL
Text Label 2850 1250 0    60   ~ 0
SCK
$Comp
L GND #PWR06
U 1 1 58DE0BD2
P 3350 1700
F 0 "#PWR06" H 3350 1450 50  0001 C CNN
F 1 "GND" H 3350 1550 50  0000 C CNN
F 2 "" H 3350 1700 50  0000 C CNN
F 3 "" H 3350 1700 50  0000 C CNN
	1    3350 1700
	1    0    0    -1  
$EndComp
Wire Wire Line
	1550 1350 1450 1350
Wire Wire Line
	1450 1250 1450 1600
Wire Wire Line
	1450 1600 3350 1600
Wire Wire Line
	1550 1250 1450 1250
Connection ~ 1450 1350
Wire Wire Line
	2750 1050 3350 1050
Wire Wire Line
	2850 1050 2850 1150
Wire Wire Line
	2850 1150 2750 1150
Connection ~ 2850 1050
$Comp
L C C2
U 1 1 58DE1215
P 3250 1300
F 0 "C2" H 3275 1400 50  0000 L CNN
F 1 "100n" H 3275 1200 50  0000 L CNN
F 2 "Resistors_SMD:R_0805_HandSoldering" H 3288 1150 50  0001 C CNN
F 3 "" H 3250 1300 50  0001 C CNN
F 4 "885012207098" H 3250 1300 60  0001 C CNN "Manf#"
	1    3250 1300
	1    0    0    -1  
$EndComp
Wire Wire Line
	3350 1600 3350 1700
Wire Wire Line
	3250 1600 3250 1450
Connection ~ 3250 1600
Wire Wire Line
	3250 1150 3250 1050
Connection ~ 3250 1050
Wire Wire Line
	1450 1050 1550 1050
Wire Wire Line
	2850 1250 2750 1250
Wire Wire Line
	2850 1350 2750 1350
Wire Wire Line
	1450 1150 1550 1150
$Comp
L 23LC512 U1
U 1 1 58DE78DA
P 2150 1200
F 0 "U1" H 2650 900 60  0000 R CNN
F 1 "23LC512" H 2150 1500 60  0000 C CNN
F 2 "Housings_SOIC:SOIC-8_3.9x4.9mm_Pitch1.27mm" H 2150 1200 60  0001 C CNN
F 3 "" H 2150 1200 60  0001 C CNN
F 4 "23LC512-I/SN" H 2150 1200 60  0001 C CNN "Manf#"
	1    2150 1200
	1    0    0    -1  
$EndComp
$EndSCHEMATC
