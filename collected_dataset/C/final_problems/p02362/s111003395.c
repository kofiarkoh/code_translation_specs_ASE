#include <stdio.h>
//#include <limits.h>
int INF = 1000000009;
int main(){
	int n,m,s,i,j;
	scanf("%d%d%d",&n,&m,&s);
	int e[2100][3],dist[2100],update=1;
	for(i=0;i<m;i++){
		for(j=0;j<3;j++){
			scanf("%d",&e[i][j]);
		}
	}
	for(i=0;i<n;i++){
		dist[i]=INF;
	}
	dist[s]=0;
	for(i=0;i<n-1;i++){
		update=0;
		for(j=0;j<m;j++){
			if(dist[e[j][0]]!=INF&&dist[e[j][1]]>dist[e[j][0]]+e[j][2]){
				dist[e[j][1]]=dist[e[j][0]]+e[j][2];
				update=1;
			}
		}
		if(update==0)break;
	}
	for(j=0;j<m;j++){
		if(dist[e[j][0]]!=INF&&dist[e[j][1]]>dist[e[j][0]]+e[j][2]){
			printf("NEGATIVE CYCLE\n");
			return 0;
		}
	}
	for(i=0;i<n;i++){
		//if(i>0)printf("\n");
		if(dist[i]==INF){
			printf("INF\n");
		}
		else{
			printf("%d\n",dist[i]);
		}
	}
	return 0;
}