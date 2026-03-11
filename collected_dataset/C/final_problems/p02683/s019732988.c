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

uint32_t nextpint(void){ char c=getchar_unlocked(); while(c<'0'||'9'<c) c=getchar_unlocked(); uint32_t x=0; while('0'<=c && c<='9'){ x=x*10+c-'0'; c=getchar_unlocked(); } return x; }

uint32_t c[12];
uint32_t a[12][12];
int main(void){
	uint32_t n=nextpint();
	uint32_t m=nextpint();
	uint32_t x=nextpint();
	for(uint32_t i=0; i<n; i++){
		c[i]=nextpint();
		for(uint32_t j=0; j<m; j++){
			a[i][j]=nextpint();
		}
	}
	uint32_t ans = UINT32_MAX;
	uint32_t l=1<<n;
	for(uint32_t i=0; i<l; i++){
		uint32_t now=0;
		uint32_t r[12]={};
		for(uint32_t j=0; j<n; j++){
			if(i&(1<<j)){
				now+=c[j];
				for(uint32_t k=0; k<m; k++){
					r[k]+=a[j][k];
				}
			}
		}
		bool f=true;
		for(uint32_t k=0; k<m; k++){
			if(r[k]<x) f=false;
		}
		if(f&&now<ans) ans=now;
	}
	printf(ans==UINT32_MAX?"-1\n":"%"PRIu32"\n", ans);
}
