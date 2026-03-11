#include <stdio.h>

#define MAX 1000000
typedef long long llong;

int n, k;
llong T[MAX];

int check(llong P)
{
	int i =0;
	for(int j=0; j<k; j++){
		llong s = 0;
		while(s + T[i] <= P){
			s += T[i];
			i++;
			if(i == n) return n;
		}
	}
	return i;
}


int solve()
{
	int v;
	llong left = 0;
	llong right = 100000 * 100000;
	llong mid;
	while(right - left > 1){
		mid = (left + right) / 2;
		v = check(mid);
		if(v >= n){right = mid; }
		else {left = mid;}
	}
	return right;
}


int main(void)
{
	scanf("%d", &n);
	scanf("%d", &k);
	
	llong ans;
	for(int i=0; i<n; i++){
		scanf("%d", &T[i]);
	}
	ans = solve();
	printf("%d\n", ans);
	
	return 0;
}

