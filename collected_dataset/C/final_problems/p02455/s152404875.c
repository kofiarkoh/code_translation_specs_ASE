#include<stdio.h>
#include<stdlib.h>
#include<string.h>
#define LARGE 500000000



int main(){
	int q,i,com,x;
	int num=0;
	int *a;
	
	scanf("%d",&q);
	a=malloc(sizeof(int)*LARGE);
	for(i=0;i<LARGE;i++)a[i]=-1;
	
	for(i=0;i<q;i++){
		scanf("%d %d",&com,&x);
		if(com==0){
			if(a[x/2]==-1){
				if(x%2==0)a[x/2]=2;
				else a[x/2]=1;
				num++;
			}
			else if(a[x/2]==1){
				if(x%2==0)a[x/2]=3;
				else num--;
				num++;
			}
			else if(a[x/2]==2){
				if(x%2==0)num--;
				else a[x/2]=3;
				num++;
			}
			printf("%d\n",num);
		}
		
		else if(com==1){
			if(x%2==0 && (a[x/2]==2 || a[x/2]==3))printf("1\n");
			else if(x%2==1 && (a[x/2]==1 ||a[x/2]==3))printf("1\n");
			else printf("0\n");
		}
	}
	
	return 0;
}
