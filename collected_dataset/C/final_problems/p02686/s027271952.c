#include <stdio.h>
#include <stdlib.h>
#include <string.h>

void neko(int* left, int* delta, const char* data) {
	size_t s = strlen(data);
	size_t i;
	int count = 0;
	int foo = 0;
	for (i = 0; i < s; i++) {
		if (data[i] == '(') count++;
		else if (data[i] == ')') count--;
		if (count < foo) foo = count;
	}
	*left = -foo;
	*delta = count;
}

char S[1234567];

struct nya {
	int left, delta;
};

int by_left(const void* x, const void* y) {
	struct nya a = *(const struct nya*)x, b = *(const struct nya*)y;
	return a.left < b.left ? -1 : a.left > b.left;
}

int by_left2(const void* x, const void* y) {
	return by_left(y, x);
}

int N;

int pc, mc;
struct nya p[1234567], m[1234567];

int main(void) {
	int i;
	int ayaneru = 0;
	if (scanf("%d", &N) != 1) return 1;
	for (i = 0; i < N; i++) {
		struct nya hoge;
		if (scanf("%1234566s", S) != 1) return 1;
		neko(&hoge.left, &hoge.delta, S);
		if (hoge.delta >= 0) p[pc++] = hoge; else m[mc++] = hoge;
	}
	qsort(p, pc, sizeof(*p), by_left);
	qsort(m, mc, sizeof(*m), by_left2);
	/* ハードルが低いのから使う */
	for (i = 0; i < pc; i++) {
		if (p[i].left > ayaneru) {
			puts("No");
			return 0;
		}
		ayaneru += p[i].delta;
		if (ayaneru < 0) {
			puts("No");
			return 0;
		}
	}
	/* ハードルが高いのからやっつける */
	for (i = 0; i < mc; i++) {
		if (m[i].left > ayaneru) {
			puts("No");
			return 0;
		}
		ayaneru += m[i].delta;
		if (ayaneru < 0) {
			puts("No");
			return 0;
		}
	}
	puts(ayaneru == 0 ? "Yes" : "No");
	return 0;
}

/*

1 ())()()(()(()() 2

差に基づく貪欲はこれでダメ

3
(((
)))(((((((
)))))))

-------------

4
(
)))(((((((
)(((
)))))))

*/
