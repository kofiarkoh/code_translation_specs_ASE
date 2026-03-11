#include <stdio.h>
int main(void){
	int n,p,d1,d2,i,no,sum=0,NO[100],SUM[100],count=0;
	scanf("%d",&n);
	while(n>0){
		for(i=0;i<n;i++){
			scanf("%d %d %d",&p,&d1,&d2);
			if(sum<d1+d2){
				sum=d1+d2;
				no=p;
			}
		}
		NO[count]=no;
		SUM[count]=sum;
		count++;
		scanf("%d",&n);
	}
	for(i=0;i<count;i++)
		printf("%d %d\n",NO[i],SUM[i]);
	return 0;
	
}