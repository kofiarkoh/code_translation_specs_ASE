#include <ctype.h>
#include <float.h>
#include <inttypes.h>
#include <limits.h>
#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <math.h>
#ifdef __cplusplus
#include <bits/stdc++.h>
#endif

#define ROOT(d, x) (d).root(&(d), (x))
#define UNITE(d, x, y) (d).unite(&(d), (x), (y))
#define UPDATE(d, x, y) (d).update(&(d), (x), (y))
#define QUERY(d, ...) (d).query(&(d), ##__VA_ARGS__)


static inline int get(void);
static inline void put(int);

int32_t nextint(void);
int64_t nextlong(void);
uint32_t nextstr(char *);
void printint(int32_t);
void printlong(int64_t);
void rsort_d32(void *base, const int n, const int size, const int index);
void rsort_r32(void *base, const int n, const int size, const int index);

static inline uint32_t flp2(uint32_t x);


typedef struct segment_int{
	int n;
	int *data, defval;
	int (*op)(int, int);
	void (*update)(struct segment_int *, int, int);
	int (*query)(struct segment_int *, int, int);
} segment_int;

typedef struct segment{
	int n, size;
	void *data, *defval;
	void (*op)(void *, void *, void *);
	void (*update)(struct segment *, int, void *);
	void (*query)(struct segment *, int, int, void *);
} segment;

void segment_int_init(struct segment_int *p, int n, int (*op)(int, int), int defval);
void segment_init(struct segment *p, int n, int size, void (*op)(void *, void *, void *), void *defval);


/* main */


void max2(void *a, void *b, void *c){ *(int*)c=*(int*)a<*(int*)b?*(int*)b:*(int*)a; }

segment s;
int zero=0;

int main(void){
	int n=nextint();
	int k=nextint();
	segment_init(&s, 300001, sizeof(int), max2, &zero);
	for(int i=0; i<n; i++){
		int a=nextint();
		int now;
		QUERY(s, a-k, a+k, &now);
		now++;
		UPDATE(s, a, &now);
	}
	int ans;
	QUERY(s, INT_MIN, INT_MAX, &ans);
	printf("%d\n", ans);
}
/* definition */


static inline int get(void){ return getchar_unlocked(); }
static inline void put(int c){ putchar_unlocked(c); }

int32_t nextint(void){ char c=get(); while(c!='-'&&(c<'0'||'9'<c)) c=get(); bool s=false; if(c=='-'){s=true;c=get();} uint32_t x=0; while('0'<=c && c<='9'){ x=x*10+c-'0'; c=get(); } return s?-x:x; }
int64_t nextlong(void){ char c=get(); while(c!='-'&&(c<'0'||'9'<c)) c=get(); int s=0; if(c=='-'){s=1;c=get();} uint64_t x=0; while('0'<=c && c<='9'){ x=x*10+c-'0'; c=get(); } return s?-x:x; }
uint32_t nextstr(char *s){ char c=get(); while(c==' '||c=='\n') c=get(); uint32_t len=0; while(c!=' '&&c!='\n'){ *s++=c; len++; c=get(); } *s='\0'; return len; }
void printint(int32_t x){ if(x<0){x=-x; putchar('-');}else if(x==0){putchar('0');} int d=0; char s[10]; while(x){ s[d]='0'+x%10; x/=10; d++; } for (int i=d-1; i>=0; i--) putchar(s[i]); putchar('\n'); }
void printlong(int64_t x){ if(x<0){x=-x; putchar('-');}else if(x==0){putchar('0');} int d=0; char s[20]; while(x){ s[d]='0'+x%10; x/=10; d++; } for (int i=d-1; i>=0; i--) putchar(s[i]); putchar('\n'); }

void rsort_d32(void *base, const int count, const int size, const int index){
	typedef int32_t rsort_t;
	rsort_t (*a)[size] = (rsort_t(*)[size]) base;
	rsort_t (*b)[size] = (rsort_t(*)[size]) malloc(count*size*sizeof(rsort_t));
	int c[0x100];
	for(int k=0; k<4; k++){
		memset(c, 0, sizeof(c)); for(int i=0; i<count; i++){ c[(a[i][index]>>(k<<3))&0xFF]++; } for(int i=1; i<0x100; i++) c[i]+=c[i-1]; for(int i=count-1; i>=0; i--){ int to=--c[(a[i][index]>>(k<<3))&0xFF]; for(int j=0; j<size; j++){ b[to][j]=a[i][j]; } }{rsort_t(*t)[size]=a; a=b; b=t;}
	}
	free(b);
}
void rsortr_d32(void *base, const int count, const int size, const int index){
	typedef int32_t rsort_t;
	rsort_t (*a)[size] = (rsort_t(*)[size]) base;
	rsort_t (*b)[size] = (rsort_t(*)[size]) malloc(count*size*sizeof(rsort_t));
	int c[0x100];
	for(int k=0; k<4; k++){
		memset(c, 0, sizeof(c)); for(int i=0; i<count; i++){ c[(a[i][index]>>(k<<3))&0xFF]++; } for(int i=0xFE; i>=0; i--) c[i]+=c[i+1]; for(int i=count-1; i>=0; i--){ int to=--c[(a[i][index]>>(k<<3))&0xFF]; for(int j=0; j<size; j++){ b[to][j]=a[i][j]; } }{rsort_t(*t)[size]=a; a=b; b=t;}
	}
	free(b);
}

static inline uint32_t flp2(uint32_t x){ return 0x80000000>>(__builtin_clz(x-1)-1); }

static void segment_int_update(struct segment_int *p, int k, int x){
	k += p->n;
	p->data[k] = x;
	k=k/2;
	while(k>0){
		p->data[k] = p->op(p->data[k*2], p->data[k*2+1]);
		k=k/2;
	}
}
static int segment_int_query(struct segment_int *p, int l, int r){
	if(l<0) l=0;
	if(r>=p->n) r=p->n-1;
	l+=p->n; r+=p->n;
	int ans = p->defval;
	while(l<=r){
		if(l%2==1){ ans = p->op(ans, p->data[l]); l++; }
		if(r%2==0){ ans = p->op(ans, p->data[r]); r--; }
		l/=2; r/=2;
	}
}
void segment_int_init(struct segment_int *p, int n, int (*op)(int, int), int defval){
	p->n = flp2(n);
	p->data = (int*) malloc(p->n * 2 * sizeof(int));
	for(int i=0; i<p->n; i++){ p->data[i] = defval; }
	p->defval = defval;
	p->op = op;
	p->update = segment_int_update;
	p->query = segment_int_query;
}

static void segment_update(struct segment *p, int k, void *x){
	char (*data)[p->size] = (char(*)[p->size]) p->data;
	k += p->n;
	memcpy(data[k], x, p->size);
	k=k/2;
	while(k>0){
		p->op(data[k*2], data[k*2+1], data[k]);
		k=k/2;
	}
}
static void segment_query(struct segment *p, int l, int r, void *dest){
	char (*data)[p->size] = (char(*)[p->size]) p->data;
	if(l<0) l=0;
	if(r>=p->n) r=p->n-1;
	l+=p->n; r+=p->n;
	void *tmp[2] = { malloc(p->size), malloc(p->size) };
	memcpy(tmp[0], p->defval, p->size);
	int t=0;
	while(l<=r){
		if(l%2==1){ t=1-t; p->op(tmp[1-t], data[l], tmp[t]); l++; }
		if(r%2==0){ t=1-t; p->op(tmp[1-t], data[r], tmp[t]); r--; }
		l/=2; r/=2;
	}
	memcpy(dest, tmp[t], p->size);
	free(tmp[0]); free(tmp[1]);
}
static inline void memcpyex(void *dest, void *src, size_t n, size_t size){
	char (*data)[size] = (char(*)[size]) dest;
	for(int i=0; i<n; i++){ memcpy(data[i], src, size); }
}
void segment_init(struct segment *p, int n, int size, void (*op)(void *, void *, void *), void *defval){
	p->n = flp2(n);
	p->data = malloc(p->n*2*size);
	memcpyex(p->data, defval, p->n*2, size);
	p->defval = malloc(size);
	memcpy(p->defval, defval, size);
	p->size = size;
	p->op = op;
	p->update = segment_update;
	p->query = segment_query;
}

