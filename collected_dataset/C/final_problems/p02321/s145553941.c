#include<stdio.h>
#include<stdlib.h>
#define INF 210000000
#define LARGE 41

typedef struct set{
	int value;
	int weight;
}set;

int r=0;
int N,W;
	int v[LARGE];
	int w[LARGE];

void Get_sum(set a[],int now,int end,set temp){
	
	if(now>end /*||temp.weight >W*/){a[r++]=temp;return;}

	Get_sum(a,now+1,end,temp);
	
	temp.value+=v[now];
	temp.weight+=w[now];

	Get_sum(a,now+1,end,temp);
	
	return;
}


int main(){

	set s1[LARGE];
	set s2[LARGE];
	set *a;
	set temp;
	
	scanf("%d %d",&N,&W);
	a=malloc(sizeof(set)*10000000);
	for(int i=0;i<N;i++){
		scanf("%d %d",&v[i],&w[i]);
	}
	temp.value=0;
	temp.weight=0;
	
	Get_sum(a,0,N-1,temp);
	
	int max=-1;
	for(int i=0;i<r;i++){
		if(a[i].weight<=W && max<a[i].value)max=a[i].value;
	}
	printf("%d\n",max);
	
	return 0;
}
