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

#define getchar getchar_unlocked
#define putchar putchar_unlocked
int32_t nextint(void){ char c=getchar_unlocked(); while(c!='-'&&(c<'0'||'9'<c)) c=getchar_unlocked(); bool s=false; if(c=='-'){s=true;c=getchar_unlocked();} uint32_t x=0; while('0'<=c && c<='9'){ x=x*10+c-'0'; c=getchar_unlocked(); } return s?-x:x; }
int64_t nextlong(void){ char c=getchar_unlocked(); while(c!='-'&&(c<'0'||'9'<c)) c=getchar_unlocked(); int s=0; if(c=='-'){s=1;c=getchar_unlocked();} uint64_t x=0; while('0'<=c && c<='9'){ x=x*10+c-'0'; c=getchar_unlocked(); } return s?-x:x; }
uint32_t nextstr(char *s){ char c=getchar_unlocked(); while(c==' '||c=='\n') c=getchar_unlocked(); uint32_t len=0; while(c!=' '&&c!='\n'){ *s++=c; len++; c=getchar_unlocked(); } *s='\0'; return len; }

#define P 998244353

int a[3000];
int b[3001][3001];
int main(void){
	int n=nextint();
	int s=nextint();
	for(int i=0; i<n; i++){
		a[i]=nextint();
	}
	b[0][0]=1;
	for(int i=0; i<=n; i++){
		for(int j=s-a[i]+1; j<=s; j++){
			b[i+1][j]=b[i][j]*2%P;
		}
		for(int j=s-a[i]; j>=0; --j){
			b[i+1][j]=b[i][j]*2%P;
			b[i+1][j]%=P;
			b[i+1][j+a[i]]+=b[i][j];
			b[i+1][j+a[i]]%=P;
		}
	}
	printf("%d\n", b[n][s]);
}
