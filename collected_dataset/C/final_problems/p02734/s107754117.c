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
int compr(const void *a, const void *b){return *(int*)b-*(int*)a;}

uint32_t nextpint(void){ char c=getchar(); while(c<'0'||'9'<c) c=getchar(); uint_fast32_t x=0; while('0'<=c && c<='9'){ x=x*10+c-'0'; c=getchar(); } return x; }

inline int min2(const int a, const int b){return a>b?b:a;}
inline int dif2(const int a, const int b){return a>b?a-b:b-a;}

int a[3001];
long d[3001];
int main(void){
	int n=nextpint();
	int s=nextpint();
	for(int i=1; i<=n; i++){
		a[i]=nextpint();
	}
	for(int i=1; i<=n; i++){
		for(int j=s; j>=1; j--){
			long D=0;
			if(a[i]==j){
				D=i;
			}else if(a[i]<j){
				D=d[j-a[i]];
			}
			if(j==s){
				D*=(n-i+1);
			}
			d[j]=(d[j]+D)%P;
		}
	}
	printf("%ld\n", d[s]);
}
