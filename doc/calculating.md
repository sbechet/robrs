https://codebase64.org/doku.php?id=base:how_to_calculate_your_own_sid_frequency_table

PAL_PHI = 985248;
NTSC_PHI = 1022727; //This is for machines with 6567R8 VIC. 6567R56A is slightly different.
CONSTANT = 256^(3) / PAL_PHI; //Select the constant appropriate for your machine (PAL vs NTSC).
SID_FREQ = CONSTANT * FREQ_HZ; //Calculate SID freq for a certain note (specified in Hz).
On a PAL machine: 17,02841924 * 440 = 7493 (or $1D45 in hex)

Calculating the (machine dependent) constant

You may wonder what is going on with that constant mentioned above, which differs between different types of machines. It is nothing less than the number of cycles in the machine in one second. It can (obviously) be calculated in this way if you don't know it already: 
lines_on_screen = 312;
cycles_per_line = 63;
framerate = 50,12454212;	//This is synonymous with "frames per second"
constant = lines_on_screen * cycles_per_line * framerate;

http://unusedino.de/ec64/technical/aay/c64/victypes.htm

```
 VIC PAL/NTSC Differences:

 Here are some specifications about the different VIC types. Please note that
 the 6567R56A was only used in very early C64s and is mentioned here only
 for completeness.

 +----------+--------+-------+---------+---------+---------+------------+
 |   VIC    | Video  | # of  | Cycles/ | Cycles/ | Frames/ | System     |
 |   Type   | system | lines |  line   | frame   | second  | Clock (Hz) |
 +----------+--------+-------+---------+---------+---------+------------+
 |   6569   |  PAL-B |  312  |   63    |  19656  | 50.125  |   985248   |
 |  6567R8  | NTSC-M |  263  |   65    |  17095  | 59.826  |  1022727   |
 | 6567R56A | NTSC-M |  262  |   64    |  16768  |   ?     |     ?      |
 +----------+--------+-------+---------+---------+---------+------------+

 +----------+---------+-------------+
 |   VIC    | Visible |  Visible    |
 |   Type   |  lines  | pixels/line |
 +----------+---------+-------------+
 |   6569   |   284   |     403     |
 |  6567R8  |   235   |     418     |
 | 6567R56A |   234   |     411     |
 +----------+---------+-------------+

 +----------+--------+--------+--------------+------------+------------+
 |   VIC    | First  |  Last  |    First     |   First    |    Last    |
 |   Type   | vblank | vblank |   X coord.   |  visible   |  visible   |
 |          |  line  |  line  |  of a line   |  X coord.  |  X coord.  |
 +----------+--------+--------+--------------+------------+------------+
 |   6569   |  300   |   15   |  404 ($194)  | 480 ($1e0) | 380 ($17c) |
 |  6567R8  |   13   |   40   |  412 ($19c)  | 489 ($1e9) | 396 ($18c) |
 | 6567R56A |   13   |   40   |  412 ($19c)  | 488 ($1e8) | 388 ($184) |
 +----------+--------+--------+--------------+------------+------------+
```
