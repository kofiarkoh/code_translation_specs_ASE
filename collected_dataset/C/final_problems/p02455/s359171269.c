#include<stdio.h>
#define N 200000

int main(){
	int q,i,x,com,j;
	int a[N];
	int num=0;
	
	
	scanf("%d",&q);
	for(i=0;i<q;i++){
		scanf("%d %d",&com,&x);
		
		if(com==0){
			for(j=0;j<num;j++){
				if(a[j]==x)break;
			}
			if(j==num)a[num++]=x;
			printf("%d\n",num);
		}
		
		else {
			for(j=0;j<num;j++){
				if(a[j]==x){
					printf("1\n");
					break;
				}
			}
			if(j==num)printf("0\n");
		}
	}
	
	return 0;
}
			
