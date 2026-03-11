// AOJ 2884 Tanka Number
// 2018.3.23 bal4u

#define _CRT_SECURE_NO_WARNINGS
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// n番目（最初は１番目の10）の短歌数は何桁で表現されるか
int keta(long long n) {
	int i; for (i = 2; ; i++) if (81LL * ((1LL << i) - i - 1) > n) return i;
}

// n桁の数字が最大何番目までの短歌数を表現するか
long long total_combi(int keta) { return 81LL * ((1LL << keta) - keta - 1); }


char ans_[60], *ans;
long long N;

// d1:1つ目の数字, d2:2つ目の数字, N: 残りの組み合わせ, k: 残りの桁数
void calc(int d1, int d2, long long N, int k)
{
	int i;
	long long s, s1, s2;

	if (k <= 0) return;
	if (d2 >= 0) {
		s = (1LL << (k - 1));
		*ans = (((N < s) ^ (d1 > d2)) ? d1 : d2);
		*ans++ += '0';
		calc(d1, d2, N % s, k - 1);
	}
	else {
		s = 0;
		s1 = 9LL * ((1LL << (k - 1)) - 1), s2 = 1LL << (k - 1);
		for (i = 0; i <= 9; i++) {
			if (i == d1) {
				if (k == 1) continue;
				if (s + s1 > N) {
					*ans++ = d1 + '0';
					calc(d1, d2, N - s, k - 1);
					break;
				}
				s += s1;
			}
			else {
				if (s + s2 > N) {
					*ans++ = i + '0';
					calc(d1, i, N - s, k - 1);
					break;
				}
				s += s2;
			}
		}
	}
}

int main()
{
	int k;

	while (scanf("%lld", &N) && N) {
		ans = ans_;
		k = keta(N);
		N -= total_combi(k - 1);
		if (N == 0) {
			k--;
			memset(ans, '9', k - 1);
			ans[k - 1] = '8', ans[k] = 0;
		}
		else {
			long long t = (9LL * ((1LL << (k - 1)) - 1));
			int d = 1 + (int)(--N / t);
			*ans++ = d + '0';
			calc(d, -1, N % t, k - 1);
			*ans = 0;
		}
		puts(ans_);
	}
	return 0;
}
