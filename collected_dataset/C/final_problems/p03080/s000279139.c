#include<stdio.h>
#include<stdlib.h>
#include<string.h>
int main(){
	char str[150];
	int a[2];
	int n,i;
	while(scanf("%d",&n)!=EOF){
		getchar();
		memset(a,0,sizeof(a));
		for(i=0;i<n;i++){
			scanf("%c",&str[i]);
		}
		for(i=0;i<n;i++){
			if(str[i] == 'R'){
				a[0]++;
			}
			else{
				a[1]++;
			}
		}
		if(a[0]>a[1]){
			printf("Yes\n");
		}
		else{
			printf("No\n");
		}
	}
	return 0;
}