#include <stdio.h>
int main(){
	int i=0;
	int j=0;
	
	int A=0;
	int B=0;
	int X=0;
	
	int ans=0;
	
	int MIN=999999;
	
	scanf("%d %d %d",&A,&B,&X);
	
	for(i=0;i<=20000/1000;i++){
		for(j=0;j<=20000/500;j++){
			//合計金額
			ans=A*i+B*j;
			if(ans<MIN&&1000*i+500*j>=X){
				MIN=ans;
			}
		}
	}
	printf("%d\n",MIN);
	return 0;
}
