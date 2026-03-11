#include <stdio.h>
#include <stdlib.h>

int uf_table[100];

void uf_init(void) {
	int i;
	for(i=0;i<100;i++)uf_table[i]=0;
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
	int a,b;
	int weight;
} connect_t;

int qsort_comp(const void* x,const void* y) {
	const connect_t* a=(const connect_t*)x;
	const connect_t* b=(const connect_t*)y;
	if((a->weight)>(b->weight))return 1;
	if((a->weight)<(b->weight))return -1;
	return 0;
}

int choten_num;
int connection_num;
connect_t connections[100*(100-1)/2];

int main(void) {
	int i,j;
	int count;
	int result,now_result;
	while(1) {
		scanf("%d%d",&choten_num,&connection_num);
		if(choten_num==0 && connection_num==0)break;
		for(i=0;i<connection_num;i++) {
			scanf("%d%d%d",&connections[i].a,&connections[i].b,
				&connections[i].weight);
			connections[i].a--;
			connections[i].b--;
		}
		qsort(connections,connection_num,sizeof(connect_t),qsort_comp);
		result=0x7fffffff;
		for(j=0;j<connection_num;j++) {
			count=1;
			uf_init();
			for(i=j;i<connection_num && count<choten_num;i++) {
				if(!uf_issame(connections[i].a,connections[i].b)) {
					uf_merge(connections[i].a,connections[i].b);
					count++;
				}
			}
			if(count>=choten_num) {
				now_result=connections[i-1].weight
					-connections[j].weight;
				if(now_result<result)result=now_result;
			} else break;
		}
		if(result==0x7fffffff)puts("-1"); else printf("%d\n",result);
	}
	return 0;
}