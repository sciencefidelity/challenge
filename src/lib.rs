#![allow(
    dead_code,
    clippy::must_use_candidate,
    clippy::unwrap_used,
    clippy::missing_panics_doc
)]
pub mod pr_0001;
pub mod pr_0002;
pub mod pr_0003;
pub mod pr_0006;
pub mod pr_0007;
pub mod pr_0008;
pub mod pr_0009;
pub mod pr_0011;
pub mod pr_0012;
pub mod pr_0013;
pub mod pr_0014;
pub mod pr_0015;
pub mod pr_0017;
pub mod pr_0019;
pub mod pr_0020;
pub mod pr_0021;
pub mod pr_0022;
pub mod pr_0026;
pub mod pr_0027;
pub mod pr_0028;
pub mod pr_0029;
pub mod pr_0035;
pub mod pr_0036;
pub mod pr_0039;
pub mod pr_0040;
pub mod pr_0043;
pub mod pr_0045;
pub mod pr_0046;
pub mod pr_0047;
pub mod pr_0048;
pub mod pr_0049;
pub mod pr_0050;
pub mod pr_0053;
pub mod pr_0054;
pub mod pr_0055;
pub mod pr_0056;
pub mod pr_0057;
pub mod pr_0058;
pub mod pr_0059;
pub mod pr_0061;
pub mod pr_0066;
pub mod pr_0067;
pub mod pr_0069;
pub mod pr_0070;
pub mod pr_0071;
pub mod pr_0073;
pub mod pr_0074;
pub mod pr_0077;
pub mod pr_0080;
pub mod pr_0082;
pub mod pr_0083;
pub mod pr_0086;
pub mod pr_0088;
pub mod pr_0092;
pub mod pr_0094;
pub mod pr_0098;
pub mod pr_0100;
pub mod pr_0101;
pub mod pr_0102;
pub mod pr_0103;
pub mod pr_0104;
pub mod pr_0105;
pub mod pr_0106;
pub mod pr_0108;
pub mod pr_0110;
pub mod pr_0111;
pub mod pr_0112;
pub mod pr_0113;
pub mod pr_0114;
pub mod pr_0118;
pub mod pr_0119;
pub mod pr_0121;
pub mod pr_0122;
pub mod pr_0125;
pub mod pr_0128;
pub mod pr_0129;
pub mod pr_0130;
pub mod pr_0134;
pub mod pr_0136;
pub mod pr_0144;
pub mod pr_0145;
pub mod pr_0146;
pub mod pr_0150;
pub mod pr_0151;
pub mod pr_0152;
pub mod pr_0155;
pub mod pr_0162;
pub mod pr_0167;
pub mod pr_0169;
pub mod pr_0173;
pub mod pr_0179;
pub mod pr_0189;
pub mod pr_0190;
pub mod pr_0191;
pub mod pr_0198;
pub mod pr_0199;
pub mod pr_0200;
pub mod pr_0202;
pub mod pr_0205;
pub mod pr_0206;
pub mod pr_0207;
pub mod pr_0208;
pub mod pr_0209;
pub mod pr_0214;
pub mod pr_0215;
pub mod pr_0216;
pub mod pr_0217;
pub mod pr_0219;
pub mod pr_0222;
pub mod pr_0226;
pub mod pr_0228;
pub mod pr_0230;
pub mod pr_0231;
pub mod pr_0232;
pub mod pr_0234;
pub mod pr_0235;
pub mod pr_0236;
pub mod pr_0238;
pub mod pr_0241;
pub mod pr_0242;
pub mod pr_0257;
pub mod pr_0263;
pub mod pr_0264;
pub mod pr_0268;
pub mod pr_0273;
pub mod pr_0274;
pub mod pr_0278;
pub mod pr_0283;
pub mod pr_0289;
pub mod pr_0290;
pub mod pr_0292;
pub mod pr_0300;
pub mod pr_0322;
pub mod pr_0326;
pub mod pr_0328;
pub mod pr_0334;
pub mod pr_0338;
pub mod pr_0342;
pub mod pr_0345;
pub mod pr_0350;
pub mod pr_0374;
pub mod pr_0380;
pub mod pr_0383;
pub mod pr_0386;
pub mod pr_0392;
pub mod pr_0394;
pub mod pr_0399;
pub mod pr_0401;
pub mod pr_0409;
pub mod pr_0415;
pub mod pr_0432;
pub mod pr_0437;
pub mod pr_0440;
pub mod pr_0443;
pub mod pr_0450;
pub mod pr_0452;
pub mod pr_0476;
pub mod pr_0530;
pub mod pr_0539;
pub mod pr_0547;
pub mod pr_0560;
pub mod pr_0564;
pub mod pr_0567;
pub mod pr_0592;
pub mod pr_0605;
pub mod pr_0624;
pub mod pr_0632;
pub mod pr_0633;
pub mod pr_0637;
pub mod pr_0641;
pub mod pr_0643;
pub mod pr_0649;
pub mod pr_0650;
pub mod pr_0664;
pub mod pr_0670;
pub mod pr_0700;
pub mod pr_0703;
pub mod pr_0704;
pub mod pr_0705;
pub mod pr_0719;
pub mod pr_0724;
pub mod pr_0725;
pub mod pr_0726;
pub mod pr_0729;
pub mod pr_0731;
pub mod pr_0733;
pub mod pr_0735;
pub mod pr_0746;
pub mod pr_0826;
pub mod pr_0840;
pub mod pr_0841;
pub mod pr_0860;
pub mod pr_0872;
pub mod pr_0874;
pub mod pr_0875;
pub mod pr_0876;
pub mod pr_0884;
pub mod pr_0885;
pub mod pr_0912;
pub mod pr_0918;
pub mod pr_0921;
pub mod pr_0933;
pub mod pr_0945;
pub mod pr_0947;
pub mod pr_0959;
pub mod pr_0962;
pub mod pr_0994;
pub mod pr_0995;
pub mod pr_0997;
pub mod pr_1004;
pub mod pr_1009;
pub mod pr_1038;
pub mod pr_1052;
pub mod pr_1071;
pub mod pr_1105;
pub mod pr_1106;
pub mod pr_1110;
pub mod pr_1137;
pub mod pr_1140;
pub mod pr_1161;
pub mod pr_1190;
pub mod pr_1207;
pub mod pr_1248;
pub mod pr_1249;
pub mod pr_1268;
pub mod pr_1290;
pub mod pr_1310;
pub mod pr_1331;
pub mod pr_1334;
pub mod pr_1367;
pub mod pr_1371;
pub mod pr_1372;
pub mod pr_1380;
pub mod pr_1381;
pub mod pr_1382;
pub mod pr_1395;
pub mod pr_1405;
pub mod pr_1431;
pub mod pr_1438;
pub mod pr_1448;
pub mod pr_1456;
pub mod pr_1460;
pub mod pr_1466;
pub mod pr_1482;
pub mod pr_1493;
pub mod pr_1497;
pub mod pr_1508;
pub mod pr_1509;
pub mod pr_1514;
pub mod pr_1518;
pub mod pr_1530;
pub mod pr_1545;
pub mod pr_1550;
pub mod pr_1552;
pub mod pr_1568;
pub mod pr_1579;
pub mod pr_1590;
pub mod pr_1593;
pub mod pr_1598;
pub mod pr_1605;
pub mod pr_1636;
pub mod pr_1653;
pub mod pr_1657;
pub mod pr_1679;
pub mod pr_1684;
pub mod pr_1701;
pub mod pr_1717;
pub mod pr_1732;
pub mod pr_1768;
pub mod pr_1791;
pub mod pr_1813;
pub mod pr_1823;
pub mod pr_1854;
pub mod pr_1905;
pub mod pr_1926;
pub mod pr_1937;
pub mod pr_1942;
pub mod pr_1945;
pub mod pr_1963;
pub mod pr_1971;
pub mod pr_1979;
pub mod pr_2022;
pub mod pr_2028;
pub mod pr_2044;
pub mod pr_2045;
pub mod pr_2053;
pub mod pr_2058;
pub mod pr_2095;
pub mod pr_2096;
pub mod pr_2130;
pub mod pr_2134;
pub mod pr_2181;
pub mod pr_2191;
pub mod pr_2192;
pub mod pr_2196;
pub mod pr_2215;
pub mod pr_2220;
pub mod pr_2285;
pub mod pr_2300;
pub mod pr_2326;
pub mod pr_2336;
pub mod pr_2352;
pub mod pr_2390;
pub mod pr_2392;
pub mod pr_2406;
pub mod pr_2416;
pub mod pr_2418;
pub mod pr_2419;
pub mod pr_2491;
pub mod pr_2530;
pub mod pr_2542;
pub mod pr_2574;
pub mod pr_2582;
pub mod pr_2583;
pub mod pr_2678;
pub mod pr_2696;
pub mod pr_2699;
pub mod pr_2707;
pub mod pr_2751;
pub mod pr_2807;
pub mod pr_2938;
pub mod pr_2976;
pub mod pr_3014;
pub mod pr_3016;
pub mod pr_3043;
pub mod pr_3146;
pub mod pr_3190;
pub mod pr_3194;
pub mod pr_3206;
pub mod pr_3207;
pub mod pr_3217;
