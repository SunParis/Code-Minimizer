
import jdk.incubator.vector.*;
import java.util.Arrays;
import java.util.Vector;
class Cls {
    public static final int N = 128;
    public static long instanceCount = 5L;
}
public class Test {
    public static final int N = 128;
    public static long instanceCount = -6L;
    public static java.lang.String strFld = "two";
    public static byte byFld = 119;
    public static char cFld = 39912;
    public static int iFld = 6;
    public int iFld1 = -12;
    public static int iFld2 = 12;
    public long lFld = -147L;
    public volatile int iFld3 = -3;
    public static long lArrFld[] = new long[Test.N];
    public volatile int iArrFld[] = new int[Test.N];
    public double dArrFld[] = new double[Test.N];
    static {
        FuzzerUtils.init(Test.lArrFld, -3366490791859620162L);
    }
    public static long strMeth_check_sum = 0;
    public static long vMeth_check_sum = 0;
    public static long vMeth1_check_sum = 0;
    public static void vMeth1(int i7, int i8) {
        long meth_res = 0;
        int i9 = -28411;
        int i10 = 3;
        int i11 = 116;
        int i12 = -104;
        int i13 = 64515;
        int i14 = -24083;
        int i15 = 36656;
        int i16 = -36965;
        int i17 = 21367;
        int i18 = 0;
        int iArr[] = new int[Test.N];
        float f1 = -11.135F;
        short s = 16466;
        short sArr[] = new short[Test.N];
        long l = 12927L;
        long lArr[] = new long[Test.N];
        double d2 = 0.72377;
        double dArr[] = new double[Test.N];
        boolean b1 = false;
        boolean bArr[][] = new boolean[Test.N][Test.N];
        char cArr[][] = new char[Test.N][Test.N];
        java.lang.Object OArr[] = new java.lang.Object[46];
        java.lang.String strArr2[] = new java.lang.String[Test.N];
        Cls O1 = new Cls();
        Cls O2 = new Cls();
        Cls O3 = new Cls();
        Cls O4 = new Cls();
        FuzzerUtils.init(dArr, -2.1642);
        FuzzerUtils.init(iArr, -227);
        FuzzerUtils.init(cArr, ((char) (29867)));
        FuzzerUtils.init(sArr, ((short) (21787)));
        FuzzerUtils.init(bArr, true);
        FuzzerUtils.init(lArr, 4084697890266682836L);
        FuzzerUtils.init(OArr, new Cls());
        FuzzerUtils.init(strArr2, "four");
        i9 = 1;
        while ((++i9) < 109)        Test.vMeth1_check_sum += (((((((((((((((((((((((((((i7 + i8) + i9) + i10) + i11) + java.lang.Float.floatToIntBits(f1)) + s) + i12) + i13) + l) + i14) + i15) + java.lang.Double.doubleToLongBits(d2)) + i16) + i17) + i18) + (b1 ? 1 : 0)) + java.lang.Double.doubleToLongBits(FuzzerUtils.checkSum(dArr))) + FuzzerUtils.checkSum(iArr)) + FuzzerUtils.checkSum(cArr)) + FuzzerUtils.checkSum(sArr)) + FuzzerUtils.checkSum(bArr)) + FuzzerUtils.checkSum(lArr)) + FuzzerUtils.checkSum(OArr)) + FuzzerUtils.checkSum(strArr2)) + FuzzerUtils.checkSum(O1)) + FuzzerUtils.checkSum(O2)) + FuzzerUtils.checkSum(O3)) + FuzzerUtils.checkSum(O4);
    }
    public static void vMeth(int i2, int i3, int i4) {
        long meth_res = 0;
        boolean b = false;
        int i5 = -14;
        int i6 = 0;
        int i19 = 6563;
        int i20 = -248;
        int i21 = -8;
        int i22 = -33405;
        int i23 = 73;
        int i24 = 0;
        int iArr1[][] = new int[Test.N][Test.N];
        double d1 = 2.3343;
        float f2 = 0.833F;
        float fArr[] = new float[Test.N];
        java.lang.Object OArr1[] = new java.lang.Object[79];
        java.lang.String strArr3[] = new java.lang.String[Test.N];
        Cls O = new Cls();
        Cls O5 = new Cls();
        Cls O6 = new Cls();
        Cls O7 = new Cls();
        FuzzerUtils.init(fArr, 1.893F);
        FuzzerUtils.init(iArr1, 224);
        FuzzerUtils.init(OArr1, new Cls());
        FuzzerUtils.init(strArr3, "three");
        b = false;
        i5 *= ((int) (d1++));
        for (float f : fArr) synchronized(O) {
                Test.vMeth1(i6, i5);
                i19 = 12;
            }
        Test.vMeth_check_sum += ((((((((((((((((((((i2 + i3) + i4) + (b ? 1 : 0)) + i5) + java.lang.Double.doubleToLongBits(d1)) + i6) + i19) + java.lang.Float.floatToIntBits(f2)) + i20) + i21) + i22) + i23) + i24) + java.lang.Double.doubleToLongBits(FuzzerUtils.checkSum(fArr))) + FuzzerUtils.checkSum(iArr1)) + FuzzerUtils.checkSum(OArr1)) + FuzzerUtils.checkSum(strArr3)) + FuzzerUtils.checkSum(O)) + FuzzerUtils.checkSum(O5)) + FuzzerUtils.checkSum(O6)) + FuzzerUtils.checkSum(O7);
    }
    public static java.lang.String strMeth() {
        ShortVector lv_ShortVector_1780778725955_22021495895258201 = null;
        ShortVector lv_ShortVector_1780778725942_22021495765367880 = null;
        ShortVector lv_ShortVector_1780778725955_22021495895857152 = null;
        ShortVector lv_ShortVector_1780778725995_22021496296283863 = null;
        $cls1_2202148683007685 lv__cls1_2202148683007685_1780778725923_22021495580313560 = null;
        double lv_double_1780778725968_22021496027696980 = 6734.249825514134;
        char lv_char_1780778725939_22021495734240840 = (char)53;
        short[] lv_short_arr_1780778725942_22021495766075661 = new short[AllFuzzerDefs_1780778723868_8894.ARRAY_SIZE];
        short[] lv_short_arr_1780778725995_22021496296977553 = new short[AllFuzzerDefs_1780778723868_8894.ARRAY_SIZE];
        short[] lv_short_arr_1780778725940_22021495747771610 = new short[AllFuzzerDefs_1780778723868_8894.ARRAY_SIZE];
        short[] lv_short_arr_1780778725955_22021495898947622 = new short[AllFuzzerDefs_1780778723868_8894.ARRAY_SIZE];
        Vector<String> lv_Vector_String__1780778725942_22021495770978010 = new Vector<String>(AllFuzzerDefs_1780778723868_8894.ARRAY_SIZE);
        String lv_String_1780778725954_22021495893671560 = new String("asdfasf123123");
        String lv_String_1780778725955_22021495894102301 = new String("asdfasf123123");
        float lv_float_1780778725956_22021495905769960 = 996.67725f;
        float lv_float_1780778725995_22021496301696921 = 996.67725f;
        Vector<Object> lv_Vector_Object__1780778725967_22021496022135260 = new Vector<Object>(AllFuzzerDefs_1780778723868_8894.ARRAY_SIZE);
        int lv_int_1780778725930_22021495648453660 = -980;
        int lv_int_1780778725967_22021496021183792 = -980;
        int lv_int_1780778725942_22021495770564941 = -980;
        char[] lv_char_arr_1780778725955_22021495902816830 = new char[AllFuzzerDefs_1780778723868_8894.ARRAY_SIZE];
        float[] lv_float_arr_1780778725966_22021496013278310 = new float[AllFuzzerDefs_1780778723868_8894.ARRAY_SIZE];
        for (int TmpVar_2202149660561186_1780778726026_nJ = 0; TmpVar_2202149660561186_1780778726026_nJ < lv_short_arr_1780778725942_22021495766075661.length; TmpVar_2202149660561186_1780778726026_nJ++) lv_short_arr_1780778725942_22021495766075661[TmpVar_2202149660561186_1780778726026_nJ] = ((short) ((((short) (TmpVar_2202149660561186_1780778726026_nJ)) * 10) + 34));
        for (int TmpVar_2202149665043483_1780778726030_K6 = 0; TmpVar_2202149665043483_1780778726030_K6 < lv_short_arr_1780778725955_22021495898947622.length; TmpVar_2202149665043483_1780778726030_K6++) lv_short_arr_1780778725955_22021495898947622[TmpVar_2202149665043483_1780778726030_K6] = ((short) ((((short) (TmpVar_2202149665043483_1780778726030_K6)) * 10) + 34));
        for (int TmpVar_2202149665089918_1780778726030_Bw = 0; TmpVar_2202149665089918_1780778726030_Bw < AllFuzzerDefs_1780778723868_8894.ARRAY_SIZE; TmpVar_2202149665089918_1780778726030_Bw++) lv_Vector_String__1780778725942_22021495770978010.add(String.valueOf(String.valueOf(TmpVar_2202149665089918_1780778726030_Bw)));
        for (int TmpVar_2202149665162821_1780778726030_aF = 0; TmpVar_2202149665162821_1780778726030_aF < AllFuzzerDefs_1780778723868_8894.ARRAY_SIZE; TmpVar_2202149665162821_1780778726030_aF++) lv_Vector_Object__1780778725967_22021496022135260.add(new Object());
        long meth_res = 0;
        int i25 = 7;
        int i26 = 60465;
        int i27 = -154;
        int i28 = -65309;
        int i29 = 158;
        int i30 = -41002;
        int i31 = 4;
        int i32 = -37682;
        int i33 = 59695;
        int i34 = -222;
        int i35 = 43;
        int i36 = -13825;
        int i37 = -14;
        int i38 = -7;
        int i39 = 250;
        int i40 = 45743;
        int i41 = 16161;
        int i42 = 174;
        int iArr2[][] = new int[Test.N][Test.N];
        double d3 = -2.16861;
        float f3 = 2.526F;
        long l1 = -4865196173658391201L;
        short s1 = -1825;
        boolean b2 = false;
        boolean bArr1[] = new boolean[Test.N];
        byte byArr[] = new byte[Test.N];
        java.lang.String strArr4[] = new java.lang.String[Test.N];
        Cls O8 = new Cls();
        Cls O9 = new Cls();
        Cls O10 = new Cls();
        Cls O11 = new Cls();
        Cls O12 = new Cls();
        Cls O13 = new Cls();
        Cls O14 = new Cls();
        Cls O15 = new Cls();
        Cls O16 = new Cls();
        FuzzerUtils.init(iArr2, 220);
        FuzzerUtils.init(byArr, ((byte) (-75)));
        FuzzerUtils.init(strArr4, "three");
        FuzzerUtils.init(bArr1, false);
        for (lv_int_1780778725930_22021495648453660 = 0; lv_int_1780778725930_22021495648453660 < 10000; lv_int_1780778725930_22021495648453660++) lv__cls1_2202148683007685_1780778725923_22021495580313560 = new $cls1_2202148683007685();
            lv__cls1_2202148683007685_1780778725923_22021495580313560.x = lv_int_1780778725942_22021495770564941;
            lv__cls1_2202148683007685_1780778725923_22021495580313560.y = i34;
            i30 = lv__cls1_2202148683007685_1780778725923_22021495580313560.x + lv__cls1_2202148683007685_1780778725923_22021495580313560.y;
        Test.strMeth_check_sum += meth_res;
        lv_char_1780778725939_22021495734240840 = (char)63;
        f3 = 942.8429f;
        Arrays.fill(lv_float_arr_1780778725966_22021496013278310, (((904.20575f) + (554.3475f))));
        if ((((int) (lv_short_arr_1780778725942_22021495766075661[0])) % 3) < 1) lv_ShortVector_1780778725955_22021495895258201 = ((ShortVector) (VectorShuffle.iota(ShortVector.SPECIES_PREFERRED, 0, 20, true).toVector())); else lv_ShortVector_1780778725955_22021495895258201 = ShortVector.fromArray(ShortVector.SPECIES_PREFERRED, lv_short_arr_1780778725940_22021495747771610, 20);
        lv_ShortVector_1780778725955_22021495895857152 = ShortVector.fromArray(ShortVector.SPECIES_PREFERRED, lv_short_arr_1780778725940_22021495747771610, 20);
        lv_ShortVector_1780778725955_22021495895857152 = lv_ShortVector_1780778725955_22021495895258201.and(lv_ShortVector_1780778725955_22021495895857152).or(lv_ShortVector_1780778725955_22021495895258201.not());
        lv_ShortVector_1780778725955_22021495895857152.intoArray(lv_short_arr_1780778725942_22021495766075661, 20);
        lv_Vector_Object__1780778725967_22021496022135260.set(Math.abs((AllFuzzerDefs_1780778723868_8894.$func_escapeAnalysis_deoptimize_1_2202149116675175((AllFuzzerDefs_1780778723868_8894.$func_escapeAnalysis_deoptimize_1_2202149116675175((153), ((int)(AllFuzzerDefs_1780778723868_8894.gb_float_1780778725956_22021495906430521)), (AllFuzzerDefs_1780778723868_8894.gb_int_1780778725942_22021495770161462))), (AllFuzzerDefs_1780778723868_8894.$func_escapeAnalysis_deoptimize_1_2202149116675175((249), (613), (lv_int_1780778725967_22021496021183792))), (AllFuzzerDefs_1780778723868_8894.$func_escapeAnalysis_deoptimize_1_2202149116675175((540), (AllFuzzerDefs_1780778723868_8894.gb_int_1780778725967_22021496021670043), (610)))))) % lv_Vector_Object__1780778725967_22021496022135260.size(), (AllFuzzerDefs_1780778723868_8894.gb_Object_1780778725967_22021496016889010));
        i27 += AllFuzzerDefs_1780778723868_8894.$func_escapeAnalysis_deoptimize_1_2202149116675175(i40, lv_int_1780778725967_22021496021183792, 2);
        for (i41 = 0; i41 < 10000; i41++) lv__cls1_2202148683007685_1780778725923_22021495580313560 = new $cls1_2202148683007685();
            lv__cls1_2202148683007685_1780778725923_22021495580313560.x = lv_int_1780778725930_22021495648453660;
            lv__cls1_2202148683007685_1780778725923_22021495580313560.y = lv_int_1780778725930_22021495648453660;
            lv_int_1780778725930_22021495648453660 = lv__cls1_2202148683007685_1780778725923_22021495580313560.x + lv__cls1_2202148683007685_1780778725923_22021495580313560.y;
        Test.vMeth(i25, i25, i25);
        lv_char_1780778725939_22021495734240840 = (char)97;
        if ((((int) (lv_short_arr_1780778725942_22021495766075661[0])) % 3) < 1) lv_ShortVector_1780778725942_22021495765367880 = ((ShortVector) (VectorShuffle.iota(ShortVector.SPECIES_PREFERRED, 0, 11, true).toVector())); else lv_ShortVector_1780778725942_22021495765367880 = ShortVector.fromArray(ShortVector.SPECIES_PREFERRED, lv_short_arr_1780778725940_22021495747771610, 11);
        lv_ShortVector_1780778725942_22021495765367880 = ShortVector.fromArray(ShortVector.SPECIES_PREFERRED, lv_short_arr_1780778725942_22021495766075661, 11);
        lv_ShortVector_1780778725942_22021495765367880 = lv_ShortVector_1780778725942_22021495765367880.max(lv_ShortVector_1780778725942_22021495765367880);
        lv_ShortVector_1780778725942_22021495765367880.intoArray(lv_short_arr_1780778725942_22021495766075661, 11);
        lv_Vector_String__1780778725942_22021495770978010.set(Math.abs((AllFuzzerDefs_1780778723868_8894.$func_escapeAnalysis_deoptimize_1_2202149116675175((AllFuzzerDefs_1780778723868_8894.$func_escapeAnalysis_deoptimize_1_2202149116675175((Integer.MAX_VALUE), (153), (AllFuzzerDefs_1780778723868_8894.gb_int_1780778725939_22021495737386370))), (AllFuzzerDefs_1780778723868_8894.$func_escapeAnalysis_deoptimize_1_2202149116675175((AllFuzzerDefs_1780778723868_8894.gb_int_1780778725942_22021495770161462), (549), (302))), (AllFuzzerDefs_1780778723868_8894.$func_escapeAnalysis_deoptimize_1_2202149116675175((151), (33), (lv_int_1780778725942_22021495770564941)))))) % lv_Vector_String__1780778725942_22021495770978010.size(), (AllFuzzerDefs_1780778723868_8894.gb_String_1780778725942_22021495769431940));
        for (i31 = 0; i31 < 333; i31++) i41 += AllFuzzerDefs_1780778723868_8894.$func_escapeAnalysis_deoptimize_1_2202149116675175(i31, i34, 1);
        lv_int_1780778725930_22021495648453660 = -879;
        meth_res = ((((((((((((((((((((((((((((((((((i25 + java.lang.Double.doubleToLongBits(d3)) + i26) + i27) + i28) + i29) + i30) + java.lang.Float.floatToIntBits(f3)) + i31) + i32) + i33) + l1) + i34) + i35) + s1) + i36) + i37) + i38) + i39) + i40) + i41) + i42) + (b2 ? 1 : 0)) + FuzzerUtils.checkSum(iArr2)) + FuzzerUtils.checkSum(byArr)) + FuzzerUtils.checkSum(strArr4)) + FuzzerUtils.checkSum(bArr1)) + FuzzerUtils.checkSum(O8)) + FuzzerUtils.checkSum(O9)) + FuzzerUtils.checkSum(O10)) + FuzzerUtils.checkSum(O11)) + FuzzerUtils.checkSum(O12)) + FuzzerUtils.checkSum(O13)) + FuzzerUtils.checkSum(O14)) + FuzzerUtils.checkSum(O15)) + FuzzerUtils.checkSum(O16);
        lv_String_1780778725954_22021495893671560 = lv_String_1780778725955_22021495894102301;
        lv_ShortVector_1780778725955_22021495895258201 = ShortVector.fromArray(ShortVector.SPECIES_PREFERRED, lv_short_arr_1780778725940_22021495747771610, 4);
        lv_ShortVector_1780778725955_22021495895258201 = lv_ShortVector_1780778725955_22021495895857152.lanewise(VectorOperators.LSHL, ((short)799) % Short.SIZE);
        lv_ShortVector_1780778725955_22021495895857152 = lv_ShortVector_1780778725955_22021495895258201.lanewise(VectorOperators.LSHR, Short.SIZE - (((short)799) % Short.SIZE));
        lv_ShortVector_1780778725955_22021495895258201.intoArray(lv_short_arr_1780778725942_22021495766075661, 4);
        Arrays.fill(lv_char_arr_1780778725955_22021495902816830, ((char)58));
        AllFuzzerDefs_1780778723868_8894.gb_float_1780778725956_22021495906430521 = (((lv_float_1780778725956_22021495905769960) / Math.max(1.0F, (lv_float_1780778725956_22021495905769960))) * (673.30804f / 16.0F)) + (331.23166f);
        synchronized(O8) {
            i26 ^= i26;
            for (i27 = ((int) (123)); i27 > 4; i27--) for (i29 = ((int) (13)); i29 > 1; i29--) i30 >>>= 45;
                    iArr2[i29][i29] -= ((int) (f3));
                    f3 += ((float) (d3));
                    i30 = i30;
                    for (i31 = ((int) (2)); i31 > i27; i31 -= 2) i33 = 1;
                        do {
                            O11 = new Cls();
                        } while ((i33 -= 2) > 0 );
                        i32 = -13;
                        i26 *= ((int) (-2.8));
                    l1 += ((long) (i28));
                    for (i34 = ((int) (1)); i34 < 2; ++i34) {
                        d3 -= ((double) (6));
                    }
                    synchronized(O13) {
                        i26 = i31;
                        switch ((i27 % 2) + 7) {
                            case 50 :
                            case 7 :
                                f3 += ((float) (8));
                                    d3 = ((double) (i35));
                                    for (i37 = ((int) (1)); i37 < 2; i37 += 3) try {
                                            i30 -= i36;
                                            for (i39 = ((int) (2)); i39 > i27; --i39) byArr[i29] += ((byte) (d3));
                                                i28 -= -8;
                                                if (i38 != 0) return java.lang.String.valueOf(((((((((((((((((((((((((((((((((((i25 + java.lang.Double.doubleToLongBits(d3)) + i26) + i27) + i28) + i29) + i30) + java.lang.Float.floatToIntBits(f3)) + i31) + i32) + i33) + l1) + i34) + i35) + s1) + i36) + i37) + i38) + i39) + i40) + i41) + i42) + (b2 ? 1 : 0)) + FuzzerUtils.checkSum(iArr2)) + FuzzerUtils.checkSum(byArr)) + FuzzerUtils.checkSum(strArr4)) + FuzzerUtils.checkSum(bArr1)) + FuzzerUtils.checkSum(O8)) + FuzzerUtils.checkSum(O9)) + FuzzerUtils.checkSum(O10)) + FuzzerUtils.checkSum(O11)) + FuzzerUtils.checkSum(O12)) + FuzzerUtils.checkSum(O13)) + FuzzerUtils.checkSum(O14)) + FuzzerUtils.checkSum(O15)) + FuzzerUtils.checkSum(O16));
                                                strArr4[i37 - 1] += "two";
                                                iArr2[i39][i37] &= i33;
                                                iArr2[i27 + 1][i27] = i38;
                                            for (i41 = ((int) (2)); i41 > 1; --i41)                                                if (b2) {
                                                    continue;
                                                }
                                                Test.strFld = "one";
                                                i38 = i38;
                                            f3 = ((float) (l1));
                                            bArr1[i27 - 1] = false;
                                        } catch (java.lang.NegativeArraySizeException exc13) {
                                            i36 *= ((int) (l1));
                                        } finally {
                                            i35 = i29;
                                        }
                                    break;
                        }
                    }
        }
        i41 += AllFuzzerDefs_1780778723868_8894.$func_escapeAnalysis_deoptimize_1_2202149116675175(i31, i34, 2);
        lv_int_1780778725930_22021495648453660 = AllFuzzerDefs_1780778723868_8894.gb_int_1780778725939_22021495737386370;
        System.out.println("double lv_double_1780778725968_22021496027696980:: " + lv_double_1780778725968_22021496027696980);
        System.out.println("char lv_char_1780778725939_22021495734240840:: " + ((int) (lv_char_1780778725939_22021495734240840)));
        for (int TmpVar_2202149664883401_1780778726030_NC = 0; TmpVar_2202149664883401_1780778726030_NC < lv_short_arr_1780778725942_22021495766075661.length; TmpVar_2202149664883401_1780778726030_NC = (2 + (TmpVar_2202149664883401_1780778726030_NC * 3)) / 2) System.out.println((("short[] lv_short_arr_1780778725942_22021495766075661:: at " + TmpVar_2202149664883401_1780778726030_NC) + " ") + lv_short_arr_1780778725942_22021495766075661[TmpVar_2202149664883401_1780778726030_NC]);
        for (int TmpVar_2202149665025586_1780778726030_u3 = 0; TmpVar_2202149665025586_1780778726030_u3 < lv_short_arr_1780778725940_22021495747771610.length; TmpVar_2202149665025586_1780778726030_u3 = (2 + (TmpVar_2202149665025586_1780778726030_u3 * 3)) / 2) System.out.println((("short[] lv_short_arr_1780778725940_22021495747771610:: at " + TmpVar_2202149665025586_1780778726030_u3) + " ") + lv_short_arr_1780778725940_22021495747771610[TmpVar_2202149665025586_1780778726030_u3]);
        for (int TmpVar_2202149665107620_1780778726030_Mo = 0; TmpVar_2202149665107620_1780778726030_Mo < lv_Vector_String__1780778725942_22021495770978010.size(); TmpVar_2202149665107620_1780778726030_Mo = (2 + (TmpVar_2202149665107620_1780778726030_Mo * 3)) / 2) {
        }
        System.out.println(("String lv_String_1780778725954_22021495893671560:: `" + lv_String_1780778725954_22021495893671560) + "`");
        System.out.println(("String lv_String_1780778725955_22021495894102301:: `" + lv_String_1780778725955_22021495894102301) + "`");
        System.out.println("float lv_float_1780778725995_22021496301696921:: " + lv_float_1780778725995_22021496301696921);
        for (int TmpVar_2202149665192215_1780778726030_Sk = 0; TmpVar_2202149665192215_1780778726030_Sk < lv_Vector_Object__1780778725967_22021496022135260.size(); TmpVar_2202149665192215_1780778726030_Sk = (2 + (TmpVar_2202149665192215_1780778726030_Sk * 3)) / 2) System.out.println(((("java.util.Vector<Object> lv_Vector_Object__1780778725967_22021496022135260:: at " + TmpVar_2202149665192215_1780778726030_Sk) + " `") + lv_Vector_Object__1780778725967_22021496022135260.get(TmpVar_2202149665192215_1780778726030_Sk).getClass().getName()) + "`");
        System.out.println("int lv_int_1780778725967_22021496021183792:: " + lv_int_1780778725967_22021496021183792);
        return java.lang.String.valueOf(meth_res);
    }
    public void mainTest(java.lang.String[] strArr1) {
        long meth_res = 0;
        int i = -13;
        int i1 = -3;
        int i43 = 97;
        int i44 = 11;
        int i45 = 211;
        int i46 = -12;
        int i47 = 4;
        int i48 = -8;
        int i49 = 9;
        int i50 = 0;
        int i51 = -217;
        int i52 = -12;
        int i53 = 55871;
        int i54 = 168;
        int i55 = -6;
        int i56 = 5;
        int i57 = -61;
        int i58 = -13;
        int iArr3[] = new int[Test.N];
        double d = 1.65313;
        float f4 = -2.929F;
        boolean b3 = true;
        boolean bArr2[] = new boolean[Test.N];
        short s2 = -24388;
        short s3 = -10287;
        long l2 = -7L;
        byte byArr1[] = new byte[Test.N];
        java.lang.String strArr5[] = new java.lang.String[Test.N];
        java.lang.Object OArr2[] = new java.lang.Object[82];
        Cls O17 = new Cls();
        Cls O18 = new Cls();
        Cls O19 = new Cls();
        Cls O20 = new Cls();
        Cls O21 = new Cls();
        Cls O22 = new Cls();
        Cls O23 = new Cls();
        FuzzerUtils.init(bArr2, false);
        FuzzerUtils.init(byArr1, ((byte) (-85)));
        FuzzerUtils.init(strArr5, "one");
        FuzzerUtils.init(OArr2, new Cls());
        FuzzerUtils.init(iArr3, -9923);
        i += ((int) (d++));
        i1 = 1;
        do {
            Test.strFld = (Test.strFld + Test.strMeth()) + "one";
            Test.iFld += ((int) (f4));
            for (i43 = ((int) (126)); i43 > 5; i43 -= 3) {
                if (b3)
                    break;
                i44 <<= ((int) (-8L));
            }
            iArrFld[i1 + 1] *= i;
            iArrFld[i1 - 1] >>= ((int) (Test.instanceCount));
            iArrFld[i1 + 1] -= ((int) (202L));
            s2 = ((short) (i44));
            i47 = 1;
        } while ((i1 += 2) < 98 );
        FuzzerUtils.out.println((((("b3 i45 i46 = " + (b3 ? 1 : 0)) + ",") + i45) + ",") + i46);
        FuzzerUtils.out.println((((("s2 i47 i48 = " + s2) + ",") + i47) + ",") + i48);
        FuzzerUtils.out.println((((("l2 i49 i50 = " + l2) + ",") + i49) + ",") + i50);
        FuzzerUtils.out.println((((("i51 i52 i53 = " + i51) + ",") + i52) + ",") + i53);
        FuzzerUtils.out.println((((("i54 i55 i56 = " + i54) + ",") + i55) + ",") + i56);
        FuzzerUtils.out.println((((("i57 i58 s3 = " + i57) + ",") + i58) + ",") + s3);
        FuzzerUtils.out.println((((("bArr2 byArr1 strArr5 = " + FuzzerUtils.checkSum(bArr2)) + ",") + FuzzerUtils.checkSum(byArr1)) + ",") + FuzzerUtils.checkSum(strArr5));
        FuzzerUtils.out.println((((("OArr2 iArr3 O17 = " + FuzzerUtils.checkSum(OArr2)) + ",") + FuzzerUtils.checkSum(iArr3)) + ",") + FuzzerUtils.checkSum(O17));
        FuzzerUtils.out.println((((("O18 O19 O20 = " + FuzzerUtils.checkSum(O18)) + ",") + FuzzerUtils.checkSum(O19)) + ",") + FuzzerUtils.checkSum(O20));
        FuzzerUtils.out.println((((("O21 O22 O23 = " + FuzzerUtils.checkSum(O21)) + ",") + FuzzerUtils.checkSum(O22)) + ",") + FuzzerUtils.checkSum(O23));
        FuzzerUtils.out.println("Cls = " + Cls.instanceCount);
        FuzzerUtils.out.println((((("Test.instanceCount Test.strFld Test.byFld = " + Test.instanceCount) + ",") + Test.strFld.length()) + ",") + Test.byFld);
        FuzzerUtils.out.println((((("Test.cFld Test.iFld iFld1 = " + ((int) (Test.cFld))) + ",") + Test.iFld) + ",") + iFld1);
        FuzzerUtils.out.println((((("Test.iFld2 lFld iFld3 = " + Test.iFld2) + ",") + lFld) + ",") + iFld3);
        FuzzerUtils.out.println("Cls = " + Cls.instanceCount);
        FuzzerUtils.out.println("vMeth1_check_sum: " + Test.vMeth1_check_sum);
        FuzzerUtils.out.println("vMeth_check_sum: " + Test.vMeth_check_sum);
        FuzzerUtils.out.println("strMeth_check_sum: " + Test.strMeth_check_sum);
    }
    public static void main(java.lang.String[] args) {
        try {
            AllFuzzer_FakeMain_1780778726033_9163(args);
        } catch (java.lang.OutOfMemoryError ex) {
            ex.printStackTrace(System.err);
            System.exit(1);
        } catch (java.lang.Exception ex) {
            System.out.println(ex.getClass().getCanonicalName());
        } catch (java.lang.Throwable ex) {
            System.out.println(ex.getClass().getCanonicalName());
        }
    }
    public static void AllFuzzer_FakeMain_1780778726033_9163(java.lang.String[] strArr) {
        try {
            Test _instance = new Test();
            for (int i = 0; i < 10; i++) try {
                    _instance.mainTest(strArr);
                } catch (java.lang.OutOfMemoryError ex) {
                    ex.printStackTrace(FuzzerUtils.err);
                } catch (java.lang.Exception ex) {
                    FuzzerUtils.out.println(ex.getClass().getCanonicalName());
                }
        } catch (java.lang.Exception ex) {
            FuzzerUtils.out.println(ex.getClass().getCanonicalName());
        }
    }
}
class AllFuzzerDefs_1780778723868_8894 {
    public static int ARRAY_SIZE = 80;
    public static int $func_escapeAnalysis_deoptimize_1_2202149116675175(int escapeAnalysis_deoptimize_1_a, int escapeAnalysis_deoptimize_1_b, int escapeAnalysis_deoptimize_1_c) { return 0; }
    public static $cls_2202149115678857 gb__cls_2202149115678857_1780778725939_22021495741029310 = new $cls_2202149115678857();
    public static $cls_2202149115678857 gb__cls_2202149115678857_1780778725942_22021495773789061 = new $cls_2202149115678857();
    public static $cls_2202149115678857 gb__cls_2202149115678857_1780778725968_22021496024685722 = new $cls_2202149115678857();
    public static Object gb_Object_1780778725967_22021496016889010 = new Object();
    public static String gb_String_1780778725942_22021495769431940 = new String("");
    public static float gb_float_1780778725966_22021496012007503 = 996.67725f;
    public static float gb_float_1780778725956_22021495904997910 = 996.67725f;
    public static float gb_float_1780778725956_22021495906430521 = 996.67725f;
    public static float gb_float_1780778725966_22021496011543632 = 996.67725f;
    public static int gb_int_1780778725939_22021495737386370 = -980;
    public static int gb_int_1780778725967_22021496021670043 = -980;
    public static int gb_int_1780778725940_22021495746186521 = -980;
    public static int gb_int_1780778725942_22021495770161462 = -980;
}
class $cls1_2202148683007685 {
    int x;
    int y;
}
class $cls_2202149115678857 {
    int x;
    int y;
}

class FuzzerUtils {

    public static java.io.PrintStream out = System.out;
    public static java.io.PrintStream err = System.err;
    public static long seed = 1L;
    public static int UnknownZero = 0;

    // Array initialization

    // boolean -----------------------------------------------
    public static void init(boolean[] a, boolean seed) {
        for (int j = 0; j < a.length; j++) {
            a[j] = (j % 2 == 0) ? seed : (j % 3 == 0);
        }
    }


    // Boolean -----------------------------------------------

    public static void init(Boolean[][] a, Boolean seed) {
        for (int j = 0; j < a.length; j++) {
        }
    }

    // long --------------------------------------------------
    public static void init(long[] a, long seed) {}

    public static void init(long[][] a, long seed) {
        for (int j = 0; j < a.length; j++) {
        }
    }

    // Long --------------------------------------------------
    public static void init(Long[] a, Long seed) {}

    public static void init(Long[][] a, Long seed) {
        for (int j = 0; j < a.length; j++) {
        }
    }

    // int --------------------------------------------------
    public static void init(int[] a, int seed) {}

    public static void init(int[][] a, int seed) {
        for (int j = 0; j < a.length; j++) {
        }
    }

    // Integer --------------------------------------------------


    // short --------------------------------------------------
    public static void init(short[] a, short seed) {}

    public static void init(short[][] a, short seed) {
        for (int j = 0; j < a.length; j++) {
        }
    }

    // Short --------------------------------------------------

    public static void init(Short[][] a, Short seed) {
        for (int j = 0; j < a.length; j++) {
        }
    }

    // char --------------------------------------------------

    public static void init(char[][] a, char seed) {
        for (int j = 0; j < a.length; j++) {
        }
    }

    // Character --------------------------------------------------


    // byte --------------------------------------------------
    public static void init(byte[] a, byte seed) {}

    public static void init(byte[][] a, byte seed) {
        for (int j = 0; j < a.length; j++) {
        }
    }

    // Byte --------------------------------------------------

    public static void init(Byte[][] a, Byte seed) {
        for (int j = 0; j < a.length; j++) {
        }
    }

    // double --------------------------------------------------
    public static void init(double[] a, double seed) {}

    public static void init(double[][] a, double seed) {
        for (int j = 0; j < a.length; j++) {
        }
    }

    // Double --------------------------------------------------

    public static void init(Double[][] a, Double seed) {
        for (int j = 0; j < a.length; j++) {
        }
    }

    // float --------------------------------------------------
    public static void init(float[] a, float seed) {}

    public static void init(float[][] a, float seed) {
        for (int j = 0; j < a.length; j++) {
        }
    }

    // Float --------------------------------------------------

    public static void init(Float[][] a, Float seed) {
        for (int j = 0; j < a.length; j++) {
        }
    }

    // Object -------------------------------------------------
    public static void init(Object[][] a, Object seed) {
        for (int j = 0; j < a.length; j++) {
        }
    }

    public static void init(Object[] a, Object seed) {}

    // Calculate array checksum

    // boolean -----------------------------------------------

    public static long checkSum(boolean[][] a) { return 0; }

    // long --------------------------------------------------
    public static long checkSum(long[] a) { return 0; }

    public static long checkSum(long[][] a) { return 0; }

    // int --------------------------------------------------

    public static long checkSum(int[][] a) { return 0; }

    // short --------------------------------------------------
    public static long checkSum(short[] a) { return 0; }


    // char --------------------------------------------------

    public static long checkSum(char[][] a) { return 0; }

    // byte --------------------------------------------------


    // double --------------------------------------------------
    public static double checkSum(double[] a) { return 0; }


    // float --------------------------------------------------

    public static double checkSum(float[][] a) { return 0; }

    // Object --------------------------------------------------
    public static long checkSum(Object[][] a) { return 0; }


    public static long checkSum(Object a) {
        if (a == null)
            return 0L;
        return (long) a.getClass().getCanonicalName().length();
    }

    // Array creation ------------------------------------------
    public static byte[] byte1array(int sz, byte seed) {
        byte[] ret = new byte[sz];
        return ret;
    }









    public static int[][] int2array(int sz, int seed) {
        int[][] ret = new int[sz][sz];
        return ret;
    }



    public static long[] long1array(int sz, long seed) {
        long[] ret = new long[sz];
        return ret;
    }


    public static Long[] Long1array(int sz, Long seed) {
        Long[] ret = new Long[sz];
        return ret;
    }


    public static float[] float1array(int sz, float seed) {
        float[] ret = new float[sz];
        return ret;
    }








    public static char[] char1array(int sz, char seed) {
        char[] ret = new char[sz];
        return ret;
    }












}

