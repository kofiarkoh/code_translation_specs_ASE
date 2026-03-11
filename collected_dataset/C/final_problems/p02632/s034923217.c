#include <float.h>
#include <inttypes.h>
#include <limits.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <unistd.h>
#include <math.h>
#ifdef __cplusplus
#include <bits/stdc++.h>
#endif

#define getchar getchar
#define putchar putchar
int32_t nextint(void){ char c=getchar(); while(c!='-'&&(c<'0'||'9'<c)) c=getchar(); bool s=false; if(c=='-'){s=true;c=getchar();} uint32_t x=0; while('0'<=c && c<='9'){ x=x*10+c-'0'; c=getchar(); } return s?-x:x; }
int64_t nextlong(void){ char c=getchar(); while(c!='-'&&(c<'0'||'9'<c)) c=getchar(); int s=0; if(c=='-'){s=1;c=getchar();} uint64_t x=0; while('0'<=c && c<='9'){ x=x*10+c-'0'; c=getchar(); } return s?-x:x; }
uint32_t nextstr(char *s){ char c=getchar(); while(c==' '||c=='\n') c=getchar(); uint32_t len=0; while(c!=' '&&c!='\n'){ *s++=c; len++; c=getchar(); } *s='\0'; return len; }

#define P 1000000007

int k[200001];
int rk[200001];

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
void comb_init(int n){
	k[0]=1;
	for(int i=1; i<=n; i++){
		k[i]=k[i-1]*((long)i)%P;
	}	
	rk[n]=getk(k[n], P-2);
	for(int i=n-1; i>=0; i--){
		rk[i]=((long)rk[i+1])*(i+1)%P;
	}
}

char s[1000001];
int a[40000][100];
int r25[1000001];
int r26[1000001];
int main(void){
	int m=nextint();
	int n=nextstr(s);
	r25[0]=1;
	r26[0]=1;
	for(int i=1; i<=m; i++){
		r25[i]=(long)r25[i-1]*25%P;
		r26[i]=(long)r26[i-1]*26%P;
	}
	long ans=0;
	comb_init(n+m);
	for(int i=m; i>=0; i--){
		long now=(long)comb(n+i-1,i)*r25[i]%P*r26[m-i]%P;
		ans=(ans+now)%P;
	}
	printf("%ld\n", ans);
	/*
	a[0][0]=1;
	for(int i=1; i<=n+m; i++){
		a[i][0]=(long)a[i-1][0]*26%P;
		for(int j=1; j<=n; j++){
			a[i][j]=((long)a[i-1][j]*25+a[i-1][j-1])%P;
		}
	}
	printf("%d\n", a[n+m][n]);
	*/
}
