/*

結果:
*/
#define _USE_MATH_DEFINES
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>
#include <limits.h>  
#include <stdbool.h>   
#define inf (INT_MAX-1)
#define INF 9223372036854775807
#define PI 3.14159265358979323846;
#define EPS 1e-10
#define sq(n) ((n)*(n))
#define rep(i,n) for(i=0;i<n;i++)
#define rev(i,n) for(i=n-1;i>=0;i--)
/*
#define sort(a,n) qsort(a,n,sizeof(TYPE),cmp)
#define sort_r(a,n) qsort(a,n,sizeof(TYPE),cmp_r)
*/
#define chsort(s,n) qsort(s,n,sizeof(char),cmp)
#define chsort_r(s,n) qsort(s,n,sizeof(char),char_cmp_r)
#define TYPE long long
#define ll long long
#define MEMSET(a) memset(a,0,sizeof(a))
#define MEMSET_U(a) memset(a,-1,sizeof(a))
const int mod = (int)1e09 + 7;

//#define DEBUG1
//#define DEBUG2
//#define DEBUGF
#define DUMMY

int in(void) { int i; scanf("%d", &i); return i; }
long long llin(void) { long long i; scanf("%lld", &i); return i; }
double din(void) { double i; scanf("%lf", &i); return i; }
void chin(char s[]) { scanf("%s", s); }

void print(int a) { printf("%d\n", a); }
void llprint(long long a) { printf("%lld\n", a); }
void dprint(double a) { printf("%.10f\n", a); }
void print2(int a, int b) { printf("%d %d\n", a, b); }
int Max(int a, int b) { if (a > b) { return a; }return b; }
int Min(int a, int b) { if (a < b) { return a; }return b; }
long long llmax(long long a, long long b) { return a > b ? a : b; }
long long llmin(long long a, long long b) { return a < b ? a : b; }
double dmax(double a, double b) { return a > b ? a : b; }
double dmin(double a, double b) { return a < b ? a : b; }
//long long llmax(long long a, long long b) { return a > b ? a : b; }
//long long llmin(long long a, long long b) { return a < b ? a : b; }
//double dmax(double a, double b) { return a > b ? a : b; }
int cmp(const void *a, const void *b) { return *(TYPE *)a - *(TYPE *)b; }
int cmp_r(const void *a, const void *b) { return *(TYPE *)b - *(TYPE *)a; }
int char_cmp(const void *a, const void *b) { return strcmp((char *)a, (char *)b); }
int char_cmp_r(const void *a, const void *b) { return strcmp((char *)b, (char *)a); }
void swap(int *a, int *b) { int t = *a; *a = *b; *b = t; }

//int main() {
//	int W, H, N;
//	int xu = 100, xd = 0, yu = 100, yd = 0;
//	int ans = 0;
//	scanf("%d%d%d", &W, &H, &N);
//	xu = W;
//	yu = H;
//	int **num = (int **)malloc(sizeof(int *) * N);
//	for (int i = 0; i < N; i++) {
//		num[i] = (int *)malloc(sizeof(int) * 3);
//		num[i][0] = 0;
//		num[i][1] = 0;
//		num[i][2] = 0;
//	}
//	for (int i = 0; i < N; i++) {
//		scanf("%d%d%d", &num[i][0], &num[i][1], &num[i][2]);
//		if (num[i][2] == 1 && num[i][0] > xd) { xd = num[i][0]; }
//		if (num[i][2] == 3 && num[i][0] > yd) { yd = num[i][1]; }
//		if (num[i][2] == 2 && num[i][0] < xu) { xu = num[i][0]; }
//		if (num[i][2] == 4 && num[i][0] < yu) { yu = num[i][1]; }
//	}
//	if (xd >= xu || yd >= yu || xd >= W || xu == 0 || yd == H || yu == 0) {
//
//	}
//	else {
//		ans = (xu - xd)*(yu - yd);
//	}
//	printf("%d\n", ans);
//
//#ifdef DEBUGF
//	getch();
//#endif
//	return 0;
//}

int main() {
	int W, H, N;
	int tmp = 0, ans = 0;

	scanf("%d%d%d", &W, &H, &N);
	int **num = (int **)malloc(sizeof(int *) * N);
	for (int i = 0; i < N; i++) {
		num[i] = (int *)malloc(sizeof(int) * 3);
		num[i][0] = 0;
		num[i][1] = 0;
		num[i][2] = 0;
	}
	int xmin = 0, xmax = W, ymin = 0, ymax = H;

	for (int i = 0; i < N; i++) {
		scanf("%d%d%d", &num[i][0], &num[i][1], &num[i][2]);
		if (num[i][2] == 1) { xmin = Max(xmin, num[i][0]); }
		if (num[i][2] == 3) { ymin = Max(ymin, num[i][1]); }
		if (num[i][2] == 2) { xmax = Min(xmax, num[i][0]); }
		if (num[i][2] == 4) { ymax = Min(ymax, num[i][1]); }
	}

	if ((xmax - xmin) <= 0 || (ymax - ymin) <= 0) {
		ans = 0;
	}
	else {
		ans = (xmax - xmin)*(ymax - ymin);
	}

	printf("%d\n", ans);

#ifdef DEBUGF
	getch();
#endif
	return 0;
}