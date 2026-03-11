#include<stdio.h>
int main(){
	int i=0,l,n,m,a[12],b;
	while(1){
		b=0;
		scanf("%d",&l);
		if(l==0){
			break;
		}
		for(i=0;i<12;i++){
			scanf("%d%d",&m,&n);
			a[i]=m-n;
		}
		for(i=0;i<12;i++){
			b=b+a[i];
			if(b>=l){
				printf("%d\n",i+1);
				break;
			}
		}
		if(b<l){
			printf("NA\n");
		}
	}
}