#include<stdio.h>
#include<stdlib.h>
#define LARGE 100000
#define INF 1000000
#define MIN(a, b) ((a) < (b) ? (a) : (b))

int **matrix;
int V;

typedef struct node{
	int cost;
	int flg;
}node;


void Dijkstra(node N[],int start,int d){
	int min_idx=-1;
	int min=INF+1;
	int i;
	
	if(d==V-1)return;
	
	if(d==0){
		N[start].cost=0;
		N[start].flg=1;
	}
	
	for(i=0;i<V;i++){
		if(matrix[start][i]!=-1 && N[i].flg==0){
			N[i].cost=MIN(N[start].cost+matrix[start][i],N[i].cost);
		}
	}
	
	for(i=0;i<V;i++){
		if(N[i].flg==0){
			if(min > N[i].cost){
				min=N[i].cost;
				min_idx=i;
			}
		}
	}
	
	if(min_idx==-1){
		for(i=0;i<V;i++){
			if(N[i].flg==0){
				min_idx=i;
				break;
			}
		}
	}
	
	N[min_idx].flg=1;
	
	Dijkstra(N,min_idx,d+1);
	
	return;
	
}


int main(){
	int E,start;
	int s,t,d;
	int i,j;
	node N[LARGE];
	
	scanf("%d %d %d",&V,&E,&start);
	
	/*隣接行列とノードの初期化*/
	matrix=malloc(sizeof(int *)*V);
	for(i=0;i<V;i++){
		matrix[i]=malloc(sizeof(int)*V);
		for(j=0;j<V;j++){
			matrix[i][j]=-1;
		}
		N[i].cost=INF;
		N[i].flg=0;
	}
	
	/*隣接行列作成*/
	for(i=0;i<E;i++){
		scanf("%d %d %d",&s,&t,&d);
		matrix[s][t]=d;
	}
	
	Dijkstra(N,start,0);
	
	for(i=0;i<V;i++){
		if(N[i].cost==INF)printf("INF\n");
		else printf("%d\n",N[i].cost);
	}
	
	
	return 0;
	
}
