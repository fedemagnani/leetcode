//! You are given a string s consisting of digits. Perform the following operation repeatedly until the string has exactly two digits:
//!
//! For each pair of consecutive digits in s, starting from the first digit, calculate a new digit as the sum of the two digits modulo 10.
//! Replace s with the sequence of newly calculated digits, maintaining the order in which they are computed.
//! Return true if the final two digits in s are the same; otherwise, return false.
//!
//!  
//!
//! Example 1:
//!
//! Input: s = "3902"
//!
//! Output: true
//!
//! Explanation:
//!
//! Initially, s = "3902"
//! First operation:
//! (s[0] + s[1]) % 10 = (3 + 9) % 10 = 2
//! (s[1] + s[2]) % 10 = (9 + 0) % 10 = 9
//! (s[2] + s[3]) % 10 = (0 + 2) % 10 = 2
//! s becomes "292"
//! Second operation:
//! (s[0] + s[1]) % 10 = (2 + 9) % 10 = 1
//! (s[1] + s[2]) % 10 = (9 + 2) % 10 = 1
//! s becomes "11"
//! Since the digits in "11" are the same, the output is true.
//! Example 2:
//!
//! Input: s = "34789"
//!
//! Output: false
//!
//! Explanation:
//!
//! Initially, s = "34789".
//! After the first operation, s = "7157".
//! After the second operation, s = "862".
//! After the third operation, s = "48".
//! Since '4' != '8', the output is false.
//!  
//!
//! Constraints:
//!
//! 3 <= s.length <= 100
//! s consists of only digits.

// The trivial solution has n^2 complexity, However we can do better: if you expand the examples in tree-form, you will see that the last row of the tree (containing the two values to be compared), can be recovered from the Pascal triangle.

// Indeed, if the input string has n digits, you can see that the first comparison digit is the inner product between the (n-1)th pascal row and the FIRST (n-1) digits, while the second comparison digit is the inner product between the (n-1)th pascal row and the LAST (n-1) digits\

// Recall that given row n of the pascal triangle, its cells are mapping k=0,1,2,..,n into binomial coefficients C(n,k).
// Recall also that the first element of the row is always 1 as C(n,0)=1 for any n
// Recall that you can Recover C(n,k+1) = C(n,k) * (n-k)/(k+1), so we can easily build the row of the pascal triangle without building previous ones

// However, the binomial coefficient as it is, would not work: we need it computed modulo 10

use super::*;

impl Solution {
    /// Compute nCk % p using Lucas theorem
    fn lucas_mod(mut n: u32, mut k: u32, p: u32) -> u32 {
        let mut res = 1;
        while n > 0 || k > 0 {
            let ni = n % p;
            let ki = k % p;
            if ki > ni {
                return 0;
            }
            // Compute ni choose ki mod p using factorial
            let mut c = 1;
            for i in 0..ki {
                c = c * (ni - i) % p;
                c = c * Self::modinv(i + 1, p) % p;
            }
            res = res * c % p;
            n /= p;
            k /= p;
        }
        res
    }

    /// Compute modular inverse using Fermat's little theorem (works because p is prime)
    fn modinv(x: u32, p: u32) -> u32 {
        Self::modpow(x, p - 2, p)
    }

    /// Fast modular exponentiation
    fn modpow(mut base: u32, mut exp: u32, m: u32) -> u32 {
        let mut res = 1;
        base %= m;
        while exp > 0 {
            if exp % 2 == 1 {
                res = res * base % m;
            }
            base = base * base % m;
            exp /= 2;
        }
        res
    }

    /// Combine two moduli using CRT for modulus 10
    fn crt2_mod10(r2: u32, r5: u32) -> u32 {
        // Solve x ≡ r2 (mod 2), x ≡ r5 (mod 5)
        // x = 5*r2 + 2*(r5*inv2_mod5) % 10
        let inv2_mod5 = 3; // 2*3 ≡ 1 mod 5
        (r5 * 2 * inv2_mod5 + r2 * 5) % 10
    }

    /// Compute C(n,k) % 10 using Lucas theorem + CRT (chinese remainder theorem)
    fn c_mod10(n: u32, k: u32) -> u32 {
        let r2 = Self::lucas_mod(n, k, 2);
        let r5 = Self::lucas_mod(n, k, 5);
        Self::crt2_mod10(r2, r5)
    }

    pub fn has_same_digits(s: String) -> bool {
        let s = s.into_bytes();
        let mut f0 = 0;
        let mut f1 = 0;
        let n = s.len();
        let p_row = (n - 2) as u8;

        for k in 0..n - 1 {
            let c = Self::c_mod10(p_row as u32, k as u32);
            f0 += c * (s[k] - b'0') as u32;
            f1 += c * (s[k + 1] - b'0') as u32;
            f0 %= 10;
            f1 %= 10;
        }

        f0 == f1
    }
    pub fn has_same_digits_slow(s: String) -> bool {
        let mut s = s.into_bytes();

        while s.len() > 2 {
            for i in 1..s.len() {
                let prev = i - 1;
                s[prev] += s[i];
                s[prev] %= 10;
            }
            s.pop();
        }
        s[0] == s[1]
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test3461() {
        let s = "3902";
        let out = Solution::has_same_digits(s.to_string());
        assert!(out);

        let s = "34789";
        let out = Solution::has_same_digits(s.to_string());
        assert!(!out);

        let s = "321881";
        let out = Solution::has_same_digits(s.to_string());
        assert!(out);
    }
}
