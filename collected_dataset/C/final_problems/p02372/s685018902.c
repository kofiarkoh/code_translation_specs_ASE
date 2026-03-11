#include<stdio.h>
#include<stdlib.h>
#define INF 2100000000
#define LARGE 10000

typedef struct node{
	int cost;
	int flg;
}node;

typedef struct list{
	int id;
	int cost;
	struct list *next;
}list;


node N[LARGE];
list *L[LARGE];
int n;


void Make_AdjList(list* L[],int s,int t,int d){
	list *new=malloc(sizeof(list));
	list **p=&L[s];
	/*card作成*/
	new->id=t;	new->cost=d;  new->next=NULL;
	
	while((*p)!=NULL)p=&((*p)->next);
	(*p)=new;
	
	return;
}


void dfs(int start,int cnt){
	int u=start;
	list *p=L[u];
	
	if(cnt==n)return;
	
	while(p!=NULL){
		if(N[p->id].flg==0){
			N[p->id].cost=N[u].cost+p->cost;
			N[p->id].flg=1;
			cnt++;
			dfs(p->id,cnt);
		}
		p=p->next;
	}
	
	return;
	
}


int main(){
	int i,j,max,idx;
	int s,t,w;

	
	scanf("%d",&n);
	
	/*初期化*/
	for(i=0;i<n;i++){
		N[i].cost=INF; N[i].flg=0;
		L[i]=NULL;
	}
	N[0].cost=0; N[0].flg=1;
	
	/*隣接リスト作成*/
	while(scanf("%d %d %d",&s,&t,&w)!=EOF){
		if(s==-1)break;
		Make_AdjList(L,s,t,w);
		Make_AdjList(L,t,s,w);
	}
	
	
	for(i=0;i<n;i++){
	
		for(j=0;j<n;j++){
			N[j].cost=INF; N[j].flg=0;
		}
		N[i].cost=0; N[i].flg=1;
		
		dfs(i,1);
		max=-1;
		for(j=0;j<n;j++){
			if(N[j].cost!=INF && max < N[j].cost){
				max=N[j].cost;
			}
		}
		printf("%d\n",max);
		
	}
	

	return 0;
}
