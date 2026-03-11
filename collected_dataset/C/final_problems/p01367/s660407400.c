// AOJ 2250 Operator
// 2018.2.1 bal4u

#include <stdio.h>
#include <string.h>

int N, T;
int M[1001], L[1001], K[1001];
int avai[1001];
char done[1001];

//#define getchar_unlocked()  getchar()
int in()
{
	int n = 0;
	int c = getchar_unlocked();
	do n = (n<<3)+(n<<1) + (c & 0xf), c = getchar_unlocked();
	while (c >= '0');
	return n;
}

int check(int p)
{
	int i, t, left;

	if (p >= N) return 1;

	memset(avai, 0, sizeof(avai));
	memset(done, 0, sizeof(done));

    for (i = 0; i < p; i++) avai[M[i]]++;
    left = N-p;

	for (t = 1; ; t++) {
		if (t > T) return 0;
		if (left <= 0) break;
		if (!avai[t]) continue;
		for (i = p; i < N; i++) {
			if (done[i]) continue;
			if (t % (L[i]+K[i]) <= L[i]) {
				if (t + M[i] > T) return 0;
				left--, done[i] = 1;
				avai[t]--, avai[t+M[i]]++;
				if (!avai[t]) break;
			}
		}
		avai[t+1] += avai[t];
	}
    return 1;
}

int main()
{
	int i;

	while (N = in()) {
		T = in();
		for (i = 0; i < N; i++) M[i] = in(), L[i] = in(), K[i] = in();
		for (i = 1; !check(i); i++);
		printf("%d\n", i);
	}
	return 0;
}
