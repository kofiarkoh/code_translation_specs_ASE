
#pragma warning(disable:4996)

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#define _USE_MATH_DEFINES
#include <math.h>
#include <float.h>

#pragma region 定義
typedef long long  ll;
typedef unsigned long long  ull;


#define _itoa(A,N) sprintf(A, "%d", N);

int i_dsort(const void* a, const void* b);
int i_asort(const void* a, const void* b);

int  _gcd(int a, int b);
int  _swp(int* a, int* b);
int _cknum(char* a, int n);
int _atoi(char* s, int len);

int s_asort(const void* a, const void* b);
int s_dsort(const void* a, const void* b);

//昇順
int s_asort(const void* a, const void* b) {
	return(strcmp((char*)a, (char*)b));
}
int s_dsort(const void* a, const void* b) {
	return(strcmp((char*)b, (char*)a));
}

int i_dsort(const void* a, const void* b) {
	return(*(int*)b - *(int*)a);
}
int i_asort(const void* a, const void* b) {
	return(*(int*)a - *(int*)b);
}

int _gcd(int a, int b) {
	int r, tmp;
	if (a < b) { tmp = a; a = b; b = tmp; }
	r = a % b;
	while (r) {
		if (!r) break;
		a = b; b = r;
		r = a % b;
	}
	return(b);
}
// puts("Yes\n") puts("No\n")
// scanf("%d.%d") 浮動小数点入力を整数２つで得る


//#define pai  3.1415926535 
//#define  _USE_MATH_DEFINES でM_PIが使える

#define deg_rad(X)  (X*(M_PI/180))


int _swp(int* a, int* b)
{
	int tmp;
	tmp = *b; *b = *a; *a = tmp;
	return 0;
}
#define _abs(x) ((x)>0?(x):-(x))
#define _min(x,y) ((x)<(y)?(x):(y))
#define _max(x,y) ((x)<(y)?(y):(x))

int _cknum(char* a, int n) {
	int i;
	char t = '0';

	for (i = 0; i < n; i++) {
		if (a[i] < '0' || a[i]>'9') return 1;
	}
	return 0;
}


int _atoi(char* s, int len) {
	char tmp[20];
	memcpy(tmp, s, len);
	tmp[len] = 0x00;
	return (atoi(tmp));
}

//素因数分解　pf素数が入る　pc 素数の数
ll _pfact(ll a);
static ll pf[100000000];
static ll pc;

ll _pfact(ll a) {
	long i, r;
	if (a % 2 == 0) {
		pf[pc] = 2; pc++;
		return(_pfact(a / 2));
	}
	r = sqrt(a);
	for (i = 3; i <= r; i += 2) {
		if (a % i == 0) {
			pf[pc] = i; pc++;
			return(_pfact(a / i));
		}
	}
	if (a != 1) {
		pf[pc] = a; pc++;
	}
	return 0;

}


#pragma endregion

////ABC  115-C Chrismas Eve
//int l_asort(const void* a, const void* b) {
//	return(*(long*)a - *(long*)b);
//}
//
//int main(void) {
//	int        i, n, k;
//	long        h[100001],t[100000];
//
//	scanf("%d %d", &n,&k);
//	for (i = 0; i < n; i++) 
//		scanf("%d", &h[i]);
//
//	qsort(h, n, sizeof(long), l_asort);
//
//	for (i = 0; i < n; i++)
//		printf("%ld ", h[i]);
//	//printf("\n");
//
//	long min = 1000000001;
//	for (i = 0; i <= n - k; i++) {
//		min = _min((h[i + k-1] - h[i]), min);
//		printf("%ld %ld %ld \n", h[i], h[i + k - 1],min);
//	}
//	
//	printf("\n%ld\n", min);
//	return 0;
//}

// 123 C-Five Transportations
int main(void) {
	ll n, a, b, c, d, e, ans;

	scanf("%lld %lld %lld %lld %lld %lld", &n, &a, &b, &c, &d, &e);
	ll min;
	min = _min(a, _min(_min(b, c), _min(d, e)));
	//printf("%lld\n", min);
	ans = n / min;
	printf("%lld\n", ans+5);
	return 0;
}
