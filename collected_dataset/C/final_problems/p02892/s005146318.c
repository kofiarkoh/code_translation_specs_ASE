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
uint32_t nextstr(char *s){ char c=getchar_unlocked(); while(c==' '||c=='\n') c=getchar_unlocked(); uint32_t len=0; while(c!=' '&&c!='\n'){ *s++=c-'0'; len++; c=getchar_unlocked(); } *s='\0'; return len; }
void printint(uint32_t x){ if(x<0){x=-x; putchar_unlocked('-');}else if(x==0){putchar_unlocked('0');} int d=0; char s[10]; while(x){ s[d]='0'+x%10; x/=10; d++; } for (int i=d-1; i>=0; i--) putchar_unlocked(s[i]); putchar_unlocked('\n'); }

uint32_t n;
char s[200][201];
char cp[200];

bool mcp(int p){
	for(uint32_t i=0; i<n; i++){
		if(s[p][i]==1){
			if(cp[i]==0){
				cp[i]=3-cp[p];
				mcp(i);
			}else if(cp[i]==cp[p]){
				return false;
			}
		}
	}
	return true;
}

int main(void){
	n=nextpint();
	for(uint32_t i=0; i<n; i++){
		nextstr(s[i]);
	}
	for(uint32_t i=0; i<n; i++){
		if(cp[i]==0){
			cp[i]=1;
			if(!mcp(i)){
				puts("-1");
				return 0;
			}
		}
	}
	for(uint32_t k=0; k<n; k++){
		for(uint32_t i=0; i<n; i++){
			for(uint32_t j=0; j<n; j++){
				if(i!=j && s[i][k]!=0 && s[k][j]!=0){
					uint32_t S=s[i][k]+s[k][j];
					if(s[i][j]==0 || s[i][j]>S) s[i][j]=S;
				}
			}
		}
	}
	uint32_t max=0;
	for(uint32_t i=0; i<n; i++){
		for(uint32_t j=0; j<n; j++){
			//putchar(s[i][j]+'0');
			if(s[i][j]>max) max=s[i][j];
		}
		//putchar(10);
	}
	printint(max==0?n:max+1);
}
