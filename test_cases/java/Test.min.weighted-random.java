
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
        FuzzerUtils.init(iArr, -227);
        FuzzerUtils.init(cArr, ((char) (29867)));
        FuzzerUtils.init(bArr, true);
        FuzzerUtils.init(lArr, 4084697890266682836L);
        FuzzerUtils.init(OArr, new Cls());
        i9 = 1;
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
        FuzzerUtils.init(OArr1, new Cls());
        FuzzerUtils.init(strArr3, "three");
        b = false;
        i5 *= ((int) (d1++));
        for (float f : fArr) {
            synchronized(O) {
                Test.vMeth1(i6, i5);
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
        char[] lv_char_arr_1780778725955_22021495902816830 = new char[AllFuzzerDefs_1780778723868_8894.ARRAY_SIZE];
        float[] lv_float_arr_1780778725966_22021496013278310 = new float[AllFuzzerDefs_1780778723868_8894.ARRAY_SIZE];
        for (int TmpVar_2202149660561186_1780778726026_nJ = 0; TmpVar_2202149660561186_1780778726026_nJ < lv_short_arr_1780778725942_22021495766075661.length; TmpVar_2202149660561186_1780778726026_nJ++) {
            lv_short_arr_1780778725942_22021495766075661[TmpVar_2202149660561186_1780778726026_nJ] = ((short) ((((short) (TmpVar_2202149660561186_1780778726026_nJ)) * 10) + 34));
        }
        for (int TmpVar_2202149665008467_1780778726030_H3 = 0; TmpVar_2202149665008467_1780778726030_H3 < lv_short_arr_1780778725940_22021495747771610.length; TmpVar_2202149665008467_1780778726030_H3++) {
        }
        for (int TmpVar_2202149665162821_1780778726030_aF = 0; TmpVar_2202149665162821_1780778726030_aF < AllFuzzerDefs_1780778723868_8894.ARRAY_SIZE; TmpVar_2202149665162821_1780778726030_aF++) {
        }
        for (int TmpVar_2202149665271027_1780778726030_A7 = 0; TmpVar_2202149665271027_1780778726030_A7 < lv_float_arr_1780778725966_22021496013278310.length; TmpVar_2202149665271027_1780778726030_A7++) {
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
        FuzzerUtils.init(bArr1, false);
        for (lv_int_1780778725930_22021495648453660 = 0; lv_int_1780778725930_22021495648453660 < 10000; lv_int_1780778725930_22021495648453660++) {
        Arrays.fill(lv_float_arr_1780778725966_22021496013278310, (((904.20575f) + (554.3475f))));
        if ((((int) (lv_short_arr_1780778725942_22021495766075661[0])) % 3) < 1) {
        } else {
            lv_ShortVector_1780778725955_22021495895258201 = ShortVector.fromArray(ShortVector.SPECIES_PREFERRED, lv_short_arr_1780778725940_22021495747771610, 20);
        }
        lv_ShortVector_1780778725955_22021495895857152 = ShortVector.fromArray(ShortVector.SPECIES_PREFERRED, lv_short_arr_1780778725940_22021495747771610, 20);
        lv_ShortVector_1780778725955_22021495895857152 = lv_ShortVector_1780778725955_22021495895258201.and(lv_ShortVector_1780778725955_22021495895857152).or(lv_ShortVector_1780778725955_22021495895258201.not());
        lv_ShortVector_1780778725955_22021495895857152.intoArray(lv_short_arr_1780778725942_22021495766075661, 20);
        }
        for (int TmpVar_2202149665025586_1780778726030_u3 = 0; TmpVar_2202149665025586_1780778726030_u3 < lv_short_arr_1780778725940_22021495747771610.length; TmpVar_2202149665025586_1780778726030_u3 = (2 + (TmpVar_2202149665025586_1780778726030_u3 * 3)) / 2) {
        }
        System.out.println(("String lv_String_1780778725954_22021495893671560:: `" + lv_String_1780778725954_22021495893671560) + "`");
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
        Cls O17 = new Cls();
        Cls O18 = new Cls();
        Cls O19 = new Cls();
        Cls O21 = new Cls();
        Cls O22 = new Cls();
        Cls O23 = new Cls();
        FuzzerUtils.init(bArr2, false);
        FuzzerUtils.init(byArr1, ((byte) (-85)));
        FuzzerUtils.init(strArr5, "one");
        FuzzerUtils.init(iArr3, -9923);
        i += ((int) (d++));
        i1 = 1;
        do {
            Test.strFld = (Test.strFld + Test.strMeth()) + "one";
            iArrFld[i1 + 1] -= ((int) (202L));
            for (i45 = ((int) (105)); i45 > 4; i45--) {
                i >>>= ((int) (Cls.instanceCount));
                f4 *= ((float) (i43));
                Test.lArrFld[i45] += ((long) (i));
                d += ((double) (i44));
                Test.iFld <<= i1;
            }
            do {
            } while ((i47 += 2) < 106 );
        } while ((i1 += 2) < 98 );
        FuzzerUtils.out.println((((("f4 i43 i44 = " + java.lang.Float.floatToIntBits(f4)) + ",") + i43) + ",") + i44);
        FuzzerUtils.out.println((((("s2 i47 i48 = " + s2) + ",") + i47) + ",") + i48);
        FuzzerUtils.out.println((((("l2 i49 i50 = " + l2) + ",") + i49) + ",") + i50);
        FuzzerUtils.out.println((((("i51 i52 i53 = " + i51) + ",") + i52) + ",") + i53);
        FuzzerUtils.out.println((((("bArr2 byArr1 strArr5 = " + FuzzerUtils.checkSum(bArr2)) + ",") + FuzzerUtils.checkSum(byArr1)) + ",") + FuzzerUtils.checkSum(strArr5));
        FuzzerUtils.out.println((((("O21 O22 O23 = " + FuzzerUtils.checkSum(O21)) + ",") + FuzzerUtils.checkSum(O22)) + ",") + FuzzerUtils.checkSum(O23));
        FuzzerUtils.out.println("Cls = " + Cls.instanceCount);
        FuzzerUtils.out.println("Cls = " + Cls.instanceCount);
        FuzzerUtils.out.println("vMeth_check_sum: " + Test.vMeth_check_sum);
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
                } catch (java.lang.Exception ex) {
                }
            }
        } catch (java.lang.Exception ex) {
        }
    }
}
class AllFuzzerDefs_1780778723868_8894 {
    public static int ARRAY_SIZE = 80;
    public static int $func_escapeAnalysis_deoptimize_1_2202149116675175(int escapeAnalysis_deoptimize_1_a, int escapeAnalysis_deoptimize_1_b, int escapeAnalysis_deoptimize_1_c) {
        $cls_2202149115678857 $tmp1 = new $cls_2202149115678857();
        $tmp1.x = escapeAnalysis_deoptimize_1_a;
        $tmp1.y = escapeAnalysis_deoptimize_1_b;
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
    }

    public static void init(boolean[][] a, boolean seed) {
    }

    // Boolean -----------------------------------------------
    public static void init(Boolean[] a, Boolean seed) {
        for (int j = 0; j < a.length; j++) {
        }
    }

    public static void init(Boolean[][] a, Boolean seed) {
        for (int j = 0; j < a.length; j++) {
        }
    }

    // long --------------------------------------------------
    public static void init(long[] a, long seed) {
    }

    public static void init(long[][] a, long seed) {
        for (int j = 0; j < a.length; j++) {
        }
    }

    // Long --------------------------------------------------
    public static void init(Long[] a, Long seed) {
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
    }

    // Integer --------------------------------------------------
    public static void init(Integer[] a, Integer seed) {
    }

    public static void init(Integer[][] a, Integer seed) {
        for (int j = 0; j < a.length; j++) {
            init(a[j], seed);
        }
    }

    // short --------------------------------------------------
    public static void init(short[] a, short seed) {
    }

    public static void init(short[][] a, short seed) {
    }

    // Short --------------------------------------------------
    public static void init(Short[] a, Short seed) {
        for (int j = 0; j < a.length; j++) {
        }
    }

    public static void init(Short[][] a, Short seed) {
        for (int j = 0; j < a.length; j++) {
        }
    }

    // char --------------------------------------------------
    public static void init(char[] a, char seed) {
        for (int j = 0; j < a.length; j++) {
            a[j] = (char) ((j % 2 == 0) ? seed + j : seed - j);
        }
    }

    public static void init(char[][] a, char seed) {
    }

    // Character --------------------------------------------------
    public static void init(Character[] a, Character seed) {
    }

    public static void init(Character[][] a, Character seed) {
    }

    // byte --------------------------------------------------
    public static void init(byte[] a, byte seed) {
    }

    public static void init(byte[][] a, byte seed) {
        for (int j = 0; j < a.length; j++) {
        }
    }

    // Byte --------------------------------------------------
    public static void init(Byte[] a, Byte seed) {
        for (int j = 0; j < a.length; j++) {
        }
    }

    public static void init(Byte[][] a, Byte seed) {
    }

    // double --------------------------------------------------
    public static void init(double[] a, double seed) {
    }

    public static void init(double[][] a, double seed) {
        for (int j = 0; j < a.length; j++) {
        }
    }

    // Double --------------------------------------------------
    public static void init(Double[] a, Double seed) {
    }

    public static void init(Double[][] a, Double seed) {
    }

    // float --------------------------------------------------
    public static void init(float[] a, float seed) {
    }

    public static void init(float[][] a, float seed) {
        for (int j = 0; j < a.length; j++) {
            init(a[j], seed);
        }
    }

    // Float --------------------------------------------------
    public static void init(Float[] a, Float seed) {
    }

    public static void init(Float[][] a, Float seed) {
    }

    // Object -------------------------------------------------
    public static void init(Object[][] a, Object seed) {
    }

    public static void init(Object[] a, Object seed) {
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
        return sum;
    }

    public static long checkSum(short[][] a) {
        long sum = 0;
        for (int j = 0; j < a.length; j++) {
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
        return sum;
    }

    // byte --------------------------------------------------
    public static long checkSum(byte[] a) {
        long sum = 0;
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
        return sum;
    }

    public static double checkSum(double[][] a) {
        double sum = 0;
        for (int j = 0; j < a.length; j++) {
        }
        return sum;
    }

    // float --------------------------------------------------
    public static double checkSum(float[] a) {
        double sum = 0;
        return sum;
    }

    public static double checkSum(float[][] a) {
        double sum = 0;
        for (int j = 0; j < a.length; j++) {
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
        return ret;
    }

    public static byte[][] byte2array(int sz, byte seed) {
        byte[][] ret = new byte[sz][sz];
        init(ret, seed);
        return ret;
    }

    public static Byte[] Byte1array(int sz, Byte seed) {
        Byte[] ret = new Byte[sz];
        return ret;
    }

    public static Byte[][] Byte2array(int sz, Byte seed) {
        Byte[][] ret = new Byte[sz][sz];
        return ret;
    }

    public static short[] short1array(int sz, short seed) {
        short[] ret = new short[sz];
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
        return ret;
    }

    public static int[][] int2array(int sz, int seed) {
        int[][] ret = new int[sz][sz];
        return ret;
    }

    public static Integer[] Integer1array(int sz, Integer seed) {
        Integer[] ret = new Integer[sz];
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
        return ret;
    }

    public static double[][] double2array(int sz, double seed) {
        double[][] ret = new double[sz][sz];
        init(ret, seed);
        return ret;
    }

    public static Double[] Double1array(int sz, Double seed) {
        Double[] ret = new Double[sz];
        return ret;
    }

    public static Double[][] Double2array(int sz, Double seed) {
        Double[][] ret = new Double[sz][sz];
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
        return ret;
    }

    public static String[][] String2array(int sz, String seed) {
        String[][] ret = new String[sz][sz];
        return ret;
    }

}

