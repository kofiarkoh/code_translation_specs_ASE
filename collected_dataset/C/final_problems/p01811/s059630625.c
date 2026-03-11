#include<stdio.h>
#include<stdlib.h>
#include<string.h>
#define ll long long
#define rep(i,l,r)for(ll i=(l);i<(r);i++)

char s[5010];
int main(){
	scanf("%s",s);
	if(strlen(s)<3){
		puts("No");
		return 0;
	}
	while(s[1]){
		char temp[5010]={};
		int cnt=0;
		int n=strlen(s);
		int flag=0;
		int check=0;
		rep(i,0,n){
			if(s[i]=='A'&&s[i+1]=='B'&&s[i+2]=='C'){
				temp[cnt++]='X';
				i+=2;
				check=1;
			}else{
				temp[cnt++]=s[i];
				flag|=1<<(s[i]-'A');
			}
		}
		if(cnt==1){
			puts("Yes");
			return 0;
		}else if((check&&(flag==3||flag==5||flag==6))){
			rep(i,0,cnt)if(temp[i]=='X')temp[i]=flag==3?'C':flag==5?'B':'A';
			memcpy(s,temp,5000);
		}else{
			puts("No");
			return 0;
		}
	}
}
