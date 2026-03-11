#include<stdio.h>
#include<stdlib.h>
#define LARGE 100000
#define INF 2000000000
#define MIN(a, b) ((a) < (b) ? (a) : (b))

int id_to_heapid[LARGE];
int V,E;

typedef struct list{
	int id;
	int cost;
	struct list *next;
}list;

typedef struct node{
	int id;
	int cost;
}node;

void swap(node N[],int a,int b){/*a,bはheap番号*/
	node temp=N[a];
	N[a]=N[b];
	N[b]=temp;
	
	int TEMP=id_to_heapid[N[a].id];
	id_to_heapid[N[a].id]=id_to_heapid[N[b].id];
	id_to_heapid[N[b].id]=TEMP;
	
	return;
}

void Make_AdjList(list* L[],int s,int t,int d){
	list *new=malloc(sizeof(list));
	list **p=&L[s];
	/*card作成*/
	new->id=t;	new->cost=d;  new->next=NULL;
	
	while((*p)!=NULL)p=&((*p)->next);
	(*p)=new;
	
	return;
}


void down_to_heap(node N[],int last){
	int i=0;
	while(i<last){
		
		if(i*2+1 > last && i*2+2 >last)break;/*子なし*/
		
		else if(i*2+1==last){/*子1つ*/
			if(N[i].cost > N[i*2+1].cost){
				swap(N,i,i*2+1);
				i=last;
			}
			else break;;
		}
		
		else if(i*2+1 <= last && i*2+2 <= last){/*子2つ*/
			if(N[i*2+1].cost <= N[i*2+2].cost && N[i*2+1].cost < N[i].cost){
				swap(N,i,i*2+1);
				i=i*2+1;
			}
			
			else if(N[i*2+1].cost > N[i*2+2].cost && N[i*2+2].cost < N[i].cost){
				swap(N,i,i*2+2);
				i=i*2+2;
			}
			
			else break;
		}
		
	}
	
	return;
	
}

void up_to_heap(node N[],int i){
	
	while(i>0){
		if(i%2==0){
			if(N[i/2-1].cost > N[i].cost){
				swap(N,i/2-1,i);
				i=i/2-1;
			}
			else break;
		}
		else {
			if(N[i/2].cost > N[i].cost){
				swap(N,i/2,i);
				i=i/2;
			}
			else break;
		}
	}
	
	return;
}


int Dijkstra(list* L[],node N[],int num){
	int u=N[0].id;
	list *p=L[u];
	
	swap(N,0,num-1);
	num--;
	down_to_heap(N,num-1);
	
	while(p!=NULL){
		if(N[ id_to_heapid[p->id] ].cost > N[ id_to_heapid[u] ].cost+(p->cost)){
			N[ id_to_heapid[p->id] ].cost=N[id_to_heapid[u] ].cost+(p->cost);
			up_to_heap(N,id_to_heapid[p->id]);
		}
		p=p->next;
	}
	
	return num;
	
}


int main(){
	int i,j;
	int s,t,d;
	int num,start;
	list *L[LARGE];
	node N[LARGE];
	
	scanf("%d %d %d",&V,&E,&start);
	num=V;
	
	for(i=0;i<V;i++){
		L[i]=NULL;
		N[i].id=i;
		N[i].cost=INF;
		id_to_heapid[i]=i;
	}
	N[id_to_heapid[start]].cost=0;
	swap(N,0,id_to_heapid[start]);
	
	for(i=0;i<E;i++){
		scanf("%d %d %d",&s,&t,&d);
		Make_AdjList(L,s,t,d);
	}
	
	while(num>0){
		num=Dijkstra(L,N,num);
	}
	
	for(i=0;i<V;i++){
		if(N[id_to_heapid[i]].cost==INF)printf("INF\n");
		else printf("%d\n",N[id_to_heapid[i]].cost);
	}
	
	return 0;
	
}
