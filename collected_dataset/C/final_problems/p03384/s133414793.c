#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define Rep(i,n) for(int (i)=0;(i)<(n);(i)++)

int **edge,*dim;

int Dig(int start,int last,int *newarr,int now){
	int way=(last==0?dim[start]:dim[start]-1);
	newarr[now]=way;
	int next=0,count=0;
	for(int i=1;count<way;i++){
		if(edge[i][start] && i!=last)
		{
			count++;
			if(dim[i]>1){
				if(next) return 0; 
				next=i;
			}
		}
	}
	if(next) return Dig(next,start,newarr,now+1);
	return 1;
}

int Arrcmp(int *arr1,int *arr2,int len){
	Rep(i,len){
		if(arr1[i]>arr2[i]) return 1;
		if(arr1[i]<arr2[i]) return 0;
	}	
	return 0;
}

int main(){
	int n;
	scanf("%d",&n);
	dim=malloc(sizeof(int)*(n+1));
    edge=malloc(sizeof(int*)*(n+1));
	int *ans=malloc(sizeof(int)*(n-1));
	Rep(i,n+1){ edge[i]=malloc(sizeof(int)*(1+n)); dim[i]=0; }
	Rep(i,n+1) Rep(j,n+1){ edge[i][j]=0; }
	Rep(i,n-1){	
		ans[i]=0;
		int a,b;
		scanf("%d %d",&a,&b);
		dim[a]++;
		dim[b]++;
		edge[a][b]=1;
		edge[b][a]=1;
	}
	int judge=0;
	Rep(i,n){
		int err=0;
		int *new=malloc(sizeof(int)*(n-1));
		Rep(j,n-1) new[j]=0;
		if(Dig(i+1,0,new,0) && !judge || Arrcmp(ans,new,n-1)){	
			if(!judge) judge=1;
			Rep(l,n-1) ans[l]=new[l];
		}		
	}
	if(!judge){ printf("-1"); return 0; }
	else{
		int num=1;
		for(int i=0;i<n-1&&ans[i]>0;i++){
			Rep(j,ans[i]-1){
				printf("%d ",num+1+j);
			}
			printf("%d ",num);
			num+=ans[i];
		}
		printf("%d\n",num);
	}
	return 0;
}
