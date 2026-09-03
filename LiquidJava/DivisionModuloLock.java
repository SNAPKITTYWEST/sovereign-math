/*
 * LiquidJava Specification: Locked Division & Modulo Pair
 */

public class DivisionModuloLock {
    public static /*@ pure @*/ boolean nonZero(int d) { return d != 0; }
    public static /*@ pure @*/ int abs(int x) { return x < 0 ? -x : x; }

    /**
     * @requires nonZero(d)
     * @ensures \exists int r. (n == res * d + r) && (r >= -abs(d) + 1) && (r <= abs(d) - 1)
     */
    public static int lockedDivide(int n, int d)
        /*@ requires nonZero(d) @*/
    {
        return n / d;
    }

    /**
     * @requires nonZero(d)
     * @ensures n == lockedDivide(n, d) * d + res
     * @ensures res > -abs(d) && res < abs(d)
     * @ensures (n >= 0) ==> (res >= 0)
     * @ensures (n <= 0) ==> (res <= 0)
     */
    public static int lockedModuloJava(int n, int d)
        /*@ requires nonZero(d) @*/
        /*@ ensures n == lockedDivide(n, d) * d + res @*/
        /*@ ensures res > -abs(d) && res < abs(d) @*/
        /*@ ensures (n >= 0) ==> (res >= 0) @*/
        /*@ ensures (n <= 0) ==> (res <= 0) @*/
    {
        return n % d;
    }

    /**
     * @requires nonZero(d)
     * @ensures 0 <= res && res < abs(d)
     * @ensures \exists int q. (n == q * d + res)
     */
    public static int lockedModuloEuclidean(int n, int d)
        /*@ requires nonZero(d) @*/
        /*@ ensures 0 <= res && res < abs(d) @*/
        /*@ ensures \exists int q. (n == q * d + res) @*/
    {
        int r = n % d;
        if (r < 0) {
            return r + abs(d);
        }
        return r;
    }

    public static void main(String[] args) {
        int q1 = lockedDivide(10, 3);
        int r1 = lockedModuloEuclidean(10, 3);
        int q2j = lockedDivide(-10, 3);
        int r2j = lockedModuloJava(-10, 3);
        int r2e = lockedModuloEuclidean(-10, 3);
    }
}
