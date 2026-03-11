#include <stdio.h>
#include <stdlib.h>

typedef struct {
	int a,b;
	int cost;
} bridge_t;

int qsort_comp(const void* x,const void* y) {
	const bridge_t* a=(const bridge_t*)x;
	const bridge_t* b=(const bridge_t*)y;
	if((a->cost)>(b->cost))return 1;
	if((a->cost)<(b->cost))return -1;
	return 0;
}

int tosi_num,bridge_num;
bridge_t* bridge_list;

int tosi_uf[100];

void uf_init(void) {
	int i;
	for(i=0;i<100;i++)tosi_uf[i]=0;
}

void uf_merge(int a,int b) {
	int next;
	int rank_a=0,rank_b=0;
	while(tosi_uf[a]>0) {
		next=tosi_uf[a]-1;
		if(tosi_uf[next]>0)tosi_uf[a]=tosi_uf[next];
		a=next;
		rank_a++;
	}
	while(tosi_uf[b]>0) {
		next=tosi_uf[b]-1;
		if(tosi_uf[next]>0)tosi_uf[b]=tosi_uf[next];
		b=next;
		rank_b++;
	}
	if(a!=b) {
		if(rank_a<=rank_b)tosi_uf[a]=b+1;
		else tosi_uf[b]=a+1;
	}
}

int uf_isgroup(int a,int b) {
	int next;
	while(tosi_uf[a]>0) {
		next=tosi_uf[a]-1;
		if(tosi_uf[next]>0)tosi_uf[a]=tosi_uf[next];
		a=next;
	}
	while(tosi_uf[b]>0) {
		next=tosi_uf[b]-1;
		if(tosi_uf[next]>0)tosi_uf[b]=tosi_uf[next];
		b=next;
	}
	return (a==b)?1:0;
}

int main(void) {
	int i,j;
	int result;
	while(1) {
		scanf("%d%d",&tosi_num,&bridge_num);
		if(tosi_num==0 && bridge_num==0)break;
		bridge_list=malloc(bridge_num*sizeof(bridge_t));
		if(bridge_list==NULL)return 1;//error
		for(i=0;i<bridge_num;i++) {
			scanf("%d%d%d",
				&bridge_list[i].a,&bridge_list[i].b,
				&bridge_list[i].cost);
		}
		qsort(bridge_list,bridge_num,sizeof(bridge_t),qsort_comp);
		uf_init();
		result=0;
		for(i=0,j=1;i<bridge_num && j<tosi_num;i++) {
			if(!uf_isgroup(bridge_list[i].a,bridge_list[i].b)) {
				uf_merge(bridge_list[i].a,bridge_list[i].b);
				result+=bridge_list[i].cost;
				j++;
			}
		}
		free(bridge_list);
		printf("%d\n",result);
	}
	return 0;
}