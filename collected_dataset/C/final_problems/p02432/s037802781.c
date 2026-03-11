#include<stdio.h>
#include<stdlib.h>

int main(){
	int q;
	int com,x,y;
	int *a;
	int idx=0;
	
	scanf("%d",&q);
	a=malloc(sizeof(int)*q);
	
	for(int i=0;i<q;i++){
		scanf("%d",&com);
		
		switch(com){
			case 0:scanf("%d %d",&x,&y);
				   if(!x){
				   		for(int j=idx-1;j>=0;j--){
				   			a[j+1]=a[j];
				   		}
				   		a[0]=y;
				   		idx++;
				   }
				   else a[idx++]=y;
				   break;
			
			case 1:scanf("%d",&x);
				   printf("%d\n",a[x]);
				   break;
				   
			case 2:scanf("%d",&x);
				   if(!x){
				   		for(int j=1;j<idx;j++){
				   			a[j-1]=a[j];
				   		}
				   		idx--;
				   }
				   else a[idx--]=-1;
				   break;
		}
	}
	
	free(a);
	
	return 0;
	
}
