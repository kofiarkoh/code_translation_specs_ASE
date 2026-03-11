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

void rsort_d32(void *base, const int count){
	int32_t *a=base;
	int32_t *b=malloc(count*sizeof(int32_t));
	int c[0x100];
	memset(c, 0, sizeof(c)); for(int i=0; i<count; i++){ c[(a[i]>>(0<<3))&0xFF]++; } for(int i=1; i<0x100; i++) c[i]+=c[i-1]; for(int i=count-1; i>=0; i--){ b[--c[(a[i]>>(0<<3))&0xFF]]=a[i]; }{void *t=a; a=b; b=t;}
	memset(c, 0, sizeof(c)); for(int i=0; i<count; i++){ c[(a[i]>>(1<<3))&0xFF]++; } for(int i=1; i<0x100; i++) c[i]+=c[i-1]; for(int i=count-1; i>=0; i--){ b[--c[(a[i]>>(1<<3))&0xFF]]=a[i]; }{void *t=a; a=b; b=t;}
	memset(c, 0, sizeof(c)); for(int i=0; i<count; i++){ c[(a[i]>>(2<<3))&0xFF]++; } for(int i=1; i<0x100; i++) c[i]+=c[i-1]; for(int i=count-1; i>=0; i--){ b[--c[(a[i]>>(2<<3))&0xFF]]=a[i]; }{void *t=a; a=b; b=t;}
	memset(c, 0, sizeof(c)); for(int i=0; i<count; i++){ c[(a[i]>>(3<<3))+0x80]++; } for(int i=1; i<0x100; i++) c[i]+=c[i-1]; for(int i=count-1; i>=0; i--){ b[--c[(a[i]>>(3<<3))+0x80]]=a[i]; }{void *t=a; a=b; b=t;}
	free(b);
}
int32_t nextint(void){ char c=getchar_unlocked(); while(c!='-'&&(c<'0'||'9'<c)) c=getchar_unlocked(); int s=0; if(c=='-'){s=1;c=getchar_unlocked();} uint32_t x=0; while('0'<=c && c<='9'){ x=x*10+c-'0'; c=getchar_unlocked(); } return s?-x:x; }

#define P 1000000007

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

int a[100000];
int main(void){
	int n=nextint();
	int k=nextint();
	for(int i=0; i<n; i++){
		a[i]=nextint();
	}
	rsort_d32(a, n);
	comb_init(n);
	long ans=0;
	for(int i=0; i<=n-k; i++){
		int c = comb(n-i-1, k-1);
		ans+=(long)(a[n-i-1]-a[i])*c;
		ans%=P;
	}
	printf("%ld\n", ans);
}
