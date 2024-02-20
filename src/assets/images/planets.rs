use rand_derive2::RandGen;
use std::fmt::Display;

#[derive(RandGen)]
pub enum PlanetSprite {
    Planet1,
    Planet2,
    Planet3,
    Planet4,
    Planet5,
    Planet6,
    Planet7,
    Planet8,
    Planet9,
    Planet10,
    Planet11,
    Planet12,
    Planet13,
    Planet14,
    Planet15,
    Planet16,
    Planet17,
    Planet18,
    Planet19,
    Planet20,
    Planet21,
    Planet22,
    Planet23,
    Planet24,
    Planet25,
    Planet26,
    Planet27,
    Planet28,
    Planet29,
    Planet30,
    Planet31,
    Planet32,
    Planet33,
    Planet34,
    Planet35,
    Planet36,
    Planet37,
    Planet38,
    Planet39,
    Planet40,
    Planet41,
    Planet42,
    Planet43,
    Planet44,
    Planet45,
    Planet46,
    Planet47,
    Planet48,
    Planet49,
    Planet50,
    Planet51,
    Planet52,
    Planet53,
    Planet54,
    Planet55,
    Planet56,
    Planet57,
    Planet58,
    Planet59,
    Planet60,
    Planet61,
    Planet62,
    Planet63,
    Planet64,
    Planet65,
    Planet66,
    Planet67,
    Planet68,
    Planet69,
    Planet70,
    Planet71,
    Planet72,
    Planet73,
    Planet74,
    Planet75,
    Planet76,
    Planet77,
    Planet78,
    Planet79,
    Planet80,
    Planet81,
    Planet82,
    Planet83,
    Planet84,
    Planet85,
    Planet86,
    Planet87,
    Planet88,
    Planet89,
    Planet90,
    Planet91,
    Planet92,
    Planet93,
    Planet94,
    Planet95,
    Planet96,
    Planet97,
    Planet98,
    Planet99,
    Planet100,
    Planet101,
    Planet102,
    Planet103,
    Planet104,
    Planet105,
    Planet106,
    Planet107,
    Planet108,
    Planet109,
    Planet110,
    Planet111,
    Planet112,
    Planet113,
    Planet114,
    Planet115,
    Planet116,
    Planet117,
    Planet118,
    Planet119,
    Planet120,
    Planet121,
    Planet122,
    Planet123,
    Planet124,
    Planet125,
    Planet126,
    Planet127,
    Planet128,
    Planet129,
    Planet130,
    Planet131,
    Planet132,
    Planet133,
    Planet134,
    Planet135,
    Planet136,
    Planet137,
    Planet138,
    Planet139,
    Planet140,
    Planet141,
    Planet142,
    Planet143,
    Planet144,
    Planet145,
    Planet146,
    Planet147,
    Planet148,
    Planet149,
    Planet150,
    Planet151,
    Planet152,
    Planet153,
    Planet154,
    Planet155,
    Planet156,
    Planet157,
    Planet158,
    Planet159,
    Planet160,
    Planet161,
    Planet162,
    Planet163,
    Planet164,
    Planet165,
    Planet166,
    Planet167,
    Planet168,
    Planet169,
    Planet170,
    Planet171,
    Planet172,
    Planet173,
    Planet174,
    Planet175,
    Planet176,
    Planet177,
    Planet178,
    Planet179,
    Planet180,
    Planet181,
    Planet182,
    Planet183,
    Planet184,
    Planet185,
    Planet186,
    Planet187,
    Planet188,
    Planet189,
    Planet190,
    Planet191,
    Planet192,
    Planet193,
    Planet194,
    Planet195,
    Planet196,
    Planet197,
    Planet198,
    Planet199,
    Planet200,
    Planet201,
    Planet202,
    Planet203,
    Planet204,
    Planet205,
    Planet206,
    Planet207,
    Planet208,
    Planet209,
    Planet210,
    Planet211,
    Planet212,
    Planet213,
    Planet214,
    Planet215,
    Planet216,
    Planet217,
    Planet218,
    Planet219,
    Planet220,
    Planet221,
    Planet222,
    Planet223,
    Planet224,
    Planet225,
    Planet226,
    Planet227,
    Planet228,
    Planet229,
    Planet230,
    Planet231,
    Planet232,
    Planet233,
    Planet234,
    Planet235,
    Planet236,
    Planet237,
    Planet238,
    Planet239,
    Planet240,
    Planet241,
    Planet242,
    Planet243,
    Planet244,
    Planet245,
    Planet246,
    Planet247,
    Planet248,
    Planet249,
}

impl Display for PlanetSprite {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlanetSprite::Planet1 => {
                write!(formatter, "images/planets/planet_1.png")
            }
            PlanetSprite::Planet2 => {
                write!(formatter, "images/planets/planet_2.png")
            }
            PlanetSprite::Planet3 => {
                write!(formatter, "images/planets/planet_3.png")
            }
            PlanetSprite::Planet4 => {
                write!(formatter, "images/planets/planet_4.png")
            }
            PlanetSprite::Planet5 => {
                write!(formatter, "images/planets/planet_5.png")
            }
            PlanetSprite::Planet6 => {
                write!(formatter, "images/planets/planet_6.png")
            }
            PlanetSprite::Planet7 => {
                write!(formatter, "images/planets/planet_7.png")
            }
            PlanetSprite::Planet8 => {
                write!(formatter, "images/planets/planet_8.png")
            }
            PlanetSprite::Planet9 => {
                write!(formatter, "images/planets/planet_9.png")
            }
            PlanetSprite::Planet10 => {
                write!(formatter, "images/planets/planet_10.png")
            }
            PlanetSprite::Planet11 => {
                write!(formatter, "images/planets/planet_11.png")
            }
            PlanetSprite::Planet12 => {
                write!(formatter, "images/planets/planet_12.png")
            }
            PlanetSprite::Planet13 => {
                write!(formatter, "images/planets/planet_13.png")
            }
            PlanetSprite::Planet14 => {
                write!(formatter, "images/planets/planet_14.png")
            }
            PlanetSprite::Planet15 => {
                write!(formatter, "images/planets/planet_15.png")
            }
            PlanetSprite::Planet16 => {
                write!(formatter, "images/planets/planet_16.png")
            }
            PlanetSprite::Planet17 => {
                write!(formatter, "images/planets/planet_17.png")
            }
            PlanetSprite::Planet18 => {
                write!(formatter, "images/planets/planet_18.png")
            }
            PlanetSprite::Planet19 => {
                write!(formatter, "images/planets/planet_19.png")
            }
            PlanetSprite::Planet20 => {
                write!(formatter, "images/planets/planet_20.png")
            }
            PlanetSprite::Planet21 => {
                write!(formatter, "images/planets/planet_21.png")
            }
            PlanetSprite::Planet22 => {
                write!(formatter, "images/planets/planet_22.png")
            }
            PlanetSprite::Planet23 => {
                write!(formatter, "images/planets/planet_23.png")
            }
            PlanetSprite::Planet24 => {
                write!(formatter, "images/planets/planet_24.png")
            }
            PlanetSprite::Planet25 => {
                write!(formatter, "images/planets/planet_25.png")
            }
            PlanetSprite::Planet26 => {
                write!(formatter, "images/planets/planet_26.png")
            }
            PlanetSprite::Planet27 => {
                write!(formatter, "images/planets/planet_27.png")
            }
            PlanetSprite::Planet28 => {
                write!(formatter, "images/planets/planet_28.png")
            }
            PlanetSprite::Planet29 => {
                write!(formatter, "images/planets/planet_29.png")
            }
            PlanetSprite::Planet30 => {
                write!(formatter, "images/planets/planet_30.png")
            }
            PlanetSprite::Planet31 => {
                write!(formatter, "images/planets/planet_31.png")
            }
            PlanetSprite::Planet32 => {
                write!(formatter, "images/planets/planet_32.png")
            }
            PlanetSprite::Planet33 => {
                write!(formatter, "images/planets/planet_33.png")
            }
            PlanetSprite::Planet34 => {
                write!(formatter, "images/planets/planet_34.png")
            }
            PlanetSprite::Planet35 => {
                write!(formatter, "images/planets/planet_35.png")
            }
            PlanetSprite::Planet36 => {
                write!(formatter, "images/planets/planet_36.png")
            }
            PlanetSprite::Planet37 => {
                write!(formatter, "images/planets/planet_37.png")
            }
            PlanetSprite::Planet38 => {
                write!(formatter, "images/planets/planet_38.png")
            }
            PlanetSprite::Planet39 => {
                write!(formatter, "images/planets/planet_39.png")
            }
            PlanetSprite::Planet40 => {
                write!(formatter, "images/planets/planet_40.png")
            }
            PlanetSprite::Planet41 => {
                write!(formatter, "images/planets/planet_41.png")
            }
            PlanetSprite::Planet42 => {
                write!(formatter, "images/planets/planet_42.png")
            }
            PlanetSprite::Planet43 => {
                write!(formatter, "images/planets/planet_43.png")
            }
            PlanetSprite::Planet44 => {
                write!(formatter, "images/planets/planet_44.png")
            }
            PlanetSprite::Planet45 => {
                write!(formatter, "images/planets/planet_45.png")
            }
            PlanetSprite::Planet46 => {
                write!(formatter, "images/planets/planet_46.png")
            }
            PlanetSprite::Planet47 => {
                write!(formatter, "images/planets/planet_47.png")
            }
            PlanetSprite::Planet48 => {
                write!(formatter, "images/planets/planet_48.png")
            }
            PlanetSprite::Planet49 => {
                write!(formatter, "images/planets/planet_49.png")
            }
            PlanetSprite::Planet50 => {
                write!(formatter, "images/planets/planet_50.png")
            }
            PlanetSprite::Planet51 => {
                write!(formatter, "images/planets/planet_51.png")
            }
            PlanetSprite::Planet52 => {
                write!(formatter, "images/planets/planet_52.png")
            }
            PlanetSprite::Planet53 => {
                write!(formatter, "images/planets/planet_53.png")
            }
            PlanetSprite::Planet54 => {
                write!(formatter, "images/planets/planet_54.png")
            }
            PlanetSprite::Planet55 => {
                write!(formatter, "images/planets/planet_55.png")
            }
            PlanetSprite::Planet56 => {
                write!(formatter, "images/planets/planet_56.png")
            }
            PlanetSprite::Planet57 => {
                write!(formatter, "images/planets/planet_57.png")
            }
            PlanetSprite::Planet58 => {
                write!(formatter, "images/planets/planet_58.png")
            }
            PlanetSprite::Planet59 => {
                write!(formatter, "images/planets/planet_59.png")
            }
            PlanetSprite::Planet60 => {
                write!(formatter, "images/planets/planet_60.png")
            }
            PlanetSprite::Planet61 => {
                write!(formatter, "images/planets/planet_61.png")
            }
            PlanetSprite::Planet62 => {
                write!(formatter, "images/planets/planet_62.png")
            }
            PlanetSprite::Planet63 => {
                write!(formatter, "images/planets/planet_63.png")
            }
            PlanetSprite::Planet64 => {
                write!(formatter, "images/planets/planet_64.png")
            }
            PlanetSprite::Planet65 => {
                write!(formatter, "images/planets/planet_65.png")
            }
            PlanetSprite::Planet66 => {
                write!(formatter, "images/planets/planet_66.png")
            }
            PlanetSprite::Planet67 => {
                write!(formatter, "images/planets/planet_67.png")
            }
            PlanetSprite::Planet68 => {
                write!(formatter, "images/planets/planet_68.png")
            }
            PlanetSprite::Planet69 => {
                write!(formatter, "images/planets/planet_69.png")
            }
            PlanetSprite::Planet70 => {
                write!(formatter, "images/planets/planet_70.png")
            }
            PlanetSprite::Planet71 => {
                write!(formatter, "images/planets/planet_71.png")
            }
            PlanetSprite::Planet72 => {
                write!(formatter, "images/planets/planet_72.png")
            }
            PlanetSprite::Planet73 => {
                write!(formatter, "images/planets/planet_73.png")
            }
            PlanetSprite::Planet74 => {
                write!(formatter, "images/planets/planet_74.png")
            }
            PlanetSprite::Planet75 => {
                write!(formatter, "images/planets/planet_75.png")
            }
            PlanetSprite::Planet76 => {
                write!(formatter, "images/planets/planet_76.png")
            }
            PlanetSprite::Planet77 => {
                write!(formatter, "images/planets/planet_77.png")
            }
            PlanetSprite::Planet78 => {
                write!(formatter, "images/planets/planet_78.png")
            }
            PlanetSprite::Planet79 => {
                write!(formatter, "images/planets/planet_79.png")
            }
            PlanetSprite::Planet80 => {
                write!(formatter, "images/planets/planet_80.png")
            }
            PlanetSprite::Planet81 => {
                write!(formatter, "images/planets/planet_81.png")
            }
            PlanetSprite::Planet82 => {
                write!(formatter, "images/planets/planet_82.png")
            }
            PlanetSprite::Planet83 => {
                write!(formatter, "images/planets/planet_83.png")
            }
            PlanetSprite::Planet84 => {
                write!(formatter, "images/planets/planet_84.png")
            }
            PlanetSprite::Planet85 => {
                write!(formatter, "images/planets/planet_85.png")
            }
            PlanetSprite::Planet86 => {
                write!(formatter, "images/planets/planet_86.png")
            }
            PlanetSprite::Planet87 => {
                write!(formatter, "images/planets/planet_87.png")
            }
            PlanetSprite::Planet88 => {
                write!(formatter, "images/planets/planet_88.png")
            }
            PlanetSprite::Planet89 => {
                write!(formatter, "images/planets/planet_89.png")
            }
            PlanetSprite::Planet90 => {
                write!(formatter, "images/planets/planet_90.png")
            }
            PlanetSprite::Planet91 => {
                write!(formatter, "images/planets/planet_91.png")
            }
            PlanetSprite::Planet92 => {
                write!(formatter, "images/planets/planet_92.png")
            }
            PlanetSprite::Planet93 => {
                write!(formatter, "images/planets/planet_93.png")
            }
            PlanetSprite::Planet94 => {
                write!(formatter, "images/planets/planet_94.png")
            }
            PlanetSprite::Planet95 => {
                write!(formatter, "images/planets/planet_95.png")
            }
            PlanetSprite::Planet96 => {
                write!(formatter, "images/planets/planet_96.png")
            }
            PlanetSprite::Planet97 => {
                write!(formatter, "images/planets/planet_97.png")
            }
            PlanetSprite::Planet98 => {
                write!(formatter, "images/planets/planet_98.png")
            }
            PlanetSprite::Planet99 => {
                write!(formatter, "images/planets/planet_99.png")
            }
            PlanetSprite::Planet100 => {
                write!(formatter, "images/planets/planet_100.png")
            }
            PlanetSprite::Planet101 => {
                write!(formatter, "images/planets/planet_101.png")
            }
            PlanetSprite::Planet102 => {
                write!(formatter, "images/planets/planet_102.png")
            }
            PlanetSprite::Planet103 => {
                write!(formatter, "images/planets/planet_103.png")
            }
            PlanetSprite::Planet104 => {
                write!(formatter, "images/planets/planet_104.png")
            }
            PlanetSprite::Planet105 => {
                write!(formatter, "images/planets/planet_105.png")
            }
            PlanetSprite::Planet106 => {
                write!(formatter, "images/planets/planet_106.png")
            }
            PlanetSprite::Planet107 => {
                write!(formatter, "images/planets/planet_107.png")
            }
            PlanetSprite::Planet108 => {
                write!(formatter, "images/planets/planet_108.png")
            }
            PlanetSprite::Planet109 => {
                write!(formatter, "images/planets/planet_109.png")
            }
            PlanetSprite::Planet110 => {
                write!(formatter, "images/planets/planet_110.png")
            }
            PlanetSprite::Planet111 => {
                write!(formatter, "images/planets/planet_111.png")
            }
            PlanetSprite::Planet112 => {
                write!(formatter, "images/planets/planet_112.png")
            }
            PlanetSprite::Planet113 => {
                write!(formatter, "images/planets/planet_113.png")
            }
            PlanetSprite::Planet114 => {
                write!(formatter, "images/planets/planet_114.png")
            }
            PlanetSprite::Planet115 => {
                write!(formatter, "images/planets/planet_115.png")
            }
            PlanetSprite::Planet116 => {
                write!(formatter, "images/planets/planet_116.png")
            }
            PlanetSprite::Planet117 => {
                write!(formatter, "images/planets/planet_117.png")
            }
            PlanetSprite::Planet118 => {
                write!(formatter, "images/planets/planet_118.png")
            }
            PlanetSprite::Planet119 => {
                write!(formatter, "images/planets/planet_119.png")
            }
            PlanetSprite::Planet120 => {
                write!(formatter, "images/planets/planet_120.png")
            }
            PlanetSprite::Planet121 => {
                write!(formatter, "images/planets/planet_121.png")
            }
            PlanetSprite::Planet122 => {
                write!(formatter, "images/planets/planet_122.png")
            }
            PlanetSprite::Planet123 => {
                write!(formatter, "images/planets/planet_123.png")
            }
            PlanetSprite::Planet124 => {
                write!(formatter, "images/planets/planet_124.png")
            }
            PlanetSprite::Planet125 => {
                write!(formatter, "images/planets/planet_125.png")
            }
            PlanetSprite::Planet126 => {
                write!(formatter, "images/planets/planet_126.png")
            }
            PlanetSprite::Planet127 => {
                write!(formatter, "images/planets/planet_127.png")
            }
            PlanetSprite::Planet128 => {
                write!(formatter, "images/planets/planet_128.png")
            }
            PlanetSprite::Planet129 => {
                write!(formatter, "images/planets/planet_129.png")
            }
            PlanetSprite::Planet130 => {
                write!(formatter, "images/planets/planet_130.png")
            }
            PlanetSprite::Planet131 => {
                write!(formatter, "images/planets/planet_131.png")
            }
            PlanetSprite::Planet132 => {
                write!(formatter, "images/planets/planet_132.png")
            }
            PlanetSprite::Planet133 => {
                write!(formatter, "images/planets/planet_133.png")
            }
            PlanetSprite::Planet134 => {
                write!(formatter, "images/planets/planet_134.png")
            }
            PlanetSprite::Planet135 => {
                write!(formatter, "images/planets/planet_135.png")
            }
            PlanetSprite::Planet136 => {
                write!(formatter, "images/planets/planet_136.png")
            }
            PlanetSprite::Planet137 => {
                write!(formatter, "images/planets/planet_137.png")
            }
            PlanetSprite::Planet138 => {
                write!(formatter, "images/planets/planet_138.png")
            }
            PlanetSprite::Planet139 => {
                write!(formatter, "images/planets/planet_139.png")
            }
            PlanetSprite::Planet140 => {
                write!(formatter, "images/planets/planet_140.png")
            }
            PlanetSprite::Planet141 => {
                write!(formatter, "images/planets/planet_141.png")
            }
            PlanetSprite::Planet142 => {
                write!(formatter, "images/planets/planet_142.png")
            }
            PlanetSprite::Planet143 => {
                write!(formatter, "images/planets/planet_143.png")
            }
            PlanetSprite::Planet144 => {
                write!(formatter, "images/planets/planet_144.png")
            }
            PlanetSprite::Planet145 => {
                write!(formatter, "images/planets/planet_145.png")
            }
            PlanetSprite::Planet146 => {
                write!(formatter, "images/planets/planet_146.png")
            }
            PlanetSprite::Planet147 => {
                write!(formatter, "images/planets/planet_147.png")
            }
            PlanetSprite::Planet148 => {
                write!(formatter, "images/planets/planet_148.png")
            }
            PlanetSprite::Planet149 => {
                write!(formatter, "images/planets/planet_149.png")
            }
            PlanetSprite::Planet150 => {
                write!(formatter, "images/planets/planet_150.png")
            }
            PlanetSprite::Planet151 => {
                write!(formatter, "images/planets/planet_151.png")
            }
            PlanetSprite::Planet152 => {
                write!(formatter, "images/planets/planet_152.png")
            }
            PlanetSprite::Planet153 => {
                write!(formatter, "images/planets/planet_153.png")
            }
            PlanetSprite::Planet154 => {
                write!(formatter, "images/planets/planet_154.png")
            }
            PlanetSprite::Planet155 => {
                write!(formatter, "images/planets/planet_155.png")
            }
            PlanetSprite::Planet156 => {
                write!(formatter, "images/planets/planet_156.png")
            }
            PlanetSprite::Planet157 => {
                write!(formatter, "images/planets/planet_157.png")
            }
            PlanetSprite::Planet158 => {
                write!(formatter, "images/planets/planet_158.png")
            }
            PlanetSprite::Planet159 => {
                write!(formatter, "images/planets/planet_159.png")
            }
            PlanetSprite::Planet160 => {
                write!(formatter, "images/planets/planet_160.png")
            }
            PlanetSprite::Planet161 => {
                write!(formatter, "images/planets/planet_161.png")
            }
            PlanetSprite::Planet162 => {
                write!(formatter, "images/planets/planet_162.png")
            }
            PlanetSprite::Planet163 => {
                write!(formatter, "images/planets/planet_163.png")
            }
            PlanetSprite::Planet164 => {
                write!(formatter, "images/planets/planet_164.png")
            }
            PlanetSprite::Planet165 => {
                write!(formatter, "images/planets/planet_165.png")
            }
            PlanetSprite::Planet166 => {
                write!(formatter, "images/planets/planet_166.png")
            }
            PlanetSprite::Planet167 => {
                write!(formatter, "images/planets/planet_167.png")
            }
            PlanetSprite::Planet168 => {
                write!(formatter, "images/planets/planet_168.png")
            }
            PlanetSprite::Planet169 => {
                write!(formatter, "images/planets/planet_169.png")
            }
            PlanetSprite::Planet170 => {
                write!(formatter, "images/planets/planet_170.png")
            }
            PlanetSprite::Planet171 => {
                write!(formatter, "images/planets/planet_171.png")
            }
            PlanetSprite::Planet172 => {
                write!(formatter, "images/planets/planet_172.png")
            }
            PlanetSprite::Planet173 => {
                write!(formatter, "images/planets/planet_173.png")
            }
            PlanetSprite::Planet174 => {
                write!(formatter, "images/planets/planet_174.png")
            }
            PlanetSprite::Planet175 => {
                write!(formatter, "images/planets/planet_175.png")
            }
            PlanetSprite::Planet176 => {
                write!(formatter, "images/planets/planet_176.png")
            }
            PlanetSprite::Planet177 => {
                write!(formatter, "images/planets/planet_177.png")
            }
            PlanetSprite::Planet178 => {
                write!(formatter, "images/planets/planet_178.png")
            }
            PlanetSprite::Planet179 => {
                write!(formatter, "images/planets/planet_179.png")
            }
            PlanetSprite::Planet180 => {
                write!(formatter, "images/planets/planet_180.png")
            }
            PlanetSprite::Planet181 => {
                write!(formatter, "images/planets/planet_181.png")
            }
            PlanetSprite::Planet182 => {
                write!(formatter, "images/planets/planet_182.png")
            }
            PlanetSprite::Planet183 => {
                write!(formatter, "images/planets/planet_183.png")
            }
            PlanetSprite::Planet184 => {
                write!(formatter, "images/planets/planet_184.png")
            }
            PlanetSprite::Planet185 => {
                write!(formatter, "images/planets/planet_185.png")
            }
            PlanetSprite::Planet186 => {
                write!(formatter, "images/planets/planet_186.png")
            }
            PlanetSprite::Planet187 => {
                write!(formatter, "images/planets/planet_187.png")
            }
            PlanetSprite::Planet188 => {
                write!(formatter, "images/planets/planet_188.png")
            }
            PlanetSprite::Planet189 => {
                write!(formatter, "images/planets/planet_189.png")
            }
            PlanetSprite::Planet190 => {
                write!(formatter, "images/planets/planet_190.png")
            }
            PlanetSprite::Planet191 => {
                write!(formatter, "images/planets/planet_191.png")
            }
            PlanetSprite::Planet192 => {
                write!(formatter, "images/planets/planet_192.png")
            }
            PlanetSprite::Planet193 => {
                write!(formatter, "images/planets/planet_193.png")
            }
            PlanetSprite::Planet194 => {
                write!(formatter, "images/planets/planet_194.png")
            }
            PlanetSprite::Planet195 => {
                write!(formatter, "images/planets/planet_195.png")
            }
            PlanetSprite::Planet196 => {
                write!(formatter, "images/planets/planet_196.png")
            }
            PlanetSprite::Planet197 => {
                write!(formatter, "images/planets/planet_197.png")
            }
            PlanetSprite::Planet198 => {
                write!(formatter, "images/planets/planet_198.png")
            }
            PlanetSprite::Planet199 => {
                write!(formatter, "images/planets/planet_199.png")
            }
            PlanetSprite::Planet200 => {
                write!(formatter, "images/planets/planet_200.png")
            }
            PlanetSprite::Planet201 => {
                write!(formatter, "images/planets/planet_201.png")
            }
            PlanetSprite::Planet202 => {
                write!(formatter, "images/planets/planet_202.png")
            }
            PlanetSprite::Planet203 => {
                write!(formatter, "images/planets/planet_203.png")
            }
            PlanetSprite::Planet204 => {
                write!(formatter, "images/planets/planet_204.png")
            }
            PlanetSprite::Planet205 => {
                write!(formatter, "images/planets/planet_205.png")
            }
            PlanetSprite::Planet206 => {
                write!(formatter, "images/planets/planet_206.png")
            }
            PlanetSprite::Planet207 => {
                write!(formatter, "images/planets/planet_207.png")
            }
            PlanetSprite::Planet208 => {
                write!(formatter, "images/planets/planet_208.png")
            }
            PlanetSprite::Planet209 => {
                write!(formatter, "images/planets/planet_209.png")
            }
            PlanetSprite::Planet210 => {
                write!(formatter, "images/planets/planet_210.png")
            }
            PlanetSprite::Planet211 => {
                write!(formatter, "images/planets/planet_211.png")
            }
            PlanetSprite::Planet212 => {
                write!(formatter, "images/planets/planet_212.png")
            }
            PlanetSprite::Planet213 => {
                write!(formatter, "images/planets/planet_213.png")
            }
            PlanetSprite::Planet214 => {
                write!(formatter, "images/planets/planet_214.png")
            }
            PlanetSprite::Planet215 => {
                write!(formatter, "images/planets/planet_215.png")
            }
            PlanetSprite::Planet216 => {
                write!(formatter, "images/planets/planet_216.png")
            }
            PlanetSprite::Planet217 => {
                write!(formatter, "images/planets/planet_217.png")
            }
            PlanetSprite::Planet218 => {
                write!(formatter, "images/planets/planet_218.png")
            }
            PlanetSprite::Planet219 => {
                write!(formatter, "images/planets/planet_219.png")
            }
            PlanetSprite::Planet220 => {
                write!(formatter, "images/planets/planet_220.png")
            }
            PlanetSprite::Planet221 => {
                write!(formatter, "images/planets/planet_221.png")
            }
            PlanetSprite::Planet222 => {
                write!(formatter, "images/planets/planet_222.png")
            }
            PlanetSprite::Planet223 => {
                write!(formatter, "images/planets/planet_223.png")
            }
            PlanetSprite::Planet224 => {
                write!(formatter, "images/planets/planet_224.png")
            }
            PlanetSprite::Planet225 => {
                write!(formatter, "images/planets/planet_225.png")
            }
            PlanetSprite::Planet226 => {
                write!(formatter, "images/planets/planet_226.png")
            }
            PlanetSprite::Planet227 => {
                write!(formatter, "images/planets/planet_227.png")
            }
            PlanetSprite::Planet228 => {
                write!(formatter, "images/planets/planet_228.png")
            }
            PlanetSprite::Planet229 => {
                write!(formatter, "images/planets/planet_229.png")
            }
            PlanetSprite::Planet230 => {
                write!(formatter, "images/planets/planet_230.png")
            }
            PlanetSprite::Planet231 => {
                write!(formatter, "images/planets/planet_231.png")
            }
            PlanetSprite::Planet232 => {
                write!(formatter, "images/planets/planet_232.png")
            }
            PlanetSprite::Planet233 => {
                write!(formatter, "images/planets/planet_233.png")
            }
            PlanetSprite::Planet234 => {
                write!(formatter, "images/planets/planet_234.png")
            }
            PlanetSprite::Planet235 => {
                write!(formatter, "images/planets/planet_235.png")
            }
            PlanetSprite::Planet236 => {
                write!(formatter, "images/planets/planet_236.png")
            }
            PlanetSprite::Planet237 => {
                write!(formatter, "images/planets/planet_237.png")
            }
            PlanetSprite::Planet238 => {
                write!(formatter, "images/planets/planet_238.png")
            }
            PlanetSprite::Planet239 => {
                write!(formatter, "images/planets/planet_239.png")
            }
            PlanetSprite::Planet240 => {
                write!(formatter, "images/planets/planet_240.png")
            }
            PlanetSprite::Planet241 => {
                write!(formatter, "images/planets/planet_241.png")
            }
            PlanetSprite::Planet242 => {
                write!(formatter, "images/planets/planet_242.png")
            }
            PlanetSprite::Planet243 => {
                write!(formatter, "images/planets/planet_243.png")
            }
            PlanetSprite::Planet244 => {
                write!(formatter, "images/planets/planet_244.png")
            }
            PlanetSprite::Planet245 => {
                write!(formatter, "images/planets/planet_245.png")
            }
            PlanetSprite::Planet246 => {
                write!(formatter, "images/planets/planet_246.png")
            }
            PlanetSprite::Planet247 => {
                write!(formatter, "images/planets/planet_247.png")
            }
            PlanetSprite::Planet248 => {
                write!(formatter, "images/planets/planet_248.png")
            }
            PlanetSprite::Planet249 => {
                write!(formatter, "images/planets/planet_249.png")
            }
        }
    }
}

#[cfg(test)]
mod planet_sprite_should {
    #[test]
    #[ignore]
    fn return_the_expected_file_path() {}
}
