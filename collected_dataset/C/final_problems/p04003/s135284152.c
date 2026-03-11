#include <stdio.h>
#include <stdlib.h>
#define FOR(i,a,n) for(i=a;i<n;i++)
#define MAX(a,b) (((a)>(b))?(a):(b))
#define INF 1000000000

typedef struct LIST{
	int to,cost;
	struct LIST *next;
}Edge;
typedef struct{
	int num,dist;
}Pos;
Edge** init(int v){
	Edge **edge=(Edge**)calloc(v,sizeof(Edge*));
	return edge;
}
void add_edge(Edge **edge,int s,int t,int c){
	Edge *tmp=(Edge*)malloc(sizeof(Edge));
	tmp->to=t,tmp->cost=c;
	tmp->next=edge[s];
	edge[s]=tmp;
	return;
}
void delete_edge(Edge **edge,int v){
	int i;
	Edge *e,*tmp;
	FOR(i,0,v){
		e=edge[i];
		while(e!=NULL){
			tmp=e->next;
			free(e);
			e=tmp;
		}
	}
	return;
}
void push(Pos *heap,int *heap_size,int n,int d){
	int i=(*heap_size)++;
	while(i>0){
		int p=(i-1)/2;
		if(heap[p].dist<=d) break;
		heap[i]=heap[p];
		i=p;
	}
	heap[i].num=n;
	heap[i].dist=d;
	return;
}
Pos pop(Pos *heap,int *heap_size){
	Pos data=heap[0];
	Pos n=heap[--*heap_size];
	int a,b,pos=0;
	while(pos*2+1<*heap_size){
		a=pos*2+1;
		b=pos*2+2;
		if(b<*heap_size&&heap[a].dist>heap[b].dist) a=b;
		if(n.dist<=heap[a].dist) break;
		heap[pos]=heap[a];
		pos=a;
	}
	heap[pos]=n;
	return data;
}
void dijkstra(Edge **edge,int v,int d[],int s){
	int i,heap_size=0;
	Pos p,*heap;
	Edge *e;
	heap=calloc(v,sizeof(Edge));
	FOR(i,0,v) d[i]=INF;
	d[s]=0;
	push(heap,&heap_size,s,0);
	while(heap_size){
		p=pop(heap,&heap_size);
		if(d[p.num]<p.dist) continue;
		for(e=edge[p.num];e!=NULL;e=e->next){
			if(d[e->to]>d[p.num]+e->cost){
				d[e->to]=d[p.num]+e->cost;
				push(heap,&heap_size,e->to,d[e->to]);
			}
		}
	}
	free(heap);
	return;
}
Edge **graph,**company,**edge;
int num[1000000];
int d[500000];
int t=0;
int main(void)
{
	int n,m,p,q,c,i;
	Edge *e,*e2;
	scanf("%d%d",&n,&m);
	graph=init(2*m+n);
	company=init(m);
	edge=init(n);
	FOR(i,0,m){
		scanf("%d%d%d",&p,&q,&c);
		p--,q--,c--;
		if(num[c]==0) num[c]=++t;
		add_edge(company,num[c]-1,p,q);
		add_edge(edge,p,q,num[c]-1);
		add_edge(edge,q,p,num[c]-1);
	}
	FOR(i,0,t){
		if(num[i]==0) continue;
		for(e=company[i];e!=NULL;e=e->next){
			add_edge(graph,e->to+i,e->cost+i,0);
			add_edge(graph,e->cost+i,e->to+i,0);
		}
	}
	FOR(i,0,n){
		for(e=edge[i];e!=NULL;e=e->next){
			add_edge(graph,i+e->cost,2*m+i,0);
			add_edge(graph,2*m+i,e->to+e->cost,1);
		}
	}
	/*FOR(i,0,2*m+n){
		for(e=graph[i];e!=NULL;e=e->next)
		    printf("%d %d %d\n",i,e->to,e->cost);
	}*/
	dijkstra(graph,2*m+n,d,2*m);
	printf("%d\n",d[2*m+n-1]==INF?-1:d[2*m+n-1]);
	delete_edge(graph,2*m+n);
	delete_edge(company,m);
	delete_edge(edge,n);
	return 0;
}
