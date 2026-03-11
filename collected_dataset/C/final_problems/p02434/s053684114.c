#include<stdio.h>
#include<stdlib.h>
#define N 1000

typedef struct array{
	int *a;
	int num;
}array;

int main(){
	int n,q,i,com,x,y;
	array S[N];
	
	scanf("%d %d",&n,&q);
	
	for(i=0;i<n;i++){
		S[i].a=malloc(sizeof(int)*q);
		S[i].num=0;
	}
	
	for(i=0;i<q;i++){
		scanf("%d",&com);
		
		if(com==0){
			scanf("%d %d",&x,&y);
			S[x].a[S[x].num++]=y;
		}
		
		if(com==1){
			scanf("%d",&x);
			if(S[x].num==0)printf("\n");
			else {
				for(int j=0;j<S[x].num-1;j++)printf("%d ",S[x].a[j]);
				printf("%d\n",S[x].a[S[x].num-1]);
			}
		}
		
		if(com==2){
			scanf("%d",&x);
			free(S[x].a);
			S[x].a=malloc(sizeof(int)*(q-i));
			S[x].num=0;
		}
		
	}
	
	return 0;
	
}
