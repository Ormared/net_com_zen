/// 0
pub type U0 = UTerm;
/// 0
pub type Ux0 = UTerm;
/// 0
pub type Ub0 = UTerm;
/// 1
pub type U1 = UInt<UTerm, B1>;
/// 1
pub type Ux1 = UInt<UTerm, B1>;
/// 1
pub type Ub1 = UInt<UTerm, B1>;
/// 2
pub type U2 = UInt<UInt<UTerm, B1>, B0>;
/// 2
pub type Ux2 = UInt<UInt<UTerm, B1>, B0>;
/// 2
pub type Ub10 = UInt<UInt<UTerm, B1>, B0>;
/// 3
pub type U3 = UInt<UInt<UTerm, B1>, B1>;
/// 3
pub type Ux3 = UInt<UInt<UTerm, B1>, B1>;
/// 3
pub type Ub11 = UInt<UInt<UTerm, B1>, B1>;
/// 4
pub type U4 = UInt<UInt<UInt<UTerm, B1>, B0>, B0>;
/// 4
pub type Ux4 = UInt<UInt<UInt<UTerm, B1>, B0>, B0>;
/// 4
pub type Ub100 = UInt<UInt<UInt<UTerm, B1>, B0>, B0>;
/// 5
pub type U5 = UInt<UInt<UInt<UTerm, B1>, B0>, B1>;
/// 5
pub type Ux5 = UInt<UInt<UInt<UTerm, B1>, B0>, B1>;
/// 5
pub type Ub101 = UInt<UInt<UInt<UTerm, B1>, B0>, B1>;
/// 6
pub type U6 = UInt<UInt<UInt<UTerm, B1>, B1>, B0>;
/// 6
pub type Ux6 = UInt<UInt<UInt<UTerm, B1>, B1>, B0>;
/// 6
pub type Ub110 = UInt<UInt<UInt<UTerm, B1>, B1>, B0>;
/// 7
pub type U7 = UInt<UInt<UInt<UTerm, B1>, B1>, B1>;
/// 7
pub type Ux7 = UInt<UInt<UInt<UTerm, B1>, B1>, B1>;
/// 7
pub type Ub111 = UInt<UInt<UInt<UTerm, B1>, B1>, B1>;
/// 8
pub type U8 = UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>;
/// 8
pub type Ux8 = UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>;
/// 8
pub type Ub1000 = UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>;
/// 9
pub type U9 = UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>;
/// 9
pub type Ux9 = UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>;
/// 9
pub type Ub1001 = UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>;
/// 10
pub type U10 = UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>;
/// 10
pub type UxA = UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>;
/// 10
pub type Ub1010 = UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>;
/// 11
pub type U11 = UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>;
/// 11
pub type UxB = UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>;
/// 11
pub type Ub1011 = UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>;
/// 12
pub type U12 = UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>;
/// 12
pub type UxC = UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>;
/// 12
pub type Ub1100 = UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>;
/// 13
pub type U13 = UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>;
/// 13
pub type UxD = UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>;
/// 13
pub type Ub1101 = UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>;
/// 14
pub type U14 = UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>;
/// 14
pub type UxE = UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>;
/// 14
pub type Ub1110 = UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>;
/// 15
pub type U15 = UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>;
/// 15
pub type UxF = UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>;
/// 15
pub type Ub1111 = UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>;
/// 16
pub type U16 = UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>;
/// 16
pub type Ux10 = UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>;
/// 16
pub type Ub10000 = UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>;
/// 17
pub type U17 = UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>;
/// 17
pub type Ux11 = UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>;
/// 17
pub type Ub10001 = UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>;
/// 18
pub type U18 = UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>;
/// 18
pub type Ux12 = UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>;
/// 18
pub type Ub10010 = UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>;
/// 19
pub type U19 = UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>;
/// 19
pub type Ux13 = UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>;
/// 19
pub type Ub10011 = UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>;
/// 20
pub type U20 = UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>;
/// 20
pub type Ux14 = UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>;
/// 20
pub type Ub10100 = UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>;
/// 21
pub type U21 = UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>;
/// 21
pub type Ux15 = UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>;
/// 21
pub type Ub10101 = UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>;
/// 22
pub type U22 = UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>;
/// 22
pub type Ux16 = UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>;
/// 22
pub type Ub10110 = UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>;
/// 23
pub type U23 = UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>;
/// 23
pub type Ux17 = UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>;
/// 23
pub type Ub10111 = UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>;
/// 24
pub type U24 = UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>;
/// 24
pub type Ux18 = UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>;
/// 24
pub type Ub11000 = UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>;
/// 25
pub type U25 = UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>;
/// 25
pub type Ux19 = UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>;
/// 25
pub type Ub11001 = UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>;
/// 26
pub type U26 = UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>;
/// 26
pub type Ux1A = UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>;
/// 26
pub type Ub11010 = UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>;
/// 27
pub type U27 = UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>;
/// 27
pub type Ux1B = UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>;
/// 27
pub type Ub11011 = UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>;
/// 28
pub type U28 = UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>;
/// 28
pub type Ux1C = UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>;
/// 28
pub type Ub11100 = UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>;
/// 29
pub type U29 = UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>;
/// 29
pub type Ux1D = UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>;
/// 29
pub type Ub11101 = UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>;
/// 30
pub type U30 = UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>;
/// 30
pub type Ux1E = UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>;
/// 30
pub type Ub11110 = UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>;
/// 31
pub type U31 = UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>;
/// 31
pub type Ux1F = UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>;
/// 31
pub type Ub11111 = UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>;
/// 32
pub type U32 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>;
/// 32
pub type Ux20 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>;
/// 32
pub type Ub100000 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>;
/// 33
pub type U33 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>;
/// 33
pub type Ux21 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>;
/// 33
pub type Ub100001 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>;
/// 34
pub type U34 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>;
/// 34
pub type Ux22 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>;
/// 34
pub type Ub100010 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>;
/// 35
pub type U35 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>;
/// 35
pub type Ux23 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>;
/// 35
pub type Ub100011 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>;
/// 36
pub type U36 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>;
/// 36
pub type Ux24 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>;
/// 36
pub type Ub100100 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>;
/// 37
pub type U37 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>;
/// 37
pub type Ux25 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>;
/// 37
pub type Ub100101 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>;
/// 38
pub type U38 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>;
/// 38
pub type Ux26 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>;
/// 38
pub type Ub100110 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>;
/// 39
pub type U39 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>;
/// 39
pub type Ux27 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>;
/// 39
pub type Ub100111 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>;
/// 40
pub type U40 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>;
/// 40
pub type Ux28 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>;
/// 40
pub type Ub101000 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>;
/// 41
pub type U41 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>;
/// 41
pub type Ux29 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>;
/// 41
pub type Ub101001 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>;
/// 42
pub type U42 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>;
/// 42
pub type Ux2A = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>;
/// 42
pub type Ub101010 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>;
/// 43
pub type U43 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>;
/// 43
pub type Ux2B = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>;
/// 43
pub type Ub101011 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>;
/// 44
pub type U44 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>;
/// 44
pub type Ux2C = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>;
/// 44
pub type Ub101100 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>;
/// 45
pub type U45 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>;
/// 45
pub type Ux2D = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>;
/// 45
pub type Ub101101 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>;
/// 46
pub type U46 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>;
/// 46
pub type Ux2E = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>;
/// 46
pub type Ub101110 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>;
/// 47
pub type U47 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>;
/// 47
pub type Ux2F = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>;
/// 47
pub type Ub101111 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>;
/// 48
pub type U48 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>;
/// 48
pub type Ux30 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>;
/// 48
pub type Ub110000 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>;
/// 49
pub type U49 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>;
/// 49
pub type Ux31 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>;
/// 49
pub type Ub110001 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>;
/// 50
pub type U50 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>;
/// 50
pub type Ux32 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>;
/// 50
pub type Ub110010 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>;
/// 51
pub type U51 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>;
/// 51
pub type Ux33 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>;
/// 51
pub type Ub110011 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>;
/// 52
pub type U52 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>;
/// 52
pub type Ux34 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>;
/// 52
pub type Ub110100 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>;
/// 53
pub type U53 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>;
/// 53
pub type Ux35 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>;
/// 53
pub type Ub110101 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>;
/// 54
pub type U54 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>;
/// 54
pub type Ux36 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>;
/// 54
pub type Ub110110 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>;
/// 55
pub type U55 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>;
/// 55
pub type Ux37 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>;
/// 55
pub type Ub110111 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>;
/// 56
pub type U56 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>;
/// 56
pub type Ux38 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>;
/// 56
pub type Ub111000 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>;
/// 57
pub type U57 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>;
/// 57
pub type Ux39 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>;
/// 57
pub type Ub111001 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>;
/// 58
pub type U58 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>;
/// 58
pub type Ux3A = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>;
/// 58
pub type Ub111010 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>;
/// 59
pub type U59 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>;
/// 59
pub type Ux3B = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>;
/// 59
pub type Ub111011 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>;
/// 60
pub type U60 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>;
/// 60
pub type Ux3C = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>;
/// 60
pub type Ub111100 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>;
/// 61
pub type U61 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>;
/// 61
pub type Ux3D = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>;
/// 61
pub type Ub111101 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>;
/// 62
pub type U62 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B0>;
/// 62
pub type Ux3E = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B0>;
/// 62
pub type Ub111110 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B0>;
/// 63
pub type U63 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B1>;
/// 63
pub type Ux3F = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B1>;
/// 63
pub type Ub111111 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B1>;
/// 64
pub type U64 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 64
pub type Ux40 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 64
pub type Ub1000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 65
pub type U65 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B1>;
/// 65
pub type Ux41 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B1>;
/// 65
pub type Ub1000001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B1>;
/// 66
pub type U66 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B0>;
/// 66
pub type Ux42 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B0>;
/// 66
pub type Ub1000010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B0>;
/// 67
pub type U67 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B1>;
/// 67
pub type Ux43 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B1>;
/// 67
pub type Ub1000011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B1>;
/// 68
pub type U68 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B0>;
/// 68
pub type Ux44 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B0>;
/// 68
pub type Ub1000100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B0>;
/// 69
pub type U69 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B1>;
/// 69
pub type Ux45 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B1>;
/// 69
pub type Ub1000101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B1>;
/// 70
pub type U70 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B0>;
/// 70
pub type Ux46 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B0>;
/// 70
pub type Ub1000110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B0>;
/// 71
pub type U71 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B1>;
/// 71
pub type Ux47 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B1>;
/// 71
pub type Ub1000111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B1>;
/// 72
pub type U72 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B0>;
/// 72
pub type Ux48 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B0>;
/// 72
pub type Ub1001000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B0>;
/// 73
pub type U73 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B1>;
/// 73
pub type Ux49 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B1>;
/// 73
pub type Ub1001001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B1>;
/// 74
pub type U74 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B0>;
/// 74
pub type Ux4A = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B0>;
/// 74
pub type Ub1001010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B0>;
/// 75
pub type U75 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B1>;
/// 75
pub type Ux4B = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B1>;
/// 75
pub type Ub1001011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B1>;
/// 76
pub type U76 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B0>;
/// 76
pub type Ux4C = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B0>;
/// 76
pub type Ub1001100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B0>;
/// 77
pub type U77 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B1>;
/// 77
pub type Ux4D = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B1>;
/// 77
pub type Ub1001101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B1>;
/// 78
pub type U78 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B0>;
/// 78
pub type Ux4E = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B0>;
/// 78
pub type Ub1001110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B0>;
/// 79
pub type U79 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B1>;
/// 79
pub type Ux4F = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B1>;
/// 79
pub type Ub1001111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B1>;
/// 80
pub type U80 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B0>;
/// 80
pub type Ux50 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B0>;
/// 80
pub type Ub1010000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B0>;
/// 81
pub type U81 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B1>;
/// 81
pub type Ux51 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B1>;
/// 81
pub type Ub1010001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B1>;
/// 82
pub type U82 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B0>;
/// 82
pub type Ux52 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B0>;
/// 82
pub type Ub1010010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B0>;
/// 83
pub type U83 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B1>;
/// 83
pub type Ux53 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B1>;
/// 83
pub type Ub1010011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B1>;
/// 84
pub type U84 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B0>;
/// 84
pub type Ux54 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B0>;
/// 84
pub type Ub1010100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B0>;
/// 85
pub type U85 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B1>;
/// 85
pub type Ux55 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B1>;
/// 85
pub type Ub1010101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B1>;
/// 86
pub type U86 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B0>;
/// 86
pub type Ux56 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B0>;
/// 86
pub type Ub1010110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B0>;
/// 87
pub type U87 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B1>;
/// 87
pub type Ux57 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B1>;
/// 87
pub type Ub1010111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B1>;
/// 88
pub type U88 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B0>;
/// 88
pub type Ux58 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B0>;
/// 88
pub type Ub1011000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B0>;
/// 89
pub type U89 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B1>;
/// 89
pub type Ux59 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B1>;
/// 89
pub type Ub1011001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B1>;
/// 90
pub type U90 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B0>;
/// 90
pub type Ux5A = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B0>;
/// 90
pub type Ub1011010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B0>;
/// 91
pub type U91 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B1>;
/// 91
pub type Ux5B = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B1>;
/// 91
pub type Ub1011011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B1>;
/// 92
pub type U92 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B0>;
/// 92
pub type Ux5C = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B0>;
/// 92
pub type Ub1011100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B0>;
/// 93
pub type U93 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B1>;
/// 93
pub type Ux5D = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B1>;
/// 93
pub type Ub1011101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B1>;
/// 94
pub type U94 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B0>;
/// 94
pub type Ux5E = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B0>;
/// 94
pub type Ub1011110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B0>;
/// 95
pub type U95 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B1>;
/// 95
pub type Ux5F = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B1>;
/// 95
pub type Ub1011111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B1>;
/// 96
pub type U96 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B0>;
/// 96
pub type Ux60 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B0>;
/// 96
pub type Ub1100000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B0>;
/// 97
pub type U97 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B1>;
/// 97
pub type Ux61 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B1>;
/// 97
pub type Ub1100001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B1>;
/// 98
pub type U98 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B0>;
/// 98
pub type Ux62 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B0>;
/// 98
pub type Ub1100010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B0>;
/// 99
pub type U99 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B1>;
/// 99
pub type Ux63 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B1>;
/// 99
pub type Ub1100011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B1>;
/// 100
pub type U100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B0>;
/// 100
pub type Ux64 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B0>;
/// 100
pub type Ub1100100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B0>;
/// 101
pub type U101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B1>;
/// 101
pub type Ux65 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B1>;
/// 101
pub type Ub1100101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B1>;
/// 102
pub type U102 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B0>;
/// 102
pub type Ux66 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B0>;
/// 102
pub type Ub1100110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B0>;
/// 103
pub type U103 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B1>;
/// 103
pub type Ux67 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B1>;
/// 103
pub type Ub1100111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B1>;
/// 104
pub type U104 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B0>;
/// 104
pub type Ux68 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B0>;
/// 104
pub type Ub1101000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B0>;
/// 105
pub type U105 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B1>;
/// 105
pub type Ux69 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B1>;
/// 105
pub type Ub1101001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B1>;
/// 106
pub type U106 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B0>;
/// 106
pub type Ux6A = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B0>;
/// 106
pub type Ub1101010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B0>;
/// 107
pub type U107 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B1>;
/// 107
pub type Ux6B = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B1>;
/// 107
pub type Ub1101011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B1>;
/// 108
pub type U108 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B0>;
/// 108
pub type Ux6C = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B0>;
/// 108
pub type Ub1101100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B0>;
/// 109
pub type U109 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B1>;
/// 109
pub type Ux6D = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B1>;
/// 109
pub type Ub1101101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B1>;
/// 110
pub type U110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B0>;
/// 110
pub type Ux6E = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B0>;
/// 110
pub type Ub1101110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B0>;
/// 111
pub type U111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B1>;
/// 111
pub type Ux6F = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B1>;
/// 111
pub type Ub1101111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B1>;
/// 112
pub type U112 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B0>;
/// 112
pub type Ux70 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B0>;
/// 112
pub type Ub1110000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B0>;
/// 113
pub type U113 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B1>;
/// 113
pub type Ux71 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B1>;
/// 113
pub type Ub1110001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B1>;
/// 114
pub type U114 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B0>;
/// 114
pub type Ux72 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B0>;
/// 114
pub type Ub1110010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B0>;
/// 115
pub type U115 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B1>;
/// 115
pub type Ux73 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B1>;
/// 115
pub type Ub1110011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B1>;
/// 116
pub type U116 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B0>;
/// 116
pub type Ux74 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B0>;
/// 116
pub type Ub1110100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B0>;
/// 117
pub type U117 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B1>;
/// 117
pub type Ux75 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B1>;
/// 117
pub type Ub1110101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B1>;
/// 118
pub type U118 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B0>;
/// 118
pub type Ux76 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B0>;
/// 118
pub type Ub1110110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B0>;
/// 119
pub type U119 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B1>;
/// 119
pub type Ux77 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B1>;
/// 119
pub type Ub1110111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B1>;
/// 120
pub type U120 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B0>;
/// 120
pub type Ux78 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B0>;
/// 120
pub type Ub1111000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B0>;
/// 121
pub type U121 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B1>;
/// 121
pub type Ux79 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B1>;
/// 121
pub type Ub1111001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B1>;
/// 122
pub type U122 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B0>;
/// 122
pub type Ux7A = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B0>;
/// 122
pub type Ub1111010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B0>;
/// 123
pub type U123 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B1>;
/// 123
pub type Ux7B = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B1>;
/// 123
pub type Ub1111011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B1>;
/// 124
pub type U124 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B0>, B0>;
/// 124
pub type Ux7C = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B0>, B0>;
/// 124
pub type Ub1111100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B0>, B0>;
/// 125
pub type U125 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B0>, B1>;
/// 125
pub type Ux7D = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B0>, B1>;
/// 125
pub type Ub1111101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B0>, B1>;
/// 126
pub type U126 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B1>, B0>;
/// 126
pub type Ux7E = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B1>, B0>;
/// 126
pub type Ub1111110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B1>, B0>;
/// 127
pub type U127 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B1>, B1>;
/// 127
pub type Ux7F = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B1>, B1>;
/// 127
pub type Ub1111111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B1>, B1>;
/// 128
pub type U128 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 128
pub type Ux80 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 128
pub type Ub10000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 129
pub type U129 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B1>;
/// 129
pub type Ux81 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B1>;
/// 129
pub type Ub10000001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B1>;
/// 130
pub type U130 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B0>;
/// 130
pub type Ux82 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B0>;
/// 130
pub type Ub10000010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B0>;
/// 131
pub type U131 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B1>;
/// 131
pub type Ux83 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B1>;
/// 131
pub type Ub10000011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B1>;
/// 132
pub type U132 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B0>;
/// 132
pub type Ux84 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B0>;
/// 132
pub type Ub10000100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B0>;
/// 133
pub type U133 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B1>;
/// 133
pub type Ux85 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B1>;
/// 133
pub type Ub10000101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B1>;
/// 134
pub type U134 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B0>;
/// 134
pub type Ux86 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B0>;
/// 134
pub type Ub10000110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B0>;
/// 135
pub type U135 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B1>;
/// 135
pub type Ux87 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B1>;
/// 135
pub type Ub10000111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B1>;
/// 136
pub type U136 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B0>;
/// 136
pub type Ux88 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B0>;
/// 136
pub type Ub10001000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B0>;
/// 137
pub type U137 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B1>;
/// 137
pub type Ux89 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B1>;
/// 137
pub type Ub10001001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B1>;
/// 138
pub type U138 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B0>;
/// 138
pub type Ux8A = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B0>;
/// 138
pub type Ub10001010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B0>;
/// 139
pub type U139 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B1>;
/// 139
pub type Ux8B = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B1>;
/// 139
pub type Ub10001011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B1>;
/// 140
pub type U140 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B0>;
/// 140
pub type Ux8C = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B0>;
/// 140
pub type Ub10001100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B0>;
/// 141
pub type U141 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B1>;
/// 141
pub type Ux8D = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B1>;
/// 141
pub type Ub10001101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B1>;
/// 142
pub type U142 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B0>;
/// 142
pub type Ux8E = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B0>;
/// 142
pub type Ub10001110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B0>;
/// 143
pub type U143 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B1>;
/// 143
pub type Ux8F = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B1>;
/// 143
pub type Ub10001111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B1>;
/// 144
pub type U144 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>;
/// 144
pub type Ux90 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>;
/// 144
pub type Ub10010000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>;
/// 145
pub type U145 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B1>;
/// 145
pub type Ux91 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B1>;
/// 145
pub type Ub10010001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B1>;
/// 146
pub type U146 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B0>;
/// 146
pub type Ux92 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B0>;
/// 146
pub type Ub10010010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B0>;
/// 147
pub type U147 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B1>;
/// 147
pub type Ux93 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B1>;
/// 147
pub type Ub10010011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B1>;
/// 148
pub type U148 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B0>;
/// 148
pub type Ux94 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B0>;
/// 148
pub type Ub10010100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B0>;
/// 149
pub type U149 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B1>;
/// 149
pub type Ux95 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B1>;
/// 149
pub type Ub10010101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B1>;
/// 150
pub type U150 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B0>;
/// 150
pub type Ux96 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B0>;
/// 150
pub type Ub10010110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B0>;
/// 151
pub type U151 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B1>;
/// 151
pub type Ux97 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B1>;
/// 151
pub type Ub10010111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B1>;
/// 152
pub type U152 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B0>;
/// 152
pub type Ux98 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B0>;
/// 152
pub type Ub10011000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B0>;
/// 153
pub type U153 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B1>;
/// 153
pub type Ux99 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B1>;
/// 153
pub type Ub10011001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B1>;
/// 154
pub type U154 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B0>;
/// 154
pub type Ux9A = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B0>;
/// 154
pub type Ub10011010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B0>;
/// 155
pub type U155 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B1>;
/// 155
pub type Ux9B = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B1>;
/// 155
pub type Ub10011011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B1>;
/// 156
pub type U156 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B0>;
/// 156
pub type Ux9C = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B0>;
/// 156
pub type Ub10011100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B0>;
/// 157
pub type U157 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B1>;
/// 157
pub type Ux9D = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B1>;
/// 157
pub type Ub10011101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B1>;
/// 158
pub type U158 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B0>;
/// 158
pub type Ux9E = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B0>;
/// 158
pub type Ub10011110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B0>;
/// 159
pub type U159 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B1>;
/// 159
pub type Ux9F = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B1>;
/// 159
pub type Ub10011111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B1>;
/// 160
pub type U160 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>;
/// 160
pub type UxA0 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>;
/// 160
pub type Ub10100000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>;
/// 161
pub type U161 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B1>;
/// 161
pub type UxA1 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B1>;
/// 161
pub type Ub10100001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B1>;
/// 162
pub type U162 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B0>;
/// 162
pub type UxA2 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B0>;
/// 162
pub type Ub10100010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B0>;
/// 163
pub type U163 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B1>;
/// 163
pub type UxA3 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B1>;
/// 163
pub type Ub10100011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B1>;
/// 164
pub type U164 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B0>;
/// 164
pub type UxA4 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B0>;
/// 164
pub type Ub10100100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B0>;
/// 165
pub type U165 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B1>;
/// 165
pub type UxA5 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B1>;
/// 165
pub type Ub10100101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B1>;
/// 166
pub type U166 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B0>;
/// 166
pub type UxA6 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B0>;
/// 166
pub type Ub10100110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B0>;
/// 167
pub type U167 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B1>;
/// 167
pub type UxA7 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B1>;
/// 167
pub type Ub10100111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B1>;
/// 168
pub type U168 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B0>;
/// 168
pub type UxA8 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B0>;
/// 168
pub type Ub10101000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B0>;
/// 169
pub type U169 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B1>;
/// 169
pub type UxA9 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B1>;
/// 169
pub type Ub10101001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B1>;
/// 170
pub type U170 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B0>;
/// 170
pub type UxAA = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B0>;
/// 170
pub type Ub10101010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B0>;
/// 171
pub type U171 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B1>;
/// 171
pub type UxAB = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B1>;
/// 171
pub type Ub10101011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B1>;
/// 172
pub type U172 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B0>;
/// 172
pub type UxAC = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B0>;
/// 172
pub type Ub10101100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B0>;
/// 173
pub type U173 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B1>;
/// 173
pub type UxAD = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B1>;
/// 173
pub type Ub10101101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B1>;
/// 174
pub type U174 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B0>;
/// 174
pub type UxAE = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B0>;
/// 174
pub type Ub10101110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B0>;
/// 175
pub type U175 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B1>;
/// 175
pub type UxAF = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B1>;
/// 175
pub type Ub10101111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B1>;
/// 176
pub type U176 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B0>;
/// 176
pub type UxB0 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B0>;
/// 176
pub type Ub10110000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B0>;
/// 177
pub type U177 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B1>;
/// 177
pub type UxB1 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B1>;
/// 177
pub type Ub10110001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B1>;
/// 178
pub type U178 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B0>;
/// 178
pub type UxB2 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B0>;
/// 178
pub type Ub10110010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B0>;
/// 179
pub type U179 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B1>;
/// 179
pub type UxB3 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B1>;
/// 179
pub type Ub10110011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B1>;
/// 180
pub type U180 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B0>;
/// 180
pub type UxB4 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B0>;
/// 180
pub type Ub10110100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B0>;
/// 181
pub type U181 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B1>;
/// 181
pub type UxB5 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B1>;
/// 181
pub type Ub10110101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B1>;
/// 182
pub type U182 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B0>;
/// 182
pub type UxB6 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B0>;
/// 182
pub type Ub10110110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B0>;
/// 183
pub type U183 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B1>;
/// 183
pub type UxB7 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B1>;
/// 183
pub type Ub10110111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B1>;
/// 184
pub type U184 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B0>;
/// 184
pub type UxB8 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B0>;
/// 184
pub type Ub10111000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B0>;
/// 185
pub type U185 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B1>;
/// 185
pub type UxB9 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B1>;
/// 185
pub type Ub10111001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B1>;
/// 186
pub type U186 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B0>;
/// 186
pub type UxBA = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B0>;
/// 186
pub type Ub10111010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B0>;
/// 187
pub type U187 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B1>;
/// 187
pub type UxBB = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B1>;
/// 187
pub type Ub10111011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B1>;
/// 188
pub type U188 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B0>;
/// 188
pub type UxBC = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B0>;
/// 188
pub type Ub10111100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B0>;
/// 189
pub type U189 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B1>;
/// 189
pub type UxBD = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B1>;
/// 189
pub type Ub10111101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B1>;
/// 190
pub type U190 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B0>;
/// 190
pub type UxBE = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B0>;
/// 190
pub type Ub10111110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B0>;
/// 191
pub type U191 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B1>;
/// 191
pub type UxBF = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B1>;
/// 191
pub type Ub10111111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B1>;
/// 192
pub type U192 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 192
pub type UxC0 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 192
pub type Ub11000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 193
pub type U193 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B1>;
/// 193
pub type UxC1 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B1>;
/// 193
pub type Ub11000001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B1>;
/// 194
pub type U194 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B0>;
/// 194
pub type UxC2 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B0>;
/// 194
pub type Ub11000010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B0>;
/// 195
pub type U195 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B1>;
/// 195
pub type UxC3 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B1>;
/// 195
pub type Ub11000011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B1>;
/// 196
pub type U196 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B0>;
/// 196
pub type UxC4 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B0>;
/// 196
pub type Ub11000100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B0>;
/// 197
pub type U197 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B1>;
/// 197
pub type UxC5 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B1>;
/// 197
pub type Ub11000101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B1>;
/// 198
pub type U198 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B0>;
/// 198
pub type UxC6 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B0>;
/// 198
pub type Ub11000110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B0>;
/// 199
pub type U199 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B1>;
/// 199
pub type UxC7 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B1>;
/// 199
pub type Ub11000111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B1>;
/// 200
pub type U200 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B0>;
/// 200
pub type UxC8 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B0>;
/// 200
pub type Ub11001000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B0>;
/// 201
pub type U201 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B1>;
/// 201
pub type UxC9 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B1>;
/// 201
pub type Ub11001001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B1>;
/// 202
pub type U202 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B0>;
/// 202
pub type UxCA = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B0>;
/// 202
pub type Ub11001010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B0>;
/// 203
pub type U203 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B1>;
/// 203
pub type UxCB = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B1>;
/// 203
pub type Ub11001011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B1>;
/// 204
pub type U204 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B0>;
/// 204
pub type UxCC = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B0>;
/// 204
pub type Ub11001100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B0>;
/// 205
pub type U205 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B1>;
/// 205
pub type UxCD = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B1>;
/// 205
pub type Ub11001101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B1>;
/// 206
pub type U206 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B0>;
/// 206
pub type UxCE = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B0>;
/// 206
pub type Ub11001110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B0>;
/// 207
pub type U207 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B1>;
/// 207
pub type UxCF = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B1>;
/// 207
pub type Ub11001111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B1>;
/// 208
pub type U208 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>;
/// 208
pub type UxD0 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>;
/// 208
pub type Ub11010000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>;
/// 209
pub type U209 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B1>;
/// 209
pub type UxD1 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B1>;
/// 209
pub type Ub11010001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B1>;
/// 210
pub type U210 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B0>;
/// 210
pub type UxD2 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B0>;
/// 210
pub type Ub11010010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B0>;
/// 211
pub type U211 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B1>;
/// 211
pub type UxD3 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B1>;
/// 211
pub type Ub11010011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B1>;
/// 212
pub type U212 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B0>;
/// 212
pub type UxD4 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B0>;
/// 212
pub type Ub11010100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B0>;
/// 213
pub type U213 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B1>;
/// 213
pub type UxD5 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B1>;
/// 213
pub type Ub11010101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B1>;
/// 214
pub type U214 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B0>;
/// 214
pub type UxD6 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B0>;
/// 214
pub type Ub11010110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B0>;
/// 215
pub type U215 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B1>;
/// 215
pub type UxD7 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B1>;
/// 215
pub type Ub11010111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B1>;
/// 216
pub type U216 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B0>;
/// 216
pub type UxD8 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B0>;
/// 216
pub type Ub11011000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B0>;
/// 217
pub type U217 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B1>;
/// 217
pub type UxD9 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B1>;
/// 217
pub type Ub11011001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B1>;
/// 218
pub type U218 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B0>;
/// 218
pub type UxDA = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B0>;
/// 218
pub type Ub11011010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B0>;
/// 219
pub type U219 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B1>;
/// 219
pub type UxDB = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B1>;
/// 219
pub type Ub11011011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B1>;
/// 220
pub type U220 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B0>;
/// 220
pub type UxDC = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B0>;
/// 220
pub type Ub11011100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B0>;
/// 221
pub type U221 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B1>;
/// 221
pub type UxDD = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B1>;
/// 221
pub type Ub11011101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B1>;
/// 222
pub type U222 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B0>;
/// 222
pub type UxDE = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B0>;
/// 222
pub type Ub11011110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B0>;
/// 223
pub type U223 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B1>;
/// 223
pub type UxDF = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B1>;
/// 223
pub type Ub11011111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B1>;
/// 224
pub type U224 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B0>;
/// 224
pub type UxE0 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B0>;
/// 224
pub type Ub11100000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B0>;
/// 225
pub type U225 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B1>;
/// 225
pub type UxE1 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B1>;
/// 225
pub type Ub11100001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B1>;
/// 226
pub type U226 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B0>;
/// 226
pub type UxE2 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B0>;
/// 226
pub type Ub11100010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B0>;
/// 227
pub type U227 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B1>;
/// 227
pub type UxE3 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B1>;
/// 227
pub type Ub11100011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B1>;
/// 228
pub type U228 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B0>;
/// 228
pub type UxE4 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B0>;
/// 228
pub type Ub11100100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B0>;
/// 229
pub type U229 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B1>;
/// 229
pub type UxE5 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B1>;
/// 229
pub type Ub11100101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B1>;
/// 230
pub type U230 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B0>;
/// 230
pub type UxE6 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B0>;
/// 230
pub type Ub11100110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B0>;
/// 231
pub type U231 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B1>;
/// 231
pub type UxE7 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B1>;
/// 231
pub type Ub11100111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B1>;
/// 232
pub type U232 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B0>;
/// 232
pub type UxE8 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B0>;
/// 232
pub type Ub11101000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B0>;
/// 233
pub type U233 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B1>;
/// 233
pub type UxE9 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B1>;
/// 233
pub type Ub11101001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B1>;
/// 234
pub type U234 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B0>;
/// 234
pub type UxEA = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B0>;
/// 234
pub type Ub11101010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B0>;
/// 235
pub type U235 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B1>;
/// 235
pub type UxEB = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B1>;
/// 235
pub type Ub11101011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B1>;
/// 236
pub type U236 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B0>;
/// 236
pub type UxEC = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B0>;
/// 236
pub type Ub11101100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B0>;
/// 237
pub type U237 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B1>;
/// 237
pub type UxED = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B1>;
/// 237
pub type Ub11101101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B1>;
/// 238
pub type U238 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B0>;
/// 238
pub type UxEE = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B0>;
/// 238
pub type Ub11101110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B0>;
/// 239
pub type U239 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B1>;
/// 239
pub type UxEF = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B1>;
/// 239
pub type Ub11101111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B1>;
/// 240
pub type U240 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B0>;
/// 240
pub type UxF0 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B0>;
/// 240
pub type Ub11110000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B0>;
/// 241
pub type U241 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B1>;
/// 241
pub type UxF1 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B1>;
/// 241
pub type Ub11110001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B1>;
/// 242
pub type U242 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B0>;
/// 242
pub type UxF2 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B0>;
/// 242
pub type Ub11110010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B0>;
/// 243
pub type U243 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B1>;
/// 243
pub type UxF3 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B1>;
/// 243
pub type Ub11110011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B1>;
/// 244
pub type U244 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B0>;
/// 244
pub type UxF4 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B0>;
/// 244
pub type Ub11110100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B0>;
/// 245
pub type U245 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B1>;
/// 245
pub type UxF5 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B1>;
/// 245
pub type Ub11110101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B1>;
/// 246
pub type U246 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B1>, B0>;
/// 246
pub type UxF6 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B1>, B0>;
/// 246
pub type Ub11110110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B1>, B0>;
/// 247
pub type U247 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B1>, B1>;
/// 247
pub type UxF7 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B1>, B1>;
/// 247
pub type Ub11110111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B1>, B1>;
/// 248
pub type U248 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B0>;
/// 248
pub type UxF8 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B0>;
/// 248
pub type Ub11111000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B0>;
/// 249
pub type U249 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B1>;
/// 249
pub type UxF9 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B1>;
/// 249
pub type Ub11111001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B1>;
/// 250
pub type U250 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B0>, B1>, B0>;
/// 250
pub type UxFA = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B0>, B1>, B0>;
/// 250
pub type Ub11111010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B0>, B1>, B0>;
/// 251
pub type U251 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B0>, B1>, B1>;
/// 251
pub type UxFB = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B0>, B1>, B1>;
/// 251
pub type Ub11111011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B0>, B1>, B1>;
/// 252
pub type U252 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B1>, B0>, B0>;
/// 252
pub type UxFC = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B1>, B0>, B0>;
/// 252
pub type Ub11111100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B1>, B0>, B0>;
/// 253
pub type U253 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B1>, B0>, B1>;
/// 253
pub type UxFD = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B1>, B0>, B1>;
/// 253
pub type Ub11111101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B1>, B0>, B1>;
/// 254
pub type U254 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B0>;
/// 254
pub type UxFE = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B0>;
/// 254
pub type Ub11111110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B0>;
/// 255
pub type U255 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>;
/// 255
pub type UxFF = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>;
/// 255
pub type Ub11111111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>;
/// 256
pub type U256 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 256
pub type Ux100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 256
pub type Ub100000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 257
pub type U257 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B1>;
/// 257
pub type Ux101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B1>;
/// 257
pub type Ub100000001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B1>;
/// 258
pub type U258 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B1>, B0>;
/// 258
pub type Ux102 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B1>, B0>;
/// 258
pub type Ub100000010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B1>, B0>;
/// 259
pub type U259 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B1>, B1>;
/// 259
pub type Ux103 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B1>, B1>;
/// 259
pub type Ub100000011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B1>, B1>;
/// 260
pub type U260 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B0>, B0>;
/// 260
pub type Ux104 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B0>, B0>;
/// 260
pub type Ub100000100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B0>, B0>;
/// 261
pub type U261 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B0>, B1>;
/// 261
pub type Ux105 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B0>, B1>;
/// 261
pub type Ub100000101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B0>, B1>;
/// 262
pub type U262 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B1>, B0>;
/// 262
pub type Ux106 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B1>, B0>;
/// 262
pub type Ub100000110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B1>, B0>;
/// 263
pub type U263 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B1>, B1>;
/// 263
pub type Ux107 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B1>, B1>;
/// 263
pub type Ub100000111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B1>, B1>;
/// 264
pub type U264 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B0>;
/// 264
pub type Ux108 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B0>;
/// 264
pub type Ub100001000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B0>;
/// 265
pub type U265 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B1>;
/// 265
pub type Ux109 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B1>;
/// 265
pub type Ub100001001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B1>;
/// 266
pub type U266 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B1>, B0>;
/// 266
pub type Ux10A = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B1>, B0>;
/// 266
pub type Ub100001010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B1>, B0>;
/// 267
pub type U267 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B1>, B1>;
/// 267
pub type Ux10B = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B1>, B1>;
/// 267
pub type Ub100001011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B1>, B1>;
/// 268
pub type U268 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B0>, B0>;
/// 268
pub type Ux10C = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B0>, B0>;
/// 268
pub type Ub100001100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B0>, B0>;
/// 269
pub type U269 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B0>, B1>;
/// 269
pub type Ux10D = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B0>, B1>;
/// 269
pub type Ub100001101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B0>, B1>;
/// 270
pub type U270 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B1>, B0>;
/// 270
pub type Ux10E = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B1>, B0>;
/// 270
pub type Ub100001110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B1>, B0>;
/// 271
pub type U271 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B1>, B1>;
/// 271
pub type Ux10F = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B1>, B1>;
/// 271
pub type Ub100001111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B1>, B1>;
/// 272
pub type U272 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B0>;
/// 272
pub type Ux110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B0>;
/// 272
pub type Ub100010000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B0>;
/// 273
pub type U273 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B1>;
/// 273
pub type Ux111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B1>;
/// 273
pub type Ub100010001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B1>;
/// 274
pub type U274 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B1>, B0>;
/// 274
pub type Ux112 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B1>, B0>;
/// 274
pub type Ub100010010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B1>, B0>;
/// 275
pub type U275 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B1>, B1>;
/// 275
pub type Ux113 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B1>, B1>;
/// 275
pub type Ub100010011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B1>, B1>;
/// 276
pub type U276 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B0>, B0>;
/// 276
pub type Ux114 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B0>, B0>;
/// 276
pub type Ub100010100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B0>, B0>;
/// 277
pub type U277 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B0>, B1>;
/// 277
pub type Ux115 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B0>, B1>;
/// 277
pub type Ub100010101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B0>, B1>;
/// 278
pub type U278 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B1>, B0>;
/// 278
pub type Ux116 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B1>, B0>;
/// 278
pub type Ub100010110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B1>, B0>;
/// 279
pub type U279 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B1>, B1>;
/// 279
pub type Ux117 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B1>, B1>;
/// 279
pub type Ub100010111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B1>, B1>;
/// 280
pub type U280 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B0>, B0>;
/// 280
pub type Ux118 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B0>, B0>;
/// 280
pub type Ub100011000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B0>, B0>;
/// 281
pub type U281 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B0>, B1>;
/// 281
pub type Ux119 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B0>, B1>;
/// 281
pub type Ub100011001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B0>, B1>;
/// 282
pub type U282 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B1>, B0>;
/// 282
pub type Ux11A = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B1>, B0>;
/// 282
pub type Ub100011010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B1>, B0>;
/// 283
pub type U283 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B1>, B1>;
/// 283
pub type Ux11B = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B1>, B1>;
/// 283
pub type Ub100011011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B1>, B1>;
/// 284
pub type U284 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B0>, B0>;
/// 284
pub type Ux11C = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B0>, B0>;
/// 284
pub type Ub100011100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B0>, B0>;
/// 285
pub type U285 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B0>, B1>;
/// 285
pub type Ux11D = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B0>, B1>;
/// 285
pub type Ub100011101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B0>, B1>;
/// 286
pub type U286 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B1>, B0>;
/// 286
pub type Ux11E = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B1>, B0>;
/// 286
pub type Ub100011110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B1>, B0>;
/// 287
pub type U287 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B1>, B1>;
/// 287
pub type Ux11F = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B1>, B1>;
/// 287
pub type Ub100011111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B1>, B1>;
/// 288
pub type U288 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>;
/// 288
pub type Ux120 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>;
/// 288
pub type Ub100100000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>;
/// 289
pub type U289 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B1>;
/// 289
pub type Ux121 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B1>;
/// 289
pub type Ub100100001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B1>;
/// 290
pub type U290 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B1>, B0>;
/// 290
pub type Ux122 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B1>, B0>;
/// 290
pub type Ub100100010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B1>, B0>;
/// 291
pub type U291 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B1>, B1>;
/// 291
pub type Ux123 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B1>, B1>;
/// 291
pub type Ub100100011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B1>, B1>;
/// 292
pub type U292 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B0>, B0>;
/// 292
pub type Ux124 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B0>, B0>;
/// 292
pub type Ub100100100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B0>, B0>;
/// 293
pub type U293 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B0>, B1>;
/// 293
pub type Ux125 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B0>, B1>;
/// 293
pub type Ub100100101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B0>, B1>;
/// 294
pub type U294 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B1>, B0>;
/// 294
pub type Ux126 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B1>, B0>;
/// 294
pub type Ub100100110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B1>, B0>;
/// 295
pub type U295 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B1>, B1>;
/// 295
pub type Ux127 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B1>, B1>;
/// 295
pub type Ub100100111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B1>, B1>;
/// 296
pub type U296 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B0>, B0>;
/// 296
pub type Ux128 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B0>, B0>;
/// 296
pub type Ub100101000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B0>, B0>;
/// 297
pub type U297 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B0>, B1>;
/// 297
pub type Ux129 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B0>, B1>;
/// 297
pub type Ub100101001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B0>, B1>;
/// 298
pub type U298 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B1>, B0>;
/// 298
pub type Ux12A = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B1>, B0>;
/// 298
pub type Ub100101010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B1>, B0>;
/// 299
pub type U299 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B1>, B1>;
/// 299
pub type Ux12B = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B1>, B1>;
/// 299
pub type Ub100101011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B1>, B1>;
/// 300
pub type U300 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B0>, B0>;
/// 300
pub type Ux12C = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B0>, B0>;
/// 300
pub type Ub100101100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B0>, B0>;
/// 301
pub type U301 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B0>, B1>;
/// 301
pub type Ux12D = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B0>, B1>;
/// 301
pub type Ub100101101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B0>, B1>;
/// 302
pub type U302 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B1>, B0>;
/// 302
pub type Ux12E = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B1>, B0>;
/// 302
pub type Ub100101110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B1>, B0>;
/// 303
pub type U303 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B1>, B1>;
/// 303
pub type Ux12F = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B1>, B1>;
/// 303
pub type Ub100101111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B1>, B1>;
/// 304
pub type U304 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B0>, B0>;
/// 304
pub type Ux130 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B0>, B0>;
/// 304
pub type Ub100110000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B0>, B0>;
/// 305
pub type U305 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B0>, B1>;
/// 305
pub type Ux131 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B0>, B1>;
/// 305
pub type Ub100110001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B0>, B1>;
/// 306
pub type U306 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B1>, B0>;
/// 306
pub type Ux132 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B1>, B0>;
/// 306
pub type Ub100110010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B1>, B0>;
/// 307
pub type U307 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B1>, B1>;
/// 307
pub type Ux133 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B1>, B1>;
/// 307
pub type Ub100110011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B1>, B1>;
/// 308
pub type U308 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B0>, B0>;
/// 308
pub type Ux134 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B0>, B0>;
/// 308
pub type Ub100110100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B0>, B0>;
/// 309
pub type U309 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B0>, B1>;
/// 309
pub type Ux135 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B0>, B1>;
/// 309
pub type Ub100110101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B0>, B1>;
/// 310
pub type U310 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B1>, B0>;
/// 310
pub type Ux136 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B1>, B0>;
/// 310
pub type Ub100110110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B1>, B0>;
/// 311
pub type U311 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B1>, B1>;
/// 311
pub type Ux137 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B1>, B1>;
/// 311
pub type Ub100110111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B1>, B1>;
/// 312
pub type U312 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B0>, B0>;
/// 312
pub type Ux138 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B0>, B0>;
/// 312
pub type Ub100111000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B0>, B0>;
/// 313
pub type U313 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B0>, B1>;
/// 313
pub type Ux139 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B0>, B1>;
/// 313
pub type Ub100111001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B0>, B1>;
/// 314
pub type U314 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B1>, B0>;
/// 314
pub type Ux13A = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B1>, B0>;
/// 314
pub type Ub100111010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B1>, B0>;
/// 315
pub type U315 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B1>, B1>;
/// 315
pub type Ux13B = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B1>, B1>;
/// 315
pub type Ub100111011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B1>, B1>;
/// 316
pub type U316 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B0>, B0>;
/// 316
pub type Ux13C = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B0>, B0>;
/// 316
pub type Ub100111100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B0>, B0>;
/// 317
pub type U317 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B0>, B1>;
/// 317
pub type Ux13D = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B0>, B1>;
/// 317
pub type Ub100111101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B0>, B1>;
/// 318
pub type U318 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B1>, B0>;
/// 318
pub type Ux13E = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B1>, B0>;
/// 318
pub type Ub100111110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B1>, B0>;
/// 319
pub type U319 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B1>, B1>;
/// 319
pub type Ux13F = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B1>, B1>;
/// 319
pub type Ub100111111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B1>, B1>;
/// 320
pub type U320 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 320
pub type Ux140 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 320
pub type Ub101000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 321
pub type U321 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B1>;
/// 321
pub type Ux141 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B1>;
/// 321
pub type Ub101000001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B1>;
/// 322
pub type U322 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B1>, B0>;
/// 322
pub type Ux142 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B1>, B0>;
/// 322
pub type Ub101000010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B1>, B0>;
/// 323
pub type U323 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B1>, B1>;
/// 323
pub type Ux143 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B1>, B1>;
/// 323
pub type Ub101000011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B1>, B1>;
/// 324
pub type U324 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B0>, B0>;
/// 324
pub type Ux144 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B0>, B0>;
/// 324
pub type Ub101000100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B0>, B0>;
/// 325
pub type U325 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B0>, B1>;
/// 325
pub type Ux145 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B0>, B1>;
/// 325
pub type Ub101000101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B0>, B1>;
/// 326
pub type U326 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B1>, B0>;
/// 326
pub type Ux146 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B1>, B0>;
/// 326
pub type Ub101000110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B1>, B0>;
/// 327
pub type U327 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B1>, B1>;
/// 327
pub type Ux147 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B1>, B1>;
/// 327
pub type Ub101000111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B1>, B1>;
/// 328
pub type U328 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B0>, B0>;
/// 328
pub type Ux148 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B0>, B0>;
/// 328
pub type Ub101001000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B0>, B0>;
/// 329
pub type U329 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B0>, B1>;
/// 329
pub type Ux149 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B0>, B1>;
/// 329
pub type Ub101001001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B0>, B1>;
/// 330
pub type U330 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B1>, B0>;
/// 330
pub type Ux14A = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B1>, B0>;
/// 330
pub type Ub101001010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B1>, B0>;
/// 331
pub type U331 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B1>, B1>;
/// 331
pub type Ux14B = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B1>, B1>;
/// 331
pub type Ub101001011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B1>, B1>;
/// 332
pub type U332 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B0>, B0>;
/// 332
pub type Ux14C = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B0>, B0>;
/// 332
pub type Ub101001100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B0>, B0>;
/// 333
pub type U333 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B0>, B1>;
/// 333
pub type Ux14D = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B0>, B1>;
/// 333
pub type Ub101001101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B0>, B1>;
/// 334
pub type U334 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B1>, B0>;
/// 334
pub type Ux14E = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B1>, B0>;
/// 334
pub type Ub101001110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B1>, B0>;
/// 335
pub type U335 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B1>, B1>;
/// 335
pub type Ux14F = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B1>, B1>;
/// 335
pub type Ub101001111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B1>, B1>;
/// 336
pub type U336 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B0>;
/// 336
pub type Ux150 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B0>;
/// 336
pub type Ub101010000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B0>;
/// 337
pub type U337 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B1>;
/// 337
pub type Ux151 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B1>;
/// 337
pub type Ub101010001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B1>;
/// 338
pub type U338 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B1>, B0>;
/// 338
pub type Ux152 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B1>, B0>;
/// 338
pub type Ub101010010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B1>, B0>;
/// 339
pub type U339 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B1>, B1>;
/// 339
pub type Ux153 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B1>, B1>;
/// 339
pub type Ub101010011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B1>, B1>;
/// 340
pub type U340 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B0>, B0>;
/// 340
pub type Ux154 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B0>, B0>;
/// 340
pub type Ub101010100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B0>, B0>;
/// 341
pub type U341 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B0>, B1>;
/// 341
pub type Ux155 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B0>, B1>;
/// 341
pub type Ub101010101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B0>, B1>;
/// 342
pub type U342 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B1>, B0>;
/// 342
pub type Ux156 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B1>, B0>;
/// 342
pub type Ub101010110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B1>, B0>;
/// 343
pub type U343 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B1>, B1>;
/// 343
pub type Ux157 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B1>, B1>;
/// 343
pub type Ub101010111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B1>, B1>;
/// 344
pub type U344 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B0>, B0>;
/// 344
pub type Ux158 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B0>, B0>;
/// 344
pub type Ub101011000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B0>, B0>;
/// 345
pub type U345 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B0>, B1>;
/// 345
pub type Ux159 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B0>, B1>;
/// 345
pub type Ub101011001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B0>, B1>;
/// 346
pub type U346 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B1>, B0>;
/// 346
pub type Ux15A = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B1>, B0>;
/// 346
pub type Ub101011010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B1>, B0>;
/// 347
pub type U347 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B1>, B1>;
/// 347
pub type Ux15B = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B1>, B1>;
/// 347
pub type Ub101011011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B1>, B1>;
/// 348
pub type U348 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B0>, B0>;
/// 348
pub type Ux15C = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B0>, B0>;
/// 348
pub type Ub101011100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B0>, B0>;
/// 349
pub type U349 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B0>, B1>;
/// 349
pub type Ux15D = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B0>, B1>;
/// 349
pub type Ub101011101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B0>, B1>;
/// 350
pub type U350 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B1>, B0>;
/// 350
pub type Ux15E = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B1>, B0>;
/// 350
pub type Ub101011110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B1>, B0>;
/// 351
pub type U351 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B1>, B1>;
/// 351
pub type Ux15F = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B1>, B1>;
/// 351
pub type Ub101011111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B1>, B1>;
/// 352
pub type U352 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B0>, B0>;
/// 352
pub type Ux160 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B0>, B0>;
/// 352
pub type Ub101100000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B0>, B0>;
/// 353
pub type U353 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B0>, B1>;
/// 353
pub type Ux161 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B0>, B1>;
/// 353
pub type Ub101100001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B0>, B1>;
/// 354
pub type U354 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B1>, B0>;
/// 354
pub type Ux162 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B1>, B0>;
/// 354
pub type Ub101100010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B1>, B0>;
/// 355
pub type U355 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B1>, B1>;
/// 355
pub type Ux163 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B1>, B1>;
/// 355
pub type Ub101100011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B1>, B1>;
/// 356
pub type U356 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B0>, B0>;
/// 356
pub type Ux164 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B0>, B0>;
/// 356
pub type Ub101100100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B0>, B0>;
/// 357
pub type U357 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B0>, B1>;
/// 357
pub type Ux165 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B0>, B1>;
/// 357
pub type Ub101100101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B0>, B1>;
/// 358
pub type U358 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B1>, B0>;
/// 358
pub type Ux166 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B1>, B0>;
/// 358
pub type Ub101100110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B1>, B0>;
/// 359
pub type U359 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B1>, B1>;
/// 359
pub type Ux167 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B1>, B1>;
/// 359
pub type Ub101100111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B1>, B1>;
/// 360
pub type U360 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B0>, B0>;
/// 360
pub type Ux168 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B0>, B0>;
/// 360
pub type Ub101101000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B0>, B0>;
/// 361
pub type U361 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B0>, B1>;
/// 361
pub type Ux169 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B0>, B1>;
/// 361
pub type Ub101101001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B0>, B1>;
/// 362
pub type U362 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B0>;
/// 362
pub type Ux16A = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B0>;
/// 362
pub type Ub101101010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B0>;
/// 363
pub type U363 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B1>;
/// 363
pub type Ux16B = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B1>;
/// 363
pub type Ub101101011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B1>;
/// 364
pub type U364 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B0>, B0>;
/// 364
pub type Ux16C = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B0>, B0>;
/// 364
pub type Ub101101100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B0>, B0>;
/// 365
pub type U365 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B0>, B1>;
/// 365
pub type Ux16D = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B0>, B1>;
/// 365
pub type Ub101101101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B0>, B1>;
/// 366
pub type U366 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B1>, B0>;
/// 366
pub type Ux16E = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B1>, B0>;
/// 366
pub type Ub101101110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B1>, B0>;
/// 367
pub type U367 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B1>, B1>;
/// 367
pub type Ux16F = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B1>, B1>;
/// 367
pub type Ub101101111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B1>, B1>;
/// 368
pub type U368 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B0>, B0>;
/// 368
pub type Ux170 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B0>, B0>;
/// 368
pub type Ub101110000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B0>, B0>;
/// 369
pub type U369 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B0>, B1>;
/// 369
pub type Ux171 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B0>, B1>;
/// 369
pub type Ub101110001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B0>, B1>;
/// 370
pub type U370 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B1>, B0>;
/// 370
pub type Ux172 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B1>, B0>;
/// 370
pub type Ub101110010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B1>, B0>;
/// 371
pub type U371 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B1>, B1>;
/// 371
pub type Ux173 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B1>, B1>;
/// 371
pub type Ub101110011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B1>, B1>;
/// 372
pub type U372 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B0>, B0>;
/// 372
pub type Ux174 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B0>, B0>;
/// 372
pub type Ub101110100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B0>, B0>;
/// 373
pub type U373 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B0>, B1>;
/// 373
pub type Ux175 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B0>, B1>;
/// 373
pub type Ub101110101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B0>, B1>;
/// 374
pub type U374 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B1>, B0>;
/// 374
pub type Ux176 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B1>, B0>;
/// 374
pub type Ub101110110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B1>, B0>;
/// 375
pub type U375 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B1>, B1>;
/// 375
pub type Ux177 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B1>, B1>;
/// 375
pub type Ub101110111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B1>, B1>;
/// 376
pub type U376 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B0>;
/// 376
pub type Ux178 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B0>;
/// 376
pub type Ub101111000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B0>;
/// 377
pub type U377 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B1>;
/// 377
pub type Ux179 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B1>;
/// 377
pub type Ub101111001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B1>;
/// 378
pub type U378 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B1>, B0>;
/// 378
pub type Ux17A = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B1>, B0>;
/// 378
pub type Ub101111010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B1>, B0>;
/// 379
pub type U379 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B1>, B1>;
/// 379
pub type Ux17B = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B1>, B1>;
/// 379
pub type Ub101111011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B1>, B1>;
/// 380
pub type U380 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B0>, B0>;
/// 380
pub type Ux17C = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B0>, B0>;
/// 380
pub type Ub101111100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B0>, B0>;
/// 381
pub type U381 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B0>, B1>;
/// 381
pub type Ux17D = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B0>, B1>;
/// 381
pub type Ub101111101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B0>, B1>;
/// 382
pub type U382 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B1>, B0>;
/// 382
pub type Ux17E = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B1>, B0>;
/// 382
pub type Ub101111110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B1>, B0>;
/// 383
pub type U383 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B1>, B1>;
/// 383
pub type Ux17F = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B1>, B1>;
/// 383
pub type Ub101111111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B1>, B1>;
/// 384
pub type U384 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 384
pub type Ux180 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 384
pub type Ub110000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 385
pub type U385 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B1>;
/// 385
pub type Ux181 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B1>;
/// 385
pub type Ub110000001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B1>;
/// 386
pub type U386 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B0>;
/// 386
pub type Ux182 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B0>;
/// 386
pub type Ub110000010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B0>;
/// 387
pub type U387 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B1>;
/// 387
pub type Ux183 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B1>;
/// 387
pub type Ub110000011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B1>;
/// 388
pub type U388 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B0>;
/// 388
pub type Ux184 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B0>;
/// 388
pub type Ub110000100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B0>;
/// 389
pub type U389 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B1>;
/// 389
pub type Ux185 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B1>;
/// 389
pub type Ub110000101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B1>;
/// 390
pub type U390 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B0>;
/// 390
pub type Ux186 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B0>;
/// 390
pub type Ub110000110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B0>;
/// 391
pub type U391 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B1>;
/// 391
pub type Ux187 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B1>;
/// 391
pub type Ub110000111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B1>;
/// 392
pub type U392 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B0>;
/// 392
pub type Ux188 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B0>;
/// 392
pub type Ub110001000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B0>;
/// 393
pub type U393 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B1>;
/// 393
pub type Ux189 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B1>;
/// 393
pub type Ub110001001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B1>;
/// 394
pub type U394 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B0>;
/// 394
pub type Ux18A = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B0>;
/// 394
pub type Ub110001010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B0>;
/// 395
pub type U395 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B1>;
/// 395
pub type Ux18B = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B1>;
/// 395
pub type Ub110001011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B1>;
/// 396
pub type U396 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B0>;
/// 396
pub type Ux18C = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B0>;
/// 396
pub type Ub110001100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B0>;
/// 397
pub type U397 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B1>;
/// 397
pub type Ux18D = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B1>;
/// 397
pub type Ub110001101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B1>;
/// 398
pub type U398 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B0>;
/// 398
pub type Ux18E = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B0>;
/// 398
pub type Ub110001110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B0>;
/// 399
pub type U399 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B1>;
/// 399
pub type Ux18F = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B1>;
/// 399
pub type Ub110001111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B1>;
/// 400
pub type U400 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>;
/// 400
pub type Ux190 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>;
/// 400
pub type Ub110010000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>;
/// 401
pub type U401 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B1>;
/// 401
pub type Ux191 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B1>;
/// 401
pub type Ub110010001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B1>;
/// 402
pub type U402 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B0>;
/// 402
pub type Ux192 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B0>;
/// 402
pub type Ub110010010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B0>;
/// 403
pub type U403 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B1>;
/// 403
pub type Ux193 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B1>;
/// 403
pub type Ub110010011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B1>;
/// 404
pub type U404 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B0>;
/// 404
pub type Ux194 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B0>;
/// 404
pub type Ub110010100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B0>;
/// 405
pub type U405 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B1>;
/// 405
pub type Ux195 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B1>;
/// 405
pub type Ub110010101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B1>;
/// 406
pub type U406 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B0>;
/// 406
pub type Ux196 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B0>;
/// 406
pub type Ub110010110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B0>;
/// 407
pub type U407 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B1>;
/// 407
pub type Ux197 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B1>;
/// 407
pub type Ub110010111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B1>;
/// 408
pub type U408 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B0>;
/// 408
pub type Ux198 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B0>;
/// 408
pub type Ub110011000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B0>;
/// 409
pub type U409 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B1>;
/// 409
pub type Ux199 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B1>;
/// 409
pub type Ub110011001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B1>;
/// 410
pub type U410 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B0>;
/// 410
pub type Ux19A = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B0>;
/// 410
pub type Ub110011010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B0>;
/// 411
pub type U411 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B1>;
/// 411
pub type Ux19B = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B1>;
/// 411
pub type Ub110011011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B1>;
/// 412
pub type U412 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B0>;
/// 412
pub type Ux19C = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B0>;
/// 412
pub type Ub110011100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B0>;
/// 413
pub type U413 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B1>;
/// 413
pub type Ux19D = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B1>;
/// 413
pub type Ub110011101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B1>;
/// 414
pub type U414 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B0>;
/// 414
pub type Ux19E = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B0>;
/// 414
pub type Ub110011110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B0>;
/// 415
pub type U415 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B1>;
/// 415
pub type Ux19F = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B1>;
/// 415
pub type Ub110011111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B1>;
/// 416
pub type U416 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>;
/// 416
pub type Ux1A0 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>;
/// 416
pub type Ub110100000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>;
/// 417
pub type U417 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B1>;
/// 417
pub type Ux1A1 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B1>;
/// 417
pub type Ub110100001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B1>;
/// 418
pub type U418 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B0>;
/// 418
pub type Ux1A2 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B0>;
/// 418
pub type Ub110100010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B0>;
/// 419
pub type U419 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B1>;
/// 419
pub type Ux1A3 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B1>;
/// 419
pub type Ub110100011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B1>;
/// 420
pub type U420 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B0>;
/// 420
pub type Ux1A4 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B0>;
/// 420
pub type Ub110100100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B0>;
/// 421
pub type U421 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B1>;
/// 421
pub type Ux1A5 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B1>;
/// 421
pub type Ub110100101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B1>;
/// 422
pub type U422 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B0>;
/// 422
pub type Ux1A6 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B0>;
/// 422
pub type Ub110100110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B0>;
/// 423
pub type U423 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B1>;
/// 423
pub type Ux1A7 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B1>;
/// 423
pub type Ub110100111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B1>;
/// 424
pub type U424 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B0>;
/// 424
pub type Ux1A8 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B0>;
/// 424
pub type Ub110101000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B0>;
/// 425
pub type U425 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B1>;
/// 425
pub type Ux1A9 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B1>;
/// 425
pub type Ub110101001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B1>;
/// 426
pub type U426 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B0>;
/// 426
pub type Ux1AA = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B0>;
/// 426
pub type Ub110101010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B0>;
/// 427
pub type U427 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B1>;
/// 427
pub type Ux1AB = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B1>;
/// 427
pub type Ub110101011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B1>;
/// 428
pub type U428 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B0>;
/// 428
pub type Ux1AC = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B0>;
/// 428
pub type Ub110101100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B0>;
/// 429
pub type U429 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B1>;
/// 429
pub type Ux1AD = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B1>;
/// 429
pub type Ub110101101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B1>;
/// 430
pub type U430 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B0>;
/// 430
pub type Ux1AE = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B0>;
/// 430
pub type Ub110101110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B0>;
/// 431
pub type U431 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B1>;
/// 431
pub type Ux1AF = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B1>;
/// 431
pub type Ub110101111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B1>;
/// 432
pub type U432 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B0>;
/// 432
pub type Ux1B0 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B0>;
/// 432
pub type Ub110110000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B0>;
/// 433
pub type U433 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B1>;
/// 433
pub type Ux1B1 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B1>;
/// 433
pub type Ub110110001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B1>;
/// 434
pub type U434 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B0>;
/// 434
pub type Ux1B2 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B0>;
/// 434
pub type Ub110110010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B0>;
/// 435
pub type U435 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B1>;
/// 435
pub type Ux1B3 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B1>;
/// 435
pub type Ub110110011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B1>;
/// 436
pub type U436 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B0>;
/// 436
pub type Ux1B4 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B0>;
/// 436
pub type Ub110110100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B0>;
/// 437
pub type U437 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B1>;
/// 437
pub type Ux1B5 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B1>;
/// 437
pub type Ub110110101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B1>;
/// 438
pub type U438 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B0>;
/// 438
pub type Ux1B6 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B0>;
/// 438
pub type Ub110110110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B0>;
/// 439
pub type U439 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B1>;
/// 439
pub type Ux1B7 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B1>;
/// 439
pub type Ub110110111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B1>;
/// 440
pub type U440 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B0>;
/// 440
pub type Ux1B8 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B0>;
/// 440
pub type Ub110111000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B0>;
/// 441
pub type U441 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B1>;
/// 441
pub type Ux1B9 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B1>;
/// 441
pub type Ub110111001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B1>;
/// 442
pub type U442 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B0>;
/// 442
pub type Ux1BA = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B0>;
/// 442
pub type Ub110111010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B0>;
/// 443
pub type U443 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B1>;
/// 443
pub type Ux1BB = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B1>;
/// 443
pub type Ub110111011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B1>;
/// 444
pub type U444 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B0>;
/// 444
pub type Ux1BC = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B0>;
/// 444
pub type Ub110111100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B0>;
/// 445
pub type U445 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B1>;
/// 445
pub type Ux1BD = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B1>;
/// 445
pub type Ub110111101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B1>;
/// 446
pub type U446 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B0>;
/// 446
pub type Ux1BE = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B0>;
/// 446
pub type Ub110111110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B0>;
/// 447
pub type U447 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B1>;
/// 447
pub type Ux1BF = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B1>;
/// 447
pub type Ub110111111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B1>;
/// 448
pub type U448 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 448
pub type Ux1C0 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 448
pub type Ub111000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 449
pub type U449 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B1>;
/// 449
pub type Ux1C1 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B1>;
/// 449
pub type Ub111000001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B1>;
/// 450
pub type U450 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B0>;
/// 450
pub type Ux1C2 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B0>;
/// 450
pub type Ub111000010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B0>;
/// 451
pub type U451 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B1>;
/// 451
pub type Ux1C3 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B1>;
/// 451
pub type Ub111000011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B1>;
/// 452
pub type U452 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B0>;
/// 452
pub type Ux1C4 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B0>;
/// 452
pub type Ub111000100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B0>;
/// 453
pub type U453 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B1>;
/// 453
pub type Ux1C5 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B1>;
/// 453
pub type Ub111000101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B1>;
/// 454
pub type U454 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B0>;
/// 454
pub type Ux1C6 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B0>;
/// 454
pub type Ub111000110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B0>;
/// 455
pub type U455 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B1>;
/// 455
pub type Ux1C7 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B1>;
/// 455
pub type Ub111000111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B1>;
/// 456
pub type U456 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B0>;
/// 456
pub type Ux1C8 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B0>;
/// 456
pub type Ub111001000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B0>;
/// 457
pub type U457 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B1>;
/// 457
pub type Ux1C9 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B1>;
/// 457
pub type Ub111001001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B1>;
/// 458
pub type U458 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B0>;
/// 458
pub type Ux1CA = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B0>;
/// 458
pub type Ub111001010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B0>;
/// 459
pub type U459 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B1>;
/// 459
pub type Ux1CB = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B1>;
/// 459
pub type Ub111001011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B1>;
/// 460
pub type U460 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B0>;
/// 460
pub type Ux1CC = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B0>;
/// 460
pub type Ub111001100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B0>;
/// 461
pub type U461 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B1>;
/// 461
pub type Ux1CD = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B1>;
/// 461
pub type Ub111001101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B1>;
/// 462
pub type U462 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B0>;
/// 462
pub type Ux1CE = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B0>;
/// 462
pub type Ub111001110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B0>;
/// 463
pub type U463 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B1>;
/// 463
pub type Ux1CF = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B1>;
/// 463
pub type Ub111001111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B1>;
/// 464
pub type U464 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>;
/// 464
pub type Ux1D0 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>;
/// 464
pub type Ub111010000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>;
/// 465
pub type U465 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B1>;
/// 465
pub type Ux1D1 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B1>;
/// 465
pub type Ub111010001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B1>;
/// 466
pub type U466 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B0>;
/// 466
pub type Ux1D2 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B0>;
/// 466
pub type Ub111010010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B0>;
/// 467
pub type U467 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B1>;
/// 467
pub type Ux1D3 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B1>;
/// 467
pub type Ub111010011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B1>;
/// 468
pub type U468 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B0>;
/// 468
pub type Ux1D4 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B0>;
/// 468
pub type Ub111010100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B0>;
/// 469
pub type U469 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B1>;
/// 469
pub type Ux1D5 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B1>;
/// 469
pub type Ub111010101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B1>;
/// 470
pub type U470 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B0>;
/// 470
pub type Ux1D6 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B0>;
/// 470
pub type Ub111010110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B0>;
/// 471
pub type U471 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B1>;
/// 471
pub type Ux1D7 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B1>;
/// 471
pub type Ub111010111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B1>;
/// 472
pub type U472 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B0>;
/// 472
pub type Ux1D8 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B0>;
/// 472
pub type Ub111011000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B0>;
/// 473
pub type U473 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B1>;
/// 473
pub type Ux1D9 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B1>;
/// 473
pub type Ub111011001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B1>;
/// 474
pub type U474 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B0>;
/// 474
pub type Ux1DA = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B0>;
/// 474
pub type Ub111011010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B0>;
/// 475
pub type U475 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B1>;
/// 475
pub type Ux1DB = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B1>;
/// 475
pub type Ub111011011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B1>;
/// 476
pub type U476 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B0>;
/// 476
pub type Ux1DC = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B0>;
/// 476
pub type Ub111011100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B0>;
/// 477
pub type U477 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B1>;
/// 477
pub type Ux1DD = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B1>;
/// 477
pub type Ub111011101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B1>;
/// 478
pub type U478 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B0>;
/// 478
pub type Ux1DE = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B0>;
/// 478
pub type Ub111011110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B0>;
/// 479
pub type U479 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B1>;
/// 479
pub type Ux1DF = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B1>;
/// 479
pub type Ub111011111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B1>;
/// 480
pub type U480 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B0>;
/// 480
pub type Ux1E0 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B0>;
/// 480
pub type Ub111100000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B0>;
/// 481
pub type U481 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B1>;
/// 481
pub type Ux1E1 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B1>;
/// 481
pub type Ub111100001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B1>;
/// 482
pub type U482 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B0>;
/// 482
pub type Ux1E2 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B0>;
/// 482
pub type Ub111100010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B0>;
/// 483
pub type U483 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B1>;
/// 483
pub type Ux1E3 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B1>;
/// 483
pub type Ub111100011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B1>;
/// 484
pub type U484 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B0>;
/// 484
pub type Ux1E4 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B0>;
/// 484
pub type Ub111100100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B0>;
/// 485
pub type U485 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B1>;
/// 485
pub type Ux1E5 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B1>;
/// 485
pub type Ub111100101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B1>;
/// 486
pub type U486 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B0>;
/// 486
pub type Ux1E6 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B0>;
/// 486
pub type Ub111100110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B0>;
/// 487
pub type U487 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B1>;
/// 487
pub type Ux1E7 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B1>;
/// 487
pub type Ub111100111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B1>;
/// 488
pub type U488 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B0>;
/// 488
pub type Ux1E8 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B0>;
/// 488
pub type Ub111101000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B0>;
/// 489
pub type U489 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B1>;
/// 489
pub type Ux1E9 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B1>;
/// 489
pub type Ub111101001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B1>;
/// 490
pub type U490 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B0>;
/// 490
pub type Ux1EA = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B0>;
/// 490
pub type Ub111101010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B0>;
/// 491
pub type U491 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B1>;
/// 491
pub type Ux1EB = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B1>;
/// 491
pub type Ub111101011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B1>;
/// 492
pub type U492 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B0>;
/// 492
pub type Ux1EC = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B0>;
/// 492
pub type Ub111101100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B0>;
/// 493
pub type U493 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B1>;
/// 493
pub type Ux1ED = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B1>;
/// 493
pub type Ub111101101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B1>;
/// 494
pub type U494 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B0>;
/// 494
pub type Ux1EE = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B0>;
/// 494
pub type Ub111101110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B0>;
/// 495
pub type U495 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B1>;
/// 495
pub type Ux1EF = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B1>;
/// 495
pub type Ub111101111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B1>;
/// 496
pub type U496 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B0>;
/// 496
pub type Ux1F0 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B0>;
/// 496
pub type Ub111110000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B0>;
/// 497
pub type U497 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B1>;
/// 497
pub type Ux1F1 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B1>;
/// 497
pub type Ub111110001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B1>;
/// 498
pub type U498 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B0>;
/// 498
pub type Ux1F2 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B0>;
/// 498
pub type Ub111110010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B0>;
/// 499
pub type U499 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B1>;
/// 499
pub type Ux1F3 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B1>;
/// 499
pub type Ub111110011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B1>;
/// 500
pub type U500 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B0>;
/// 500
pub type Ux1F4 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B0>;
/// 500
pub type Ub111110100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B0>;
/// 501
pub type U501 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B1>;
/// 501
pub type Ux1F5 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B1>;
/// 501
pub type Ub111110101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B1>;
/// 502
pub type U502 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B0>, B1>, B1>, B0>;
/// 502
pub type Ux1F6 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B0>, B1>, B1>, B0>;
/// 502
pub type Ub111110110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B0>, B1>, B1>, B0>;
/// 503
pub type U503 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B0>, B1>, B1>, B1>;
/// 503
pub type Ux1F7 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B0>, B1>, B1>, B1>;
/// 503
pub type Ub111110111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B0>, B1>, B1>, B1>;
/// 504
pub type U504 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B0>;
/// 504
pub type Ux1F8 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B0>;
/// 504
pub type Ub111111000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B0>;
/// 505
pub type U505 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B1>;
/// 505
pub type Ux1F9 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B1>;
/// 505
pub type Ub111111001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B1>;
/// 506
pub type U506 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B1>, B0>, B1>, B0>;
/// 506
pub type Ux1FA = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B1>, B0>, B1>, B0>;
/// 506
pub type Ub111111010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B1>, B0>, B1>, B0>;
/// 507
pub type U507 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B1>, B0>, B1>, B1>;
/// 507
pub type Ux1FB = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B1>, B0>, B1>, B1>;
/// 507
pub type Ub111111011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B1>, B0>, B1>, B1>;
/// 508
pub type U508 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B0>, B0>;
/// 508
pub type Ux1FC = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B0>, B0>;
/// 508
pub type Ub111111100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B0>, B0>;
/// 509
pub type U509 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B0>, B1>;
/// 509
pub type Ux1FD = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B0>, B1>;
/// 509
pub type Ub111111101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B0>, B1>;
/// 510
pub type U510 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B0>;
/// 510
pub type Ux1FE = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B0>;
/// 510
pub type Ub111111110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B0>;
/// 511
pub type U511 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>;
/// 511
pub type Ux1FF = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>;
/// 511
pub type Ub111111111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>;
/// 512
pub type U512 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 512
pub type Ux200 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 512
pub type Ub1000000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 513
pub type U513 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B1>;
/// 513
pub type Ux201 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B1>;
/// 513
pub type Ub1000000001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B1>;
/// 514
pub type U514 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B1>, B0>;
/// 514
pub type Ux202 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B1>, B0>;
/// 514
pub type Ub1000000010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B1>, B0>;
/// 515
pub type U515 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B1>, B1>;
/// 515
pub type Ux203 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B1>, B1>;
/// 515
pub type Ub1000000011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B1>, B1>;
/// 516
pub type U516 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B1>, B0>, B0>;
/// 516
pub type Ux204 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B1>, B0>, B0>;
/// 516
pub type Ub1000000100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B1>, B0>, B0>;
/// 517
pub type U517 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B1>, B0>, B1>;
/// 517
pub type Ux205 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B1>, B0>, B1>;
/// 517
pub type Ub1000000101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B1>, B0>, B1>;
/// 518
pub type U518 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B1>, B1>, B0>;
/// 518
pub type Ux206 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B1>, B1>, B0>;
/// 518
pub type Ub1000000110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B1>, B1>, B0>;
/// 519
pub type U519 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B1>, B1>, B1>;
/// 519
pub type Ux207 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B1>, B1>, B1>;
/// 519
pub type Ub1000000111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B1>, B1>, B1>;
/// 520
pub type U520 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B0>;
/// 520
pub type Ux208 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B0>;
/// 520
pub type Ub1000001000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B0>;
/// 521
pub type U521 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B1>;
/// 521
pub type Ux209 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B1>;
/// 521
pub type Ub1000001001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B1>;
/// 522
pub type U522 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B0>, B1>, B0>;
/// 522
pub type Ux20A = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B0>, B1>, B0>;
/// 522
pub type Ub1000001010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B0>, B1>, B0>;
/// 523
pub type U523 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B0>, B1>, B1>;
/// 523
pub type Ux20B = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B0>, B1>, B1>;
/// 523
pub type Ub1000001011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B0>, B1>, B1>;
/// 524
pub type U524 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B1>, B0>, B0>;
/// 524
pub type Ux20C = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B1>, B0>, B0>;
/// 524
pub type Ub1000001100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B1>, B0>, B0>;
/// 525
pub type U525 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B1>, B0>, B1>;
/// 525
pub type Ux20D = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B1>, B0>, B1>;
/// 525
pub type Ub1000001101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B1>, B0>, B1>;
/// 526
pub type U526 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B1>, B1>, B0>;
/// 526
pub type Ux20E = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B1>, B1>, B0>;
/// 526
pub type Ub1000001110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B1>, B1>, B0>;
/// 527
pub type U527 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B1>, B1>, B1>;
/// 527
pub type Ux20F = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B1>, B1>, B1>;
/// 527
pub type Ub1000001111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B1>, B1>, B1>;
/// 528
pub type U528 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B0>;
/// 528
pub type Ux210 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B0>;
/// 528
pub type Ub1000010000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B0>;
/// 529
pub type U529 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B1>;
/// 529
pub type Ux211 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B1>;
/// 529
pub type Ub1000010001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B1>;
/// 530
pub type U530 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B1>, B0>;
/// 530
pub type Ux212 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B1>, B0>;
/// 530
pub type Ub1000010010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B1>, B0>;
/// 531
pub type U531 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B1>, B1>;
/// 531
pub type Ux213 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B1>, B1>;
/// 531
pub type Ub1000010011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B1>, B1>;
/// 532
pub type U532 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B1>, B0>, B0>;
/// 532
pub type Ux214 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B1>, B0>, B0>;
/// 532
pub type Ub1000010100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B1>, B0>, B0>;
/// 533
pub type U533 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B1>, B0>, B1>;
/// 533
pub type Ux215 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B1>, B0>, B1>;
/// 533
pub type Ub1000010101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B1>, B0>, B1>;
/// 534
pub type U534 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B1>, B1>, B0>;
/// 534
pub type Ux216 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B1>, B1>, B0>;
/// 534
pub type Ub1000010110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B1>, B1>, B0>;
/// 535
pub type U535 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B1>, B1>, B1>;
/// 535
pub type Ux217 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B1>, B1>, B1>;
/// 535
pub type Ub1000010111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B1>, B1>, B1>;
/// 536
pub type U536 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B0>, B0>, B0>;
/// 536
pub type Ux218 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B0>, B0>, B0>;
/// 536
pub type Ub1000011000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B0>, B0>, B0>;
/// 537
pub type U537 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B0>, B0>, B1>;
/// 537
pub type Ux219 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B0>, B0>, B1>;
/// 537
pub type Ub1000011001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B0>, B0>, B1>;
/// 538
pub type U538 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B0>, B1>, B0>;
/// 538
pub type Ux21A = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B0>, B1>, B0>;
/// 538
pub type Ub1000011010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B0>, B1>, B0>;
/// 539
pub type U539 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B0>, B1>, B1>;
/// 539
pub type Ux21B = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B0>, B1>, B1>;
/// 539
pub type Ub1000011011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B0>, B1>, B1>;
/// 540
pub type U540 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B1>, B0>, B0>;
/// 540
pub type Ux21C = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B1>, B0>, B0>;
/// 540
pub type Ub1000011100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B1>, B0>, B0>;
/// 541
pub type U541 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B1>, B0>, B1>;
/// 541
pub type Ux21D = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B1>, B0>, B1>;
/// 541
pub type Ub1000011101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B1>, B0>, B1>;
/// 542
pub type U542 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B1>, B1>, B0>;
/// 542
pub type Ux21E = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B1>, B1>, B0>;
/// 542
pub type Ub1000011110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B1>, B1>, B0>;
/// 543
pub type U543 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B1>, B1>, B1>;
/// 543
pub type Ux21F = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B1>, B1>, B1>;
/// 543
pub type Ub1000011111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B1>, B1>, B1>;
/// 544
pub type U544 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>;
/// 544
pub type Ux220 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>;
/// 544
pub type Ub1000100000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>;
/// 545
pub type U545 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B1>;
/// 545
pub type Ux221 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B1>;
/// 545
pub type Ub1000100001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B1>;
/// 546
pub type U546 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B1>, B0>;
/// 546
pub type Ux222 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B1>, B0>;
/// 546
pub type Ub1000100010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B1>, B0>;
/// 547
pub type U547 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B1>, B1>;
/// 547
pub type Ux223 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B1>, B1>;
/// 547
pub type Ub1000100011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B1>, B1>;
/// 548
pub type U548 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B1>, B0>, B0>;
/// 548
pub type Ux224 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B1>, B0>, B0>;
/// 548
pub type Ub1000100100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B1>, B0>, B0>;
/// 549
pub type U549 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B1>, B0>, B1>;
/// 549
pub type Ux225 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B1>, B0>, B1>;
/// 549
pub type Ub1000100101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B1>, B0>, B1>;
/// 550
pub type U550 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B1>, B1>, B0>;
/// 550
pub type Ux226 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B1>, B1>, B0>;
/// 550
pub type Ub1000100110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B1>, B1>, B0>;
/// 551
pub type U551 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B1>, B1>, B1>;
/// 551
pub type Ux227 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B1>, B1>, B1>;
/// 551
pub type Ub1000100111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B1>, B1>, B1>;
/// 552
pub type U552 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B0>, B0>, B0>;
/// 552
pub type Ux228 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B0>, B0>, B0>;
/// 552
pub type Ub1000101000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B0>, B0>, B0>;
/// 553
pub type U553 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B0>, B0>, B1>;
/// 553
pub type Ux229 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B0>, B0>, B1>;
/// 553
pub type Ub1000101001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B0>, B0>, B1>;
/// 554
pub type U554 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B0>, B1>, B0>;
/// 554
pub type Ux22A = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B0>, B1>, B0>;
/// 554
pub type Ub1000101010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B0>, B1>, B0>;
/// 555
pub type U555 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B0>, B1>, B1>;
/// 555
pub type Ux22B = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B0>, B1>, B1>;
/// 555
pub type Ub1000101011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B0>, B1>, B1>;
/// 556
pub type U556 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B1>, B0>, B0>;
/// 556
pub type Ux22C = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B1>, B0>, B0>;
/// 556
pub type Ub1000101100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B1>, B0>, B0>;
/// 557
pub type U557 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B1>, B0>, B1>;
/// 557
pub type Ux22D = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B1>, B0>, B1>;
/// 557
pub type Ub1000101101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B1>, B0>, B1>;
/// 558
pub type U558 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B1>, B1>, B0>;
/// 558
pub type Ux22E = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B1>, B1>, B0>;
/// 558
pub type Ub1000101110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B1>, B1>, B0>;
/// 559
pub type U559 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B1>, B1>, B1>;
/// 559
pub type Ux22F = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B1>, B1>, B1>;
/// 559
pub type Ub1000101111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B1>, B1>, B1>;
/// 560
pub type U560 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B0>, B0>, B0>;
/// 560
pub type Ux230 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B0>, B0>, B0>;
/// 560
pub type Ub1000110000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B0>, B0>, B0>;
/// 561
pub type U561 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B0>, B0>, B1>;
/// 561
pub type Ux231 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B0>, B0>, B1>;
/// 561
pub type Ub1000110001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B0>, B0>, B1>;
/// 562
pub type U562 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B0>, B1>, B0>;
/// 562
pub type Ux232 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B0>, B1>, B0>;
/// 562
pub type Ub1000110010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B0>, B1>, B0>;
/// 563
pub type U563 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B0>, B1>, B1>;
/// 563
pub type Ux233 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B0>, B1>, B1>;
/// 563
pub type Ub1000110011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B0>, B1>, B1>;
/// 564
pub type U564 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B1>, B0>, B0>;
/// 564
pub type Ux234 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B1>, B0>, B0>;
/// 564
pub type Ub1000110100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B1>, B0>, B0>;
/// 565
pub type U565 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B1>, B0>, B1>;
/// 565
pub type Ux235 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B1>, B0>, B1>;
/// 565
pub type Ub1000110101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B1>, B0>, B1>;
/// 566
pub type U566 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B1>, B1>, B0>;
/// 566
pub type Ux236 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B1>, B1>, B0>;
/// 566
pub type Ub1000110110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B1>, B1>, B0>;
/// 567
pub type U567 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B1>, B1>, B1>;
/// 567
pub type Ux237 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B1>, B1>, B1>;
/// 567
pub type Ub1000110111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B1>, B1>, B1>;
/// 568
pub type U568 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B0>, B0>, B0>;
/// 568
pub type Ux238 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B0>, B0>, B0>;
/// 568
pub type Ub1000111000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B0>, B0>, B0>;
/// 569
pub type U569 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B0>, B0>, B1>;
/// 569
pub type Ux239 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B0>, B0>, B1>;
/// 569
pub type Ub1000111001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B0>, B0>, B1>;
/// 570
pub type U570 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B0>, B1>, B0>;
/// 570
pub type Ux23A = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B0>, B1>, B0>;
/// 570
pub type Ub1000111010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B0>, B1>, B0>;
/// 571
pub type U571 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B0>, B1>, B1>;
/// 571
pub type Ux23B = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B0>, B1>, B1>;
/// 571
pub type Ub1000111011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B0>, B1>, B1>;
/// 572
pub type U572 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B1>, B0>, B0>;
/// 572
pub type Ux23C = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B1>, B0>, B0>;
/// 572
pub type Ub1000111100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B1>, B0>, B0>;
/// 573
pub type U573 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B1>, B0>, B1>;
/// 573
pub type Ux23D = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B1>, B0>, B1>;
/// 573
pub type Ub1000111101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B1>, B0>, B1>;
/// 574
pub type U574 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B1>, B1>, B0>;
/// 574
pub type Ux23E = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B1>, B1>, B0>;
/// 574
pub type Ub1000111110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B1>, B1>, B0>;
/// 575
pub type U575 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B1>, B1>, B1>;
/// 575
pub type Ux23F = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B1>, B1>, B1>;
/// 575
pub type Ub1000111111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B1>, B1>, B1>;
/// 576
pub type U576 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 576
pub type Ux240 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 576
pub type Ub1001000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 577
pub type U577 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B1>;
/// 577
pub type Ux241 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B1>;
/// 577
pub type Ub1001000001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B1>;
/// 578
pub type U578 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B1>, B0>;
/// 578
pub type Ux242 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B1>, B0>;
/// 578
pub type Ub1001000010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B1>, B0>;
/// 579
pub type U579 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B1>, B1>;
/// 579
pub type Ux243 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B1>, B1>;
/// 579
pub type Ub1001000011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B1>, B1>;
/// 580
pub type U580 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B1>, B0>, B0>;
/// 580
pub type Ux244 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B1>, B0>, B0>;
/// 580
pub type Ub1001000100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B1>, B0>, B0>;
/// 581
pub type U581 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B1>, B0>, B1>;
/// 581
pub type Ux245 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B1>, B0>, B1>;
/// 581
pub type Ub1001000101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B1>, B0>, B1>;
/// 582
pub type U582 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B1>, B1>, B0>;
/// 582
pub type Ux246 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B1>, B1>, B0>;
/// 582
pub type Ub1001000110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B1>, B1>, B0>;
/// 583
pub type U583 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B1>, B1>, B1>;
/// 583
pub type Ux247 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B1>, B1>, B1>;
/// 583
pub type Ub1001000111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B1>, B1>, B1>;
/// 584
pub type U584 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B0>, B0>, B0>;
/// 584
pub type Ux248 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B0>, B0>, B0>;
/// 584
pub type Ub1001001000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B0>, B0>, B0>;
/// 585
pub type U585 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B0>, B0>, B1>;
/// 585
pub type Ux249 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B0>, B0>, B1>;
/// 585
pub type Ub1001001001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B0>, B0>, B1>;
/// 586
pub type U586 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B0>, B1>, B0>;
/// 586
pub type Ux24A = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B0>, B1>, B0>;
/// 586
pub type Ub1001001010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B0>, B1>, B0>;
/// 587
pub type U587 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B0>, B1>, B1>;
/// 587
pub type Ux24B = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B0>, B1>, B1>;
/// 587
pub type Ub1001001011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B0>, B1>, B1>;
/// 588
pub type U588 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B1>, B0>, B0>;
/// 588
pub type Ux24C = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B1>, B0>, B0>;
/// 588
pub type Ub1001001100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B1>, B0>, B0>;
/// 589
pub type U589 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B1>, B0>, B1>;
/// 589
pub type Ux24D = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B1>, B0>, B1>;
/// 589
pub type Ub1001001101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B1>, B0>, B1>;
/// 590
pub type U590 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B1>, B1>, B0>;
/// 590
pub type Ux24E = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B1>, B1>, B0>;
/// 590
pub type Ub1001001110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B1>, B1>, B0>;
/// 591
pub type U591 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B1>, B1>, B1>;
/// 591
pub type Ux24F = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B1>, B1>, B1>;
/// 591
pub type Ub1001001111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B1>, B1>, B1>;
/// 592
pub type U592 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B0>;
/// 592
pub type Ux250 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B0>;
/// 592
pub type Ub1001010000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B0>;
/// 593
pub type U593 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B1>;
/// 593
pub type Ux251 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B1>;
/// 593
pub type Ub1001010001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B1>;
/// 594
pub type U594 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B0>, B1>, B0>;
/// 594
pub type Ux252 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B0>, B1>, B0>;
/// 594
pub type Ub1001010010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B0>, B1>, B0>;
/// 595
pub type U595 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B0>, B1>, B1>;
/// 595
pub type Ux253 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B0>, B1>, B1>;
/// 595
pub type Ub1001010011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B0>, B1>, B1>;
/// 596
pub type U596 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B1>, B0>, B0>;
/// 596
pub type Ux254 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B1>, B0>, B0>;
/// 596
pub type Ub1001010100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B1>, B0>, B0>;
/// 597
pub type U597 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B1>, B0>, B1>;
/// 597
pub type Ux255 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B1>, B0>, B1>;
/// 597
pub type Ub1001010101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B1>, B0>, B1>;
/// 598
pub type U598 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B1>, B1>, B0>;
/// 598
pub type Ux256 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B1>, B1>, B0>;
/// 598
pub type Ub1001010110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B1>, B1>, B0>;
/// 599
pub type U599 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B1>, B1>, B1>;
/// 599
pub type Ux257 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B1>, B1>, B1>;
/// 599
pub type Ub1001010111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B1>, B1>, B1>;
/// 600
pub type U600 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B0>, B0>, B0>;
/// 600
pub type Ux258 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B0>, B0>, B0>;
/// 600
pub type Ub1001011000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B0>, B0>, B0>;
/// 601
pub type U601 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B0>, B0>, B1>;
/// 601
pub type Ux259 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B0>, B0>, B1>;
/// 601
pub type Ub1001011001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B0>, B0>, B1>;
/// 602
pub type U602 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B0>, B1>, B0>;
/// 602
pub type Ux25A = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B0>, B1>, B0>;
/// 602
pub type Ub1001011010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B0>, B1>, B0>;
/// 603
pub type U603 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B0>, B1>, B1>;
/// 603
pub type Ux25B = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B0>, B1>, B1>;
/// 603
pub type Ub1001011011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B0>, B1>, B1>;
/// 604
pub type U604 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B1>, B0>, B0>;
/// 604
pub type Ux25C = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B1>, B0>, B0>;
/// 604
pub type Ub1001011100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B1>, B0>, B0>;
/// 605
pub type U605 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B1>, B0>, B1>;
/// 605
pub type Ux25D = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B1>, B0>, B1>;
/// 605
pub type Ub1001011101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B1>, B0>, B1>;
/// 606
pub type U606 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B1>, B1>, B0>;
/// 606
pub type Ux25E = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B1>, B1>, B0>;
/// 606
pub type Ub1001011110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B1>, B1>, B0>;
/// 607
pub type U607 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B1>, B1>, B1>;
/// 607
pub type Ux25F = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B1>, B1>, B1>;
/// 607
pub type Ub1001011111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B1>, B1>, B1>;
/// 608
pub type U608 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B0>, B0>, B0>;
/// 608
pub type Ux260 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B0>, B0>, B0>;
/// 608
pub type Ub1001100000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B0>, B0>, B0>;
/// 609
pub type U609 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B0>, B0>, B1>;
/// 609
pub type Ux261 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B0>, B0>, B1>;
/// 609
pub type Ub1001100001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B0>, B0>, B1>;
/// 610
pub type U610 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B0>, B1>, B0>;
/// 610
pub type Ux262 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B0>, B1>, B0>;
/// 610
pub type Ub1001100010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B0>, B1>, B0>;
/// 611
pub type U611 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B0>, B1>, B1>;
/// 611
pub type Ux263 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B0>, B1>, B1>;
/// 611
pub type Ub1001100011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B0>, B1>, B1>;
/// 612
pub type U612 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B1>, B0>, B0>;
/// 612
pub type Ux264 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B1>, B0>, B0>;
/// 612
pub type Ub1001100100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B1>, B0>, B0>;
/// 613
pub type U613 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B1>, B0>, B1>;
/// 613
pub type Ux265 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B1>, B0>, B1>;
/// 613
pub type Ub1001100101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B1>, B0>, B1>;
/// 614
pub type U614 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B1>, B1>, B0>;
/// 614
pub type Ux266 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B1>, B1>, B0>;
/// 614
pub type Ub1001100110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B1>, B1>, B0>;
/// 615
pub type U615 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B1>, B1>, B1>;
/// 615
pub type Ux267 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B1>, B1>, B1>;
/// 615
pub type Ub1001100111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B1>, B1>, B1>;
/// 616
pub type U616 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B0>, B0>, B0>;
/// 616
pub type Ux268 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B0>, B0>, B0>;
/// 616
pub type Ub1001101000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B0>, B0>, B0>;
/// 617
pub type U617 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B0>, B0>, B1>;
/// 617
pub type Ux269 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B0>, B0>, B1>;
/// 617
pub type Ub1001101001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B0>, B0>, B1>;
/// 618
pub type U618 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B0>;
/// 618
pub type Ux26A = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B0>;
/// 618
pub type Ub1001101010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B0>;
/// 619
pub type U619 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B1>;
/// 619
pub type Ux26B = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B1>;
/// 619
pub type Ub1001101011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B1>;
/// 620
pub type U620 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B1>, B0>, B0>;
/// 620
pub type Ux26C = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B1>, B0>, B0>;
/// 620
pub type Ub1001101100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B1>, B0>, B0>;
/// 621
pub type U621 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B1>, B0>, B1>;
/// 621
pub type Ux26D = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B1>, B0>, B1>;
/// 621
pub type Ub1001101101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B1>, B0>, B1>;
/// 622
pub type U622 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B1>, B1>, B0>;
/// 622
pub type Ux26E = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B1>, B1>, B0>;
/// 622
pub type Ub1001101110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B1>, B1>, B0>;
/// 623
pub type U623 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B1>, B1>, B1>;
/// 623
pub type Ux26F = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B1>, B1>, B1>;
/// 623
pub type Ub1001101111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B1>, B1>, B1>;
/// 624
pub type U624 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B0>, B0>, B0>;
/// 624
pub type Ux270 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B0>, B0>, B0>;
/// 624
pub type Ub1001110000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B0>, B0>, B0>;
/// 625
pub type U625 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B0>, B0>, B1>;
/// 625
pub type Ux271 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B0>, B0>, B1>;
/// 625
pub type Ub1001110001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B0>, B0>, B1>;
/// 626
pub type U626 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B0>, B1>, B0>;
/// 626
pub type Ux272 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B0>, B1>, B0>;
/// 626
pub type Ub1001110010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B0>, B1>, B0>;
/// 627
pub type U627 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B0>, B1>, B1>;
/// 627
pub type Ux273 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B0>, B1>, B1>;
/// 627
pub type Ub1001110011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B0>, B1>, B1>;
/// 628
pub type U628 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B1>, B0>, B0>;
/// 628
pub type Ux274 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B1>, B0>, B0>;
/// 628
pub type Ub1001110100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B1>, B0>, B0>;
/// 629
pub type U629 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B1>, B0>, B1>;
/// 629
pub type Ux275 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B1>, B0>, B1>;
/// 629
pub type Ub1001110101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B1>, B0>, B1>;
/// 630
pub type U630 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B1>, B1>, B0>;
/// 630
pub type Ux276 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B1>, B1>, B0>;
/// 630
pub type Ub1001110110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B1>, B1>, B0>;
/// 631
pub type U631 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B1>, B1>, B1>;
/// 631
pub type Ux277 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B1>, B1>, B1>;
/// 631
pub type Ub1001110111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B1>, B1>, B1>;
/// 632
pub type U632 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B0>;
/// 632
pub type Ux278 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B0>;
/// 632
pub type Ub1001111000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B0>;
/// 633
pub type U633 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B1>;
/// 633
pub type Ux279 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B1>;
/// 633
pub type Ub1001111001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B1>;
/// 634
pub type U634 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B0>, B1>, B0>;
/// 634
pub type Ux27A = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B0>, B1>, B0>;
/// 634
pub type Ub1001111010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B0>, B1>, B0>;
/// 635
pub type U635 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B0>, B1>, B1>;
/// 635
pub type Ux27B = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B0>, B1>, B1>;
/// 635
pub type Ub1001111011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B0>, B1>, B1>;
/// 636
pub type U636 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B1>, B0>, B0>;
/// 636
pub type Ux27C = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B1>, B0>, B0>;
/// 636
pub type Ub1001111100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B1>, B0>, B0>;
/// 637
pub type U637 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B1>, B0>, B1>;
/// 637
pub type Ux27D = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B1>, B0>, B1>;
/// 637
pub type Ub1001111101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B1>, B0>, B1>;
/// 638
pub type U638 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B1>, B1>, B0>;
/// 638
pub type Ux27E = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B1>, B1>, B0>;
/// 638
pub type Ub1001111110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B1>, B1>, B0>;
/// 639
pub type U639 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B1>, B1>, B1>;
/// 639
pub type Ux27F = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B1>, B1>, B1>;
/// 639
pub type Ub1001111111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B1>, B1>, B1>;
/// 640
pub type U640 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 640
pub type Ux280 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 640
pub type Ub1010000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 641
pub type U641 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B1>;
/// 641
pub type Ux281 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B1>;
/// 641
pub type Ub1010000001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B1>;
/// 642
pub type U642 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B0>;
/// 642
pub type Ux282 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B0>;
/// 642
pub type Ub1010000010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B0>;
/// 643
pub type U643 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B1>;
/// 643
pub type Ux283 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B1>;
/// 643
pub type Ub1010000011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B1>;
/// 644
pub type U644 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B0>;
/// 644
pub type Ux284 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B0>;
/// 644
pub type Ub1010000100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B0>;
/// 645
pub type U645 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B1>;
/// 645
pub type Ux285 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B1>;
/// 645
pub type Ub1010000101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B1>;
/// 646
pub type U646 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B0>;
/// 646
pub type Ux286 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B0>;
/// 646
pub type Ub1010000110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B0>;
/// 647
pub type U647 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B1>;
/// 647
pub type Ux287 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B1>;
/// 647
pub type Ub1010000111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B1>;
/// 648
pub type U648 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B0>;
/// 648
pub type Ux288 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B0>;
/// 648
pub type Ub1010001000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B0>;
/// 649
pub type U649 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B1>;
/// 649
pub type Ux289 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B1>;
/// 649
pub type Ub1010001001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B1>;
/// 650
pub type U650 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B0>;
/// 650
pub type Ux28A = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B0>;
/// 650
pub type Ub1010001010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B0>;
/// 651
pub type U651 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B1>;
/// 651
pub type Ux28B = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B1>;
/// 651
pub type Ub1010001011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B1>;
/// 652
pub type U652 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B0>;
/// 652
pub type Ux28C = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B0>;
/// 652
pub type Ub1010001100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B0>;
/// 653
pub type U653 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B1>;
/// 653
pub type Ux28D = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B1>;
/// 653
pub type Ub1010001101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B1>;
/// 654
pub type U654 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B0>;
/// 654
pub type Ux28E = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B0>;
/// 654
pub type Ub1010001110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B0>;
/// 655
pub type U655 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B1>;
/// 655
pub type Ux28F = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B1>;
/// 655
pub type Ub1010001111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B1>;
/// 656
pub type U656 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>;
/// 656
pub type Ux290 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>;
/// 656
pub type Ub1010010000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>;
/// 657
pub type U657 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B1>;
/// 657
pub type Ux291 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B1>;
/// 657
pub type Ub1010010001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B1>;
/// 658
pub type U658 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B0>;
/// 658
pub type Ux292 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B0>;
/// 658
pub type Ub1010010010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B0>;
/// 659
pub type U659 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B1>;
/// 659
pub type Ux293 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B1>;
/// 659
pub type Ub1010010011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B1>;
/// 660
pub type U660 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B0>;
/// 660
pub type Ux294 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B0>;
/// 660
pub type Ub1010010100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B0>;
/// 661
pub type U661 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B1>;
/// 661
pub type Ux295 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B1>;
/// 661
pub type Ub1010010101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B1>;
/// 662
pub type U662 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B0>;
/// 662
pub type Ux296 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B0>;
/// 662
pub type Ub1010010110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B0>;
/// 663
pub type U663 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B1>;
/// 663
pub type Ux297 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B1>;
/// 663
pub type Ub1010010111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B1>;
/// 664
pub type U664 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B0>;
/// 664
pub type Ux298 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B0>;
/// 664
pub type Ub1010011000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B0>;
/// 665
pub type U665 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B1>;
/// 665
pub type Ux299 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B1>;
/// 665
pub type Ub1010011001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B1>;
/// 666
pub type U666 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B0>;
/// 666
pub type Ux29A = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B0>;
/// 666
pub type Ub1010011010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B0>;
/// 667
pub type U667 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B1>;
/// 667
pub type Ux29B = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B1>;
/// 667
pub type Ub1010011011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B1>;
/// 668
pub type U668 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B0>;
/// 668
pub type Ux29C = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B0>;
/// 668
pub type Ub1010011100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B0>;
/// 669
pub type U669 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B1>;
/// 669
pub type Ux29D = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B1>;
/// 669
pub type Ub1010011101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B1>;
/// 670
pub type U670 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B0>;
/// 670
pub type Ux29E = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B0>;
/// 670
pub type Ub1010011110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B0>;
/// 671
pub type U671 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B1>;
/// 671
pub type Ux29F = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B1>;
/// 671
pub type Ub1010011111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B1>;
/// 672
pub type U672 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>;
/// 672
pub type Ux2A0 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>;
/// 672
pub type Ub1010100000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>;
/// 673
pub type U673 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B1>;
/// 673
pub type Ux2A1 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B1>;
/// 673
pub type Ub1010100001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B1>;
/// 674
pub type U674 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B0>;
/// 674
pub type Ux2A2 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B0>;
/// 674
pub type Ub1010100010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B0>;
/// 675
pub type U675 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B1>;
/// 675
pub type Ux2A3 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B1>;
/// 675
pub type Ub1010100011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B1>;
/// 676
pub type U676 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B0>;
/// 676
pub type Ux2A4 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B0>;
/// 676
pub type Ub1010100100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B0>;
/// 677
pub type U677 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B1>;
/// 677
pub type Ux2A5 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B1>;
/// 677
pub type Ub1010100101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B1>;
/// 678
pub type U678 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B0>;
/// 678
pub type Ux2A6 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B0>;
/// 678
pub type Ub1010100110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B0>;
/// 679
pub type U679 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B1>;
/// 679
pub type Ux2A7 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B1>;
/// 679
pub type Ub1010100111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B1>;
/// 680
pub type U680 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B0>;
/// 680
pub type Ux2A8 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B0>;
/// 680
pub type Ub1010101000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B0>;
/// 681
pub type U681 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B1>;
/// 681
pub type Ux2A9 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B1>;
/// 681
pub type Ub1010101001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B1>;
/// 682
pub type U682 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B0>;
/// 682
pub type Ux2AA = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B0>;
/// 682
pub type Ub1010101010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B0>;
/// 683
pub type U683 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B1>;
/// 683
pub type Ux2AB = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B1>;
/// 683
pub type Ub1010101011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B1>;
/// 684
pub type U684 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B0>;
/// 684
pub type Ux2AC = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B0>;
/// 684
pub type Ub1010101100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B0>;
/// 685
pub type U685 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B1>;
/// 685
pub type Ux2AD = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B1>;
/// 685
pub type Ub1010101101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B1>;
/// 686
pub type U686 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B0>;
/// 686
pub type Ux2AE = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B0>;
/// 686
pub type Ub1010101110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B0>;
/// 687
pub type U687 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B1>;
/// 687
pub type Ux2AF = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B1>;
/// 687
pub type Ub1010101111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B1>;
/// 688
pub type U688 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B0>;
/// 688
pub type Ux2B0 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B0>;
/// 688
pub type Ub1010110000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B0>;
/// 689
pub type U689 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B1>;
/// 689
pub type Ux2B1 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B1>;
/// 689
pub type Ub1010110001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B1>;
/// 690
pub type U690 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B0>;
/// 690
pub type Ux2B2 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B0>;
/// 690
pub type Ub1010110010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B0>;
/// 691
pub type U691 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B1>;
/// 691
pub type Ux2B3 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B1>;
/// 691
pub type Ub1010110011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B1>;
/// 692
pub type U692 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B0>;
/// 692
pub type Ux2B4 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B0>;
/// 692
pub type Ub1010110100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B0>;
/// 693
pub type U693 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B1>;
/// 693
pub type Ux2B5 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B1>;
/// 693
pub type Ub1010110101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B1>;
/// 694
pub type U694 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B0>;
/// 694
pub type Ux2B6 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B0>;
/// 694
pub type Ub1010110110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B0>;
/// 695
pub type U695 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B1>;
/// 695
pub type Ux2B7 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B1>;
/// 695
pub type Ub1010110111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B1>;
/// 696
pub type U696 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B0>;
/// 696
pub type Ux2B8 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B0>;
/// 696
pub type Ub1010111000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B0>;
/// 697
pub type U697 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B1>;
/// 697
pub type Ux2B9 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B1>;
/// 697
pub type Ub1010111001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B1>;
/// 698
pub type U698 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B0>;
/// 698
pub type Ux2BA = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B0>;
/// 698
pub type Ub1010111010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B0>;
/// 699
pub type U699 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B1>;
/// 699
pub type Ux2BB = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B1>;
/// 699
pub type Ub1010111011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B1>;
/// 700
pub type U700 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B0>;
/// 700
pub type Ux2BC = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B0>;
/// 700
pub type Ub1010111100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B0>;
/// 701
pub type U701 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B1>;
/// 701
pub type Ux2BD = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B1>;
/// 701
pub type Ub1010111101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B1>;
/// 702
pub type U702 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B0>;
/// 702
pub type Ux2BE = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B0>;
/// 702
pub type Ub1010111110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B0>;
/// 703
pub type U703 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B1>;
/// 703
pub type Ux2BF = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B1>;
/// 703
pub type Ub1010111111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B1>;
/// 704
pub type U704 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 704
pub type Ux2C0 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 704
pub type Ub1011000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 705
pub type U705 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B1>;
/// 705
pub type Ux2C1 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B1>;
/// 705
pub type Ub1011000001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B1>;
/// 706
pub type U706 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B0>;
/// 706
pub type Ux2C2 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B0>;
/// 706
pub type Ub1011000010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B0>;
/// 707
pub type U707 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B1>;
/// 707
pub type Ux2C3 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B1>;
/// 707
pub type Ub1011000011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B1>;
/// 708
pub type U708 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B0>;
/// 708
pub type Ux2C4 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B0>;
/// 708
pub type Ub1011000100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B0>;
/// 709
pub type U709 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B1>;
/// 709
pub type Ux2C5 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B1>;
/// 709
pub type Ub1011000101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B1>;
/// 710
pub type U710 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B0>;
/// 710
pub type Ux2C6 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B0>;
/// 710
pub type Ub1011000110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B0>;
/// 711
pub type U711 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B1>;
/// 711
pub type Ux2C7 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B1>;
/// 711
pub type Ub1011000111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B1>;
/// 712
pub type U712 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B0>;
/// 712
pub type Ux2C8 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B0>;
/// 712
pub type Ub1011001000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B0>;
/// 713
pub type U713 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B1>;
/// 713
pub type Ux2C9 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B1>;
/// 713
pub type Ub1011001001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B1>;
/// 714
pub type U714 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B0>;
/// 714
pub type Ux2CA = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B0>;
/// 714
pub type Ub1011001010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B0>;
/// 715
pub type U715 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B1>;
/// 715
pub type Ux2CB = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B1>;
/// 715
pub type Ub1011001011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B1>;
/// 716
pub type U716 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B0>;
/// 716
pub type Ux2CC = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B0>;
/// 716
pub type Ub1011001100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B0>;
/// 717
pub type U717 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B1>;
/// 717
pub type Ux2CD = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B1>;
/// 717
pub type Ub1011001101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B1>;
/// 718
pub type U718 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B0>;
/// 718
pub type Ux2CE = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B0>;
/// 718
pub type Ub1011001110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B0>;
/// 719
pub type U719 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B1>;
/// 719
pub type Ux2CF = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B1>;
/// 719
pub type Ub1011001111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B1>;
/// 720
pub type U720 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>;
/// 720
pub type Ux2D0 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>;
/// 720
pub type Ub1011010000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>;
/// 721
pub type U721 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B1>;
/// 721
pub type Ux2D1 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B1>;
/// 721
pub type Ub1011010001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B1>;
/// 722
pub type U722 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B0>;
/// 722
pub type Ux2D2 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B0>;
/// 722
pub type Ub1011010010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B0>;
/// 723
pub type U723 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B1>;
/// 723
pub type Ux2D3 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B1>;
/// 723
pub type Ub1011010011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B1>;
/// 724
pub type U724 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B0>;
/// 724
pub type Ux2D4 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B0>;
/// 724
pub type Ub1011010100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B0>;
/// 725
pub type U725 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B1>;
/// 725
pub type Ux2D5 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B1>;
/// 725
pub type Ub1011010101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B1>;
/// 726
pub type U726 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B0>;
/// 726
pub type Ux2D6 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B0>;
/// 726
pub type Ub1011010110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B0>;
/// 727
pub type U727 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B1>;
/// 727
pub type Ux2D7 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B1>;
/// 727
pub type Ub1011010111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B1>;
/// 728
pub type U728 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B0>;
/// 728
pub type Ux2D8 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B0>;
/// 728
pub type Ub1011011000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B0>;
/// 729
pub type U729 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B1>;
/// 729
pub type Ux2D9 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B1>;
/// 729
pub type Ub1011011001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B1>;
/// 730
pub type U730 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B0>;
/// 730
pub type Ux2DA = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B0>;
/// 730
pub type Ub1011011010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B0>;
/// 731
pub type U731 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B1>;
/// 731
pub type Ux2DB = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B1>;
/// 731
pub type Ub1011011011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B1>;
/// 732
pub type U732 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B0>;
/// 732
pub type Ux2DC = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B0>;
/// 732
pub type Ub1011011100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B0>;
/// 733
pub type U733 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B1>;
/// 733
pub type Ux2DD = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B1>;
/// 733
pub type Ub1011011101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B1>;
/// 734
pub type U734 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B0>;
/// 734
pub type Ux2DE = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B0>;
/// 734
pub type Ub1011011110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B0>;
/// 735
pub type U735 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B1>;
/// 735
pub type Ux2DF = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B1>;
/// 735
pub type Ub1011011111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B1>;
/// 736
pub type U736 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B0>;
/// 736
pub type Ux2E0 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B0>;
/// 736
pub type Ub1011100000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B0>;
/// 737
pub type U737 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B1>;
/// 737
pub type Ux2E1 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B1>;
/// 737
pub type Ub1011100001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B1>;
/// 738
pub type U738 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B0>;
/// 738
pub type Ux2E2 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B0>;
/// 738
pub type Ub1011100010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B0>;
/// 739
pub type U739 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B1>;
/// 739
pub type Ux2E3 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B1>;
/// 739
pub type Ub1011100011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B1>;
/// 740
pub type U740 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B0>;
/// 740
pub type Ux2E4 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B0>;
/// 740
pub type Ub1011100100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B0>;
/// 741
pub type U741 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B1>;
/// 741
pub type Ux2E5 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B1>;
/// 741
pub type Ub1011100101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B1>;
/// 742
pub type U742 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B0>;
/// 742
pub type Ux2E6 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B0>;
/// 742
pub type Ub1011100110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B0>;
/// 743
pub type U743 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B1>;
/// 743
pub type Ux2E7 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B1>;
/// 743
pub type Ub1011100111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B1>;
/// 744
pub type U744 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B0>;
/// 744
pub type Ux2E8 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B0>;
/// 744
pub type Ub1011101000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B0>;
/// 745
pub type U745 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B1>;
/// 745
pub type Ux2E9 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B1>;
/// 745
pub type Ub1011101001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B1>;
/// 746
pub type U746 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B0>;
/// 746
pub type Ux2EA = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B0>;
/// 746
pub type Ub1011101010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B0>;
/// 747
pub type U747 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B1>;
/// 747
pub type Ux2EB = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B1>;
/// 747
pub type Ub1011101011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B1>;
/// 748
pub type U748 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B0>;
/// 748
pub type Ux2EC = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B0>;
/// 748
pub type Ub1011101100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B0>;
/// 749
pub type U749 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B1>;
/// 749
pub type Ux2ED = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B1>;
/// 749
pub type Ub1011101101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B1>;
/// 750
pub type U750 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B0>;
/// 750
pub type Ux2EE = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B0>;
/// 750
pub type Ub1011101110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B0>;
/// 751
pub type U751 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B1>;
/// 751
pub type Ux2EF = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B1>;
/// 751
pub type Ub1011101111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B1>;
/// 752
pub type U752 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B0>;
/// 752
pub type Ux2F0 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B0>;
/// 752
pub type Ub1011110000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B0>;
/// 753
pub type U753 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B1>;
/// 753
pub type Ux2F1 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B1>;
/// 753
pub type Ub1011110001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B1>;
/// 754
pub type U754 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B0>;
/// 754
pub type Ux2F2 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B0>;
/// 754
pub type Ub1011110010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B0>;
/// 755
pub type U755 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B1>;
/// 755
pub type Ux2F3 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B1>;
/// 755
pub type Ub1011110011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B1>;
/// 756
pub type U756 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B0>;
/// 756
pub type Ux2F4 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B0>;
/// 756
pub type Ub1011110100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B0>;
/// 757
pub type U757 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B1>;
/// 757
pub type Ux2F5 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B1>;
/// 757
pub type Ub1011110101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B1>;
/// 758
pub type U758 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B1>, B1>, B0>;
/// 758
pub type Ux2F6 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B1>, B1>, B0>;
/// 758
pub type Ub1011110110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B1>, B1>, B0>;
/// 759
pub type U759 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B1>, B1>, B1>;
/// 759
pub type Ux2F7 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B1>, B1>, B1>;
/// 759
pub type Ub1011110111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B1>, B1>, B1>;
/// 760
pub type U760 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B0>;
/// 760
pub type Ux2F8 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B0>;
/// 760
pub type Ub1011111000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B0>;
/// 761
pub type U761 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B1>;
/// 761
pub type Ux2F9 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B1>;
/// 761
pub type Ub1011111001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B1>;
/// 762
pub type U762 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B0>, B1>, B0>;
/// 762
pub type Ux2FA = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B0>, B1>, B0>;
/// 762
pub type Ub1011111010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B0>, B1>, B0>;
/// 763
pub type U763 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B0>, B1>, B1>;
/// 763
pub type Ux2FB = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B0>, B1>, B1>;
/// 763
pub type Ub1011111011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B0>, B1>, B1>;
/// 764
pub type U764 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B1>, B0>, B0>;
/// 764
pub type Ux2FC = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B1>, B0>, B0>;
/// 764
pub type Ub1011111100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B1>, B0>, B0>;
/// 765
pub type U765 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B1>, B0>, B1>;
/// 765
pub type Ux2FD = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B1>, B0>, B1>;
/// 765
pub type Ub1011111101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B1>, B0>, B1>;
/// 766
pub type U766 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B0>;
/// 766
pub type Ux2FE = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B0>;
/// 766
pub type Ub1011111110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B0>;
/// 767
pub type U767 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>;
/// 767
pub type Ux2FF = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>;
/// 767
pub type Ub1011111111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>;
/// 768
pub type U768 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 768
pub type Ux300 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 768
pub type Ub1100000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 769
pub type U769 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B1>;
/// 769
pub type Ux301 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B1>;
/// 769
pub type Ub1100000001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B1>;
/// 770
pub type U770 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B1>, B0>;
/// 770
pub type Ux302 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B1>, B0>;
/// 770
pub type Ub1100000010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B1>, B0>;
/// 771
pub type U771 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B1>, B1>;
/// 771
pub type Ux303 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B1>, B1>;
/// 771
pub type Ub1100000011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B1>, B1>;
/// 772
pub type U772 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B0>, B0>;
/// 772
pub type Ux304 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B0>, B0>;
/// 772
pub type Ub1100000100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B0>, B0>;
/// 773
pub type U773 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B0>, B1>;
/// 773
pub type Ux305 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B0>, B1>;
/// 773
pub type Ub1100000101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B0>, B1>;
/// 774
pub type U774 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B1>, B0>;
/// 774
pub type Ux306 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B1>, B0>;
/// 774
pub type Ub1100000110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B1>, B0>;
/// 775
pub type U775 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B1>, B1>;
/// 775
pub type Ux307 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B1>, B1>;
/// 775
pub type Ub1100000111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B1>, B1>;
/// 776
pub type U776 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B0>;
/// 776
pub type Ux308 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B0>;
/// 776
pub type Ub1100001000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B0>;
/// 777
pub type U777 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B1>;
/// 777
pub type Ux309 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B1>;
/// 777
pub type Ub1100001001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B1>;
/// 778
pub type U778 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B1>, B0>;
/// 778
pub type Ux30A = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B1>, B0>;
/// 778
pub type Ub1100001010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B1>, B0>;
/// 779
pub type U779 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B1>, B1>;
/// 779
pub type Ux30B = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B1>, B1>;
/// 779
pub type Ub1100001011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B1>, B1>;
/// 780
pub type U780 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B0>, B0>;
/// 780
pub type Ux30C = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B0>, B0>;
/// 780
pub type Ub1100001100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B0>, B0>;
/// 781
pub type U781 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B0>, B1>;
/// 781
pub type Ux30D = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B0>, B1>;
/// 781
pub type Ub1100001101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B0>, B1>;
/// 782
pub type U782 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B1>, B0>;
/// 782
pub type Ux30E = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B1>, B0>;
/// 782
pub type Ub1100001110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B1>, B0>;
/// 783
pub type U783 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B1>, B1>;
/// 783
pub type Ux30F = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B1>, B1>;
/// 783
pub type Ub1100001111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B1>, B1>;
/// 784
pub type U784 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B0>;
/// 784
pub type Ux310 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B0>;
/// 784
pub type Ub1100010000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B0>;
/// 785
pub type U785 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B1>;
/// 785
pub type Ux311 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B1>;
/// 785
pub type Ub1100010001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B1>;
/// 786
pub type U786 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B1>, B0>;
/// 786
pub type Ux312 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B1>, B0>;
/// 786
pub type Ub1100010010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B1>, B0>;
/// 787
pub type U787 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B1>, B1>;
/// 787
pub type Ux313 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B1>, B1>;
/// 787
pub type Ub1100010011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B1>, B1>;
/// 788
pub type U788 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B0>, B0>;
/// 788
pub type Ux314 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B0>, B0>;
/// 788
pub type Ub1100010100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B0>, B0>;
/// 789
pub type U789 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B0>, B1>;
/// 789
pub type Ux315 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B0>, B1>;
/// 789
pub type Ub1100010101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B0>, B1>;
/// 790
pub type U790 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B1>, B0>;
/// 790
pub type Ux316 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B1>, B0>;
/// 790
pub type Ub1100010110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B1>, B0>;
/// 791
pub type U791 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B1>, B1>;
/// 791
pub type Ux317 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B1>, B1>;
/// 791
pub type Ub1100010111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B1>, B1>;
/// 792
pub type U792 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B0>, B0>;
/// 792
pub type Ux318 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B0>, B0>;
/// 792
pub type Ub1100011000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B0>, B0>;
/// 793
pub type U793 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B0>, B1>;
/// 793
pub type Ux319 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B0>, B1>;
/// 793
pub type Ub1100011001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B0>, B1>;
/// 794
pub type U794 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B1>, B0>;
/// 794
pub type Ux31A = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B1>, B0>;
/// 794
pub type Ub1100011010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B1>, B0>;
/// 795
pub type U795 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B1>, B1>;
/// 795
pub type Ux31B = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B1>, B1>;
/// 795
pub type Ub1100011011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B1>, B1>;
/// 796
pub type U796 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B0>, B0>;
/// 796
pub type Ux31C = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B0>, B0>;
/// 796
pub type Ub1100011100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B0>, B0>;
/// 797
pub type U797 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B0>, B1>;
/// 797
pub type Ux31D = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B0>, B1>;
/// 797
pub type Ub1100011101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B0>, B1>;
/// 798
pub type U798 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B1>, B0>;
/// 798
pub type Ux31E = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B1>, B0>;
/// 798
pub type Ub1100011110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B1>, B0>;
/// 799
pub type U799 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B1>, B1>;
/// 799
pub type Ux31F = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B1>, B1>;
/// 799
pub type Ub1100011111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B1>, B1>;
/// 800
pub type U800 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>;
/// 800
pub type Ux320 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>;
/// 800
pub type Ub1100100000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>;
/// 801
pub type U801 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B1>;
/// 801
pub type Ux321 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B1>;
/// 801
pub type Ub1100100001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B1>;
/// 802
pub type U802 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B1>, B0>;
/// 802
pub type Ux322 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B1>, B0>;
/// 802
pub type Ub1100100010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B1>, B0>;
/// 803
pub type U803 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B1>, B1>;
/// 803
pub type Ux323 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B1>, B1>;
/// 803
pub type Ub1100100011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B1>, B1>;
/// 804
pub type U804 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B0>, B0>;
/// 804
pub type Ux324 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B0>, B0>;
/// 804
pub type Ub1100100100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B0>, B0>;
/// 805
pub type U805 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B0>, B1>;
/// 805
pub type Ux325 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B0>, B1>;
/// 805
pub type Ub1100100101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B0>, B1>;
/// 806
pub type U806 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B1>, B0>;
/// 806
pub type Ux326 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B1>, B0>;
/// 806
pub type Ub1100100110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B1>, B0>;
/// 807
pub type U807 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B1>, B1>;
/// 807
pub type Ux327 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B1>, B1>;
/// 807
pub type Ub1100100111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B1>, B1>;
/// 808
pub type U808 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B0>, B0>;
/// 808
pub type Ux328 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B0>, B0>;
/// 808
pub type Ub1100101000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B0>, B0>;
/// 809
pub type U809 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B0>, B1>;
/// 809
pub type Ux329 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B0>, B1>;
/// 809
pub type Ub1100101001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B0>, B1>;
/// 810
pub type U810 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B1>, B0>;
/// 810
pub type Ux32A = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B1>, B0>;
/// 810
pub type Ub1100101010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B1>, B0>;
/// 811
pub type U811 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B1>, B1>;
/// 811
pub type Ux32B = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B1>, B1>;
/// 811
pub type Ub1100101011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B1>, B1>;
/// 812
pub type U812 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B0>, B0>;
/// 812
pub type Ux32C = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B0>, B0>;
/// 812
pub type Ub1100101100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B0>, B0>;
/// 813
pub type U813 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B0>, B1>;
/// 813
pub type Ux32D = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B0>, B1>;
/// 813
pub type Ub1100101101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B0>, B1>;
/// 814
pub type U814 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B1>, B0>;
/// 814
pub type Ux32E = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B1>, B0>;
/// 814
pub type Ub1100101110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B1>, B0>;
/// 815
pub type U815 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B1>, B1>;
/// 815
pub type Ux32F = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B1>, B1>;
/// 815
pub type Ub1100101111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B1>, B1>;
/// 816
pub type U816 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B0>, B0>;
/// 816
pub type Ux330 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B0>, B0>;
/// 816
pub type Ub1100110000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B0>, B0>;
/// 817
pub type U817 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B0>, B1>;
/// 817
pub type Ux331 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B0>, B1>;
/// 817
pub type Ub1100110001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B0>, B1>;
/// 818
pub type U818 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B1>, B0>;
/// 818
pub type Ux332 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B1>, B0>;
/// 818
pub type Ub1100110010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B1>, B0>;
/// 819
pub type U819 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B1>, B1>;
/// 819
pub type Ux333 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B1>, B1>;
/// 819
pub type Ub1100110011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B1>, B1>;
/// 820
pub type U820 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B0>, B0>;
/// 820
pub type Ux334 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B0>, B0>;
/// 820
pub type Ub1100110100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B0>, B0>;
/// 821
pub type U821 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B0>, B1>;
/// 821
pub type Ux335 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B0>, B1>;
/// 821
pub type Ub1100110101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B0>, B1>;
/// 822
pub type U822 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B1>, B0>;
/// 822
pub type Ux336 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B1>, B0>;
/// 822
pub type Ub1100110110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B1>, B0>;
/// 823
pub type U823 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B1>, B1>;
/// 823
pub type Ux337 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B1>, B1>;
/// 823
pub type Ub1100110111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B1>, B1>;
/// 824
pub type U824 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B0>, B0>;
/// 824
pub type Ux338 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B0>, B0>;
/// 824
pub type Ub1100111000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B0>, B0>;
/// 825
pub type U825 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B0>, B1>;
/// 825
pub type Ux339 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B0>, B1>;
/// 825
pub type Ub1100111001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B0>, B1>;
/// 826
pub type U826 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B1>, B0>;
/// 826
pub type Ux33A = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B1>, B0>;
/// 826
pub type Ub1100111010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B1>, B0>;
/// 827
pub type U827 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B1>, B1>;
/// 827
pub type Ux33B = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B1>, B1>;
/// 827
pub type Ub1100111011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B1>, B1>;
/// 828
pub type U828 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B0>, B0>;
/// 828
pub type Ux33C = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B0>, B0>;
/// 828
pub type Ub1100111100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B0>, B0>;
/// 829
pub type U829 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B0>, B1>;
/// 829
pub type Ux33D = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B0>, B1>;
/// 829
pub type Ub1100111101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B0>, B1>;
/// 830
pub type U830 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B1>, B0>;
/// 830
pub type Ux33E = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B1>, B0>;
/// 830
pub type Ub1100111110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B1>, B0>;
/// 831
pub type U831 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B1>, B1>;
/// 831
pub type Ux33F = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B1>, B1>;
/// 831
pub type Ub1100111111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B1>, B1>;
/// 832
pub type U832 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 832
pub type Ux340 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 832
pub type Ub1101000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 833
pub type U833 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B1>;
/// 833
pub type Ux341 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B1>;
/// 833
pub type Ub1101000001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B1>;
/// 834
pub type U834 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B1>, B0>;
/// 834
pub type Ux342 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B1>, B0>;
/// 834
pub type Ub1101000010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B1>, B0>;
/// 835
pub type U835 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B1>, B1>;
/// 835
pub type Ux343 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B1>, B1>;
/// 835
pub type Ub1101000011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B1>, B1>;
/// 836
pub type U836 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B0>, B0>;
/// 836
pub type Ux344 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B0>, B0>;
/// 836
pub type Ub1101000100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B0>, B0>;
/// 837
pub type U837 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B0>, B1>;
/// 837
pub type Ux345 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B0>, B1>;
/// 837
pub type Ub1101000101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B0>, B1>;
/// 838
pub type U838 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B1>, B0>;
/// 838
pub type Ux346 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B1>, B0>;
/// 838
pub type Ub1101000110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B1>, B0>;
/// 839
pub type U839 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B1>, B1>;
/// 839
pub type Ux347 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B1>, B1>;
/// 839
pub type Ub1101000111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B1>, B1>;
/// 840
pub type U840 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B0>, B0>;
/// 840
pub type Ux348 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B0>, B0>;
/// 840
pub type Ub1101001000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B0>, B0>;
/// 841
pub type U841 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B0>, B1>;
/// 841
pub type Ux349 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B0>, B1>;
/// 841
pub type Ub1101001001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B0>, B1>;
/// 842
pub type U842 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B1>, B0>;
/// 842
pub type Ux34A = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B1>, B0>;
/// 842
pub type Ub1101001010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B1>, B0>;
/// 843
pub type U843 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B1>, B1>;
/// 843
pub type Ux34B = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B1>, B1>;
/// 843
pub type Ub1101001011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B1>, B1>;
/// 844
pub type U844 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B0>, B0>;
/// 844
pub type Ux34C = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B0>, B0>;
/// 844
pub type Ub1101001100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B0>, B0>;
/// 845
pub type U845 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B0>, B1>;
/// 845
pub type Ux34D = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B0>, B1>;
/// 845
pub type Ub1101001101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B0>, B1>;
/// 846
pub type U846 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B1>, B0>;
/// 846
pub type Ux34E = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B1>, B0>;
/// 846
pub type Ub1101001110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B1>, B0>;
/// 847
pub type U847 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B1>, B1>;
/// 847
pub type Ux34F = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B1>, B1>;
/// 847
pub type Ub1101001111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B1>, B1>;
/// 848
pub type U848 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B0>;
/// 848
pub type Ux350 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B0>;
/// 848
pub type Ub1101010000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B0>;
/// 849
pub type U849 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B1>;
/// 849
pub type Ux351 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B1>;
/// 849
pub type Ub1101010001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B1>;
/// 850
pub type U850 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B1>, B0>;
/// 850
pub type Ux352 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B1>, B0>;
/// 850
pub type Ub1101010010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B1>, B0>;
/// 851
pub type U851 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B1>, B1>;
/// 851
pub type Ux353 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B1>, B1>;
/// 851
pub type Ub1101010011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B1>, B1>;
/// 852
pub type U852 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B0>, B0>;
/// 852
pub type Ux354 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B0>, B0>;
/// 852
pub type Ub1101010100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B0>, B0>;
/// 853
pub type U853 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B0>, B1>;
/// 853
pub type Ux355 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B0>, B1>;
/// 853
pub type Ub1101010101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B0>, B1>;
/// 854
pub type U854 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B1>, B0>;
/// 854
pub type Ux356 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B1>, B0>;
/// 854
pub type Ub1101010110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B1>, B0>;
/// 855
pub type U855 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B1>, B1>;
/// 855
pub type Ux357 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B1>, B1>;
/// 855
pub type Ub1101010111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B1>, B1>;
/// 856
pub type U856 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B0>, B0>;
/// 856
pub type Ux358 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B0>, B0>;
/// 856
pub type Ub1101011000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B0>, B0>;
/// 857
pub type U857 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B0>, B1>;
/// 857
pub type Ux359 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B0>, B1>;
/// 857
pub type Ub1101011001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B0>, B1>;
/// 858
pub type U858 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B1>, B0>;
/// 858
pub type Ux35A = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B1>, B0>;
/// 858
pub type Ub1101011010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B1>, B0>;
/// 859
pub type U859 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B1>, B1>;
/// 859
pub type Ux35B = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B1>, B1>;
/// 859
pub type Ub1101011011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B1>, B1>;
/// 860
pub type U860 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B0>, B0>;
/// 860
pub type Ux35C = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B0>, B0>;
/// 860
pub type Ub1101011100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B0>, B0>;
/// 861
pub type U861 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B0>, B1>;
/// 861
pub type Ux35D = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B0>, B1>;
/// 861
pub type Ub1101011101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B0>, B1>;
/// 862
pub type U862 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B1>, B0>;
/// 862
pub type Ux35E = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B1>, B0>;
/// 862
pub type Ub1101011110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B1>, B0>;
/// 863
pub type U863 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B1>, B1>;
/// 863
pub type Ux35F = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B1>, B1>;
/// 863
pub type Ub1101011111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B1>, B1>;
/// 864
pub type U864 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B0>, B0>;
/// 864
pub type Ux360 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B0>, B0>;
/// 864
pub type Ub1101100000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B0>, B0>;
/// 865
pub type U865 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B0>, B1>;
/// 865
pub type Ux361 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B0>, B1>;
/// 865
pub type Ub1101100001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B0>, B1>;
/// 866
pub type U866 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B1>, B0>;
/// 866
pub type Ux362 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B1>, B0>;
/// 866
pub type Ub1101100010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B1>, B0>;
/// 867
pub type U867 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B1>, B1>;
/// 867
pub type Ux363 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B1>, B1>;
/// 867
pub type Ub1101100011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B1>, B1>;
/// 868
pub type U868 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B0>, B0>;
/// 868
pub type Ux364 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B0>, B0>;
/// 868
pub type Ub1101100100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B0>, B0>;
/// 869
pub type U869 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B0>, B1>;
/// 869
pub type Ux365 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B0>, B1>;
/// 869
pub type Ub1101100101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B0>, B1>;
/// 870
pub type U870 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B1>, B0>;
/// 870
pub type Ux366 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B1>, B0>;
/// 870
pub type Ub1101100110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B1>, B0>;
/// 871
pub type U871 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B1>, B1>;
/// 871
pub type Ux367 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B1>, B1>;
/// 871
pub type Ub1101100111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B1>, B1>;
/// 872
pub type U872 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B0>, B0>;
/// 872
pub type Ux368 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B0>, B0>;
/// 872
pub type Ub1101101000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B0>, B0>;
/// 873
pub type U873 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B0>, B1>;
/// 873
pub type Ux369 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B0>, B1>;
/// 873
pub type Ub1101101001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B0>, B1>;
/// 874
pub type U874 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B0>;
/// 874
pub type Ux36A = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B0>;
/// 874
pub type Ub1101101010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B0>;
/// 875
pub type U875 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B1>;
/// 875
pub type Ux36B = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B1>;
/// 875
pub type Ub1101101011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B1>;
/// 876
pub type U876 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B0>, B0>;
/// 876
pub type Ux36C = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B0>, B0>;
/// 876
pub type Ub1101101100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B0>, B0>;
/// 877
pub type U877 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B0>, B1>;
/// 877
pub type Ux36D = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B0>, B1>;
/// 877
pub type Ub1101101101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B0>, B1>;
/// 878
pub type U878 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B1>, B0>;
/// 878
pub type Ux36E = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B1>, B0>;
/// 878
pub type Ub1101101110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B1>, B0>;
/// 879
pub type U879 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B1>, B1>;
/// 879
pub type Ux36F = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B1>, B1>;
/// 879
pub type Ub1101101111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B1>, B1>;
/// 880
pub type U880 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B0>, B0>;
/// 880
pub type Ux370 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B0>, B0>;
/// 880
pub type Ub1101110000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B0>, B0>;
/// 881
pub type U881 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B0>, B1>;
/// 881
pub type Ux371 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B0>, B1>;
/// 881
pub type Ub1101110001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B0>, B1>;
/// 882
pub type U882 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B1>, B0>;
/// 882
pub type Ux372 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B1>, B0>;
/// 882
pub type Ub1101110010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B1>, B0>;
/// 883
pub type U883 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B1>, B1>;
/// 883
pub type Ux373 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B1>, B1>;
/// 883
pub type Ub1101110011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B1>, B1>;
/// 884
pub type U884 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B0>, B0>;
/// 884
pub type Ux374 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B0>, B0>;
/// 884
pub type Ub1101110100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B0>, B0>;
/// 885
pub type U885 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B0>, B1>;
/// 885
pub type Ux375 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B0>, B1>;
/// 885
pub type Ub1101110101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B0>, B1>;
/// 886
pub type U886 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B1>, B0>;
/// 886
pub type Ux376 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B1>, B0>;
/// 886
pub type Ub1101110110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B1>, B0>;
/// 887
pub type U887 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B1>, B1>;
/// 887
pub type Ux377 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B1>, B1>;
/// 887
pub type Ub1101110111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B1>, B1>;
/// 888
pub type U888 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B0>;
/// 888
pub type Ux378 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B0>;
/// 888
pub type Ub1101111000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B0>;
/// 889
pub type U889 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B1>;
/// 889
pub type Ux379 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B1>;
/// 889
pub type Ub1101111001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B1>;
/// 890
pub type U890 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B1>, B0>;
/// 890
pub type Ux37A = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B1>, B0>;
/// 890
pub type Ub1101111010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B1>, B0>;
/// 891
pub type U891 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B1>, B1>;
/// 891
pub type Ux37B = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B1>, B1>;
/// 891
pub type Ub1101111011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B1>, B1>;
/// 892
pub type U892 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B0>, B0>;
/// 892
pub type Ux37C = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B0>, B0>;
/// 892
pub type Ub1101111100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B0>, B0>;
/// 893
pub type U893 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B0>, B1>;
/// 893
pub type Ux37D = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B0>, B1>;
/// 893
pub type Ub1101111101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B0>, B1>;
/// 894
pub type U894 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B1>, B0>;
/// 894
pub type Ux37E = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B1>, B0>;
/// 894
pub type Ub1101111110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B1>, B0>;
/// 895
pub type U895 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B1>, B1>;
/// 895
pub type Ux37F = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B1>, B1>;
/// 895
pub type Ub1101111111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B1>, B1>;
/// 896
pub type U896 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 896
pub type Ux380 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 896
pub type Ub1110000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 897
pub type U897 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B1>;
/// 897
pub type Ux381 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B1>;
/// 897
pub type Ub1110000001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B1>;
/// 898
pub type U898 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B0>;
/// 898
pub type Ux382 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B0>;
/// 898
pub type Ub1110000010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B0>;
/// 899
pub type U899 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B1>;
/// 899
pub type Ux383 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B1>;
/// 899
pub type Ub1110000011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B1>;
/// 900
pub type U900 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B0>;
/// 900
pub type Ux384 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B0>;
/// 900
pub type Ub1110000100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B0>;
/// 901
pub type U901 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B1>;
/// 901
pub type Ux385 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B1>;
/// 901
pub type Ub1110000101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B1>;
/// 902
pub type U902 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B0>;
/// 902
pub type Ux386 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B0>;
/// 902
pub type Ub1110000110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B0>;
/// 903
pub type U903 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B1>;
/// 903
pub type Ux387 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B1>;
/// 903
pub type Ub1110000111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B1>;
/// 904
pub type U904 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B0>;
/// 904
pub type Ux388 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B0>;
/// 904
pub type Ub1110001000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B0>;
/// 905
pub type U905 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B1>;
/// 905
pub type Ux389 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B1>;
/// 905
pub type Ub1110001001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B1>;
/// 906
pub type U906 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B0>;
/// 906
pub type Ux38A = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B0>;
/// 906
pub type Ub1110001010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B0>;
/// 907
pub type U907 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B1>;
/// 907
pub type Ux38B = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B1>;
/// 907
pub type Ub1110001011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B1>;
/// 908
pub type U908 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B0>;
/// 908
pub type Ux38C = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B0>;
/// 908
pub type Ub1110001100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B0>;
/// 909
pub type U909 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B1>;
/// 909
pub type Ux38D = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B1>;
/// 909
pub type Ub1110001101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B1>;
/// 910
pub type U910 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B0>;
/// 910
pub type Ux38E = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B0>;
/// 910
pub type Ub1110001110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B0>;
/// 911
pub type U911 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B1>;
/// 911
pub type Ux38F = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B1>;
/// 911
pub type Ub1110001111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B1>;
/// 912
pub type U912 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>;
/// 912
pub type Ux390 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>;
/// 912
pub type Ub1110010000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>;
/// 913
pub type U913 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B1>;
/// 913
pub type Ux391 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B1>;
/// 913
pub type Ub1110010001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B1>;
/// 914
pub type U914 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B0>;
/// 914
pub type Ux392 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B0>;
/// 914
pub type Ub1110010010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B0>;
/// 915
pub type U915 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B1>;
/// 915
pub type Ux393 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B1>;
/// 915
pub type Ub1110010011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B1>;
/// 916
pub type U916 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B0>;
/// 916
pub type Ux394 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B0>;
/// 916
pub type Ub1110010100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B0>;
/// 917
pub type U917 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B1>;
/// 917
pub type Ux395 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B1>;
/// 917
pub type Ub1110010101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B1>;
/// 918
pub type U918 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B0>;
/// 918
pub type Ux396 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B0>;
/// 918
pub type Ub1110010110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B0>;
/// 919
pub type U919 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B1>;
/// 919
pub type Ux397 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B1>;
/// 919
pub type Ub1110010111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B1>;
/// 920
pub type U920 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B0>;
/// 920
pub type Ux398 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B0>;
/// 920
pub type Ub1110011000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B0>;
/// 921
pub type U921 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B1>;
/// 921
pub type Ux399 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B1>;
/// 921
pub type Ub1110011001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B1>;
/// 922
pub type U922 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B0>;
/// 922
pub type Ux39A = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B0>;
/// 922
pub type Ub1110011010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B0>;
/// 923
pub type U923 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B1>;
/// 923
pub type Ux39B = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B1>;
/// 923
pub type Ub1110011011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B1>;
/// 924
pub type U924 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B0>;
/// 924
pub type Ux39C = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B0>;
/// 924
pub type Ub1110011100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B0>;
/// 925
pub type U925 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B1>;
/// 925
pub type Ux39D = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B1>;
/// 925
pub type Ub1110011101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B1>;
/// 926
pub type U926 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B0>;
/// 926
pub type Ux39E = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B0>;
/// 926
pub type Ub1110011110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B0>;
/// 927
pub type U927 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B1>;
/// 927
pub type Ux39F = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B1>;
/// 927
pub type Ub1110011111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B1>;
/// 928
pub type U928 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>;
/// 928
pub type Ux3A0 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>;
/// 928
pub type Ub1110100000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>;
/// 929
pub type U929 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B1>;
/// 929
pub type Ux3A1 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B1>;
/// 929
pub type Ub1110100001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B1>;
/// 930
pub type U930 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B0>;
/// 930
pub type Ux3A2 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B0>;
/// 930
pub type Ub1110100010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B0>;
/// 931
pub type U931 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B1>;
/// 931
pub type Ux3A3 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B1>;
/// 931
pub type Ub1110100011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B1>;
/// 932
pub type U932 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B0>;
/// 932
pub type Ux3A4 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B0>;
/// 932
pub type Ub1110100100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B0>;
/// 933
pub type U933 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B1>;
/// 933
pub type Ux3A5 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B1>;
/// 933
pub type Ub1110100101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B1>;
/// 934
pub type U934 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B0>;
/// 934
pub type Ux3A6 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B0>;
/// 934
pub type Ub1110100110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B0>;
/// 935
pub type U935 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B1>;
/// 935
pub type Ux3A7 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B1>;
/// 935
pub type Ub1110100111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B1>;
/// 936
pub type U936 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B0>;
/// 936
pub type Ux3A8 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B0>;
/// 936
pub type Ub1110101000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B0>;
/// 937
pub type U937 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B1>;
/// 937
pub type Ux3A9 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B1>;
/// 937
pub type Ub1110101001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B1>;
/// 938
pub type U938 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B0>;
/// 938
pub type Ux3AA = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B0>;
/// 938
pub type Ub1110101010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B0>;
/// 939
pub type U939 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B1>;
/// 939
pub type Ux3AB = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B1>;
/// 939
pub type Ub1110101011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B1>;
/// 940
pub type U940 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B0>;
/// 940
pub type Ux3AC = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B0>;
/// 940
pub type Ub1110101100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B0>;
/// 941
pub type U941 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B1>;
/// 941
pub type Ux3AD = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B1>;
/// 941
pub type Ub1110101101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B1>;
/// 942
pub type U942 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B0>;
/// 942
pub type Ux3AE = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B0>;
/// 942
pub type Ub1110101110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B0>;
/// 943
pub type U943 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B1>;
/// 943
pub type Ux3AF = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B1>;
/// 943
pub type Ub1110101111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B1>;
/// 944
pub type U944 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B0>;
/// 944
pub type Ux3B0 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B0>;
/// 944
pub type Ub1110110000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B0>;
/// 945
pub type U945 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B1>;
/// 945
pub type Ux3B1 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B1>;
/// 945
pub type Ub1110110001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B1>;
/// 946
pub type U946 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B0>;
/// 946
pub type Ux3B2 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B0>;
/// 946
pub type Ub1110110010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B0>;
/// 947
pub type U947 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B1>;
/// 947
pub type Ux3B3 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B1>;
/// 947
pub type Ub1110110011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B1>;
/// 948
pub type U948 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B0>;
/// 948
pub type Ux3B4 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B0>;
/// 948
pub type Ub1110110100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B0>;
/// 949
pub type U949 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B1>;
/// 949
pub type Ux3B5 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B1>;
/// 949
pub type Ub1110110101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B1>;
/// 950
pub type U950 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B0>;
/// 950
pub type Ux3B6 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B0>;
/// 950
pub type Ub1110110110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B0>;
/// 951
pub type U951 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B1>;
/// 951
pub type Ux3B7 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B1>;
/// 951
pub type Ub1110110111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B1>;
/// 952
pub type U952 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B0>;
/// 952
pub type Ux3B8 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B0>;
/// 952
pub type Ub1110111000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B0>;
/// 953
pub type U953 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B1>;
/// 953
pub type Ux3B9 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B1>;
/// 953
pub type Ub1110111001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B1>;
/// 954
pub type U954 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B0>;
/// 954
pub type Ux3BA = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B0>;
/// 954
pub type Ub1110111010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B0>;
/// 955
pub type U955 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B1>;
/// 955
pub type Ux3BB = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B1>;
/// 955
pub type Ub1110111011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B1>;
/// 956
pub type U956 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B0>;
/// 956
pub type Ux3BC = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B0>;
/// 956
pub type Ub1110111100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B0>;
/// 957
pub type U957 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B1>;
/// 957
pub type Ux3BD = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B1>;
/// 957
pub type Ub1110111101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B1>;
/// 958
pub type U958 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B0>;
/// 958
pub type Ux3BE = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B0>;
/// 958
pub type Ub1110111110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B0>;
/// 959
pub type U959 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B1>;
/// 959
pub type Ux3BF = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B1>;
/// 959
pub type Ub1110111111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B1>;
/// 960
pub type U960 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 960
pub type Ux3C0 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 960
pub type Ub1111000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 961
pub type U961 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B1>;
/// 961
pub type Ux3C1 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B1>;
/// 961
pub type Ub1111000001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B1>;
/// 962
pub type U962 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B0>;
/// 962
pub type Ux3C2 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B0>;
/// 962
pub type Ub1111000010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B0>;
/// 963
pub type U963 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B1>;
/// 963
pub type Ux3C3 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B1>;
/// 963
pub type Ub1111000011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B1>;
/// 964
pub type U964 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B0>;
/// 964
pub type Ux3C4 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B0>;
/// 964
pub type Ub1111000100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B0>;
/// 965
pub type U965 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B1>;
/// 965
pub type Ux3C5 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B1>;
/// 965
pub type Ub1111000101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B1>;
/// 966
pub type U966 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B0>;
/// 966
pub type Ux3C6 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B0>;
/// 966
pub type Ub1111000110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B0>;
/// 967
pub type U967 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B1>;
/// 967
pub type Ux3C7 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B1>;
/// 967
pub type Ub1111000111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B1>;
/// 968
pub type U968 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B0>;
/// 968
pub type Ux3C8 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B0>;
/// 968
pub type Ub1111001000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B0>;
/// 969
pub type U969 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B1>;
/// 969
pub type Ux3C9 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B1>;
/// 969
pub type Ub1111001001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B1>;
/// 970
pub type U970 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B0>;
/// 970
pub type Ux3CA = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B0>;
/// 970
pub type Ub1111001010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B0>;
/// 971
pub type U971 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B1>;
/// 971
pub type Ux3CB = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B1>;
/// 971
pub type Ub1111001011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B1>;
/// 972
pub type U972 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B0>;
/// 972
pub type Ux3CC = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B0>;
/// 972
pub type Ub1111001100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B0>;
/// 973
pub type U973 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B1>;
/// 973
pub type Ux3CD = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B1>;
/// 973
pub type Ub1111001101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B1>;
/// 974
pub type U974 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B0>;
/// 974
pub type Ux3CE = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B0>;
/// 974
pub type Ub1111001110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B0>;
/// 975
pub type U975 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B1>;
/// 975
pub type Ux3CF = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B1>;
/// 975
pub type Ub1111001111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B1>;
/// 976
pub type U976 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>;
/// 976
pub type Ux3D0 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>;
/// 976
pub type Ub1111010000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>;
/// 977
pub type U977 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B1>;
/// 977
pub type Ux3D1 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B1>;
/// 977
pub type Ub1111010001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B1>;
/// 978
pub type U978 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B0>;
/// 978
pub type Ux3D2 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B0>;
/// 978
pub type Ub1111010010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B0>;
/// 979
pub type U979 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B1>;
/// 979
pub type Ux3D3 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B1>;
/// 979
pub type Ub1111010011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B1>;
/// 980
pub type U980 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B0>;
/// 980
pub type Ux3D4 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B0>;
/// 980
pub type Ub1111010100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B0>;
/// 981
pub type U981 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B1>;
/// 981
pub type Ux3D5 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B1>;
/// 981
pub type Ub1111010101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B1>;
/// 982
pub type U982 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B0>;
/// 982
pub type Ux3D6 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B0>;
/// 982
pub type Ub1111010110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B0>;
/// 983
pub type U983 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B1>;
/// 983
pub type Ux3D7 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B1>;
/// 983
pub type Ub1111010111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B1>;
/// 984
pub type U984 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B0>;
/// 984
pub type Ux3D8 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B0>;
/// 984
pub type Ub1111011000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B0>;
/// 985
pub type U985 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B1>;
/// 985
pub type Ux3D9 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B1>;
/// 985
pub type Ub1111011001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B1>;
/// 986
pub type U986 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B0>;
/// 986
pub type Ux3DA = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B0>;
/// 986
pub type Ub1111011010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B0>;
/// 987
pub type U987 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B1>;
/// 987
pub type Ux3DB = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B1>;
/// 987
pub type Ub1111011011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B1>;
/// 988
pub type U988 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B0>;
/// 988
pub type Ux3DC = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B0>;
/// 988
pub type Ub1111011100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B0>;
/// 989
pub type U989 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B1>;
/// 989
pub type Ux3DD = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B1>;
/// 989
pub type Ub1111011101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B1>;
/// 990
pub type U990 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B0>;
/// 990
pub type Ux3DE = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B0>;
/// 990
pub type Ub1111011110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B0>;
/// 991
pub type U991 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B1>;
/// 991
pub type Ux3DF = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B1>;
/// 991
pub type Ub1111011111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B1>;
/// 992
pub type U992 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B0>;
/// 992
pub type Ux3E0 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B0>;
/// 992
pub type Ub1111100000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B0>;
/// 993
pub type U993 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B1>;
/// 993
pub type Ux3E1 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B1>;
/// 993
pub type Ub1111100001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B1>;
/// 994
pub type U994 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B0>;
/// 994
pub type Ux3E2 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B0>;
/// 994
pub type Ub1111100010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B0>;
/// 995
pub type U995 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B1>;
/// 995
pub type Ux3E3 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B1>;
/// 995
pub type Ub1111100011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B1>;
/// 996
pub type U996 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B0>;
/// 996
pub type Ux3E4 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B0>;
/// 996
pub type Ub1111100100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B0>;
/// 997
pub type U997 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B1>;
/// 997
pub type Ux3E5 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B1>;
/// 997
pub type Ub1111100101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B1>;
/// 998
pub type U998 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B0>;
/// 998
pub type Ux3E6 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B0>;
/// 998
pub type Ub1111100110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B0>;
/// 999
pub type U999 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B1>;
/// 999
pub type Ux3E7 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B1>;
/// 999
pub type Ub1111100111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B1>;
/// 1000
pub type U1000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B0>;
/// 1000
pub type Ux3E8 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B0>;
/// 1000
pub type Ub1111101000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B0>;
/// 0
pub type U10pow0 = UInt<UTerm, B1>;
/// 1
pub type U10pow1 = UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>;
/// 2
pub type U10pow2 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B0>;
/// 3
pub type U10pow3 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B0>;
/// 4
pub type U10pow4 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B0>;
/// 4
pub type U10000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B0>;
/// 4
pub type Ux2710 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B0>;
/// 4
pub type Ub10011100010000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B0>;
/// 5
pub type U10pow5 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>;
/// 5
pub type U100000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>;
/// 5
pub type Ux186A0 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>;
/// 5
pub type Ub11000011010100000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>;
/// 6
pub type U10pow6 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 6
pub type U1000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 6
pub type UxF4240 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 6
pub type Ub11110100001001000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 7
pub type U10pow7 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 7
pub type U10000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 7
pub type Ux989680 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 7
pub type Ub100110001001011010000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 8
pub type U10pow8 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 8
pub type U100000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 8
pub type Ux5F5E100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 8
pub type Ub101111101011110000100000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 9
pub type U10pow9 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 9
pub type U1000000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 9
pub type Ux3B9ACA00 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 9
pub type Ub111011100110101100101000000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 10
pub type U10pow10 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 10
pub type U10000000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 10
pub type Ux2540BE400 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 10
pub type Ub1001010100000010111110010000000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 11
pub type U10pow11 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 11
pub type U100000000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 11
pub type Ux174876E800 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 11
pub type Ub1011101001000011101101110100000000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 12
pub type U10pow12 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 12
pub type U1000000000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 12
pub type UxE8D4A51000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 12
pub type Ub1110100011010100101001010001000000000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 13
pub type U10pow13 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 13
pub type U10000000000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 13
pub type Ux9184E72A000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 13
pub type Ub10010001100001001110011100101010000000000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 14
pub type U10pow14 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 14
pub type U100000000000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 14
pub type Ux5AF3107A4000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 14
pub type Ub10110101111001100010000011110100100000000000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 15
pub type U10pow15 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 15
pub type U1000000000000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 15
pub type Ux38D7EA4C68000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 15
pub type Ub11100011010111111010100100110001101000000000000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 16
pub type U10pow16 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 16
pub type U10000000000000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 16
pub type Ux2386F26FC10000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 16
pub type Ub100011100001101111001001101111110000010000000000000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 17
pub type U10pow17 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 17
pub type U100000000000000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 17
pub type Ux16345785D8A0000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 17
pub type Ub101100011010001010111100001011101100010100000000000000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 18
pub type U10pow18 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 18
pub type U1000000000000000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 18
pub type UxDE0B6B3A7640000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 18
pub type Ub110111100000101101101011001110100111011001000000000000000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 19
pub type U10pow19 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 19
pub type U10000000000000000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 19
pub type Ux8AC7230489E80000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 19
pub type Ub1000101011000111001000110000010010001001111010000000000000000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 20
pub type U10pow20 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 20
pub type U100000000000000000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 20
pub type Ux56BC75E2D63100000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 20
pub type Ub1010110101111000111010111100010110101100011000100000000000000000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 21
pub type U10pow21 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 21
pub type U1000000000000000000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 21
pub type Ux3635C9ADC5DEA00000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 21
pub type Ub1101100011010111001001101011011100010111011110101000000000000000000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 22
pub type U10pow22 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 22
pub type U10000000000000000000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 22
pub type Ux21E19E0C9BAB2400000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 22
pub type Ub10000111100001100111100000110010011011101010110010010000000000000000000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 23
pub type U10pow23 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 23
pub type U100000000000000000000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 23
pub type Ux152D02C7E14AF6800000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 23
pub type Ub10101001011010000001011000111111000010100101011110110100000000000000000000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 24
pub type U10pow24 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 24
pub type U1000000000000000000000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 24
pub type UxD3C21BCECCEDA1000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 24
pub type Ub11010011110000100001101111001110110011001110110110100001000000000000000000000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 25
pub type U10pow25 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 25
pub type U10000000000000000000000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 25
pub type Ux84595161401484A000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 25
pub type Ub100001000101100101010001011000010100000000010100100001001010000000000000000000000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 26
pub type U10pow26 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 26
pub type U100000000000000000000000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 26
pub type Ux52B7D2DCC80CD2E4000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 26
pub type Ub101001010110111110100101101110011001000000011001101001011100100000000000000000000000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 27
pub type U10pow27 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 27
pub type U1000000000000000000000000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 27
pub type Ux33B2E3C9FD0803CE8000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 27
pub type Ub110011101100101110001111001001111111010000100000000011110011101000000000000000000000000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 28
pub type U10pow28 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 28
pub type U10000000000000000000000000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 28
pub type Ux204FCE5E3E25026110000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 28
pub type Ub1000000100111111001110010111100011111000100101000000100110000100010000000000000000000000000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 29
pub type U10pow29 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 29
pub type U100000000000000000000000000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 29
pub type Ux1431E0FAE6D7217CAA0000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 29
pub type Ub1010000110001111000001111101011100110110101110010000101111100101010100000000000000000000000000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 30
pub type U10pow30 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 30
pub type U1000000000000000000000000000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 30
pub type UxC9F2C9CD04674EDEA40000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 30
pub type Ub1100100111110010110010011100110100000100011001110100111011011110101001000000000000000000000000000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 31
pub type U10pow31 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 31
pub type U10000000000000000000000000000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 31
pub type Ux7E37BE2022C0914B2680000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 31
pub type Ub1111110001101111011111000100000001000101100000010010001010010110010011010000000000000000000000000000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 32
pub type U10pow32 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 32
pub type U100000000000000000000000000000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 32
pub type Ux4EE2D6D415B85ACEF8100000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 32
pub type Ub10011101110001011010110110101000001010110111000010110101100111011111000000100000000000000000000000000000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 33
pub type U10pow33 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 33
pub type U1000000000000000000000000000000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 33
pub type Ux314DC6448D9338C15B0A00000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 33
pub type Ub11000101001101110001100100010010001101100100110011100011000001010110110000101000000000000000000000000000000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 34
pub type U10pow34 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 34
pub type U10000000000000000000000000000000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 34
pub type Ux1ED09BEAD87C0378D8E6400000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 34
pub type Ub11110110100001001101111101010110110000111110000000011011110001101100011100110010000000000000000000000000000000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 35
pub type U10pow35 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 35
pub type U100000000000000000000000000000000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 35
pub type Ux13426172C74D822B878FE800000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 35
pub type Ub100110100001001100001011100101100011101001101100000100010101110000111100011111110100000000000000000000000000000000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 36
pub type U10pow36 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 36
pub type U1000000000000000000000000000000000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 36
pub type UxC097CE7BC90715B34B9F1000000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 36
pub type Ub110000001001011111001110011110111100100100000111000101011011001101001011100111110001000000000000000000000000000000000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 37
pub type U10pow37 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 37
pub type U10000000000000000000000000000000000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 37
pub type Ux785EE10D5DA46D900F436A000000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 37
pub type Ub111100001011110111000010000110101011101101001000110110110010000000011110100001101101010000000000000000000000000000000000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 38
pub type U10pow38 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 38
pub type U100000000000000000000000000000000000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 38
pub type Ux4B3B4CA85A86C47A098A224000000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 38
pub type Ub1001011001110110100110010101000010110101000011011000100011110100000100110001010001000100100000000000000000000000000000000000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 0
pub type U2pow0 = UInt<UTerm, B1>;
/// 1
pub type U2pow1 = UInt<UInt<UTerm, B1>, B0>;
/// 2
pub type U2pow2 = UInt<UInt<UInt<UTerm, B1>, B0>, B0>;
/// 3
pub type U2pow3 = UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>;
/// 4
pub type U2pow4 = UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>;
/// 5
pub type U2pow5 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>;
/// 6
pub type U2pow6 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 7
pub type U2pow7 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 8
pub type U2pow8 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 9
pub type U2pow9 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 10
pub type U2pow10 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 10
pub type U1024 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 11
pub type U2pow11 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 11
pub type U2048 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 12
pub type U2pow12 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 12
pub type U4096 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 13
pub type U2pow13 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 13
pub type U8192 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 14
pub type U2pow14 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 14
pub type U16384 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 15
pub type U2pow15 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 15
pub type U32768 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 16
pub type U2pow16 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 16
pub type U65536 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 17
pub type U2pow17 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 17
pub type U131072 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 18
pub type U2pow18 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 18
pub type U262144 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 19
pub type U2pow19 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 19
pub type U524288 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 20
pub type U2pow20 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 20
pub type U1048576 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 21
pub type U2pow21 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 21
pub type U2097152 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 22
pub type U2pow22 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 22
pub type U4194304 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 23
pub type U2pow23 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 23
pub type U8388608 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 24
pub type U2pow24 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 24
pub type U16777216 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 25
pub type U2pow25 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 25
pub type U33554432 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 26
pub type U2pow26 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 26
pub type U67108864 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 27
pub type U2pow27 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 27
pub type U134217728 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 28
pub type U2pow28 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 28
pub type U268435456 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 29
pub type U2pow29 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 29
pub type U536870912 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 30
pub type U2pow30 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 30
pub type U1073741824 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 31
pub type U2pow31 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 31
pub type U2147483648 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 32
pub type U2pow32 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 32
pub type U4294967296 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 33
pub type U2pow33 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 33
pub type U8589934592 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 34
pub type U2pow34 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 34
pub type U17179869184 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 35
pub type U2pow35 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 35
pub type U34359738368 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 36
pub type U2pow36 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 36
pub type U68719476736 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 37
pub type U2pow37 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 37
pub type U137438953472 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 38
pub type U2pow38 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 38
pub type U274877906944 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 39
pub type U2pow39 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 39
pub type U549755813888 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 40
pub type U2pow40 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 40
pub type U1099511627776 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 41
pub type U2pow41 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 41
pub type U2199023255552 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 42
pub type U2pow42 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 42
pub type U4398046511104 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 43
pub type U2pow43 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 43
pub type U8796093022208 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 44
pub type U2pow44 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 44
pub type U17592186044416 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 45
pub type U2pow45 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 45
pub type U35184372088832 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 46
pub type U2pow46 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 46
pub type U70368744177664 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 47
pub type U2pow47 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 47
pub type U140737488355328 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 48
pub type U2pow48 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 48
pub type U281474976710656 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 49
pub type U2pow49 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 49
pub type U562949953421312 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 50
pub type U2pow50 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 50
pub type U1125899906842624 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 51
pub type U2pow51 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 51
pub type U2251799813685248 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 52
pub type U2pow52 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 52
pub type U4503599627370496 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 53
pub type U2pow53 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 53
pub type U9007199254740992 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 54
pub type U2pow54 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 54
pub type U18014398509481984 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 55
pub type U2pow55 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 55
pub type U36028797018963968 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 56
pub type U2pow56 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 56
pub type U72057594037927936 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 57
pub type U2pow57 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 57
pub type U144115188075855872 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 58
pub type U2pow58 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 58
pub type U288230376151711744 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 59
pub type U2pow59 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 59
pub type U576460752303423488 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 60
pub type U2pow60 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 60
pub type U1152921504606846976 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 61
pub type U2pow61 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 61
pub type U2305843009213693952 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 62
pub type U2pow62 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 62
pub type U4611686018427387904 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 63
pub type U2pow63 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 63
pub type U9223372036854775808 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 64
pub type U2pow64 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 64
pub type U18446744073709551616 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 65
pub type U2pow65 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 65
pub type U36893488147419103232 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 66
pub type U2pow66 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 66
pub type U73786976294838206464 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 67
pub type U2pow67 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 67
pub type U147573952589676412928 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 68
pub type U2pow68 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 68
pub type U295147905179352825856 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 69
pub type U2pow69 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 69
pub type U590295810358705651712 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 70
pub type U2pow70 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 70
pub type U1180591620717411303424 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 71
pub type U2pow71 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 71
pub type U2361183241434822606848 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 72
pub type U2pow72 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 72
pub type U4722366482869645213696 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 73
pub type U2pow73 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 73
pub type U9444732965739290427392 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 74
pub type U2pow74 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 74
pub type U18889465931478580854784 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 75
pub type U2pow75 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 75
pub type U37778931862957161709568 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 76
pub type U2pow76 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 76
pub type U75557863725914323419136 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 77
pub type U2pow77 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 77
pub type U151115727451828646838272 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 78
pub type U2pow78 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 78
pub type U302231454903657293676544 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 79
pub type U2pow79 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 79
pub type U604462909807314587353088 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 80
pub type U2pow80 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 80
pub type U1208925819614629174706176 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 81
pub type U2pow81 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 81
pub type U2417851639229258349412352 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 82
pub type U2pow82 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 82
pub type U4835703278458516698824704 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 83
pub type U2pow83 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 83
pub type U9671406556917033397649408 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 84
pub type U2pow84 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 84
pub type U19342813113834066795298816 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 85
pub type U2pow85 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 85
pub type U38685626227668133590597632 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 86
pub type U2pow86 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 86
pub type U77371252455336267181195264 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 87
pub type U2pow87 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 87
pub type U154742504910672534362390528 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 88
pub type U2pow88 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 88
pub type U309485009821345068724781056 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 89
pub type U2pow89 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 89
pub type U618970019642690137449562112 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 90
pub type U2pow90 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 90
pub type U1237940039285380274899124224 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 91
pub type U2pow91 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 91
pub type U2475880078570760549798248448 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 92
pub type U2pow92 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 92
pub type U4951760157141521099596496896 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 93
pub type U2pow93 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 93
pub type U9903520314283042199192993792 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 94
pub type U2pow94 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 94
pub type U19807040628566084398385987584 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 95
pub type U2pow95 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 95
pub type U39614081257132168796771975168 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 96
pub type U2pow96 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 96
pub type U79228162514264337593543950336 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 97
pub type U2pow97 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 97
pub type U158456325028528675187087900672 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 98
pub type U2pow98 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 98
pub type U316912650057057350374175801344 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 99
pub type U2pow99 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 99
pub type U633825300114114700748351602688 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 100
pub type U2pow100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 100
pub type U1267650600228229401496703205376 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 101
pub type U2pow101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 101
pub type U2535301200456458802993406410752 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 102
pub type U2pow102 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 102
pub type U5070602400912917605986812821504 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 103
pub type U2pow103 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 103
pub type U10141204801825835211973625643008 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 104
pub type U2pow104 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 104
pub type U20282409603651670423947251286016 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 105
pub type U2pow105 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 105
pub type U40564819207303340847894502572032 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 106
pub type U2pow106 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 106
pub type U81129638414606681695789005144064 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 107
pub type U2pow107 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 107
pub type U162259276829213363391578010288128 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 108
pub type U2pow108 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 108
pub type U324518553658426726783156020576256 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 109
pub type U2pow109 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 109
pub type U649037107316853453566312041152512 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 110
pub type U2pow110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 110
pub type U1298074214633706907132624082305024 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 111
pub type U2pow111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 111
pub type U2596148429267413814265248164610048 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 112
pub type U2pow112 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 112
pub type U5192296858534827628530496329220096 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 113
pub type U2pow113 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 113
pub type U10384593717069655257060992658440192 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 114
pub type U2pow114 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 114
pub type U20769187434139310514121985316880384 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 115
pub type U2pow115 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 115
pub type U41538374868278621028243970633760768 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 116
pub type U2pow116 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 116
pub type U83076749736557242056487941267521536 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 117
pub type U2pow117 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 117
pub type U166153499473114484112975882535043072 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 118
pub type U2pow118 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 118
pub type U332306998946228968225951765070086144 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 119
pub type U2pow119 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 119
pub type U664613997892457936451903530140172288 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 120
pub type U2pow120 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 120
pub type U1329227995784915872903807060280344576 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 121
pub type U2pow121 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 121
pub type U2658455991569831745807614120560689152 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 122
pub type U2pow122 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 122
pub type U5316911983139663491615228241121378304 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 123
pub type U2pow123 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 123
pub type U10633823966279326983230456482242756608 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 124
pub type U2pow124 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 124
pub type U21267647932558653966460912964485513216 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 125
pub type U2pow125 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 125
pub type U42535295865117307932921825928971026432 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 126
pub type U2pow126 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 126
pub type U85070591730234615865843651857942052864 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 127
pub type U2pow127 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
/// 127
pub type U170141183460469231731687303715884105728 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
