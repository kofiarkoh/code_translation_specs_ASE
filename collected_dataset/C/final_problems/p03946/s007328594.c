#include<stdio.h>

int main(){
	int n,t;
	scanf("%d %d",&n,&t);
	int a[n];
	int i,j;
	int diffMax=0,diffMaxNum=0;
	int temp=0;
	
	for(i=0;i<n;i++){
		scanf("%d",&a[i]);
	}
	
	for(i=0;i<n;i++){
		temp = 0;
		for(j=i+1;j<n;j++){
			if(a[j]>temp){
				temp=a[j];
			}
//			printf("i=%2d j=%2d temp=%d\n",i,j,temp);
		}
		if(temp-a[i]>diffMax){
			diffMax = temp-a[i];
			diffMaxNum = 1;
		}else if(temp -a[i] == diffMax){
			diffMaxNum++;
		}
	}
	
	printf("%d\n",diffMaxNum);
	
	return 0;
}