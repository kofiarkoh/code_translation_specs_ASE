#include<stdio.h>
void sort(int box[]){
	int i,j,temp;
	for(i=0;i<7;i++){
		for(j=7;j>i;j--){
			if(box[j]>box[j-1]){
				temp=box[j];
				box[j]=box[j-1];
				box[j-1]=temp;
			}
		}
	}
	
}
int main(void){
	int n,i,origin,box[10],max,min;
	scanf("%d",&n);
	while(n--){
		scanf("%d",&origin);
		max=0;min=0;
		for(i=0;i<8;i++){box[i]=origin%10;origin/=10;}
		sort(box);
		for(i=0;i<8;i++){max+=box[i];max*=10;}
		max/=10;
		for(i=7;i>=0;i--){min+=box[i];min*=10;}
		min/=10;
		printf("%d\n",max-min);
	}
	return 0;
}