#include<stdio.h>
#define LARGE 2000
#define INF 1000000000
#define MIN(a, b) ((a) < (b) ? (a) : (b))

int V,e;

typedef struct edge{
	int from;
	int to;
	int cost;
}edge;

int BellmanFord(edge E[],int C[],int start){
	int i,j;
	int cnt=0;
	for(j=0;j<V-1;j++){
	
		for(i=0;i<e;i++){
			if(C[E[i].from]!=INF && C[E[i].to] > C[E[i].from]+E[i].cost){
				C[E[i].to]=C[E[i].from]+E[i].cost;
				cnt++;
				if(C[start]<0){
					printf("NEGATIVE CYCLE\n");
					return 0;
				}
			}
		}
		
	}
	
	if(cnt>e){
		printf("NEGATIVE CYCLE\n");
		return 0;
	}
	
	return 1;
}
	
	

int main(){
	int start;
	int s,t,d;
	int i,flg;
	int C[LARGE];
	edge E[LARGE];
	
	scanf("%d %d %d",&V,&e,&start);
	
	for(i=0;i<e;i++){
		scanf("%d %d %d",&s,&t,&d);
		E[i].from=s;
		E[i].to=t;
		E[i].cost=d;
	}
	
	for(i=0;i<V;i++){
		C[i]=INF;
	}
	
	C[start]=0;
	
	flg=BellmanFord(E,C,start);
	
	if(flg==0)return 0;
	
	for(i=0;i<V;i++){
		if(C[i]==INF)printf("INF\n");
		else printf("%d\n",C[i]);
	}
	
	return 0;
	
}
