#include<stdio.h>
#include<stdlib.h>
#define N 1000

typedef struct stack{
	int *a;
	int top;
}stack;

int main(){
	int n,q,i,x,y;
	int com;
	stack S[N];
	
	scanf("%d %d",&n,&q);
	
	for(i=0;i<n;i++){
		S[i].top=0;
		S[i].a=malloc(sizeof(int)*q);
	}
	
	for(i=0;i<q;i++){
		scanf("%d",&com);
		
		if(com==0){
			scanf("%d %d",&x,&y);
			S[x].a[(S[x].top)++]=y;
		}
		
		else if(com==1){
			scanf("%d",&x);
			if(S[x].top!=0)printf("%d\n",S[x].a[S[x].top-1]);
		}
		
		else if(com==2){
			scanf("%d",&x);
			if(S[x].top!=0)S[x].top--;
		}
	}
	
	return 0;
	
}
