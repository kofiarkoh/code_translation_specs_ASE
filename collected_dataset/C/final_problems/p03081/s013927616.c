#include<stdio.h>

int main(){
	int n,q,i,j;
	char s[200001],td[200000][2][2];
	scanf("%d%d%s",&n,&q,s);
	/*for(i=0;i<q;i++){
		scanf("%c%c",&td[i][0],&td[i][1]);
	}*/
	for(i=0;i<q;i++)scanf("%s%s",td[i][0],td[i][1]);
	int r=n-1,l=0;
	for(i=q-1;i>=0;i--){
		if(s[r]==td[i][0][0]){
			if(td[i][1][0]=='R'){
				r--;
			}
		}
		if(s[l]==td[i][0][0]){
			if(td[i][1][0]=='L'){
				l++;
			}
		}
	}
	printf("%d\n",r-l+1);
	return 0;
}