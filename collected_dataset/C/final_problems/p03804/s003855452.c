#include<stdio.h>
int main(){
	int A_wh,B_wh,i,j,mas=0,ans=0,k=0,l=0;
	char mapA[50][50],mapB[50][50];
	
	scanf("%d %d ",&A_wh,&B_wh);
	//c=getchar();ここでscanfで残された\nを除去する.
	
	//'.'=白,'#'=黒
	for(i=0;i<A_wh;i++){
		for(j=0;j<=A_wh;j++){
			mapA[i][j]=getchar();
		}
	}
	
	for(i=0;i<B_wh;i++){
		for(j=0;j<=B_wh;j++){//改行記号を読み込むため
			mapB[i][j]=getchar();
		}
	}
	//調べる
	while(l!=A_wh-1){
		for(i=0;i<A_wh;i++){
			for(j=0;j<B_wh;j++){
				if(mapA[k][l]==mapB[i][j]){
					mas++;
				}
			}
		}
		if(mas==B_wh*B_wh){
			ans=1;
			break;
		}
		else{
			k++;
			if(k==B_wh){
				l++;
				if(l==B_wh){
					break;
				}
			}
		}
	}
	if(ans==1){
		printf("Yes\n");
	}
	else{
		printf("No\n");
	}
	
	return 0;
}
