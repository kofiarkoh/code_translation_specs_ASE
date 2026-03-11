#include <ctype.h>
#include <float.h>
#include <inttypes.h>
#include <limits.h>
#include <signal.h>
#include <stdarg.h>
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
#pragma GCC target("avx2")
#pragma GCC optimize("O3")
#pragma GCC optimize("unroll-loops")

//char rbuf[]; int rbufl;
//#define getchar() rbuf[rbufl++]
#define getchar getchar_unlocked
#define putchar putchar_unlocked
int32_t nextint(void){ char c=getchar(); while(c!='-'&&(c<'0'||'9'<c)) c=getchar(); bool s=false; if(c=='-'){s=true;c=getchar();} uint32_t x=0; while('0'<=c && c<='9'){ x=x*10+c-'0'; c=getchar(); } return s?-x:x; }
int64_t nextlong(void){ char c=getchar(); while(c!='-'&&(c<'0'||'9'<c)) c=getchar(); int s=0; if(c=='-'){s=1;c=getchar();} uint64_t x=0; while('0'<=c && c<='9'){ x=x*10+c-'0'; c=getchar(); } return s?-x:x; }
uint32_t nextstr(char *s){ char c=getchar(); while(c==' '||c=='\n') c=getchar(); uint32_t len=0; while(c!=' '&&c!='\n'){ *s++=c; len++; c=getchar(); } *s='\0'; return len; }
void printint(int32_t x){ if(x<0){x=-x; putchar('-');}else if(x==0){putchar('0');} int d=0; char s[10]; while(x){ s[d]='0'+x%10; x/=10; d++; } for (int i=d-1; i>=0; i--) putchar(s[i]); putchar('\n'); }
void rsorta_d32(void *base, const int count, const int size, const int index){
	typedef int32_t rsort_t;
	rsort_t (*a)[size] = (rsort_t(*)[size])base;
	rsort_t (*b)[size] = (rsort_t(*)[size])malloc(count*size*sizeof(rsort_t));
	int c[0x100];
	for(int k=0; k<4; k++){
		memset(c, 0, sizeof(c)); for(int i=0; i<count; i++){ c[(a[i][index]>>(k<<3))&0xFF]++; } for(int i=1; i<0x100; i++) c[i]+=c[i-1]; for(int i=count-1; i>=0; i--){ int to=--c[(a[i][index]>>(k<<3))&0xFF]; for(int j=0; j<size; j++){ b[to][j]=a[i][j]; } }{rsort_t(*t)[size]=a; a=b; b=t;}
	}
	free(b);
}

#define P 998244353

int dat[1048577];
int d[10][2];

void update(int k, int x){
	k+=524288-1;
	dat[k]=(dat[k]+x)%P;
	while(k>0){
		k=(k-1)/2;
		dat[k]=(dat[k]+x)%P;
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
	return (vl+vr)%P;
}

inline int q(int l, int r){
	return query(l, r, 0, 0, 524288);
}

int main(void){
	int n=nextint();
	int k=nextint();
	for(int i=0; i<k; i++){
		d[i][0]=nextint();
		d[i][1]=nextint();
	}
	update(n, 1);
	for(int i=n-1; i>0; i--){
		long A=0;
		for(int j=0; j<k; j++){
			A += q(i+d[j][0], i+d[j][1]+1);
		}
		A%=P;
		update(i, A);
	}
	int ans = q(1, 2);
	printf("%d\n", ans);
}
