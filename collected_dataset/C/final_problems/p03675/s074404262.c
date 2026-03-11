//#include "oj/atcoder.h"
//#include "std.h"
#include <stdlib.h>
#include <stdbool.h>
#include <limits.h>
#include <stdio.h>
#include <string.h>
#include <math.h>
#include <assert.h>
#include <float.h>

#define MIN(a,b) ((a)<(b)?(a):(b))
#define MAX(a,b) ((a)>(b)?(a):(b))
#define SWAP(T,a,b) do { T _tmp=a; a=b; b=_tmp; } while (0)

//#include "oj/io.h"

void print_ai(int *a, int n) {
	for (int i=0; i<n; i++) {
		if (i) printf(" ");
		printf("%d", a[i]);
	}
	printf("\n");
}

void scan_ai(int *a, int n) {
	for (int i=0; i<n; i++)
		scanf("%d", &a[i]);
}

//#include "oj/debug.h"

void print_hr() {
	printf("---------------------------\n");
}



int main() {
	int n;
	while (scanf("%d", &n)==1) {
		int a[n];
		scan_ai(a, n);
		// 2 0 1 3
		// 4 2 0 1 3
		int p = n-1;
		printf("%d", a[n-1]);
		p -= 2;
		while (p >=0) {
			printf(" %d", a[p]);
			p -= 2;
		}
		p = (n%2==0 ? 0: 1);
		while (p < n) {
			printf(" %d", a[p]);
			p += 2;
		}
		printf("\n");
	}
}
