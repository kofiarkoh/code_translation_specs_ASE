#include<stdio.h>
int main(){
	int x,y,a,f,i;
	while(scanf("%d %d",&x,&y)!=EOF){
		if(x==0 && y==0)break;
		a=x+y;f=0;
		for(i=1;i<5;i++){
			scanf("%d %d",&x,&y);
			if(a<x+y)a=x+y,f=i;
		}
		printf("%c %d\n",65+f,a);
	}
	
	return 0;
}