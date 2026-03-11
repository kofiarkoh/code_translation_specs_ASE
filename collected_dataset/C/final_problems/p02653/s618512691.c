#include <stdio.h>
#include <stdlib.h>

#define Mod 1000000007

typedef struct List {
	struct List *prev, *next;
	long long v;
} list;

int main()
{
	int N, A, B;
	scanf("%d %d %d", &N, &A, &B);
	if (A > B) {
		B += A;
		A = B - A;
		B -= A;
	}
	
	int i, j, k;
	list **head = (list**)malloc(sizeof(list*) * B), **tail = (list**)malloc(sizeof(list*) * B), *d = (list*)malloc(sizeof(list) * B * (A + 1)), *p;
	long long ans = 0, dp[2][5001], tmp[2], pow;
	if (A == 1 || B == 1) {
		for (i = 1, ans = 1; i <= N; i++) ans = ans * 2 % Mod;
	} else if (A == B) {
		for (j = A - 2, dp[A%2][A-1] = 2; j >= 1; j--) dp[A%2][j] = dp[A%2][j+1] * 2 % Mod;
		for (i = A + 1, ans = 2; i <= N; i++) {
			ans = (ans * 2 + dp[1-i%2][A-1]) % Mod;
			for (j = A - 2, dp[i%2][1] = dp[1-i%2][A-1]; j >= 1; j--) {
				dp[i%2][j+1] = dp[1-i%2][j];
				dp[i%2][1] += dp[1-i%2][j];
			}
			dp[i%2][1] %= Mod;
		}
	} else {
		for (j = 0; j <= B - 1; j++) {
			head[j] = &(d[j*(A+1)]);
			tail[j] = &(d[j*(A+1)+A]);
			d[j*(A+1)].v = 0;
			d[j*(A+1)].prev = &(d[j*(A+1)+A]);
			d[j*(A+1)+A].next = &(d[j*(A+1)]);
			for (k = 1; k <= A; k++) {
				d[j*(A+1)+k].v = 0;
				d[j*(A+1)+k].prev = &(d[j*(A+1)+k-1]);
				d[j*(A+1)+k-1].next = &(d[j*(A+1)+k]);
			}
		}
		d[1].v = 1;
		d[A+1].v = 1;
		for (i = 2, pow = 2; i <= N; i++, pow = pow * 2 % Mod) {
			for (j = 0, tmp[0] = 0; j <= B - 1; j++) tmp[0] += head[j]->v + tail[j]->v;
			tmp[0] = (pow - ans - tmp[0] + tail[B-A]->v) % Mod;
			if (tmp[0] < 0) tmp[0] += Mod;
			ans = (ans * 2 + head[B-1]->v + tail[B-A]->prev->v + tail[B-A-1]->v * 2) % Mod;
			
			for (j = B - 1, tmp[1] = head[B-A]->v; j > B - A; j--) {
				tmp[1] += head[j]->v;
				head[j]->v = head[j-1]->v;
				if (j >= A + 1) head[j]->v = (head[j]->v + tail[j-A-1]->v) % Mod;
			}
			head[B-A]->v = tmp[1] % Mod;
			tail[B-A]->v = head[B-A-1]->v;
			if (B - A >= A + 1) tail[B-A]->v += tail[B-A*2-1]->v;
			tail[B-A]->v %= Mod;
			head[B-A] = tail[B-A];
			tail[B-A] = tail[B-A]->prev;
			for (j = B - A - 1; j >= A + 1; j--) {
				tail[j]->prev->v = (tail[j]->prev->v + tail[j-1]->v) % Mod;
				tail[j]->v = (head[j-1]->v + tail[j-A-1]->v) % Mod;
				head[j] = tail[j];
				tail[j] = tail[j]->prev;
			}
			for (; j >= 1; j--) {
				tail[j]->prev->v = (tail[j]->prev->v + tail[j-1]->v) % Mod;
				tail[j]->v = head[j-1]->v;
				head[j] = tail[j];
				tail[j] = tail[j]->prev;
			}
			head[1]->v = tmp[0];
			head[0] = tail[0];
			tail[0] = tail[0]->prev;
			head[0]->v = 0;
		}
	}
	
	printf("%lld\n", ans);
	fflush(stdout);
	return 0;
}