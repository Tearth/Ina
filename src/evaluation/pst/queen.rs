// ------------------------------------------------------------------------- //
// Generated at 08-09-2024 18:53:08 UTC (e = 0.114547, k = 0.0077, r = 1.00) //
// ------------------------------------------------------------------------- //

use super::*;

impl EvaluationParameters {
    #[rustfmt::skip]
    pub const QUEEN_PST_PATTERN: [[PackedEval; 64]; KING_BUCKETS_COUNT] =
    [
        [
            s!(1038, 1076), s!(1024, 1112), s!(1049, 1094), s!(1081, 1092), s!(1074, 1100), s!(1103, 1090), s!(1118, 1077), s!(1081, 1091),
            s!(1033, 1072), s!(1012, 1103), s!(1036, 1102), s!(1042, 1115), s!(1040, 1126), s!(1092, 1096), s!(1067, 1105), s!(1107, 1063),
            s!(1039, 1045), s!(1041, 1071), s!(1052, 1039), s!(1060, 1078), s!(1077, 1082), s!(1114, 1075), s!(1104, 1067), s!(1069, 1089),
            s!(1042, 1051), s!(1046, 1055), s!(1026, 1071), s!(1049, 1063), s!(1087, 1056), s!(1081, 1082), s!(1079, 1093), s!(1078, 1079),
            s!(1052, 1031), s!(1039, 1065), s!(1057, 1052), s!(1056, 1055), s!(1076, 1044), s!(1075, 1057), s!(1086, 1055), s!(1086, 1060),
            s!(1032, 1017), s!(1057, 1014), s!(1058, 1040), s!(1058, 1040), s!(1060, 1051), s!(1086, 1033), s!(1076, 1046), s!(1067, 1036),
            s!(1039, 1060), s!(1057, 1030), s!(1060, 1021), s!(1062, 1033), s!(1068, 1031), s!(1075, 1015), s!(1080,  993), s!(1035, 1022),
            s!(1056, 1006), s!(1048, 1020), s!(1053,  991), s!(1063, 1006), s!(1069,  997), s!(1050,  998), s!(1026, 1026), s!(1003, 1010),
        ],
        [
            s!(1059, 1081), s!(1032, 1114), s!(1062, 1091), s!(1078, 1081), s!(1084, 1090), s!(1087, 1090), s!(1124, 1066), s!(1076, 1094),
            s!(1045, 1081), s!(1029, 1114), s!(1056, 1084), s!(1056, 1110), s!(1040, 1132), s!(1075, 1098), s!(1049, 1118), s!(1101, 1079),
            s!(1072, 1039), s!(1056, 1069), s!(1065, 1077), s!(1057, 1090), s!(1071, 1093), s!(1088, 1099), s!(1104, 1092), s!(1071, 1090),
            s!(1067, 1049), s!(1058, 1073), s!(1065, 1064), s!(1063, 1088), s!(1074, 1077), s!(1071, 1089), s!(1070, 1099), s!(1068, 1087),
            s!(1066, 1054), s!(1062, 1062), s!(1070, 1051), s!(1065, 1068), s!(1070, 1061), s!(1065, 1071), s!(1081, 1057), s!(1073, 1060),
            s!(1068, 1014), s!(1067, 1041), s!(1075, 1036), s!(1068, 1044), s!(1070, 1045), s!(1080, 1043), s!(1083, 1044), s!(1078, 1038),
            s!(1077, 1025), s!(1073, 1031), s!(1077, 1025), s!(1077, 1015), s!(1082, 1016), s!(1094,  984), s!(1080,  994), s!(1031, 1035),
            s!(1088, 1002), s!(1072, 1021), s!(1079,  997), s!(1078, 1000), s!(1081,  989), s!(1064,  989), s!(1008, 1032), s!(1025, 1028),
        ],
        [
            s!( 996, 1056), s!(1019, 1118), s!(1036, 1086), s!(1060, 1080), s!(1079, 1081), s!(1075, 1077), s!(1108, 1064), s!(1054, 1072),
            s!( 994, 1052), s!( 999, 1086), s!(1015, 1082), s!(1025, 1107), s!(1022, 1120), s!(1061, 1077), s!(1031, 1097), s!(1079, 1064),
            s!(1030, 1027), s!(1020, 1044), s!(1047, 1050), s!(1045, 1073), s!(1070, 1075), s!(1095, 1052), s!(1084, 1066), s!(1057, 1086),
            s!(1017, 1030), s!(1028, 1041), s!(1029, 1059), s!(1043, 1073), s!(1071, 1052), s!(1063, 1074), s!(1065, 1095), s!(1074, 1082),
            s!(1029, 1025), s!(1013, 1052), s!(1052, 1039), s!(1061, 1052), s!(1074, 1047), s!(1072, 1064), s!(1072, 1060), s!(1071, 1065),
            s!(1020, 1011), s!(1035, 1007), s!(1054, 1029), s!(1058, 1029), s!(1043, 1028), s!(1087, 1027), s!(1069, 1042), s!(1064, 1064),
            s!(1013, 1056), s!(1048, 1022), s!(1058, 1001), s!(1060, 1006), s!(1062, 1001), s!(1061,  992), s!(1045, 1013), s!(1049, 1044),
            s!(1033,  995), s!(1037,  994), s!(1029,  991), s!(1055,  965), s!(1029, 1012), s!(1032, 1006), s!(1019, 1047), s!(1024, 1020),
        ],
        [
            s!(1003, 1059), s!(1008, 1102), s!(1062, 1098), s!(1076, 1088), s!(1069, 1077), s!(1074, 1068), s!(1095, 1062), s!(1035, 1072),
            s!(1006, 1048), s!( 995, 1078), s!( 986, 1089), s!(1009, 1115), s!( 991, 1110), s!(1023, 1074), s!(1009, 1093), s!(1066, 1077),
            s!(1037, 1034), s!(1007, 1054), s!(1015, 1066), s!(1030, 1093), s!(1024, 1086), s!(1059, 1064), s!(1067, 1071), s!(1059, 1083),
            s!(1032, 1046), s!(1035, 1046), s!(1023, 1071), s!(1022, 1080), s!(1008, 1059), s!(1036, 1073), s!(1022, 1084), s!(1051, 1098),
            s!(1054, 1027), s!(1021, 1055), s!(1049, 1046), s!(1039, 1052), s!(1043, 1060), s!(1046, 1064), s!(1048, 1068), s!(1051, 1079),
            s!(1049, 1022), s!(1059,  997), s!(1066, 1034), s!(1053, 1020), s!(1052, 1043), s!(1062, 1016), s!(1061, 1034), s!(1055, 1054),
            s!(1030, 1045), s!(1066,  996), s!(1073,  961), s!(1071,  973), s!(1071,  980), s!(1076,  989), s!(1066, 1012), s!(1090, 1041),
            s!(1067, 1003), s!(1061, 1007), s!(1052,  981), s!(1063,  944), s!(1036, 1010), s!(1031,  997), s!(1015, 1047), s!(1023, 1029),
        ],
        [
            s!(1010, 1059), s!(1010, 1099), s!(1034, 1070), s!(1063, 1071), s!(1083, 1084), s!(1076, 1070), s!(1110, 1068), s!(1049, 1064),
            s!(1018, 1057), s!(1001, 1098), s!(1036, 1096), s!(1023, 1119), s!(1009, 1110), s!(1067, 1073), s!(1035, 1094), s!(1093, 1078),
            s!(1042, 1028), s!(1038, 1057), s!(1048, 1049), s!(1042, 1069), s!(1076, 1071), s!(1086, 1061), s!(1065, 1057), s!(1066, 1087),
            s!(1023, 1045), s!(1037, 1059), s!(1026, 1069), s!(1027, 1063), s!(1062, 1055), s!(1056, 1053), s!(1054, 1083), s!(1057, 1079),
            s!(1030, 1033), s!(1036, 1059), s!(1049, 1043), s!(1075, 1070), s!(1060, 1040), s!(1080, 1069), s!(1049, 1058), s!(1058, 1066),
            s!(1039, 1015), s!(1063, 1006), s!(1059, 1030), s!(1059, 1032), s!(1049, 1028), s!(1065, 1028), s!(1054, 1033), s!(1051, 1052),
            s!(1012, 1042), s!(1040, 1001), s!(1033,  996), s!(1055, 1006), s!(1065,  994), s!(1046,  999), s!(1063, 1022), s!(1060, 1044),
            s!(1046,  995), s!(1040, 1002), s!(1029, 1002), s!(1028,  986), s!(1032, 1013), s!(1036, 1006), s!(1017, 1050), s!(1012, 1019),
        ],
        [
            s!(1008, 1052), s!(1001, 1093), s!(1042, 1082), s!(1070, 1081), s!(1082, 1079), s!(1072, 1065), s!(1101, 1055), s!(1054, 1071),
            s!(1024, 1065), s!(1003, 1084), s!(1030, 1107), s!(1023, 1113), s!(1012, 1122), s!(1073, 1080), s!(1033, 1089), s!(1069, 1065),
            s!(1037, 1037), s!(1042, 1065), s!(1052, 1051), s!(1048, 1083), s!(1071, 1070), s!(1081, 1063), s!(1083, 1070), s!(1073, 1073),
            s!(1053, 1046), s!(1033, 1060), s!(1047, 1072), s!(1055, 1066), s!(1072, 1062), s!(1073, 1054), s!(1068, 1077), s!(1062, 1077),
            s!(1058, 1032), s!(1040, 1058), s!(1066, 1057), s!(1066, 1054), s!(1067, 1029), s!(1064, 1068), s!(1053, 1068), s!(1047, 1069),
            s!(1034, 1017), s!(1065, 1023), s!(1092, 1039), s!(1072, 1021), s!(1076, 1040), s!(1069, 1026), s!(1072, 1034), s!(1055, 1062),
            s!(1009, 1051), s!(1045, 1009), s!(1092, 1004), s!(1077, 1024), s!(1070, 1031), s!(1059, 1001), s!(1062, 1022), s!(1069, 1041),
            s!(1040,  994), s!(1027, 1002), s!(1051, 1001), s!(1053,  979), s!(1049, 1011), s!(1042, 1004), s!(1017, 1048), s!(1028, 1024),
        ],
        [
            s!(1019, 1049), s!(1016, 1097), s!(1045, 1083), s!(1078, 1077), s!(1071, 1072), s!(1075, 1067), s!(1114, 1062), s!(1066, 1088),
            s!(1036, 1058), s!(1002, 1096), s!(1045, 1099), s!(1037, 1121), s!(1032, 1116), s!(1072, 1066), s!(1045, 1089), s!(1080, 1067),
            s!(1052, 1044), s!(1044, 1067), s!(1065, 1062), s!(1061, 1090), s!(1084, 1080), s!(1100, 1068), s!(1090, 1072), s!(1077, 1082),
            s!(1046, 1052), s!(1050, 1069), s!(1055, 1072), s!(1056, 1077), s!(1087, 1075), s!(1066, 1055), s!(1059, 1083), s!(1060, 1081),
            s!(1053, 1036), s!(1044, 1073), s!(1079, 1049), s!(1077, 1061), s!(1083, 1041), s!(1060, 1058), s!(1065, 1079), s!(1036, 1071),
            s!(1024, 1015), s!(1072, 1007), s!(1062, 1055), s!(1076, 1044), s!(1066, 1047), s!(1074, 1021), s!(1052, 1041), s!(1052, 1065),
            s!(1017, 1047), s!(1056, 1014), s!(1083,  994), s!(1073, 1027), s!(1064, 1026), s!(1070, 1002), s!(1062, 1026), s!(1069, 1048),
            s!(1054, 1002), s!(1053, 1005), s!(1058,  991), s!(1057,  994), s!(1051, 1023), s!(1038, 1003), s!(1033, 1056), s!(1008, 1018),
        ],
        [
            s!(1025, 1056), s!(1021, 1106), s!(1051, 1093), s!(1067, 1071), s!(1073, 1074), s!(1077, 1066), s!(1106, 1060), s!(1063, 1082),
            s!(1033, 1070), s!(1029, 1102), s!(1037, 1099), s!(1030, 1120), s!(1019, 1116), s!(1071, 1070), s!(1036, 1092), s!(1079, 1062),
            s!(1055, 1035), s!(1060, 1074), s!(1065, 1066), s!(1051, 1089), s!(1072, 1074), s!(1092, 1067), s!(1089, 1072), s!(1051, 1085),
            s!(1050, 1051), s!(1065, 1075), s!(1048, 1082), s!(1043, 1066), s!(1069, 1056), s!(1059, 1059), s!(1066, 1088), s!(1057, 1082),
            s!(1062, 1041), s!(1048, 1069), s!(1059, 1054), s!(1067, 1058), s!(1072, 1049), s!(1055, 1063), s!(1059, 1071), s!(1036, 1062),
            s!(1036, 1018), s!(1077, 1024), s!(1083, 1041), s!(1072, 1031), s!(1070, 1036), s!(1068, 1020), s!(1053, 1039), s!(1061, 1055),
            s!(1021, 1052), s!(1058, 1017), s!(1081, 1015), s!(1080, 1026), s!(1058, 1021), s!(1081,  996), s!(1063, 1027), s!(1062, 1057),
            s!(1064, 1005), s!(1052, 1006), s!(1054, 1004), s!(1090,  996), s!(1046, 1021), s!(1039,  999), s!(1012, 1048), s!(1012, 1023),
        ],
    ];
}
