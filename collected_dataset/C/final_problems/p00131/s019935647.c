#include<stdio.h>
#include<string.h>
int main(){
	int i,j,n,s[12][12],a[12][12];
	scanf("%d",&n);
	while(n--){
		for(i=0;i<10;i++)for(j=0;j<10;j++)scanf("%d",&s[i][j]);
		memset(a,0,sizeof(a));
		for(i=0;i<9;i++){
			for(j=0;j<10;j++){
				if(s[i][j]%2==1){
					a[i+1][j]=1;
					s[i][j]++;
					s[i+1][j]++;
					s[i+1][j-1]++;
					s[i+1][j+1]++;
					s[i+2][j]++;
				}
			}
		}
		for(i=0;i<10;i++){for(j=0;j<10;j++)printf("%d ",a[i][j]);printf("\n");}
	}
	return 0;
}