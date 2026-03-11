#include <limits.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <math.h>

//#define P 1000000007
#define P 998244353

int comp(const void *a, const void *b){return *(int*)a-*(int*)b;}
int compw(const void *a, const void *b){return (*(int*)a>*(int*)b)-(*(int*)a<*(int*)b);}

uint32_t nextpint(void){
	char c=getchar();
	while(c<'0'||'9'<c) c=getchar();
	uint_fast32_t x=0;
	while('0'<=c && c<='9'){ x=x*10+c-'0'; c=getchar(); }
	return x;
}

#define N 200000
inline int max(const int a, const int b){ return a>b?a:b; }
int m;
int dat[524288-1];

void init(int n){
	m=1;
	while(m<n) m*=2;
}

void update(int k, int x){
	k+=m-1;
	dat[k]=x;
	while(k>0){
		k=(k-1)/2;
		dat[k]=max(dat[k*2+1], dat[k*2+2]);
	}
}

// q(a, b, 0, 0, m)
// [a, b)
int query(int a, int b, int k, int l, int r){
	if(r<=a || b<=l) return 0;
	if(a<=l && r<=b) return dat[k];
	int m=(l+r)/2;
	int vl = query(a, b, k*2+1, l, m);
	int vr = query(a, b, k*2+2, m, r);
	return max(vl, vr);
}

int n;
int x[N][2];
int bs(int X){
	int l=0, r=n-1;
	while(l<=r){
		int m=(l+r)/2;
		if(x[m][0]>=X) r=m-1; else l=m+1;
	}
	return l;
}
int d[N+1];
int main(int argc, char *argv[], char *envp[]){
	scanf("%d", &n);
	for(int i=0; i<n; i++){
		scanf("%d%d", &x[i][0], &x[i][1]);
	}
	qsort(x, n, sizeof(int)*2, comp);
	init(n);
	for(int i=n-1; i>=0; i--){
		int l=bs(x[i][0]+x[i][1])-1;
		int r=i;
		if(i<l){
			int j=query(i+1, l+1, 0, 0, m);
			if(j>r) r=j;
		}
		update(i, r);
	}
	d[n]=1;
	for(int i=n-1; i>=0; i--){
		d[i]=(d[i+1]+d[dat[i+m-1]+1])%P;
	}
	printf("%d\n", d[0]);
}
