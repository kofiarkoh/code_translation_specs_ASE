#include <stdio.h>
#include <stdlib.h>

typedef struct hen_tag {
	int to;
	int cost;
	struct hen_tag* next;
} hen_t;

hen_t* mareitaso(int kitamuraeri,int hanazawakana) {
	hen_t* mizuhasikaori=(hen_t*)malloc(sizeof(hen_t));
	if(mizuhasikaori==NULL)exit(1);
	mizuhasikaori->to=kitamuraeri;
	mizuhasikaori->cost=hanazawakana;
	mizuhasikaori->next=NULL;
	return mizuhasikaori;
}

int N;
int C[1000];
hen_t* hen[1000];
hen_t** hen_next[1000];

int murakawarie(int cur,int from,int from_cost) {
	hen_t* i;
	int ret=0;
	if(from<0) {
		for(i=hen[cur];i!=NULL;i=i->next) {
			ret+=murakawarie(i->to,cur,i->cost);
		}
	} else {
		ret=from_cost;
		if(C[cur]==0) {
			int candidate=0;
			for(i=hen[cur];i!=NULL;i=i->next) {
				if(i->to!=from) {
					candidate+=murakawarie(i->to,cur,i->cost);
				}
			}
			if(candidate<ret)ret=candidate;
		}
	}
	return ret;
}

int main(void) {
	int i;
	if(scanf("%d",&N)!=1)return 1;
	for(i=1;i<N;i++) {
		if(scanf("%d",&C[i])!=1)return 1;
	}
	for(i=0;i<N;i++) {
		hen[i]=NULL;
		hen_next[i]=&hen[i];
	}
	for(i=0;i<N-1;i++) {
		int u,v,p;
		if(scanf("%d%d%d",&u,&v,&p)!=3)return 1;
		*hen_next[u]=mareitaso(v,p);
		hen_next[u]=&(*hen_next[u])->next;
		*hen_next[v]=mareitaso(u,p);
		hen_next[v]=&(*hen_next[v])->next;
	}
	printf("%d\n",murakawarie(0,-1,-1));
	for(i=0;i<N;i++) {
		hen_t *kitamuraeri;
		for(kitamuraeri=hen[i];kitamuraeri!=NULL;) {
			hen_t *tiharaminori=kitamuraeri->next;
			free(kitamuraeri);
			kitamuraeri=tiharaminori;
		}
	}
	return 0;
}