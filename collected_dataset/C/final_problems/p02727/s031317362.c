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
#include <bits/stdc++.h>
#endif

int comp(const void *a, const void *b){return (*(int*)a>*(int*)b)-(*(int*)a<*(int*)b);}
int compr(const void *a, const void *b){return (*(int*)a<*(int*)b)-(*(int*)a>*(int*)b);}

#define putchar(c) putc_unlocked((c), stdout);
uint32_t nextpint(void){ char c=getc_unlocked(stdin); while(c<'0'||'9'<c) c=getc_unlocked(stdin); uint32_t x=0; while('0'<=c && c<='9'){ x=x*10+c-'0'; c=getc_unlocked(stdin); } return x; }
uint64_t nextplong(void){ char c=getc_unlocked(stdin); while(c<'0'||'9'<c) c=getc_unlocked(stdin); uint64_t x=0; while('0'<=c && c<='9'){ x=x*10+c-'0'; c=getc_unlocked(stdin); } return x; }
uint32_t nextstr(char *s){ char c=getc_unlocked(stdin); while(c==' '||c=='\n') c=getc_unlocked(stdin); uint32_t len=0; while(c!=' '&&c!='\n'){ *s++=c; len++; c=getc_unlocked(stdin); } *s='\0'; return len; }
uint32_t nextstrln(char *s){ char c=getc_unlocked(stdin); while(c==' '||c=='\n') c=getc_unlocked(stdin); uint32_t len=0; while(c!='\n'){ *s++=c; len++; c=getc_unlocked(stdin); } *s='\0'; return len; }

void printint(int x){ if(x<0){x=-x; putc_unlocked('-', stdout);}else if(x==0){putc_unlocked('0', stdout);} int d=0; char s[10]; while(x){ s[d]='0'+x%10; x/=10; d++; } for (int i=d-1; i>=0; i--) putc_unlocked(s[i], stdout); }
void printlong(long x){ if(x<0){x=-x; putc_unlocked('-', stdout);}else if(x==0){putc_unlocked('0', stdout);} int d=0; char s[20]; while(x){ s[d]='0'+x%10; x/=10; d++; } for (int i=d-1; i>=0; i--) putc_unlocked(s[i], stdout); }

inline int max2(const int a, const int b){return a>b?a:b;}
inline int min2(const int a, const int b){return a>b?b:a;}
inline int dif2(const int a, const int b){return a>b?a-b:b-a;}
inline int abs2(const int a){return a>=0?a:-a;}

void rsortr_u32(void *base, const int count);

int p[100000];
int q[100000];
int r[100000];
int main(void){
	int x=nextpint()-1;
	int y=nextpint()-1;
	int z=0;
	int a=nextpint();
	int b=nextpint();
	int c=nextpint();
	for(int i=0; i<a; i++){ p[i]=nextpint(); }
	for(int i=0; i<b; i++){ q[i]=nextpint(); }
	for(int i=0; i<c; i++){ r[i]=nextpint(); }
	rsortr_u32(p, a);
	rsortr_u32(q, b);
	rsortr_u32(r, c);
	while(z<c){
		if(x>=0 && p[x]<r[z]){
			if(y>=0 && q[y]<r[z]){
				if(p[x]<q[y]){
					x--;
				}else{
					y--;
				}
			}else{
				x--;
			}
		}else{
			if(y>=0 && q[y]<r[z]){
				y--;
			}else{
				break;
			}
		}
		z++;
	}
	long ans=0;
	for(int i=0; i<=x; i++){ ans+=p[i]; }
	for(int i=0; i<=y; i++){ ans+=q[i]; }
	for(int i=0; i<z; i++){ ans+=r[i]; }
	printf("%ld\n", ans);
}
void rsortr_u32(void *base, const int count){
	uint32_t *a=(uint32_t *)base;
	uint32_t *b=(uint32_t *)malloc(count*sizeof(uint32_t));
	int *c=(int *)malloc(0x100*sizeof(int));
	memset(c, 0, 0x100*sizeof(int)); for(int i=0; i<count; i++){ c[(a[i]>>(0<<3))&0xFF]++; } for(int i=0xFE; i>=0; i--) c[i]+=c[i+1]; for(int i=count-1; i>=0; i--){ b[--c[(a[i]>>(0<<3))&0xFF]]=a[i]; }{uint32_t *t=a; a=b; b=t;}
	memset(c, 0, 0x100*sizeof(int)); for(int i=0; i<count; i++){ c[(a[i]>>(1<<3))&0xFF]++; } for(int i=0xFE; i>=0; i--) c[i]+=c[i+1]; for(int i=count-1; i>=0; i--){ b[--c[(a[i]>>(1<<3))&0xFF]]=a[i]; }{uint32_t *t=a; a=b; b=t;}
	memset(c, 0, 0x100*sizeof(int)); for(int i=0; i<count; i++){ c[(a[i]>>(2<<3))&0xFF]++; } for(int i=0xFE; i>=0; i--) c[i]+=c[i+1]; for(int i=count-1; i>=0; i--){ b[--c[(a[i]>>(2<<3))&0xFF]]=a[i]; }{uint32_t *t=a; a=b; b=t;}
	memset(c, 0, 0x100*sizeof(int)); for(int i=0; i<count; i++){ c[(a[i]>>(3<<3))&0xFF]++; } for(int i=0xFE; i>=0; i--) c[i]+=c[i+1]; for(int i=count-1; i>=0; i--){ b[--c[(a[i]>>(3<<3))&0xFF]]=a[i]; }{uint32_t *t=a; a=b; b=t;}
	free(c);
	free(b);
}
