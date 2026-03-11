#include <stdio.h>
#include <stdlib.h>

#define UNION_FIND_MAX 1000

int uf_table[UNION_FIND_MAX];

void uf_init(void) {
	int i;
	for(i=0;i<UNION_FIND_MAX;i++)uf_table[i]=0;
}

void uf_merge(int a,int b) {
	int next;
	while(uf_table[a]>0) {
		next=uf_table[a]-1;
		if(uf_table[next]>0)uf_table[a]=uf_table[next];
		a=next;
	}
	while(uf_table[b]>0) {
		next=uf_table[b]-1;
		if(uf_table[next]>0)uf_table[b]=uf_table[next];
		b=next;
	}
	if(a!=b)uf_table[a]=b+1;
}

int uf_issame(int a,int b) {
	int next;
	while(uf_table[a]>0) {
		next=uf_table[a]-1;
		if(uf_table[next]>0)uf_table[a]=uf_table[next];
		a=next;
	}
	while(uf_table[b]>0) {
		next=uf_table[b]-1;
		if(uf_table[next]>0)uf_table[b]=uf_table[next];
		b=next;
	}
	return a==b;
}

typedef struct {
	int s,t,c;
} edge_t;

int qscedge(const void* x,const void* y) {
	int a=((const edge_t*)x)->c;
	int b=((const edge_t*)y)->c;
	if(a>b)return 1;
	if(a<b)return -1;
	return 0;
}

int main(void) {
	int n,m;
	static edge_t edges[10000];
	while(scanf("%d%d",&n,&m)==2 && (n|m)!=0) {
		int i;
		int left,right;
		for(i=0;i<m;i++) {
			scanf("%d%d%d",&edges[i].s,&edges[i].t,&edges[i].c);
		}
		qsort(edges,m,sizeof(edge_t),qscedge);
		left=0;right=m-1;
		while(left<=right) {
			int mid=(left+right)/2;
			int count=0;
			uf_init();
			for(i=0;i<m;i++) {
				if(!uf_issame(edges[i].s,edges[i].t)) {
					uf_merge(edges[i].s,edges[i].t);
					if(mid<i)count++;
				}
			}
			if(count<=n/2-1)right=mid-1; else left=mid+1;
		}
		printf("%d\n",edges[right+1].c);
	}
	return 0;
}