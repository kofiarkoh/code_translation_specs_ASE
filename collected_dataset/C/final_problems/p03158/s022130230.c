#include <stdio.h>
#include <stdlib.h>

#define DEBUG 0

int iniPos(int n,unsigned long long a[],unsigned long long int x)
{
	int i;
	for (i=0;i<n;i++) {
		if (a[i] >= x) break;
	}
	return i-1;
}

char judgeContact(int rP,int tP)
{
	if (rP > tP) return 1;
	return 0;
}

char nearCard(unsigned long long int x[],int n,int i,int l,int r)
{
	unsigned long long int now,left,right;
	now = x[i];
	if (l < 0) left = -9999;
	else left = x[l];
	if (r >= n) right = 1000000001;
	else right = x[r];

	if ((now - left) <= (right - now)) return 0;
	else return 1;
}

int main(void)
{
	int n, q, i, j, num;
	unsigned long long int *a;
	unsigned long long int *x;
	
	int aokiPos,leftPos,rightPos,takaPos;
	char des,turn;
	unsigned long long int *res;

	scanf("%d %d",&n,&q);

	a = (unsigned long long int *)malloc(sizeof(unsigned long long int) * n);
	x = (unsigned long long int *)malloc(sizeof(unsigned long long int) * q);
	res = (unsigned long long int *)malloc(sizeof(unsigned long long int) * q);
	

	for (i=0;i<n;i++) {
		scanf("%lld",&a[i]);
		if (DEBUG) printf("a[%d] = %lld\n",i,a[i]);
	}
	for (i=0;i<q;i++) {
		scanf("%lld",&x[i]);
		if (DEBUG) printf("x[%d] = %lld\n",i,x[i]);
	}

	for (i=0;i<q;i++) {
		if (DEBUG) printf("i = %d\n",i);
		res[i] = 0;
		aokiPos = iniPos(n,a,x[i]);
		leftPos = aokiPos;
		rightPos = aokiPos+1;
		takaPos = n-1;
		num = n;
		
		// before contact
		while (1) {
			if (DEBUG) printf("=== %d %d %d %d\n",aokiPos,leftPos,rightPos,takaPos);
	
			if (judgeContact(rightPos,takaPos) == 1) {
				turn = 0;
				break;
			}
		// turn of Takahashi
			if (DEBUG) printf("res + %lld\n",a[takaPos]);
			res[i] += a[takaPos];
			takaPos -= 1;
			num -= 1;

			if (judgeContact(rightPos,takaPos) == 1) {
				turn = 1;
				break;
			}

			if (DEBUG) printf("=== %d %d %d %d\n",aokiPos,leftPos,rightPos,takaPos);
			// turn of Aoki
			des = nearCard(x,n,i,leftPos,rightPos);
			if (des == 0) {
				leftPos -= 1;
			} else {
				rightPos += 1;
			}
			num -= 1;
		}

		// after contact
		if (DEBUG) printf("after\n");
			// turn of Takahashi
		if (turn == 1) {
			leftPos -= 1;
			num -= 1;
		}
		while (1) {
			if (num <= 0) break;
			if (DEBUG) printf("res + %lld\n",a[leftPos]);
			res[i] += a[leftPos];
			leftPos -= 2;
			num -= 2;
		}

		printf("%lld\n",res[i]);
	}


	return 0;
}