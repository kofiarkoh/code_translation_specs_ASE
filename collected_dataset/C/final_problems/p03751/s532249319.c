#include<stdio.h>
char s[10000][21],t[21];
int main(int argc, char const *argv[]){
	int n,a=0,b=0,i,j,bf,af;
	scanf("%d",&n);
	for(i=0;i<n;i++){
		scanf("%s",s[i]);
	}
	scanf("%s",t);
	for(i=0;i<n;i++){
		af=1;
		bf=1;
		for(j=0;j<20;j++){
			if(s[i][j]=='\0'){
				if(t[j]!='\0'){
					if(af) a++;
				}
				break;
			}else if(t[j]=='\0'){
				if(bf) b++;
				break;
			}else if(s[i][j]=='?'){
				if(t[j]=='a') bf=0;
				else if(t[j]=='z') af=0;
				else break;
			}else if(s[i][j]>t[j]){
				if(bf) b++;
			}else if(s[i][j]<t[j]){
				if(af) a++;
			}
		}
	}
	printf("%d",a+1);
	for(i=a+1;i<n+1-b;i++){
		printf(" %d",i);
	}
	printf("\n");
	return 0;
}