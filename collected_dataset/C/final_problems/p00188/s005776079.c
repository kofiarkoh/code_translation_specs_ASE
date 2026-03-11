#define _CRT_SECURE_NO_WARNINGS
//#define _USE_MATH_DEFINES
#include<stdio.h>
//#include<stdlib.h>
//#include<math.h>
//#include<string.h>
#define P(type,x) fprintf(stdout,"%"#type"\n",x)
int main() {
	int n, l, m, r, A[100] = {}, i,key,cnt;
	while (~fscanf(stdin, "%d", &n), n) {
		l = cnt = 0, r = n, m = (l + r) / 2;
		for (i = 0; i < n; i++) fscanf(stdin, "%d", &A[i]);
		fscanf(stdin, "%d", &key);
		while (l < r) {
			cnt++;
			if (A[m] < key) r = m, m = (l + r) / 2;
			else if (A[m] > key) l = m + 1, m = (l + r) / 2;
			else break;
		}
		P(d, cnt);
	}
	return 0;
}