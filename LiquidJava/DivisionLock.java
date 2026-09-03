/*
 * LiquidJava Specification for Locked Integer Division
 */

public class DivisionLock {
    public static boolean isNonZero(int d) {
        return d != 0;
    }

    /**
     * Locked Division Theorem
     * @param n The dividend
     * @param d The divisor - Refined to be non-zero
     * @return The quotient q such that n = q*d + r and |r| < |d|
     * @requires d != 0
     * @ensures \exists int r. (n == res * d + r) && (Math.abs(r) < Math.abs(d))
     */
    public static int lockedDivide(int n, int d)
        /*@ requires d != 0 @*/
        /*@ ensures \exists int r. (n == res * d + r) && (Math.abs(r) < Math.abs(d)) @*/
    {
        return n / d;
    }

    public static void main(String[] args) {
        int a = 10;
        int b = 2;
        System.out.println(lockedDivide(a, b));
    }
}
