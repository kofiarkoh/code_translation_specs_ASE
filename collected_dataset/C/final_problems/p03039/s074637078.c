#include <float.h>
#include <inttypes.h>
#include <limits.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <unistd.h>
#include <math.h>
#ifdef __cplusplus
#endif

int comp(const void *a, const void *b){return (*(int*)a>*(int*)b)-(*(int*)a<*(int*)b);}
int compr(const void *a, const void *b){return (*(int*)a<*(int*)b)-(*(int*)a>*(int*)b);}

uint32_t nextpint(void){ char c=getchar(); while(c<'0'||'9'<c) c=getchar(); uint32_t x=0; while('0'<=c && c<='9'){ x=x*10+c-'0'; c=getchar(); } return x; }
uint64_t nextplong(void){ char c=getchar(); while(c<'0'||'9'<c) c=getchar(); uint64_t x=0; while('0'<=c && c<='9'){ x=x*10+c-'0'; c=getchar(); } return x; }
uint32_t nextstr(char *s){ char c=getchar(); while(c==' '||c=='\n') c=getchar(); uint32_t len=0; while(c!=' '&&c!='\n'){ *s++=c; len++; c=getchar(); } *s='\0'; return len; }

inline int max2(const int a, const int b){return a>b?a:b;}
inline int min2(const int a, const int b){return a>b?b:a;}
inline int dif2(const int a, const int b){return a>b?a-b:b-a;}
inline int abs2(const int a){return a>=0?a:-a;}

#define P 1000000007
#define p 1000000007

int k[200000];
int rk[200000];

int getk(int a, int x){
	if(x==1) return a;
	int b = getk(a, x/2);
	b = ((long)b)*b%P;
	if((x&1)==0){
		return b;
	}else{
		return ((long)b)*a%P;
	}
}
 
int comb(int a, int b){
	if(a<b) return 0;
	return ((long)k[a])*rk[b]%P*rk[a-b]%P;
}

void init(int n){
	k[0]=1;
	for(int i=1; i<=n; i++){
		k[i]=k[i-1]*((long)i)%P;
	}	
	rk[n]=getk(k[n], P-2);
	for(int i=n-1; i>=0; i--){
		rk[i]=((long)rk[i+1])*(i+1)%P;
	}
}
int main(void){
	int n=nextpint();
	int m=nextpint();
	int k=nextpint();
	int ans=0;
	for(int i=1; i<n; i++){
		int now=i*(n-i)*m%p*m%p;
		ans=(ans+now)%p;
	}
	for(int i=1; i<m; i++){
		int now=i*(m-i)*n%p*n%p;
		ans=(ans+now)%p;
	}
	init(n*m-2);
	ans=(long)ans*comb(n*m-2, k-2)%p;
	printf("%d\n", ans);
}
