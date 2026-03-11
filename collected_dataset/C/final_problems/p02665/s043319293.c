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
int32_t nextint(void){ char c=getchar_unlocked(); while(c!='-'&&(c<'0'||'9'<c)) c=getchar_unlocked(); int s=0; if(c=='-'){s=1;c=getchar_unlocked();} uint32_t x=0; while('0'<=c && c<='9'){ x=x*10+c-'0'; c=getchar_unlocked(); } return s?-x:x; }

int a[100001];
long b[100001];
long d[100001];
int main(void){
	int n=nextint();
	a[0]=nextint();
	if(n==0){
		if(a[0]==1){
			puts("1");
		}else{
			puts("-1");
		}
	}else{
		if(a[0]==0){
			d[0]=1;
			for(int i=1; i<=n; i++){
				a[i]=nextint();
				if(a[i]>d[i-1]*2){
					puts("-1");
					return 0;
				}
				d[i]=d[i-1]*2-a[i];
				if(d[i]>LONG_MAX/2) d[i]=LONG_MAX/2;
			}
			b[n]=0;
			for(int i=n; i>0; i--){
				if(d[i-1]>=a[i]+b[i]){
					b[i-1]=a[i]+b[i];
				}else{
					b[i-1]=d[i-1];
				}
			}
			long ans=0;
			for(int i=0; i<=n; i++){
				ans+=a[i];
				ans+=b[i];
			}
			printf("%ld\n", ans);
		}else{
			puts("-1");
		}
	}
}
