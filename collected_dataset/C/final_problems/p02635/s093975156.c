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

int dif2(int a, int b){return a<b?b-a:a-b;}

char s[301];
int a[302];
int b[302];
int d[302][302][604];
int main(void){
	int n=nextstr(s);
	int K=nextint();
	int m=0;
	for(int i=0; i<n; i++){
		if(s[i]=='0'){
			m++;
		}else{
			a[m]++;
		}
	}
	m++;
	b[m-1]=a[m-1];
	for(int i=m-2; i>=0; i--){
		b[i]=b[i+1]+a[i];
	}
	d[m][0][0]=1;
	for(int i=m-1; i>=0; i--){
		for(int j=0; j<=b[i]; j++){
			for(int k=0; k<=K*2; k++){
				for(int l=0; l<=j; l++){
					int p=k-dif2(a[i],l);
					if(p>=0){
						d[i][j][k]+=d[i+1][j-l][p];
						d[i][j][k]%=P;
					}
				}
			}
		}
	}
	long ans=0;
	for(int k=0; k<=K*2; k++){
		ans+=d[0][n-m+1][k];
	}
	ans%=P;
	printf("%ld\n", ans);
}
