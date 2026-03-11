#include <stdio.h>
#include <stdlib.h>
#include <sys/time.h>

#define N	250000
#define MD	0x7fffffff

int rand_(int n) {
	return (rand() * 76543LL + rand()) % n;
}

int inv(int a) {
	return a == 1 ? 1 : (long long) inv(a - MD % a) * (MD / a + 1) % MD; 
}

int X, Y, IX, IY;

void srand_() {
	struct timeval tv;

	gettimeofday(&tv, NULL);
	srand(tv.tv_sec ^ tv.tv_usec);
	X = rand_(MD - N * 2) + N * 2, IX = inv(X);
	Y = rand_(MD - N * 2) + N * 2, IY = inv(Y);
}

long long xy[N + 1];

int compare(const void *a, const void *b) {
	int i = *(int *) a;
	int j = *(int *) b;

	return xy[i] == xy[j] ? 0 : (xy[i] < xy[j] ? -1 : 1);
}

int find(long long *aa, int n, long long a) {
	int lower = -1, upper = n;

	while (upper - lower > 1) {
		int i = (lower + upper) / 2;

		if (aa[i] <= a)
			lower = i;
		else
			upper = i;
	}
	return lower == -1 || aa[lower] != a ? -1 : lower;
}

int main() {
	static char cc[N + 1];
	static long long xy_[N + 1];
	static int ii[N + 1], hh[N + 1], kk[N + 1];
	int n, n_, i;
	long long x, y, px, py, ans;

	srand_();
	scanf("%d%s", &n, cc);
	x = y = 0, px = py = 1;
	for (i = 0; i < n; i++) {
		if (cc[i] == '+')
			x = (x + px) % MD, y = (y + py) % MD;
		else if (cc[i] == '-')
			x = (x - px) % MD, y = (y - py) % MD;
		else if (cc[i] == '>')
			px = px * X % MD, py = py * Y % MD;
		else
			px = px * IX % MD, py = py * IY % MD;
		if (x < 0)
			x += MD;
		if (y < 0)
			y += MD;
		xy[i + 1] = x * MD + y;
	}
	for (i = 0; i <= n; i++)
		ii[i] = i;
	qsort(ii, n + 1, sizeof *ii, compare);
	n_ = 0;
	for (i = 0; i <= n; i++) {
		int i_ = ii[i];

		if (n_ == 0 || xy_[n_ - 1] != xy[i_])
			xy_[n_++] = xy[i_];
		hh[i_] = n_ - 1;
	}
	ans = 0;
	for (i = n; i >= 0; i--) {
		int h;

		if (cc[i] == '>')
			px = px * IX % MD, py = py * IY % MD;
		else if (cc[i] == '<')
			px = px * X % MD, py = py * Y % MD;
		x = (xy[n] / MD * px + xy[i] / MD) % MD;
		y = (xy[n] % MD * py + xy[i] % MD) % MD;
		if (x < 0)
			x += MD;
		if (y < 0)
			y += MD;
		h = find(xy_, n_, x * MD + y);
		if (h != -1)
			ans += kk[h];
		kk[hh[i]]++;
	}
	printf("%lld\n", ans);
	return 0;
}