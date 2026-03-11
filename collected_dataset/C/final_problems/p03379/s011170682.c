#define _CRT_SECURE_NO_WARNINGS

#include<stdio.h>
#include<string.h>
#include<stdlib.h>
#include<math.h>

//for
#define REP(i,n) for(int i=0,i##_len=(int)(n);i<i##_len;i++)
#define REPS(i,n) for(int i=1,i##_len=(int)(n);i<=i##_len;i++)
#define RREP(i,n) for(int i=(int)(n)-1;i>=0;i--)
#define RREPS(i,n) for(int i=(int)(n);i>0;i--)
#define SREP(i,s,n) for(int i=(int)(s),i##_len=(int)(n);i<i##_len;i++)

//printf
#define PRINTD(d) printf("%d\n",(d))
#define PRINTF(d) printf("%lld\n",(d))

//memset
#define m0(s) memset(s,0,sizeof(s))
#define ml(s) memset(s,63,sizeof(s))
#define fill(s,c) memset(s,c,sizeof(s))

#define INF 1e9
#define NMAX 1e9

typedef long long ll;
int diff[4][2] = {
	{ 0, -1 },
	{ -1, 0 },
	{ 1, 0 },
	{ 0, 1 },
};

int x[200000];
int tmp[200000];

int Min(int a, int b) {
	return (a)<(b) ? (a) : (b);
}
int Max(int a, int b) {
	return (a) > (b) ? (a) : (b);
}
void Swap(int x[], int i, int j)
{
	int temp;

	temp = x[i];
	x[i] = x[j];
	x[j] = temp;
}
void QSort(int x[], int left, int right, int n) {
	int i, j;	//左端,右端
	int pivot;	//軸

	i = left;
	j = right;
	pivot = x[(left + right) / 2];

	while (1) {
		if (n > 0) {				//n>0なら昇順、n<=0なら降順
			while ((x[i] < pivot) && (i <= right))i++;	//軸値より大きい要素
			while ((pivot < x[j]) && (i <= right))j--;	//軸値より小さい要素
		}
		else {
			while ((x[i] > pivot) && (i <= right))i++;	//軸値より小さい要素
			while ((pivot > x[j]) && (i <= right))j--;	//軸値より大きい要素
		}
		if (i >= j)	break;
		Swap(x, i, j);
		i++; j--;
	}
	//ShowData(x, left, right);

	if (left < i - 1)QSort(x, left, i - 1, n);
	if (j + 1 <  right)QSort(x, j + 1, right, n);
}

int main() {
	int n;
	scanf("%d", &n);

	REP(i, n) {
		scanf("%d", &x[i]);
		tmp[i] = x[i];
	}
	QSort(tmp, 0, n - 1, 1);

	int s = tmp[(n - 1) / 2];
	int b = tmp[(n - 1) / 2 + 1];

	REP(i, n) {
		if (x[i] <= s) {
			PRINTD(b);
		}
		else {
			PRINTD(s);
		}
	}
}