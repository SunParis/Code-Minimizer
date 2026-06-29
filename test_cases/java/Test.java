
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
        while ((++i9) < 109) {
            for (i10 = ((int) (14)); i10 > i9; i10 -= 2) {
                f1 = ((float) (79L));
                i11 >>= i7;
                i12 = 1;
                do {
                    synchronized(O2) {
                        dArr[i10 - 1] -= ((double) (l));
                        iArr[i9] = ((int) (27508L));
                        for (i14 = ((int) (1)); 1 < i14; i14 -= 2) {
                            switch (((i10 % 9) * 5) + 88) {
                                case 133 :
                                case 116 :
                                    {
                                        d2 = ((double) (2.129F));
                                    }
                                case 91 :
                                    {
                                        cArr[i12][i9] >>= ((char) (i15));
                                    }
                                case 114 :
                                    {
                                        l >>= Cls.instanceCount;
                                        break;
                                    }
                                case 103 :
                                    {
                                        O3 = O3;
                                        if (i9 != 0) {
                                            Test.vMeth1_check_sum += (((((((((((((((((((((((((((i7 + i8) + i9) + i10) + i11) + java.lang.Float.floatToIntBits(f1)) + s) + i12) + i13) + l) + i14) + i15) + java.lang.Double.doubleToLongBits(d2)) + i16) + i17) + i18) + (b1 ? 1 : 0)) + java.lang.Double.doubleToLongBits(FuzzerUtils.checkSum(dArr))) + FuzzerUtils.checkSum(iArr)) + FuzzerUtils.checkSum(cArr)) + FuzzerUtils.checkSum(sArr)) + FuzzerUtils.checkSum(bArr)) + FuzzerUtils.checkSum(lArr)) + FuzzerUtils.checkSum(OArr)) + FuzzerUtils.checkSum(strArr2)) + FuzzerUtils.checkSum(O1)) + FuzzerUtils.checkSum(O2)) + FuzzerUtils.checkSum(O3)) + FuzzerUtils.checkSum(O4);
                                            return;
                                        }
                                        iArr[i10 - 1] = i10;
                                        switch ((i9 % 2) + 77) {
                                            case 77 :
                                                {
                                                    f1 += ((float) (7L));
                                                    f1 -= ((float) (i16));
                                                    i16 *= i9;
                                                    i11 = i8;
                                                    break;
                                                }
                                            case 78 :
                                                {
                                                    sArr[i14] = ((short) (-77));
                                                    Test.strFld = Test.strFld;
                                                    switch (((6 >>> 1) % 7) + 62) {
                                                        case 62 :
                                                            {
                                                                for (i17 = ((int) (1)); i17 < 1; i17 += 3) {
                                                                    i8 = i10;
                                                                    switch (((i15 >>> 1) % 9) + 40) {
                                                                        case 40 :
                                                                            {
                                                                                bArr[i17][i14 + 1] = false;
                                                                                break;
                                                                            }
                                                                        case 41 :
                                                                            {
                                                                                d2 /= ((double) (((long) (f1)) | 1));
                                                                                break;
                                                                            }
                                                                        case 42 :
                                                                            {
                                                                                bArr[i10 + 1][i17 + 1] = b1;
                                                                                iArr[i12 + 1] = i14;
                                                                                break;
                                                                            }
                                                                        case 43 :
                                                                            {
                                                                                cArr[i10] = cArr[i12 - 1];
                                                                                i15 /= ((int) (i11 | 1));
                                                                                i18 *= i15;
                                                                                l = ((long) (i14));
                                                                                lArr[i14] -= l;
                                                                                break;
                                                                            }
                                                                        case 44 :
                                                                            {
                                                                                if (b1) {
                                                                                    continue;
                                                                                }
                                                                                iArr = iArr;
                                                                                Test.instanceCount %= i8 | 1;
                                                                                i15 = i9;
                                                                                OArr[(42951 >>> 1) % 46] = O1;
                                                                                break;
                                                                            }
                                                                        case 45 :
                                                                            {
                                                                                i18 -= ((int) (Test.cFld));
                                                                                Test.strFld = "hhhh";
                                                                                Test.strFld += "two";
                                                                                break;
                                                                            }
                                                                        case 46 :
                                                                        case 47 :
                                                                            {
                                                                                strArr2[i17] = Test.strFld;
                                                                                break;
                                                                            }
                                                                        case 48 :
                                                                            {
                                                                                l = ((long) (i13));
                                                                                break;
                                                                            }
                                                                        default :
                                                                            {
                                                                                i15 += -36;
                                                                            }
                                                                    }
                                                                }
                                                            }
                                                        case 63 :
                                                            {
                                                                i18 >>= ((int) (l));
                                                                break;
                                                            }
                                                        case 64 :
                                                        case 65 :
                                                            {
                                                                i18 = -121;
                                                                break;
                                                            }
                                                        case 66 :
                                                            {
                                                                Test.instanceCount -= ((long) (i10));
                                                            }
                                                        case 67 :
                                                            {
                                                                bArr[i9 - 1][i9 + 1] = b1;
                                                                break;
                                                            }
                                                        case 68 :
                                                            {
                                                                i18 *= ((int) (d2));
                                                                break;
                                                            }
                                                    }
                                                }
                                            default :
                                                {
                                                    Test.instanceCount = ((long) (i11));
                                                }
                                        }
                                    }
                                case 106 :
                                    {
                                        Cls.instanceCount -= ((long) (-70));
                                        break;
                                    }
                                case 120 :
                                    {
                                        O1 = O2;
                                        break;
                                    }
                                case 92 :
                                    {
                                        i7 += ((int) (d2));
                                        break;
                                    }
                                case 95 :
                                    {
                                        i8 >>>= ((int) (l));
                                        break;
                                    }
                                default :
                                    {
                                        i8 *= ((int) (60405L));
                                    }
                            }
                        }
                    }
                } while ((++i12) < 1 );
            }
        }
        Test.vMeth1_check_sum += (((((((((((((((((((((((((((i7 + i8) + i9) + i10) + i11) + java.lang.Float.floatToIntBits(f1)) + s) + i12) + i13) + l) + i14) + i15) + java.lang.Double.doubleToLongBits(d2)) + i16) + i17) + i18) + (b1 ? 1 : 0)) + java.lang.Double.doubleToLongBits(FuzzerUtils.checkSum(dArr))) + FuzzerUtils.checkSum(iArr)) + FuzzerUtils.checkSum(cArr)) + FuzzerUtils.checkSum(sArr)) + FuzzerUtils.checkSum(bArr)) + FuzzerUtils.checkSum(lArr)) + FuzzerUtils.checkSum(OArr)) + FuzzerUtils.checkSum(strArr2)) + FuzzerUtils.checkSum(O1)) + FuzzerUtils.checkSum(O2)) + FuzzerUtils.checkSum(O3)) + FuzzerUtils.checkSum(O4);
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
        for (float f : fArr) {
            synchronized(O) {
                Test.vMeth1(i6, i5);
                i19 = 12;
                do {
                    Test.strFld = "";
                    Test.strFld += Test.strFld;
                    try {
                        iArr1[i19 + 1][i19] = i4 / (-46342);
                        i3 = 13792 % iArr1[i19 - 1][i19 - 1];
                        iArr1[i19][i19 - 1] = (-155) % i19;
                    } catch (java.lang.ArithmeticException a_e) {
                    }
                    Test.cFld |= ((char) (Cls.instanceCount));
                    switch ((i19 % 1) + 122) {
                        case 122 :
                            {
                                for (f2 = ((float) (1)); f2 < 1; ++f2) {
                                    i5 = ((int) (Test.instanceCount));
                                    Test.lArrFld[i19] = ((long) (76));
                                    for (i21 = ((int) (1)); i21 < 1; ++i21) {
                                        switch (((i19 % 3) * 5) + 65) {
                                            case 77 :
                                                {
                                                    f -= ((float) (i22));
                                                    d1 *= ((double) (i19));
                                                    b = b;
                                                    iArr1[i21 - 1][((int) (f2 + 1))] = ((int) (1901L));
                                                    for (i23 = ((int) (1)); i23 > 1; --i23) {
                                                        OArr1[(i6 >>> 1) % 79] = new Cls();
                                                        switch ((i23 % 9) + 7) {
                                                            case 7 :
                                                                {
                                                                    iArr1[i21 - 1][(49 >>> 1) % Test.N] <<= ((int) (Cls.instanceCount));
                                                                    i4 -= i23;
                                                                    i20 = ((int) (Test.instanceCount));
                                                                    break;
                                                                }
                                                            case 8 :
                                                                {
                                                                    i6 *= i20;
                                                                    d1 *= ((double) (-155));
                                                                    f += ((float) (i3));
                                                                    Test.strFld = Test.strFld;
                                                                    switch ((i21 % 3) + 58) {
                                                                        case 58 :
                                                                            {
                                                                                i6 += ((int) (Test.instanceCount));
                                                                                Test.instanceCount = ((long) (i2));
                                                                                strArr3[((int) (f2))] = "two";
                                                                                Test.instanceCount += ((long) (i21));
                                                                                break;
                                                                            }
                                                                        case 59 :
                                                                            {
                                                                                iArr1[i21 + 1][i23] += i21;
                                                                                iArr1[i19][((int) (f2 + 1))] |= 1;
                                                                                break;
                                                                            }
                                                                        case 60 :
                                                                    }
                                                                    Test.instanceCount += ((long) (-81.367));
                                                                    Test.strFld = "four";
                                                                    i22 |= i6;
                                                                    if (i21 != 0) {
                                                                        Test.vMeth_check_sum += ((((((((((((((((((((i2 + i3) + i4) + (b ? 1 : 0)) + i5) + java.lang.Double.doubleToLongBits(d1)) + i6) + i19) + java.lang.Float.floatToIntBits(f2)) + i20) + i21) + i22) + i23) + i24) + java.lang.Double.doubleToLongBits(FuzzerUtils.checkSum(fArr))) + FuzzerUtils.checkSum(iArr1)) + FuzzerUtils.checkSum(OArr1)) + FuzzerUtils.checkSum(strArr3)) + FuzzerUtils.checkSum(O)) + FuzzerUtils.checkSum(O5)) + FuzzerUtils.checkSum(O6)) + FuzzerUtils.checkSum(O7);
                                                                        return;
                                                                    }
                                                                }
                                                            case 9 :
                                                                {
                                                                    i22 ^= i22;
                                                                    i2 >>>= i23;
                                                                    Test.byFld |= ((byte) (Cls.instanceCount));
                                                                }
                                                            case 10 :
                                                                {
                                                                    i24 >>= ((int) (-12034L));
                                                                }
                                                            case 11 :
                                                                {
                                                                    break;
                                                                }
                                                            case 12 :
                                                                {
                                                                    Test.strFld = "hhhh";
                                                                    Test.strFld += Test.strFld;
                                                                    break;
                                                                }
                                                            case 13 :
                                                                {
                                                                    i4 *= ((int) (-3677892532499366294L));
                                                                    break;
                                                                }
                                                            case 14 :
                                                                {
                                                                    break;
                                                                }
                                                            case 15 :
                                                            default :
                                                                {
                                                                    i6 = i2;
                                                                }
                                                        }
                                                    }
                                                    break;
                                                }
                                            case 67 :
                                                {
                                                    iArr1[((int) (f2))][((int) (f2))] += i2;
                                                    break;
                                                }
                                            case 75 :
                                                {
                                                    Test.strFld = "";
                                                    Test.strFld += Test.strFld;
                                                }
                                        }
                                    }
                                }
                                break;
                            }
                    }
                } while ((--i19) > 0 );
            }
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
        for (int TmpVar_2202149660561186_1780778726026_nJ = 0; TmpVar_2202149660561186_1780778726026_nJ < lv_short_arr_1780778725942_22021495766075661.length; TmpVar_2202149660561186_1780778726026_nJ++) {
            lv_short_arr_1780778725942_22021495766075661[TmpVar_2202149660561186_1780778726026_nJ] = ((short) ((((short) (TmpVar_2202149660561186_1780778726026_nJ)) * 10) + 34));
        }
        for (int TmpVar_2202149664964473_1780778726030_nN = 0; TmpVar_2202149664964473_1780778726030_nN < lv_short_arr_1780778725995_22021496296977553.length; TmpVar_2202149664964473_1780778726030_nN++) {
            lv_short_arr_1780778725995_22021496296977553[TmpVar_2202149664964473_1780778726030_nN] = ((short) ((((short) (TmpVar_2202149664964473_1780778726030_nN)) * 10) + 34));
        }
        for (int TmpVar_2202149665008467_1780778726030_H3 = 0; TmpVar_2202149665008467_1780778726030_H3 < lv_short_arr_1780778725940_22021495747771610.length; TmpVar_2202149665008467_1780778726030_H3++) {
            lv_short_arr_1780778725940_22021495747771610[TmpVar_2202149665008467_1780778726030_H3] = ((short) ((((short) (TmpVar_2202149665008467_1780778726030_H3)) * 10) + 34));
        }
        for (int TmpVar_2202149665043483_1780778726030_K6 = 0; TmpVar_2202149665043483_1780778726030_K6 < lv_short_arr_1780778725955_22021495898947622.length; TmpVar_2202149665043483_1780778726030_K6++) {
            lv_short_arr_1780778725955_22021495898947622[TmpVar_2202149665043483_1780778726030_K6] = ((short) ((((short) (TmpVar_2202149665043483_1780778726030_K6)) * 10) + 34));
        }
        for (int TmpVar_2202149665089918_1780778726030_Bw = 0; TmpVar_2202149665089918_1780778726030_Bw < AllFuzzerDefs_1780778723868_8894.ARRAY_SIZE; TmpVar_2202149665089918_1780778726030_Bw++) {
            lv_Vector_String__1780778725942_22021495770978010.add(String.valueOf(String.valueOf(TmpVar_2202149665089918_1780778726030_Bw)));
        }
        for (int TmpVar_2202149665162821_1780778726030_aF = 0; TmpVar_2202149665162821_1780778726030_aF < AllFuzzerDefs_1780778723868_8894.ARRAY_SIZE; TmpVar_2202149665162821_1780778726030_aF++) {
            lv_Vector_Object__1780778725967_22021496022135260.add(new Object());
        }
        for (int TmpVar_2202149665234815_1780778726030_O6 = 0; TmpVar_2202149665234815_1780778726030_O6 < lv_char_arr_1780778725955_22021495902816830.length; TmpVar_2202149665234815_1780778726030_O6++) {
            lv_char_arr_1780778725955_22021495902816830[TmpVar_2202149665234815_1780778726030_O6] = ((char) ((((char) (TmpVar_2202149665234815_1780778726030_O6)) * 5) + 12));
        }
        for (int TmpVar_2202149665271027_1780778726030_A7 = 0; TmpVar_2202149665271027_1780778726030_A7 < lv_float_arr_1780778725966_22021496013278310.length; TmpVar_2202149665271027_1780778726030_A7++) {
            lv_float_arr_1780778725966_22021496013278310[TmpVar_2202149665271027_1780778726030_A7] = (((float) (TmpVar_2202149665271027_1780778726030_A7)) * 1.5F) + 7.89F;
        }
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
        for (lv_int_1780778725930_22021495648453660 = 0; lv_int_1780778725930_22021495648453660 < 10000; lv_int_1780778725930_22021495648453660++) {
            lv__cls1_2202148683007685_1780778725923_22021495580313560 = new $cls1_2202148683007685();
            lv__cls1_2202148683007685_1780778725923_22021495580313560.x = lv_int_1780778725942_22021495770564941;
            lv__cls1_2202148683007685_1780778725923_22021495580313560.y = i34;
            i30 = lv__cls1_2202148683007685_1780778725923_22021495580313560.x + lv__cls1_2202148683007685_1780778725923_22021495580313560.y;
        Test.strMeth_check_sum += meth_res;
        lv_char_1780778725939_22021495734240840 = (char)63;
        f3 = 942.8429f;
        Arrays.fill(lv_float_arr_1780778725966_22021496013278310, (((904.20575f) + (554.3475f))));
        if ((((int) (lv_short_arr_1780778725942_22021495766075661[0])) % 3) < 1) {
            lv_ShortVector_1780778725955_22021495895258201 = ((ShortVector) (VectorShuffle.iota(ShortVector.SPECIES_PREFERRED, 0, 20, true).toVector()));
        } else {
            lv_ShortVector_1780778725955_22021495895258201 = ShortVector.fromArray(ShortVector.SPECIES_PREFERRED, lv_short_arr_1780778725940_22021495747771610, 20);
        }
        lv_ShortVector_1780778725955_22021495895857152 = ShortVector.fromArray(ShortVector.SPECIES_PREFERRED, lv_short_arr_1780778725940_22021495747771610, 20);
        lv_ShortVector_1780778725955_22021495895857152 = lv_ShortVector_1780778725955_22021495895258201.and(lv_ShortVector_1780778725955_22021495895857152).or(lv_ShortVector_1780778725955_22021495895258201.not());
        lv_ShortVector_1780778725955_22021495895857152.intoArray(lv_short_arr_1780778725942_22021495766075661, 20);
        if ((((lv_short_arr_1780778725940_22021495747771610 == null) || (lv_short_arr_1780778725940_22021495747771610 == null)) || (lv_short_arr_1780778725942_22021495766075661 == null)) && ((20 % 5) <= 3)) {
            lv_short_arr_1780778725940_22021495747771610[1] = ((short) (1 + lv_ShortVector_1780778725955_22021495895857152.reduceLanes(VectorOperators.ADD)));
            lv_short_arr_1780778725940_22021495747771610[2] = ((short) (1 + lv_ShortVector_1780778725955_22021495895857152.reduceLanes(VectorOperators.XOR)));
            lv_short_arr_1780778725940_22021495747771610[3] = ((short) (1 + lv_ShortVector_1780778725955_22021495895857152.reduceLanes(VectorOperators.MAX)));
        }
        lv_Vector_Object__1780778725967_22021496022135260.set(Math.abs((AllFuzzerDefs_1780778723868_8894.$func_escapeAnalysis_deoptimize_1_2202149116675175((AllFuzzerDefs_1780778723868_8894.$func_escapeAnalysis_deoptimize_1_2202149116675175((153), ((int)(AllFuzzerDefs_1780778723868_8894.gb_float_1780778725956_22021495906430521)), (AllFuzzerDefs_1780778723868_8894.gb_int_1780778725942_22021495770161462))), (AllFuzzerDefs_1780778723868_8894.$func_escapeAnalysis_deoptimize_1_2202149116675175((249), (613), (lv_int_1780778725967_22021496021183792))), (AllFuzzerDefs_1780778723868_8894.$func_escapeAnalysis_deoptimize_1_2202149116675175((540), (AllFuzzerDefs_1780778723868_8894.gb_int_1780778725967_22021496021670043), (610)))))) % lv_Vector_Object__1780778725967_22021496022135260.size(), (AllFuzzerDefs_1780778723868_8894.gb_Object_1780778725967_22021496016889010));
        for (i40 = 0; i40 < 333; i40++) {
            i27 += AllFuzzerDefs_1780778723868_8894.$func_escapeAnalysis_deoptimize_1_2202149116675175(i40, lv_int_1780778725967_22021496021183792, 1);
        lv_char_1780778725939_22021495734240840 = (char)43;
        if ((((int) (lv_short_arr_1780778725995_22021496296977553[0])) % 3) < 1) {
            lv_ShortVector_1780778725995_22021496296283863 = ((ShortVector) (VectorShuffle.iota(ShortVector.SPECIES_PREFERRED, 0, 9, true).toVector()));
        } else {
            lv_ShortVector_1780778725995_22021496296283863 = ShortVector.fromArray(ShortVector.SPECIES_PREFERRED, lv_short_arr_1780778725955_22021495898947622, 9);
        }
        lv_ShortVector_1780778725955_22021495895857152 = ShortVector.fromArray(ShortVector.SPECIES_PREFERRED, lv_short_arr_1780778725995_22021496296977553, 9);
        lv_ShortVector_1780778725955_22021495895258201 = lv_ShortVector_1780778725995_22021496296283863.sub(lv_ShortVector_1780778725955_22021495895857152);
        lv_ShortVector_1780778725955_22021495895258201.intoArray(lv_short_arr_1780778725995_22021496296977553, 9);
        if ((((lv_short_arr_1780778725955_22021495898947622 == null) || (lv_short_arr_1780778725995_22021496296977553 == null)) || (lv_short_arr_1780778725995_22021496296977553 == null)) && ((9 % 5) <= 3)) {
            lv_short_arr_1780778725955_22021495898947622[1] = ((short) (1 + lv_ShortVector_1780778725955_22021495895258201.reduceLanes(VectorOperators.ADD)));
            lv_short_arr_1780778725955_22021495898947622[2] = ((short) (1 + lv_ShortVector_1780778725955_22021495895258201.reduceLanes(VectorOperators.XOR)));
            lv_short_arr_1780778725955_22021495898947622[3] = ((short) (1 + lv_ShortVector_1780778725955_22021495895258201.reduceLanes(VectorOperators.MAX)));
        }
        Arrays.fill(lv_short_arr_1780778725995_22021496296977553, ((short)888));
        AllFuzzerDefs_1780778723868_8894.gb_float_1780778725966_22021496011543632 = (Math.max(lv_float_1780778725956_22021495905769960, lv_float_1780778725995_22021496301696921) + Math.min(lv_float_1780778725956_22021495905769960, lv_float_1780778725995_22021496301696921)) + 1.0F;
        lv_double_1780778725968_22021496027696980 = Double.NaN;
        }
        i27 += AllFuzzerDefs_1780778723868_8894.$func_escapeAnalysis_deoptimize_1_2202149116675175(i40, lv_int_1780778725967_22021496021183792, 2);
        for (i41 = 0; i41 < 10000; i41++) {
            lv__cls1_2202148683007685_1780778725923_22021495580313560 = new $cls1_2202148683007685();
            lv__cls1_2202148683007685_1780778725923_22021495580313560.x = lv_int_1780778725930_22021495648453660;
            lv__cls1_2202148683007685_1780778725923_22021495580313560.y = lv_int_1780778725930_22021495648453660;
            lv_int_1780778725930_22021495648453660 = lv__cls1_2202148683007685_1780778725923_22021495580313560.x + lv__cls1_2202148683007685_1780778725923_22021495580313560.y;
        Test.vMeth(i25, i25, i25);
        lv_char_1780778725939_22021495734240840 = (char)97;
        switch (((i25 >>> 1) % 2) + 127) {
            case 127 :
            case 128 :
                {
                    i25 &= ((int) (57568842023531987L));
                    d3 += ((double) (i25));
                }
        }
        if (((((AllFuzzerDefs_1780778723868_8894.$func_escapeAnalysis_deoptimize_1_2202149116675175((AllFuzzerDefs_1780778723868_8894.$func_escapeAnalysis_deoptimize_1_2202149116675175((AllFuzzerDefs_1780778723868_8894.gb_int_1780778725939_22021495737386370), (-717), (lv_int_1780778725930_22021495648453660))), (AllFuzzerDefs_1780778723868_8894.gb_int_1780778725940_22021495746186521), (140))) < 0) || ((AllFuzzerDefs_1780778723868_8894.$func_escapeAnalysis_deoptimize_1_2202149116675175((-436), (-6), (851))) < 0)) || ((AllFuzzerDefs_1780778723868_8894.$func_escapeAnalysis_deoptimize_1_2202149116675175((-436), (-6), (851))) > lv_short_arr_1780778725940_22021495747771610.length)) || ((AllFuzzerDefs_1780778723868_8894.$func_escapeAnalysis_deoptimize_1_2202149116675175((AllFuzzerDefs_1780778723868_8894.$func_escapeAnalysis_deoptimize_1_2202149116675175((AllFuzzerDefs_1780778723868_8894.gb_int_1780778725939_22021495737386370), (-717), (lv_int_1780778725930_22021495648453660))), (AllFuzzerDefs_1780778723868_8894.gb_int_1780778725940_22021495746186521), (140))) > (AllFuzzerDefs_1780778723868_8894.$func_escapeAnalysis_deoptimize_1_2202149116675175((-436), (-6), (851))))) {
            Arrays.fill(lv_short_arr_1780778725940_22021495747771610, 0, lv_short_arr_1780778725940_22021495747771610.length, ((short)414));
        } else {
            Arrays.fill(lv_short_arr_1780778725940_22021495747771610, (AllFuzzerDefs_1780778723868_8894.$func_escapeAnalysis_deoptimize_1_2202149116675175((AllFuzzerDefs_1780778723868_8894.$func_escapeAnalysis_deoptimize_1_2202149116675175((AllFuzzerDefs_1780778723868_8894.gb_int_1780778725939_22021495737386370), (-717), (lv_int_1780778725930_22021495648453660))), (AllFuzzerDefs_1780778723868_8894.gb_int_1780778725940_22021495746186521), (140))), (AllFuzzerDefs_1780778723868_8894.$func_escapeAnalysis_deoptimize_1_2202149116675175((-436), (-6), (851))), ((short)414));
        }
        if ((((int) (lv_short_arr_1780778725942_22021495766075661[0])) % 3) < 1) {
            lv_ShortVector_1780778725942_22021495765367880 = ((ShortVector) (VectorShuffle.iota(ShortVector.SPECIES_PREFERRED, 0, 11, true).toVector()));
        } else {
            lv_ShortVector_1780778725942_22021495765367880 = ShortVector.fromArray(ShortVector.SPECIES_PREFERRED, lv_short_arr_1780778725940_22021495747771610, 11);
        }
        lv_ShortVector_1780778725942_22021495765367880 = ShortVector.fromArray(ShortVector.SPECIES_PREFERRED, lv_short_arr_1780778725942_22021495766075661, 11);
        lv_ShortVector_1780778725942_22021495765367880 = lv_ShortVector_1780778725942_22021495765367880.max(lv_ShortVector_1780778725942_22021495765367880);
        lv_ShortVector_1780778725942_22021495765367880.intoArray(lv_short_arr_1780778725942_22021495766075661, 11);
        if ((((lv_short_arr_1780778725940_22021495747771610 == null) || (lv_short_arr_1780778725942_22021495766075661 == null)) || (lv_short_arr_1780778725942_22021495766075661 == null)) && ((11 % 5) <= 3)) {
            lv_short_arr_1780778725940_22021495747771610[1] = ((short) (1 + lv_ShortVector_1780778725942_22021495765367880.reduceLanes(VectorOperators.ADD)));
            lv_short_arr_1780778725940_22021495747771610[2] = ((short) (1 + lv_ShortVector_1780778725942_22021495765367880.reduceLanes(VectorOperators.XOR)));
            lv_short_arr_1780778725940_22021495747771610[3] = ((short) (1 + lv_ShortVector_1780778725942_22021495765367880.reduceLanes(VectorOperators.MAX)));
        }
        lv_Vector_String__1780778725942_22021495770978010.set(Math.abs((AllFuzzerDefs_1780778723868_8894.$func_escapeAnalysis_deoptimize_1_2202149116675175((AllFuzzerDefs_1780778723868_8894.$func_escapeAnalysis_deoptimize_1_2202149116675175((Integer.MAX_VALUE), (153), (AllFuzzerDefs_1780778723868_8894.gb_int_1780778725939_22021495737386370))), (AllFuzzerDefs_1780778723868_8894.$func_escapeAnalysis_deoptimize_1_2202149116675175((AllFuzzerDefs_1780778723868_8894.gb_int_1780778725942_22021495770161462), (549), (302))), (AllFuzzerDefs_1780778723868_8894.$func_escapeAnalysis_deoptimize_1_2202149116675175((151), (33), (lv_int_1780778725942_22021495770564941)))))) % lv_Vector_String__1780778725942_22021495770978010.size(), (AllFuzzerDefs_1780778723868_8894.gb_String_1780778725942_22021495769431940));
        for (i31 = 0; i31 < 333; i31++) {
            i41 += AllFuzzerDefs_1780778723868_8894.$func_escapeAnalysis_deoptimize_1_2202149116675175(i31, i34, 1);
        lv_int_1780778725930_22021495648453660 = -879;
        meth_res = ((((((((((((((((((((((((((((((((((i25 + java.lang.Double.doubleToLongBits(d3)) + i26) + i27) + i28) + i29) + i30) + java.lang.Float.floatToIntBits(f3)) + i31) + i32) + i33) + l1) + i34) + i35) + s1) + i36) + i37) + i38) + i39) + i40) + i41) + i42) + (b2 ? 1 : 0)) + FuzzerUtils.checkSum(iArr2)) + FuzzerUtils.checkSum(byArr)) + FuzzerUtils.checkSum(strArr4)) + FuzzerUtils.checkSum(bArr1)) + FuzzerUtils.checkSum(O8)) + FuzzerUtils.checkSum(O9)) + FuzzerUtils.checkSum(O10)) + FuzzerUtils.checkSum(O11)) + FuzzerUtils.checkSum(O12)) + FuzzerUtils.checkSum(O13)) + FuzzerUtils.checkSum(O14)) + FuzzerUtils.checkSum(O15)) + FuzzerUtils.checkSum(O16);
        lv_String_1780778725954_22021495893671560 = lv_String_1780778725955_22021495894102301;
        if ((((int) (lv_short_arr_1780778725942_22021495766075661[0])) % 3) < 1) {
            lv_ShortVector_1780778725955_22021495895857152 = ((ShortVector) (VectorShuffle.iota(ShortVector.SPECIES_PREFERRED, 0, 4, true).toVector()));
        } else {
            lv_ShortVector_1780778725955_22021495895857152 = ShortVector.fromArray(ShortVector.SPECIES_PREFERRED, lv_short_arr_1780778725955_22021495898947622, 4);
        }
        lv_ShortVector_1780778725955_22021495895258201 = ShortVector.fromArray(ShortVector.SPECIES_PREFERRED, lv_short_arr_1780778725940_22021495747771610, 4);
        lv_ShortVector_1780778725955_22021495895258201 = lv_ShortVector_1780778725955_22021495895857152.lanewise(VectorOperators.LSHL, ((short)799) % Short.SIZE);
        lv_ShortVector_1780778725955_22021495895857152 = lv_ShortVector_1780778725955_22021495895258201.lanewise(VectorOperators.LSHR, Short.SIZE - (((short)799) % Short.SIZE));
        lv_ShortVector_1780778725955_22021495895258201.intoArray(lv_short_arr_1780778725942_22021495766075661, 4);
        if ((((lv_short_arr_1780778725955_22021495898947622 == null) || (lv_short_arr_1780778725940_22021495747771610 == null)) || (lv_short_arr_1780778725942_22021495766075661 == null)) && ((4 % 5) <= 3)) {
            lv_short_arr_1780778725955_22021495898947622[1] = ((short) (1 + lv_ShortVector_1780778725955_22021495895258201.reduceLanes(VectorOperators.ADD)));
            lv_short_arr_1780778725955_22021495898947622[2] = ((short) (1 + lv_ShortVector_1780778725955_22021495895258201.reduceLanes(VectorOperators.XOR)));
            lv_short_arr_1780778725955_22021495898947622[3] = ((short) (1 + lv_ShortVector_1780778725955_22021495895258201.reduceLanes(VectorOperators.MAX)));
        }
        Arrays.fill(lv_char_arr_1780778725955_22021495902816830, ((char)58));
        AllFuzzerDefs_1780778723868_8894.gb_float_1780778725956_22021495906430521 = (((lv_float_1780778725956_22021495905769960) / Math.max(1.0F, (lv_float_1780778725956_22021495905769960))) * (673.30804f / 16.0F)) + (331.23166f);
        synchronized(O8) {
            i26 ^= i26;
            for (i27 = ((int) (123)); i27 > 4; i27--) {
                for (i29 = ((int) (13)); i29 > 1; i29--) {
                    i30 >>>= 45;
                    iArr2[i29][i29] -= ((int) (f3));
                    f3 += ((float) (d3));
                    i30 = i30;
                    for (i31 = ((int) (2)); i31 > i27; i31 -= 2) {
                        i33 = 1;
                        do {
                            O11 = new Cls();
                        } while ((i33 -= 2) > 0 );
                        i32 = -13;
                        i26 *= ((int) (-2.125943));
                    }
                    l1 += ((long) (i28));
                    for (i34 = ((int) (1)); i34 < 2; ++i34) {
                        d3 -= ((double) (6));
                    }
                    synchronized(O13) {
                        i26 = i31;
                        switch ((i27 % 2) + 50) {
                            case 50 :
                            case 51 :
                                {
                                    f3 += ((float) (28647));
                                    d3 = ((double) (i35));
                                    for (i37 = ((int) (1)); i37 < 2; i37 += 3) {
                                        try {
                                            i30 -= i36;
                                            for (i39 = ((int) (2)); i39 > i27; --i39) {
                                                byArr[i29] += ((byte) (d3));
                                                i28 -= -10904;
                                                if (i38 != 0) {
                                                    return java.lang.String.valueOf(((((((((((((((((((((((((((((((((((i25 + java.lang.Double.doubleToLongBits(d3)) + i26) + i27) + i28) + i29) + i30) + java.lang.Float.floatToIntBits(f3)) + i31) + i32) + i33) + l1) + i34) + i35) + s1) + i36) + i37) + i38) + i39) + i40) + i41) + i42) + (b2 ? 1 : 0)) + FuzzerUtils.checkSum(iArr2)) + FuzzerUtils.checkSum(byArr)) + FuzzerUtils.checkSum(strArr4)) + FuzzerUtils.checkSum(bArr1)) + FuzzerUtils.checkSum(O8)) + FuzzerUtils.checkSum(O9)) + FuzzerUtils.checkSum(O10)) + FuzzerUtils.checkSum(O11)) + FuzzerUtils.checkSum(O12)) + FuzzerUtils.checkSum(O13)) + FuzzerUtils.checkSum(O14)) + FuzzerUtils.checkSum(O15)) + FuzzerUtils.checkSum(O16));
                                                }
                                                strArr4[i37 - 1] += "two";
                                                iArr2[i39][i37] &= i33;
                                                iArr2[i27 + 1][i27] = i38;
                                            }
                                            for (i41 = ((int) (2)); i41 > 1; --i41) {
                                                if (i37 != 0) {
                                                    return java.lang.String.valueOf(((((((((((((((((((((((((((((((((((i25 + java.lang.Double.doubleToLongBits(d3)) + i26) + i27) + i28) + i29) + i30) + java.lang.Float.floatToIntBits(f3)) + i31) + i32) + i33) + l1) + i34) + i35) + s1) + i36) + i37) + i38) + i39) + i40) + i41) + i42) + (b2 ? 1 : 0)) + FuzzerUtils.checkSum(iArr2)) + FuzzerUtils.checkSum(byArr)) + FuzzerUtils.checkSum(strArr4)) + FuzzerUtils.checkSum(bArr1)) + FuzzerUtils.checkSum(O8)) + FuzzerUtils.checkSum(O9)) + FuzzerUtils.checkSum(O10)) + FuzzerUtils.checkSum(O11)) + FuzzerUtils.checkSum(O12)) + FuzzerUtils.checkSum(O13)) + FuzzerUtils.checkSum(O14)) + FuzzerUtils.checkSum(O15)) + FuzzerUtils.checkSum(O16));
                                                }
                                                if (b2) {
                                                    continue;
                                                }
                                                Test.strFld = "one";
                                                i38 = i38;
                                            }
                                            f3 = ((float) (l1));
                                            bArr1[i27 - 1] = false;
                                        } catch (java.lang.NegativeArraySizeException exc13) {
                                            i36 *= ((int) (l1));
                                        } finally {
                                            i35 = i29;
                                        }
                                    }
                                    break;
                                }
                        }
                    }
                }
            }
        }
        }
        i41 += AllFuzzerDefs_1780778723868_8894.$func_escapeAnalysis_deoptimize_1_2202149116675175(i31, i34, 2);
        lv_int_1780778725930_22021495648453660 = AllFuzzerDefs_1780778723868_8894.gb_int_1780778725939_22021495737386370;
        }
        }
        System.out.println("double lv_double_1780778725968_22021496027696980:: " + lv_double_1780778725968_22021496027696980);
        System.out.println("char lv_char_1780778725939_22021495734240840:: " + ((int) (lv_char_1780778725939_22021495734240840)));
        for (int TmpVar_2202149664883401_1780778726030_NC = 0; TmpVar_2202149664883401_1780778726030_NC < lv_short_arr_1780778725942_22021495766075661.length; TmpVar_2202149664883401_1780778726030_NC = (2 + (TmpVar_2202149664883401_1780778726030_NC * 3)) / 2) {
            System.out.println((("short[] lv_short_arr_1780778725942_22021495766075661:: at " + TmpVar_2202149664883401_1780778726030_NC) + " ") + lv_short_arr_1780778725942_22021495766075661[TmpVar_2202149664883401_1780778726030_NC]);
        }
        for (int TmpVar_2202149664988695_1780778726030_bh = 0; TmpVar_2202149664988695_1780778726030_bh < lv_short_arr_1780778725995_22021496296977553.length; TmpVar_2202149664988695_1780778726030_bh = (2 + (TmpVar_2202149664988695_1780778726030_bh * 3)) / 2) {
            System.out.println((("short[] lv_short_arr_1780778725995_22021496296977553:: at " + TmpVar_2202149664988695_1780778726030_bh) + " ") + lv_short_arr_1780778725995_22021496296977553[TmpVar_2202149664988695_1780778726030_bh]);
        }
        for (int TmpVar_2202149665025586_1780778726030_u3 = 0; TmpVar_2202149665025586_1780778726030_u3 < lv_short_arr_1780778725940_22021495747771610.length; TmpVar_2202149665025586_1780778726030_u3 = (2 + (TmpVar_2202149665025586_1780778726030_u3 * 3)) / 2) {
            System.out.println((("short[] lv_short_arr_1780778725940_22021495747771610:: at " + TmpVar_2202149665025586_1780778726030_u3) + " ") + lv_short_arr_1780778725940_22021495747771610[TmpVar_2202149665025586_1780778726030_u3]);
        }
        for (int TmpVar_2202149665107620_1780778726030_Mo = 0; TmpVar_2202149665107620_1780778726030_Mo < lv_Vector_String__1780778725942_22021495770978010.size(); TmpVar_2202149665107620_1780778726030_Mo = (2 + (TmpVar_2202149665107620_1780778726030_Mo * 3)) / 2) {
            System.out.println(((("java.util.Vector<String lv_Vector_String__1780778725942_22021495770978010:: at " + TmpVar_2202149665107620_1780778726030_Mo) + " `") + String.valueOf(lv_Vector_String__1780778725942_22021495770978010.get(TmpVar_2202149665107620_1780778726030_Mo))) + "`");
        }
        System.out.println(("String lv_String_1780778725954_22021495893671560:: `" + lv_String_1780778725954_22021495893671560) + "`");
        System.out.println(("String lv_String_1780778725955_22021495894102301:: `" + lv_String_1780778725955_22021495894102301) + "`");
        System.out.println("float lv_float_1780778725995_22021496301696921:: " + lv_float_1780778725995_22021496301696921);
        for (int TmpVar_2202149665192215_1780778726030_Sk = 0; TmpVar_2202149665192215_1780778726030_Sk < lv_Vector_Object__1780778725967_22021496022135260.size(); TmpVar_2202149665192215_1780778726030_Sk = (2 + (TmpVar_2202149665192215_1780778726030_Sk * 3)) / 2) {
            System.out.println(((("java.util.Vector<Object> lv_Vector_Object__1780778725967_22021496022135260:: at " + TmpVar_2202149665192215_1780778726030_Sk) + " `") + lv_Vector_Object__1780778725967_22021496022135260.get(TmpVar_2202149665192215_1780778726030_Sk).getClass().getName()) + "`");
        }
        System.out.println("int lv_int_1780778725967_22021496021183792:: " + lv_int_1780778725967_22021496021183792);
        for (int TmpVar_2202149665253362_1780778726030_a1 = 0; TmpVar_2202149665253362_1780778726030_a1 < lv_char_arr_1780778725955_22021495902816830.length; TmpVar_2202149665253362_1780778726030_a1 = (2 + (TmpVar_2202149665253362_1780778726030_a1 * 3)) / 2) {
            System.out.println((("char[] lv_char_arr_1780778725955_22021495902816830:: at " + TmpVar_2202149665253362_1780778726030_a1) + " ") + ((int) (lv_char_arr_1780778725955_22021495902816830[TmpVar_2202149665253362_1780778726030_a1])));
        }
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
                i44 <<= ((int) (-172L));
            }
            iArrFld[i1 + 1] *= i;
            iArrFld[i1 - 1] >>= ((int) (Test.instanceCount));
            iArrFld[i1 + 1] -= ((int) (202L));
            for (i45 = ((int) (105)); i45 > 4; i45--) {
                i >>>= ((int) (Cls.instanceCount));
                bArr2[i1 - 1] = b3;
                O17 = O18;
                f4 *= ((float) (i43));
                Test.lArrFld[i45] += ((long) (i));
                d += ((double) (i44));
                Test.iFld <<= i1;
                i44 -= ((int) (Cls.instanceCount));
            }
            s2 = ((short) (i44));
            i47 = 1;
            do {
                if (b3) {
                    i48 = 10;
                    do {
                        byArr1 = byArr1;
                        strArr5[i48] = Test.strFld;
                        i46 ^= ((int) (Test.instanceCount));
                        iArrFld[i1 - 1] += ((int) (d));
                        switch (((i44 >>> 1) % 5) + 126) {
                            case 126 :
                                {
                                    OArr2[((-134) >>> 1) % 82] = O17;
                                    try {
                                        i44 *= i1;
                                        switch ((i1 % 2) + 75) {
                                            case 75 :
                                                {
                                                    Test.iFld -= i48;
                                                    if (b3) {
                                                        break;
                                                    }
                                                    break;
                                                }
                                            case 76 :
                                                {
                                                    Test.iFld += 3;
                                                    f4 = ((float) (53));
                                                    switch (((i46 >>> 1) % 1) + 67) {
                                                        case 67 :
                                                            {
                                                                i46 += Test.iFld;
                                                                switch ((i47 % 1) + 15) {
                                                                    case 15 :
                                                                        {
                                                                            i &= i48;
                                                                            for (l2 = ((long) (((long) (1)) + ((long) (128)))); l2 > 1; l2--) {
                                                                                Cls.instanceCount += ((long) (f4));
                                                                                f4 = ((float) (i1));
                                                                                Test.instanceCount = ((long) (Test.iFld));
                                                                                i50 = 1;
                                                                                while ((--i50) > 0) {
                                                                                    bArr2[i1 - 1] = b3;
                                                                                    i44 -= i44;
                                                                                    i46 = 29829;
                                                                                    OArr2[(i45 >>> 1) % 82] = new Cls();
                                                                                    synchronized(O20) {
                                                                                        i -= ((int) (Test.cFld));
                                                                                        f4 += ((float) (Test.iFld));
                                                                                        d += ((double) (i43));
                                                                                        strArr5[i50 - 1] = Test.strFld;
                                                                                        i = 230;
                                                                                    }
                                                                                    iArrFld[((int) (l2))] -= ((int) (l2));
                                                                                }
                                                                                i49 = i48;
                                                                                O17 = O18;
                                                                            }
                                                                            Test.byFld *= ((byte) (f4));
                                                                            Test.lArrFld[i47] += ((long) (Test.cFld));
                                                                            synchronized(O21) {
                                                                                i46 -= i47;
                                                                                iArrFld[i48] += i45;
                                                                                i46 >>= ((int) (l2));
                                                                                i44 = -105;
                                                                                i52 = i1;
                                                                                iArrFld[i1 - 1] += i52;
                                                                            }
                                                                            for (i53 = ((int) (1)); i53 < 1; i53 += 2) {
                                                                                Test.strFld = Test.strFld;
                                                                                switch (((i48 % 2) * 5) + 97) {
                                                                                    case 105 :
                                                                                    case 100 :
                                                                                        {
                                                                                            i46 += i47;
                                                                                            break;
                                                                                        }
                                                                                    default :
                                                                                        {
                                                                                            i44 = i45;
                                                                                            Test.instanceCount = ((long) (s2));
                                                                                        }
                                                                                }
                                                                                i44 = ((int) (Cls.instanceCount));
                                                                                switch ((i47 % 3) + 93) {
                                                                                    case 93 :
                                                                                        {
                                                                                            for (i55 = ((int) (((long) (1)) + ((long) (128)))); i55 > 1; i55--) {
                                                                                                Test.strFld = Test.strFld;
                                                                                                Test.iFld -= i49;
                                                                                                i51 >>= ((int) (s2));
                                                                                                iFld1 = i45;
                                                                                                switch ((i1 * 5) + 62) {
                                                                                                    case 123 :
                                                                                                        {
                                                                                                            Test.byFld *= ((byte) (i56));
                                                                                                            Test.strFld = Test.strFld;
                                                                                                            i46 += i47;
                                                                                                            switch (((i47 % 9) * 5) + 32) {
                                                                                                                case 54 :
                                                                                                                    {
                                                                                                                        b3 = b3;
                                                                                                                        f4 += ((float) (Test.instanceCount));
                                                                                                                        switch (((i50 >>> 1) % 3) + 40) {
                                                                                                                            case 40 :
                                                                                                                                {
                                                                                                                                    Cls.instanceCount -= ((long) (i54));
                                                                                                                                    strArr5[i47 - 1] = Test.strFld;
                                                                                                                                    Cls.instanceCount += ((long) (74.308F));
                                                                                                                                    break;
                                                                                                                                }
                                                                                                                            case 41 :
                                                                                                                                {
                                                                                                                                    break;
                                                                                                                                }
                                                                                                                            case 42 :
                                                                                                                                {
                                                                                                                                    Test.strFld = Test.strFld;
                                                                                                                                    break;
                                                                                                                                }
                                                                                                                            default :
                                                                                                                                {
                                                                                                                                    iArrFld[i55] += Test.iFld;
                                                                                                                                }
                                                                                                                        }
                                                                                                                        break;
                                                                                                                    }
                                                                                                                case 62 :
                                                                                                                    {
                                                                                                                        i52 = ((int) (Test.instanceCount));
                                                                                                                        break;
                                                                                                                    }
                                                                                                                case 59 :
                                                                                                                    {
                                                                                                                        Test.instanceCount += ((long) (Test.cFld));
                                                                                                                    }
                                                                                                                case 43 :
                                                                                                                    {
                                                                                                                        i54 ^= Test.iFld;
                                                                                                                    }
                                                                                                                case 48 :
                                                                                                                    {
                                                                                                                        iArrFld[i1 + 1] *= i;
                                                                                                                        break;
                                                                                                                    }
                                                                                                                case 46 :
                                                                                                                    {
                                                                                                                        break;
                                                                                                                    }
                                                                                                                case 65 :
                                                                                                                    {
                                                                                                                        i56 += ((int) (Test.cFld));
                                                                                                                        break;
                                                                                                                    }
                                                                                                                case 67 :
                                                                                                                    {
                                                                                                                        i54 <<= ((int) (Test.instanceCount));
                                                                                                                    }
                                                                                                                case 38 :
                                                                                                                    {
                                                                                                                        i *= ((int) (f4));
                                                                                                                        break;
                                                                                                                    }
                                                                                                                default :
                                                                                                                    {
                                                                                                                        Test.iFld = i53;
                                                                                                                    }
                                                                                                            }
                                                                                                            break;
                                                                                                        }
                                                                                                    case 359 :
                                                                                                        {
                                                                                                            Test.lArrFld[i53] += Test.instanceCount;
                                                                                                            break;
                                                                                                        }
                                                                                                    case 230 :
                                                                                                        {
                                                                                                            iFld1 <<= i52;
                                                                                                            break;
                                                                                                        }
                                                                                                    case 194 :
                                                                                                        {
                                                                                                            dArrFld[i1] -= ((double) (Test.instanceCount));
                                                                                                            break;
                                                                                                        }
                                                                                                    case 72 :
                                                                                                        {
                                                                                                            Test.iFld2 -= i52;
                                                                                                            break;
                                                                                                        }
                                                                                                    case 119 :
                                                                                                        {
                                                                                                            Test.instanceCount -= ((long) (i46));
                                                                                                            break;
                                                                                                        }
                                                                                                    case 374 :
                                                                                                        {
                                                                                                            Cls.instanceCount *= ((long) (f4));
                                                                                                            break;
                                                                                                        }
                                                                                                    case 308 :
                                                                                                        {
                                                                                                            i51 = i57;
                                                                                                            break;
                                                                                                        }
                                                                                                    case 189 :
                                                                                                        {
                                                                                                            i54 += ((int) (Test.instanceCount));
                                                                                                            break;
                                                                                                        }
                                                                                                    case 276 :
                                                                                                    case 289 :
                                                                                                        {
                                                                                                            i51 = i55;
                                                                                                            break;
                                                                                                        }
                                                                                                    case 233 :
                                                                                                        {
                                                                                                            i51 += -204;
                                                                                                            break;
                                                                                                        }
                                                                                                    case 288 :
                                                                                                        {
                                                                                                            Test.byFld >>= ((byte) (i52));
                                                                                                            break;
                                                                                                        }
                                                                                                    case 99 :
                                                                                                        {
                                                                                                            d -= ((double) (i50));
                                                                                                        }
                                                                                                    case 334 :
                                                                                                        {
                                                                                                            strArr5[i53 + 1] += Test.strFld;
                                                                                                            break;
                                                                                                        }
                                                                                                    case 278 :
                                                                                                        {
                                                                                                            i51 = i56;
                                                                                                            break;
                                                                                                        }
                                                                                                    case 101 :
                                                                                                        {
                                                                                                            i44 += ((int) (f4));
                                                                                                            break;
                                                                                                        }
                                                                                                    case 388 :
                                                                                                        {
                                                                                                            i56 = i56;
                                                                                                            break;
                                                                                                        }
                                                                                                    case 406 :
                                                                                                        {
                                                                                                            Test.lArrFld[i1] = Test.instanceCount;
                                                                                                            break;
                                                                                                        }
                                                                                                    case 244 :
                                                                                                        {
                                                                                                            i52 = i44;
                                                                                                            break;
                                                                                                        }
                                                                                                    case 236 :
                                                                                                        {
                                                                                                            i52 += ((int) (Test.instanceCount));
                                                                                                        }
                                                                                                    case 180 :
                                                                                                    case 330 :
                                                                                                        {
                                                                                                            i51 += ((int) (Test.byFld));
                                                                                                        }
                                                                                                    case 149 :
                                                                                                    case 367 :
                                                                                                        {
                                                                                                            Test.strFld = "two";
                                                                                                            break;
                                                                                                        }
                                                                                                    case 285 :
                                                                                                        {
                                                                                                            iArrFld = iArrFld;
                                                                                                        }
                                                                                                    case 352 :
                                                                                                        {
                                                                                                            i46 *= ((int) (39L));
                                                                                                            break;
                                                                                                        }
                                                                                                    case 164 :
                                                                                                        {
                                                                                                            Test.instanceCount ^= Test.instanceCount;
                                                                                                            break;
                                                                                                        }
                                                                                                    case 168 :
                                                                                                        {
                                                                                                            Test.instanceCount = ((long) (-10));
                                                                                                        }
                                                                                                    case 371 :
                                                                                                        {
                                                                                                            O20 = new Cls();
                                                                                                        }
                                                                                                    case 86 :
                                                                                                        {
                                                                                                            i56 -= i45;
                                                                                                            break;
                                                                                                        }
                                                                                                    case 142 :
                                                                                                        {
                                                                                                            Test.instanceCount = lFld;
                                                                                                            break;
                                                                                                        }
                                                                                                    case 383 :
                                                                                                        {
                                                                                                            if (b3) {
                                                                                                                break;
                                                                                                            }
                                                                                                            break;
                                                                                                        }
                                                                                                    case 319 :
                                                                                                        {
                                                                                                            Test.lArrFld[i48] = l2;
                                                                                                            break;
                                                                                                        }
                                                                                                    case 373 :
                                                                                                        {
                                                                                                            break;
                                                                                                        }
                                                                                                    case 130 :
                                                                                                        {
                                                                                                            break;
                                                                                                        }
                                                                                                    case 379 :
                                                                                                        {
                                                                                                            strArr5 = strArr5;
                                                                                                            break;
                                                                                                        }
                                                                                                    case 73 :
                                                                                                        {
                                                                                                            i52 = i46;
                                                                                                            break;
                                                                                                        }
                                                                                                    case 284 :
                                                                                                        {
                                                                                                            Test.cFld = ((char) (2551583529L));
                                                                                                        }
                                                                                                    case 295 :
                                                                                                        {
                                                                                                            i54 += i55;
                                                                                                            break;
                                                                                                        }
                                                                                                    case 290 :
                                                                                                        {
                                                                                                            i54 += ((int) (Cls.instanceCount));
                                                                                                            break;
                                                                                                        }
                                                                                                    case 350 :
                                                                                                        {
                                                                                                            b3 = b3;
                                                                                                        }
                                                                                                    case 321 :
                                                                                                        {
                                                                                                            break;
                                                                                                        }
                                                                                                    case 361 :
                                                                                                        {
                                                                                                            if (b3) {
                                                                                                                continue;
                                                                                                            }
                                                                                                            break;
                                                                                                        }
                                                                                                    case 147 :
                                                                                                        {
                                                                                                            break;
                                                                                                        }
                                                                                                    case 232 :
                                                                                                        {
                                                                                                            lFld = ((long) (d));
                                                                                                            break;
                                                                                                        }
                                                                                                    case 176 :
                                                                                                        {
                                                                                                            i49 -= ((int) (54735));
                                                                                                        }
                                                                                                    case 265 :
                                                                                                        {
                                                                                                            i58 += i48;
                                                                                                            break;
                                                                                                        }
                                                                                                    case 138 :
                                                                                                        {
                                                                                                            Cls.instanceCount = ((long) (f4));
                                                                                                        }
                                                                                                    case 151 :
                                                                                                        {
                                                                                                            break;
                                                                                                        }
                                                                                                    case 173 :
                                                                                                        {
                                                                                                            Test.iFld = ((int) (Test.byFld));
                                                                                                            break;
                                                                                                        }
                                                                                                    case 102 :
                                                                                                        {
                                                                                                            i49 <<= i44;
                                                                                                            break;
                                                                                                        }
                                                                                                    case 131 :
                                                                                                        {
                                                                                                            Test.iFld *= i48;
                                                                                                            break;
                                                                                                        }
                                                                                                    case 231 :
                                                                                                        {
                                                                                                            i58 ^= -18144;
                                                                                                            break;
                                                                                                        }
                                                                                                    case 384 :
                                                                                                        {
                                                                                                            break;
                                                                                                        }
                                                                                                    case 141 :
                                                                                                        {
                                                                                                            break;
                                                                                                        }
                                                                                                    case 353 :
                                                                                                        {
                                                                                                            iArrFld[i53 + 1] -= ((int) (f4));
                                                                                                        }
                                                                                                    case 98 :
                                                                                                        {
                                                                                                            i57 -= ((int) (d));
                                                                                                            break;
                                                                                                        }
                                                                                                    case 224 :
                                                                                                        {
                                                                                                            break;
                                                                                                        }
                                                                                                    case 218 :
                                                                                                        {
                                                                                                            Test.strFld = "one";
                                                                                                            break;
                                                                                                        }
                                                                                                    case 124 :
                                                                                                        {
                                                                                                            s3 -= ((short) (i53));
                                                                                                            break;
                                                                                                        }
                                                                                                    case 409 :
                                                                                                        {
                                                                                                            iArr3[i48 + 1] = i58;
                                                                                                            break;
                                                                                                        }
                                                                                                    case 145 :
                                                                                                        {
                                                                                                            i46 *= -213;
                                                                                                            break;
                                                                                                        }
                                                                                                    case 377 :
                                                                                                        {
                                                                                                            Cls.instanceCount *= ((long) (i58));
                                                                                                        }
                                                                                                    case 235 :
                                                                                                        {
                                                                                                            d = ((double) (i));
                                                                                                            break;
                                                                                                        }
                                                                                                    case 362 :
                                                                                                        {
                                                                                                            break;
                                                                                                        }
                                                                                                    case 338 :
                                                                                                        {
                                                                                                            d -= ((double) (Test.iFld));
                                                                                                        }
                                                                                                    case 222 :
                                                                                                        {
                                                                                                            i58 >>= -7524;
                                                                                                            break;
                                                                                                        }
                                                                                                    case 234 :
                                                                                                        {
                                                                                                            break;
                                                                                                        }
                                                                                                    case 364 :
                                                                                                        {
                                                                                                            i51 -= ((int) (Cls.instanceCount));
                                                                                                            break;
                                                                                                        }
                                                                                                    default :
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    case 94 :
                                                                                        {
                                                                                            f4 = ((float) (i49));
                                                                                            break;
                                                                                        }
                                                                                    case 95 :
                                                                                        {
                                                                                            Test.strFld = "hhhh";
                                                                                            Test.strFld += Test.strFld;
                                                                                        }
                                                                                    default :
                                                                                }
                                                                            }
                                                                            break;
                                                                        }
                                                                    default :
                                                                }
                                                                break;
                                                            }
                                                        default :
                                                            {
                                                                O21 = O19;
                                                            }
                                                    }
                                                    break;
                                                }
                                        }
                                    } catch (java.lang.ArrayIndexOutOfBoundsException exc14) {
                                    } finally {
                                        i56 >>= ((int) (Test.instanceCount));
                                    }
                                    break;
                                }
                            case 127 :
                            case 128 :
                                {
                                    iArr3 = iArrFld;
                                    break;
                                }
                            case 129 :
                                {
                                    f4 %= ((float) (Test.iFld | 1));
                                    break;
                                }
                            case 130 :
                                {
                                    break;
                                }
                            default :
                                {
                                    try {
                                        i52 = (-210) % i43;
                                        iFld3 = i44 / (-232);
                                        iArr3[i47 + 1] = i / i;
                                    } catch (java.lang.ArithmeticException a_e) {
                                    }
                                }
                        }
                    } while ((--i48) > 0 );
                } else if (b3) {
                } else if (b3) {
                    s3 -= ((short) (i55));
                } else {
                    Test.lArrFld[i47] -= ((long) (Test.byFld));
                }
            } while ((i47 += 2) < 106 );
        } while ((i1 += 2) < 98 );
        FuzzerUtils.out.println((((("i d i1 = " + i) + ",") + java.lang.Double.doubleToLongBits(d)) + ",") + i1);
        FuzzerUtils.out.println((((("f4 i43 i44 = " + java.lang.Float.floatToIntBits(f4)) + ",") + i43) + ",") + i44);
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
        FuzzerUtils.out.println((((("Test.lArrFld iArrFld dArrFld = " + FuzzerUtils.checkSum(Test.lArrFld)) + ",") + FuzzerUtils.checkSum(iArrFld)) + ",") + java.lang.Double.doubleToLongBits(FuzzerUtils.checkSum(dArrFld)));
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
            for (int i = 0; i < 10; i++) {
                try {
                    _instance.mainTest(strArr);
                } catch (java.lang.OutOfMemoryError ex) {
                    ex.printStackTrace(FuzzerUtils.err);
                    java.lang.System.exit(1);
                } catch (java.lang.Exception ex) {
                    FuzzerUtils.out.println(ex.getClass().getCanonicalName());
                }
            }
        } catch (java.lang.Exception ex) {
            FuzzerUtils.out.println(ex.getClass().getCanonicalName());
        }
    }
}
class AllFuzzerDefs_1780778723868_8894 {
    public static int ARRAY_SIZE = 80;
    public static int $func_escapeAnalysis_deoptimize_1_2202149116675175(int escapeAnalysis_deoptimize_1_a, int escapeAnalysis_deoptimize_1_b, int escapeAnalysis_deoptimize_1_c) {
        $cls_2202149115678857 $tmp1 = new $cls_2202149115678857();
        $tmp1.x = escapeAnalysis_deoptimize_1_a;
        $tmp1.y = escapeAnalysis_deoptimize_1_b;
        if (escapeAnalysis_deoptimize_1_c != 1) {
            AllFuzzerDefs_1780778723868_8894.gb__cls_2202149115678857_1780778725939_22021495741029310 = $tmp1;
            }
        return $tmp1.x + $tmp1.y;
        }
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

    public static void init(boolean[][] a, boolean seed) {
        for (int j = 0; j < a.length; j++) {
            init(a[j], seed);
        }
    }

    // Boolean -----------------------------------------------
    public static void init(Boolean[] a, Boolean seed) {
        for (int j = 0; j < a.length; j++) {
            a[j] = Boolean.valueOf((j % 2 == 0) ? seed : (j % 3 == 0));
        }
    }

    public static void init(Boolean[][] a, Boolean seed) {
        for (int j = 0; j < a.length; j++) {
            init(a[j], seed);
        }
    }

    // long --------------------------------------------------
    public static void init(long[] a, long seed) {
        for (int j = 0; j < a.length; j++) {
            a[j] = (j % 2 == 0) ? seed + j : seed - j;
        }
    }

    public static void init(long[][] a, long seed) {
        for (int j = 0; j < a.length; j++) {
            init(a[j], seed);
        }
    }

    // Long --------------------------------------------------
    public static void init(Long[] a, Long seed) {
        for (int j = 0; j < a.length; j++) {
            a[j] = Long.valueOf((long)((j % 2 == 0) ? seed + j : seed - j));
        }
    }

    public static void init(Long[][] a, Long seed) {
        for (int j = 0; j < a.length; j++) {
            init(a[j], seed);
        }
    }

    // int --------------------------------------------------
    public static void init(int[] a, int seed) {
        for (int j = 0; j < a.length; j++) {
            a[j] = (j % 2 == 0) ? seed + j : seed - j;
        }
    }

    public static void init(int[][] a, int seed) {
        for (int j = 0; j < a.length; j++) {
            init(a[j], seed);
        }
    }

    // Integer --------------------------------------------------
    public static void init(Integer[] a, Integer seed) {
        for (int j = 0; j < a.length; j++) {
            a[j] = Integer.valueOf((j % 2 == 0) ? seed + j : seed - j);
        }
    }

    public static void init(Integer[][] a, Integer seed) {
        for (int j = 0; j < a.length; j++) {
            init(a[j], seed);
        }
    }

    // short --------------------------------------------------
    public static void init(short[] a, short seed) {
        for (int j = 0; j < a.length; j++) {
            a[j] = (short) ((j % 2 == 0) ? seed + j : seed - j);
        }
    }

    public static void init(short[][] a, short seed) {
        for (int j = 0; j < a.length; j++) {
            init(a[j], seed);
        }
    }

    // Short --------------------------------------------------
    public static void init(Short[] a, Short seed) {
        for (int j = 0; j < a.length; j++) {
            a[j] = Short.valueOf((short) ((j % 2 == 0) ? seed + j : seed - j));
        }
    }

    public static void init(Short[][] a, Short seed) {
        for (int j = 0; j < a.length; j++) {
            init(a[j], seed);
        }
    }

    // char --------------------------------------------------
    public static void init(char[] a, char seed) {
        for (int j = 0; j < a.length; j++) {
            a[j] = (char) ((j % 2 == 0) ? seed + j : seed - j);
        }
    }

    public static void init(char[][] a, char seed) {
        for (int j = 0; j < a.length; j++) {
            init(a[j], seed);
        }
    }

    // Character --------------------------------------------------
    public static void init(Character[] a, Character seed) {
        for (int j = 0; j < a.length; j++) {
            a[j] = Character.valueOf((char) ((j % 2 == 0) ? seed + j : seed - j));
        }
    }

    public static void init(Character[][] a, Character seed) {
        for (int j = 0; j < a.length; j++) {
            init(a[j], seed);
        }
    }

    // byte --------------------------------------------------
    public static void init(byte[] a, byte seed) {
        for (int j = 0; j < a.length; j++) {
            a[j] = (byte) ((j % 2 == 0) ? seed + j : seed - j);
        }
    }

    public static void init(byte[][] a, byte seed) {
        for (int j = 0; j < a.length; j++) {
            init(a[j], seed);
        }
    }

    // Byte --------------------------------------------------
    public static void init(Byte[] a, Byte seed) {
        for (int j = 0; j < a.length; j++) {
            a[j] = Byte.valueOf((byte) ((j % 2 == 0) ? seed + j : seed - j));
        }
    }

    public static void init(Byte[][] a, Byte seed) {
        for (int j = 0; j < a.length; j++) {
            init(a[j], seed);
        }
    }

    // double --------------------------------------------------
    public static void init(double[] a, double seed) {
        for (int j = 0; j < a.length; j++) {
            a[j] = (j % 2 == 0) ? seed + j : seed - j;
        }
    }

    public static void init(double[][] a, double seed) {
        for (int j = 0; j < a.length; j++) {
            init(a[j], seed);
        }
    }

    // Double --------------------------------------------------
    public static void init(Double[] a, Double seed) {
        for (int j = 0; j < a.length; j++) {
            a[j] = Double.valueOf((j % 2 == 0) ? seed + j : seed - j);
        }
    }

    public static void init(Double[][] a, Double seed) {
        for (int j = 0; j < a.length; j++) {
            init(a[j], seed);
        }
    }

    // float --------------------------------------------------
    public static void init(float[] a, float seed) {
        for (int j = 0; j < a.length; j++) {
            a[j] = (j % 2 == 0) ? seed + j : seed - j;
        }
    }

    public static void init(float[][] a, float seed) {
        for (int j = 0; j < a.length; j++) {
            init(a[j], seed);
        }
    }

    // Float --------------------------------------------------
    public static void init(Float[] a, Float seed) {
        for (int j = 0; j < a.length; j++) {
            a[j] = Float.valueOf((j % 2 == 0) ? seed + j : seed - j);
        }
    }

    public static void init(Float[][] a, Float seed) {
        for (int j = 0; j < a.length; j++) {
            init(a[j], seed);
        }
    }

    // Object -------------------------------------------------
    public static void init(Object[][] a, Object seed) {
        for (int j = 0; j < a.length; j++) {
            init(a[j], seed);
        }
    }

    public static void init(Object[] a, Object seed) {
        for (int j = 0; j < a.length; j++)
            try {
                a[j] = seed.getClass().getDeclaredConstructor().newInstance();
            } catch (Exception ex) {
                a[j] = seed;
            }
    }

    // Calculate array checksum

    // boolean -----------------------------------------------
    public static long checkSum(boolean[] a) {
        long sum = 0;
        for (int j = 0; j < a.length; j++) {
            sum += (a[j] ? j + 1 : 0);
        }
        return sum;
    }

    public static long checkSum(boolean[][] a) {
        long sum = 0;
        for (int j = 0; j < a.length; j++) {
            sum += checkSum(a[j]);
        }
        return sum;
    }

    // long --------------------------------------------------
    public static long checkSum(long[] a) {
        long sum = 0;
        for (int j = 0; j < a.length; j++) {
            sum += (a[j] / (j + 1) + a[j] % (j + 1));
        }
        return sum;
    }

    public static long checkSum(long[][] a) {
        long sum = 0;
        for (int j = 0; j < a.length; j++) {
            sum += checkSum(a[j]);
        }
        return sum;
    }

    // int --------------------------------------------------
    public static long checkSum(int[] a) {
        long sum = 0;
        for (int j = 0; j < a.length; j++) {
            sum += (a[j] / (j + 1) + a[j] % (j + 1));
        }
        return sum;
    }

    public static long checkSum(int[][] a) {
        long sum = 0;
        for (int j = 0; j < a.length; j++) {
            sum += checkSum(a[j]);
        }
        return sum;
    }

    // short --------------------------------------------------
    public static long checkSum(short[] a) {
        long sum = 0;
        for (int j = 0; j < a.length; j++) {
            sum += (short) (a[j] / (j + 1) + a[j] % (j + 1));
        }
        return sum;
    }

    public static long checkSum(short[][] a) {
        long sum = 0;
        for (int j = 0; j < a.length; j++) {
            sum += checkSum(a[j]);
        }
        return sum;
    }

    // char --------------------------------------------------
    public static long checkSum(char[] a) {
        long sum = 0;
        for (int j = 0; j < a.length; j++) {
            sum += (char) (a[j] / (j + 1) + a[j] % (j + 1));
        }
        return sum;
    }

    public static long checkSum(char[][] a) {
        long sum = 0;
        for (int j = 0; j < a.length; j++) {
            sum += checkSum(a[j]);
        }
        return sum;
    }

    // byte --------------------------------------------------
    public static long checkSum(byte[] a) {
        long sum = 0;
        for (int j = 0; j < a.length; j++) {
            sum += (byte) (a[j] / (j + 1) + a[j] % (j + 1));
        }
        return sum;
    }

    public static long checkSum(byte[][] a) {
        long sum = 0;
        for (int j = 0; j < a.length; j++) {
            sum += checkSum(a[j]);
        }
        return sum;
    }

    // double --------------------------------------------------
    public static double checkSum(double[] a) {
        double sum = 0;
        for (int j = 0; j < a.length; j++) {
            sum += (a[j] / (j + 1) + a[j] % (j + 1));
        }
        return sum;
    }

    public static double checkSum(double[][] a) {
        double sum = 0;
        for (int j = 0; j < a.length; j++) {
            sum += checkSum(a[j]);
        }
        return sum;
    }

    // float --------------------------------------------------
    public static double checkSum(float[] a) {
        double sum = 0;
        for (int j = 0; j < a.length; j++) {
            sum += (a[j] / (j + 1) + a[j] % (j + 1));
        }
        return sum;
    }

    public static double checkSum(float[][] a) {
        double sum = 0;
        for (int j = 0; j < a.length; j++) {
            sum += checkSum(a[j]);
        }
        return sum;
    }

    // Object --------------------------------------------------
    public static long checkSum(Object[][] a) {
        long sum = 0;
        for (int j = 0; j < a.length; j++) {
            sum += checkSum(a[j]);
        }
        return sum;
    }

    public static long checkSum(Object[] a) {
        long sum = 0;
        for (int j = 0; j < a.length; j++) {
            sum += checkSum(a[j]) * Math.pow(2, j);
        }
        return sum;
    }

    public static long checkSum(Object a) {
        if (a == null)
            return 0L;
        return (long) a.getClass().getCanonicalName().length();
    }

    // Array creation ------------------------------------------
    public static byte[] byte1array(int sz, byte seed) {
        byte[] ret = new byte[sz];
        init(ret, seed);
        return ret;
    }

    public static byte[][] byte2array(int sz, byte seed) {
        byte[][] ret = new byte[sz][sz];
        init(ret, seed);
        return ret;
    }

    public static Byte[] Byte1array(int sz, Byte seed) {
        Byte[] ret = new Byte[sz];
        init(ret, seed);
        return ret;
    }

    public static Byte[][] Byte2array(int sz, Byte seed) {
        Byte[][] ret = new Byte[sz][sz];
        init(ret, seed);
        return ret;
    }

    public static short[] short1array(int sz, short seed) {
        short[] ret = new short[sz];
        init(ret, seed);
        return ret;
    }

    public static short[][] short2array(int sz, short seed) {
        short[][] ret = new short[sz][sz];
        init(ret, seed);
        return ret;
    }

    public static Short[] Short1array(int sz, Short seed) {
        Short[] ret = new Short[sz];
        init(ret, seed);
        return ret;
    }

    public static Short[][] Short2array(int sz, Short seed) {
        Short[][] ret = new Short[sz][sz];
        init(ret, seed);
        return ret;
    }

    public static int[] int1array(int sz, int seed) {
        int[] ret = new int[sz];
        init(ret, seed);
        return ret;
    }

    public static int[][] int2array(int sz, int seed) {
        int[][] ret = new int[sz][sz];
        init(ret, seed);
        return ret;
    }

    public static Integer[] Integer1array(int sz, Integer seed) {
        Integer[] ret = new Integer[sz];
        init(ret, seed);
        return ret;
    }

    public static Integer[][] Integer2array(int sz, Integer seed) {
        Integer[][] ret = new Integer[sz][sz];
        init(ret, seed);
        return ret;
    }

    public static long[] long1array(int sz, long seed) {
        long[] ret = new long[sz];
        init(ret, seed);
        return ret;
    }

    public static long[][] long2array(int sz, long seed) {
        long[][] ret = new long[sz][sz];
        init(ret, seed);
        return ret;
    }

    public static Long[] Long1array(int sz, Long seed) {
        Long[] ret = new Long[sz];
        init(ret, seed);
        return ret;
    }

    public static Long[][] Long2array(int sz, Long seed) {
        Long[][] ret = new Long[sz][sz];
        init(ret, seed);
        return ret;
    }

    public static float[] float1array(int sz, float seed) {
        float[] ret = new float[sz];
        init(ret, seed);
        return ret;
    }

    public static float[][] float2array(int sz, float seed) {
        float[][] ret = new float[sz][sz];
        init(ret, seed);
        return ret;
    }

    public static Float[] Float1array(int sz, Float seed) {
        Float[] ret = new Float[sz];
        init(ret, seed);
        return ret;
    }

    public static Float[][] Float2array(int sz, Float seed) {
        Float[][] ret = new Float[sz][sz];
        init(ret, seed);
        return ret;
    }

    public static double[] double1array(int sz, double seed) {
        double[] ret = new double[sz];
        init(ret, seed);
        return ret;
    }

    public static double[][] double2array(int sz, double seed) {
        double[][] ret = new double[sz][sz];
        init(ret, seed);
        return ret;
    }

    public static Double[] Double1array(int sz, Double seed) {
        Double[] ret = new Double[sz];
        init(ret, seed);
        return ret;
    }

    public static Double[][] Double2array(int sz, Double seed) {
        Double[][] ret = new Double[sz][sz];
        init(ret, seed);
        return ret;
    }

    public static char[] char1array(int sz, char seed) {
        char[] ret = new char[sz];
        init(ret, seed);
        return ret;
    }

    public static char[][] char2array(int sz, char seed) {
        char[][] ret = new char[sz][sz];
        init(ret, seed);
        return ret;
    }

    public static Character[] Character1array(int sz, Character seed) {
        Character[] ret = new Character[sz];
        init(ret, seed);
        return ret;
    }

    public static Character[][] Character2array(int sz, Character seed) {
        Character[][] ret = new Character[sz][sz];
        init(ret, seed);
        return ret;
    }

    public static Object[] Object1array(int sz, Object seed) {
        Object[] ret = new Object[sz];
        init(ret, seed);
        return ret;
    }

    public static Object[][] Object2array(int sz, Object seed) {
        Object[][] ret = new Object[sz][sz];
        init(ret, seed);
        return ret;
    }

    public static boolean[] boolean1array(int sz, boolean seed) {
        boolean[] ret = new boolean[sz];
        init(ret, seed);
        return ret;
    }

    public static boolean[][] boolean2array(int sz, boolean seed) {
        boolean[][] ret = new boolean[sz][sz];
        init(ret, seed);
        return ret;
    }

    public static Boolean[] Boolean1array(int sz, Boolean seed) {
        Boolean[] ret = new Boolean[sz];
        init(ret, seed);
        return ret;
    }

    public static Boolean[][] Boolean2array(int sz, Boolean seed) {
        Boolean[][] ret = new Boolean[sz][sz];
        init(ret, seed);
        return ret;
    }

    public static String[] String1array(int sz, String seed) {
        String[] ret = new String[sz];
        init(ret, seed);
        return ret;
    }

    public static String[][] String2array(int sz, String seed) {
        String[][] ret = new String[sz][sz];
        init(ret, seed);
        return ret;
    }

}

