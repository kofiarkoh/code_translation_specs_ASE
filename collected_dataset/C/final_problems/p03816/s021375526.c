#include <stdio.h>
#include <string.h>
#include <stdbool.h>

#define S_MAX 200000
#define N_MAX 100000
#define M_MAX 100000
#define INF 10000000
#define MOD 1000000007
#define SMAP(a, b) ((a)!=(b))&&((a)^=((b)^=((a)^= (b))))

typedef unsigned long long ull;
typedef   signed long long dll;

ull n, m;
int h, w;
int i, j, k;

int a[N_MAX];
int b[M_MAX];

typedef struct {
	int in;
	int out;
} twoway;

void solve(){
	ull result = 0;
	ull prev;

	for (i = 0; i < n; i++) {
		b[a[i] - 1]++;
	}

	for (i = 0; i < M_MAX; i++) {
		if (b[i]) result += (ull)(b[i] - 1);
	}

	printf("%llu\n", (ull)n - ((result >> 1) + (result & 1)) * 2);

	return;
}

int main (void) {
	char *t;
	scanf("%d", &n);

	for (i = 0; i < n; i++) scanf("%d", &a[i]);

	solve();
	return 0;
}


